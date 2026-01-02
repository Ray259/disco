pub enum SkillType {
    INT(IntSkill),
    PSY(PsySkill),
    FYS(FysSkill),
    MOT(MotSkill),
}

pub enum IntSkill {
    LOGIC,
    RHETORIC,
    ENCYCLOPEDIA,
    DRAMA,
    VISUAL_CALCULUS,
}

pub enum PsySkill {
    VOLITION,
    INLAND_EMPIRE,
    EMPATHY,
    ESPRIT_DE_CORPS,
    AUTHORITY,
    SUGGESTION,
}

pub enum FysSkill {
    ENDURANCE,
    PHYSICAL_INSTRUMENT,
    PAIN_THRESHOLD,
    SHIVERS,
    HALF_LIGHT,
}

pub enum MotSkill {
    HAND_EYE_COORDINATION,
    PERCEPTION,
    SAVOIR_FAIRE,
    REACTION_SPEED,
    INTERFACING,
    COMPOSURE,
}