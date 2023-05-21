//! Helper types to combine and consolidate IRs across devices.

use crate::ir;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap, HashSet},
};

/// An element version.
pub struct Version<'ir, E> {
    /// Reference to the element.
    elem: &'ir E,
    /// The IRs that use this version.
    irs: Vec<&'ir ir::IR>,
}

impl<'ir, E> Version<'ir, E> {
    fn new(elem: &'ir E, ir: &'ir ir::IR) -> Self {
        Self {
            elem,
            irs: vec![ir],
        }
    }

    /// Acquire the IR element.
    pub fn element(&self) -> &'ir E {
        self.elem
    }

    /// Returns `true` if the provided IR uses this element version.
    ///
    /// This uses a pointer comparison to understand if the IRs are equal.
    /// It does not use any (Partial)Eq trait.
    pub fn is_used_by(&self, ir: &ir::IR) -> bool {
        self.irs.iter().any(|jr| std::ptr::eq(ir, *jr))
    }
}

/// A version of an enum.
pub type EnumVersion<'ir> = Version<'ir, ir::Enum>;
/// A version of a field set.
pub type FieldSetVersion<'ir> = Version<'ir, ir::FieldSet>;
/// A version of a block.
pub type BlockVersion<'ir> = Version<'ir, ir::Block>;

/// Multiple versions of some element.
type Versions<'ir, E> = Vec<Version<'ir, E>>;

/// Used to sort versions by most popular (most IR associations) to least
/// popular (fewest IR associations).
fn popularity<E>(a: &Version<'_, E>, b: &Version<'_, E>) -> Ordering {
    b.irs.len().cmp(&a.irs.len())
}

#[derive(Clone, Copy)]
struct CompareIr<'ir, E> {
    elem: &'ir E,
    ir: &'ir ir::IR,
}

impl<'ir, E> CompareIr<'ir, E> {
    fn from_version(version: &Version<'ir, E>) -> Self {
        Self::new(
            version.element(),
            version.irs.first().expect("Versions always have an IR"),
        )
    }
    fn new(elem: &'ir E, ir: &'ir ir::IR) -> Self {
        Self { elem, ir }
    }
    fn query(ir: &'ir ir::IR, query: impl FnOnce(&'ir ir::IR) -> Option<&'ir E>) -> Option<Self> {
        query(ir).map(|elem| Self::new(elem, ir))
    }
}

type IrPath<'ir> = &'ir str;

/// Extract the part of the IR path that describes the peripheral.
fn peripheral_part(path: IrPath) -> &str {
    path.split("::")
        .next()
        .expect("IR paths are separated by ::")
}

/// Assert two elements as equivalent.
///
/// The implementation invokes `compare` for similarly-named things
/// across IRs. For instance, the input will always be two UART
/// blocks from two different devices. You'll never see an UART and an
/// I2C block being compared for equivalence (unless your IR is really
/// messed up).
trait Equivalence<E> {
    fn compare(&self, left: CompareIr<E>, right: CompareIr<E>, path: IrPath) -> bool;
}

impl<E, Equiv> Equivalence<E> for &Equiv
where
    Equiv: Equivalence<E>,
{
    fn compare(&self, left: CompareIr<E>, right: CompareIr<E>, path: IrPath) -> bool {
        (*self).compare(left, right, path)
    }
}

/// Equivalence adapter for checking paths against a set of regexes.
///
/// If there's a regex match, the element is deemed "never equivalent".
/// This check happens before calling the wrapped equivalence.
struct PathExcluded<'a, Equiv> {
    exclusions: &'a [Regex],
    equiv: Equiv,
}

impl<Equiv, E> Equivalence<E> for PathExcluded<'_, Equiv>
where
    Equiv: Equivalence<E>,
{
    fn compare(&self, left: CompareIr<E>, right: CompareIr<E>, path: IrPath) -> bool {
        for exclusion in self.exclusions {
            if exclusion.is_match(path) {
                return false;
            }
        }

        self.equiv.compare(left, right, path)
    }
}

