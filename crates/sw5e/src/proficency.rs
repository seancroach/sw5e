/// This enum represents the proficency levels in Star Wars 5e.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Proficency {
    /// The character is not proficent in the skill and has no bonus.
    #[default]
    Untrained,
    /// The character is half-proficient in the skill and adds half their
    /// proficiency bonus, rounded down. Training can only be obtained in
    /// skills, tools, saving throws, and weapons.
    Trained,
    /// The character is proficient in the skill and adds their full proficiency
    /// bonus. Proficency can only be obtained in skills, tools, saving throws,
    /// and weapons.
    Proficent,
    /// The character is an expert in the skill and adds twice their proficiency
    /// bonus. Expertise can only be obtained in skills, tools, and saving
    /// throws.
    Expertise,
    /// The character is a master in the skill and adds twice their proficiency
    /// bonus. Additionally, they always have advantage with mastery. Mastery
    /// can only be obtained in skills, tools, and saving throws.
    Mastery,
    /// The character is a high master in the skill and adds twice their
    /// proficiency bonus. Additionally, they always have advantage with
    /// high mastery, and when you make a roll with advantage at this tier
    /// of proficency, you can reroll one of the dice once; they must use
    /// the new roll. High mastery can only be obtained in skills, tools,
    /// and saving throws.
    HighMastery,
    /// Grand mastery lets you add twice your proficiency bonus. Additionally,
    /// you always have advantage with grand mastery, and when you make a roll
    /// with advantage at this tier of proficiency, you can reroll each of the
    /// dice once. You must use the new roll for each die. Grand mastery can be
    /// obtained in skills, tools, and saving throws.
    GrandMastery,
}

impl Proficency {
    /// Returns the next proficency level, or `None` if the current level is
    /// `GrandMastery`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sw5e::Proficency::*;
    ///
    /// assert_eq!(Untrained.increase(), Some(Trained));
    /// assert_eq!(Trained.increase(), Some(Proficent));
    /// assert_eq!(Proficent.increase(), Some(Expertise));
    /// assert_eq!(Expertise.increase(), Some(Mastery));
    /// assert_eq!(Mastery.increase(), Some(HighMastery));
    /// assert_eq!(HighMastery.increase(), Some(GrandMastery));
    /// assert_eq!(GrandMastery.increase(), None);
    /// ```
    #[must_use]
    pub const fn increase(self) -> Option<Self> {
        match self {
            Self::Untrained => Some(Self::Trained),
            Self::Trained => Some(Self::Proficent),
            Self::Proficent => Some(Self::Expertise),
            Self::Expertise => Some(Self::Mastery),
            Self::Mastery => Some(Self::HighMastery),
            Self::HighMastery => Some(Self::GrandMastery),
            Self::GrandMastery => None,
        }
    }

    /// Returns the next proficency level, wrapping around to `Untrained` if the
    /// current level is `GrandMastery`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sw5e::Proficency::*;
    ///
    /// assert_eq!(Untrained.increase_wrapping(), Trained);
    /// assert_eq!(Trained.increase_wrapping(), Proficent);
    /// assert_eq!(Proficent.increase_wrapping(), Expertise);
    /// assert_eq!(Expertise.increase_wrapping(), Mastery);
    /// assert_eq!(Mastery.increase_wrapping(), HighMastery);
    /// assert_eq!(HighMastery.increase_wrapping(), GrandMastery);
    /// assert_eq!(GrandMastery.increase_wrapping(), Untrained);
    /// ```
    #[must_use]
    pub const fn increase_wrapping(self) -> Self {
        match self {
            Self::Untrained => Self::Trained,
            Self::Trained => Self::Proficent,
            Self::Proficent => Self::Expertise,
            Self::Expertise => Self::Mastery,
            Self::Mastery => Self::HighMastery,
            Self::HighMastery => Self::GrandMastery,
            Self::GrandMastery => Self::Untrained,
        }
    }

    /// Returns the previous proficency level, or `None` if the current level is
    /// `Untrained`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sw5e::Proficency::*;
    ///
    /// assert_eq!(Untrained.decrease(), None);
    /// assert_eq!(Trained.decrease(), Some(Untrained));
    /// assert_eq!(Proficent.decrease(), Some(Trained));
    /// assert_eq!(Expertise.decrease(), Some(Proficent));
    /// assert_eq!(Mastery.decrease(), Some(Expertise));
    /// assert_eq!(HighMastery.decrease(), Some(Mastery));
    /// assert_eq!(GrandMastery.decrease(), Some(HighMastery));
    /// ```
    #[must_use]
    pub const fn decrease(self) -> Option<Self> {
        match self {
            Self::Untrained => None,
            Self::Trained => Some(Self::Untrained),
            Self::Proficent => Some(Self::Trained),
            Self::Expertise => Some(Self::Proficent),
            Self::Mastery => Some(Self::Expertise),
            Self::HighMastery => Some(Self::Mastery),
            Self::GrandMastery => Some(Self::HighMastery),
        }
    }

    /// Returns the previous proficency level, wrapping around to `GrandMastery`
    /// if the current level is `Untrained`.
    ///
    /// # Examples
    ///
    /// ```
    /// use sw5e::Proficency::*;
    ///
    /// assert_eq!(Untrained.decrease_wrapping(), GrandMastery);
    /// assert_eq!(Trained.decrease_wrapping(), Untrained);
    /// assert_eq!(Proficent.decrease_wrapping(), Trained);
    /// assert_eq!(Expertise.decrease_wrapping(), Proficent);
    /// assert_eq!(Mastery.decrease_wrapping(), Expertise);
    /// assert_eq!(HighMastery.decrease_wrapping(), Mastery);
    /// assert_eq!(GrandMastery.decrease_wrapping(), HighMastery);
    /// ```
    #[must_use]
    pub const fn decrease_wrapping(self) -> Self {
        match self {
            Self::Untrained => Self::GrandMastery,
            Self::Trained => Self::Untrained,
            Self::Proficent => Self::Trained,
            Self::Expertise => Self::Proficent,
            Self::Mastery => Self::Expertise,
            Self::HighMastery => Self::Mastery,
            Self::GrandMastery => Self::HighMastery,
        }
    }
}
