
/// # Examples
/// ```
/// pub struct HealthModifierSettings {
///     pub enabled: bool,
///     pub max_health_mul: f32,
///     pub healing_mul: f32
///     pub damage_taken_mul: f32
/// }
/// 
/// impl mm_hook::settings::ModSettings for HealthMultiplierSettings {
///     fn items() -> &'static [UISystemMenuItem] {
///         [
///         ]
///     }
/// }
/// 
/// // ... //
/// 
/// mm_hook::init_mod!(
///     "HealthModifier",
///     "1.0.0",
///     "L",
///     {
///         mm_hook::init();
///         mm_hook::settings::register_settings<HealthModifierSettings>();
///     }
/// )
/// 
/// ```
pub trait ModSettings {
    fn items() -> &'static [Item<Self>] where Self: Sized;
}

// Type(Title, ...data)
pub enum Item<T: ModSettings> {
    /// Toggle(title, default, on_change)
    Toggle(&'static str, bool, fn(T, bool)),
    /// Slider(title, min, max, default, on_change)
    Slider(&'static str, u32, u32, u32, fn(T, u32)),
    /// Select(title, options, default, on_change)
    Select(&'static str, &'static [&'static str], usize, fn(T, usize)),
    // TODO: Colour(title, options, default, on_change)
}