/// Ensure the items in two, possibly non-sorted contiguous
/// collections are equivalent.
fn equivalent_slices<E>(xs: &[E], ys: &[E], equiv: impl Fn(&E, &E) -> bool) -> bool {
    xs.len() == ys.len() && xs.iter().all(|x| ys.iter().any(|y| equiv(x, y)))
}

fn equivalent_options<E>(
    a: Option<CompareIr<E>>,
    b: Option<CompareIr<E>>,
    path: IrPath,
    equiv: impl Equivalence<E>,
) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => equiv.compare(a, b, path),
        (None, None) => true,
        (_, _) => false,
    }
}

#[derive(Clone, Copy)]
struct EquivalentEnums<'a> {
    peripherals: &'a HashSet<String>,
}

impl Equivalence<ir::Enum> for EquivalentEnums<'_> {
    fn compare(
        &self,
        CompareIr { elem: a, .. }: CompareIr<ir::Enum>,
        CompareIr { elem: b, .. }: CompareIr<ir::Enum>,
        path: IrPath,
    ) -> bool {
        let assert_name_equivalence = self.peripherals.contains(peripheral_part(path));
        a.bit_size == b.bit_size
            && equivalent_slices(&a.variants, &b.variants, |q, r| {
                q.value == r.value && (!assert_name_equivalence || q.name == r.name)
            })
    }
}

#[derive(Clone, Copy)]
struct EquivalentFieldSets<'a> {
    enums: EquivalentEnums<'a>,
}

impl Equivalence<ir::FieldSet> for EquivalentFieldSets<'_> {
    fn compare(
        &self,
        CompareIr { elem: a, ir: air }: CompareIr<ir::FieldSet>,
        CompareIr { elem: b, ir: bir }: CompareIr<ir::FieldSet>,
        path: IrPath,
    ) -> bool {
        let try_equivalent_enum = |a: &Option<String>, b: &Option<String>| -> bool {
            let a = a
                .as_ref()
                .and_then(|a| CompareIr::query(air, |ir| ir.enums.get(a)));
            let b = b
                .as_ref()
                .and_then(|b| CompareIr::query(bir, |ir| ir.enums.get(b)));
            equivalent_options(a, b, path, self.enums)
        };

        a.bit_size == b.bit_size
            && equivalent_slices(&a.fields, &b.fields, |q, r| {
                q.name == r.name
                    && q.bit_offset == r.bit_offset
                    && q.array == r.array
                    && q.bit_size == r.bit_size
                    && try_equivalent_enum(&q.enum_read, &r.enum_read)
                    && try_equivalent_enum(&q.enum_write, &r.enum_write)
                    && try_equivalent_enum(&q.enum_readwrite, &r.enum_readwrite)
            })
    }
}

#[derive(Clone, Copy)]
struct EquivalentBlocks<'a> {
    fieldsets: EquivalentFieldSets<'a>,
}

impl EquivalentBlocks<'_> {
    fn equivalent_registers(
        &self,
        CompareIr { elem: a, ir: air }: CompareIr<ir::Register>,
        CompareIr { elem: b, ir: bir }: CompareIr<ir::Register>,
        path: IrPath,
    ) -> bool {
        let query_builder =
            |ir| move |fieldset: &String| CompareIr::query(ir, |ir| ir.fieldsets.get(fieldset));

        a.access == b.access
            && a.bit_size == b.bit_size
            && equivalent_options(
                a.fieldset.as_ref().and_then(query_builder(air)),
                b.fieldset.as_ref().and_then(query_builder(bir)),
                path,
                self.fieldsets,
            )
    }

    /// Check if two blocks are equivalent.
    fn equivalent_blocks(
        &self,
        CompareIr { elem: a, ir: air }: CompareIr<ir::Block>,
        CompareIr { elem: b, ir: bir }: CompareIr<ir::Block>,
        path: IrPath,
    ) -> bool {
        a.extends == b.extends
            && equivalent_slices(&a.items, &b.items, |q, r| {
                q.byte_offset == r.byte_offset
                    && q.array == r.array
                    && match (&q.inner, &r.inner) {
                        (
                            ir::BlockItemInner::Block(ir::BlockItemBlock { block: ablock }),
                            ir::BlockItemInner::Block(ir::BlockItemBlock { block: bblock }),
                        ) => self.equivalent_blocks(
                            CompareIr::query(air, |ir| ir.blocks.get(ablock)).unwrap(),
                            CompareIr::query(bir, |ir| ir.blocks.get(bblock)).unwrap(),
                            path,
                        ),
                        (
                            ir::BlockItemInner::Register(aregister),
                            ir::BlockItemInner::Register(bregister),
                        ) => self.equivalent_registers(
                            CompareIr::new(aregister, air),
                            CompareIr::new(bregister, bir),
                            path,
                        ),
                        _ => false,
                    }
            })
    }
}

