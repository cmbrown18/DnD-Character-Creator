///File that holds the different types of proficiency a character might have
/// along with the functions for setting said proficiency


//List of proficiency that a character has
struct Prof{
    acrobatics: bool,
    animal_handling: bool,
    arcana: bool,
    deception: bool,
    history: bool,
    insight: bool,
    intimidation: bool,
    investigation: bool,
    medicine: bool,
    nature: bool,
    perception: bool,
    persuasion: bool,
    religion: bool,
    sleight_of_hand: bool,
    stealth: bool,
    survival: bool,
}

//List of Saving throws the character is proficient in
struct Saving_thr{
    str: bool,
    dex: bool,
    con: bool,
    int: bool,
    wis: bool,
    cha: bool
}

//List of musical instruments that a character is proficient in
struct musical_prof{
    bagpipes: bool,
    drum: bool,
    dulcimer: bool,
    flute: bool,
    lute: bool,
    lyre: bool,
    horn: bool,
    pan_flute: bool,
    shawm: bool,
    viol: bool
}

//List of vehicles that a character is proficient in
struct vehicle_prof{
    land: bool,
    water: bool
}

//List of games that a character is proficient in
struct game_prof{
    board: bool,
    card: bool,
    dice: bool,
    other: bool
}

//List of tools that a character is proficient in
struct tool_prof{
    disguise: bool,
    forgery: bool,
    herbalism: bool,
    navigator: bool,
    poisoner: bool,
    thieves: bool,
    alchemist: bool,
    brewer: bool,
    calligrapher: bool,
    carpenter: bool,
    cartographer: bool,
    cobbler: bool,
    cook: bool,
    glassblower: bool,
    jeweler: bool,
    leather_worker: bool,
    mason: bool,
    painter: bool,
    potter: bool,
    smith: bool,
    tinker: bool,
    weaver: bool,
    woodcarver: bool
}

///Accepts a vector of strings that is the proficiencies and sets corresponding
/// proficiencies to true
fn set_prof(Vec:prof){

}

///Accepts a vector of string that is the saving throws and sets corresponding
/// saving throws to true
fn set_saving_throws(Vec:saving_throws){

}

///Accepts a vector of strings that is the musical instruments a character
/// is proficient in and sets them to true
fn set_musical_prof(Vec:prof){

}

///Accepts a vector of strings that is the vehicles that a character is
/// proficient in and sets them to true
fn set_vehicle_prof(Vec:prof){

}

///Accepts a vector of strings that is the games a character is
/// proficient in and sets them to true
fn set_game_prof(Vec:prof){

}

///Accepts a vector of strings that is the tools that a character is
/// proficient in and sets them to true
fn set_tools_prof(Vec:prof){

}