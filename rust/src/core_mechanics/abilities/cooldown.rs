use crate::latex_formatting;


#[derive(Clone)]
pub enum Cooldown {
    Brief(Option<String>),
    ShortRest(Option<String>),
    LongRest(Option<String>),
}

impl Cooldown {
    pub fn description(&self, use_you: bool) -> String {
        let tag = match self {
            Self::Brief(tag) => tag,
            Self::ShortRest(tag) => tag,
            Self::LongRest(tag) => tag,
        };
        let it = if let Some(t) = tag {
            format!("it or any other \\abilitytag<{}> ability", t)
        } else {
            "it".to_string()
        };
        if use_you {
            let until = match self {
                Self::Brief(_) => format!("\\glossterm<briefly> cannot use {} again", it),
                Self::ShortRest(_) => format!(
                    "cannot use {} again until you take a \\glossterm<short rest>",
                    it
                ),
                Self::LongRest(_) => format!(
                    "cannot use {} again until you take a \\glossterm<long rest>",
                    it
                ),
            };
            return latex_formatting::latexify(format!(
                "After you use this ability, you {until}.",
                until = until,
            ));
        } else {
            let until = match self {
                Self::Brief(_) => format!("\\glossterm<briefly> cannot use {} again", it),
                Self::ShortRest(_) => format!(
                    "cannot use {} again until it takes a \\glossterm<short rest>",
                    it
                ),
                Self::LongRest(_) => format!(
                    "cannot use {} again until it takes a \\glossterm<long rest>",
                    it
                ),
            };
            return latex_formatting::latexify(format!(
                "After the $name uses this ability, it {until}.",
                until = until,
            ));
        }
    }
}