impl Equivalence<ir::Block> for EquivalentBlocks<'_> {
    fn compare(
        &self,
        left: CompareIr<ir::Block>,
        right: CompareIr<ir::Block>,
        path: IrPath,
    ) -> bool {
        self.equivalent_blocks(left, right, path)
    }
}

/// Manages versions for an IR element type.
struct VersionLookup<'ir, E> {
    versions: HashMap<&'ir str, Versions<'ir, E>>,
}

impl<'ir, E> VersionLookup<'ir, E> {
    /// Create new version lookups for an IR's elements.
    fn new(
        equiv: impl Equivalence<E>,
        map: impl Iterator<Item = (&'ir ir::IR, &'ir String, &'ir E)>,
    ) -> Self {
        let versions = map.fold(
            HashMap::new(),
            |mut versions: HashMap<&'ir str, Versions<'ir, E>>, (ir, path, elem)| {
                versions
                    .entry(path.as_str())
                    .and_modify(|versions| {
                        if let Some(version) = versions.iter_mut().find(|version| {
                            equiv.compare(
                                CompareIr::from_version(version),
                                CompareIr::new(elem, ir),
                                path.as_str(),
                            )
                        }) {
                            version.irs.push(ir);
                        } else {
                            versions.push(Version::new(elem, ir))
                        }
                    })
                    .or_insert_with(|| vec![Version::new(elem, ir)])
                    .sort_unstable_by(popularity);
                versions
            },
        );
        Self { versions }
    }

    fn from_irs(
        equiv: impl Equivalence<E>,
        irs: &'ir [ir::IR],
        access: impl Fn(&'ir ir::IR) -> &HashMap<String, E>,
    ) -> Self {
        let map = irs
            .iter()
            .flat_map(|ir| std::iter::repeat(ir).zip(access(ir).iter()))
            .map(|(ir, (path, elem))| (ir, path, elem));
        Self::new(equiv, map)
    }

    fn get(&self, ir: &ir::IR, path: &str) -> Option<&Version<E>> {
        self.versions
            .get(path)
            .and_then(|versions| versions.iter().find(|version| version.is_used_by(ir)))
    }
}

/// Manages versions of IR elements.
///
/// The implementation uses the address of the IR when querying for versioned elements.
/// This should be fine, since the implementation takes shared references to the IR, so
/// things can't (safely) move or be reassigned while this exists.
pub struct IrVersions<'ir> {
    enums: VersionLookup<'ir, ir::Enum>,
    fieldsets: VersionLookup<'ir, ir::FieldSet>,
    blocks: VersionLookup<'ir, ir::Block>,
}

impl<'ir> IrVersions<'ir> {
    /// Define versions of IR elements from the collection of IRs.
    pub fn from_irs(irs: &'ir [ir::IR], config: &Config) -> Self {
        let exclusions: Vec<_> = config
            .never_combine
            .iter()
            .map(|path| Regex::new(&path).unwrap())
            .collect();
        let exclusions = &exclusions;

        let enums = EquivalentEnums {
            peripherals: &config.strict_enum_names,
        };
        let fieldsets = EquivalentFieldSets { enums };
        let blocks = EquivalentBlocks { fieldsets };

        Self {
            enums: VersionLookup::from_irs(
                PathExcluded {
                    exclusions,
                    equiv: enums,
                },
                irs,
                |ir| &ir.enums,
            ),
            fieldsets: VersionLookup::from_irs(
                PathExcluded {
                    exclusions,
                    equiv: fieldsets,
                },
                irs,
                |ir| &ir.fieldsets,
            ),
            blocks: VersionLookup::from_irs(
                PathExcluded {
                    exclusions,
                    equiv: blocks,
                },
                irs,
                |ir| &ir.blocks,
            ),
        }
    }
    /// Access an enum version that corresponds to this IR.
    pub fn get_enum(&self, ir: &ir::IR, path: &str) -> Option<&EnumVersion> {
        self.enums.get(ir, path)
    }
    /// Access a fieldset version that corresponds to this IR.
    pub fn get_fieldset(&self, ir: &ir::IR, path: &str) -> Option<&FieldSetVersion> {
        self.fieldsets.get(ir, path)
    }
    /// Access a block version that corresponds to this IR.
    pub fn get_block(&self, ir: &ir::IR, path: &str) -> Option<&BlockVersion> {
        self.blocks.get(ir, path)
    }
}

/// Hashing a reference by its address.
struct RefHash<'a, T>(&'a T);

impl<T> std::hash::Hash for RefHash<'_, T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::ptr::hash(self.0, state);
    }
}

impl<T> std::cmp::PartialEq for RefHash<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl<T> std::cmp::Eq for RefHash<'_, T> {}

impl<T> Clone for RefHash<'_, T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<T> Copy for RefHash<'_, T> {}

type RefMap<'a, K, V> = HashMap<RefHash<'a, K>, V>;

#[derive(Default)]
pub struct Config {
    strict_enum_names: HashSet<String>,
    never_combine: HashSet<String>,
}

/// Combine all IRs into a single IR.
pub fn combine(irs: &[ir::IR], config: &Config) -> ir::IR {
    assert!(
        irs.iter().all(|ir| !ir.devices.is_empty()),
        "Cannot combine an IR with empty devices."
    );
    assert!(
        irs.iter().all(|ir| ir.devices.len() == 1),
        "Sorry, not ready to combine IRs that were already combined"
    );
    {
        let device_names: Vec<_> = irs
            .iter()
            .map(|ir| ir.devices.keys().next().unwrap())
            .collect();
        assert!(
            device_names.len() == irs.len(),
            "Each IR must describe a unique device."
        );
        assert!(
            device_names.iter().all(|name| !name.is_empty()),
            "Each device needs a name."
        );
    }

    let versions = IrVersions::from_irs(irs, config);

    let mut consolidated = ir::IR::new();

    // Combine enums.
    let mut enums: RefMap<ir::Enum, String> = RefMap::new();
    for ir in irs {
        let device_name = ir.devices.keys().next().expect("Each IR has a name");

        for path in ir.enums.keys() {
            let version = versions
                .get_enum(ir, path)
                .expect("There's definitely a version");

            if let Entry::Vacant(entry) = enums.entry(RefHash(version.element())) {
                let path = format!("{device_name}::{path}");
                entry.insert(path.clone());
                consolidated.enums.insert(path, version.element().clone());
            }
        }
    }

    // Combine fieldsets.
    let mut fieldsets: RefMap<ir::FieldSet, String> = RefMap::new();
    for ir in irs {
        let device_name = ir.devices.keys().next().unwrap();

        for path in ir.fieldsets.keys() {
            let version = versions.get_fieldset(ir, path).unwrap();

            if let Entry::Vacant(entry) = fieldsets.entry(RefHash(version.element())) {
                let path = format!("{device_name}::{path}");
                entry.insert(path.clone());

                let mut fieldset = version.element().clone();
                // Fix references to enums by looking up the version, then mapping it to
                // the updated path.
                for field in &mut fieldset.fields {
                    for name in [
                        field.enum_readwrite.as_mut(),
                        field.enum_read.as_mut(),
                        field.enum_write.as_mut(),
                    ]
                    .into_iter()
                    .flatten()
                    {
                        let version = versions.get_enum(ir, name).unwrap();
                        *name = enums.get(&RefHash(version.element())).unwrap().into();
                    }
                }
                consolidated.fieldsets.insert(path, fieldset);
            }
        }
    }

    // Combine blocks.
    //
    // Block consolidation uses two passes, since a block
    // can have a reference to another block. The first pass
    // manages the version -> rename mapping, and the second
    // pass does the touch-up.
    let mut blocks: RefMap<ir::Block, String> = RefMap::new();
    for ir in irs {
        let device_name = ir.devices.keys().next().unwrap();

        for path in ir.blocks.keys() {
            let version = versions.get_block(ir, path).unwrap();

            if let Entry::Vacant(entry) = blocks.entry(RefHash(version.element())) {
                let path = format!("{device_name}::{path}");
                entry.insert(path.clone());
                consolidated.blocks.insert(path, version.element().clone());
            }
        }
    }

    let blocks = blocks;
    // Remove from this to ensure patches only happens once.
    let mut filter = blocks.clone();
    for ir in irs {
        for path in ir.blocks.keys() {
            let version = versions.get_block(ir, path).unwrap();

            if let Some(path) = filter.get(&RefHash(version.element())) {
                let block = consolidated.blocks.get_mut(path).unwrap();
                for item in &mut block.items {
                    match &mut item.inner {
                        ir::BlockItemInner::Register(reg) => {
                            for fieldset in &mut reg.fieldset {
                                let version = versions.get_fieldset(ir, fieldset).unwrap();
                                *fieldset =
                                    fieldsets.get(&RefHash(version.element())).unwrap().into()
                            }
                        }
                        ir::BlockItemInner::Block(ir::BlockItemBlock { block }) => {
                            let version = versions.get_block(ir, block).unwrap();
                            *block = blocks.get(&RefHash(version.element())).unwrap().into();
                        }
                    }
                }
            }
            filter.remove(&RefHash(version.element()));
        }
    }

    // Update all devices to point to new blocks.
    for ir in irs {
        let mut devices = ir.devices.clone();
        devices
            .values_mut()
            .flat_map(|device| device.peripherals.iter_mut())
            .flat_map(|peripheral| &mut peripheral.block)
            .for_each(|name: &mut String| {
                let version = versions.get_block(ir, name).unwrap();
                *name = blocks.get(&RefHash(version.element())).unwrap().into();
            });
        consolidated.devices.extend(devices);
    }

    consolidated
}

/// Configurations for the combiner pass.
#[derive(Debug, Serialize, Deserialize)]
pub enum Combine {
    /// The list of peripheral names that require strict enum name
    /// checks.
    ///
    /// By default, the combiner will not check the names of enum variants
    /// when evaluating enum equivalence. This allows the combiner treat
    /// superficial name differense, like "OFF / ON" and "DISABLE / ENABLE,"
    /// as equivalent.
    ///
    /// To enable strict name checking, add a peripheral name to this list.
    /// This means "OFF / ON" and "DISABLE / ENABLE" are not equivalent. It's
    /// always safe to add to this list; however, it means there may be more
    /// code generated.
    StrictEnumNames(Vec<String>),
    /// The list of patterns (regex string) to never combine.
    ///
    /// You should design patterns to the IR path names. Note that, unlike
    /// transform regexes, these do not implicitly match only starting
    /// characters.
    NeverCombine(Vec<String>),
}

impl<I> From<I> for Config
where
    I: IntoIterator<Item = Combine>,
{
    fn from(combines: I) -> Self {
        let mut config = Config::default();

        for combine in combines {
            match combine {
                Combine::StrictEnumNames(peripherals) => {
                    config.strict_enum_names.extend(peripherals);
                }
                Combine::NeverCombine(paths) => {
                    config.never_combine.extend(paths);
                }
            }
        }

        config
    }
}
