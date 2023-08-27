// Decompiled with JetBrains decompiler
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
// Type: WindowsApplication1.DataClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Runtime.Serialization;
// usingSystem.Runtime.Serialization.Formatters.Binary;
// usingSystem.Windows.Forms;

// namespace WindowsApplication1
// {
//   [Serializable]
//   pub class DataClass : ISerializable

use crate::action_card_class::ActionCardClass;
use crate::lib_id_class::LibIdClass;

pub const CONSTVERSION: i32 =  424;
pub const CONSTSUBVERSION: &'static str = ".04b";
pub const CONSTFILEEXTENSIONSLOAD: &'static str = "SE1 Scenario file (*.se1)|*.se1";
pub const CONSTFILEEXTENSIONSSAVE: &'static str = "SE1 Scenario file (*.se1)|*.se1";
pub const CONSTFILEEXTENSIONAUTOSAVE: &'static str = ".se1";
pub const CONSTVERSIONDISPLAYMINUS: i32 =  314;
pub const CONSTFILEEXTENSTIONLOADMAP: &'static str = "SE1 Map file(*.se1map)|*.se1map";
pub const CONSTFILEEXTENSIONEVENTLIB: &'static str = "SE1 Event library(*.se1evlib)|*.se1evlib";
pub const CONSTFILEEXTENSIONTROOPSLIB: &'static str = "SE1 Troops&Equipment library(*.se1troops)|*.se1troops";
pub const CONSTFILEEXTENSIONHISTORICALLIB: &'static str = "SE1 Historical library(*.se1his)|*.se1his";
pub const CONSTFILEEXTENSIONOFFICERLIB: &'static str = "SE1 Officer library(*.se1off)|*.se1off";
pub const CONSTFILEEXTENSIONOFFICERCARDLIB: &'static str = "SE1 Officer Card Library(*.se1offcard)|*.se1offcard";
pub const CONSTFILEEXTENSIONLOADANYLIB: &'static str = "SE1 Scenario file (*.se1)|*.se1|SE1 Map file(*.se1map)|*.se1map|SE1 Master file(*.se1master)|*.se1master|SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off";
pub const CONSTFILEEXTENSIONLOADLIB: &'static str = "SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off";
pub const CONSTFILEEXTENSIONLOADMASTER: &'static str = "SE1 Master file(*.se1master)|*.se1master";
pub const CONSTSHORTLOADMAP: &'static str = ".se1map";
pub const CONSTSHORTEVENTLIB: &'static str = ".se1evlib";
pub const CONSTSHORTTROOPSLIB: &'static str = ".se1troops";
pub const CONSTSHORTHISTORICALLIB: &'static str = ".se1his";
pub const CONSTSHORTOFFICERLIB: &'static str = ".se1off";
pub const CONSTSHORTOFFICERCARDLIB: &'static str = ".se1offcard";
pub const CONSTSHORTMASTER: &'static str = ".se1master";
pub const CONSTPREINSTALLEDGFXDIR: &'static str = "shadow";
pub const Slotcounter: i32 =  499;
pub const MoveGroup1: i32 =  0;
pub const MoveGroup2: i32 = 99;
pub const LandscapeGroup1: i32 = 100;
pub const LandscapeGroup2: i32 = 199;
pub const PeopleGroup1: i32 = 200;
pub const PeopleGroup2: i32 = 299;
pub const ItemGroup1: i32 = 300;
pub const ItemGroup2: i32 = 399;
pub const SFTypeGroup1: i32 = 400;
pub const SFTypeGroup2: i32 = 499;
pub const LocTypeGroup1: i32 = 500;
pub const LocTypeGroup2: i32 = 599;

#[derive(Default,Debug,Clone)]
pub struct DataClass
{
    pub Name: String,
    pub Product: i32,
    pub Description: String,
    pub Designer: String,
    pub Designer2: String,
    pub Round: i32,
    pub Version: i32,
    pub Turn: i32,
    pub InTurn: bool,
    pub StepNr: i32,
    pub GameSlot: Vec<i32>,
    pub GameSlotName: Vec<String>,
    pub GameSlotShow: Vec<bool>,
    pub GameSlotShow2: Vec<bool>,
    pub RegimeSlotName: Vec<String>,
    pub RegimeSlotShow: Vec<bool>,
    pub RegimeSlotShow2: Vec<i32>,
    pub RegimeSlotNato: Vec<i32>,
    pub RegimeSlotSmallGfx: Vec<i32>,
    pub RealTempString: Vec<String>,
    pub GameSlotNato: Vec<i32>,
    pub GameSlotSmallGfx: Vec<i32>,
    pub ShrowdOn: bool,
    pub FOWOn: bool,
    pub UncertaintyOn: bool,
    pub ASOn: bool,
    pub ResMod: i32,
    pub LoadPass: String,
    pub EditPass: String,
    pub MasterFile: String,
    pub Winner: i32,
    pub LastWinner: i32,
    pub VPWin: i32,
    pub PasswordsOn: bool,
    pub PBEM: bool,
    pub ScreenShotOn: bool,
    pub CreatedWithShrowd: bool,
    pub ShrowdPeek: bool,
    pub AutoSave: bool,
    pub Verify1: bool,
    pub Verify2: bool,
    pub MapCounter: i32,
    pub MapObj: Vec<MapClass>,
    pub LibraryCounter: i32,
    pub LibraryObj: Vec<LibraryClass>,
    pub LibVarCounter: i32,
    pub LibVarObj: Vec<LibVarClass>,
    pub MapWidth: i32,
    pub MapHeight: i32,
    pub HexObj: Vec<HexClass>,
    pub LandscapeTypeCounter: i32,
    pub LandscapeTypeObj: Vec<LandscapeTypeClass>,
    pub RoadTypeCounter: i32,
    pub RoadTypeObj: Vec<RoadTypeClass>,
    pub RegimeCounter: i32,
    pub RegimeObj: Vec<RegimeClass>,
    pub RegimeIdCounter: i32,
    pub UnitCounter: i32,
    pub UnitObj: Vec<UnitClass>,
    pub SFCounter: i32,
    pub SFObj: Vec<SFClass>,
    pub SFTypeCounter: i32,
    pub SFTypeObj: Vec<SFTypeClass>,
    pub SFTypeIdCounter: i32,
    pub LocTypeCounter: i32,
    pub LocTypeObj: Vec<LocationTypeClass>,
    pub LocCounter: i32,
    pub LocObj: Vec<LocationClass>,
    pub LocIdCounter: object,
    pub ItemTypeCounter: i32,
    pub ItemTypeObj: Vec<ItemTypeClass>,
    pub PeopleCounter: i32,
    pub PeopleObj: Vec<PeopleClass>,
    pub PeopleIdCounter: i32,
    pub StringCounter: i32,
    pub TempString: Vec<String>,
    pub RuleVar: Vec<f32>,
    pub RuleString: Vec<String>,
    pub RuleGroup: Vec<i32>,
    pub RuleGroup2: Vec<i32>,
    pub RuleCounter: i32,
    pub RiverTypeCounter: i32,
    pub RiverTypeObj: Vec<RiverTypeClass>,
    pub AreaCounter: i32,
    pub AreaObj: Vec<AreaClass>,
    pub AreaIDCounter: i32,
    pub HistoricalUnitCounter: i32,
    pub HistoricalUnitObj: Vec<HistoricalUnitClass>,
    pub HistoricalIDCounter: i32,
    pub BridgeObj: Vec<BridgeClass>,
    pub ActionCardCounter: i32,
    pub ActionCardObj: Vec<ActionCardClass>,
    pub ResearchCounter: i32,
    pub ResearchObj: Vec<ResearchClass>,
    pub EventCounter: i32,
    pub EventObj: Vec<EventClass>,
    pub EventIdCounter: i32,
    pub EventPicCounter: i32,
    pub EventPicName: Vec<String>,
    pub EventPicNr: Vec<i32>,
    pub eventPicLibId: Vec<LibIdClass>,
    pub SmallPicCounter: i32,
    pub SmallPicName: Vec<String>,
    pub SmallPicNr: Vec<i32>,
    pub SmallLibId: Vec<LibIdClass>,
    pub reinfIdCounter: i32,
    pub ReinfCounter: i32,
    pub ReinfName: Vec<String>,
    pub ReinfLibId: Vec<LibIdClass>,
    pub ReinfId: Vec<i32>,
    pub StringListCounter: i32,
    pub StringListObj: Vec<StringListClass>,
    pub StringIDCounter: i32,
    pub CheckTypeNames: Vec<String>,
    pub ExecTypeNames: Vec<String>,
    pub CheckTypeVarName: Vec<Vec<String>>,
    pub CheckTypeVarCount: Vec<i32>,
    pub CheckTypeCount: i32,
    pub CheckDesc: Vec<String>,
    pub CheckCategory: Vec<i32>,
    pub CheckCategory2: Vec<i32>,
    pub ExecCategory: Vec<i32>,
    pub ExecCategory2: Vec<i32>,
    pub ExecTypeVarName: Vec<String>,
    pub ExecTypeVarCount: Vec<i32>,
    pub ExecDesc: Vec<String>,
    pub ExecTypeCount: i32,
    pub ExecTypeString: Vec<i32>,
    pub TempVar: Vec<i32>,
    pub ExecCategoryName: Vec<String>,
    pub CheckCategoryName: Vec<String>,
    pub AlternateRound: i32,
    pub AlternateRound2: i32,
    pub StartData: DateTime,
    pub PPMultiplier: f32,
    pub SupplyMultiplier: f32,
    pub ResCostMod: f32,
    pub NoPlayChoice: bool,
    pub NoAIAdvice: bool,
    pub Variants: Vec<i32>,
    pub VariantEvent: Vec<i32>,
    pub MoveTypePenalty: Vec<i32>,
    pub UnitTypePenalty: Vec<i32>,
    pub WheaterColor: Vec<i32>,
    pub MasterfileReadPeople: bool,
    pub MapLoop: bool,
    pub ReinfRatio: Vec<i32>,
    pub GameID: i32,
    pub TerrorMode: bool,
    pub DontShowAIMove: bool,
    pub LoadGame: String,
    pub UseAI: i32,
    pub CampaignRoom: i32,
    pub SystemGfx: String,
    pub ScenarioDir: String,
    pub SoundDir: String,
    pub SFModelIDCounter: i32,
    pub TurnString: String,
    pub Loadable: bool,
    pub specialSaveMode: i32,
    pub RuleSetName: String,
    pub DoAllied: bool,
    pub PermanentOverlayUse: bool,
    pub PermanentOverlayName: String,
    pub PermanentOverlaySpriteID: i32,
    pub PbemGameID: i32,
    pub PbemPlayer1: String,
    pub PbemPlayer2: String,
    pub PbemGameOver: i32,
    pub PbemDrawGame: i32,
    pub AIUnitCounter: i32,
    pub SimpleEditor: bool,
    pub ExtraTabName: String,
    pub ExtraTabEvent: i32,
    pub ExtraTabName2: String,
    pub ExtraTabEvent2: i32,
    pub ExtraTabName3: String,
    pub ExtraTabEvent3: i32,
    pub ExtraTabName4: String,
    pub ExtraTabEvent4: i32,
    pub MapName: String,
    pub MapVersion: i32,
    pub MapDesigner: String,
    pub CombatLogId: i32,
    pub scenarioVersion: String,
    pub scenarioVersionMaster: String,
    pub transportMovementType: Vec<i32>,
    pub se1_earlyCinematicsLogin: i32,
}

impl DataClass {
    pub fn SetEventNames(&mut self)
    {
        self.CheckCategoryName.reserve(13);
      self.CheckCategoryName[1] = "Game & Map globals".to_owned();
      self.CheckCategoryName[2] = "Regimes".to_owned();
      self.CheckCategoryName[3] = "Logic & Calculations".to_owned();
      self.CheckCategoryName[4] = "Hex & Areaslots".to_owned();
      self.CheckCategoryName[5] = "Unit".to_owned();
      self.CheckCategoryName[6] = "Stringlist".to_owned();
      self.CheckCategoryName[7] = "SFType/ItemTypes".to_owned();
      self.CheckCategoryName[8] = "DC1 AI".to_owned();
      self.CheckCategoryName[9] = "DC2 AI".to_owned();
      self.CheckCategoryName[10] = "Library Checks".to_owned();
      self.CheckCategoryName[11] = "UDS".to_owned();
      self.CheckCategoryName[12] = "Random Related".to_owned();

      self.CheckTypeNames[1] = "CheckTurn".to_owned();
      self.CheckCategory[1] = 1;
      self.CheckTypeNames[2] = "CheckHexOwner".to_owned();
      self.CheckTypeVarCount[2] = 2;
      self.CheckTypeVarName[2][1] = "X".to_string();
      self.CheckTypeVarName[2][2] = "Y".to_owned();
      self.CheckCategory[2] = 4;
      self.CheckTypeNames[3] = "CheckRound".to_owned();
      self.CheckCategory[3] = 1;
      self.CheckTypeNames[4] = "CheckRegimeMorale".to_owned();
      self.CheckTypeVarCount[4] = 1;
      self.CheckTypeVarName[4][1] = "RegimeNr".to_owned();
      self.CheckCategory[4] = 2;
      self.CheckTypeNames[5] = "CheckWinner".to_owned();
      self.CheckTypeVarCount[5] = 0;
      self.CheckCategory[5] = 1;
      self.CheckTypeNames[6] = "CheckWar".to_string();
      self.CheckTypeVarCount[6] = 2;
      self.CheckTypeVarName[6][1] = "RegimeNr in Question".to_owned();
      self.CheckTypeVarName[6][2] = "Relation with RegimeNr".to_owned();
      self.CheckCategory[6] = 2;
      self.CheckTypeNames[7] = "CheckPeace".to_owned();
      self.CheckTypeVarCount[7] = 2;
      self.CheckTypeVarName[7][1] = "RegimeNr in Question";
      self.CheckTypeVarName[7][2] = "Relation with RegimeNr".to_owned();
      self.CheckCategory[7] = 2;
      self.CheckTypeNames[8] = "CheckUnitsFrom".to_owned();
      self.CheckTypeVarCount[8] = 4;
      self.CheckTypeVarName[8][1] = "AreaSlot".to_owned();
      self.CheckTypeVarName[8][2] = "AreaCode".to_owned();
      self.CheckTypeVarName[8][3] = "RegimeNr".to_owned();
      self.CheckTypeVarName[8][4] = "Not From PeopleNr".to_owned();
      self.CheckCategory[8] = 4;
      self.CheckTypeNames[10] = "CheckRandomPercent".to_owned();
      self.CheckTypeVarCount[10] = 0;
      self.CheckCategory[10] = 3;
      self.CheckTypeNames[11] = "CheckLandscapeType".to_owned();
      self.CheckTypeVarCount[11] = 2;
      self.CheckTypeVarName[11][1] = "X".to_owned();
      self.CheckTypeVarName[11][2] = "Y".to_owned();
      self.CheckCategory[11] = 4;
      self.CheckTypeNames[12] = "CheckLandscapeSprite".to_owned();
      self.CheckTypeVarCount[12] = 2;
      self.CheckTypeVarName[12][1] = "X".to_owned();
      self.CheckTypeVarName[12][2] = "Y".to_owned();
      self.CheckCategory[12] = 4;
      self.CheckTypeNames[13] = "CheckSlot".to_owned();
      self.CheckTypeVarCount[13] = 3;
      self.CheckTypeVarName[13][1] = "X".to_owned();
      self.CheckTypeVarName[13][2] = "Y".to_owned();
      self.CheckTypeVarName[13][3] = "Slot#".to_owned();
      self.CheckCategory[13] = 4;
      self.CheckTypeNames[14] = "CheckMapWidth".to_owned();
      self.CheckTypeVarCount[14] = 0;
      self.CheckCategory[14] = 1;
      self.CheckTypeNames[15] = "CheckMapHeight".to_owned();
      self.CheckTypeVarCount[15] = 0;
      self.CheckCategory[15] = 1;
      self.CheckTypeNames[16] = "CheckYear".to_owned();
      self.CheckTypeVarCount[16] = 0;
      self.CheckCategory[16] = 1;
      self.CheckTypeNames[17] = "CheckMonth".to_owned();
      self.CheckTypeVarCount[17] = 0;
      self.CheckCategory[17] = 1;
      self.CheckTypeNames[18] = "CheckDay".to_owned();
      self.CheckTypeVarCount[18] = 0;
      self.CheckCategory[18] = 1;
      self.CheckTypeNames[19] = "CheckIfAI".to_owned();
      self.CheckTypeVarCount[19] = 1;
      self.CheckTypeVarName[19][1] = "Regime#".to_owned();
      self.CheckCategory[19] = 2;
      self.CheckTypeNames[20] = "CheckPolPts".to_owned();
      self.CheckTypeVarCount[20] = 1;
      self.CheckTypeVarName[20][1] = "Regime#".to_owned();
      self.CheckCategory[20] = 2;
      self.CheckTypeNames[21] = "CheckVP".to_owned();
      self.CheckTypeVarCount[21] = 1;
      self.CheckTypeVarName[21][1] = "Regime#".to_owned();
      self.CheckCategory[21] = 2;
      self.CheckTypeNames[22] = "CheckDifferenceInDays".to_owned();
      self.CheckTypeVarCount[22] = 3;
      self.CheckTypeVarName[22][1] = "Year".to_owned();
      self.CheckTypeVarName[22][2] = "Month".to_owned();
      self.CheckTypeVarName[22][3] = "Day".to_owned();
      self.CheckCategory[22] = 3;
      self.CheckTypeNames[23] = "CheckSupplyNeeded".to_owned();
      self.CheckTypeVarCount[23] = 1;
      self.CheckTypeVarName[23][1] = "Regime".to_owned();
      self.CheckCategory[23] = 2;
      self.CheckTypeNames[24] = "CheckHQFor".to_owned();
      self.CheckTypeVarCount[24] = 2;
      self.CheckTypeVarName[24][1] = "X".to_owned();
      self.CheckTypeVarName[24][2] = "Y".to_owned();
      self.CheckCategory[24] = 4;
      self.CheckTypeNames[27] = "CheckActionCard".to_owned();
      self.CheckTypeVarCount[27] = 2;
      self.CheckTypeVarName[27][1] = "Regime Owner".to_owned();
      self.CheckTypeVarName[27][2] = "Card Nr".to_owned();
      self.CheckCategory[27] = 2;
      self.CheckTypeNames[28] = "CheckDipBlock".to_owned();
      self.CheckTypeVarCount[28] = 1;
      self.CheckTypeVarName[28][1] = "Regime#".to_owned();
      self.CheckCategory[28] = 2;
      self.CheckTypeNames[29] = "CheckSleep".to_owned();
      self.CheckTypeVarCount[29] = 1;
      self.CheckTypeVarName[29][1] = "Regime#".to_owned();
      self.CheckCategory[29] = 2;
      self.CheckTypeNames[30] = "CheckHistoricActionCard".to_owned();
      self.CheckTypeVarCount[30] = 2;
      self.CheckTypeVarName[30][1] = "Regime Owner".to_owned();
      self.CheckTypeVarName[30][2] = "Card Nr".to_owned();
      self.CheckCategory[30] = 2;
      self.CheckTypeNames[31] = "CheckSFTypeInArea".to_owned();
      self.CheckTypeVarCount[31] = 4;
      self.CheckTypeVarName[31][1] = "AreaSlot".to_owned();
      self.CheckTypeVarName[31][2] = "AreaCode (-1=all hexes)".to_owned();
      self.CheckTypeVarName[31][3] = "SFType".to_owned();
      self.CheckTypeVarName[31][4] = "Regime".to_owned();
      self.CheckCategory[31] = 4;
      self.CheckTypeNames[32] = "CheckPowerPointsInArea".to_owned();
      self.CheckTypeVarCount[32] = 4;
      self.CheckTypeVarName[32][1] = "AreaSlot".to_owned();
      self.CheckTypeVarName[32][2] = "AreaCode".to_owned();
      self.CheckTypeVarName[32][3] = "Regime".to_owned();
      self.CheckTypeVarName[32][4] = "Land only (0=no check, 1=land theater only)".to_owned();
      self.CheckCategory[32] = 4;
      self.CheckTypeNames[33] = "CheckSFTypeInXY".to_owned();
      self.CheckTypeVarCount[33] = 4;
      self.CheckTypeVarName[33][1] = "X".to_owned();
      self.CheckTypeVarName[33][2] = "Y".to_owned();
      self.CheckTypeVarName[33][3] = "SFType (-1=all)".to_owned();
      self.CheckTypeVarName[33][4] = "Regime".to_owned();
      self.CheckCategory[33] = 4;
      self.CheckTypeNames[34] = "CheckRegimeKills".to_owned();
      self.CheckTypeVarCount[34] = 4;
      self.CheckTypeVarName[34][1] = "Regime".to_owned();
      self.CheckTypeVarName[34][2] = "SFType (-1=all, 1000-10XX = SFTypeGroups 0-XX)".to_owned();
      self.CheckTypeVarName[34][3] = "Round (-1=all rounds)".to_owned();
      self.CheckTypeVarName[34][4] = "Report 0=qty, 1=power pts".to_owned();
      self.CheckCategory[34] = 2;
      self.CheckTypeNames[35] = "CheckRegimeLosses".to_owned();
      self.CheckTypeVarCount[35] = 4;
      self.CheckTypeVarName[35][1] = "Regime".to_owned();
      self.CheckTypeVarName[35][2] = "SFType (-1=all, 1000-100X = SFTypeGroups 0-XX)".to_owned();
      self.CheckTypeVarName[35][3] = "Round (-1=all rounds)".to_owned();
      self.CheckTypeVarName[35][4] = "Report 0=qty, 1=power pts".to_owned();
      self.CheckCategory[35] = 2;
      self.CheckTypeNames[36] = "CheckRegimeProduction".to_owned();
      self.CheckTypeVarCount[36] = 4;
      self.CheckTypeVarName[36][1] = "Regime".to_owned();
      self.CheckTypeVarName[36][2] = "SFType (-1=all)".to_owned();
      self.CheckTypeVarName[36][3] = "Round (-1=all rounds)".to_owned();
      self.CheckTypeVarName[36][4] = "Report 0=qty, 1=power pts".to_owned();
      self.CheckCategory[36] = 2;
      self.CheckTypeNames[37] = "CheckRegimeCount".to_owned();
      self.CheckTypeVarCount[37] = 0;
      self.CheckCategory[37] = 1;
      self.CheckTypeNames[38] = "CheckIfNeighbour".to_owned();
      self.CheckTypeVarCount[38] = 4;
      self.CheckTypeVarName[38][1] = "X".to_owned();
      self.CheckTypeVarName[38][2] = "Y".to_owned();
      self.CheckTypeVarName[38][3] = "X2".to_owned();
      self.CheckTypeVarName[38][4] = "Y2".to_owned();
      self.CheckCategory[38] = 3;
      self.CheckTypeNames[39] = "CheckDistance".to_owned();
      self.CheckTypeVarCount[39] = 4;
      self.CheckTypeVarName[39][1] = "X".to_owned();
      self.CheckTypeVarName[39][2] = "Y".to_owned();
      self.CheckTypeVarName[39][3] = "X2".to_owned();
      self.CheckTypeVarName[39][4] = "Y2".to_owned();
      self.CheckCategory[39] = 3;
      self.CheckTypeNames[40] = "CheckSFTypeGroupInXY".to_owned();
      self.CheckTypeVarCount[40] = 4;
      self.CheckTypeVarName[40][1] = "X".to_owned();
      self.CheckTypeVarName[40][2] = "Y".to_owned();
      self.CheckTypeVarName[40][3] = "SFType Group (-1=all)";
      self.CheckTypeVarName[40][4] = "Regime".to_owned();
      self.CheckCategory[40] = 4;
      self.CheckTypeNames[42] = "CheckAIThinkSpeed".to_owned();
      self.CheckTypeVarCount[42] = 1;
      self.CheckTypeVarName[42][1] = "Regime".to_owned();
      self.CheckCategory[42] = 2;
      self.CheckDesc[42] = "Gives back the AI think speed level. 0=Normal, 100=Slow, 250=Very Slow. (in old AT this was the production bonus- if ever merge keep this in mind)";
      self.CheckTypeNames[43] = "CheckTotalUnits".to_owned();
      self.CheckTypeVarCount[43] = 0;
      self.CheckCategory[43] = 5;
      self.CheckTypeNames[44] = "CheckUnitOwner".to_owned();
      self.CheckTypeVarCount[44] = 1;
      self.CheckTypeVarName[44][1] = "Unit Nr";
      self.CheckCategory[44] = 5;
      self.CheckTypeNames[45] = "CheckUnitHQ".to_owned();
      self.CheckTypeVarCount[45] = 1;
      self.CheckTypeVarName[45][1] = "Unit Nr";
      self.CheckCategory[45] = 5;
      self.CheckTypeNames[46] = "CheckUnitX".to_owned();
      self.CheckTypeVarCount[46] = 1;
      self.CheckTypeVarName[46][1] = "Unit Nr";
      self.CheckCategory[46] = 5;
      self.CheckTypeNames[47] = "CheckUnitY".to_owned();
      self.CheckTypeVarCount[47] = 1;
      self.CheckTypeVarName[47][1] = "Unit Nr";
      self.CheckCategory[47] = 5;
      self.CheckTypeNames[48] = "CheckUnitHistoricalID".to_owned();
      self.CheckTypeVarCount[48] = 1;
      self.CheckTypeVarName[48][1] = "Unit Nr";
      self.CheckCategory[48] = 5;
      self.CheckTypeNames[49] = "CheckUnitIsHQ".to_owned();
      self.CheckTypeVarCount[49] = 1;
      self.CheckTypeVarName[49][1] = "Unit Nr";
      self.CheckCategory[49] = 5;
      self.CheckTypeNames[50] = "CheckRegimeRelation".to_owned();
      self.CheckTypeVarCount[50] = 2;
      self.CheckTypeVarName[50][1] = "Reg1".to_owned();
      self.CheckTypeVarName[50][2] = "Reg2".to_owned();
      self.CheckCategory[50] = 2;
      self.CheckDesc[50] = "Returns 0 for war, 1 for peace, 2 for alliance";
      self.CheckTypeNames[51] = "CheckUnitInHQChain".to_owned();
      self.CheckTypeVarCount[51] = 2;
      self.CheckTypeVarName[51][1] = "Unit Nr";
      self.CheckTypeVarName[51][2] = "HQ Nr";
      self.CheckCategory[51] = 5;
      self.CheckTypeNames[52] = "CheckUnitCommanderStaffPercent".to_owned();
      self.CheckTypeVarCount[52] = 1;
      self.CheckTypeVarName[52][1] = "Unit Nr";
      self.CheckCategory[52] = 5;
      self.CheckTypeNames[53] = "CheckUnitHQPower".to_owned();
      self.CheckTypeVarCount[53] = 1;
      self.CheckTypeVarName[53][1] = "Unit Nr";
      self.CheckCategory[53] = 5;
      self.CheckTypeNames[54] = "CheckFirstUnitWithHistoricalID".to_owned();
      self.CheckTypeVarCount[54] = 1;
      self.CheckTypeVarName[54][1] = "Historical ID";
      self.CheckCategory[54] = 5;
      self.CheckTypeNames[55] = "CheckLocTypeXY".to_owned();
      self.CheckTypeVarCount[55] = 2;
      self.CheckTypeVarName[55][1] = "X".to_owned();
      self.CheckTypeVarName[55][2] = "Y".to_owned();
      self.CheckCategory[55] = 4;
      self.CheckTypeNames[56] = "CheckStrucPtsXY".to_owned();
      self.CheckTypeVarCount[56] = 2;
      self.CheckTypeVarName[56][1] = "X".to_owned();
      self.CheckTypeVarName[56][2] = "Y".to_owned();
      self.CheckCategory[56] = 4;
      self.CheckTypeNames[57] = "CheckClosestCommanderUnit".to_owned();
      self.CheckTypeVarCount[57] = 3;
      self.CheckTypeVarName[57][1] = "X".to_owned();
      self.CheckTypeVarName[57][2] = "Y".to_owned();
      self.CheckTypeVarName[57][3] = "Regime".to_owned();
      self.CheckCategory[57] = 5;
      self.CheckTypeNames[58] = "CheckStringList".to_owned();
      self.CheckTypeVarCount[58] = 3;
      self.CheckTypeVarName[58][1] = "Stringlist ID";
      self.CheckTypeVarName[58][2] = "Row".to_owned();
      self.CheckTypeVarName[58][3] = "Col".to_owned();
      self.CheckCategory[58] = 6;
      self.CheckTypeNames[59] = "CheckStringReplace".to_owned();
      self.CheckTypeVarCount[59] = 3;
      self.CheckTypeVarName[59][1] = "From String";
      self.CheckTypeVarName[59][2] = "Which substring";
      self.CheckTypeVarName[59][3] = "Replace with what";
      self.CheckCategory[59] = 6;
      self.CheckTypeNames[60] = "CheckRegimeVar".to_owned();
      self.CheckTypeVarCount[60] = 2;
      self.CheckTypeVarName[60][1] = "Regime Nr";
      self.CheckTypeVarName[60][2] = "Var Nr";
      self.CheckCategory[60] = 2;
      self.CheckTypeNames[61] = "CheckHistoricalUnitTempVar".to_owned();
      self.CheckTypeVarCount[61] = 3;
      self.CheckTypeVarName[61][1] = "Unit Nr (or -1 to use hist.)";
      self.CheckTypeVarName[61][2] = "TempVar Nr (1-5), 6=AiZoneListNr";
      self.CheckTypeVarName[61][3] = "Use Historical Unit ID";
      self.CheckCategory[61] = 5;
      self.CheckTypeNames[62] = "CheckCommanderDescript/Name";
      self.CheckTypeVarCount[62] = 3;
      self.CheckTypeVarName[62][1] = "Unit Nr";
      self.CheckTypeVarName[62][2] = "0=Descript, 1=Name";
      self.CheckTypeVarName[62][3] = "HisUnit ID overrule (<=0 is no overrule)";
      self.CheckCategory[62] = 5;
      self.CheckTypeNames[63] = "CheckCommanderXP".to_owned();
      self.CheckTypeVarCount[63] = 4;
      self.CheckTypeVarName[63][1] = "Unit Nr (or -1)";
      self.CheckTypeVarName[63][2] = "Overrule His Unit ID (or -1)";
      self.CheckTypeVarName[63][3] = "0=Xp, 1=Total Cards, 2=Specific Card, 3=Total Ev, 4=Spec Ev";
      self.CheckTypeVarName[63][4] = "Opt: card or ev#";
      self.CheckCategory[63] = 5;
      self.CheckTypeNames[64] = "CheckStringListRows".to_owned();
      self.CheckTypeVarCount[64] = 1;
      self.CheckTypeVarName[64][1] = "StringList ID";
      self.CheckCategory[64] = 6;
      self.CheckTypeNames[65] = "CheckStringListCols".to_owned();
      self.CheckTypeVarCount[65] = 1;
      self.CheckTypeVarName[65][1] = "StringList ID";
      self.CheckCategory[65] = 6;
      self.CheckTypeNames[66] = "CheckStringListLastID".to_owned();
      self.CheckTypeVarCount[66] = 0;
      self.CheckCategory[66] = 6;
      self.CheckTypeNames[67] = "CheckUnitRdn".to_owned();
      self.CheckTypeVarCount[67] = 1;
      self.CheckTypeVarName[67][1] = "Unit#";
      self.CheckDesc[68] = "Keep in mind this is an average of the subformations.";
      self.CheckCategory[67] = 5;
      self.CheckTypeNames[68] = "CheckUnitSFType".to_owned();
      self.CheckTypeVarCount[68] = 3;
      self.CheckTypeVarName[68][1] = "Unit #";
      self.CheckTypeVarName[68][2] = "SFType Group (-1 for none)";
      self.CheckTypeVarName[68][3] = "Specific SFType (-1 for none)";
      self.CheckDesc[68] = "Returns the number of individuals in this unit";
      self.CheckCategory[68] = 5;
      self.CheckTypeNames[69] = "ReturnCRLF".to_owned();
      self.CheckTypeVarCount[69] = 0;
      self.CheckCategory[69] = 3;
      self.CheckTypeNames[70] = "CheckUnitName".to_owned();
      self.CheckTypeVarCount[70] = 1;
      self.CheckTypeVarName[70][1] = "Unit#";
      self.CheckCategory[70] = 5;
      self.CheckTypeNames[71] = "CheckRegimeUnitsMorale".to_owned();
      self.CheckTypeVarCount[71] = 1;
      self.CheckTypeVarName[71][1] = "Reg".to_owned();
      self.CheckCategory[71] = 2;
      self.CheckDesc[71] = "Returns the average morale of all units on map for this regime";
      self.CheckTypeNames[72] = "CheckRegimeUnitsExperience".to_owned();
      self.CheckTypeVarCount[72] = 1;
      self.CheckTypeVarName[72][1] = "Reg".to_owned();
      self.CheckCategory[72] = 2;
      self.CheckDesc[72] = "Returns the average experience of all units on map for this regime";
      self.CheckTypeNames[73] = "CheckUnitTempOwner".to_owned();
      self.CheckTypeVarCount[73] = 1;
      self.CheckTypeVarName[73][1] = "Unit Nr";
      self.CheckCategory[73] = 5;
      self.CheckDesc[73] = "Returns the real owner. If this is a subregime it will actually give back the subregime regime # instead of the uberregime which the normal CheckUnitOwner will return.";
      self.ExecTypeNames[74] = "CheckPreDef".to_owned();
      self.ExecTypeVarCount[74] = 1;
      self.ExecTypeVarName[74][1] = "Predef#";
      self.ExecDesc[74] = "Returns unit# of this predef ID#.";
      self.ExecCategory[74] = 5;
      self.CheckTypeNames[75] = "CheckUnitPower".to_owned();
      self.CheckTypeVarCount[75] = 1;
      self.CheckTypeVarName[75][1] = "Unit Nr";
      self.CheckCategory[75] = 5;
      self.CheckDesc[75] = "Returns the real total power points in the unit.";
      self.CheckTypeNames[76] = "CheckSpecialType".to_owned();
      self.CheckTypeVarCount[76] = 2;
      self.CheckTypeVarName[76][1] = "X".to_owned();
      self.CheckTypeVarName[76][2] = "Y".to_owned();
      self.CheckCategory[76] = 4;
      self.CheckTypeNames[77] = "CheckSpecialSprite".to_owned();
      self.CheckTypeVarCount[77] = 2;
      self.CheckTypeVarName[77][1] = "X".to_owned();
      self.CheckTypeVarName[77][2] = "Y".to_owned();
      self.CheckCategory[77] = 4;
      self.CheckTypeNames[78] = "CheckUnitSelectable".to_owned();
      self.CheckTypeVarCount[78] = 1;
      self.CheckTypeVarName[78][1] = "Unit Nr";
      self.CheckCategory[78] = 5;
      self.CheckDesc[78] = "Returns 0 if not selectable. returns 1 if selectable.";
      self.CheckTypeNames[79] = "DistanceClosestEnemyHex".to_owned();
      self.CheckTypeVarCount[79] = 4;
      self.CheckTypeVarName[79][1] = "Minimum (AI)VP value? 0=any";
      self.CheckTypeVarName[79][2] = "from X";
      self.CheckTypeVarName[79][3] = "from Y";
      self.CheckTypeVarName[79][4] = "Max distance (-1=no maximum)";
      self.CheckCategory[79] = 3;
      self.CheckDesc[79] = "Returns the distance. Max distance can be set to keep processing time down if necc.";
      self.CheckTypeNames[80] = "CheckEnemyTroopsCloseBy".to_owned();
      self.CheckTypeVarCount[80] = 4;
      self.CheckTypeVarName[80][1] = "Max Distance";
      self.CheckTypeVarName[80][2] = "from X";
      self.CheckTypeVarName[80][3] = "from Y";
      self.CheckTypeVarName[80][4] = "Distance divider modifier (0=none)";
      self.CheckCategory[80] = 3;
      self.CheckDesc[80] = "Max distance sets circle in which too look. Distance divider modifier set by what the power points of a unit are divided. if 0 they are not divided. if 1 they are divided by distance. if 2 by double distance.. etc..";
      self.CheckTypeNames[81] = "CheckUnitMor".to_owned();
      self.CheckTypeVarCount[81] = 1;
      self.CheckTypeVarName[81][1] = "Unit#";
      self.CheckDesc[81] = "Keep in mind this is an average of the subformations.";
      self.CheckCategory[81] = 5;
      self.CheckTypeNames[82] = "OBSOLETE".to_owned();
      self.CheckTypeVarCount[82] = 1;
      self.CheckTypeVarName[82][1] = "SFType#";
      self.CheckCategory[82] = -1;
      self.CheckTypeNames[83] = "CheckSFTypePower".to_owned();
      self.CheckTypeVarCount[83] = 3;
      self.CheckTypeVarName[83][1] = "SFType#";
      self.CheckTypeVarName[83][2] = "SFType Targetgroup";
      self.CheckTypeVarName[83][3] = "1=attack score, 2=defend score";
      self.CheckCategory[83] = 7;
      self.CheckTypeNames[84] = "CheckSFTypeDescription".to_owned();
      self.CheckTypeVarCount[84] = 1;
      self.CheckTypeVarName[84][1] = "SFType#";
      self.CheckCategory[84] = 7;
      self.CheckTypeNames[85] = "CheckSFTypeHP".to_owned();
      self.CheckTypeVarCount[85] = 3;
      self.CheckTypeVarName[85][1] = "SFType#";
      self.CheckTypeVarName[85][2] = "SFType Targetgroup";
      self.CheckTypeVarName[85][3] = "1=attack hp, 2=defend hp";
      self.CheckCategory[85] = 7;
      self.CheckTypeNames[86] = "CheckSFTypeMoveType".to_owned();
      self.CheckTypeVarCount[86] = 1;
      self.CheckTypeVarName[86][1] = "SFType#";
      self.CheckCategory[86] = 7;
      self.CheckTypeNames[87] = "CheckSFTypeMoveRedux".to_owned();
      self.CheckTypeVarCount[87] = 1;
      self.CheckTypeVarName[87][1] = "SFType#";
      self.CheckCategory[87] = 7;
      self.CheckTypeNames[88] = "CheckSFTypeModelLastState".to_owned();
      self.CheckTypeVarCount[88] = 2;
      self.CheckTypeVarName[88][1] = "SFType#";
      self.CheckTypeVarName[88][2] = "Research#";
      self.CheckCategory[88] = 7;
      self.CheckTypeNames[89] = "CheckSFTypeLogo".to_owned();
      self.CheckTypeVarCount[89] = 2;
      self.CheckTypeVarName[89][1] = "SFType#";
      self.CheckTypeVarName[89][2] = "Logo#";
      self.CheckCategory[89] = 7;
      self.CheckTypeNames[92] = "CheckSFTypeFuelUse".to_owned();
      self.CheckTypeVarCount[92] = 2;
      self.CheckTypeVarName[92][1] = "SFtype#";
      self.CheckTypeVarName[92][2] = "1=ForMovement, 2=ForCombat";
      self.CheckCategory[92] = 7;
      self.CheckTypeNames[93] = "CheckGameVar".to_owned();
      self.CheckTypeVarCount[93] = 1;
      self.CheckTypeVarName[93][1] = "Nr".to_owned();
      self.CheckCategory[93] = 3;
      self.CheckTypeNames[94] = "CheckTempVar".to_owned();
      self.CheckTypeVarCount[94] = 1;
      self.CheckTypeVarName[94][1] = "Nr".to_owned();
      self.CheckCategory[94] = 3;
      self.CheckTypeNames[95] = "CheckRegimePeople".to_owned();
      self.CheckTypeVarCount[95] = 1;
      self.CheckTypeVarName[95][1] = "Regime".to_owned();
      self.CheckCategory[95] = 2;
      self.CheckTypeNames[96] = "CheckSFTypeVar".to_owned();
      self.CheckTypeVarCount[96] = 2;
      self.CheckTypeVarName[96][1] = "SFType".to_owned();
      self.CheckTypeVarName[96][2] = "Var#";
      self.CheckCategory[96] = 7;
      self.CheckTypeNames[97] = "CheckSFTypeItem".to_owned();
      self.CheckTypeVarCount[97] = 1;
      self.CheckTypeVarName[97][1] = "SFType#";
      self.CheckCategory[97] = 7;
      self.CheckTypeNames[98] = "CheckHexVP".to_owned();
      self.CheckTypeVarCount[98] = 2;
      self.CheckTypeVarName[98][1] = "X".to_owned();
      self.CheckTypeVarName[98][2] = "Y".to_owned();
      self.CheckCategory[98] = 4;
      self.CheckTypeNames[99] = "CheckDipOffer".to_owned();
      self.CheckTypeVarCount[99] = 2;
      self.CheckTypeVarName[99][1] = "From Regime";
      self.CheckTypeVarName[99][2] = "Too Regime";
      self.CheckCategory[99] = 2;
      self.CheckTypeNames[101] = "CheckDefinedAreaID".to_owned();
      self.CheckTypeVarCount[101] = 2;
      self.CheckTypeVarName[101][1] = "Slot#(0-9)";
      self.CheckTypeVarName[101][2] = "Value#";
      self.CheckCategory[101] = 3;
      self.CheckDesc[101] = "Returns first Defined area with that slot&value coupled to it.";
      self.CheckTypeNames[102] = "CheckHexPeople".to_owned();
      self.CheckTypeVarCount[102] = 2;
      self.CheckTypeVarName[102][1] = "X".to_owned();
      self.CheckTypeVarName[102][2] = "Y".to_owned();
      self.CheckCategory[102] = 4;
      self.CheckDesc[102] = "Returns people number of location here. if any. if not returns -1.";
      self.CheckTypeNames[104] = "CheckNonSleepingRegimesToCome".to_owned();
      self.CheckTypeVarCount[104] = 0;
      self.CheckCategory[104] = 2;
      self.CheckDesc[104] = "Returns the number of regimes that are still to be played in this round that are not put to sleep.";
      self.CheckTypeNames[108] = "CheckCalcGetPercent".to_owned();
      self.CheckTypeVarCount[108] = 2;
      self.CheckTypeVarName[108][1] = "Var X";
      self.CheckTypeVarName[108][2] = "of Var Y";
      self.CheckCategory[108] = 3;
      self.CheckDesc[108] = "Returns the percentage that var X makes up of var Y.";
      self.CheckTypeNames[109] = "CheckLocCounter".to_owned();
      self.CheckTypeVarCount[109] = 0;
      self.CheckCategory[109] = 3;
      self.CheckDesc[109] = "Returns the number of locations. remember counting starts at 0.";
      self.CheckTypeNames[110] = "CheckLocX".to_owned();
      self.CheckTypeVarCount[110] = 1;
      self.CheckTypeVarName[110][1] = "Location#";
      self.CheckCategory[110] = 3;
      self.CheckDesc[110] = "Returns the number of locations. remember counting starts at 0.";
      self.CheckTypeNames[111] = "CheckLocY".to_owned();
      self.CheckTypeVarCount[111] = 1;
      self.CheckTypeVarName[111][1] = "Location#";
      self.CheckCategory[111] = 3;
      self.CheckDesc[111] = "Returns the number of locations. remember counting starts at 0.";
      self.CheckTypeNames[113] = "CheckMaxFuelUse".to_owned();
      self.CheckTypeVarCount[113] = 2;
      self.CheckTypeVarName[113][1] = "Regime (-1=all)";
      self.CheckTypeVarName[113][2] = "Fuel Type (-1=all)";
      self.CheckCategory[113] = 8;
      self.CheckTypeNames[115] = "CheckHexName".to_owned();
      self.CheckTypeVarCount[115] = 2;
      self.CheckTypeVarName[115][1] = "X".to_owned();
      self.CheckTypeVarName[115][2] = "Y".to_owned();
      self.CheckCategory[115] = 4;
      self.CheckDesc[115] = "";
      self.CheckTypeNames[116] = "CheckLocName".to_owned();
      self.CheckTypeVarCount[116] = 1;
      self.CheckTypeVarName[116][1] = "LocNr".to_owned();
      self.CheckCategory[116] = 4;
      self.CheckDesc[116] = "";
      self.CheckTypeNames[117] = "CheckHexLocNr".to_owned();
      self.CheckTypeVarCount[117] = 2;
      self.CheckTypeVarName[117][1] = "X".to_owned();
      self.CheckTypeVarName[117][2] = "Y".to_owned();
      self.CheckCategory[117] = 4;
      self.CheckDesc[117] = "Returns -1 if no Location in hex. 0 or higher if present.";
      self.CheckTypeNames[118] = "CheckHexUnitCounter".to_owned();
      self.CheckTypeVarCount[118] = 2;
      self.CheckTypeVarName[118][1] = "X".to_owned();
      self.CheckTypeVarName[118][2] = "Y".to_owned();
      self.CheckCategory[118] = 5;
      self.CheckDesc[118] = "Returns -1 if no units. Returns 0 is one unit (in slot 0). Returns 1 if two units present (one in slot 0, one in slot1), etc..";
      self.CheckTypeNames[119] = "CheckHexUnit".to_owned();
      self.CheckTypeVarCount[119] = 3;
      self.CheckTypeVarName[119][1] = "X".to_owned();
      self.CheckTypeVarName[119][2] = "Y".to_owned();
      self.CheckTypeVarName[119][3] = "Unit Slot";
      self.CheckCategory[119] = 5;
      self.CheckDesc[119] = "You should use CheckHexUnitCounter to determine the number of units on the hex. Start with slot=0! ";
      self.CheckTypeNames[120] = "CheckStringLength".to_owned();
      self.CheckTypeVarCount[120] = 1;
      self.CheckTypeVarName[120][1] = "String".to_owned();
      self.CheckCategory[120] = 3;
      self.CheckDesc[120] = "Return of 0 means an empty has: String been tested.";
      self.CheckTypeNames[121] = "CheckUnitStaffPercent".to_owned();
      self.CheckTypeVarCount[121] = 2;
      self.CheckTypeVarName[121][1] = "Unit Nr";
      self.CheckTypeVarName[121][2] = "CheckMode=0,1,2";
      self.CheckCategory[121] = 5;
      self.CheckDesc[121] = "Return the staff % of the unit... you might want to use this check in combination with CheckUnitHqPower since distance is not checked for in this event. Check Mode 0 returns staff%. Check Mode 1 returns combat mod%, Check Mode 2 returns morale mod%. ";
      self.CheckTypeNames[122] = "CheckSFTypeRatio".to_owned();
      self.CheckTypeVarCount[122] = 1;
      self.CheckTypeVarName[122][1] = "SFType#";
      self.CheckCategory[122] = 7;
      self.CheckTypeNames[123] = "CheckSFTypeName".to_owned();
      self.CheckTypeVarCount[123] = 1;
      self.CheckTypeVarName[123][1] = "SFType#";
      self.CheckCategory[123] = 7;
      self.CheckTypeNames[124] = "CheckRoadType".to_owned();
      self.CheckTypeVarCount[124] = 3;
      self.CheckTypeVarName[124][1] = "X".to_owned();
      self.CheckTypeVarName[124][2] = "Y".to_owned();
      self.CheckTypeVarName[124][3] = "Direction".to_owned();
      self.CheckCategory[124] = 4;
      self.CheckDesc[124] = "Direction: 0=n, 1=ne, 2=se, 3=s, 4=sw, 5=nw. Use direction -1 to test for all directions: first type found is returned. -1 is returned if none.";
      self.CheckTypeNames[125] = "CheckRiverType".to_owned();
      self.CheckTypeVarCount[125] = 3;
      self.CheckTypeVarName[125][1] = "X".to_owned();
      self.CheckTypeVarName[125][2] = "Y".to_owned();
      self.CheckTypeVarName[125][3] = "Direction".to_owned();
      self.CheckCategory[125] = 4;
      self.CheckDesc[125] = "Direction: 0=n, 1=ne, 2=se, 3=s, 4=sw, 5=nw. Use direction -1 to test for all directions: first type found is returned. -1 is returned if none.";
      self.CheckTypeNames[126] = "CheckBridge".to_owned();
      self.CheckTypeVarCount[126] = 3;
      self.CheckTypeVarName[126][1] = "X".to_owned();
      self.CheckTypeVarName[126][2] = "Y".to_owned();
      self.CheckTypeVarName[126][3] = "Direction".to_owned();
      self.CheckCategory[126] = 4;
      self.CheckDesc[126] = "Direction: 0=n, 1=ne, 2=se, 3=s, 4=sw, 5=nw. Use direction -1 to test for all directions. Returns 0 for no bridge and 1 for yes bridge.";
      self.CheckTypeNames[ sbyte.MaxValue] = "CheckHisVar".to_owned();
      self.CheckTypeVarCount[ sbyte.MaxValue] = 2;
      self.CheckTypeVarName[ sbyte.MaxValue, 1] = "His Unit ID#";
      self.CheckTypeVarName[ sbyte.MaxValue, 2] = "Type#";
      self.CheckCategory[ sbyte.MaxValue] = 5;
      self.CheckDesc[ sbyte.MaxValue] = "Returns value if type is present. If type is not present it returns 0.";
      self.CheckTypeNames[128] = "CheckUnitRecon".to_owned();
      self.CheckTypeVarCount[128] = 4;
      self.CheckTypeVarName[128][1] = "Unit#";
      self.CheckTypeVarName[128][2] = "Include land theater 0=no, 1=yes";
      self.CheckTypeVarName[128][3] = "Include sea theater 0=no, 1=yes";
      self.CheckTypeVarName[128][4] = "Include air theater 0=no, 1=yes";
      self.CheckDesc[128] = "Returns total recon points in the unit.";
      self.CheckCategory[128] = 5;
      self.CheckTypeNames[129] = "CheckRoadTypeSpecific".to_owned();
      self.CheckTypeVarCount[129] = 3;
      self.CheckTypeVarName[129][1] = "X".to_owned();
      self.CheckTypeVarName[129][2] = "Y".to_owned();
      self.CheckTypeVarName[129][3] = "Specific Road Type";
      self.CheckCategory[129] = 4;
      self.CheckDesc[129] = "Check is specied roadtype is present in this hex. Returns 1 if is. Returns 0 if not.";
      self.CheckTypeNames[130] = "CheckHistoricalUnitAverageXP".to_owned();
      self.CheckTypeVarCount[130] = 1;
      self.CheckTypeVarName[130][1] = "Unit#";
      self.CheckCategory[130] = 5;
      self.CheckDesc[130] = "Will look up other units on map with the same historical unit and return average xp";
      self.CheckTypeNames[131] = "CheckRandomRowStringList".to_owned();
      self.CheckTypeVarCount[131] = 2;
      self.CheckTypeVarName[131][1] = "Stringlist ID";
      self.CheckTypeVarName[131][2] = "Col used as weight (-1=none)";
      self.CheckCategory[131] = 6;
      self.CheckTypeNames[132] = "CheckFindInStringList".to_owned();
      self.CheckTypeVarCount[132] = 3;
      self.CheckTypeVarName[132][1] = "Stringlist ID";
      self.CheckTypeVarName[132][2] = "In Col";
      self.CheckTypeVarName[132][3] = "Find Value";
      self.CheckCategory[132] = 6;
      self.CheckDesc[132] = "Returns row# where value is found or -1 if not.";
      self.CheckTypeNames[133] = "CheckTotalHistoricalUnits".to_owned();
      self.CheckTypeVarCount[133] = 0;
      self.CheckCategory[133] = 5;
      self.CheckTypeNames[134] = "CheckGetHistoricalUnitID".to_owned();
      self.CheckTypeVarCount[134] = 1;
      self.CheckTypeVarName[134][1] = "Slot#";
      self.CheckCategory[134] = 5;
      self.CheckDesc[134] = "Keep in mind that this function should be used to scroll through all historical units using a looper from 0 to CheckTotalHistoricalUnits. Use this function to get the actual ID to be used in other functions.";
      self.CheckTypeNames[135] = "CheckGetHistoricalUnitRegime".to_owned();
      self.CheckTypeVarCount[135] = 1;
      self.CheckTypeVarName[135][1] = "Historical Unit ID#";
      self.CheckCategory[135] = 5;
      self.CheckTypeNames[136] = "CheckPeopleUnderHQ".to_owned();
      self.CheckTypeVarCount[136] = 4;
      self.CheckTypeVarName[136][1] = "HQ level to check";
      self.CheckTypeVarName[136][2] = "Troop people";
      self.CheckTypeVarName[136][3] = "HQ people";
      self.CheckTypeVarName[136][4] = "regime#";
      self.CheckCategory[136] = 5;
      self.CheckDesc[136] = "A complicated function. HQ level 1 is HQs with historical unit set to Corps. HQ level 2 is only HQs with historical HQ set to Army, 3=armygroup, 4=high command.. Function returns % of total power points of people on map is under a hq whose historical unit people is set to hqppl. This function could for example be used to check if axis minor are under wehrmacht HQs or under their own.";
      self.CheckTypeNames[137] = "CheckGetHistoricalUnitPeople".to_owned();
      self.CheckTypeVarCount[137] = 1;
      self.CheckTypeVarName[137][1] = "Historical Unit ID#";
      self.CheckCategory[137] = 5;
      self.CheckTypeNames[138] = "CheckGetHistoricalUnitType".to_owned();
      self.CheckTypeVarCount[138] = 1;
      self.CheckTypeVarName[138][1] = "Historical Unit ID#";
      self.CheckCategory[138] = 5;
      self.CheckDesc[138] = "Returns the type. 5=corps, 6=army, etc..";
      self.CheckTypeNames[139] = "CheckGetHistoricalUnitOfficer".to_owned();
      self.CheckTypeVarCount[139] = 1;
      self.CheckTypeVarName[139][1] = "Historical Unit ID#";
      self.CheckCategory[139] = 5;
      self.CheckDesc[139] = "returns 1 if officer, returns 0 if not. Basicly the length of officer name is checked.";
      self.CheckTypeNames[140] = "CheckCompareString".to_owned();
      self.CheckTypeVarCount[140] = 3;
      self.CheckTypeVarName[140][1] = "String1".to_owned();
      self.CheckTypeVarName[140][2] = "String2".to_owned();
      self.CheckTypeVarName[140][3] = "Mode".to_owned();
      self.CheckCategory[140] = 3;
      self.CheckDesc[140] = "Case insensitive. Mode=1 means it looks if the strings are exactly alike. Mode=2 same but all spaces are removed. Mode=3 means it looks if part of string1 is equal to 2: String. Function returns 1 if equal or 0 if not.";
      self.CheckTypeNames[141] = "CheckMin".to_owned();
      self.CheckTypeVarCount[141] = 2;
      self.CheckTypeVarName[141][1] = "Value1".to_owned();
      self.CheckTypeVarName[141][2] = "Value2".to_owned();
      self.CheckTypeVarName[141][3] = "Mode".to_owned();
      self.CheckCategory[141] = 3;
      self.CheckDesc[141] = "Returns the lowest of the 2 values entered";
      self.CheckTypeNames[142] = "CheckMax".to_owned();
      self.CheckTypeVarCount[142] = 2;
      self.CheckTypeVarName[142][1] = "Value1".to_owned();
      self.CheckTypeVarName[142][2] = "Value2".to_owned();
      self.CheckTypeVarName[142][3] = "Mode".to_owned();
      self.CheckCategory[142] = 3;
      self.CheckDesc[142] = "Returns the highest of the 2 values entered";
      self.CheckTypeNames[143] = "CheckGetHistoricalUnitName".to_owned();
      self.CheckTypeVarCount[143] = 1;
      self.CheckTypeVarName[143][1] = "Historical Unit ID#";
      self.CheckCategory[143] = 5;
      self.CheckTypeNames[144] = "CheckGetHistoricalSubparts".to_owned();
      self.CheckTypeVarCount[144] = 1;
      self.CheckTypeVarName[144][1] = "Historical Unit ID#";
      self.CheckCategory[144] = 5;
      self.CheckTypeNames[147] = "CheckPbemPlayer".to_owned();
      self.CheckTypeVarCount[147] = 1;
      self.CheckTypeVarName[147][1] = "Regime#";
      self.CheckCategory[147] = 2;
      self.CheckDesc[147] = "Expert use only! Returns either 1 for pbem player 1 (challenger), 2 for pbem player 2(opponent) or 0 if not set (not a started pbem++ game for example)";
      self.CheckTypeNames[148] = "CheckDrawGame".to_owned();
      self.CheckTypeVarCount[148] = 0;
      self.CheckCategory[148] = 1;
      self.CheckDesc[148] = "Returns 0 if not. Returns 1 if this is a draw game.";
      self.CheckTypeNames[146] = "CheckUnitAP".to_owned();
      self.CheckTypeVarCount[146] = 1;
      self.CheckTypeVarName[146][1] = "Unit#";
      self.CheckDesc[146] = "Keep in mind this is the lowest AP subformations.";
      self.CheckCategory[146] = 5;
      self.CheckTypeNames[145] = "CheckTotalSupplyUse".to_owned();
      self.CheckTypeVarCount[145] = 1;
      self.CheckTypeVarName[145][1] = "Regime #";
      self.CheckCategory[145] = 2;
      self.CheckDesc[145] = "Returns the BasicSupplyUse of all troops on the map added up.";
      self.CheckTypeNames[149] = "CheckTotalActioncards".to_owned();
      self.CheckTypeVarCount[149] = 3;
      self.CheckTypeVarName[149][1] = "Regime #";
      self.CheckTypeVarName[149][2] = "Category # (-1=all)";
      self.CheckTypeVarName[149][3] = "Color # (-1=all)";
      self.CheckCategory[149] = 2;
      self.CheckDesc[149] = "Returns the total number of action cards the regime has in hand. However if you set regime=-1 it returns the top action card slot in the actioncard list. This is usefull if your manually adding actioncards through events.";
      self.CheckTypeNames[150] = "CheckAICombatMod".to_owned();
      self.CheckTypeVarCount[150] = 1;
      self.CheckTypeVarName[150][1] = "Regime".to_owned();
      self.CheckCategory[150] = 2;
      self.CheckDesc[150] = "Gives back the AI combat modifier. 0=none, >0 is the percentual bonus. (This is the AI difficulty in the scenario setup screen)";
      self.CheckTypeNames[151] = "CheckIsUnitAPreDef".to_owned();
      self.CheckTypeVarCount[151] = 1;
      self.CheckTypeVarName[151][1] = "Unr#";
      self.CheckDesc[151] = "Returns -1 if not a predef. Returns the predef ID # if it is a predef.";
      self.CheckCategory[151] = 5;
      self.CheckTypeNames[152] = "CheckRuleVar".to_owned();
      self.CheckTypeVarCount[152] = 2;
      self.CheckTypeVarName[152][1] = "RuleVar#";
      self.CheckTypeVarName[152][2] = "Mulitply by 100? 0=no 1=yes";
      self.CheckDesc[152] = "Multiply option is there because some rulevar have single values like 0.4 or 0.03. And the scripting sytem can only handle integers.";
      self.CheckCategory[152] = 1;
      self.CheckTypeNames[153] = "CheckUnitXP".to_owned();
      self.CheckTypeVarCount[153] = 2;
      self.CheckTypeVarName[153][1] = "Unit#";
      self.CheckTypeVarName[153][2] = "Only if staff (0=no,1=yes)";
      self.CheckDesc[153] = "Keep in mind that it returns the highest XP found in any subformation.";
      self.CheckCategory[153] = 5;
      self.CheckTypeNames[154] = "CheckGetHistoricalUnitCommanderName".to_owned();
      self.CheckTypeVarCount[154] = 1;
      self.CheckTypeVarName[154][1] = "Historical Unit ID#";
      self.CheckCategory[154] = 5;
      self.CheckDesc[154] = "Returns the commandername (not the his.unit name!). If no commander empty is: String returned.";
      self.CheckTypeNames[155] = "CheckFindHistoricalUnitWithCommanderName".to_owned();
      self.CheckTypeVarCount[155] = 1;
      self.CheckTypeVarName[155][1] = "Commander Name";
      self.CheckCategory[155] = 5;
      self.CheckDesc[155] = "Returns the first historical unit ID number of a unit ON the map with this commander name present. -1= none find. ";
      self.CheckTypeNames[156] = "CheckUnitHQChanged".to_owned();
      self.CheckTypeVarCount[156] = 1;
      self.CheckTypeVarName[156][1] = "Unit Nr";
      self.CheckCategory[156] = 5;
      self.CheckDesc[156] = "Returns 1 if the unit HQ has been changed in this turn. Returns 0 if not.";
      self.CheckTypeNames[157] = "CheckHistoricalUnitAirAndNavy".to_owned();
      self.CheckTypeVarCount[157] = 3;
      self.CheckTypeVarName[157][1] = "His Unit ID#";
      self.CheckTypeVarName[157][2] = "Check for air theater troops? 0=no,1=yes";
      self.CheckTypeVarName[157][3] = "Check for navy theater troops? 0=no,1=yes";
      self.CheckCategory[157] = 5;
      self.CheckDesc[157] = "Returns 1 if those troops are present. 0 if not.";
      self.CheckTypeNames[158] = "CheckForLocationProperty".to_owned();
      self.CheckTypeVarCount[158] = 4;
      self.CheckTypeVarName[158][1] = "X".to_owned();
      self.CheckTypeVarName[158][2] = "Y".to_owned();
      self.CheckTypeVarName[158][3] = "1=airbase, 2=port(and sea next door)";
      self.CheckCategory[158] = 3;
      self.CheckDesc[158] = "Returns 1 if property is available, Returns 0 if not.";
      self.CheckTypeNames[159] = "CheckGetStringlistID".to_owned();
      self.CheckTypeVarCount[159] = 2;
      self.CheckTypeVarName[159][1] = "Library Name";
      self.CheckTypeVarName[159][2] = "Stringlist ID";
      self.CheckCategory[159] = 10;
      self.CheckDesc[159] = "Use when writing library events to get correct stringlist when library is used. This Check returns always the correct ID of the stringlist. ";
      self.CheckTypeNames[160] = "CheckLibVarGlobal".to_owned();
      self.CheckTypeVarCount[160] = 2;
      self.CheckTypeVarName[160][1] = "Library Name";
      self.CheckTypeVarName[160][2] = "Variable Name";
      self.CheckCategory[160] = 10;
      self.CheckDesc[160] = "Returns the value of specified global libvar. ";
      self.CheckTypeNames[161] = "CheckGetEventPic".to_owned();
      self.CheckTypeVarCount[161] = 2;
      self.CheckTypeVarName[161][1] = "Library Name";
      self.CheckTypeVarName[161][2] = "Orig eventPic SlotNr";
      self.CheckCategory[161] = 10;
      self.CheckDesc[161] = "Returns the current eventPic Nr. If you are writing library events that use eventpics you should always call this check first. ";
      self.CheckTypeNames[162] = "CheckGetSmallPic".to_owned();
      self.CheckTypeVarCount[162] = 2;
      self.CheckTypeVarName[162][1] = "Library Name";
      self.CheckTypeVarName[162][2] = "Orig smallPic SlotNr";
      self.CheckCategory[162] = 10;
      self.CheckDesc[162] = "Returns the current smallPic Nr.  If you are writing library events that use small pics you should always call this check first.  ";
      self.CheckTypeNames[163] = "CheckGetCurrentDateString".to_owned();
      self.CheckTypeVarCount[163] = 0;
      self.CheckCategory[163] = 3;
      self.CheckDesc[163] = "Returns a thats: String date formatted. Keep in mind that strings when compared using <,>,=, etc.. are compared as if they were dates.";
      self.CheckTypeNames[164] = "CheckGetDateString".to_owned();
      self.CheckTypeVarCount[164] = 0;
      self.CheckTypeVarCount[164] = 4;
      self.CheckTypeVarName[164][1] = "Day".to_owned();
      self.CheckTypeVarName[164][2] = "Month".to_owned();
      self.CheckTypeVarName[164][3] = "Year".to_owned();
      self.CheckTypeVarName[164][4] = "Hour".to_owned();
      self.CheckCategory[164] = 3;
      self.CheckDesc[164] = "Returns a thats: String date formatted. Keep in mind that strings when compared using <,>,=, etc.. are compared as if they were dates.";
      self.CheckTypeNames[165] = "CheckGetHistoricalIDByLib".to_owned();
      self.CheckTypeVarCount[165] = 2;
      self.CheckTypeVarName[165][1] = "Library Name";
      self.CheckTypeVarName[165][2] = "ID".to_owned();
      self.CheckCategory[165] = 10;
      self.CheckDesc[165] = "Returns the ID of the historical unit/historical model in the current scenario.  ";
      self.CheckTypeNames[166] = "CheckGetHistoricalIDForCommanderByLib".to_owned();
      self.CheckTypeVarCount[166] = 2;
      self.CheckTypeVarName[166][1] = "Library Name";
      self.CheckTypeVarName[166][2] = "ID".to_owned();
      self.CheckCategory[166] = 10;
      self.CheckDesc[166] = "Returns the ID of the commander in the current scenario.  ";
      self.CheckTypeNames[167] = "CheckGetPeopleByLib".to_owned();
      self.CheckTypeVarCount[167] = 2;
      self.CheckTypeVarName[167][1] = "Library Name";
      self.CheckTypeVarName[167][2] = "ID".to_owned();
      self.CheckCategory[167] = 10;
      self.CheckDesc[167] = "Returns the slot of the people in the current scenario.  ";
      self.CheckTypeNames[168] = "CheckGetRegimeByLib".to_owned();
      self.CheckTypeVarCount[168] = 2;
      self.CheckTypeVarName[168][1] = "Library Name";
      self.CheckTypeVarName[168][2] = "ID".to_owned();
      self.CheckCategory[168] = 10;
      self.CheckDesc[168] = "Returns the slot of the regime in the current scenario.  ";
      self.CheckTypeNames[169] = "CheckGetSFTypeByLib".to_owned();
      self.CheckTypeVarCount[169] = 2;
      self.CheckTypeVarName[169][1] = "Library Name";
      self.CheckTypeVarName[169][2] = "ID".to_owned();
      self.CheckCategory[169] = 10;
      self.CheckDesc[169] = "Returns the slot of the SFType in the current scenario.  ";
      self.CheckTypeNames[170] = "CheckGetEventByLib".to_owned();
      self.CheckTypeVarCount[170] = 2;
      self.CheckTypeVarName[170][1] = "Library Name";
      self.CheckTypeVarName[170][2] = "ID".to_owned();
      self.CheckCategory[170] = 10;
      self.CheckDesc[170] = "Returns the slot of the event in the current scenario.  ";
      self.CheckTypeNames[171] = "CheckGetCardByLib".to_owned();
      self.CheckTypeVarCount[171] = 2;
      self.CheckTypeVarName[171][1] = "Library Name";
      self.CheckTypeVarName[171][2] = "Library Slot";
      self.CheckCategory[171] = 10;
      self.CheckDesc[171] = "Returns the slot of the actioncard in the current scenario.  ";
      self.CheckTypeNames[172] = "CheckLibVarHistorical".to_owned();
      self.CheckTypeVarCount[172] = 3;
      self.CheckTypeVarName[172][1] = "Historical Unit Slot (in this scenario)";
      self.CheckTypeVarName[172][2] = "Library Name";
      self.CheckTypeVarName[172][3] = "LibVar Name";
      self.CheckCategory[172] = 10;
      self.CheckDesc[172] = "Returns the value of specified Libvar for historical unit or historical model. ";
      self.CheckTypeNames[173] = "CheckLibVarLandscape".to_owned();
      self.CheckTypeVarCount[173] = 2;
      self.CheckTypeVarName[173][1] = "Landscape Slot (in this scenario)";
      self.CheckTypeVarName[173][2] = "Library Name";
      self.CheckTypeVarName[173][3] = "LibVar Name";
      self.CheckCategory[173] = 10;
      self.CheckDesc[173] = "Returns the value of specified Libvar. ";
      self.CheckTypeNames[174] = "CheckLibVarPeople".to_owned();
      self.CheckTypeVarCount[174] = 3;
      self.CheckTypeVarName[174][1] = "People Slot (in this scenario)";
      self.CheckTypeVarName[174][2] = "Library Name";
      self.CheckTypeVarName[174][3] = "LibVar Name";
      self.CheckCategory[174] = 10;
      self.CheckDesc[174] = "Returns the value of specified Libvar. ";
      self.CheckTypeNames[175] = "CheckLibVarRegime".to_owned();
      self.CheckTypeVarCount[175] = 3;
      self.CheckTypeVarName[175][1] = "Regime Slot (in this scenario)";
      self.CheckTypeVarName[175][2] = "Library Name";
      self.CheckTypeVarName[175][3] = "LibVar Name";
      self.CheckCategory[175] = 10;
      self.CheckDesc[175] = "Returns the value of specified Libvar. ";
      self.CheckTypeNames[176] = "CheckLibVarRiver".to_owned();
      self.CheckTypeVarCount[176] = 3;
      self.CheckTypeVarName[176][1] = "River Slot (in this scenario)";
      self.CheckTypeVarName[176][2] = "Library Name";
      self.CheckTypeVarName[176][3] = "LibVar Name";
      self.CheckCategory[176] = 10;
      self.CheckDesc[176] = "Returns the value of specified Libvar. ";
      self.CheckTypeNames[177] = "CheckLibVarRoad".to_owned();
      self.CheckTypeVarCount[177] = 2;
      self.CheckTypeVarName[177][1] = "Road Slot (in this scenario)";
      self.CheckTypeVarName[177][2] = "Library Name";
      self.CheckTypeVarName[177][3] = "LibVar Name";
      self.CheckCategory[177] = 10;
      self.CheckDesc[177] = "Returns the value of specified Libvar. ";
      self.CheckTypeNames[178] = "CheckLibVarSFType".to_owned();
      self.CheckTypeVarCount[178] = 3;
      self.CheckTypeVarName[178][1] = "SFType Slot (in this scenario)";
      self.CheckTypeVarName[178][2] = "Library Name";
      self.CheckTypeVarName[178][3] = "LibVar Name";
      self.CheckCategory[178] = 10;
      self.CheckDesc[178] = "Returns the value of specified Libvar. ";
      self.CheckTypeNames[179] = "CheckLibVarLocType".to_owned();
      self.CheckTypeVarCount[179] = 3;
      self.CheckTypeVarName[179][1] = "LocType Slot (in this scenario)";
      self.CheckTypeVarName[179][2] = "Library Name";
      self.CheckTypeVarName[179][3] = "LibVar Name";
      self.CheckCategory[179] = 10;
      self.CheckDesc[179] = "Returns the value of specified Libvar. ";
      self.CheckTypeNames[180] = "CheckLibVarHex".to_owned();
      self.CheckTypeVarCount[180] = 4;
      self.CheckTypeVarName[180][1] = "X".to_owned();
      self.CheckTypeVarName[180][2] = "Y".to_owned();
      self.CheckTypeVarName[180][3] = "Library Name";
      self.CheckTypeVarName[180][4] = "LibVar Name";
      self.CheckCategory[180] = 10;
      self.CheckDesc[180] = "Returns the value of specified libvar of that hex. No value set returns 0. ";
      self.CheckTypeNames[181] = "CheckLibVarCommander".to_owned();
      self.CheckTypeVarCount[181] = 3;
      self.CheckTypeVarName[181][1] = "Commanders Historical Unit Slot";
      self.CheckTypeVarName[181][2] = "Library Name";
      self.CheckTypeVarName[181][3] = "LibVar Name";
      self.CheckCategory[181] = 10;
      self.CheckDesc[181] = "Returns the value of specified Libvar for historical commander. ";
      self.CheckTypeNames[182] = "CheckVisible".to_owned();
      self.CheckTypeVarCount[182] = 1;
      self.CheckTypeVarName[182][1] = "SFType#";
      self.CheckCategory[182] = 7;
      self.CheckDesc[182] = "Returns 1 if visible (dont show in list=false), returns 0 if not (dont show in list=true). Invisible SFTypes are used as base models in the Intermediate TroopType editor.";
      self.CheckTypeNames[183] = "CheckSFTypeCounter".to_owned();
      self.CheckTypeVarCount[183] = 0;
      self.CheckCategory[183] = 7;
      self.CheckDesc[183] = "Returns the highest SFType slot.";
      self.CheckTypeNames[184] = "CheckSFTypeTheaterOrGroup".to_owned();
      self.CheckTypeVarCount[184] = 2;
      self.CheckTypeVarName[184][1] = "SFType#";
      self.CheckTypeVarName[184][2] = "1=Check Theater, 2=CheckUnitGroup";
      self.CheckCategory[184] = 7;
      self.CheckDesc[184] = "Theater codes are: 0=land, 1=sea, 2=air.";
      self.CheckTypeNames[185] = "CheckHistoricalUnitMaster".to_owned();
      self.CheckTypeVarCount[185] = 1;
      self.CheckTypeVarName[185][1] = "Historical Unit ID";
      self.CheckCategory[185] = 5;
      self.CheckDesc[185] = "Returns the historical unit model ID";
      self.CheckTypeNames[186] = "CheckRegimeName".to_owned();
      self.CheckTypeVarCount[186] = 1;
      self.CheckTypeVarName[186][1] = "RegimeNr".to_owned();
      self.CheckCategory[186] = 2;
      self.CheckTypeNames[187] = "CheckUnitActivity".to_owned();
      self.CheckTypeVarCount[187] = 2;
      self.CheckTypeVarName[187][1] = "Unit Nr";
      self.CheckTypeVarName[187][2] = "Activity Data Type";
      self.CheckCategory[187] = 5;
      self.CheckDesc[187] = "These stats are reset after lateTurn events are executed. Activity Data Types are 1=Offensive combat AP spent, 2=Defensive combat AP spent, 3=Move AP spent, 4=Off+Def AP spent, 5=Off+Def+Move AP spent. 6=Did unit spent move AP or moved (0=no, 1=yes) ";
      self.CheckTypeNames[188] = "CheckGetPreDefIDByLib".to_owned();
      self.CheckTypeVarCount[188] = 2;
      self.CheckTypeVarName[188][1] = "Library Name";
      self.CheckTypeVarName[188][2] = "PreDefID".to_owned();
      self.CheckCategory[188] = 10;
      self.CheckDesc[188] = "Returns the preDefID in the current scenario for a the PreDefID inside a specific library.  ";
      self.CheckTypeNames[189] = "CheckTempString".to_owned();
      self.CheckTypeVarCount[189] = 1;
      self.CheckTypeVarName[189][1] = "TempString #";
      self.CheckCategory[189] = 3;
      self.CheckTypeNames[190] = "CheckPopupSkipped".to_owned();
      self.CheckTypeVarCount[190] = 0;
      self.CheckDesc[190] = "Returns 1 if popup skipped otherwise returns 0.";
      self.CheckCategory[190] = 3;
      self.CheckTypeNames[191] = "CheckForUnitVisibleToPlayer".to_owned();
      self.CheckTypeVarCount[191] = 2;
      self.CheckTypeVarName[191][1] = "Unit Number";
      self.CheckTypeVarName[191][2] = "Player Number";
      self.CheckCategory[191] = 3;
      self.CheckDesc[191] = "Returns 1,2,3 if unit is visible. Returns 0 if it is not. 1=?mark counter, 2=fuzzy info, 3=exact strength known ";
      self.CheckTypeNames[192] = "CheckFOW".to_owned();
      self.CheckTypeVarCount[192] = 0;
      self.CheckCategory[192] = 1;
      self.CheckDesc[192] = "Returns 1 if fow is on. 0 if off.";
      self.CheckTypeNames[193] = "CheckMoveOriginX".to_owned();
      self.CheckTypeVarCount[193] = 2;
      self.CheckTypeVarName[193][1] = "X".to_owned();
      self.CheckTypeVarName[193][2] = "Y".to_owned();
      self.CheckCategory[193] = 4;
      self.CheckDesc[193] = "You should only call this function if ExecSetMoveMatrix or ExecSetSupplyMatrix has been called before it. Any hex that has a moveable score (<9999) and that is not the source hex of the matrix called will have an origin. These 2 functions (x and y) will return that origin coordinate. -1 is returned if no origin hex is available.";
      self.CheckTypeNames[194] = "CheckMoveOriginY".to_owned();
      self.CheckTypeVarCount[194] = 2;
      self.CheckTypeVarName[194][1] = "X".to_owned();
      self.CheckTypeVarName[194][2] = "Y".to_owned();
      self.CheckCategory[194] = 4;
      self.CheckDesc[194] = "You should only call this function if ExecSetMoveMatrix or ExecSetSupplyMatrix has been called before it. Any hex that has a moveable score (<9999) and that is not the source hex of the matrix called will have an origin. These 2 functions (x and y) will return that origin coordinate. -1 is returned if no origin hex is available.";
      self.CheckTypeNames[195] = "CheckActionCardPPCost".to_owned();
      self.CheckTypeVarCount[195] = 1;
      self.CheckTypeVarName[195][1] = "Card slot #";
      self.CheckCategory[195] = 2;
      self.CheckDesc[195] = "Returns the PP cost of the action card specified.";
      self.CheckTypeNames[196] = "CheckGetHistoricalUnitSlot".to_owned();
      self.CheckTypeVarCount[196] = 1;
      self.CheckTypeVarName[196][1] = "ID#";
      self.CheckCategory[196] = 5;
      self.CheckDesc[196] = "Supply the historical unit ID# and get the slot# in return. If not found it will return -1.";
      self.CheckTypeNames[197] = "CheckActionCardTempVar".to_owned();
      self.CheckTypeVarCount[197] = 2;
      self.CheckTypeVarName[197][1] = "Card slot #";
      self.CheckTypeVarName[197][2] = "Tempvar 0 or 1";
      self.CheckCategory[197] = 2;
      self.CheckDesc[197] = "Returns the tempvar number selected.";
      self.CheckTypeNames[198] = "CheckLibraryOfCard".to_owned();
      self.CheckTypeVarCount[198] = 1;
      self.CheckTypeVarName[198][1] = "Action Card Slot";
      self.CheckCategory[198] = 10;
      self.CheckDesc[198] = "Returns a with: String the library name.  ";
      self.CheckTypeNames[199] = "CheckGetCommanderPoolValue".to_owned();
      self.CheckTypeVarCount[199] = 1;
      self.CheckTypeVarName[199][1] = "ID#";
      self.CheckCategory[199] = 5;
      self.CheckDesc[199] = "Returns 1 if in pool. Returns 0 if not. ";
      self.CheckTypeNames[200] = "CheckModulo".to_owned();
      self.CheckTypeVarCount[200] = 2;
      self.CheckTypeVarName[200][1] = "Number".to_owned();
      self.CheckTypeVarName[200][2] = "Divider".to_owned();
      self.CheckCategory[200] = 3;
      self.CheckDesc[200] = "Return (number) modulo (divider)... For example 4 modulo 3 = 1";
      self.CheckTypeNames[201] = "CheckSlotOwnerPercentage".to_owned();
      self.CheckTypeVarCount[201] = 4;
      self.CheckTypeVarName[201][1] = "AreaSlot#";
      self.CheckTypeVarName[201][2] = "Startval (>=)";
      self.CheckTypeVarName[201][3] = "Endval (<=)";
      self.CheckTypeVarName[201][4] = "Owner".to_owned();
      self.CheckCategory[201] = 4;
      self.CheckDesc[201] = "Returns a percentage of the specified areaslots between start and endvalue occupied by owner regime slot#";
      self.CheckTypeNames[202] = "CheckSlotNotInAIList".to_owned();
      self.CheckTypeVarCount[202] = 4;
      self.CheckTypeVarName[202][1] = "AreaSlot#";
      self.CheckTypeVarName[202][2] = "Startval (>=)";
      self.CheckTypeVarName[202][3] = "Endval (<=)";
      self.CheckTypeVarName[202][4] = "Max Dist";
      self.CheckCategory[202] = 4;
      self.CheckDesc[202] = "Within areaslot returns a value out of the range between start and endvalue occupied by current regime owner slot# closest to SetAreaXY specified. Returns -1 if nothing found. Function only looks within supply range of SetAreaXY and only within MaxDist.";
      self.CheckTypeNames[203] = "CheckSlotValueFirstXY".to_owned();
      self.CheckTypeVarCount[203] = 3;
      self.CheckTypeVarName[203][1] = "AreaSlot#";
      self.CheckTypeVarName[203][2] = "Value".to_owned();
      self.CheckTypeVarName[203][3] = "What? 1=x,2=y,3=reg";
      self.CheckCategory[203] = 4;
      self.CheckDesc[203] = "Within areaslot returns the x,y coordinate first found with the value given (x or y). Only values >= 1 work.";
      self.CheckTypeNames[204] = "CheckAbs".to_owned();
      self.CheckTypeVarCount[204] = 1;
      self.CheckTypeVarName[204][1] = "Number".to_owned();
      self.CheckCategory[204] = 3;
      self.CheckDesc[204] = "Return the absolute of the number passed";
      self.CheckTypeNames[205] = "CheckSubString".to_owned();
      self.CheckTypeVarCount[205] = 3;
      self.CheckTypeVarName[205][1] = "String".to_owned();
      self.CheckTypeVarName[205][2] = "Position".to_owned();
      self.CheckTypeVarName[205][3] = "Length".to_owned();
      self.CheckCategory[205] = 3;
      self.CheckDesc[205] = "For example. If string='abcdef' then position 2, length 2 will return 'bc'";
      self.CheckTypeNames[206] = "CheckRandomRowStringList2".to_owned();
      self.CheckTypeVarCount[206] = 3;
      self.CheckTypeVarName[206][1] = "Stringlist ID";
      self.CheckTypeVarName[206][2] = "Col used as weight (-1=none)";
      self.CheckTypeVarName[206][3] = "Col used as block (-1=none)";
      self.CheckCategory[206] = 6;
      self.CheckDesc[206] = "Concerning the block column. <= 0 will be considered NOT blocked. >= 1 will be considered blocked. Returns -1 if no suitable row found. ";
      self.CheckTypeNames[207] = "CheckAppendString".to_owned();
      self.CheckTypeVarCount[207] = 4;
      self.CheckTypeVarName[207][1] = "String".to_owned();
      self.CheckTypeVarName[207][2] = "Position".to_owned();
      self.CheckTypeVarName[207][3] = "String to Insert";
      self.CheckTypeVarName[207][4] = "Mode".to_owned();
      self.CheckCategory[207] = 3;
      self.CheckDesc[207] = "Mode 0 = Insert, Mode 1 = Overwrite. Example: string 'abcdef', pos=2, stringToInsert='123', mode=0 results in 'a123bcdef'";
      self.CheckTypeNames[208] = "CheckCapitalise".to_owned();
      self.CheckTypeVarCount[208] = 2;
      self.CheckTypeVarName[208][1] = "String".to_owned();
      self.CheckTypeVarName[208][2] = "Mode".to_owned();
      self.CheckCategory[208] = 3;
      self.CheckDesc[208] = "Mode 0 = Cap. 1st char of phrase, Mode 1 = Cap 1st char each word, Mode 2 = Cap all, Mode 3 = lowercase all";
      self.CheckTypeNames[209] = "CheckUnitEntrench".to_owned();
      self.CheckTypeVarCount[209] = 1;
      self.CheckTypeVarName[209][1] = "Unit#";
      self.CheckDesc[209] = "Keep in mind this is an average of the subformations.";
      self.CheckCategory[209] = 5;
      self.CheckTypeNames[210] = "CheckImportStringList".to_owned();
      self.CheckTypeVarCount[210] = 2;
      self.CheckTypeVarName[210][1] = "Stringlist ID";
      self.CheckTypeVarName[210][2] = "Filename".to_owned();
      self.CheckCategory[210] = 6;
      self.CheckDesc[210] = "Will replace said stringlist with the contents of the filename. This will be a full replacement and nothing of old contents will remain. Directory used is the 'metadata' directory. Please include a file extension in the filename like .txt. Returns 1 on succes. Returns -1 on file not found. Returns -2 on data corruption of file. ";
      self.CheckTypeNames[211] = "CheckUnitIntegrity".to_owned();
      self.CheckTypeVarCount[211] = 1;
      self.CheckTypeVarName[211][1] = "Unit#";
      self.CheckDesc[211] = "Returns a percentage between 0-100. ";
      self.CheckCategory[211] = 5;
      self.CheckTypeNames[212] = "CheckUnitSupplyConsumption".to_owned();
      self.CheckTypeVarCount[212] = 2;
      self.CheckTypeVarName[212][1] = "Unit#";
      self.CheckTypeVarName[212][2] = "Mode".to_owned();
      self.CheckDesc[212] = "Mode<=1 : Returns a percentage between 0-100. Mode 2: Returns missing supply, Mode 3: Returns missing fuel";
      self.CheckCategory[212] = 5;
      self.CheckTypeNames[213] = "CheckKey".to_owned();
      self.CheckTypeVarCount[213] = 2;
      self.CheckTypeVarName[213][1] = "Stringlist ID";
      self.CheckTypeVarName[213][2] = "Key name";
      self.CheckCategory[213] = 6;
      self.CheckDesc[213] = "Presumes key names in column 0 and values in column 1 ";
      self.CheckTypeNames[256] = "CheckSuperAdmin".to_owned();
      self.CheckTypeVarCount[256] = 0;
      self.CheckDesc[256] = "Returns 0 if normal player, Returns 1 is super admin ";
      self.CheckCategory[256] = 11;
      self.CheckTypeNames[214] = "CheckTextHeight".to_owned();
      self.CheckTypeVarCount[214] = 1;
      self.CheckTypeVarName[214][1] = "Full Element Text";
      self.CheckDesc[214] = "Returns the number of pixels ";
      self.CheckCategory[214] = 11;
      self.CheckTypeNames[ byte.MaxValue] = "CheckTextWidth".to_owned();
      self.CheckTypeVarCount[ byte.MaxValue] = 1;
      self.CheckTypeVarName[ byte.MaxValue, 1] = "Full Element Text";
      self.CheckDesc[ byte.MaxValue] = "Returns the number of pixels ";
      self.CheckCategory[ byte.MaxValue] = 11;
      self.CheckTypeNames[215] = "CheckUDSInput".to_owned();
      self.CheckTypeVarCount[215] = 1;
      self.CheckTypeVarName[215][1] = "Key name";
      self.CheckCategory[215] = 11;
      self.CheckDesc[215] = "Returns value for key";
      self.CheckTypeNames[216] = "CheckAreaslotWaterLevel".to_owned();
      self.CheckTypeVarCount[216] = 2;
      self.CheckTypeVarName[216][1] = "Height Map Areaslot#";
      self.CheckTypeVarName[216][2] = "Avg water points per hex";
      self.CheckCategory[216] = 12;
      self.CheckDesc[216] = "Expects an areaslot with height values that use the same scale as the avg water points. If you would use earth heigh map in meters and specify 2000 meters of water per hex you'll get the real earth seas as a result.";
      self.CheckTypeNames[217] = "CheckAreaslotSpecial".to_owned();
      self.CheckTypeVarCount[217] = 4;
      self.CheckTypeVarName[217][1] = "Areaslot#";
      self.CheckTypeVarName[217][2] = "SpecialType".to_owned();
      self.CheckTypeVarName[217][3] = "Second Areaslot#";
      self.CheckTypeVarName[217][4] = "Second Areaslot Value";
      self.CheckCategory[217] = 12;
      self.CheckDesc[217] = "SpecialType 1 is highest, 2 is lowest, 3 is average. You can specify -1 to not use optional second areaslot. If you do however you can limit the hexes being read to those with the specified value";
      self.CheckTypeNames[220] = "CheckStringListQuick".to_owned();
      self.CheckTypeVarCount[220] = 3;
      self.CheckTypeVarName[220][1] = "Stringlist ID";
      self.CheckTypeVarName[220][2] = "Row where col0 has value..";
      self.CheckTypeVarName[220][3] = "Col".to_owned();
      self.CheckCategory[220] = 6;
      self.CheckTypeNames[221] = "CheckAreaCount".to_owned();
      self.CheckTypeVarCount[221] = 3;
      self.CheckTypeVarName[221][1] = "Areaslot#";
      self.CheckTypeVarName[221][2] = "From".to_owned();
      self.CheckTypeVarName[221][3] = "Too".to_owned();
      self.CheckCategory[221] = 12;
      self.CheckDesc[221] = "Returns the number of hex equal or between FROM and TOO.";
      self.CheckTypeNames[224] = "CheckStringPart".to_owned();
      self.CheckTypeVarCount[224] = 2;
      self.CheckTypeVarName[224][1] = "String".to_owned();
      self.CheckTypeVarName[224][2] = "Part #";
      self.CheckCategory[224] = 3;
      self.CheckDesc[224] = "Returns the stringpart specified. Remember @ is used as a seperator. ";
      self.CheckTypeNames[225] = "CheckStringPartCounter".to_owned();
      self.CheckTypeVarCount[225] = 1;
      self.CheckTypeVarName[225][1] = "String".to_owned();
      self.CheckCategory[225] = 3;
      self.CheckDesc[225] = "Returns the number of parts: String in the string. Remember @ is used as a seperator. ";
      self.CheckTypeNames[218] = "CheckSqrt".to_owned();
      self.CheckTypeVarCount[218] = 1;
      self.CheckTypeVarName[218][1] = "Number".to_owned();
      self.CheckCategory[218] = 3;
      self.CheckDesc[218] = "Returns the square root of the number.";
      self.CheckTypeNames[219] = "CheckRandomNumber".to_owned();
      self.CheckTypeVarCount[219] = 2;
      self.CheckTypeVarName[219][1] = "Number1".to_owned();
      self.CheckTypeVarName[219][2] = "Number2".to_owned();
      self.CheckCategory[219] = 3;
      self.CheckDesc[219] = "Returns a random number including and between number1 and number2 ";
      self.CheckTypeNames[222] = "CheckPercentage".to_owned();
      self.CheckTypeVarCount[222] = 2;
      self.CheckTypeVarName[222][1] = "Number 1 (part)";
      self.CheckTypeVarName[222][2] = "Number 2 (total)";
      self.CheckCategory[222] = 3;
      self.CheckDesc[222] = "Returns how much percentage points number 1 if of number 2.";
      self.CheckTypeNames[223] = "CheckNeighbour".to_owned();
      self.CheckTypeVarCount[223] = 4;
      self.CheckTypeVarName[223][1] = "X".to_owned();
      self.CheckTypeVarName[223][2] = "Y".to_owned();
      self.CheckTypeVarName[223][3] = "Direction".to_owned();
      self.CheckTypeVarName[223][4] = "Mode".to_owned();
      self.CheckCategory[223] = 4;
      self.CheckDesc[223] = "Direction=1-6. Mode=1 returns X. Mode=2 returns Y. Returns -1 if neighbour is off-map.";
      self.CheckTypeNames[226] = "CheckSqrt2".to_owned();
      self.CheckTypeVarCount[226] = 3;
      self.CheckTypeVarName[226][1] = "Number".to_owned();
      self.CheckTypeVarName[226][2] = "Number square roots";
      self.CheckTypeVarName[226][3] = "Multiply result by";
      self.CheckCategory[226] = 3;
      self.CheckDesc[226] = "Returns the square root of the number. Allow for betting calculation of floating ponumbers: i32.";
      self.CheckTypeNames[227] = "CheckDecimal".to_owned();
      self.CheckTypeVarCount[227] = 3;
      self.CheckTypeVarName[227][1] = "Number".to_owned();
      self.CheckTypeVarName[227][2] = "Divided by number";
      self.CheckTypeVarName[227][3] = "Maximum decimals";
      self.CheckCategory[227] = 3;
      self.CheckDesc[227] = "Returns a text with: String the decimal number.";
      self.CheckTypeNames[228] = "CheckLogicPart".to_owned();
      self.CheckTypeVarCount[228] = 2;
      self.CheckTypeVarName[228][1] = "String".to_owned();
      self.CheckTypeVarName[228][2] = "Part # (1-3)";
      self.CheckCategory[228] = 3;
      self.CheckDesc[228] = "Returns the logicpart specified as string. Remember <,>,=,/,*,+,- is used as a seperator as well as logic part 2. ";
      self.CheckTypeNames[229] = "CheckKeyExists".to_owned();
      self.CheckTypeVarCount[229] = 2;
      self.CheckTypeVarName[229][1] = "Stringlist ID";
      self.CheckTypeVarName[229][2] = "Key name";
      self.CheckCategory[229] = 6;
      self.CheckDesc[229] = "If key exists returns 1 otherwise 0 ";
      self.CheckTypeNames[230] = "CheckDynamicElementHeight".to_owned();
      self.CheckTypeVarCount[230] = 1;
      self.CheckTypeVarName[230][1] = "Full Element Text";
      self.CheckDesc[230] = "Returns the number of pixels of height ";
      self.CheckCategory[230] = 3;
      self.CheckTypeNames[231] = "CheckReplaceFlags".to_owned();
      self.CheckTypeVarCount[231] = 2;
      self.CheckTypeVarName[231][1] = "String".to_owned();
      self.CheckTypeVarName[231][2] = "Stringlist ID for Flags";
      self.CheckDesc[231] = "Returns a equal: String to the passed: String but with any [flags] removed. Flag name will be looked for in stringlist col0 and replaced by value in col1. ";
      self.CheckCategory[231] = 3;
      self.CheckTypeNames[232] = "CheckLookupValue".to_owned();
      self.CheckTypeVarCount[232] = 1;
      self.CheckTypeVarName[232][1] = "LookupString".to_owned();
      self.CheckDesc[232] = "Returns or: String number on a hardcoded basis. If hardcoded LookupString is not found original String is returned. See documentation.";
      self.CheckCategory[232] = 3;
      self.CheckTypeNames[233] = "CheckFirstActionCardWith".to_owned();
      self.CheckTypeVarCount[233] = 4;
      self.CheckTypeVarName[233][1] = "Regime # (or -1 for all actioncards)";
      self.CheckTypeVarName[233][2] = "Tempvar0 (or -1 if ignore)";
      self.CheckTypeVarName[233][3] = "Tempvar1 (or -1 if ignore)";
      self.CheckTypeVarName[233][4] = "Library name (or empty for ignore)";
      self.CheckCategory[233] = 2;
      self.CheckDesc[233] = "Returns the actioncard slot# of the first card in the hand of specified regime with those exact tempvar values. If not found returns -1.";
      self.CheckTypeNames[234] = "CheckMultiplyByVerySmallNumber".to_owned();
      self.CheckTypeVarCount[234] = 3;
      self.CheckTypeVarName[234][1] = "Value as string";
      self.CheckTypeVarName[234][2] = "Multiplier".to_owned();
      self.CheckTypeVarName[234][3] = "Number of comma shifts";
      self.CheckCategory[234] = 3;
      self.CheckDesc[234] = "Value as allows: String you to work with long numbers. Number of comma shifts is applied to multiplier and then the value is multiplied by the multiplier. For exame value=1000, multiplier=5 and comma shift 2 results in 0.05*1000=50. Maximum 10 comma shifts.";
      self.CheckTypeNames[235] = "CheckSqrt3".to_owned();
      self.CheckTypeVarCount[235] = 4;
      self.CheckTypeVarName[235][1] = "Number a";
      self.CheckTypeVarName[235][2] = "Divide by number b";
      self.CheckTypeVarName[235][3] = "Number of square roots";
      self.CheckTypeVarName[235][4] = "Multiply by number c";
      self.CheckCategory[235] = 3;
      self.CheckDesc[235] = "Returns (the square root of (the number a / number b)) multiplied by number c. Allow for betting calculation of floating ponumbers: i32.";
      self.CheckTypeNames[236] = "CheckPowerLong".to_owned();
      self.CheckTypeVarCount[236] = 1;
      self.CheckTypeVarName[236][1] = "Number".to_owned();
      self.CheckCategory[236] = 3;
      self.CheckDesc[236] = "Returns the power of the number as as string, allowing you to work with long numbers.";
      self.CheckTypeNames[237] = "CheckStringListTotal".to_owned();
      self.CheckTypeVarCount[237] = 4;
      self.CheckTypeVarName[237][1] = "Stringlist ID";
      self.CheckTypeVarName[237][2] = "Col# ";
      self.CheckTypeVarName[237][3] = "Must have value";
      self.CheckTypeVarName[237][4] = "Add up values in col2#";
      self.CheckCategory[237] = 6;
      self.CheckDesc[237] = "If you put -1 for Col# than all values will be added up.";
      self.CheckTypeNames[238] = "CheckRegimeID".to_owned();
      self.CheckTypeVarCount[238] = 1;
      self.CheckTypeVarName[238][1] = "RegimeSlot#";
      self.CheckCategory[238] = 2;
      self.CheckTypeNames[239] = "CheckLocationID".to_owned();
      self.CheckTypeVarCount[239] = 1;
      self.CheckTypeVarName[239][1] = "LocationSlot#";
      self.CheckCategory[239] = 4;
      self.CheckTypeNames[240] = "CheckLocationSlot".to_owned();
      self.CheckTypeVarCount[240] = 1;
      self.CheckTypeVarName[240][1] = "Location ID";
      self.CheckCategory[240] = 4;
      self.CheckTypeNames[241] = "CheckRegimeSlot".to_owned();
      self.CheckTypeVarCount[241] = 1;
      self.CheckTypeVarName[241][1] = "Regime ID";
      self.CheckCategory[241] = 2;
      self.CheckTypeNames[242] = "CheckFindInStringList2".to_owned();
      self.CheckTypeVarCount[242] = 4;
      self.CheckTypeVarName[242][1] = "Stringlist ID";
      self.CheckTypeVarName[242][2] = "In Col";
      self.CheckTypeVarName[242][3] = "Find Value";
      self.CheckTypeVarName[242][4] = "Col # must have value >0";
      self.CheckCategory[242] = 6;
      self.CheckDesc[242] = "Returns row# where value is found or -1 if not.";
      self.CheckTypeNames[259] = "CheckTempUDSInput".to_owned();
      self.CheckTypeVarCount[259] = 1;
      self.CheckTypeVarName[259][1] = "TEMP Key name";
      self.CheckCategory[259] = 11;
      self.CheckDesc[259] = "Returns TEMP UdsKey value for key";
      self.CheckTypeNames[247] = "CheckLogicString".to_owned();
      self.CheckTypeVarCount[247] = 3;
      self.CheckTypeVarName[247][1] = "Stringlist ID for Flags";
      self.CheckTypeVarName[247][2] = "Stringlist ID for Flag Instructions";
      self.CheckTypeVarName[247][3] = "Logic String";
      self.CheckCategory[247] = 12;
      self.CheckTypeNames[243] = "CheckLocationLISitem".to_owned();
      self.CheckTypeVarCount[243] = 2;
      self.CheckTypeVarName[243][1] = "Location Slot#";
      self.CheckTypeVarName[243][2] = "LIS Item ID#";
      self.CheckCategory[243] = 12;
      self.CheckTypeNames[244] = "CheckUnitLISitem".to_owned();
      self.CheckTypeVarCount[244] = 2;
      self.CheckTypeVarName[244][1] = "Unit Slot#";
      self.CheckTypeVarName[244][2] = "LIS Item ID#";
      self.CheckCategory[244] = 12;
      self.CheckTypeNames[248] = "CheckLIS".to_owned();
      self.CheckTypeVarCount[248] = 3;
      self.CheckTypeVarName[248][1] = "Loc nr";
      self.CheckTypeVarName[248][2] = "To X";
      self.CheckTypeVarName[248][3] = "To Y";
      self.CheckCategory[248] = 12;
      self.CheckTypeNames[249] = "CheckHardcodedIF".to_owned();
      self.CheckTypeVarCount[249] = 3;
      self.CheckTypeVarName[249][1] = "Hardcoded Number";
      self.CheckTypeVarName[249][2] = "Story Id";
      self.CheckTypeVarName[249][3] = "Instance 1 Id";
      self.CheckTypeVarName[249][4] = "Instance 2 Id";
      self.CheckCategory[249] = 12;
      self.CheckTypeNames[245] = "CheckDoubleKey".to_owned();
      self.CheckTypeVarCount[245] = 3;
      self.CheckTypeVarName[245][1] = "Stringlist ID";
      self.CheckTypeVarName[245][2] = "Col 0 value";
      self.CheckTypeVarName[245][3] = "Col 1 Key name";
      self.CheckCategory[245] = 6;
      self.CheckDesc[245] = "Presumes an ID value in col 0 and key names in column 1 and values to look up in column 2 ";
      self.CheckTypeNames[246] = "CheckDoubleKeyExists".to_owned();
      self.CheckTypeVarCount[246] = 3;
      self.CheckTypeVarName[246][1] = "Stringlist ID";
      self.CheckTypeVarName[246][2] = "Col 0 value";
      self.CheckTypeVarName[246][3] = "Col 1 Key name";
      self.CheckCategory[246] = 6;
      self.CheckDesc[246] = "If key exists returns 1 otherwise 0 ";
      self.CheckTypeNames[250] = "CheckGetReinfTypeByLib".to_owned();
      self.CheckTypeVarCount[250] = 2;
      self.CheckTypeVarName[250][1] = "Library Name";
      self.CheckTypeVarName[250][2] = "ID".to_owned();
      self.CheckCategory[250] = 10;
      self.CheckDesc[250] = "Returns the ID of the Reinf.Type in the current scenario.  ";
      self.CheckTypeNames[251] = "ChecSFTypeFav".to_owned();
      self.CheckTypeVarCount[251] = 3;
      self.CheckTypeVarName[251][1] = "SfType #";
      self.CheckTypeVarName[251][2] = "Targetgroup#";
      self.CheckTypeVarName[251][3] = "Type 1=favscore for regular attack, 2=for artillery attack";
      self.CheckCategory[251] = 7;
      self.CheckTypeNames[252] = "ChecSFTypeArtPower".to_owned();
      self.CheckTypeVarCount[252] = 2;
      self.CheckTypeVarName[252][1] = "SfType #";
      self.CheckTypeVarName[252][2] = "Targetgroup#";
      self.CheckCategory[252] = 7;
      self.CheckTypeNames[253] = "ChecSFTypeStat".to_owned();
      self.CheckTypeVarCount[253] = 2;
      self.CheckTypeVarName[253][1] = "SfType #";
      self.CheckTypeVarName[253][2] = "StatNumber".to_owned();
      self.CheckCategory[253] = 7;
      self.CheckDesc[253] = "Stat Numbers: 1=weight, 2=carrycap, 3=canDoParadrop(0/1), 4=EP, 5=BlowBridgPts, 6=BasicSupNeed, 7=SupCarry, 8=ReconPts, 9=HidePts, 10=AntiSupplyPts, 11=AntiSupplyRange, 12=AnitSupplySea, 13=InitiativeAtt, 14=InitiativeDef, 15=Attacks, 16=StackPts, 17=RearArea(0/1), 18=ArtRange, 19=Fav.Target tries, 20=AA-range, 21=HitKill%, 22=HitRetreat%, 23=ArtilleryModSfTypeNr, 24=maxAttacked, 25=moveRedux";
      self.CheckTypeNames[254] = "ChecSFTypeLandscapeMod".to_owned();
      self.CheckTypeVarCount[254] = 4;
      self.CheckTypeVarName[254][1] = "SfType #";
      self.CheckTypeVarName[254][2] = "LandscapeType#";
      self.CheckTypeVarName[254][3] = "Type 1=offMod %, 2=defMod %.. 100=noMod, 50=half, 200=double";
      self.CheckCategory[254] = 7;
      self.CheckTypeNames[257] = "CheckHexLocNr2".to_owned();
      self.CheckTypeVarCount[257] = 2;
      self.CheckTypeVarName[257][1] = "X".to_owned();
      self.CheckTypeVarName[257][2] = "Y".to_owned();
      self.CheckCategory[257] = 4;
      self.CheckDesc[257] = "Only use of Supply Bases and Sources. Returns -1 if no Location in hex. 0 or higher if present.";
      self.CheckTypeNames[258] = "CheckLocTypeXY2".to_owned();
      self.CheckTypeVarCount[258] = 2;
      self.CheckTypeVarName[258][1] = "X".to_owned();
      self.CheckTypeVarName[258][2] = "Y".to_owned();
      self.CheckCategory[258] = 4;
      self.CheckTypeNames[260] = "CheckLibraryOfSFType".to_owned();
      self.CheckTypeVarCount[260] = 1;
      self.CheckTypeVarName[260][1] = "SFType Slot";
      self.CheckCategory[260] = 10;
      self.CheckDesc[260] = "Returns a with: String the library name.  ";
      self.CheckTypeNames[261] = "CheckDLC".to_owned();
      self.CheckTypeVarCount[261] = 1;
      self.CheckTypeVarName[261][1] = "DLC ID Code";
      self.CheckCategory[261] = 10;
      self.CheckDesc[261] = "Not documented. Usage by other than Vic is not advised.";
      self.CheckTypeNames[262] = "CheckLibraryOfThisEvent".to_owned();
      self.CheckTypeVarCount[262] = 0;
      self.CheckTypeVarName[262][1] = "";
      self.CheckCategory[262] = 10;
      self.CheckDesc[262] = "Returns a with: String the library name.  ";
      self.CheckTypeNames[263] = "CheckGetDateStringForRoundX".to_owned();
      self.CheckTypeVarCount[263] = 0;
      self.CheckTypeVarCount[263] = 1;
      self.CheckTypeVarName[263][1] = "Round".to_owned();
      self.CheckCategory[263] = 3;
      self.CheckDesc[263] = "Returns a thats: String date formatted.";
      self.CheckTypeCount = 263;
      self.ExecCategoryName[1] = "Regimes, People, Research";
      self.ExecCategoryName[2] = "Group Troop/Unit modifications";
      self.ExecCategoryName[3] = "Messanging, Campaign, Sound";
      self.ExecCategoryName[4] = "Hex & Areaslots";
      self.ExecCategoryName[5] = "Action Cards";
      self.ExecCategoryName[6] = "Logic & Vars";
      self.ExecCategoryName[7] = "AI in General";
      self.ExecCategoryName[8] = "Modifying Units";
      self.ExecCategoryName[9] = "SFtype & Itemtypes";
      self.ExecCategoryName[10] = "Game".to_owned();
      self.ExecCategoryName[11] = "Landscapetypes".to_owned();
      self.ExecCategoryName[12] = "Officerpool".to_owned();
      self.ExecCategoryName[13] = "Historical Units";
      self.ExecCategoryName[14] = "Mods of movetype/unitgrouptype";
      self.ExecCategoryName[15] = "Location(types)";
      self.ExecCategoryName[16] = "Stringlists".to_owned();
      self.ExecCategoryName[17] = "Miscellaneous".to_owned();
      self.ExecCategoryName[18] = "DC1 AI";
      self.ExecCategoryName[19] = "DC2 AI";
      self.ExecCategoryName[20] = "Library Execs";
      self.ExecCategoryName[21] = "UDS".to_owned();
      self.ExecCategoryName[22] = "Random Related";
      self.ExecTypeNames[1] = "ExecGiveRegimePP".to_owned();
      self.ExecTypeVarCount[1] = 2;
      self.ExecTypeVarName[1][1] = "RegimeNr".to_owned();
      self.ExecTypeVarName[1][2] = "PP".to_owned();
      self.ExecCategory[1] = 1;
      self.ExecTypeNames[2] = "OldStyleMessage (DEPRECIATED)";
      self.ExecTypeVarCount[2] = 4;
      self.ExecTypeVarName[2][1] = "Regime1".to_owned();
      self.ExecTypeVarName[2][2] = "Regime2".to_owned();
      self.ExecTypeVarName[2][3] = "BackPic#";
      self.ExecTypeVarName[2][4] = "FrontPic#";
      self.ExecTypeString[2] = 1;
      self.ExecCategory[2] = 3;
      self.ExecDesc[2] = "Is depreciated. Avoid using it";
      self.ExecTypeNames[3] = "ExecChangeDip (DEPRECIATED)";
      self.ExecTypeVarCount[3] = 2;
      self.ExecTypeVarName[3][1] = "RegimeNr1".to_owned();
      self.ExecTypeVarName[3][2] = "RegimeNr2".to_owned();
      self.ExecCategory[3] = 1;
      self.ExecDesc[3] = "Reverses the diplomatic relation from war to peace. Or from peace to war. Use a check to see current relation. IS DEPRECIATED. USE EXECCHANGERELATION INSTEAD.";
      self.ExecTypeNames[4] = "ExecSetWinner".to_owned();
      self.ExecTypeVarCount[4] = 1;
      self.ExecTypeVarName[4][1] = "RegimeNr".to_owned();
      self.ExecCategory[4] = 10;
      self.ExecTypeNames[5] = "ExecAddRegimeMorale".to_owned();
      self.ExecTypeVarCount[5] = 2;
      self.ExecTypeVarName[5][1] = "RegimeNr".to_owned();
      self.ExecTypeVarName[5][2] = "Mutation +/- pts";
      self.ExecCategory[5] = 1;
      self.ExecTypeNames[6] = "ExecAddUnit".to_owned();
      self.ExecTypeVarCount[6] = 4;
      self.ExecTypeVarName[6][1] = "UnitPreDef#";
      self.ExecTypeVarName[6][2] = "X".to_owned();
      self.ExecTypeVarName[6][3] = "Y".to_owned();
      self.ExecTypeVarName[6][4] = "Regime#";
      self.ExecTypeString[6] = 2;
      self.ExecCategory[6] = 8;
      self.ExecTypeNames[7] = "ExecLoad".to_owned();
      self.ExecTypeString[7] = 2;
      self.ExecCategory[7] = 3;
      self.ExecTypeNames[8] = "ExecSetSleep".to_owned();
      self.ExecTypeVarCount[8] = 2;
      self.ExecTypeVarName[8][1] = "Regime#";
      self.ExecTypeVarName[8][2] = "Sleep=1, Awake=0";
      self.ExecCategory[8] = 1;
      self.ExecTypeNames[9] = "ExecJoinRegime".to_owned();
      self.ExecTypeVarCount[9] = 3;
      self.ExecTypeVarName[9][1] = "Losing Regime#";
      self.ExecTypeVarName[9][2] = "Added to Regime#";
      self.ExecTypeVarName[9][3] = "Including Units 1=yes,0=no";
      self.ExecCategory[9] = 1;
      self.ExecTypeNames[11] = "ExecMessage".to_owned();
      self.ExecTypeVarCount[11] = 4;
      self.ExecTypeVarName[11][1] = "Regime1".to_owned();
      self.ExecTypeVarName[11][2] = "Regime2".to_owned();
      self.ExecTypeVarName[11][3] = "Overwrite scndesc >0=yes";
      self.ExecTypeVarName[11][4] = "Event Pic Nr (-1=none)";
      self.ExecTypeString[11] = 1;
      self.ExecCategory[11] = 3;
      self.ExecDesc[11] = "if reg1=-1 and reg2=-1 then message is send to all regimes. ";
      self.ExecTypeNames[12] = "BlockEvent".to_owned();
      self.ExecTypeVarCount[12] = 0;
      self.ExecCategory[12] = 6;
      self.ExecTypeNames[13] = "ExecChangeRegimeName".to_owned();
      self.ExecTypeVarCount[13] = 1;
      self.ExecTypeVarName[13][1] = "Regime#";
      self.ExecTypeString[13] = 2;
      self.ExecCategory[13] = 1;
      self.ExecTypeNames[14] = "ExecJoinArea".to_owned();
      self.ExecTypeVarCount[14] = 4;
      self.ExecTypeVarName[14][1] = "Areaslot# (0-9)";
      self.ExecTypeVarName[14][2] = "Areacode".to_owned();
      self.ExecTypeVarName[14][3] = "To Regime#";
      self.ExecTypeVarName[14][4] = "Units flee too areacode (-1=destr,-2=keephex)";
      self.ExecDesc[14] = "All units not allied with the new owner will be either flee or be destroyed or removed or keep hex.";
      self.ExecCategory[14] = 4;
      self.ExecTypeNames[15] = "ExecDisbandUnits".to_owned();
      self.ExecTypeVarCount[15] = 2;
      self.ExecTypeVarName[15][1] = "Regime#";
      self.ExecTypeVarName[15][2] = "People#";
      self.ExecCategory[15] = 2;
      self.ExecTypeNames[16] = "ExecChangeDipBlock".to_owned();
      self.ExecTypeVarCount[16] = 1;
      self.ExecTypeVarName[16][1] = "Regime#";
      self.ExecCategory[16] = 1;
      self.ExecTypeNames[17] = "ExecDoStructuralDamage".to_owned();
      self.ExecTypeVarCount[17] = 3;
      self.ExecTypeVarName[17][1] = "x".to_owned();
      self.ExecTypeVarName[17][2] = "y".to_owned();
      self.ExecTypeVarName[17][3] = "damage pts";
      self.ExecCategory[17] = 4;
      self.ExecDesc[17] = "If you give negative ammount of points it is handled as a repair";
      self.ExecTypeNames[18] = "ExecSetLandscape".to_owned();
      self.ExecTypeVarCount[18] = 3;
      self.ExecTypeVarName[18][1] = "X".to_owned();
      self.ExecTypeVarName[18][2] = "Y".to_owned();
      self.ExecTypeVarName[18][3] = "New LT#";
      self.ExecCategory[18] = 4;
      self.ExecTypeNames[19] = "ExecSetSprite".to_owned();
      self.ExecTypeVarCount[19] = 3;
      self.ExecTypeVarName[19][1] = "X".to_owned();
      self.ExecTypeVarName[19][2] = "Y".to_owned();
      self.ExecTypeVarName[19][3] = "New Sprite#";
      self.ExecCategory[19] = 4;
      self.ExecTypeNames[20] = "ExecSetSlot".to_owned();
      self.ExecTypeVarCount[20] = 4;
      self.ExecTypeVarName[20][1] = "X".to_owned();
      self.ExecTypeVarName[20][2] = "Y".to_owned();
      self.ExecTypeVarName[20][3] = "AreaSlot".to_owned();
      self.ExecTypeVarName[20][4] = "AreaCode".to_owned();
      self.ExecCategory[20] = 4;
      self.ExecTypeNames[21] = "ExecChangePeopleCombatMod".to_owned();
      self.ExecTypeVarCount[21] = 3;
      self.ExecTypeVarName[21][1] = "People#";
      self.ExecTypeVarName[21][2] = "PeopleGroupOwner#";
      self.ExecTypeVarName[21][3] = "New Combat Mod in %";
      self.ExecCategory[21] = 1;
      self.ExecTypeNames[22] = "ExecMoveTypeModifier".to_owned();
      self.ExecTypeVarCount[22] = 2;
      self.ExecTypeVarName[22][1] = "MoveType#";
      self.ExecTypeVarName[22][2] = "Percentage of normal movecost (100%=normal)";
      self.ExecCategory[22] = 14;
      self.ExecTypeNames[23] = "ExecUnitTypeModifier".to_owned();
      self.ExecTypeVarCount[23] = 2;
      self.ExecTypeVarName[23][1] = "UnitType#";
      self.ExecTypeVarName[23][2] = "Percentage of normal combat att/def (100%=normal)";
      self.ExecCategory[23] = 14;
      self.ExecTypeNames[24] = "ExecWheaterColor".to_owned();
      self.ExecTypeVarCount[24] = 3;
      self.ExecTypeVarName[24][1] = "Red Component (0=no change, 255=full)";
      self.ExecTypeVarName[24][2] = "Green Component (0=no change, 255=full)";
      self.ExecTypeVarName[24][3] = "Blue Component (0=no change, 255=full)";
      self.ExecCategory[24] = 17;
      self.ExecDesc[24] = "Changes the colour of the prehex sprite. But only of its core sprite not if its optional 64 border sprites.";
      self.ExecTypeNames[25] = "ExecChangeRiver".to_owned();
      self.ExecTypeVarCount[25] = 4;
      self.ExecTypeVarName[25][1] = "X".to_owned();
      self.ExecTypeVarName[25][2] = "Y".to_owned();
      self.ExecTypeVarName[25][3] = "From River #";
      self.ExecTypeVarName[25][4] = "Too River #";
      self.ExecCategory[25] = 4;
      self.ExecTypeNames[26] = "ExecChangeRoad".to_owned();
      self.ExecTypeVarCount[26] = 4;
      self.ExecTypeVarName[26][1] = "X".to_owned();
      self.ExecTypeVarName[26][2] = "Y".to_owned();
      self.ExecTypeVarName[26][3] = "From Road #";
      self.ExecTypeVarName[26][4] = "Too Road #";
      self.ExecCategory[26] = 4;
      self.ExecTypeNames[27] = "ExecChangeBridge".to_owned();
      self.ExecTypeVarCount[27] = 4;
      self.ExecTypeVarName[27][1] = "X".to_owned();
      self.ExecTypeVarName[27][2] = "Y".to_owned();
      self.ExecTypeVarName[27][3] = "Direction = 0(North),1,2,3,4,5(NorthWest)";
      self.ExecTypeVarName[27][4] = "-1=no bridge, >=0 = bridge";
      self.ExecCategory[27] = 4;
      self.ExecTypeNames[28] = "AI_SetAIVP".to_owned();
      self.ExecTypeVarCount[28] = 4;
      self.ExecTypeVarName[28][1] = "X".to_owned();
      self.ExecTypeVarName[28][2] = "Y".to_owned();
      self.ExecTypeVarName[28][3] = "Regime Nr#";
      self.ExecTypeVarName[28][4] = "AIVP amount. 0(none) - 999";
      self.ExecCategory[28] = 18;
      self.ExecCategory[28] = 19;
      self.ExecTypeNames[29] = "ExecChangeLocationType".to_owned();
      self.ExecTypeVarCount[29] = 4;
      self.ExecTypeVarName[29][1] = "X".to_owned();
      self.ExecTypeVarName[29][2] = "Y".to_owned();
      self.ExecTypeVarName[29][3] = "Loctype (-1=remove any location)";
      self.ExecTypeVarName[29][4] = "People #";
      self.ExecTypeString[29] = 2;
      self.ExecCategory[29] = 4;
      self.ExecTypeNames[30] = "ExecGiveSupply".to_owned();
      self.ExecTypeVarCount[30] = 4;
      self.ExecTypeVarName[30][1] = "X ";
      self.ExecTypeVarName[30][2] = "Y".to_owned();
      self.ExecTypeVarName[30][3] = "Supply Points to the top hq of the closest hq to x,y";
      self.ExecTypeVarName[30][4] = "Overrule recipient regime with regime #";
      self.ExecDesc[30] = "Overule recipient is inactive if put to 0. If no overrule the supplies are given to closest unit owned by regime that owns hex. But >0 is overrule 1=reg 0, 2=reg1 !!!! This +1 shift is inhere to keep backwards compatible with AdvTactics scenarios.";
      self.ExecCategory[30] = 4;
      self.ExecTypeNames[31] = "ExecSetRuleVar".to_owned();
      self.ExecTypeVarCount[31] = 3;
      self.ExecTypeVarName[31][1] = "Rulevar # ";
      self.ExecTypeVarName[31][2] = "New Value";
      self.ExecTypeVarName[31][3] = "Behind comma... 0=no action.. >0=divide new value by 100 to set a decimal.";
      self.ExecCategory[31] = 10;
      self.ExecTypeNames[32] = "ExecChangePeople".to_owned();
      self.ExecTypeVarCount[32] = 3;
      self.ExecTypeVarName[32][1] = "X".to_owned();
      self.ExecTypeVarName[32][2] = "Y".to_owned();
      self.ExecTypeVarName[32][3] = "Changes the people at location x,y";
      self.ExecCategory[32] = 4;
      self.ExecTypeNames[33] = "ExecGiveReinforcement".to_owned();
      self.ExecTypeVarCount[33] = 4;
      self.ExecTypeVarName[33][1] = "X".to_owned();
      self.ExecTypeVarName[33][2] = "Y".to_owned();
      self.ExecTypeVarName[33][3] = "Predef# to use";
      self.ExecTypeVarName[33][4] = "Multiplier".to_owned();
      self.ExecCategory[33] = 2;
      self.ExecTypeNames[34] = "ExecGiveActionCard".to_owned();
      self.ExecTypeVarCount[34] = 2;
      self.ExecTypeVarName[34][1] = "Card Nr";
      self.ExecTypeVarName[34][2] = "Regime Nr";
      self.ExecCategory[34] = 5;
      self.ExecTypeNames[35] = "ExecReduceReadiness".to_owned();
      self.ExecTypeVarCount[35] = 4;
      self.ExecTypeVarName[35][1] = "Regime Nr (-1=all)";
      self.ExecTypeVarName[35][2] = "People Nr (-1=all)";
      self.ExecTypeVarName[35][3] = "%rdn loss";
      self.ExecTypeVarName[35][4] = "%chance per unit";
      self.ExecCategory[35] = 2;
      self.ExecTypeNames[36] = "ExecAddHistoryActionCard".to_owned();
      self.ExecTypeVarCount[36] = 2;
      self.ExecTypeVarName[36][1] = "Card Nr";
      self.ExecTypeVarName[36][2] = "Regime Nr";
      self.ExecCategory[36] = 5;
      self.ExecTypeNames[37] = "ExecRemoveHistoryActionCard".to_owned();
      self.ExecTypeVarCount[37] = 2;
      self.ExecTypeVarName[37][1] = "Card Nr";
      self.ExecTypeVarName[37][2] = "Regime Nr";
      self.ExecCategory[37] = 5;
      self.ExecTypeNames[38] = "-Obsolete-";
      self.ExecTypeVarCount[38] = 4;
      self.ExecTypeVarName[38][1] = "Regime Nr";
      self.ExecTypeVarName[38][2] = "People Nr (-1=all)";
      self.ExecTypeVarName[38][3] = "% morale loss";
      self.ExecTypeVarName[38][4] = "%chance per unit";
      self.ExecCategory[38] = -1;
      self.ExecTypeNames[39] = "ExecLTReduceReadiness".to_owned();
      self.ExecTypeVarCount[39] = 4;
      self.ExecTypeVarName[39][1] = "Regime Nr";
      self.ExecTypeVarName[39][2] = "People Nr (-1=all)";
      self.ExecTypeVarName[39][3] = "%rdn loss";
      self.ExecTypeVarName[39][4] = "LandscapeType Nr";
      self.ExecCategory[39] = 2;
      self.ExecTypeNames[40] = "ExecLTReduceMorale".to_owned();
      self.ExecTypeVarCount[40] = 4;
      self.ExecTypeVarName[40][1] = "Regime Nr";
      self.ExecTypeVarName[40][2] = "People Nr (-1=all)";
      self.ExecTypeVarName[40][3] = "%mor loss";
      self.ExecTypeVarName[40][4] = "LandscapeType Nr";
      self.ExecCategory[40] = 2;
      self.ExecTypeNames[41] = "ExecSetItemCost".to_owned();
      self.ExecTypeVarCount[41] = 2;
      self.ExecTypeVarName[41][1] = "ItemType Nr";
      self.ExecTypeVarName[41][2] = "Set Prod Cost";
      self.ExecCategory[41] = 9;
      self.ExecTypeNames[42] = "ExecSetBaseMorale".to_owned();
      self.ExecTypeVarCount[42] = 2;
      self.ExecTypeVarName[42][1] = "Regnr".to_owned();
      self.ExecTypeVarName[42][2] = "Basemorale".to_owned();
      self.ExecCategory[42] = 1;
      self.ExecDesc[42] = "Not only sets base morale of regime, but also of unterregimes to the same as well as reducing all morale of all units with the same percentual ammount as the basemorale lost.";
      self.ExecTypeNames[43] = "ExecChangeVP".to_owned();
      self.ExecTypeVarCount[43] = 3;
      self.ExecTypeVarName[43][1] = "X".to_owned();
      self.ExecTypeVarName[43][2] = "Y".to_owned();
      self.ExecTypeVarName[43][3] = "new VP amount. 0(none) - 999";
      self.ExecCategory[43] = 4;
      self.ExecTypeNames[44] = "ExecRemoveActionCard".to_owned();
      self.ExecTypeVarCount[44] = 2;
      self.ExecTypeVarName[44][1] = "Card Nr";
      self.ExecTypeVarName[44][2] = "Regime Nr";
      self.ExecCategory[44] = 5;
      self.ExecTypeNames[45] = "ExecAIConservative".to_owned();
      self.ExecTypeVarCount[45] = 2;
      self.ExecTypeVarName[45][1] = "Regime Nr";
      self.ExecTypeVarName[45][2] = "Convervative.. 100% = normal >100% more defensive. <100 more aggressive";
      self.ExecCategory[45] = 18;
      self.ExecTypeNames[46] = "ExecPlayActionCard".to_owned();
      self.ExecTypeVarCount[46] = 2;
      self.ExecTypeVarName[46][1] = "Regime Nr";
      self.ExecTypeVarName[46][2] = "Action Card Nr";
      self.ExecCategory[46] = 5;
      self.ExecTypeNames[47] = "ExecExecuteEvent".to_owned();
      self.ExecTypeVarCount[47] = 1;
      self.ExecTypeVarName[47][1] = "Event Nr";
      self.ExecCategory[47] = 6;
      self.ExecDesc[47] = "IS DEPRECIATED BY CallFunction. tempvar 0,1,4,5 are passed on (but not returned).";
      self.ExecTypeNames[48] = "ExecActionCardName".to_owned();
      self.ExecTypeVarCount[48] = 1;
      self.ExecTypeVarName[48][1] = "Actioncard Nr";
      self.ExecTypeString[48] = 2;
      self.ExecCategory[48] = 5;
      self.ExecTypeNames[49] = "ExecActionCardPPCost".to_owned();
      self.ExecTypeVarCount[49] = 4;
      self.ExecTypeVarName[49][1] = "Actioncard Nr";
      self.ExecTypeVarName[49][2] = "New PPCost (-1 = keep same)";
      self.ExecTypeVarName[49][3] = "PPCost absolute increase or decrease (0=no change)";
      self.ExecTypeVarName[49][4] = "PPCost % increase or decrease (0=no % change)";
      self.ExecCategory[49] = 5;
      self.ExecTypeNames[50] = "ExecReplaceLandscapeSprite".to_owned();
      self.ExecTypeVarCount[50] = 1;
      self.ExecTypeVarName[50][1] = "Landscape nr";
      self.ExecTypeString[50] = 2;
      self.ExecCategory[50] = 11;
      self.ExecDesc[50] = "Actually loads a new PreHex graphic from the graphics directory.";
      self.ExecTypeNames[52] = "ExecMutateSFType".to_owned();
      self.ExecTypeVarCount[52] = 4;
      self.ExecTypeVarName[52][1] = "AreaSlot".to_owned();
      self.ExecTypeVarName[52][2] = "AreaCode".to_owned();
      self.ExecTypeVarName[52][3] = "From Sftype#";
      self.ExecTypeVarName[52][4] = "Too Sftype# (-1=remove)";
      self.ExecCategory[52] = 2;
      self.ExecDesc[52] = "After completion sets # of changed individuals in tempvar[999].";
      self.ExecTypeNames[53] = "ExecSingleMutateSFType".to_owned();
      self.ExecTypeVarCount[53] = 4;
      self.ExecTypeVarName[53][1] = "AreaSlot".to_owned();
      self.ExecTypeVarName[53][2] = "AreaCode".to_owned();
      self.ExecTypeVarName[53][3] = "From Sftype#";
      self.ExecTypeVarName[53][4] = "Too Sftype# (-1=remove)";
      self.ExecCategory[53] = 2;
      self.ExecDesc[53] = "Note: Currents turn units take precedence";
      self.ExecTypeNames[54] = "ExecSingleMutateSFTypeXY".to_owned();
      self.ExecTypeVarCount[54] = 4;
      self.ExecTypeVarName[54][1] = "X".to_owned();
      self.ExecTypeVarName[54][2] = "Y".to_owned();
      self.ExecTypeVarName[54][3] = "From Sftype#";
      self.ExecTypeVarName[54][4] = "Too Sftype# (-1=remove)";
      self.ExecCategory[54] = 2;
      self.ExecDesc[54] = "Note: Currents turn units take precedence";
      self.ExecTypeNames[55] = "ExecAntiSupplyRangeMod".to_owned();
      self.ExecTypeVarCount[55] = 2;
      self.ExecTypeVarName[55][1] = "MoveType#";
      self.ExecTypeVarName[55][2] = "Percentage of current AP (100%=normal)";
      self.ExecCategory[55] = 14;
      self.ExecTypeNames[56] = "ExecChangePeopleCombatModVS".to_owned();
      self.ExecTypeVarCount[56] = 3;
      self.ExecTypeVarName[56][1] = "People#";
      self.ExecTypeVarName[56][2] = "Versus PeopleGroupOwner#";
      self.ExecTypeVarName[56][3] = "New Combat Mod in %";
      self.ExecCategory[56] = 1;
      self.ExecTypeNames[57] = "ExecRemoveTroops".to_owned();
      self.ExecTypeVarCount[57] = 4;
      self.ExecTypeVarName[57][1] = "X".to_owned();
      self.ExecTypeVarName[57][2] = "Y".to_owned();
      self.ExecTypeVarName[57][3] = "SFType group";
      self.ExecTypeVarName[57][4] = "Percentage to remove";
      self.ExecCategory[57] = 2;
      self.ExecTypeNames[58] = "ExecRemoveTroopsByPeople".to_owned();
      self.ExecTypeVarCount[58] = 4;
      self.ExecTypeVarName[58][1] = "X".to_owned();
      self.ExecTypeVarName[58][2] = "Y".to_owned();
      self.ExecTypeVarName[58][3] = "specific people #";
      self.ExecTypeVarName[58][4] = "Percentage to remove";
      self.ExecCategory[58] = 2;
      self.ExecTypeNames[59] = "ExecRemoveTroopsByLowMorale".to_owned();
      self.ExecTypeVarCount[59] = 4;
      self.ExecTypeVarName[59][1] = "X".to_owned();
      self.ExecTypeVarName[59][2] = "Y".to_owned();
      self.ExecTypeVarName[59][3] = "Morale =< then";
      self.ExecTypeVarName[59][4] = "Percentage to remove";
      self.ExecCategory[59] = 2;
      self.ExecTypeNames[60] = "ExecSetLabel [old]";
      self.ExecTypeVarCount[60] = 4;
      self.ExecTypeVarName[60][1] = "X (tempvar999>0 overwrites datastring)";
      self.ExecTypeVarName[60][2] = "Y".to_owned();
      self.ExecTypeVarName[60][3] = "Type (0-10)";
      self.ExecTypeVarName[60][4] = "Slot (1-2)";
      self.ExecTypeString[60] = 2;
      self.ExecCategory[60] = 4;
      self.ExecDesc[60] = "If there is a value > 0 in tempvar999 it is ammended to the end of the string.";
      self.ExecTypeNames[61] = "ExecReadinessLossByArea".to_owned();
      self.ExecTypeVarCount[61] = 4;
      self.ExecTypeVarName[61][1] = "Areaslot".to_owned();
      self.ExecTypeVarName[61][2] = "Value of slot";
      self.ExecTypeVarName[61][3] = "Readiness loss";
      self.ExecTypeVarName[61][4] = "Chance to lose this (x%)";
      self.ExecCategory[61] = 2;
      self.ExecTypeNames[62] = "ExecOffensiveMod".to_owned();
      self.ExecTypeVarCount[62] = 4;
      self.ExecTypeVarName[62][1] = "X".to_owned();
      self.ExecTypeVarName[62][2] = "Y".to_owned();
      self.ExecTypeVarName[62][3] = "SfTypeGroup# (-1=all)";
      self.ExecTypeVarName[62][4] = "Absolute Modifier (0=none, -50 = -50%, +50 = +50%)";
      self.ExecCategory[62] = 14;
      self.ExecTypeNames[63] = "ExecDefensiveMod".to_owned();
      self.ExecTypeVarCount[63] = 4;
      self.ExecTypeVarName[63][1] = "X".to_owned();
      self.ExecTypeVarName[63][2] = "Y".to_owned();
      self.ExecTypeVarName[63][3] = "SfTypeGroup# (-1=all)";
      self.ExecTypeVarName[63][4] = "Absolute Modifier (0=none, -50 = -50%, +50 = +50%)";
      self.ExecCategory[63] = 14;
      self.ExecTypeNames[64] = "ExecOffensiveModByPeople".to_owned();
      self.ExecTypeVarCount[64] = 4;
      self.ExecTypeVarName[64][1] = "X".to_owned();
      self.ExecTypeVarName[64][2] = "Y".to_owned();
      self.ExecTypeVarName[64][3] = "People# (-1=all)";
      self.ExecTypeVarName[64][4] = "Absolute Modifier (0=none, -50 = -50%, +50 = +50%)";
      self.ExecCategory[64] = 14;
      self.ExecTypeNames[65] = "ExecDefensiveModByPeople".to_owned();
      self.ExecTypeVarCount[65] = 4;
      self.ExecTypeVarName[65][1] = "X".to_owned();
      self.ExecTypeVarName[65][2] = "Y".to_owned();
      self.ExecTypeVarName[65][3] = "People# (-1=all)";
      self.ExecTypeVarName[65][4] = "Absolute Modifier (0=none, -50 = -50%, +50 = +50%)";
      self.ExecCategory[65] = 14;
      self.ExecTypeNames[66] = "ExecOffensiveModByRegime".to_owned();
      self.ExecTypeVarCount[66] = 4;
      self.ExecTypeVarName[66][1] = "Regime # (-1=all)";
      self.ExecTypeVarName[66][2] = "People # (-1=all)";
      self.ExecTypeVarName[66][3] = "SfTypeGroup# (-1=all)";
      self.ExecTypeVarName[66][4] = "Absolute Modifier (0=none, -50 = -50%, +50 = +50%)";
      self.ExecCategory[66] = 14;
      self.ExecTypeNames[67] = "ExecDefensiveModByRegime".to_owned();
      self.ExecTypeVarCount[67] = 4;
      self.ExecTypeVarName[67][1] = "Regime # (-1=all)";
      self.ExecTypeVarName[67][2] = "People # (-1=all)";
      self.ExecTypeVarName[67][3] = "SfTypeGroup# (-1=all)";
      self.ExecTypeVarName[67][4] = "Absolute Modifier (0=none, -50 = -50%, +50 = +50%)";
      self.ExecCategory[67] = 14;
      self.ExecTypeNames[69] = "ExecLoad_UnitTransfer".to_owned();
      self.ExecTypeVarCount[69] = 3;
      self.ExecTypeVarName[69][1] = "Regime # ";
      self.ExecTypeVarName[69][2] = "AreaSlot #";
      self.ExecTypeVarName[69][3] = "Max Per Hex";
      self.ExecCategory[69] = 3;
      self.ExecTypeNames[72] = "ExecAreaSpreadOut".to_owned();
      self.ExecTypeVarCount[72] = 2;
      self.ExecTypeVarName[72][1] = "AreaSlot # ";
      self.ExecTypeVarName[72][2] = "Divideby ";
      self.ExecCategory[72] = 4;
      self.ExecTypeNames[73] = "ExecSetLoctypeDestruct".to_owned();
      self.ExecTypeVarCount[73] = 3;
      self.ExecTypeVarName[73][1] = "LocType#";
      self.ExecTypeVarName[73][2] = "LandscapeType#";
      self.ExecTypeVarName[73][3] = "Sprite #";
      self.ExecCategory[73] = 15;
      self.ExecTypeNames[76] = "ExecCopyActionCard (+Set Name)";
      self.ExecTypeVarCount[76] = 2;
      self.ExecTypeVarName[76][1] = "From#";
      self.ExecTypeVarName[76][2] = "Too#";
      self.ExecTypeString[76] = 2;
      self.ExecCategory[76] = 5;
      self.ExecDesc[76] = "If the Too# specified is a slot higher then existing number of actioncards, the number of actioncards slots will be extended";
      self.ExecTypeNames[77] = "ExecSetActionCard (+Set Text)";
      self.ExecTypeVarCount[77] = 4;
      self.ExecTypeVarName[77][1] = "Card#";
      self.ExecTypeVarName[77][2] = "TempVar0".to_owned();
      self.ExecTypeVarName[77][3] = "TempVar1".to_owned();
      self.ExecTypeVarName[77][4] = "EventPic#";
      self.ExecTypeString[77] = 2;
      self.ExecCategory[77] = 5;
      self.ExecTypeNames[79] = "ExecGetReconFrom".to_owned();
      self.ExecTypeVarCount[79] = 1;
      self.ExecTypeVarName[79][1] = "From Regime#";
      self.ExecCategory[79] = 1;
      self.ExecTypeNames[80] = "ExecAddExperience".to_owned();
      self.ExecTypeVarCount[80] = 4;
      self.ExecTypeVarName[80][1] = "People# (-1=all)";
      self.ExecTypeVarName[80][2] = "Points to add (+/-)";
      self.ExecTypeVarName[80][3] = "Minimum XP";
      self.ExecTypeVarName[80][4] = "Maximum XP";
      self.ExecTypeString[80] = 0;
      self.ExecCategory[80] = 2;
      self.ExecTypeNames[81] = "ExecMaxResearch".to_owned();
      self.ExecTypeVarCount[81] = 0;
      self.ExecCategory[81] = 2;
      self.ExecDesc[81] = "Every Suformation thats not in a predef unit that can upgrade if its regime has the research will upgrade";
      self.ExecTypeNames[82] = "ExecHexActionCard".to_owned();
      self.ExecTypeVarCount[82] = 3;
      self.ExecTypeVarName[82][1] = "X".to_owned();
      self.ExecTypeVarName[82][2] = "Y".to_owned();
      self.ExecTypeVarName[82][3] = "ActionCard# (-1=no actioncard)";
      self.ExecTypeString[82] = 0;
      self.ExecCategory[82] = 5;
      self.ExecDesc[82] = "Will place an action card on the hex in question. The card is executed immediatly upon the player conquering the hex. And will cause a popup to appear immediatly (if message used) Only if hex is conquered by combat or direct move in. So always place on locations because they cannot be conquered by ZOC. If an AI regime takes the card it cannot load a new game, select a unit or a hex.";
      self.ExecTypeNames[83] = "ExecSetHistoricUnitMove".to_owned();
      self.ExecTypeVarCount[83] = 4;
      self.ExecTypeVarName[83][1] = "Historic HQ #";
      self.ExecTypeVarName[83][2] = "Defend Defined Area #";
      self.ExecTypeVarName[83][3] = "Attack Defined Area #";
      self.ExecTypeVarName[83][4] = "Stance (1=def,2=norm,3=att)";
      self.ExecTypeString[83] = 0;
      self.ExecCategory[83] = 18;
      self.ExecTypeNames[84] = "ExecChangeAIPower".to_owned();
      self.ExecTypeVarCount[84] = 4;
      self.ExecTypeVarName[84][1] = "Regime".to_owned();
      self.ExecTypeVarName[84][2] = "X".to_owned();
      self.ExecTypeVarName[84][3] = "Y".to_owned();
      self.ExecTypeVarName[84][4] = "Power Points";
      self.ExecTypeString[84] = 0;
      self.ExecCategory[84] = 18;
      self.ExecDesc[84] = "If positive power points given it is meant to help influence the behaviour of the AI. This way you can set reinforcements to a certain regime in the area that hex x,y is in. But you can also use this function to set negative powerpoints versus a regime to simulate immediate pressure and force the AI to keep a certain reserve in the area.";
      self.ExecTypeNames[85] = "OBSOLETE".to_owned();
      self.ExecTypeVarCount[85] = 2;
      self.ExecTypeVarName[85][1] = "HQ".to_owned();
      self.ExecTypeVarName[85][2] = "Area".to_owned();
      self.ExecTypeString[85] = 0;
      self.ExecTypeNames[86] = "OBSOLETE".to_owned();
      self.ExecTypeVarCount[86] = 2;
      self.ExecTypeVarName[86][1] = "HQ".to_owned();
      self.ExecTypeVarName[86][2] = "Area".to_owned();
      self.ExecTypeString[86] = 0;
      self.ExecTypeNames[87] = "PNSOLETE".to_owned();
      self.ExecTypeVarCount[87] = 2;
      self.ExecTypeVarName[87][1] = "HQ".to_owned();
      self.ExecTypeVarName[87][2] = "HQ".to_owned();
      self.ExecTypeString[87] = 0;
      self.ExecTypeNames[88] = "OBSOLETE".to_owned();
      self.ExecTypeVarCount[88] = 2;
      self.ExecTypeVarName[88][1] = "HQ".to_owned();
      self.ExecTypeVarName[88][2] = "HQ".to_owned();
      self.ExecTypeString[88] = 0;
      self.ExecTypeNames[89] = "ExecPlayEventWav".to_owned();
      self.ExecTypeVarCount[89] = 0;
      self.ExecTypeString[89] = 2;
      self.ExecCategory[89] = 3;
      self.ExecDesc[89] = "Adds a sound file from the sound directory to the next ExecMessage, it will be played when the message is actually shown. So the effect is synchronized.";
      self.ExecTypeNames[90] = "ExecLoadCampaignRoomBackground".to_owned();
      self.ExecTypeVarCount[90] = 1;
      self.ExecTypeVarName[90][1] = "Event Picture# to be used";
      self.ExecCategory[90] = 3;
      self.ExecTypeNames[91] = "ExecSetCampaignRoomTitle".to_owned();
      self.ExecTypeVarCount[91] = 0;
      self.ExecTypeString[91] = 2;
      self.ExecCategory[91] = 3;
      self.ExecTypeNames[92] = "ExecPlayEventBackgroundWav".to_owned();
      self.ExecTypeVarCount[92] = 0;
      self.ExecTypeString[92] = 2;
      self.ExecCategory[92] = 3;
      self.ExecDesc[92] = "Loads the sound file from the sound directory and plays it immediatly.";
      self.ExecTypeNames[93] = "-DEPRECIATED- ExecHistoricalUnitForAddUnit";
      self.ExecTypeVarCount[93] = 4;
      self.ExecTypeVarName[93][1] = "Historical Unit#";
      self.ExecTypeVarName[93][2] = "Copy? (1=yes,0=no)";
      self.ExecTypeVarName[93][3] = "Number (only if copy)";
      self.ExecTypeVarName[93][4] = "NATO counter # (only if copy, <1=keep same)";
      self.ExecTypeString[93] = 2;
      self.ExecCategory[93] = 2;
      self.ExecDesc[93] = "You should use ExecAddHistoricalUnit instead. Old description: If you want added units to be linked to a historical unit you have to call this exec before you call the addunit. If you make a copy, we'll use the given historical unit as a template but change its name to number + string. By putting a ',' in the you: String can then specify the shortname. The shortname is always number+string. Once set it stays valid till changed or end of event in which it is declared.";
      self.ExecTypeNames[94] = "ExecSetAI".to_owned();
      self.ExecTypeVarCount[94] = 3;
      self.ExecTypeVarName[94][1] = "Regime #";
      self.ExecTypeVarName[94][2] = "AI (1=yes,0=no)";
      self.ExecTypeVarName[94][3] = "ProdBonus(0,100,250)";
      self.ExecCategory[94] = 7;
      self.ExecDesc[94] = "If you want to set a regime to AI player or make an AI a human player. ";
      self.ExecTypeNames[95] = "ExecUnitRdnModify".to_owned();
      self.ExecTypeVarCount[95] = 4;
      self.ExecTypeVarName[95][1] = "Unit #";
      self.ExecTypeVarName[95][2] = "Absolute modifier pts +/-";
      self.ExecTypeVarName[95][3] = "People (-1=all)";
      self.ExecTypeVarName[95][4] = "SFTypeGroup (-1=all)";
      self.ExecCategory[95] = 8;
      self.ExecDesc[95] = "Change stats of subformations in a specified unit.";
      self.ExecTypeNames[96] = "ExecUnitMorModify".to_owned();
      self.ExecTypeVarCount[96] = 4;
      self.ExecTypeVarName[96][1] = "Unit #";
      self.ExecTypeVarName[96][2] = "Absolute modifier pts +/-";
      self.ExecTypeVarName[96][3] = "People (-1=all)";
      self.ExecTypeVarName[96][4] = "SFTypeGroup (-1=all)";
      self.ExecCategory[96] = 8;
      self.ExecDesc[96] = "Change stats of subformations in a specified unit.";
      self.ExecTypeNames[97] = "ExecUnitXpModify".to_owned();
      self.ExecTypeVarCount[97] = 4;
      self.ExecTypeVarName[97][1] = "Unit #";
      self.ExecTypeVarName[97][2] = "Absolute modifier pts +/-";
      self.ExecTypeVarName[97][3] = "People (-1=all)";
      self.ExecTypeVarName[97][4] = "SFTypeGroup (-1=all)";
      self.ExecCategory[97] = 8;
      self.ExecDesc[97] = "Change stats of subformations in a specified unit.";
      self.ExecTypeNames[98] = "ExecUnitApModify".to_owned();
      self.ExecTypeVarCount[98] = 4;
      self.ExecTypeVarName[98][1] = "Unit #";
      self.ExecTypeVarName[98][2] = "Absolute modifier pts +/-";
      self.ExecTypeVarName[98][3] = "MoveTypeGroup (-1=all)";
      self.ExecTypeVarName[98][4] = "SFTypeGroup (-1=all)";
      self.ExecCategory[98] = 8;
      self.ExecDesc[98] = "Change stats of subformations in a specified unit.";
      self.ExecTypeNames[99] = "ExecUnitGiveTroops".to_owned();
      self.ExecTypeVarCount[99] = 2;
      self.ExecTypeVarName[99][1] = "Unit #";
      self.ExecTypeVarName[99][2] = "Predef Unit#";
      self.ExecCategory[99] = 8;
      self.ExecDesc[99] = "Add all troops in PreDef unit to this Unit.";
      self.ExecTypeNames[100] = "ExecUnitOffensiveBonus".to_owned();
      self.ExecTypeVarCount[100] = 4;
      self.ExecTypeVarName[100][1] = "Unit #";
      self.ExecTypeVarName[100][2] = "Absolute modifier pts +/-";
      self.ExecTypeVarName[100][3] = "People (-1=all)";
      self.ExecTypeVarName[100][4] = "SFTypeGroup (-1=all) (>=100 = specific sftype -100) (9001=only for artRange>0)";
      self.ExecCategory[100] = 8;
      self.ExecDesc[100] = "Change stats of subformations in a specified unit. If you set modifier to 0 it resets the offmod to 0. Each pois: i32 a percentage point. If you specify a sftypegroup >=100 then you are actually specifiying a specific SFType Nr.";
      self.ExecTypeNames[101] = "ExecUnitDefensiveBonus".to_owned();
      self.ExecTypeVarCount[101] = 4;
      self.ExecTypeVarName[101][1] = "Unit #";
      self.ExecTypeVarName[101][2] = "Absolute modifier pts +/-";
      self.ExecTypeVarName[101][3] = "People (-1=all)";
      self.ExecTypeVarName[101][4] = "SFTypeGroup (-1=all)  (>=100 = specific sftype -100)";
      self.ExecCategory[101] = 8;
      self.ExecDesc[101] = "Change stats of subformations in a specified unit. If you set the modifier to 0 it resets the defmod to 0. Each pois: i32 a percentage point.  If you specify a sftypegroup >=100 then you are actually specifiying a specific SFType Nr.";
      self.ExecTypeNames[102] = "ExecUnitSelectable".to_owned();
      self.ExecTypeVarCount[102] = 1;
      self.ExecTypeVarName[102][1] = "Unit #";
      self.ExecCategory[102] = 8;
      self.ExecDesc[102] = "The specified unit will now be selectable in the 'select a unit' popup screen. Use this exec in a pre-event from an actioncard.";
      self.ExecTypeNames[103] = "ExecSetMessageStyle <DEPRECIACTED>";
      self.ExecTypeVarCount[103] = 1;
      self.ExecTypeVarName[103][1] = "Message Style (0,1,2)";
      self.ExecCategory[103] = 3;
      self.ExecTypeString[103] = 2;
      self.ExecDesc[103] = "The specified style will be used for the remainder of this event. 0=normal. 1=black, 2=text cloud+black. It also allows you to set an optional footnote (messageNote1). only shown in style 1 + 2. Directly under the main message in italics. SetMessageStyle is used by DC1 and an earlier prototype, kept here for backwards compatibility. The variable it accesses will also be used by DC3's new ExecDynamicMessage which will put it to value 3.. ";
      self.ExecTypeNames[104] = "ExecSetMessageNote2".to_owned();
      self.ExecTypeVarCount[104] = 0;
      self.ExecCategory[104] = 3;
      self.ExecTypeString[104] = 2;
      self.ExecDesc[104] = "Sets a for: String underwriting of picture of character with text cloud. Shown in style 2.";
      self.ExecTypeNames[105] = "ExecChangeRelation".to_owned();
      self.ExecTypeVarCount[105] = 4;
      self.ExecTypeVarName[105][1] = "RegimeNr1".to_owned();
      self.ExecTypeVarName[105][2] = "RegimeNr2".to_owned();
      self.ExecTypeVarName[105][3] = "Diplomatic Relation #";
      self.ExecTypeVarName[105][4] = "Hide Message (1=yes hide)";
      self.ExecCategory[105] = 1;
      self.ExecDesc[105] = "Sets the relation between regime 1 and 2 to relation 0-2. 0=war, 1=peace, 2=allied.";
      self.ExecTypeNames[106] = "ExecAddRegime".to_owned();
      self.ExecTypeVarCount[106] = 2;
      self.ExecTypeVarName[106][1] = "Copy From";
      self.ExecTypeVarName[106][2] = "Set Uberregime";
      self.ExecCategory[106] = 1;
      self.ExecTypeString[106] = 2;
      self.ExecDesc[106] = "EXPERT USE ONLY!!!. Only use before start of game. Adds a regime and copies all settings from specified reigme. You can also set name of new regime. You can also set an uberregime here. Automaticly will ally with uberregime.";
      self.ExecTypeNames[107] = "ExecRemoveRegime".to_owned();
      self.ExecTypeVarCount[107] = 2;
      self.ExecTypeVarName[107][1] = "RegimeNr".to_owned();
      self.ExecTypeVarName[107][2] = "Add stuff too RegimerNr";
      self.ExecTypeString[107] = 2;
      self.ExecDesc[107] = "EXPERT USE ONLY!!!. Only use before start of game. You can set RegimeNr to -1 and then the exec will try to find a regime with the specified name to delete.";
      self.ExecCategory[107] = 1;
      self.ExecTypeNames[108] = "ExecAddOOB".to_owned();
      self.ExecTypeVarCount[108] = 2;
      self.ExecTypeVarName[108][1] = "Regime".to_owned();
      self.ExecTypeVarName[108][2] = "Historical Unit";
      self.ExecCategory[108] = 1;
      self.ExecDesc[108] = "Sets the historical unit to the regime and also everything in the OOB of the selected historical unit.";
      self.ExecTypeNames[109] = "ExecSetRegimeColor".to_owned();
      self.ExecTypeVarCount[109] = 4;
      self.ExecTypeVarName[109][1] = "Regime".to_owned();
      self.ExecTypeVarName[109][2] = "Red (0-255)";
      self.ExecTypeVarName[109][3] = "Green (0-255)";
      self.ExecTypeVarName[109][4] = "Blue (0-255)";
      self.ExecCategory[109] = 1;
      self.ExecDesc[109] = "Sets the of: Color the regime. And you can overwrite the Name if you want too. Leave blank: String to keep name.";
      self.ExecTypeString[109] = 2;
      self.ExecTypeNames[110] = "ExecAddHistoricalUnit".to_owned();
      self.ExecTypeVarCount[110] = 4;
      self.ExecTypeVarName[110][1] = "X".to_owned();
      self.ExecTypeVarName[110][2] = "Y".to_owned();
      self.ExecTypeVarName[110][3] = "Historical Unit ID";
      self.ExecTypeVarName[110][4] = "Dispersion in Hex";
      self.ExecCategory[110] = 13;
      self.ExecDesc[110] = "If Historical unit is a Model an instance of it will be created. If it is not it will be put on the map itself. If you set Dispersion to 0 all will go normal. Set dispersion>0 to simulate scatter effect on same sea/land type and regime type as selected for target hex. Set x=-1 to only create a new historical unit (based on specified model) and not place any unit on the map.";
      self.ExecTypeNames[111] = "ExecRemoveHistoricalUnit".to_owned();
      self.ExecTypeVarCount[111] = 1;
      self.ExecTypeVarName[111][1] = "Historical Unit";
      self.ExecCategory[111] = 13;
      self.ExecDesc[111] = "All units assigned to this exact Historical Unit will be removed.";
      self.ExecTypeNames[112] = "ExecSetGameVar".to_owned();
      self.ExecTypeVarCount[112] = 2;
      self.ExecTypeVarName[112][1] = "GameVar #";
      self.ExecTypeVarName[112][2] = "New Value";
      self.ExecCategory[112] = 6;
      self.ExecTypeNames[113] = "ExecSetTempVar".to_owned();
      self.ExecTypeVarCount[113] = 2;
      self.ExecTypeVarName[113][1] = "TempVar #";
      self.ExecTypeVarName[113][2] = "New Value";
      self.ExecCategory[113] = 6;
      self.ExecTypeNames[114] = "ExecSetRegimeVar".to_owned();
      self.ExecTypeVarCount[114] = 3;
      self.ExecTypeVarName[114][1] = "Regime #";
      self.ExecTypeVarName[114][2] = "Variable #";
      self.ExecTypeVarName[114][3] = "New Value";
      self.ExecCategory[114] = 6;
      self.ExecTypeNames[115] = "ExecSetTempString".to_owned();
      self.ExecTypeVarCount[115] = 2;
      self.ExecTypeVarName[115][1] = "Tempstring #";
      self.ExecTypeVarName[115][2] = "New Value";
      self.ExecCategory[115] = 6;
      self.ExecTypeNames[116] = "ExecSetStringList".to_owned();
      self.ExecTypeVarCount[116] = 4;
      self.ExecTypeVarName[116][1] = "Stringlist ID #";
      self.ExecTypeVarName[116][2] = "Row".to_owned();
      self.ExecTypeVarName[116][3] = "Col".to_owned();
      self.ExecTypeVarName[116][4] = "New Value";
      self.ExecCategory[116] = 16;
      self.ExecTypeNames[117] = "ExecMoveRegimeUp".to_owned();
      self.ExecTypeVarCount[117] = 1;
      self.ExecTypeVarName[117][1] = "RegNr".to_owned();
      self.ExecCategory[117] = 1;
      self.ExecTypeString[117] = 2;
      self.ExecDesc[117] = "EXPERT USE ONLY!!!. You can overrule with using name and setting Regnr=-1. Moves this regime to the end of the list of regimes. Do only use in editor or startup screen for it might screw up things if used in game.";
      self.ExecTypeNames[118] = "ExecSetRecon".to_owned();
      self.ExecTypeVarCount[118] = 4;
      self.ExecTypeVarName[118][1] = "X".to_owned();
      self.ExecTypeVarName[118][2] = "Y".to_owned();
      self.ExecTypeVarName[118][3] = "RegNr".to_owned();
      self.ExecTypeVarName[118][4] = "Pts".to_owned();
      self.ExecCategory[118] = 1;
      self.ExecDesc[118] = "Sets recon for this regime only on this hex only.";
      self.ExecTypeNames[119] = "ExecChangeCommanderSkill".to_owned();
      self.ExecTypeVarCount[119] = 4;
      self.ExecTypeVarName[119][1] = "Unit#";
      self.ExecTypeVarName[119][2] = "CombatMod +/-";
      self.ExecTypeVarName[119][3] = "MoraleMod +/-";
      self.ExecTypeVarName[119][4] = "StaffPts +/-";
      self.ExecCategory[119] = 13;
      self.ExecDesc[119] = "Inrease or diminish the officers stats.";
      self.ExecTypeNames[120] = "ExecChangeHisTempValue".to_owned();
      self.ExecTypeVarCount[120] = 4;
      self.ExecTypeVarName[120][1] = "Unit# (-1= use hist.unit#)";
      self.ExecTypeVarName[120][2] = "TempVar#(1-5), 6=aiZoneListNr";
      self.ExecTypeVarName[120][3] = "New Value";
      self.ExecTypeVarName[120][4] = "Use Historical Unit#";
      self.ExecCategory[120] = 13;
      self.ExecDesc[120] = "Keep in mind you can also use hisvars to store data. of which you have unlimited quantity. For aiZoneListNr only number above >0 are considered valid.";
      self.ExecTypeNames[121] = "ExecChangeCommanderDescript".to_owned();
      self.ExecTypeVarCount[121] = 2;
      self.ExecTypeVarName[121][1] = "His Unit ID#";
      self.ExecTypeVarName[121][2] = "New Description";
      self.ExecCategory[121] = 13;
      self.ExecDesc[121] = "";
      self.ExecTypeNames[122] = "ExecChangeCommanderOverlay".to_owned();
      self.ExecTypeVarCount[122] = 2;
      self.ExecTypeVarName[122][1] = "Unit#";
      self.ExecTypeVarName[122][2] = "New overlay filename";
      self.ExecCategory[122] = 13;
      self.ExecDesc[122] = "Reloads the overlay file for this commander. specifying an empty string '' will result in no overlay";
      self.ExecTypeNames[123] = "ExecCommanderAddDeckCardUsingStringList".to_owned();
      self.ExecTypeVarCount[123] = 2;
      self.ExecTypeVarName[123][1] = "Unit#";
      self.ExecTypeVarName[123][2] = "StringList CardList ID#";
      self.ExecCategory[123] = 13;
      self.ExecDesc[123] = "Picks a weighted chance card from the stringlist in question. will not take doubles. will add text to description";
      self.ExecTypeNames[124] = "ExecCommanderAddAutoEventUsingStringList".to_owned();
      self.ExecTypeVarCount[124] = 2;
      self.ExecTypeVarName[124][1] = "Unit#";
      self.ExecTypeVarName[124][2] = "StringList EventList ID#";
      self.ExecCategory[124] = 13;
      self.ExecDesc[124] = "Picks a weighted auto-event from the stringlist in question. will not take doubles. will add text to description";
      self.ExecTypeNames[125] = "ExecAddStringList".to_owned();
      self.ExecTypeVarCount[125] = 2;
      self.ExecTypeVarName[125][1] = "Rows".to_owned();
      self.ExecTypeVarName[125][2] = "Cols".to_owned();
      self.ExecCategory[125] = 16;
      self.ExecDesc[125] = "Adds a whole new stringlist object to the game data. Use the CheckStringListLastID to get its ID, which you need to change or read anything from it.";
      self.ExecTypeNames[126] = "ExecAddStringListCells".to_owned();
      self.ExecTypeVarCount[126] = 3;
      self.ExecTypeVarName[126][1] = "StringList ID";
      self.ExecTypeVarName[126][2] = "Extra Rows";
      self.ExecTypeVarName[126][3] = "Extra Cols";
      self.ExecCategory[126] = 16;
      self.ExecDesc[126] = "If you specify 0 will NOT add anything. specify 1 for adding one row or col. You can add rows and cols at the same time. Rows are added at the bottom, Cols on the right.";
      self.ExecTypeNames[ sbyte.MaxValue] = "ExecRemoveStringListRow".to_owned();
      self.ExecTypeVarCount[ sbyte.MaxValue] = 2;
      self.ExecTypeVarName[ sbyte.MaxValue, 1] = "StringList ID";
      self.ExecTypeVarName[ sbyte.MaxValue, 2] = "Row Number";
      self.ExecCategory[ sbyte.MaxValue] = 16;
      self.ExecTypeNames[128] = "ExecRemoveStringListCol".to_owned();
      self.ExecTypeVarCount[128] = 2;
      self.ExecTypeVarName[128][1] = "StringList ID";
      self.ExecTypeVarName[128][2] = "Col Number";
      self.ExecCategory[128] = 16;
      self.ExecTypeNames[129] = "ExecChangeAIDefensive".to_owned();
      self.ExecTypeVarCount[129] = 4;
      self.ExecTypeVarName[129][1] = "Regime".to_owned();
      self.ExecTypeVarName[129][2] = "X".to_owned();
      self.ExecTypeVarName[129][3] = "Y".to_owned();
      self.ExecTypeVarName[129][4] = "Defensive Bonus %";
      self.ExecTypeString[129] = 0;
      self.ExecCategory[129] = 18;
      self.ExecDesc[129] = ">0 is a percentual defensive bonus for defending this area for this regime. NEW AI ONLY";
      self.ExecTypeNames[130] = "ExecGiveCommanderXP".to_owned();
      self.ExecTypeVarCount[130] = 2;
      self.ExecTypeVarName[130][1] = "Unit#";
      self.ExecTypeVarName[130][2] = "XP to give or take +/-";
      self.ExecCategory[130] = 13;
      self.ExecTypeNames[131] = "ExecCreateCommander".to_owned();
      self.ExecTypeVarCount[131] = 2;
      self.ExecTypeVarName[131][1] = "Regime #";
      self.ExecTypeVarName[131][2] = "Overrule Officer Pool StringList ID (-1=dont)";
      self.ExecCategory[131] = 13;
      self.ExecTypeNames[132] = "ExecMoveUnit".to_owned();
      self.ExecTypeVarCount[132] = 3;
      self.ExecTypeVarName[132][1] = "Unit #";
      self.ExecTypeVarName[132][2] = "New X";
      self.ExecTypeVarName[132][3] = "New Y";
      self.ExecDesc[132] = "Keep in mind combat can occur immediatly after the move. Rebel Battle Mode will be used.";
      self.ExecCategory[132] = 8;
      self.ExecTypeNames[133] = "ExecChangeUnitRegime".to_owned();
      self.ExecTypeVarCount[133] = 2;
      self.ExecTypeVarName[133][1] = "Unit #";
      self.ExecTypeVarName[133][2] = "New Regime#";
      self.ExecDesc[133] = "Keep in mind combat can occur immediatly after the move. Rebel Battle Mode will be used.";
      self.ExecCategory[133] = 8;
      self.ExecTypeNames[134] = "ExecChangeLandscapeMoveCost".to_owned();
      self.ExecTypeVarCount[134] = 3;
      self.ExecTypeVarName[134][1] = "Landscape #";
      self.ExecTypeVarName[134][2] = "MovementType #";
      self.ExecTypeVarName[134][3] = "AP Cost";
      self.ExecDesc[134] = "";
      self.ExecCategory[134] = 11;
      self.ExecTypeNames[135] = "-DEPRECIATED- (execmodhitpoints)";
      self.ExecTypeVarCount[135] = 3;
      self.ExecTypeVarName[135][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[135][2] = "SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[135][3] = "Modifier (100=same, 50=half, 200=double) #";
      self.ExecDesc[135] = "";
      self.ExecCategory[135] = 9;
      self.ExecTypeNames[136] = "ExecModifyAirOverrule".to_owned();
      self.ExecTypeVarCount[136] = 3;
      self.ExecTypeVarName[136][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[136][2] = "SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[136][3] = "Modifier (100=same, 50=half, 200=double) #";
      self.ExecDesc[136] = "";
      self.ExecCategory[136] = 9;
      self.ExecTypeNames[137] = "ExecModifyEntrenchmentSpeed".to_owned();
      self.ExecTypeVarCount[137] = 3;
      self.ExecTypeVarName[137][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[137][2] = "SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[137][3] = "Modifier (100=same, 50=half, 200=double) #";
      self.ExecDesc[137] = "";
      self.ExecCategory[137] = 9;
      self.ExecTypeNames[138] = "ExecModifySupplyStock".to_owned();
      self.ExecTypeVarCount[138] = 4;
      self.ExecTypeVarName[138][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[138][2] = "SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[138][3] = "Modifier (100=same, 50=half, 200=double) #";
      self.ExecTypeVarName[138][4] = "For dc4+ only. Special Mode. <1=dont use. 1=Supply, 2=Fuel";
      self.ExecDesc[138] = "Modifies supply and stockpile stats";
      self.ExecCategory[138] = 9;
      self.ExecTypeNames[139] = "ExecModifyAttackScore".to_owned();
      self.ExecTypeVarCount[139] = 3;
      self.ExecTypeVarName[139][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[139][2] = "SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[139][3] = "Versus SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[139][4] = "Modifier (100=same, 50=half, 200=double) #";
      self.ExecDesc[139] = "Modifies AttackArt,AttackPower,AttackPowerDef all 3.";
      self.ExecCategory[139] = 9;
      self.ExecTypeNames[140] = "ExecModifyUnitSupply".to_owned();
      self.ExecTypeVarCount[140] = 4;
      self.ExecTypeVarName[140][1] = "Regime (-1=none)";
      self.ExecTypeVarName[140][2] = "Specific Unit (-1=none)";
      self.ExecTypeVarName[140][3] = "Modifier % (100=keep same, 50=half, 200=double)";
      self.ExecTypeVarName[140][4] = "Modifier absolute (0=nill, -x , +x )";
      self.ExecCategory[140] = 8;
      self.ExecDesc[140] = "Increases or decreases supplies of the unit. cannot go over max supplystore.";
      self.ExecTypeNames[141] = "ExecRemoveUnits".to_owned();
      self.ExecTypeVarCount[141] = 1;
      self.ExecTypeVarName[141][1] = "Regime #";
      self.ExecCategory[141] = 8;
      self.ExecTypeNames[142] = "ExecLoad_HistoricalTransfer".to_owned();
      self.ExecTypeVarCount[142] = 1;
      self.ExecTypeVarName[142][1] = "Regime # ";
      self.ExecCategory[142] = 3;
      self.ExecTypeNames[143] = "ExecRemoveTroopsByReinforcementGroup".to_owned();
      self.ExecTypeVarCount[143] = 4;
      self.ExecTypeVarName[143][1] = "X (-1 = from unit)";
      self.ExecTypeVarName[143][2] = "Y (or unit #)";
      self.ExecTypeVarName[143][3] = "Reinforcement group";
      self.ExecTypeVarName[143][4] = "Percentage to remove";
      self.ExecCategory[143] = 2;
      self.ExecTypeNames[144] = "ExecChangeCommander".to_owned();
      self.ExecTypeVarCount[144] = 2;
      self.ExecTypeVarName[144][1] = "Unit#";
      self.ExecTypeVarName[144][2] = "Historical Unit#";
      self.ExecCategory[144] = 8;
      self.ExecDesc[144] = "Replace the commander with that from a historical unit. ";
      self.ExecTypeNames[145] = "ExecSetSupplyMatrix".to_owned();
      self.ExecTypeVarCount[145] = 4;
      self.ExecTypeVarName[145][1] = "Source X";
      self.ExecTypeVarName[145][2] = "Source Y";
      self.ExecTypeVarName[145][3] = "AreaSlot #";
      self.ExecTypeVarName[145][4] = "Regime #";
      self.ExecCategory[145] = 6;
      self.ExecDesc[145] = "Sets movecost value on each hex. if not in supply value will be set to 9999. It will not overwrite a lower value with a higher value. So make sure you set the areaslot matrix to 9999 before you call. this is done to allow multiple sources for making the matrix.";
      self.ExecTypeNames[146] = "ExecClearMatrix".to_owned();
      self.ExecTypeVarCount[146] = 2;
      self.ExecTypeVarName[146][1] = "AreaSlot #";
      self.ExecTypeVarName[146][2] = "Value".to_owned();
      self.ExecCategory[146] = 6;
      self.ExecDesc[146] = "Sets the specified slot for all hexes to specified value";
      self.ExecTypeNames[147] = "SetUberRegime".to_owned();
      self.ExecTypeVarCount[147] = 2;
      self.ExecTypeVarName[147][1] = "ForRegime".to_owned();
      self.ExecTypeVarName[147][2] = "UberRegime (-1=none)";
      self.ExecCategory[147] = 1;
      self.ExecTypeNames[148] = "SetCardXY".to_owned();
      self.ExecTypeVarCount[148] = 2;
      self.ExecTypeVarName[148][1] = "X".to_owned();
      self.ExecTypeVarName[148][2] = "Y".to_owned();
      self.ExecDesc[148] = "If you are calling a card who needs a hex input. set it with this exec.";
      self.ExecCategory[148] = 5;
      self.ExecTypeNames[149] = "ExecRemoveTroopsForNextHistorical".to_owned();
      self.ExecTypeVarCount[149] = 2;
      self.ExecTypeVarName[149][1] = "Reinforcement group";
      self.ExecTypeVarName[149][2] = "Loss %";
      self.ExecCategory[149] = 13;
      self.ExecDesc[149] = "For each event you have to set the loss levels. These values are reset per event. If set any unit added with AddHistorical will suffer these losses.";
      self.ExecTypeNames[150] = "ExecSetHistoricalCounter".to_owned();
      self.ExecTypeVarCount[150] = 2;
      self.ExecTypeVarName[150][1] = "Historical Unit";
      self.ExecTypeVarName[150][2] = "New Counter Value";
      self.ExecCategory[150] = 13;
      self.ExecTypeNames[151] = "ExecSetHistoricalUnitHQ".to_owned();
      self.ExecTypeVarCount[151] = 3;
      self.ExecTypeVarName[151][1] = "Historical Unit";
      self.ExecTypeVarName[151][2] = "New unit # HQ";
      self.ExecTypeVarName[151][3] = "Historical Unit # HQ";
      self.ExecCategory[151] = 13;
      self.ExecDesc[151] = "Sets all units that belong to this historical unit to the unit or historical unit HQ # specified.. Historical <= 0 means unit is used.";
      self.ExecTypeNames[152] = "ExecAddAntiSupply".to_owned();
      self.ExecTypeVarCount[152] = 4;
      self.ExecTypeVarName[152][1] = "X".to_owned();
      self.ExecTypeVarName[152][2] = "Y".to_owned();
      self.ExecTypeVarName[152][3] = "Regime".to_owned();
      self.ExecTypeVarName[152][4] = "Points".to_owned();
      self.ExecCategory[152] = 4;
      self.ExecDesc[152] = "Must be used in early turn check in order to be used after regular setting of AS and before Supply system...";
      self.ExecTypeNames[153] = "ExecChangeDate".to_owned();
      self.ExecTypeVarCount[153] = 3;
      self.ExecTypeVarName[153][1] = "Day".to_owned();
      self.ExecTypeVarName[153][2] = "Month".to_owned();
      self.ExecTypeVarName[153][3] = "Year".to_owned();
      self.ExecCategory[153] = 10;
      self.ExecTypeNames[154] = "ExecQuit".to_owned();
      self.ExecTypeVarCount[154] = 0;
      self.ExecCategory[154] = 6;
      self.ExecDesc[154] = "Quits the player back to the main menu. Can only be played from the research hand cards screen!";
      self.ExecTypeNames[155] = "ExecAIStance".to_owned();
      self.ExecTypeVarCount[155] = 2;
      self.ExecTypeVarName[155][1] = "Regime".to_owned();
      self.ExecTypeVarName[155][2] = "Stance (0=no overrule, 1=att, 2=def, 3=meeting)";
      self.ExecTypeString[155] = 0;
      self.ExecCategory[155] = 18;
      self.ExecDesc[155] = "If 0 the regime with most powerpoints is considered the attacker. If 3 too, but if 3 then this regime also gets the meeting flag.";
      self.ExecTypeNames[156] = "ExecReduceLandInitialEntrench".to_owned();
      self.ExecTypeVarCount[156] = 2;
      self.ExecTypeVarName[156][1] = "LandscapeType (-1=all)";
      self.ExecTypeVarName[156][2] = "Percentage (100=same. 50=less >100=more)";
      self.ExecTypeString[156] = 0;
      self.ExecCategory[156] = 11;
      self.ExecTypeNames[157] = "ExecSetSpecial".to_owned();
      self.ExecTypeVarCount[157] = 4;
      self.ExecTypeVarName[157][1] = "SpecialType".to_owned();
      self.ExecTypeVarName[157][2] = "SpecialSprite".to_owned();
      self.ExecTypeVarName[157][3] = "X".to_owned();
      self.ExecTypeVarName[157][4] = "Y".to_owned();
      self.ExecTypeString[157] = 0;
      self.ExecCategory[157] = 4;
      self.ExecDesc[157] = "Either Type is a LanscapeType slot with corresponding Sprite or it is -1 and a smallgfx slot will be used.";
      self.ExecTypeNames[158] = "ExecReduceAntiStructural".to_owned();
      self.ExecTypeVarCount[158] = 2;
      self.ExecTypeVarName[158][1] = "SFType (-1=all)";
      self.ExecTypeVarName[158][2] = "Percentage (100=same. 50=less >100=more)";
      self.ExecTypeString[158] = 0;
      self.ExecCategory[158] = 9;
      self.ExecTypeNames[159] = "ExecReduceTroops".to_owned();
      self.ExecTypeVarCount[159] = 4;
      self.ExecTypeVarName[159][1] = "SFType (-1=all)";
      self.ExecTypeVarName[159][2] = "PeopleType (-1=all)";
      self.ExecTypeVarName[159][3] = "Remove x%";
      self.ExecTypeVarName[159][4] = "+ remove between 0-y% random";
      self.ExecTypeString[159] = 0;
      self.ExecCategory[159] = 2;
      self.ExecTypeNames[160] = "OBSOLETE".to_owned();
      self.ExecTypeVarCount[160] = 2;
      self.ExecTypeVarName[160][1] = "SFType".to_owned();
      self.ExecTypeVarName[160][2] = "Hitpoints".to_owned();
      self.ExecTypeString[160] = 0;
      self.ExecCategory[160] = -1;
      self.ExecTypeNames[161] = "ExecSFTypePower".to_owned();
      self.ExecTypeVarCount[161] = 4;
      self.ExecTypeVarName[161][1] = "SFType".to_owned();
      self.ExecTypeVarName[161][2] = "Targetgroup# (-1=all)";
      self.ExecTypeVarName[161][3] = "Attack score";
      self.ExecTypeVarName[161][4] = "Defend score";
      self.ExecTypeString[161] = 0;
      self.ExecCategory[161] = 9;
      self.ExecTypeNames[162] = "CallFunction".to_owned();
      self.ExecTypeVarCount[162] = 1;
      self.ExecTypeVarName[162][1] = "Name of function to call";
      self.ExecTypeString[162] = 0;
      self.ExecCategory[162] = 6;
      self.ExecDesc[162] = "Case of name does not matter, but must be exact otherwise. The tempvar900-999 and tempstring900-999 will be copied to the receiving function and can be altered by it and will be returned back to the calling function. this way you dont have to use gamevars like in ExecExecute. (You can use these call by name feature to make functionlibs that can be loaded by import all). Tempvar 0,1,4,5 are copied to called function, but not back again.";
      self.ExecTypeNames[163] = "ExecSFTypeDescription".to_owned();
      self.ExecTypeVarCount[163] = 2;
      self.ExecTypeVarName[163][1] = "SFType#";
      self.ExecTypeVarName[163][2] = "Description".to_owned();
      self.ExecTypeString[163] = 0;
      self.ExecCategory[163] = 9;
      self.ExecDesc[163] = "";
      self.ExecTypeNames[164] = "ExecSFTypeHP".to_owned();
      self.ExecTypeVarCount[164] = 4;
      self.ExecTypeVarName[164][1] = "SFType".to_owned();
      self.ExecTypeVarName[164][2] = "Targetgroup# (-1=all)";
      self.ExecTypeVarName[164][3] = "Attack HP";
      self.ExecTypeVarName[164][4] = "Defend HP";
      self.ExecTypeString[164] = 0;
      self.ExecCategory[164] = 9;
      self.ExecTypeNames[165] = "ExecSFTypeMoveType".to_owned();
      self.ExecTypeVarCount[165] = 2;
      self.ExecTypeVarName[165][1] = "SFType#";
      self.ExecTypeVarName[165][2] = "Type".to_owned();
      self.ExecTypeString[165] = 0;
      self.ExecCategory[165] = 9;
      self.ExecDesc[165] = "";
      self.ExecTypeNames[166] = "ExecSFTypeMoveRedux".to_owned();
      self.ExecTypeVarCount[166] = 2;
      self.ExecTypeVarName[166][1] = "SFType#";
      self.ExecTypeVarName[166][2] = "Redux".to_owned();
      self.ExecTypeString[166] = 0;
      self.ExecCategory[166] = 9;
      self.ExecDesc[166] = "";
      self.ExecTypeNames[168] = "ExecSFTypeLogo".to_owned();
      self.ExecTypeVarCount[168] = 3;
      self.ExecTypeVarName[168][1] = "SFType#";
      self.ExecTypeVarName[168][2] = "Logo#";
      self.ExecTypeVarName[168][3] = "value".to_owned();
      self.ExecTypeString[168] = 0;
      self.ExecCategory[168] = 9;
      self.ExecDesc[168] = "";
      self.ExecTypeNames[170] = "ExecSFTypeFuelUse".to_owned();
      self.ExecTypeVarCount[170] = 3;
      self.ExecTypeVarName[170][1] = "SFType#";
      self.ExecTypeVarName[170][2] = "ForMove (10ap)";
      self.ExecTypeVarName[170][3] = "ForAttack (combatround)";
      self.ExecTypeString[170] = 0;
      self.ExecCategory[170] = 9;
      self.ExecDesc[170] = "";
      self.ExecTypeNames[171] = "ExecSetRegimePeople".to_owned();
      self.ExecTypeVarCount[171] = 3;
      self.ExecTypeVarName[171][1] = "Regime# ";
      self.ExecTypeVarName[171][2] = "People # (or -1=to leave as is) ";
      self.ExecTypeVarName[171][3] = "officerpool # (or-1=to leave as is)";
      self.ExecCategory[171] = 1;
      self.ExecTypeNames[172] = "ExecSetHQ".to_owned();
      self.ExecTypeVarCount[172] = 2;
      self.ExecTypeVarName[172][1] = "Unit".to_owned();
      self.ExecTypeVarName[172][2] = "HQ".to_owned();
      self.ExecCategory[172] = 8;
      self.ExecTypeNames[173] = "ExecSetMatrix".to_owned();
      self.ExecTypeVarCount[173] = 3;
      self.ExecTypeVarName[173][1] = "Slot".to_owned();
      self.ExecTypeVarName[173][2] = "Operation".to_owned();
      self.ExecTypeVarName[173][3] = "Disallow over sea (0=no, 1=yes)";
      self.ExecCategory[173] = 6;
      self.ExecDesc[173] = "You cannot set operation yet. For now it just subtracts 1 of the value and copies if higher then neighbour. Thus make sure to make a clearmatrix(0) and then set coordinates you want to spread out.";
      self.ExecTypeNames[174] = "ExecSetHexOwner".to_owned();
      self.ExecTypeVarCount[174] = 3;
      self.ExecTypeVarName[174][1] = "X".to_owned();
      self.ExecTypeVarName[174][2] = "Y".to_owned();
      self.ExecTypeVarName[174][3] = "Owner #";
      self.ExecCategory[174] = 4;
      self.ExecTypeNames[175] = "ExecRewardPowerOn".to_owned();
      self.ExecTypeVarCount[175] = 4;
      self.ExecTypeVarName[175][1] = "X".to_owned();
      self.ExecTypeVarName[175][2] = "Y".to_owned();
      self.ExecTypeVarName[175][3] = "Power Above";
      self.ExecTypeVarName[175][4] = "Multi".to_owned();
      self.ExecTypeString[175] = 0;
      self.ExecCategory[175] = 18;
      self.ExecDesc[175] = "If AI can get more then power above on X,Y coordinate. all pts above this treshold will be multiplied by multi and added to score. This gives AI incentive to get a certain build up here. so it must be able to hold it to get it.. This is only used in Init AI so if you use uberregime only the uberregime needs this setting. This setting is wiped clean after every AI has played. You thus need to set this in early turn event.";
      self.ExecTypeNames[176] = "ExecRemoveunit".to_owned();
      self.ExecTypeVarCount[176] = 1;
      self.ExecTypeVarName[176][1] = "Unit #";
      self.ExecCategory[176] = 8;
      self.ExecDesc[176] = "Removes specific Unit";
      self.ExecTypeNames[177] = "ExecSetTurnString".to_owned();
      self.ExecTypeVarCount[177] = 1;
      self.ExecTypeVarName[177][1] = "Short String";
      self.ExecTypeString[177] = 0;
      self.ExecCategory[177] = 10;
      self.ExecDesc[177] = "if you set it to '' then it will not overrule the date or round. Keep the short: String to make it fit.";
      self.ExecTypeNames[178] = "ExecSetRound".to_owned();
      self.ExecTypeVarCount[178] = 1;
      self.ExecTypeVarName[178][1] = "Round".to_owned();
      self.ExecTypeString[178] = 0;
      self.ExecCategory[178] = 10;
      self.ExecDesc[178] = "Do not use! It will screw with the statistics. Never set higher. You can only set lower then current round.";
      self.ExecTypeNames[179] = "ExecSortStringListID".to_owned();
      self.ExecTypeVarCount[179] = 3;
      self.ExecTypeVarName[179][1] = "StringListID#";
      self.ExecTypeVarName[179][2] = "Column# to sort on";
      self.ExecTypeVarName[179][3] = "0=asc, 1=desc";
      self.ExecTypeString[179] = 0;
      self.ExecCategory[179] = 16;
      self.ExecDesc[179] = "Remember first column is column 0. ASCENDING:  highest at top. DESCENDING: lowest at top.  ";
      self.ExecTypeNames[180] = "ExecChangeUnitEntrenchment".to_owned();
      self.ExecTypeVarCount[180] = 2;
      self.ExecTypeVarName[180][1] = "Unit#";
      self.ExecTypeVarName[180][2] = "-/+ entrenchment pts";
      self.ExecTypeString[180] = 0;
      self.ExecCategory[180] = 8;
      self.ExecDesc[180] = "Will auto cut at 0 or maximum entrench for sftype in hex..";
      self.ExecTypeNames[181] = "ExecSetSFTypeVar".to_owned();
      self.ExecTypeVarCount[181] = 3;
      self.ExecTypeVarName[181][1] = "SFType".to_owned();
      self.ExecTypeVarName[181][2] = "Var#";
      self.ExecTypeVarName[181][3] = "Value".to_owned();
      self.ExecTypeString[181] = 0;
      self.ExecCategory[181] = 9;
      self.ExecDesc[181] = "";
      self.ExecTypeNames[182] = "ExecSetHistoricUnitToGroup".to_owned();
      self.ExecTypeVarCount[182] = 2;
      self.ExecTypeVarName[182][1] = "Historic HQ/Unit/Div #";
      self.ExecTypeVarName[182][2] = "To Group with HQ/Unit/Div# in it";
      self.ExecTypeString[182] = 0;
      self.ExecCategory[182] = 18;
      self.ExecDesc[182] = "This should be called in an 'AI Init Check' since it re-orders strategic groups (after makegroups) and before going into strategic analysis (findbeststrategy). ";
      self.ExecTypeNames[183] = "ExecMoveHistoricalUnit".to_owned();
      self.ExecTypeVarCount[183] = 4;
      self.ExecTypeVarName[183][1] = "Historical Unit #";
      self.ExecTypeVarName[183][2] = "New X";
      self.ExecTypeVarName[183][3] = "New Y";
      self.ExecTypeVarName[183][4] = "Optional Historical HQ #";
      self.ExecDesc[183] = "If you specify >0 for optional HQ all units attached to the HQ will be moved. you can specify -1 for historical unit if you specify hq.";
      self.ExecCategory[183] = 13;
      self.ExecTypeNames[185] = "ExecSetUnitName".to_owned();
      self.ExecTypeVarCount[185] = 3;
      self.ExecTypeVarName[185][1] = "Unit#";
      self.ExecTypeVarName[185][2] = "New Name";
      self.ExecTypeVarName[186][3] = "Shortname".to_owned();
      self.ExecCategory[185] = 8;
      self.ExecDesc[185] = "Will set name of all belonging to units historical unit if using historical units. Shortname is only applied if historical unit is present.";
      self.ExecTypeNames[186] = "ExecDateMode".to_owned();
      self.ExecTypeVarCount[186] = 2;
      self.ExecTypeVarName[186][1] = "0=rounds, 1=dates";
      self.ExecTypeVarName[186][2] = "days per round";
      self.ExecCategory[186] = 10;
      self.ExecTypeNames[187] = "ExecUnitPeopleModify".to_owned();
      self.ExecTypeVarCount[187] = 3;
      self.ExecTypeVarName[187][1] = "Unit #";
      self.ExecTypeVarName[187][2] = "Change People# (-1=all)";
      self.ExecTypeVarName[187][3] = "Too People#";
      self.ExecCategory[187] = 8;
      self.ExecDesc[187] = "Change the people in the unit.";
      self.ExecTypeNames[188] = "ExecSetUnitSelected *DEPRECIATED*";
      self.ExecTypeVarCount[188] = 1;
      self.ExecTypeVarName[188][1] = "Unit #";
      self.ExecCategory[188] = 5;
      self.ExecDesc[188] = "Do not confuse this function with ExecUnitSelectable to make a unit choseable in popup. This function provides some backwards compatibility with ATG and otherwise sets the unitselected for the game display. -1= no unit selected.";
      self.ExecTypeNames[190] = "ExecAICombatMoveMod".to_owned();
      self.ExecTypeVarCount[190] = 3;
      self.ExecTypeVarName[190][1] = "Regime #";
      self.ExecTypeVarName[190][2] = "Combat Bonus";
      self.ExecTypeVarName[190][3] = "Movement Bonus";
      self.ExecCategory[190] = 18;
      self.ExecDesc[190] = "Combat bonus can be everything between 0-999%, movement bonus between 0-100%. If you set -1 for combat or move the bonus is not changed.";
      self.ExecTypeNames[191] = "ExecAIChangeRoleScore".to_owned();
      self.ExecTypeVarCount[191] = 3;
      self.ExecTypeVarName[191][1] = "SFType #";
      self.ExecTypeVarName[191][2] = "AI Role # (0-49)";
      self.ExecTypeVarName[191][3] = "New score";
      self.ExecCategory[191] = 18;
      self.ExecDesc[191] = "Allows you to change AI role scores ingame so to influence AI production";
      self.ExecTypeNames[192] = "ExecReloadGraphic".to_owned();
      self.ExecTypeVarCount[192] = 2;
      self.ExecTypeVarName[192][1] = "Current filename";
      self.ExecTypeVarName[192][2] = "New filename";
      self.ExecCategory[192] = 17;
      self.ExecDesc[192] = "expert use only. part or whole of filename (current filename) is looked for in all loaded graphics and replaced by new filename. use only the filename itself like 'background2.png' for the file to be replaced. use only filename like 'background2b.png' for the that: String replaces it. ";
      self.ExecTypeNames[193] = "ExecAIProductionResourceComplient".to_owned();
      self.ExecTypeVarCount[193] = 0;
      self.ExecDesc[193] = "Will call the AI function that makes sure that production is research complient. If rulevar(875) is active this is called by AI after setting production. However if you script the production with events you might want to call this function add the end of that scripting in some scenarios or give the AI unlimited resources instead.";
      self.ExecCategory[193] = 18;
      self.ExecTypeNames[194] = "ExecChangeDipOffer".to_owned();
      self.ExecTypeVarCount[194] = 3;
      self.ExecTypeVarName[194][1] = "RegimeNr1".to_owned();
      self.ExecTypeVarName[194][2] = "RegimeNr2".to_owned();
      self.ExecTypeVarName[194][3] = "Diplomatic Offer # (0=none or 1=offer)";
      self.ExecCategory[194] = 1;
      self.ExecDesc[194] = "Does not give message. You have to that manually. Sets the offer mode to either 0= cancel offer or 1=give offer";
      self.ExecTypeNames[197] = "ExecSetLabel".to_owned();
      self.ExecTypeVarCount[197] = 4;
      self.ExecTypeVarName[197][1] = "X ";
      self.ExecTypeVarName[197][2] = "Y".to_owned();
      self.ExecTypeVarName[197][3] = "Type (0-10)";
      self.ExecTypeVarName[197][4] = "String".to_owned();
      self.ExecCategory[197] = 4;
      self.ExecDesc[197] = "Slot 1 is always used. Auto complete is used and name can stretch several hexes.";
      self.ExecTypeNames[198] = "ExecClearAllLabels".to_owned();
      self.ExecTypeVarCount[198] = 0;
      self.ExecCategory[198] = 4;
      self.ExecTypeNames[199] = "ExecMutateRandomSFType".to_owned();
      self.ExecTypeVarCount[199] = 4;
      self.ExecTypeVarName[199][1] = "People (-1=all)";
      self.ExecTypeVarName[199][2] = "Random percentage";
      self.ExecTypeVarName[199][3] = "From Sftype#";
      self.ExecTypeVarName[199][4] = "Too Sftype# (-1=remove)";
      self.ExecCategory[199] = 2;
      self.ExecDesc[199] = "The random percentage determines ammount of sftype to mutate. After completion sets # of changed individuals in tempvar[999].";
      self.ExecTypeNames[200] = "ExecRemoveTroopsGlobal".to_owned();
      self.ExecTypeVarCount[200] = 4;
      self.ExecTypeVarName[200][1] = "SFType group (-1=all)";
      self.ExecTypeVarName[200][2] = "People (-1=all)";
      self.ExecTypeVarName[200][3] = "Reinforcement Type (-1=all)";
      self.ExecTypeVarName[200][4] = "Percentage to remove";
      self.ExecCategory[200] = 2;
      self.ExecDesc[200] = "Remove percentage is not exact in this function... its the chance for each individual that it will be removed. If it is set to 999 empty HQ units will also be removed. If not only empty normal units will be removed.";
      self.ExecTypeNames[201] = "ExecGiveReinforcement2".to_owned();
      self.ExecTypeVarCount[201] = 4;
      self.ExecTypeVarName[201][1] = "SFType#";
      self.ExecTypeVarName[201][2] = "People#";
      self.ExecTypeVarName[201][3] = "Experience".to_owned();
      self.ExecTypeVarName[201][4] = "Multiplier".to_owned();
      self.ExecCategory[201] = 2;
      self.ExecDesc[201] = "Uses SetCardXY for coordinate to drop troops input.";
      self.ExecTypeNames[202] = "ExecSetMoveMatrix".to_owned();
      self.ExecTypeVarCount[202] = 4;
      self.ExecTypeVarName[202][1] = "Source X";
      self.ExecTypeVarName[202][2] = "Source Y";
      self.ExecTypeVarName[202][3] = "AreaSlot #";
      self.ExecTypeVarName[202][4] = "MoveType #";
      self.ExecCategory[202] = 6;
      self.ExecDesc[202] = "Sets movecost value on each hex for specified movetype. if not reachable will be set to 9999. It will not overwrite a lower value with a higher value. So make sure you set the areaslot matrix to 9999 before you call. this is done to allow multiple sources for making the matrix. The matrix is calculated for the regime that owns the hex.";
      self.ExecTypeNames[203] = "ExecSetHisVar (DEPRECIATED)";
      self.ExecTypeVarCount[203] = 4;
      self.ExecTypeVarName[203][1] = "His.Unit ID#";
      self.ExecTypeVarName[203][2] = "Type #";
      self.ExecTypeVarName[203][3] = "Value".to_owned();
      self.ExecTypeVarName[203][4] = "Nato Counter (0= do not set)";
      self.ExecCategory[203] = 13;
      self.ExecTypeNames[204] = "ExecRemoveHisVar".to_owned();
      self.ExecTypeVarCount[204] = 2;
      self.ExecTypeVarName[204][1] = "His.Unit ID#";
      self.ExecTypeVarName[204][2] = "Type #";
      self.ExecCategory[204] = 13;
      self.ExecTypeNames[205] = "ExecCommanderAddDeckCard2".to_owned();
      self.ExecTypeVarCount[205] = 4;
      self.ExecTypeVarName[205][1] = "Unit# (or -1)";
      self.ExecTypeVarName[205][2] = "Historical Unit# (or -1)";
      self.ExecTypeVarName[205][3] = "DeckCard#";
      self.ExecTypeVarName[205][4] = "Percentage Chance % (or -1,-2,-3,-4,-5,-6)";
      self.ExecCategory[205] = 13;
      self.ExecDesc[205] = "Either specify a normal unit# or a historical unit#. If instead of percentage you specify -1 your remove specified card if present, if you specifify -2 you remove all deck cards. if you specify -3 you'll remove all hand cards and all deck cards. If you specify -4 you'll only remove the hand card and not the deck card. If specify -5 you will not add any card but if the card is present set its chance to 0%. If -6 then you will also not add a card but if card is present set its chance to 100%.";
      self.ExecTypeNames[206] = "ExecCommanderAddAutoEvent".to_owned();
      self.ExecTypeVarCount[206] = 4;
      self.ExecTypeVarName[206][1] = "Unit#";
      self.ExecTypeVarName[206][2] = "AutoEvent#";
      self.ExecTypeVarName[206][3] = "Percentage Chance % (-1=delete if present)";
      self.ExecTypeVarName[206][4] = "His.Unit ID overrule#";
      self.ExecCategory[206] = 13;
      self.ExecDesc[206] = "";
      self.ExecTypeNames[207] = "ExecSetReconUnit".to_owned();
      self.ExecTypeVarCount[207] = 4;
      self.ExecTypeVarName[207][1] = "X".to_owned();
      self.ExecTypeVarName[207][2] = "Y".to_owned();
      self.ExecTypeVarName[207][3] = "RegNr".to_owned();
      self.ExecTypeVarName[207][4] = "Recon points";
      self.ExecCategory[207] = 1;
      self.ExecDesc[207] = "Sets recon for this regime only on this hex and all its neighbours up to range. Uses the ammount of recon points the specified unit# has.";
      self.ExecTypeNames[208] = "ExecCommanderPool".to_owned();
      self.ExecTypeVarCount[208] = 2;
      self.ExecTypeVarName[208][1] = "Historical unit ID#";
      self.ExecTypeVarName[208][2] = "In Pool? 1=yes, 0=no";
      self.ExecTypeVarName[208][3] = "new PP value (-1 = dont change)";
      self.ExecCategory[208] = 12;
      self.ExecTypeNames[209] = "ExecClearCommander".to_owned();
      self.ExecTypeVarCount[209] = 4;
      self.ExecTypeVarName[209][1] = "Historical Unit# (or -1)";
      self.ExecTypeVarName[209][2] = "Clear HandCards (1=yes)";
      self.ExecTypeVarName[209][3] = "Clear DeckCards (1=yes)";
      self.ExecTypeVarName[209][4] = "Clr Events,StaffSiz,Mormod,combatmod (1=yes)";
      self.ExecCategory[209] = 13;
      self.ExecDesc[209] = "Allows you to clean all cards & events associated with a historical unit.";
      self.ExecTypeNames[210] = "ExecChangeCommanderSkill2".to_owned();
      self.ExecTypeVarCount[210] = 4;
      self.ExecTypeVarName[210][1] = "Historical Unit#";
      self.ExecTypeVarName[210][2] = "CombatMod".to_owned();
      self.ExecTypeVarName[210][3] = "MoraleMod".to_owned();
      self.ExecTypeVarName[210][4] = "StaffPts".to_owned();
      self.ExecCategory[210] = 13;
      self.ExecDesc[210] = "Same as other function. But here the historical unit# instead of the unit# is targeted. And the setting is absolute not +/-.";
      self.ExecTypeNames[211] = "ExecSwitchUnitModel".to_owned();
      self.ExecTypeVarCount[211] = 3;
      self.ExecTypeVarName[211][1] = "Unit#";
      self.ExecTypeVarName[211][2] = "To Model ID#";
      self.ExecTypeVarName[211][3] = "Change name as if new unit 0=no ,1=yes";
      self.ExecCategory[211] = 8;
      self.ExecDesc[211] = "This function will attempt to change the whole historical unit# of unit# to a new model and will implement the changes immediately. A new name will be also be given if specified so.";
      self.ExecTypeNames[212] = "ExecClearStringList".to_owned();
      self.ExecTypeVarCount[212] = 1;
      self.ExecTypeVarName[212][1] = "Stringlist ID #";
      self.ExecCategory[212] = 16;
      self.ExecDesc[212] = "Delets all rows of this stringlist.";
      self.ExecTypeNames[213] = "ExecSetPeopleMorale".to_owned();
      self.ExecTypeVarCount[213] = 4;
      self.ExecTypeVarName[213][1] = "People #";
      self.ExecTypeVarName[213][2] = "Under People Group#";
      self.ExecTypeVarName[213][3] = "New Morale";
      self.ExecTypeVarName[213][4] = "Reset all troops 0=no,1=yes";
      self.ExecCategory[213] = 1;
      self.ExecDesc[213] = "Reset=1 sets all morale to regime base morale * people morale as if in start of scenario.";
      self.ExecTypeNames[214] = "AI_SetTacticalScript".to_owned();
      self.ExecTypeVarCount[214] = 4;
      self.ExecTypeVarName[214][1] = "Historical Unit ID#";
      self.ExecTypeVarName[214][2] = "Hex Target X";
      self.ExecTypeVarName[214][3] = "Hex Target Y";
      self.ExecTypeVarName[214][4] = "Attack Towards (1-6) -1=no attack";
      self.ExecCategory[214] = 18;
      self.ExecTypeNames[215] = "ExecActionCardText".to_owned();
      self.ExecTypeVarCount[215] = 3;
      self.ExecTypeVarName[215][1] = "Actioncard Nr";
      self.ExecTypeVarName[215][2] = "New Title";
      self.ExecTypeVarName[215][3] = "New Text on card";
      self.ExecTypeString[215] = 0;
      self.ExecCategory[215] = 5;
      self.ExecDesc[215] = "The difference with ExecActionCardName is that you can here also set the text on the card + input can be done with tempstrings with this exec";
      self.ExecTypeNames[216] = "ExecSetUnitCapacityPoints".to_owned();
      self.ExecTypeVarCount[216] = 4;
      self.ExecTypeVarName[216][1] = "Unit#";
      self.ExecTypeVarName[216][2] = "LandCap Pts +/-";
      self.ExecTypeVarName[216][3] = "NavyCap Pts +/-";
      self.ExecTypeVarName[216][3] = "RailCap Pts +/-";
      self.ExecCategory[216] = 8;
      self.ExecDesc[216] = "Allows you to remove or add capacity points to a unit. Use in late turn so the unit Cap pogain: i32 has already been processed before use of this exec.";
      self.ExecTypeNames[217] = "ExecRemoveTroopsUnderHistorical".to_owned();
      self.ExecTypeVarCount[217] = 4;
      self.ExecTypeVarName[217][1] = "SFType group (-1=all)";
      self.ExecTypeVarName[217][2] = "Historical Unit";
      self.ExecTypeVarName[217][3] = "Reinforcement Type (-1=all)";
      self.ExecTypeVarName[217][4] = "Percentage to remove";
      self.ExecCategory[217] = 2;
      self.ExecDesc[217] = "Remove percentage is not exact in this function... its the chance for each individual that it will be removed. Compared to its namesake exec this one only affects units under command of the historical unit (HQ!) specified. Only units directly under the historical unit are affected and HQs are always excluded. (if you want to set the HQs you'll have to do it manually)";
      self.ExecTypeNames[218] = "ExecSetHistoricalUnitOwner *DEPRECIATED*";
      self.ExecTypeVarCount[218] = 2;
      self.ExecTypeVarName[218][1] = "Old owner #";
      self.ExecTypeVarName[218][2] = "New owner #";
      self.ExecCategory[218] = 2;
      self.ExecDesc[218] = "Function no longer does anything.";
      self.ExecTypeNames[219] = "ExecSetPbemPlayer".to_owned();
      self.ExecTypeVarCount[219] = 2;
      self.ExecTypeVarName[219][1] = "Regime #";
      self.ExecTypeVarName[219][2] = "PBEM++ Player number #";
      self.ExecCategory[219] = 1;
      self.ExecDesc[219] = "Expert use only!. You can switch PBEM++ player here. only set to 1=player 1(challenger) or 2=player 2(opponent).";
      self.ExecTypeNames[220] = "ExecSetDrawGame".to_owned();
      self.ExecTypeVarCount[220] = 0;
      self.ExecCategory[220] = 1;
      self.ExecDesc[220] = "When this is set the game will be considered game over by the PBEM++ server and a draw game at it. Same for non PBEM++ games by the way. ";
      self.ExecTypeNames[221] = "ExecRemoveSupplyUnderHistorical".to_owned();
      self.ExecTypeVarCount[221] = 3;
      self.ExecTypeVarName[221][1] = "Historical Unit";
      self.ExecTypeVarName[221][2] = "Percentage of supply to leave";
      self.ExecTypeVarName[221][3] = "New Supply Consumption % (normal is 100%)";
      self.ExecCategory[221] = 2;
      self.ExecDesc[221] = "this one only affects units under command of the historical unit (HQ!) specified. Only units directly under the historical unit are affected and HQs are always excluded. (if you want to set the HQs you'll have to do it manually)";
      self.ExecTypeNames[222] = "ExecSetInterceptRdnStop".to_owned();
      self.ExecTypeVarCount[222] = 2;
      self.ExecTypeVarName[222][1] = "For Regime (-1=all)";
      self.ExecTypeVarName[222][2] = "Percentage for intercept";
      self.ExecCategory[222] = 2;
      self.ExecDesc[222] = "Only affects units with air units in them. percentage example: 75 means only at 75% rdn.";
      self.ExecTypeNames[223] = "ExecSetHistoricalUnitMaxRecruit".to_owned();
      self.ExecTypeVarCount[223] = 2;
      self.ExecTypeVarName[223][1] = "ID#";
      self.ExecTypeVarName[223][2] = "MaxRecruit#(-1=unl,0=none)";
      self.ExecCategory[223] = 2;
      self.ExecDesc[223] = "A function that allows you to change the max recruit";
      self.ExecTypeNames[224] = "ExecSetHistoricalUnitType".to_owned();
      self.ExecTypeVarCount[224] = 2;
      self.ExecTypeVarName[224][1] = "ID#";
      self.ExecTypeVarName[224][2] = "Type#(5=corps,6=a,7=ag,8=high)";
      self.ExecCategory[224] = 2;
      self.ExecDesc[224] = "A function that allows you to change the type. this is usefull to make 1st panzer army immobile and the sov front too. ";
      self.ExecTypeNames[225] = "ExecChangeDateAdd".to_owned();
      self.ExecTypeVarCount[225] = 3;
      self.ExecTypeVarName[225][1] = "add x Day";
      self.ExecTypeVarName[225][2] = "add x Month";
      self.ExecTypeVarName[225][3] = "add x Year";
      self.ExecCategory[225] = 10;
      self.ExecTypeNames[226] = "ExecSetAllUnitReady".to_owned();
      self.ExecTypeVarCount[226] = 1;
      self.ExecTypeVarName[226][1] = "regime. -1=all";
      self.ExecCategory[226] = 10;
      self.ExecDesc[226] = "Sets all units with max supply, ap and readiness.";
      self.ExecTypeNames[227] = "ExecRemoveActiveOfficersFromStringlist".to_owned();
      self.ExecTypeVarCount[227] = 2;
      self.ExecTypeVarName[227][1] = "regime#";
      self.ExecTypeVarName[227][2] = "stringlist ID";
      self.ExecCategory[227] = 12;
      self.ExecDesc[227] = "Looks up the officer historical ID in column 0 and sets availability to 0 in column 1. Active officers are officers that are in the pool or on the map in a unit.";
      self.ExecTypeNames[228] = "ExecMutateRandomSFTypeUnderHis".to_owned();
      self.ExecTypeVarCount[228] = 4;
      self.ExecTypeVarName[228][1] = "HistoricalID".to_owned();
      self.ExecTypeVarName[228][2] = "Random percentage";
      self.ExecTypeVarName[228][3] = "From Sftype#";
      self.ExecTypeVarName[228][4] = "Too Sftype# (-1=remove)";
      self.ExecCategory[228] = 2;
      self.ExecDesc[228] = "The random percentage determines ammount of sftype to mutate. After completion sets # of changed individuals in tempvar[999]. ";
      self.ExecTypeNames[229] = "ExecRemovePeopleOfficersFromStringlist".to_owned();
      self.ExecTypeVarCount[229] = 2;
      self.ExecTypeVarName[229][1] = "people#";
      self.ExecTypeVarName[229][2] = "stringlist ID";
      self.ExecCategory[229] = 12;
      self.ExecDesc[229] = "Looks up the officer historical ID in column 0 and sets availability to 0 in column 1 if  the officer has specified people#.";
      self.ExecTypeNames[230] = "ExecRemoveFromOfficerPool".to_owned();
      self.ExecTypeVarCount[230] = 4;
      self.ExecTypeVarName[230][1] = "regime# (-1=all)";
      self.ExecTypeVarName[230][2] = "people# (-1=all)";
      self.ExecTypeVarName[230][3] = "VarType (-1=ignore condition)";
      self.ExecTypeVarName[230][4] = "=VarQty";
      self.ExecCategory[230] = 12;
      self.ExecDesc[230] = "Removes any officers from the officerpool of specified regime who has the specified characeristics. You can deted vartype = varqty to tailormake catch a specific officer.";
      self.ExecTypeNames[231] = "ExecModifyStockpiles".to_owned();
      self.ExecTypeVarCount[231] = 2;
      self.ExecTypeVarName[231][1] = "Under Historical Unit#";
      self.ExecTypeVarName[231][2] = "Modifier (100=same, 50=half, 200=double) #";
      self.ExecDesc[231] = "The function will cap at maximum stockpiles. So you can specify 999 or more modifier to give unit max. and 0 to set stockpile to nothing.";
      self.ExecCategory[231] = 9;
      self.ExecTypeNames[232] = "ExecSetRegimeFerryEff".to_owned();
      self.ExecTypeVarCount[232] = 2;
      self.ExecTypeVarName[232][1] = "Regime nr";
      self.ExecTypeVarName[232][2] = "Ferry eff (0=none, 100=full)";
      self.ExecCategory[232] = 1;
      self.ExecTypeNames[233] = "CallFunctionByLibrary".to_owned();
      self.ExecTypeVarCount[233] = 2;
      self.ExecTypeVarName[233][1] = "Name of library";
      self.ExecTypeVarName[233][2] = "Name of function to call";
      self.ExecTypeString[233] = 0;
      self.ExecCategory[233] = 20;
      self.ExecDesc[233] = "Same as CallFunction, but only looks in specific library. This is to avoid double name issues.";
      self.ExecTypeNames[234] = "ExecModifyAttackScore2".to_owned();
      self.ExecTypeVarCount[234] = 4;
      self.ExecTypeVarName[234][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[234][2] = "1=offensive, 2=defensive";
      self.ExecTypeVarName[234][3] = "Versus SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[234][4] = "New value to be multiplicated with current value and divided by 100";
      self.ExecDesc[234] = "1=Modifies AttackArt,AttackPower 2=modifies: AttackPowerDef.";
      self.ExecCategory[234] = 9;
      self.ExecTypeNames[235] = "ExecModifyHitpoints2".to_owned();
      self.ExecTypeVarCount[235] = 3;
      self.ExecTypeVarName[235][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[235][2] = "Versus SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[235][3] = "New value to be multiplicated with current value and divided by 100";
      self.ExecDesc[235] = "";
      self.ExecCategory[235] = 9;
      self.ExecTypeNames[236] = "ExecModifyAttackScore3".to_owned();
      self.ExecTypeVarCount[236] = 4;
      self.ExecTypeVarName[236][1] = "SFType # (-1=all)";
      self.ExecTypeVarName[236][2] = "1=art attack score";
      self.ExecTypeVarName[236][3] = "Versus SFTypeGroup # (-1=all)";
      self.ExecTypeVarName[236][4] = "New value to be multiplicated with current value and divided by 100";
      self.ExecDesc[236] = "1=Modifies AttackArt";
      self.ExecCategory[236] = 9;
      self.ExecTypeNames[237] = "ExecSetGlobalLibVar".to_owned();
      self.ExecTypeVarCount[237] = 3;
      self.ExecTypeVarName[237][1] = "Name of library";
      self.ExecTypeVarName[237][2] = "Name of libvar";
      self.ExecTypeVarName[237][3] = "New value";
      self.ExecTypeString[237] = 0;
      self.ExecCategory[237] = 20;
      self.ExecDesc[237] = "Allows you to set a global text value-type libvar. This basically gives you unlimited alternative to gameslots that are excellent for modular design.";
      self.ExecTypeNames[238] = "ExecSetHexLibVar".to_owned();
      self.ExecTypeVarCount[238] = 3;
      self.ExecTypeVarName[238][1] = "Name of library";
      self.ExecTypeVarName[238][2] = "Name of libvar";
      self.ExecTypeVarName[238][3] = "New value";
      self.ExecTypeString[238] = 0;
      self.ExecCategory[238] = 20;
      self.ExecDesc[238] = "Uses ExecSetCardXY in the line before this call for specifiying the hex this operation is done on. You can only set numeric values to the map. This gives you basically unlimited data slots on map hexes.";
      self.ExecTypeNames[239] = "ExecDynamicMessage".to_owned();
      self.ExecTypeVarCount[239] = 4;
      self.ExecTypeVarName[239][1] = "Regime1".to_owned();
      self.ExecTypeVarName[239][2] = "Regime2".to_owned();
      self.ExecTypeVarName[239][3] = "Message String";
      self.ExecTypeVarName[239][4] = "EventPicture (-1=none)";
      self.ExecCategory[239] = 3;
      self.ExecDesc[239] = "This is the  new DynamicMessage. It allows advanced scripting and utilises system graphics 3 background parts. if reg1=-1 and reg2=-1 then message is send to all regimes. (inside note: Dynamic Message sets a message to Style=3 to notify the engine it uses dynamic message mode and advanced scripting) ";
      self.ExecTypeNames[240] = "ExecSetMessageNameAndGroup".to_owned();
      self.ExecTypeVarCount[240] = 4;
      self.ExecTypeVarName[240][1] = "Name (not: String value) Set empty for: String no-name! ";
      self.ExecTypeVarName[240][2] = "Group (0=none)";
      self.ExecTypeVarName[240][3] = "HideFromTab (1=yes,0=no)";
      self.ExecTypeVarName[240][4] = "HideFromStart (1=yes,0=no)";
      self.ExecCategory[240] = 3;
      self.ExecDesc[240] = "Will be applied to the next ExecMessage2 or ExecDynamicMessage. ";
      self.ExecTypeNames[241] = "ExecSetHisVar2".to_owned();
      self.ExecTypeVarCount[241] = 4;
      self.ExecTypeVarName[241][1] = "His.Unit ID#";
      self.ExecTypeVarName[241][2] = "Type #";
      self.ExecTypeVarName[241][3] = "Value".to_owned();
      self.ExecTypeVarName[241][4] = "SmallGfx (-1= no smallgfx)";
      self.ExecCategory[241] = 13;
      self.ExecDesc[241] = "If a smallGfx is specified the value and the smallgfx will be displayed for officers.";
      self.ExecTypeNames[242] = "ExecSetHistoricalLibVar".to_owned();
      self.ExecTypeVarCount[242] = 4;
      self.ExecTypeVarName[242][1] = "Name of library";
      self.ExecTypeVarName[242][2] = "Slot# of historical unit";
      self.ExecTypeVarName[242][3] = "Name of libvar";
      self.ExecTypeVarName[242][4] = "New value";
      self.ExecTypeString[242] = 0;
      self.ExecCategory[242] = 20;
      self.ExecDesc[242] = "Set a historical unit LibVar. Keep in mind its NOT! the ID asked but the slot number in the array of historical units. You can use this to change the LibVar of a historical unit or historical model.  Since both use historical units data structure behind the scenes.";
      self.ExecTypeNames[243] = "ExecUnitOnTop".to_owned();
      self.ExecTypeVarCount[243] = 1;
      self.ExecTypeVarName[243][1] = "Unit nr#";
      self.ExecCategory[243] = 8;
      self.ExecDesc[243] = "Makes sure this unit nr is on top of the stack in its hex.";
      self.ExecTypeNames[244] = "ExecSetGroupName".to_owned();
      self.ExecTypeVarCount[244] = 2;
      self.ExecTypeVarName[244][1] = "GroupName #";
      self.ExecTypeVarName[244][2] = "New Value";
      self.ExecCategory[244] = 6;
      self.ExecTypeNames[245] = "ExecAppendTempString".to_owned();
      self.ExecTypeVarCount[245] = 2;
      self.ExecTypeVarName[245][1] = "Tempstring #";
      self.ExecTypeVarName[245][2] = "Value to be appended";
      self.ExecCategory[245] = 6;
      self.ExecTypeNames[246] = "ExecSetUnitStandingOrders".to_owned();
      self.ExecTypeVarCount[246] = 4;
      self.ExecTypeVarName[246][1] = "Unit#";
      self.ExecTypeVarName[246][2] = "Retreat%";
      self.ExecTypeVarName[246][3] = "Supply%";
      self.ExecTypeVarName[246][4] = "Replacement%";
      self.ExecCategory[246] = 8;
      self.ExecDesc[246] = "Give unit slot number and percentages. Allows you to set a number of standing order % to a unit. If you specify -1 then the setting is not changed. Only 0-100 values are thus accepted. Retreat100% = fight till last Retreat0%=immediate retreat.";
      self.ExecTypeNames[247] = "ExecClearSpecialTypesOnMap".to_owned();
      self.ExecTypeVarCount[247] = 2;
      self.ExecTypeVarName[247][1] = "Which special type # (-1=all)";
      self.ExecTypeVarName[247][2] = "Which sprite # (-1=all)";
      self.ExecCategory[247] = 4;
      self.ExecDesc[247] = "Remember that special types are just landscape types under another name.";
      self.ExecTypeNames[248] = "ExecSetExtraTab".to_owned();
      self.ExecTypeVarCount[248] = 3;
      self.ExecTypeVarName[248][1] = "Name of tab";
      self.ExecTypeVarName[248][2] = "Event slot# generating message to be used";
      self.ExecTypeVarName[248][3] = "tab# (1,2 or 3)";
      self.ExecDesc[28] = "Remember that setting the name to nothing '' disables the extra tab. Also remember that the event called is supposed to generate 1 message only: no more, no less. The message contents should fit in a 1000x200 pixel area. Not setting the third parameter or to 0 results in tab#=1.";
      self.ExecCategory[248] = 10;
      self.ExecTypeNames[249] = "ExecSetCardQuickButton".to_owned();
      self.ExecTypeVarCount[249] = 3;
      self.ExecTypeVarName[249][1] = "Card Slot#";
      self.ExecTypeVarName[249][2] = "Quick Mode (0=no,1=yes,2=yes+hide)";
      self.ExecTypeVarName[249][3] = "Small graphic on order button";
      self.ExecDesc[249] = "If mode>=1 then an order button will be generated if the player/ selected hex/ selected unit confirms to the (pre-event) conditions of playing a card in general/ on hex/ on unit. And enough PP must be available. You must specifiy a small graphic slot # as well for the order button.";
      self.ExecCategory[249] = 5;
      self.ExecTypeNames[250] = "ExecSetSmallLabel".to_owned();
      self.ExecTypeVarCount[250] = 3;
      self.ExecTypeVarName[250][1] = "X ";
      self.ExecTypeVarName[250][2] = "Y".to_owned();
      self.ExecTypeVarName[250][3] = "Text".to_owned();
      self.ExecCategory[250] = 4;
      self.ExecDesc[250] = "Set to empty to: String reset a label to nothing. Max 9 letters will be displayed in 64x48 hex mode. but label texts can be bigger, they will just not be fully shown.";
      self.ExecTypeNames[251] = "ExecSetHistoricalUnitName".to_owned();
      self.ExecTypeVarCount[251] = 3;
      self.ExecTypeVarName[251][1] = "Historical Unit ID";
      self.ExecTypeVarName[251][2] = "New name";
      self.ExecTypeVarName[251][3] = "New short name";
      self.ExecCategory[251] = 13;
      self.ExecTypeNames[252] = "ExecPlayEventBackgroundWav2".to_owned();
      self.ExecTypeVarCount[252] = 1;
      self.ExecTypeVarName[252][1] = "Full file path relative to the game executable";
      self.ExecCategory[252] = 3;
      self.ExecDesc[252] = "Loads the sound file. ";
      self.ExecTypeNames[253] = "ExecActionCardCategory".to_owned();
      self.ExecTypeVarCount[253] = 2;
      self.ExecTypeVarName[253][1] = "Actioncard Nr";
      self.ExecTypeVarName[253][2] = "New category number";
      self.ExecTypeString[253] = 0;
      self.ExecCategory[253] = 5;
      self.ExecDesc[253] = "For changing the category numner of the card";
      self.ExecTypeNames[254] = "ExecDeleteActionCard".to_owned();
      self.ExecTypeVarCount[254] = 1;
      self.ExecTypeVarName[254][1] = "Card Nr";
      self.ExecCategory[254] = 5;
      self.ExecDesc[254] = "*USE WITH CAUTION* This exec actually really deletes the card. Use ExecRemoveActionCard to keep the card but remove it from a regime hand. Deleting cards will change numbers it might cause events to break. *EXPERT USE ONLY*";
      self.ExecTypeCount = 254;
      self.ExecTypeNames[ byte.MaxValue] = "ExecSetActionCardEventPicture".to_owned();
      self.ExecTypeVarCount[ byte.MaxValue] = 2;
      self.ExecTypeVarName[ byte.MaxValue, 1] = "Card Nr";
      self.ExecTypeVarName[ byte.MaxValue, 2] = "Event Picture Nr";
      self.ExecCategory[ byte.MaxValue] = 5;
      self.ExecDesc[ byte.MaxValue] = "A simple replace picture of the card. ";
      self.ExecTypeNames[256] = "ExecChanceOnDeathIfMakesHit".to_owned();
      self.ExecTypeVarCount[256] = 2;
      self.ExecTypeVarName[256][1] = "SFType#";
      self.ExecTypeVarName[256][2] = "Percentage%";
      self.ExecTypeString[256] = 0;
      self.ExecCategory[256] = 9;
      self.ExecDesc[256] = "0% is will never die due to making a hit on enemy. 1=1% chance, etc... ";
      self.ExecTypeNames[257] = "ExecWritePDF".to_owned();
      self.ExecTypeVarCount[257] = 3;
      self.ExecTypeVarName[257][1] = "String".to_owned();
      self.ExecTypeVarName[257][2] = "Filename".to_owned();
      self.ExecTypeVarName[257][3] = "Title".to_owned();
      self.ExecCategory[257] = 3;
      self.ExecDesc[257] = "The String should be coded like a dynamic message. Pagebreaks are not automatic. You need to use CheckDynamicTextHeight() to predict sizes. Pages are 595 x 842 points. Filename should be without file extension. for example 'doc' or 'oobfile'. Title is displayed as a watermarkish thing on each page on top and pagenumber in the bottom. ";
      self.ExecTypeNames[258] = "ExecGiveSupplyImmediateDisperse".to_owned();
      self.ExecTypeVarCount[258] = 4;
      self.ExecTypeVarName[258][1] = "X ";
      self.ExecTypeVarName[258][2] = "Y".to_owned();
      self.ExecTypeVarName[258][3] = "Supply Points to be shared by all units";
      self.ExecTypeVarName[258][4] = "AP modifier %. 100=normal. 50=only half range. 200=double range.";
      self.ExecDesc[258] = "This exec acts basically like an airdrop from ATG would work. All units in range, modified for distance immediately receive supplies. Stats are updated. It uses rulevar 99 for movement type. ap range is rulevar 3. penalties used from rulevar 51,52,53. You should call this in a late turn event.";
      self.ExecCategory[258] = 4;
      self.ExecTypeNames[259] = "ExecChangeRoadTypeMoveCost".to_owned();
      self.ExecTypeVarCount[259] = 3;
      self.ExecTypeVarName[259][1] = "RoadType #";
      self.ExecTypeVarName[259][2] = "MovementType #";
      self.ExecTypeVarName[259][3] = "AP Cost";
      self.ExecDesc[259] = "";
      self.ExecCategory[259] = 11;
      self.ExecTypeNames[260] = "ExecExportStringlist".to_owned();
      self.ExecTypeVarCount[260] = 2;
      self.ExecTypeVarName[260][1] = "StringListID#";
      self.ExecTypeVarName[260][2] = "Filename".to_owned();
      self.ExecCategory[260] = 16;
      self.ExecDesc[260] = "Will save the stringlist data inside specified filename. File contents will be completely overwritten. Please supply a file extenstion like .txt in the name. Will be saved in the 'metadata' directory. ";
      self.ExecTypeNames[261] = "ExecMetrics".to_owned();
      self.ExecTypeVarCount[261] = 4;
      self.ExecTypeVarName[261][1] = "Custom string/numeric data 1";
      self.ExecTypeVarName[261][2] = "Custom string/numeric data 2";
      self.ExecTypeVarName[261][3] = "Custom string/numeric data 3";
      self.ExecTypeVarName[261][4] = "Custom string/numeric data 4";
      self.ExecCategory[261] = 16;
      self.ExecDesc[261] = "Will use the URL in 2nd of: String the metrics.txt to pass on this data.";
      self.ExecTypeNames[262] = "ExecKey".to_owned();
      self.ExecTypeVarCount[262] = 3;
      self.ExecTypeVarName[262][1] = "StringListID#";
      self.ExecTypeVarName[262][2] = "Key Name";
      self.ExecTypeVarName[262][3] = "New Value";
      self.ExecCategory[262] = 16;
      self.ExecDesc[262] = "Presumes a stringlist with in column 0 the key names and in column 1 the text values. ";
      self.ExecTypeNames[263] = "ExecKeyAdd".to_owned();
      self.ExecTypeVarCount[263] = 3;
      self.ExecTypeVarName[263][1] = "StringListID#";
      self.ExecTypeVarName[263][2] = "Key Name";
      self.ExecTypeVarName[263][3] = "Value to be appended at end of string";
      self.ExecCategory[263] = 16;
      self.ExecDesc[263] = "Presumes a stringlist with in column 0 the key names and in column 1 the text values. Data is treated as STRINGS!! ";
      self.ExecTypeNames[264] = "ExecSetUDStabText".to_owned();
      self.ExecTypeVarCount[264] = 1;
      self.ExecTypeVarName[264][1] = "UDS Text";
      self.ExecCategory[264] = 21;
      self.ExecDesc[264] = "To set the UDS text to appear in a management tab that is selected by user. ";
      self.ExecTypeNames[265] = "ExecKeyAddNumeric".to_owned();
      self.ExecTypeVarCount[265] = 3;
      self.ExecTypeVarName[265][1] = "StringListID#";
      self.ExecTypeVarName[265][2] = "Key Name";
      self.ExecTypeVarName[265][3] = "Value to be appended at end of string";
      self.ExecCategory[265] = 16;
      self.ExecDesc[265] = "Presumes a stringlist with in column 0 the key names and in column 1 the text values. Data is treated as NUMERIC. ";
      self.ExecTypeNames[266] = "ExecSetUDSpopupText".to_owned();
      self.ExecTypeVarCount[266] = 2;
      self.ExecTypeVarName[266][1] = "UDS Text";
      self.ExecTypeVarName[266][2] = "UDS Popuptype ";
      self.ExecCategory[266] = 21;
      self.ExecDesc[266] = "To set the UDS text to appear in a popup. Popup Type 0+1 = regular text popup ";
      self.ExecTypeNames[267] = "ExecSetInterfaceCue".to_owned();
      self.ExecTypeVarCount[267] = 1;
      self.ExecTypeVarName[267][1] = "Cue Type Number";
      self.ExecCategory[267] = 22;
      self.ExecDesc[267] = "Cue Type 1 = Exit Random Screen, go the scn. setup screen. 2 = Refresh everything ";
      self.ExecTypeNames[268] = "ExecNewMapSize".to_owned();
      self.ExecTypeVarCount[268] = 3;
      self.ExecTypeVarName[268][1] = "Width in hexes";
      self.ExecTypeVarName[268][2] = "Height in hexes";
      self.ExecTypeVarName[268][3] = "Maploop (0=no,1=yes)";
      self.ExecCategory[268] = 22;
      self.ExecDesc[268] = "Keep in mind first hex is 0,0. specifying width=1 and height=1 will result in a grand total of (1x1) 1 hex on map. width=4 an height=4 will give you 16 hexes. etc... Make sure to use even width for mapLoop enabled maps.";
      self.ExecTypeNames[269] = "ExecCopyHexToArea".to_owned();
      self.ExecTypeVarCount[269] = 3;
      self.ExecTypeVarName[269][1] = "From Library Name";
      self.ExecTypeVarName[269][2] = "From Hex LibVar Name";
      self.ExecTypeVarName[269][3] = "Too AreaSlot #";
      self.ExecCategory[269] = 22;
      self.ExecDesc[269] = "Copies numeric values from libvar to areaslot";
      self.ExecTypeNames[270] = "ExecCopyAreaToHex".to_owned();
      self.ExecTypeVarCount[270] = 3;
      self.ExecTypeVarName[270][1] = "From AreaSlot #";
      self.ExecTypeVarName[270][2] = "Too Library Name";
      self.ExecTypeVarName[270][3] = "Too Hex LibVar Name";
      self.ExecCategory[270] = 22;
      self.ExecDesc[270] = "Copies numeric values from libvar to areaslot";
      self.ExecTypeNames[271] = "ExecCopyAreaToArea".to_owned();
      self.ExecTypeVarCount[271] = 2;
      self.ExecTypeVarName[271][1] = "From AreaSlot #";
      self.ExecTypeVarName[271][2] = "Too AreaSlot #";
      self.ExecCategory[271] = 22;
      self.ExecDesc[271] = "Copies numeric values from one areaslot to another areaslot";
      self.ExecTypeNames[272] = "ExecAreaSet".to_owned();
      self.ExecTypeVarCount[272] = 4;
      self.ExecTypeVarName[272][1] = "AreaSlot#";
      self.ExecTypeVarName[272][2] = "New Value";
      self.ExecTypeVarName[272][3] = "Optional Second AreaSlot#";
      self.ExecTypeVarName[272][4] = "Second Slot Value";
      self.ExecCategory[272] = 22;
      self.ExecDesc[272] = "Set the values of all the hexes. If -1 is specified second areaslot is not used. If second areaslot is used only hexes with specified second slot value are changed. You can of course specify the same areaslot for the second to use this exec as a replace function.";
      self.ExecTypeNames[273] = "ExecAreaAdd".to_owned();
      self.ExecTypeVarCount[273] = 4;
      self.ExecTypeVarName[273][1] = "AreaSlot#";
      self.ExecTypeVarName[273][2] = "Value Change +/-";
      self.ExecTypeVarName[273][3] = "Optional Second AreaSlot#";
      self.ExecTypeVarName[273][4] = "Second Slot Value";
      self.ExecCategory[273] = 22;
      self.ExecDesc[273] = "Modifies the values of all the hexes. If -1 is specified second areaslot is not used. If second areaslot is used only hexes with specified second slot value are changed.";
      self.ExecTypeNames[274] = "ExecAreaExpand".to_owned();
      self.ExecTypeVarCount[274] = 4;
      self.ExecTypeVarName[274][1] = "AreaSlot#";
      self.ExecTypeVarName[274][2] = "Value increase";
      self.ExecTypeVarName[274][3] = "Optional Second AreaSlot# (or -2 for random mode)";
      self.ExecTypeVarName[274][4] = "Second Slot Value (or randomness 0-100)";
      self.ExecCategory[274] = 22;
      self.ExecDesc[274] = "Value increase must be >0, 0 or <0. Any value in the areaslot >0 will be expand to neighbouring hexes with lower values and the new value of that hex will be old value +/- value change. Allows you to create distance maps.  If -1 is specified second areaslot is not used. If second areaslot is used only hexes with specified second slot value are changed. If value change 0 is used the values are just copied into any neighbours with 0 value.";
      self.ExecTypeNames[275] = "ExecAreaRangeChange".to_owned();
      self.ExecTypeVarCount[275] = 4;
      self.ExecTypeVarName[275][1] = "AreaSlot#";
      self.ExecTypeVarName[275][2] = "Minimum or equal value";
      self.ExecTypeVarName[275][3] = "Maximum or equal value";
      self.ExecTypeVarName[275][4] = "New value";
      self.ExecCategory[275] = 22;
      self.ExecDesc[275] = "If value is beween min or max or equal than its changed for a new value. Ideal for creating a mask to be used as a second areaslot for another areaslot modifying exec.";
      self.ExecTypeNames[276] = "ExecAreaToStringlist".to_owned();
      self.ExecTypeVarCount[276] = 2;
      self.ExecTypeVarName[276][1] = "AreaSlot #";
      self.ExecTypeVarName[276][2] = "Stringlist ID#";
      self.ExecCategory[276] = 22;
      self.ExecDesc[276] = "Each UNIQUE value in the areaslots will be put in column 0 and the count of the number of hexes with that value in column 1.";
      self.ExecTypeNames[277] = "ExecFindBorders".to_owned();
      self.ExecTypeVarCount[277] = 4;
      self.ExecTypeVarName[277][1] = "AreaSlot#";
      self.ExecTypeVarName[277][2] = "Value A";
      self.ExecTypeVarName[277][3] = "Value B";
      self.ExecTypeVarName[277][4] = "Border Mode";
      self.ExecCategory[277] = 22;
      self.ExecDesc[277] = "Will set all values to 0 except for some that are set to 1 and that are those that are value A with a neighbour hex with value B. Value A can be a specific value or -1 for all values. Value B the same. Even if both values are set to -1 a border will not be detected if hex value and its neighbour value are the same. Only hexes with a value higher >0 are taken into consideration. BorderMode 1 is flag both value A and value B, BorderMode 2 is just flag value A hexes.  ";
      self.ExecTypeNames[278] = "ExecScreenshot".to_owned();
      self.ExecTypeVarCount[278] = 2;
      self.ExecTypeVarName[278][1] = "AreaSlot#";
      self.ExecTypeVarName[278][2] = "Filename".to_owned();
      self.ExecCategory[278] = 10;
      self.ExecDesc[278] = "Please give the filename without extension. .jpg will be added automaticly. just 'test' will do. Or 'pic12'.  ";
      self.ExecTypeNames[280] = "ExecSetStringListQuick".to_owned();
      self.ExecTypeVarCount[280] = 4;
      self.ExecTypeVarName[280][1] = "Stringlist ID #";
      self.ExecTypeVarName[280][2] = "Rows where Col0 is..";
      self.ExecTypeVarName[280][3] = "Col".to_owned();
      self.ExecTypeVarName[280][4] = "New Value";
      self.ExecDesc[280] = "This Quick version of ExecSetStringList allows you to operate quickly if the col0 of the stringlist are ID numbers. Avoids you from having to loop through the whole thing. ";
      self.ExecCategory[280] = 16;
      self.ExecTypeNames[281] = "ExecAreaJoin".to_owned();
      self.ExecTypeVarCount[281] = 4;
      self.ExecTypeVarName[281][1] = "Target AreaSlot#";
      self.ExecTypeVarName[281][2] = "Source AreaSlot#";
      self.ExecTypeVarName[281][3] = "Mode".to_owned();
      self.ExecTypeVarName[281][4] = "Percentage (100=advise)";
      self.ExecCategory[281] = 22;
      self.ExecDesc[281] = "Mode = 1 is copy to target if higher. Mode = 2 is make average. The source value will be modified by percentage before applying mode and the operation.";
      self.ExecTypeNames[282] = "ExecAreaBlur".to_owned();
      self.ExecTypeVarCount[282] = 4;
      self.ExecTypeVarName[282][1] = "AreaSlot#";
      self.ExecTypeVarName[282][2] = "Distance".to_owned();
      self.ExecTypeVarName[282][3] = "Percentage to blur";
      self.ExecTypeVarName[282][4] = "Mode".to_owned();
      self.ExecCategory[282] = 22;
      self.ExecDesc[282] = "Mode=1 is all blur results are accepteble. Mode=2 only higher resulting values are exceptable";
      self.ExecTypeNames[283] = "ExecPopup".to_owned();
      self.ExecTypeVarCount[283] = 1;
      self.ExecTypeVarName[283][1] = "Message".to_owned();
      self.ExecCategory[283] = 10;
      self.ExecDesc[283] = "Causes a Windows Popup that halts all further event execution untill OK is pressed. Ideal for debugging.  ";
      self.ExecTypeNames[284] = "ExecAreaBlur2".to_owned();
      self.ExecTypeVarCount[284] = 4;
      self.ExecTypeVarName[284][1] = "AreaSlot#";
      self.ExecTypeVarName[284][2] = "Percentage to Blur";
      self.ExecTypeVarName[284][3] = "Optional Second Areaslot#";
      self.ExecTypeVarName[284][4] = "Value required at minimum";
      self.ExecCategory[284] = 22;
      self.ExecDesc[284] = "Adaption of ExecAreaBlur. Uses Distance = 1 and mode that allows only lower results. It is meant to simulate erosion on land masses. ";
      self.ExecTypeNames[285] = "ExecAreaDivide".to_owned();
      self.ExecTypeVarCount[285] = 4;
      self.ExecTypeVarName[285][1] = "AreaSlot#";
      self.ExecTypeVarName[285][2] = "Divide by value";
      self.ExecTypeVarName[285][3] = "Optional Second AreaSlot#";
      self.ExecTypeVarName[285][4] = "Second Slot Value";
      self.ExecCategory[285] = 22;
      self.ExecDesc[285] = "Modifies the values of all the hexes. If -1 is specified second areaslot is not used. If second areaslot is used only hexes with specified second slot value are changed.";
      self.ExecTypeNames[286] = "ExecSetStringPart".to_owned();
      self.ExecTypeVarCount[286] = 3;
      self.ExecTypeVarName[286][1] = "Tempstring #";
      self.ExecTypeVarName[286][2] = "Part #";
      self.ExecTypeVarName[286][3] = "String/Value";
      self.ExecCategory[286] = 16;
      self.ExecDesc[286] = "Sets specified part to the string/value specified.";
      self.ExecTypeNames[287] = "ExecStringPartAdd".to_owned();
      self.ExecTypeVarCount[287] = 2;
      self.ExecTypeVarName[287][1] = "Tempstring #";
      self.ExecTypeVarName[287][2] = "String/Value";
      self.ExecCategory[287] = 16;
      self.ExecDesc[287] = "Adds a new subpart with string/value specified at the end of the tempstring#.";
      self.ExecTypeNames[288] = "ExecSetShrowd".to_owned();
      self.ExecTypeVarCount[288] = 1;
      self.ExecTypeVarName[288][1] = "On(1)/Off(0)";
      self.ExecCategory[288] = 22;
      self.ExecDesc[288] = "Switch shrowd on/off.";
      self.ExecTypeNames[289] = "ExecAreaMinMax".to_owned();
      self.ExecTypeVarCount[289] = 3;
      self.ExecTypeVarName[289][1] = "AreaSlot#";
      self.ExecTypeVarName[289][2] = "Minimum value";
      self.ExecTypeVarName[289][3] = "Maximum value";
      self.ExecCategory[289] = 22;
      self.ExecDesc[289] = "Assures no value in area is below minimum or above maximum.";
      self.ExecTypeNames[290] = "ExecAreaRandomize".to_owned();
      self.ExecTypeVarCount[290] = 4;
      self.ExecTypeVarName[290][1] = "AreaSlot#";
      self.ExecTypeVarName[290][2] = "Maximum negative randomize";
      self.ExecTypeVarName[290][3] = "Maximum positive randomize";
      self.ExecTypeVarName[290][4] = "Only randomize values above or equal value";
      self.ExecCategory[290] = 22;
      self.ExecDesc[290] = "Changes the areaslot values based on random rolls.";
      self.ExecTypeNames[305] = "ExecSetUDSbottomText".to_owned();
      self.ExecTypeVarCount[305] = 1;
      self.ExecTypeVarName[305][1] = "UDS Text";
      self.ExecCategory[305] = 21;
      self.ExecDesc[305] = "To set the UDS text to appear in a bottom tab that is selected by user. ";
      self.ExecTypeNames[308] = "ExecChangeLocationLISitem".to_owned();
      self.ExecTypeVarCount[308] = 4;
      self.ExecTypeVarName[308][1] = "Location slot#";
      self.ExecTypeVarName[308][2] = "LIS item ID";
      self.ExecTypeVarName[308][3] = "Qty".to_owned();
      self.ExecTypeVarName[308][4] = "Extra instruction";
      self.ExecCategory[308] = 21;
      self.ExecDesc[308] = "Extra instruction<1 is ADD/SUBTRACT. Extra instruction 1 is OVERWRITE.";
      self.ExecTypeNames[309] = "ExecChangeUnitLISitem".to_owned();
      self.ExecTypeVarCount[309] = 4;
      self.ExecTypeVarName[309][1] = "Unit slot#";
      self.ExecTypeVarName[309][2] = "LIS item ID";
      self.ExecTypeVarName[309][3] = "Qty".to_owned();
      self.ExecTypeVarName[309][4] = "Extra instruction";
      self.ExecCategory[309] = 21;
      self.ExecDesc[309] = "Extra instruction<1 is ADD/SUBTRACT. Extra instruction 1 is OVERWRITE.";
      self.ExecTypeNames[312] = "ExecClearLISpoints".to_owned();
      self.ExecTypeVarCount[312] = 0;
      self.ExecCategory[312] = 21;
      self.ExecDesc[312] = "Clear all temporary LIS points and free AP points. They are used by the hardcoded LIS_SetNetwork. ";
      self.ExecTypeNames[313] = "ExecSetLISpoints".to_owned();
      self.ExecTypeVarCount[313] = 4;
      self.ExecTypeVarName[313][1] = "Location slot#";
      self.ExecTypeVarName[313][2] = "LIS Transport Mode ID";
      self.ExecTypeVarName[313][3] = "(optional) Target ID";
      self.ExecTypeVarName[313][4] = "Add Points";
      self.ExecCategory[313] = 21;
      self.ExecDesc[313] = "LIS Points are added to the ID+Target combination. Put -1 if no target. ";
      self.ExecTypeNames[314] = "ExecSetLISfreeAP".to_owned();
      self.ExecTypeVarCount[314] = 4;
      self.ExecTypeVarName[314][1] = "Location slot#";
      self.ExecTypeVarName[314][2] = "LIS Transport Mode ID";
      self.ExecTypeVarName[314][3] = "(optional) Target ID";
      self.ExecTypeVarName[314][4] = "Add Free AP";
      self.ExecCategory[314] = 21;
      self.ExecDesc[314] = "LIS Free AP Points are added to the ID+Target combination. Put -1 if no target. ";
      self.ExecTypeNames[343] = "ExecSetLogic".to_owned();
      self.ExecTypeVarCount[343] = 3;
      self.ExecCategory[343] = 22;
      self.ExecTypeVarName[343][1] = "Flag Stringlist ID";
      self.ExecTypeVarName[343][2] = "Flag Instructions Stringlist ID";
      self.ExecTypeVarName[343][3] = "Logic String";
      self.ExecDesc[343] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[292] = "ExecMakeErosionAndRainMap".to_owned();
      self.ExecTypeVarCount[292] = 1;
      self.ExecTypeVarName[292][1] = "Stringlist ID for Planetary Data";
      self.ExecCategory[292] = 22;
      self.ExecDesc[292] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[293] = "ExecMakeHeightMapBasedOnTectonicPlates".to_owned();
      self.ExecTypeVarCount[293] = 1;
      self.ExecTypeVarName[293][1] = "Stringlist ID for Planetary Data";
      self.ExecCategory[293] = 22;
      self.ExecDesc[293] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[294] = "ExecMakeMountainsCorrectHeight".to_owned();
      self.ExecTypeVarCount[294] = 1;
      self.ExecTypeVarName[294][1] = "Stringlist ID for Planetary Data";
      self.ExecCategory[294] = 22;
      self.ExecDesc[294] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[295] = "ExecMakeVegetation".to_owned();
      self.ExecTypeVarCount[295] = 4;
      self.ExecTypeVarName[295][1] = "Stringlist ID for Planetary Data";
      self.ExecTypeVarName[295][2] = "Stringlist ID for Helper Table";
      self.ExecTypeVarName[295][3] = "Stringlist ID for Translation Table";
      self.ExecTypeVarName[295][4] = "Last call 0=no, 1=yes";
      self.ExecCategory[295] = 22;
      self.ExecDesc[295] = "The last call sets the origLandscapes. and is the powere: i32 we should save the vegetation into them. ";
      self.ExecTypeNames[296] = "ExecMakeZones".to_owned();
      self.ExecTypeVarCount[296] = 4;
      self.ExecTypeVarName[296][1] = "Stringlist ID for Planetary Data";
      self.ExecTypeVarName[296][2] = "Stringlist ID for Vegetation Similarity Table";
      self.ExecTypeVarName[296][3] = "Library name Random";
      self.ExecTypeVarName[296][4] = "Library name Data";
      self.ExecCategory[296] = 22;
      self.ExecDesc[296] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[297] = "ExecMakeBiosphereAndAtmosphere".to_owned();
      self.ExecTypeVarCount[297] = 2;
      self.ExecTypeVarName[297][1] = "Random Library Name";
      self.ExecTypeVarName[297][2] = "Data Library Name";
      self.ExecCategory[297] = 22;
      self.ExecDesc[297] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[299] = "ExecMakeBiosphereDetail".to_owned();
      self.ExecTypeVarCount[299] = 3;
      self.ExecTypeVarName[299][1] = "Random Library Name";
      self.ExecTypeVarName[299][2] = "Data Library Name";
      self.ExecTypeVarName[299][3] = "Linguistic Library Name";
      self.ExecCategory[299] = 22;
      self.ExecDesc[299] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[300] = "ExecMakeBiosphereSimulation".to_owned();
      self.ExecTypeVarCount[300] = 4;
      self.ExecTypeVarName[300][1] = "Random Library Name";
      self.ExecTypeVarName[300][2] = "Data Library Name";
      self.ExecTypeVarName[300][3] = "Initial Call (0/1)";
      self.ExecTypeVarName[300][4] = "Number of rounds to simulate";
      self.ExecCategory[300] = 22;
      self.ExecDesc[300] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[301] = "ExecMakeColonyLanding".to_owned();
      self.ExecTypeVarCount[301] = 3;
      self.ExecTypeVarName[301][1] = "Random Library Name";
      self.ExecTypeVarName[301][2] = "Data Library Name";
      self.ExecTypeVarName[301][3] = "Linguistics Library Name";
      self.ExecCategory[301] = 22;
      self.ExecDesc[301] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[302] = "ExecMakeColonyHistory".to_owned();
      self.ExecTypeVarCount[302] = 3;
      self.ExecTypeVarName[302][1] = "Random Library Name";
      self.ExecTypeVarName[302][2] = "Data Library Name";
      self.ExecTypeVarName[302][3] = "Linguistics Library Name";
      self.ExecCategory[302] = 22;
      self.ExecDesc[302] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[303] = "ExecMakeDissolutionHistory".to_owned();
      self.ExecTypeVarCount[303] = 3;
      self.ExecTypeVarName[303][1] = "Random Library Name";
      self.ExecTypeVarName[303][2] = "Data Library Name";
      self.ExecTypeVarName[303][3] = "Linguistics Library Name";
      self.ExecCategory[303] = 22;
      self.ExecDesc[303] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[304] = "ExecMakeSurvivors".to_owned();
      self.ExecTypeVarCount[304] = 3;
      self.ExecTypeVarName[304][1] = "Random Library Name";
      self.ExecTypeVarName[304][2] = "Data Library Name";
      self.ExecTypeVarName[304][3] = "Linguistics Library Name";
      self.ExecCategory[304] = 22;
      self.ExecDesc[304] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[315] = "ExecMakeListForLocationRequests".to_owned();
      self.ExecTypeVarCount[315] = 2;
      self.ExecTypeVarName[315][1] = "Data Library Name";
      self.ExecTypeVarName[315][2] = "Zone ID or -1 for all";
      self.ExecCategory[315] = 22;
      self.ExecDesc[315] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[316] = "ExecMakeListForLocationReturns".to_owned();
      self.ExecTypeVarCount[316] = 2;
      self.ExecTypeVarName[316][1] = "Data Library Name";
      self.ExecTypeVarName[316][2] = "Zone ID or -1 for all";
      self.ExecCategory[316] = 22;
      self.ExecDesc[316] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[317] = "ExecMakeFreeLocationReturns".to_owned();
      self.ExecTypeVarCount[317] = 0;
      self.ExecCategory[317] = 22;
      self.ExecDesc[317] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[318] = "ExecMakeProduction".to_owned();
      self.ExecTypeVarCount[318] = 3;
      self.ExecTypeVarName[318][1] = "Data Library Name";
      self.ExecTypeVarName[318][2] = "Zone ID or -1 for all";
      self.ExecTypeVarName[318][3] = "Free of cost 0/1";
      self.ExecCategory[318] = 22;
      self.ExecDesc[318] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[319] = "ExecMakeZoneEconomy".to_owned();
      self.ExecTypeVarCount[319] = 4;
      self.ExecTypeVarName[319][1] = "Data Library Name";
      self.ExecTypeVarName[319][2] = "Zone ID or -1 for all";
      self.ExecTypeVarName[319][3] = "Free of cost 0/1";
      self.ExecTypeVarName[319][4] = "Game Setup 0/1";
      self.ExecCategory[319] = 22;
      self.ExecDesc[319] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[320] = "ExecMakeAssetPresentation".to_owned();
      self.ExecTypeVarCount[320] = 3;
      self.ExecTypeVarName[320][1] = "Data Library Name";
      self.ExecTypeVarName[320][2] = "Asset Row Number";
      self.ExecTypeVarName[320][3] = "Mode".to_owned();
      self.ExecCategory[320] = 22;
      self.ExecDesc[320] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc. Mode 0/1=normal, >1=all the ones above this count, -1=all";
      self.ExecTypeNames[321] = "ExecHardcodedScript".to_owned();
      self.ExecTypeVarCount[321] = 2;
      self.ExecTypeVarName[321][1] = "Hardcoded Number >0";
      self.ExecTypeVarName[321][2] = "Create Feedback Message 0/1";
      self.ExecCategory[321] = 22;
      self.ExecDesc[321] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc. Mode 0/1=normal, >1=all the ones above this count, -1=all";
      self.ExecTypeNames[322] = "ExecMakeUnitUpkeepCheck".to_owned();
      self.ExecTypeVarCount[322] = 1;
      self.ExecTypeVarName[322][1] = "Data Library Name";
      self.ExecCategory[322] = 22;
      self.ExecDesc[322] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc. Mode 0/1=normal, >1=all the ones above this count, -1=all";
      self.ExecTypeNames[323] = "ExecMakeListForUnitRequests".to_owned();
      self.ExecTypeVarCount[323] = 2;
      self.ExecTypeVarName[323][1] = "Data Library Name";
      self.ExecTypeVarName[323][2] = "historical unit ID or -1 for all";
      self.ExecCategory[323] = 22;
      self.ExecDesc[323] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[324] = "ExecMakeFreeUnitSupply".to_owned();
      self.ExecTypeVarCount[324] = 0;
      self.ExecCategory[324] = 22;
      self.ExecDesc[324] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[325] = "ExecRemoveLIS".to_owned();
      self.ExecTypeVarCount[325] = 4;
      self.ExecTypeVarName[325][1] = "Loc nr";
      self.ExecTypeVarName[325][2] = "To X";
      self.ExecTypeVarName[325][3] = "To Y";
      self.ExecTypeVarName[325][3] = "LIS points";
      self.ExecCategory[325] = 22;
      self.ExecDesc[325] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc. Mode 0/1=normal, >1=all the ones above this count, -1=all";
      self.ExecTypeNames[326] = "ExecResetForRandom".to_owned();
      self.ExecTypeVarCount[326] = 3;
      self.ExecTypeVarName[326][1] = "Random Library Name";
      self.ExecTypeVarName[326][2] = "Data Library Name";
      self.ExecTypeVarName[326][3] = "Reset Code";
      self.ExecCategory[326] = 22;
      self.ExecDesc[326] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[327] = "ExecMakeCustomRandomPage".to_owned();
      self.ExecTypeVarCount[327] = 3;
      self.ExecTypeVarName[327][1] = "Random Library Name";
      self.ExecTypeVarName[327][2] = "Data Library Name";
      self.ExecTypeVarName[327][3] = "Custom Page ID ";
      self.ExecCategory[327] = 22;
      self.ExecDesc[327] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[328] = "ExecMakeStoryGeneration".to_owned();
      self.ExecTypeVarCount[328] = 1;
      self.ExecTypeVarName[328][1] = "Mode.. 1<=early, 2=late";
      self.ExecTypeVarName[328][2] = "";
      self.ExecTypeVarName[328][3] = " ";
      self.ExecCategory[328] = 22;
      self.ExecDesc[328] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[329] = "ExecHardcodedStory".to_owned();
      self.ExecTypeVarCount[329] = 4;
      self.ExecTypeVarName[329][1] = "tv0".to_owned();
      self.ExecTypeVarName[329][2] = "tv1".to_owned();
      self.ExecTypeVarName[329][3] = "tv2 (tempvar4)";
      self.ExecTypeVarName[329][4] = "tv3 (tempvar5)";
      self.ExecCategory[329] = 22;
      self.ExecDesc[329] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[330] = "ExecSetDssDominant".to_owned();
      self.ExecTypeVarCount[330] = 1;
      self.ExecTypeVarName[330][1] = "Dominant ID (or -1)";
      self.ExecCategory[330] = 22;
      self.ExecDesc[330] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[331] = "ExecHardcodedDecision".to_owned();
      self.ExecTypeVarCount[331] = 0;
      self.ExecCategory[331] = 22;
      self.ExecDesc[331] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[332] = "ExecMakeStartOfRound".to_owned();
      self.ExecTypeVarCount[332] = 0;
      self.ExecCategory[332] = 22;
      self.ExecDesc[332] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[333] = "ExecMakeCloseTurn".to_owned();
      self.ExecTypeVarCount[333] = 0;
      self.ExecCategory[333] = 22;
      self.ExecDesc[333] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[334] = "ExecMakeStartTurn".to_owned();
      self.ExecTypeVarCount[334] = 0;
      self.ExecCategory[334] = 22;
      self.ExecDesc[334] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[342] = "ExecCustomDataImport".to_owned();
      self.ExecTypeVarCount[342] = 2;
      self.ExecCategory[342] = 22;
      self.ExecTypeVarName[342][1] = "Stringlist ID";
      self.ExecTypeVarName[342][2] = "Data Import Mode";
      self.ExecDesc[342] = "1 = Basic Story (Model Alpha).";
      self.ExecTypeNames[344] = "ExecSetWholeLIS".to_owned();
      self.ExecTypeVarCount[344] = 0;
      self.ExecCategory[344] = 22;
      self.ExecDesc[344] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[345] = "ExecMakeLateStartTurn".to_owned();
      self.ExecTypeVarCount[345] = 0;
      self.ExecCategory[345] = 22;
      self.ExecDesc[345] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[346] = "ExecHardcodedSet".to_owned();
      self.ExecTypeVarCount[346] = 3;
      self.ExecCategory[346] = 22;
      self.ExecTypeVarName[346][1] = "type".to_owned();
      self.ExecTypeVarName[346][2] = "custom 1";
      self.ExecTypeVarName[346][3] = "custom 2";
      self.ExecTypeVarName[346][4] = "custom 3";
      self.ExecDesc[346] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[347] = "ExecMakeEarlyCinematics".to_owned();
      self.ExecTypeVarCount[347] = 0;
      self.ExecCategory[347] = 22;
      self.ExecDesc[347] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[348] = "ExecMakeLoadGameEarly".to_owned();
      self.ExecTypeVarCount[348] = 0;
      self.ExecCategory[348] = 22;
      self.ExecDesc[348] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[349] = "ExecSuperImposeMessage".to_owned();
      self.ExecTypeVarCount[349] = 2;
      self.ExecCategory[349] = 22;
      self.ExecTypeVarName[349][1] = "text Heading";
      self.ExecTypeVarName[349][2] = "text Small";
      self.ExecDesc[349] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[352] = "ExecMakeHardcodedSFTypeMods".to_owned();
      self.ExecTypeVarCount[352] = 1;
      self.ExecCategory[352] = 22;
      self.ExecTypeVarName[352][1] = "SFType Slot#";
      self.ExecDesc[352] = "This is a hardcoded random generation function. Please ask for documentation how to use if necc.";
      self.ExecTypeNames[357] = "ExecUdsFileOps".to_owned();
      self.ExecTypeVarCount[357] = 2;
      self.ExecCategory[357] = 22;
      self.ExecTypeVarName[357][1] = "1=Save, 2=Load";
      self.ExecTypeVarName[357][2] = "Filename".to_owned();
      self.ExecDesc[357] = "Saves the UDS keys. Loads the keys in TempUDS";
      self.ExecTypeNames[350] = "ExecHardcodedDC".to_owned();
      self.ExecTypeVarCount[350] = 4;
      self.ExecCategory[350] = 10;
      self.ExecTypeVarName[350][1] = "Data1".to_owned();
      self.ExecTypeVarName[350][2] = "Data2".to_owned();
      self.ExecTypeVarName[350][3] = "Data3".to_owned();
      self.ExecTypeVarName[350][4] = "Data4".to_owned();
      self.ExecDesc[350] = "This is a hardcoded function. It is not meant for general use. And should be used by VR only. (For VR Weather Library => Data1=1,Data2=Lib Name,Data3=Mode(1=editor,2=new round))";
      self.ExecTypeNames[279] = "ExecSetHistoricalSmallGfx".to_owned();
      self.ExecTypeVarCount[279] = 2;
      self.ExecTypeVarName[279][1] = "Historical Unit ID";
      self.ExecTypeVarName[279][2] = "New Small Gfx slot#";
      self.ExecCategory[279] = 13;
      self.ExecTypeNames[291] = "ExecSetHistoricalColor".to_owned();
      self.ExecTypeVarCount[291] = 4;
      self.ExecTypeVarName[291][1] = "Historical Unit ID";
      self.ExecTypeVarName[291][2] = "Red modifier";
      self.ExecTypeVarName[291][3] = "Green modifier";
      self.ExecTypeVarName[291][4] = "Blue modifier";
      self.ExecCategory[291] = 13;
      self.ExecDesc[291] = "Remember you cannot set the using: Color Historical unit colors, only the modifier to the regime or people of: Color the counter. For example -50 or +100. ";
      self.ExecTypeNames[298] = "ExecCopyStringlist".to_owned();
      self.ExecTypeVarCount[298] = 4;
      self.ExecTypeVarName[298][1] = "From ID";
      self.ExecTypeVarName[298][2] = "Too ID";
      self.ExecTypeVarName[298][3] = "Number of columns";
      self.ExecTypeVarName[298][4] = "Mode 1=clear&copy, 2=add. columns can be set to 99 or something very high to include all";
      self.ExecCategory[298] = 16;
      self.ExecTypeNames[306] = "ExecChangeLocationHQ".to_owned();
      self.ExecTypeVarCount[306] = 2;
      self.ExecTypeVarName[306][1] = "Location Slot#";
      self.ExecTypeVarName[306][2] = "HQ Unit Slot#";
      self.ExecCategory[306] = 4;
      self.ExecTypeNames[307] = "ExecSetHistoricalRegime".to_owned();
      self.ExecTypeVarCount[307] = 2;
      self.ExecTypeVarName[307][1] = "Historical Unit ID";
      self.ExecTypeVarName[307][2] = "Regime slot#";
      self.ExecCategory[307] = 13;
      self.ExecDesc[307] = "Sets the historical to the correct regime.";
      self.ExecTypeNames[310] = "ExecDoubleKey".to_owned();
      self.ExecTypeVarCount[310] = 4;
      self.ExecTypeVarName[310][1] = "StringListID#";
      self.ExecTypeVarName[310][2] = "Col0 value";
      self.ExecTypeVarName[310][3] = "Key Name";
      self.ExecTypeVarName[310][4] = "New Value";
      self.ExecCategory[310] = 16;
      self.ExecDesc[310] = "Presumes a stringlist with in column 1 the key names and in column 2 the text values. ";
      self.ExecTypeNames[311] = "ExecDoubleKeyAppend".to_owned();
      self.ExecTypeVarCount[311] = 4;
      self.ExecTypeVarName[311][1] = "StringListID#";
      self.ExecTypeVarName[311][2] = "Col0 value";
      self.ExecTypeVarName[311][3] = "Key Name";
      self.ExecTypeVarName[311][4] = "Value to append";
      self.ExecCategory[311] = 16;
      self.ExecDesc[311] = "Presumes a stringlist with in column 1 the key names and in column 2 the text values. ";
      self.ExecTypeNames[336] = "ExecSFTypeFavorite".to_owned();
      self.ExecTypeVarCount[336] = 4;
      self.ExecTypeVarName[336][1] = "SFType".to_owned();
      self.ExecTypeVarName[336][2] = "Targetgroup# (-1=all)";
      self.ExecTypeVarName[336][3] = "Favorite score for regular attack";
      self.ExecTypeVarName[336][4] = "Favorite score for art attack";
      self.ExecTypeString[336] = 0;
      self.ExecCategory[336] = 9;
      self.ExecTypeNames[337] = "ExecSFTypeArtPower".to_owned();
      self.ExecTypeVarCount[337] = 3;
      self.ExecTypeVarName[337][1] = "SFType".to_owned();
      self.ExecTypeVarName[337][2] = "Targetgroup# (-1=all)";
      self.ExecTypeVarName[337][3] = "Art Power";
      self.ExecTypeString[337] = 0;
      self.ExecCategory[337] = 9;
      self.ExecTypeNames[338] = "ExecSFTypeStat".to_owned();
      self.ExecTypeVarCount[338] = 3;
      self.ExecTypeVarName[338][1] = "SFType".to_owned();
      self.ExecTypeVarName[338][2] = "Stat Number";
      self.ExecTypeVarName[338][3] = "New Value";
      self.ExecTypeString[338] = 0;
      self.ExecCategory[338] = 9;
      self.ExecDesc[338] = "Stat Numbers: 1=weight, 2=carrycap, 3=canDoParadrop(0/1), 4=EP, 5=BlowBridgPts, 6=BasicSupNeed, 7=SupCarry, 8=ReconPts, 9=HidePts, 10=AntiSupplyPts, 11=AntiSupplyRange, 12=AnitSupplySea, 13=InitiativeAtt, 14=InitiativeDef, 15=Attacks, 16=StackPts, 17=RearArea(0/1), 18=ArtRange, 19=Fav.Target tries, 20=AA-range, 21=HitKill%, 22=HitRetreat%, 23=ArtilleryModSfTypeNr, 24=maxAttacked, 25=moveRedux";
      self.ExecTypeNames[339] = "ExecSFTypeLandscapeCombatMod".to_owned();
      self.ExecTypeVarCount[339] = 4;
      self.ExecTypeVarName[339][1] = "SFType#";
      self.ExecTypeVarName[339][2] = "LandscapeType# (-1=all)";
      self.ExecTypeVarName[339][3] = "Off Mod in percentage points. 100=noMod, 50=half, 200=double";
      self.ExecTypeVarName[339][4] = "Def Mod in percentage points. 100=noMod, 50=half, 200=double";
      self.ExecTypeString[339] = 0;
      self.ExecCategory[339] = 9;
      self.ExecTypeNames[340] = "ExecSFTypeSetName".to_owned();
      self.ExecTypeVarCount[340] = 3;
      self.ExecTypeVarName[340][1] = "SFType#";
      self.ExecTypeVarName[340][2] = "Name (if length>1) ";
      self.ExecTypeVarName[340][3] = "ModelName (if length>1)";
      self.ExecTypeString[340] = 0;
      self.ExecCategory[340] = 9;
      self.ExecDesc[340] = "Warning: Highly experimental if used together with model creation. Name changes the name of the current SFType. ModelName changes the name of upgrades or improvements from this sfType. The hardcoded algorithms should still remember the version and mark numbers no matter if you change the modelName.";
      self.ExecTypeNames[341] = "ExecSFTypeReinforcementType".to_owned();
      self.ExecTypeVarCount[341] = 2;
      self.ExecTypeVarName[341][1] = "SFType".to_owned();
      self.ExecTypeVarName[341][2] = "ReinforcementType".to_owned();
      self.ExecTypeString[341] = 0;
      self.ExecCategory[341] = 9;
      self.ExecTypeNames[351] = "ExecLocStats".to_owned();
      self.ExecTypeVarCount[351] = 4;
      self.ExecTypeVarName[351][1] = "Loc#";
      self.ExecTypeVarName[351][2] = "Stat #";
      self.ExecTypeVarName[351][3] = "Mode 0/1=%, 2=absolute new value";
      self.ExecTypeVarName[351][4] = "Value".to_owned();
      self.ExecCategory[351] = 4;
      self.ExecDesc[351] = "Stats: 1=supply points, 2=fuel points, 3=mode var, 4=fixed var";
      self.ExecTypeNames[353] = "ExecChangeUnitStats".to_owned();
      self.ExecTypeVarCount[353] = 3;
      self.ExecTypeVarName[353][1] = "Unit #";
      self.ExecTypeVarName[353][2] = "Stat #";
      self.ExecTypeVarName[353][3] = "Value".to_owned();
      self.ExecCategory[353] = 4;
      self.ExecDesc[353] = "Stat # 1=Readiness, 2=Entrenchment, 3=Morale, 4=Experience";
      self.ExecTypeNames[354] = "ExecChangeLocationType2".to_owned();
      self.ExecTypeVarCount[354] = 4;
      self.ExecTypeVarName[354][1] = "X".to_owned();
      self.ExecTypeVarName[354][2] = "Y".to_owned();
      self.ExecTypeVarName[354][3] = "Loctype (-1=remove any location)";
      self.ExecTypeVarName[354][4] = "People #";
      self.ExecTypeString[354] = 2;
      self.ExecCategory[354] = 4;
      self.ExecDesc[354] = "Only use this for Supply Base/Source location types.";
      self.ExecTypeNames[355] = "ExecGiveFuel".to_owned();
      self.ExecTypeVarCount[355] = 4;
      self.ExecTypeVarName[355][1] = "X ";
      self.ExecTypeVarName[355][2] = "Y".to_owned();
      self.ExecTypeVarName[355][3] = "Fuel Points on x,y";
      self.ExecTypeVarName[355][4] = "Overrule recipient regime with regime #";
      self.ExecDesc[355] = "Only works for Supply Sources active (471). Overule recipient is inactive if put to 0. If no overrule the supplies are given to closest unit owned by regime that owns hex. But >0 is overrule 1=reg 0, 2=reg1 !!!! This +1 shift is inhere to keep backwards compatible with AdvTactics scenarios.";
      self.ExecCategory[355] = 4;
      self.ExecTypeNames[356] = "ExecMoveTypeTransportType".to_owned();
      self.ExecTypeVarCount[356] = 2;
      self.ExecTypeVarName[356][1] = "MoveType#";
      self.ExecTypeVarName[356][2] = "Transport Type (0=none, 1=Transporter, 2=Attachable)";
      self.ExecCategory[356] = 14;
      self.ExecTypeNames[358] = "ExecSetRegimeSetting".to_owned();
      self.ExecTypeVarCount[358] = 3;
      self.ExecTypeVarName[358][1] = "Regime Slot#";
      self.ExecTypeVarName[358][2] = "Setting# 1=alternate card gfx";
      self.ExecTypeVarName[358][3] = "Value".to_owned();
      self.ExecCategory[358] = 14;
      self.ExecTypeNames[359] = "ExecModifyUnitFuel".to_owned();
      self.ExecTypeVarCount[359] = 4;
      self.ExecTypeVarName[359][1] = "Regime (-1=none)";
      self.ExecTypeVarName[359][2] = "Specific Unit (-1=none)";
      self.ExecTypeVarName[359][3] = "Modifier % (100=keep same, 50=half, 200=double)";
      self.ExecTypeVarName[359][4] = "Modifier absolute (0=nill, -x , +x )";
      self.ExecCategory[359] = 8;
      self.ExecDesc[359] = "Increases or decreases Fuel of the unit. cannot go over max fuel reserves";
      self.ExecTypeNames[360] = "ExecAddUnitTextLog".to_owned();
      self.ExecTypeVarCount[360] = 2;
      self.ExecTypeVarName[360][1] = "Unit Slot#";
      self.ExecTypeVarName[360][2] = "Text".to_owned();
      self.ExecCategory[360] = 14;
      self.ExecTypeCount = 360;
    }

    pub fn SetDefaultRules(bool namesonly = false)
    {
      let mut ruleCounter: i32 =  self.RuleCounter;
      for (let mut index: i32 =  0; index <= ruleCounter; index += 1)
      {
        if (!namesonly)
          self.RuleVar[index] = 0.0f;
        self.RuleString[index] = "";
        if (!namesonly)
          self.RuleGroup[index] = 0;
      }
      self.RuleString[2] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[2] = 6f;
      self.RuleGroup[2] = 0;
      self.RuleString[34] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[34] = 0.25f;
      self.RuleGroup[34] = 0;
      self.RuleString[39] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[39] = 5f;
      self.RuleGroup[39] = 0;
      self.RuleString[45] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[45] = 0.25f;
      self.RuleGroup[45] = 0;
      self.RuleString[62] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[62] = 50f;
      self.RuleGroup[62] = 0;
      self.RuleString[71] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[71] = 0.5f;
      self.RuleGroup[71] = 0;
      self.RuleString[72] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[72] = 1f;
      self.RuleGroup[72] = 0;
      self.RuleString[76] = "";
      if (!namesonly)
        self.RuleVar[76] = 100f;
      self.RuleGroup[76] = 0;
      self.RuleString[6] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[6] = 0.2f;
      self.RuleGroup[6] = 0;
      self.RuleString[43] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[43] = 1f;
      self.RuleGroup[43] = 0;
      self.RuleString[0] = "Land Transfer Movement Type Nr used";
      if (!namesonly)
        self.RuleVar[0] = 2f;
      self.RuleGroup[0] = 1;
      self.RuleString[1] = "Sea Tranfer Movement Type Nr used";
      if (!namesonly)
        self.RuleVar[1] = 8f;
      self.RuleGroup[1] = 1;
      self.RuleString[2] = "Rail Transfer Movement Type Nr used";
      if (!namesonly)
        self.RuleVar[2] = 2f;
      self.RuleGroup[2] = 1;
      self.RuleString[3] = "Supply Action PoRange: i32";
      if (!namesonly)
        self.RuleVar[3] = 250f;
      self.RuleGroup[3] = 1;
      self.RuleString[38] = "Use which landscapetype for unit movementtype logo prediction";
      if (!namesonly)
        self.RuleVar[38] = 0.0f;
      self.RuleGroup[38] = 1;
      self.RuleString[78] = "Transfer Action PoRange: i32";
      if (!namesonly)
        self.RuleVar[78] = 1000f;
      self.RuleGroup[78] = 1;
      self.RuleString[99] = "Supply Movement Type Nr used";
      if (!namesonly)
        self.RuleVar[99] = 9f;
      self.RuleGroup[99] = 1;
      self.RuleString[148] = "Show Big VPs on Mini & Strategic Map (=>x vp)";
      if (!namesonly)
        self.RuleVar[148] = 5f;
      self.RuleGroup[148] = 14;
      self.RuleString[149] = "Show small VPs on Mini & Strategic Map (=>x vp)";
      if (!namesonly)
        self.RuleVar[149] = 1f;
      self.RuleGroup[149] = 14;
      self.RuleString[302] = "Highlighted hex white: Color (0) - black(1)";
      if (!namesonly)
        self.RuleVar[302] = 0.0f;
      self.RuleGroup[302] = 14;
      self.RuleString[306] = "Highlighted hex alpha: Color value (default 0.3 put higher for more color)";
      if (!namesonly)
        self.RuleVar[306] = 0.3f;
      self.RuleGroup[306] = 14;
      self.RuleString[857] = "Airsupply Supply MovementType";
      if (!namesonly)
        self.RuleVar[857] = 9f;
      self.RuleGroup[857] = 1;
      self.RuleString[858] = "Airsupply Supply Range in AP";
      if (!namesonly)
        self.RuleVar[858] = 9f;
      self.RuleGroup[858] = 1;
      self.RuleString[335] = "Which regimevar #(x to x+15) gives prediction of supply points (0=disabl)";
      if (!namesonly)
        self.RuleVar[335] = 0.0f;
      self.RuleGroup[335] = 1;
      self.RuleGroup2[335] = 19;
      self.RuleString[350] = "Use easy Strategic Transfer Mode? (0=no, 1=yes)";
      if (!namesonly)
        self.RuleVar[350] = 0.0f;
      self.RuleGroup[350] = 1;
      self.RuleString[351] = "Startupcost for Strategic Transfer in AP";
      if (!namesonly)
        self.RuleVar[351] = 0.0f;
      self.RuleGroup[351] = 1;
      self.RuleString[851] = "LandCap use costs Regimevar #";
      if (!namesonly)
        self.RuleVar[851] = 0.0f;
      self.RuleGroup[851] = 1;
      self.RuleString[852] = "LandCap 1000 CAP pts use costs Regimevar Qty";
      if (!namesonly)
        self.RuleVar[852] = 0.0f;
      self.RuleGroup[852] = 1;
      self.RuleString[853] = "NavyCap use costs Regimevar #";
      if (!namesonly)
        self.RuleVar[853] = 0.0f;
      self.RuleGroup[853] = 1;
      self.RuleString[854] = "NavyCap 1000 CAP pts use costs Regimevar Qty";
      if (!namesonly)
        self.RuleVar[854] = 0.0f;
      self.RuleGroup[854] = 1;
      self.RuleString[855] = "RailCap use costs Regimevar #";
      if (!namesonly)
        self.RuleVar[855] = 0.0f;
      self.RuleGroup[855] = 1;
      self.RuleString[856] = "RailCap 1000 CAP pts use costs Regimevar Qty";
      if (!namesonly)
        self.RuleVar[856] = 0.0f;
      self.RuleGroup[856] = 1;
      self.RuleString[886] = "No fuel cost for own power transfer. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[886] = 0.0f;
      self.RuleGroup[886] = 1;
      self.RuleString[8] = "Minimum Recon Pts needed to see anything / Free recon points for owned hex";
      if (!namesonly)
        self.RuleVar[8] = 2f;
      self.RuleGroup[8] = 2;
      self.RuleString[9] = "Minimum Zoc Pts needed to capture an enemy hex";
      if (!namesonly)
        self.RuleVar[9] = 4f;
      self.RuleGroup[9] = 2;
      self.RuleString[10] = "Minimum Zoc Pts needed to capture a neutral hex";
      if (!namesonly)
        self.RuleVar[10] = 1f;
      self.RuleGroup[10] = 2;
      self.RuleString[11] = "Recon Dist 1 Modifier";
      if (!namesonly)
        self.RuleVar[11] = 1f;
      self.RuleGroup[11] = 2;
      self.RuleString[12] = "Recon Dist 2 Modifier";
      if (!namesonly)
        self.RuleVar[12] = 0.25f;
      self.RuleGroup[12] = 2;
      self.RuleString[13] = "Recon Dist 3 Modifier";
      if (!namesonly)
        self.RuleVar[13] = 0.05f;
      self.RuleGroup[13] = 2;
      self.RuleString[14] = "Recon Dist 4 Modifier";
      if (!namesonly)
        self.RuleVar[14] = 0.01f;
      self.RuleGroup[14] = 2;
      self.RuleString[21] = "Zoc Dist 1 Modifier";
      if (!namesonly)
        self.RuleVar[21] = 1f;
      self.RuleGroup[21] = 2;
      self.RuleString[22] = "Zoc Dist 2 Modifier";
      if (!namesonly)
        self.RuleVar[22] = 0.0f;
      self.RuleGroup[22] = 2;
      self.RuleString[23] = "Zoc Dist 3 Modifier";
      if (!namesonly)
        self.RuleVar[23] = 0.0f;
      self.RuleGroup[23] = 2;
      self.RuleString[24] = "Zoc Dist 4 Modifier";
      if (!namesonly)
        self.RuleVar[24] = 0.0f;
      self.RuleGroup[24] = 2;
      self.RuleString[40] = "X times more ZOC points needed than enemy ZOC pts to capture a hex";
      if (!namesonly)
        self.RuleVar[40] = 4f;
      self.RuleGroup[40] = 2;
      self.RuleString[55] = "Minimal Recon Pts needed for partial Info on Unit";
      if (!namesonly)
        self.RuleVar[55] = 30f;
      self.RuleGroup[55] = 2;
      self.RuleString[56] = "Recon Pts needed for full Info on Unit";
      if (!namesonly)
        self.RuleVar[56] = 250f;
      self.RuleGroup[56] = 2;
      self.RuleString[79] = "Auto Conquer neutral hex start of turn (1=yes)";
      if (!namesonly)
        self.RuleVar[79] = 1f;
      self.RuleGroup[79] = 2;
      self.RuleString[307] = "Minimum Power Points needed to capture enemy hex";
      if (!namesonly)
        self.RuleVar[307] = 10f;
      self.RuleGroup[307] = 2;
      self.RuleString[323] = "AP Penalty for entering enemy ZOC. Double value for crossing river into enemy ZOC.";
      if (!namesonly)
        self.RuleVar[323] = 1f;
      self.RuleGroup[323] = 2;
      self.RuleString[945] = "Ferry effectivity only for movement type #X";
      if (!namesonly)
        self.RuleVar[945] = 0.0f;
      self.RuleGroup[945] = 2;
      self.RuleString[840] = "Keep original owner for correct ally ownership returning with reconquest. 0=no 1=yes.";
      if (!namesonly)
        self.RuleVar[840] = 1f;
      self.RuleGroup[840] = 2;
      self.RuleString[843] = "Call event# after move,production or battle. 0=no. 1=event #.";
      if (!namesonly)
        self.RuleVar[843] = 1f;
      self.RuleGroup[843] = 2;
      self.RuleString[901] = "No ZOC capture of ANY locations. 0=off. 1=on. 2=also no roads capture";
      if (!namesonly)
        self.RuleVar[901] = 1f;
      self.RuleGroup[901] = 2;
      self.RuleString[902] = "Disable that engineers can repair locations. 0=normal. 1=disabled.";
      if (!namesonly)
        self.RuleVar[902] = 1f;
      self.RuleGroup[902] = 2;
      self.RuleString[874] = "Hide labels + loc.names on shrowded hexes with peek-info? 0=no. 1=yes.";
      if (!namesonly)
        self.RuleVar[874] = 1f;
      self.RuleGroup[874] = 2;
      self.RuleString[33] = "Supply Weight";
      if (!namesonly)
        self.RuleVar[33] = 0.1f;
      self.RuleGroup[33] = 3;
      self.RuleString[41] = "Excess Supply at HQ uses MovementType Nr";
      if (!namesonly)
        self.RuleVar[41] = 0.0f;
      self.RuleGroup[41] = 3;
      self.RuleString[51] = "only 75% supply at Action Podistance: i32";
      if (!namesonly)
        self.RuleVar[51] = 100f;
      self.RuleGroup[51] = 3;
      self.RuleString[52] = "only 50% supply at Action Podistance: i32";
      if (!namesonly)
        self.RuleVar[52] = 150f;
      self.RuleGroup[52] = 3;
      self.RuleString[53] = "only 25% supply at Action Podistance: i32";
      if (!namesonly)
        self.RuleVar[53] = 200f;
      self.RuleGroup[53] = 3;
      self.RuleString[77] = "1 Supply PoCosts: i32 X Production Points (for calculation of upgrade cost)";
      if (!namesonly)
        self.RuleVar[77] = 3f;
      self.RuleGroup[77] = 3;
      self.RuleString[82] = "ActionPopenalty: i32 for  from sea to land without port";
      if (!namesonly)
        self.RuleVar[82] = 100f;
      self.RuleGroup[82] = 3;
      self.RuleString[303] = "Anti Supply PoMultiplier: i32 if on sea hex next to enemy port.";
      if (!namesonly)
        self.RuleVar[302] = 20f;
      self.RuleGroup[303] = 3;
      self.RuleString[309] = "Enable transfer losses for Anti Supply on land transfers too (0=no, 1=yes)";
      if (!namesonly)
        self.RuleVar[309] = 0.0f;
      self.RuleGroup[309] = 3;
      self.RuleString[324] = "Enable Hardcore Supply rules, only 250ap from High Hq (1=yes, 0=no)";
      if (!namesonly)
        self.RuleVar[324] = 0.0f;
      self.RuleGroup[324] = 3;
      self.RuleString[336] = "Remove excess supply HQs.(1=yes after give sup, 2=yes before give sup, >100 = yes after but remove only sup >x at highC, 0=no)";
      if (!namesonly)
        self.RuleVar[336] = 0.0f;
      self.RuleGroup[336] = 3;
      self.RuleString[801] = "Enforce Models when attempting to transfer (1=yes, 0=no)";
      if (!namesonly)
        self.RuleVar[801] = 0.0f;
      self.RuleGroup[801] = 3;
      self.RuleString[883] = "Maximum auto-reinf percentage (at 100% supply level)";
      if (!namesonly)
        self.RuleVar[883] = 0.0f;
      self.RuleGroup[883] = 3;
      self.RuleString[887] = "Enable High HQ Supply/AutoReinf EASY rules which means no HARDCORE chain of HQs just high and the others (1=yes, 0=no)";
      if (!namesonly)
        self.RuleVar[887] = 0.0f;
      self.RuleGroup[887] = 3;
      self.RuleString[7] = "Bridge Structural Pts";
      if (!namesonly)
        self.RuleVar[7] = 500f;
      self.RuleGroup[7] = 4;
      self.RuleString[32] = "RoadType your engineers can build. (-1 = no bridge/no road building)";
      if (!namesonly)
        self.RuleVar[32] = 0.0f;
      self.RuleGroup[32] = 4;
      self.RuleString[4] = "Enemy Territory entry extra AP cost";
      if (!namesonly)
        self.RuleVar[4] = 10f;
      self.RuleGroup[4] = 4;
      self.RuleString[44] = "Minimum Action Points for navy left if out of supply.";
      if (!namesonly)
        self.RuleVar[44] = 30f;
      self.RuleGroup[44] = 4;
      self.RuleString[305] = "Bridges cannot be destroyed by combat (see 505 for engineers). 0=no , 1=yes";
      if (!namesonly)
        self.RuleVar[305] = 0.0f;
      self.RuleGroup[305] = 4;
      self.RuleString[308] = "Dont show broken bridges sprites. 0=no show them , 1=yes hide broken bridge";
      if (!namesonly)
        self.RuleVar[308] = 0.0f;
      self.RuleGroup[308] = 4;
      self.RuleString[320] = "Only allow bridge building if road is already on both sides. 0=no , 1=yes";
      if (!namesonly)
        self.RuleVar[320] = 0.0f;
      self.RuleGroup[320] = 4;
      self.RuleString[452] = "Road over river acts as bridge (1=yes, 0=no)";
      if (!namesonly)
        self.RuleVar[452] = 0.0f;
      self.RuleGroup[452] = 14;
      self.RuleString[321] = "Lose EP on movement. 0=no , 1=yes";
      if (!namesonly)
        self.RuleVar[321] = 0.0f;
      self.RuleGroup[321] = 4;
      self.RuleString[322] = "Disallow supply transfer & setting reserve supply prefs for HQ. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[322] = 0.0f;
      self.RuleGroup[322] = 4;
      self.RuleString[814] = "Allow Free Movement (0=no/1=yes) (can be split in zones by setting groupnames 740,741)";
      if (!namesonly)
        self.RuleVar[814] = 0.0f;
      self.RuleGroup[814] = 4;
      self.RuleString[48] = "Readiness modifier for assigning new HQ";
      if (!namesonly)
        self.RuleVar[48] = 0.5f;
      self.RuleGroup[48] = 5;
      self.RuleString[49] = "Readiness modifier for transfer to same HQ unit";
      if (!namesonly)
        self.RuleVar[49] = 0.75f;
      self.RuleGroup[49] = 5;
      self.RuleString[50] = "Readiness modifier for transfer outside HQ";
      if (!namesonly)
        self.RuleVar[50] = 0.25f;
      self.RuleGroup[50] = 5;
      self.RuleString[59] = "Max Readiness increase compared to before autodrop";
      if (!namesonly)
        self.RuleVar[59] = 30f;
      self.RuleGroup[59] = 5;
      self.RuleString[60] = "Minimum readiness allowed";
      if (!namesonly)
        self.RuleVar[60] = 10f;
      self.RuleGroup[60] = 5;
      self.RuleString[61] = "Autodrop Readiness  in % , before supply";
      if (!namesonly)
        self.RuleVar[61] = 40f;
      self.RuleGroup[61] = 5;
      self.RuleString[131] = "Readiness modifier for strategic transfer";
      if (!namesonly)
        self.RuleVar[131] = 0.5f;
      self.RuleGroup[131] = 5;
      self.RuleString[36] = "staff Xp loss with transfers/ getting new troops";
      if (!namesonly)
        self.RuleVar[36] = 0.25f;
      self.RuleGroup[36] = 6;
      self.RuleString[42] = "Max turns Engineer Points can be saved up";
      if (!namesonly)
        self.RuleVar[42] = 5f;
      self.RuleGroup[42] = 6;
      self.RuleString[63] = "Free XP up to xp polimit: i32";
      if (!namesonly)
        self.RuleVar[63] = 40f;
      self.RuleGroup[63] = 6;
      self.RuleString[64] = "Max Free XP a turn";
      if (!namesonly)
        self.RuleVar[64] = 10f;
      self.RuleGroup[64] = 6;
      self.RuleString[65] = "Max normal Morale recovery per turn of basemorale value";
      if (!namesonly)
        self.RuleVar[65] = 0.25f;
      self.RuleGroup[65] = 6;
      self.RuleString[75] = "XP modifier for staff of combat individual xp at 100% staff level";
      if (!namesonly)
        self.RuleVar[75] = 1f;
      self.RuleGroup[75] = 6;
      self.RuleString[80] = "Xp growth modifier (1-4). 1=50% at 50xp/40% at 60xp. 2=25% at 50xp/16% at 60xp, etc..";
      if (!namesonly)
        self.RuleVar[80] = 1.5f;
      self.RuleGroup[80] = 6;
      self.RuleString[81] = "Max XP allowed ";
      if (!namesonly)
        self.RuleVar[81] = 100f;
      self.RuleGroup[81] = 6;
      self.RuleString[926] = "XP Modifier for disbanding. 0=all. 0.5=half. 1=none.";
      if (!namesonly)
        self.RuleVar[926] = 0.0f;
      self.RuleGroup[926] = 6;
      self.RuleString[46] = "PP Cost for formation";
      if (!namesonly)
        self.RuleVar[46] = 1f;
      self.RuleGroup[46] = 7;
      self.RuleString[47] = "PP Cost for HQ";
      if (!namesonly)
        self.RuleVar[47] = 3f;
      self.RuleGroup[47] = 7;
      self.RuleString[894] = "High Command HQs (historical type) cannot move. And will be destroyed on retreat. 0=no rule 1=rule enabled. ";
      if (!namesonly)
        self.RuleVar[894] = 3f;
      self.RuleGroup[894] = 7;
      self.RuleString[904] = "Base PP Cost for officerpool switch. ";
      if (!namesonly)
        self.RuleVar[904] = 3f;
      self.RuleGroup[904] = 7;
      self.RuleString[896] = "High Command HQs (historical) cannot get officer changed by officerpool. 0=no dont use rule, 1=yes cannot get changed ";
      if (!namesonly)
        self.RuleVar[896] = 0.0f;
      self.RuleGroup[896] = 7;
      self.RuleString[897] = "HQs lose action cards when unit is assigned to them. 0=yes. 1=no cancel this losing cards rule. ";
      if (!namesonly)
        self.RuleVar[897] = 0.0f;
      self.RuleGroup[897] = 7;
      self.RuleString[944] = "HQs cannot play action cards on a unit whose HQ has been changed in this turn. 0=rule disabled. 1=enable rule. ";
      if (!namesonly)
        self.RuleVar[944] = 0.0f;
      self.RuleGroup[944] = 7;
      self.RuleString[907] = "hisvar type X to diminish when unit is assigned or to clear when moved out from officerpool. (0=dont use) hisvar Type X= ";
      if (!namesonly)
        self.RuleVar[907] = 0.0f;
      self.RuleGroup[907] = 7;
      self.RuleString[927] = "Only Corps level HQs (type=5) give combat and morale bonus. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[927] = 0.0f;
      self.RuleGroup[927] = 7;
      self.RuleString[314] = "DISABLEd".to_owned();
      if (!namesonly)
        self.RuleVar[314] = 0.0f;
      self.RuleGroup[314] = 0;
      self.RuleString[73] = "Distance (in hex) at which hq pow mod stays 100%";
      if (!namesonly)
        self.RuleVar[73] = 2f;
      self.RuleGroup[73] = 8;
      self.RuleString[74] = "-X% for hq pow for every distance step beyond 100% distance ";
      if (!namesonly)
        self.RuleVar[74] = 20f;
      self.RuleGroup[74] = 8;
      self.RuleString[140] = "Basic staff 100%level and hqpow combat modifier in + 0.x ";
      if (!namesonly)
        self.RuleVar[140] = 0.5f;
      self.RuleGroup[140] = 8;
      self.RuleString[141] = "Basic staff at 100%level and hqpow morale modifier in 0.x ";
      if (!namesonly)
        self.RuleVar[141] = 0.25f;
      self.RuleGroup[141] = 8;
      self.RuleString[304] = "Max number of HQs in a chain of command. 0=no limit. >0=specified limit.";
      if (!namesonly)
        self.RuleVar[304] = 3f;
      self.RuleGroup[304] = 8;
      self.RuleString[5] = "Bridge attack river penalty modifier";
      if (!namesonly)
        self.RuleVar[5] = 0.5f;
      self.RuleGroup[5] = 9;
      self.RuleString[30] = "Hex minimum StackMax";
      if (!namesonly)
        self.RuleVar[30] = 100f;
      self.RuleGroup[30] = 9;
      self.RuleString[31] = "AttackMax Per Hexside in FrontagePts";
      if (!namesonly)
        self.RuleVar[31] = 50f;
      self.RuleGroup[31] = 9;
      self.RuleString[35] = "PowerPts Destroyed gives 1 officer XP";
      if (!namesonly)
        self.RuleVar[35] = 5f;
      self.RuleGroup[35] = 9;
      self.RuleString[37] = "Equal to this morale = capitulation chance rnd()>rdn/301 IF also Rdn < rulevar(301)";
      if (!namesonly)
        self.RuleVar[37] = 10f;
      self.RuleGroup[37] = 9;
      self.RuleString[70] = "Panic chance if more casualties than morale can cope";
      if (!namesonly)
        self.RuleVar[70] = 0.25f;
      self.RuleGroup[70] = 9;
      self.RuleString[100] = "Allow Air Move/Bombing/Recon/Paradrop missions into shroud? (1=yes)";
      if (!namesonly)
        self.RuleVar[100] = 1f;
      self.RuleGroup[100] = 9;
      self.RuleString[101] = "Counter Attack modifier for attacker on land";
      if (!namesonly)
        self.RuleVar[101] = 0.5f;
      self.RuleGroup[101] = 9;
      self.RuleString[102] = "Counter Attack modifier for defender on land";
      if (!namesonly)
        self.RuleVar[102] = 1f;
      self.RuleGroup[102] = 9;
      self.RuleString[103] = "Counter Attack modifier for attacker on sea/air";
      if (!namesonly)
        self.RuleVar[103] = 1f;
      self.RuleGroup[103] = 9;
      self.RuleString[104] = "Counter Attack modifier for defender on sea/air";
      if (!namesonly)
        self.RuleVar[104] = 1f;
      self.RuleGroup[104] = 9;
      self.RuleString[105] = "Flak assistance outside own hex modifier";
      if (!namesonly)
        self.RuleVar[105] = 0.5f;
      self.RuleGroup[105] = 9;
      self.RuleString[106] = "Attacker max readiness penalty if attacking";
      if (!namesonly)
        self.RuleVar[106] = 1f;
      self.RuleGroup[106] = 9;
      self.RuleString[107] = "Defender max readiness penalty if attacking";
      if (!namesonly)
        self.RuleVar[107] = 0.5f;
      self.RuleGroup[107] = 9;
      self.RuleString[108] = "Landsurprise defender modifier";
      if (!namesonly)
        self.RuleVar[108] = 2f;
      self.RuleGroup[108] = 9;
      self.RuleString[109] = "Paradrop defender modifier";
      if (!namesonly)
        self.RuleVar[109] = 3f;
      self.RuleGroup[109] = 9;
      self.RuleString[110] = "Amphibious asault defender modifier";
      if (!namesonly)
        self.RuleVar[110] = 3f;
      self.RuleGroup[110] = 9;
      self.RuleString[111] = "Rebel Advantage modifier";
      if (!namesonly)
        self.RuleVar[111] = 2f;
      self.RuleGroup[111] = 9;
      self.RuleString[112] = "Modifier if shooting at orderly retreating attacker";
      if (!namesonly)
        self.RuleVar[112] = 0.25f;
      self.RuleGroup[112] = 9;
      self.RuleString[113] = "Modifier if shooting at orderly retreating defender";
      if (!namesonly)
        self.RuleVar[113] = 1f;
      self.RuleGroup[113] = 9;
      self.RuleString[114] = "Modifier if shooting at panicking attacker";
      if (!namesonly)
        self.RuleVar[114] = 1f;
      self.RuleGroup[114] = 9;
      self.RuleString[115] = "Modifier if shooting at panicking defender";
      if (!namesonly)
        self.RuleVar[115] = 2f;
      self.RuleGroup[115] = 9;
      self.RuleString[116] = "Max readiness penalty on hitpoints for defending individual";
      if (!namesonly)
        self.RuleVar[116] = 0.25f;
      self.RuleGroup[116] = 9;
      self.RuleString[117] = "Minimal xp modifier due to powerpodifference: i32";
      if (!namesonly)
        self.RuleVar[117] = 0.1f;
      self.RuleGroup[117] = 9;
      self.RuleString[118] = "Maximal xp modifier due to powerpodifference: i32";
      if (!namesonly)
        self.RuleVar[118] = 3f;
      self.RuleGroup[118] = 9;
      self.RuleString[119] = "Score Kill gives XP pts";
      if (!namesonly)
        self.RuleVar[119] = 30f;
      self.RuleGroup[119] = 9;
      self.RuleString[120] = "Score Retreat gives XP pts";
      if (!namesonly)
        self.RuleVar[120] = 10f;
      self.RuleGroup[120] = 9;
      self.RuleString[121] = "Score Pinned gives XP pts";
      if (!namesonly)
        self.RuleVar[121] = 0.0f;
      self.RuleGroup[121] = 9;
      self.RuleString[122] = "Score Retreat Readiness loss %";
      if (!namesonly)
        self.RuleVar[122] = 50f;
      self.RuleGroup[122] = 9;
      self.RuleString[123] = "Score Pinned Readiness loss %";
      if (!namesonly)
        self.RuleVar[123] = 50f;
      self.RuleGroup[123] = 9;
      self.RuleString[124] = "Score Retreat Morale loss %";
      if (!namesonly)
        self.RuleVar[124] = 25f;
      self.RuleGroup[124] = 9;
      self.RuleString[125] = "Score Pinned Morale loss %";
      if (!namesonly)
        self.RuleVar[125] = 10f;
      self.RuleGroup[125] = 9;
      self.RuleString[126] = "Score Retreat Entrench loss %";
      if (!namesonly)
        self.RuleVar[126] = 100f;
      self.RuleGroup[126] = 9;
      self.RuleString[ sbyte.MaxValue] = "Score Pinned Entrench loss %";
      if (!namesonly)
        self.RuleVar[ sbyte.MaxValue] = 50f;
      self.RuleGroup[ sbyte.MaxValue] = 9;
      self.RuleString[996] = "Revised Panic Rules (always possible) (0=no, 1=yes) ";
      if (!namesonly)
        self.RuleVar[996] = 0.0f;
      self.RuleGroup[996] = 19;
      self.RuleString[128] = "If Air and not a bombing mission anti-struc pts damage done modifier";
      if (!namesonly)
        self.RuleVar[128] = 0.25f;
      self.RuleGroup[128] = 9;
      self.RuleString[129] = "Morale % penalty if individual panics";
      if (!namesonly)
        self.RuleVar[129] = 50f;
      self.RuleGroup[129] = 9;
      self.RuleString[130] = "Max Supply Consume modifier";
      if (!namesonly)
        self.RuleVar[130] = 0.75f;
      self.RuleGroup[130] = 9;
      self.RuleString[132] = "Concentric Attack Default 10% bonus";
      if (!namesonly)
        self.RuleVar[132] = 0.1f;
      self.RuleGroup[132] = 9;
      self.RuleString[133] = "Concentric Attack Default 25% bonus";
      if (!namesonly)
        self.RuleVar[133] = 0.25f;
      self.RuleGroup[133] = 9;
      self.RuleString[134] = "Concentric Attack Default 50% bonus";
      if (!namesonly)
        self.RuleVar[134] = 0.5f;
      self.RuleGroup[134] = 9;
      self.RuleString[135] = "Concentric Attack Default 75% bonus";
      if (!namesonly)
        self.RuleVar[135] = 0.75f;
      self.RuleGroup[135] = 9;
      self.RuleString[136] = "Concentric Attack Default 100% bonus";
      if (!namesonly)
        self.RuleVar[136] = 1f;
      self.RuleGroup[136] = 9;
      self.RuleString[137] = "Concentric Attack Default 150% bonus";
      if (!namesonly)
        self.RuleVar[137] = 1.5f;
      self.RuleGroup[137] = 9;
      self.RuleString[138] = "Concentric Attack Default 200% bonus";
      if (!namesonly)
        self.RuleVar[138] = 2f;
      self.RuleGroup[138] = 9;
      self.RuleString[139] = "Concentric Attack Default 250% bonus";
      if (!namesonly)
        self.RuleVar[139] = 2.5f;
      self.RuleGroup[139] = 9;
      self.RuleString[142] = "Allow artillery to fireback if artillery attack. (counter battery fire) 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[142] = 0.0f;
      self.RuleGroup[142] = 9;
      self.RuleString[925] = "Allow counter-attack when already beyond max-attacked. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[925] = 0.0f;
      self.RuleGroup[925] = 9;
      self.RuleString[143] = "Divisional Bonus Step 1";
      if (!namesonly)
        self.RuleVar[143] = -1f;
      self.RuleGroup[143] = 9;
      self.RuleString[144] = "Divisional Bonus Step 2";
      if (!namesonly)
        self.RuleVar[144] = 0.0f;
      self.RuleGroup[144] = 9;
      self.RuleString[145] = "Divisional Bonus Step 3";
      if (!namesonly)
        self.RuleVar[145] = 0.0f;
      self.RuleGroup[145] = 9;
      self.RuleString[146] = "Divisional Bonus Step 4";
      if (!namesonly)
        self.RuleVar[146] = 0.0f;
      self.RuleGroup[146] = 9;
      self.RuleString[147] = "Intercept range as 0.x of normal range";
      if (!namesonly)
        self.RuleVar[147] = 0.5f;
      self.RuleGroup[147] = 9;
      self.RuleString[300] = "Max division of attackval due to maxattacked";
      if (!namesonly)
        self.RuleVar[300] = 3f;
      self.RuleGroup[300] = 9;
      self.RuleString[301] = "If retreat with rdn <x chance for surrender (0=never)";
      if (!namesonly)
        self.RuleVar[301] = 25f;
      self.RuleGroup[301] = 9;
      self.RuleString[316] = "Non Artillery Land Attacks 1st round modifier. <1 is a penalty. 1=normal";
      if (!namesonly)
        self.RuleVar[316] = 0.0f;
      self.RuleGroup[316] = 9;
      self.RuleString[317] = "Non Artillery Land Attacks 2nd round modifier. <1 is a penalty. 1=normal";
      if (!namesonly)
        self.RuleVar[317] = 0.0f;
      self.RuleGroup[317] = 9;
      self.RuleString[318] = "Do airstrike on empty hex? 1=yes 0=no";
      if (!namesonly)
        self.RuleVar[318] = 0.0f;
      self.RuleGroup[318] = 9;
      self.RuleString[319] = "On startup lower Kill ratio to retreat hit. 1=no changes. 0.x = lower.";
      if (!namesonly)
        self.RuleVar[319] = 0.0f;
      self.RuleGroup[319] = 9;
      self.RuleString[325] = "Extra AP cost to move into hex (ap combat penalty) per combat round (fought with 100 stack points). Max 10 times this value.";
      if (!namesonly)
        self.RuleVar[325] = 0.0f;
      self.RuleGroup[325] = 9;
      self.RuleString[341] = "Airstrike always fights up all its action points like bombing attack (0=no 1=yes) ";
      if (!namesonly)
        self.RuleVar[341] = 0.0f;
      self.RuleGroup[341] = 9;
      self.RuleString[807] = "No counterattacks. 1=disable counter attacks. 0=keep counterattacks ";
      if (!namesonly)
        self.RuleVar[807] = 0.0f;
      self.RuleGroup[807] = 9;
      self.RuleString[833] = "AttackMax in FrontagePts for Air to Ground battle in hex. 0=not active";
      if (!namesonly)
        self.RuleVar[833] = 100f;
      self.RuleGroup[833] = 9;
      self.RuleString[834] = "AttackMax in FrontagePts for Artillery attack on hex. 0=not active";
      if (!namesonly)
        self.RuleVar[834] = 100f;
      self.RuleGroup[834] = 9;
      self.RuleString[835] = "Airfield suprise start reduce attackpower % in combat. 0=none/rule not active";
      if (!namesonly)
        self.RuleVar[835] = 0.0f;
      self.RuleGroup[835] = 9;
      self.RuleString[836] = "Airfield suprise reducement diminishment every round.";
      if (!namesonly)
        self.RuleVar[836] = 0.0f;
      self.RuleGroup[836] = 9;
      self.RuleString[837] = "Intercept Chance Rule. Percentage chance of fail to intercept at edge of radius. 0=dont use rule";
      if (!namesonly)
        self.RuleVar[837] = 0.0f;
      self.RuleGroup[837] = 9;
      self.RuleString[838] = "Intercept Chance Rule. Percentage chance of fail to intercept at theoretical 0 distance. 0=dont use rule";
      if (!namesonly)
        self.RuleVar[838] = 0.0f;
      self.RuleGroup[838] = 9;
      self.RuleString[841] = "Put combat kills/loss statistics in the round they happened (as opposed to round they get reported in). 1=yes.";
      if (!namesonly)
        self.RuleVar[841] = 0.0f;
      self.RuleGroup[841] = 9;
      self.RuleString[876] = "Dont Use only % of xp bonus on hp. 0=normal/default.. 50=50%.. max 100%=no xp bonus on hitpoints.";
      if (!namesonly)
        self.RuleVar[876] = 0.0f;
      self.RuleGroup[876] = 9;
      self.RuleString[877] = "Dont Use only % of xp bonus on att points. 0=normal/default.. 50=50%.. max 100%=no xp bonus on attack points.";
      if (!namesonly)
        self.RuleVar[877] = 0.0f;
      self.RuleGroup[877] = 9;
      self.RuleString[895] = "Artillery capable equipment gets retreat kill modifier in NON-artillery attack. 0=no modifier used!. >0 is modifier use: 0.5 = half, 2=double";
      if (!namesonly)
        self.RuleVar[895] = 0.0f;
      self.RuleGroup[895] = 9;
      self.RuleString[898] = "When location is taking from any side by another side structural points are set to 0. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[898] = 0.0f;
      self.RuleGroup[898] = 9;
      self.RuleString[326] = "Navy movement can get ambushed. 0=no 25 ap losee. 1=yes battle.";
      if (!namesonly)
        self.RuleVar[326] = 0.0f;
      self.RuleGroup[326] = 15;
      self.RuleString[328] = "Allies share recon. 0=no 1=yes.";
      if (!namesonly)
        self.RuleVar[328] = 0.0f;
      self.RuleGroup[328] = 15;
      self.RuleString[342] = "Allies support ally vs other ally. 0=no 1=yes.";
      if (!namesonly)
        self.RuleVar[342] = 0.0f;
      self.RuleGroup[342] = 15;
      self.RuleString[337] = "Use Auto Reinforce (0=no, 1=yes)";
      if (!namesonly)
        self.RuleVar[337] = 0.0f;
      self.RuleGroup[337] = 16;
      self.RuleString[977] = "Forbid disbanding units by replacement% setting (0=no, 1=yes)";
      if (!namesonly)
        self.RuleVar[977] = 0.0f;
      self.RuleGroup[977] = 16;
      self.RuleString[910] = "Auto Reinforce Mobility MoveType To Keep #1 (0=none)";
      if (!namesonly)
        self.RuleVar[910] = 0.0f;
      self.RuleGroup[910] = 16;
      self.RuleString[911] = "Auto Reinforce Mobility MoveType To Keep #2 (0=none)";
      if (!namesonly)
        self.RuleVar[911] = 0.0f;
      self.RuleGroup[911] = 16;
      self.RuleString[912] = "Auto Reinforce Mobility MoveType To Keep #3 (0=none)";
      if (!namesonly)
        self.RuleVar[912] = 0.0f;
      self.RuleGroup[912] = 16;
      self.RuleString[884] = "Unit Integrity based on: 0=start power, 1=hist.unit ideal";
      if (!namesonly)
        self.RuleVar[884] = 0.0f;
      self.RuleGroup[884] = 16;
      self.RuleString[949] = "HexLibVar-based oil use (0=dont, >0 is the X in the hexlibvar name 'oilX' thats used for looking up regimeslot#)";
      if (!namesonly)
        self.RuleVar[949] = 0.0f;
      self.RuleGroup[949] = 12;
      self.RuleString[885] = "";
      if (!namesonly)
        self.RuleVar[885] = 0.0f;
      self.RuleGroup[885] = 16;
      self.RuleString[338] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[338] = 0.0f;
      self.RuleGroup[338] = 0;
      self.RuleString[339] = "Auto Reinforce Max AP range";
      if (!namesonly)
        self.RuleVar[339] = 0.0f;
      self.RuleGroup[339] = 16;
      self.RuleString[340] = "Auto Reinforce Use CapTypes (-1=free, 0-2=only that cap type, 3=all)";
      if (!namesonly)
        self.RuleVar[340] = 0.0f;
      self.RuleGroup[340] = 16;
      self.RuleString[343] = "Use Officerpool (1=yes)";
      if (!namesonly)
        self.RuleVar[343] = 0.0f;
      self.RuleGroup[343] = 16;
      self.RuleString[344] = "Use Historical Units (1=yes)";
      if (!namesonly)
        self.RuleVar[344] = 0.0f;
      self.RuleGroup[344] = 12;
      self.RuleString[346] = "NOT USED";
      if (!namesonly)
        self.RuleVar[346] = 0.0f;
      self.RuleGroup[346] = 0;
      self.RuleString[347] = "-NOT USED-";
      if (!namesonly)
        self.RuleVar[347] = 0.0f;
      self.RuleGroup[347] = 0;
      self.RuleString[348] = "Enforce strict OOB rules for historical HQs (1=yes)";
      if (!namesonly)
        self.RuleVar[348] = 0.0f;
      self.RuleGroup[348] = 7;
      self.RuleString[354] = "Use unit integrity rules. 0=no. 1=yes";
      if (!namesonly)
        self.RuleVar[354] = 0.0f;
      self.RuleGroup[354] = 12;
      self.RuleString[355] = "Keep cards if change officer through officerpool. 0=no. 1=yes";
      if (!namesonly)
        self.RuleVar[355] = 0.0f;
      self.RuleGroup[355] = 16;
      self.RuleString[329] = "Dont show map looping even if it is looped 1=dont show. 0=show";
      if (!namesonly)
        self.RuleVar[329] = 0.0f;
      self.RuleGroup[329] = 14;
      self.RuleString[330] = "Show map border. 0=dont show. 1=show";
      if (!namesonly)
        self.RuleVar[330] = 0.0f;
      self.RuleGroup[330] = 14;
      self.RuleString[334] = "Dont show readyness + supplystatus on counter. 0=do show. 1=hide";
      if (!namesonly)
        self.RuleVar[334] = 0.0f;
      self.RuleGroup[334] = 14;
      self.RuleString[352] = "Dont show labels in zoomed in mode. 0=do show. 1=hide";
      if (!namesonly)
        self.RuleVar[352] = 0.0f;
      self.RuleGroup[352] = 14;
      self.RuleString[353] = "Allow setting of Shrowd. 0=yes allow. 1=disable";
      if (!namesonly)
        self.RuleVar[353] = 0.0f;
      self.RuleGroup[353] = 14;
      self.RuleString[908] = "Draw river last. 0=no , 1=yes ";
      if (!namesonly)
        self.RuleVar[908] = 0.0f;
      self.RuleGroup[908] = 14;
      self.RuleString[879] = "Officer description mode. 0=text, 1=hisvar stats.";
      if (!namesonly)
        self.RuleVar[879] = 0.0f;
      self.RuleGroup[879] = 14;
      self.RuleString[860] = "Show locations power pobar: i32 and HQ on map with code: Color. 1=yes, 0=no.";
      if (!namesonly)
        self.RuleVar[860] = 0.0f;
      self.RuleGroup[860] = 14;
      self.RuleString[265] = "NEW AI/DC2 AI: If falls below or equal this ammount of VP. It means (DEAD) end of game. 0=not used.";
      if (!namesonly)
        self.RuleVar[265] = 0.0f;
      self.RuleGroup[265] = 18;
      self.RuleGroup2[265] = 19;
      self.RuleString[266] = "NEW AI/DC2 AI: If get above or equal this ammount of VP. It means VICTORY. 0=not used.";
      if (!namesonly)
        self.RuleVar[266] = 0.0f;
      self.RuleGroup[266] = -1;
      self.RuleString[267] = "NEW AI: Areagroups are defined in slot# (1-9). 0=not used.";
      if (!namesonly)
        self.RuleVar[267] = 0.0f;
      self.RuleGroup[267] = 18;
      self.RuleString[268] = "NEW AI: ";
      if (!namesonly)
        self.RuleVar[268] = 0.0f;
      self.RuleGroup[268] = 0;
      self.RuleString[356] = "NEW AI. expect divisions / multi-united historical units. 0=do expect. 1=do not expect";
      if (!namesonly)
        self.RuleVar[356] = 0.0f;
      self.RuleGroup[356] = 18;
      self.RuleString[357] = "NEW AI. VP without locations are areacenters too. 0=no they are not 1=yes they are";
      if (!namesonly)
        self.RuleVar[357] = 0.0f;
      self.RuleGroup[357] = 18;
      self.RuleString[358] = "NEW AI. Attack from X points. 0=default. >0 overrule=> 20=aggressive very much. 80=cautious. ";
      if (!namesonly)
        self.RuleVar[358] = 0.0f;
      self.RuleGroup[358] = 18;
      self.RuleString[359] = "NEW AI. Hex occupation modifier for ATTacker. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[359] = 0.0f;
      self.RuleGroup[359] = 18;
      self.RuleString[360] = "NEW AI. Hex occupation modifier for DEFender. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[360] = 0.0f;
      self.RuleGroup[360] = 18;
      self.RuleString[361] = "NEW AI. Air Penalty distance. 0=default. >0 is number of hexes equal below gives penalty. ";
      if (!namesonly)
        self.RuleVar[361] = 0.0f;
      self.RuleGroup[361] = 18;
      self.RuleString[362] = "NEW AI. Normal HQ Penalty distance. 0=default. >0 is number of hexes equal below gives penalty. ";
      if (!namesonly)
        self.RuleVar[362] = 0.0f;
      self.RuleGroup[362] = 18;
      self.RuleString[363] = "NEW AI. Penalties given to Air/HQ. 0=default. >0 is number of points. ";
      if (!namesonly)
        self.RuleVar[363] = 0.0f;
      self.RuleGroup[363] = 18;
      self.RuleString[364] = "NEW AI. Penalties given to Air/HQ in Port hex. 0=default. >0 is number of points. ";
      if (!namesonly)
        self.RuleVar[364] = 0.0f;
      self.RuleGroup[364] = 18;
      self.RuleString[365] = "NEW AI. Is HQ on supplyline/advancematrixline penalty mod. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[365] = 0.0f;
      self.RuleGroup[365] = 18;
      self.RuleString[366] = "NEW AI. Is average HQ power below 100% penalties. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[366] = 0.0f;
      self.RuleGroup[366] = 18;
      self.RuleString[367] = "NEW AI. Score modifier for Attacks for Attacker. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[367] = 0.0f;
      self.RuleGroup[367] = 18;
      self.RuleString[368] = "NEW AI. Score modifier for Attacks for Defender. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[368] = 0.0f;
      self.RuleGroup[368] = 18;
      self.RuleString[369] = "NEW AI. Moving in modifier for Attacker. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[369] = 0.0f;
      self.RuleGroup[369] = 18;
      self.RuleString[370] = "NEW AI. Moving in modifier for Defender. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[370] = 0.0f;
      self.RuleGroup[370] = 18;
      self.RuleString[371] = "NEW AI. Counterattack modifier for attacker. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[371] = 0.0f;
      self.RuleGroup[371] = 18;
      self.RuleString[372] = "NEW AI. Counterattack modifier for Defender. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[372] = 0.0f;
      self.RuleGroup[372] = 18;
      self.RuleString[373] = "NEW AI. Countermove modifier for attacker. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[373] = 0.0f;
      self.RuleGroup[373] = 18;
      self.RuleString[374] = "NEW AI. Countermove modifier for Defender. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[374] = 0.0f;
      self.RuleGroup[374] = 18;
      self.RuleString[375] = "NEW AI. Get ourselves encircled for attacker. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[375] = 0.0f;
      self.RuleGroup[375] = 18;
      self.RuleString[376] = "NEW AI. Get ourselves encircled for Defender. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[376] = 0.0f;
      self.RuleGroup[376] = 18;
      self.RuleString[377] = "NEW AI. Entrenchment points modifier for Attacker. 0=default (def=low). 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[377] = 0.0f;
      self.RuleGroup[377] = 18;
      self.RuleString[378] = "NEW AI. Entrenchments points modifier for Defender. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[378] = 0.0f;
      self.RuleGroup[378] = 18;
      self.RuleString[379] = "NEW AI. Stacking penalty modifier. 0=default. 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[379] = 0.0f;
      self.RuleGroup[379] = 18;
      self.RuleString[380] = "NEW AI. Enemy encircled modifier for Attacker. 0=default (def=25%). 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[380] = 0.0f;
      self.RuleGroup[380] = 18;
      self.RuleString[381] = "NEW AI. Enemy encircled modifier for Defender. 0=default (def=25%). 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[381] = 0.0f;
      self.RuleGroup[381] = 18;
      self.RuleString[382] = "NEW AI. AdvanceMatrix for units for Attacker. 0=default (def=30%). 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[382] = 0.0f;
      self.RuleGroup[382] = 18;
      self.RuleString[383] = "NEW AI. AdvanceMatrix for hexes for Attacker. 0=default (def=15%). 100%. 50%=half. 200%=double. ";
      if (!namesonly)
        self.RuleVar[383] = 0.0f;
      self.RuleGroup[383] = 18;
      self.RuleString[384] = "NEW AI. Combat Target Size modifier Above X powpts. 0=default(def=80) >0 is X powpts";
      if (!namesonly)
        self.RuleVar[384] = 0.0f;
      self.RuleGroup[384] = 18;
      self.RuleString[385] = "NEW AI. Allow areas without road to be connected. 0=no. 1=yes";
      if (!namesonly)
        self.RuleVar[385] = 0.0f;
      self.RuleGroup[385] = 18;
      self.RuleString[386] = "NEW AI. Give defender full attacks in Strategy Simulation. 0=no. 1=yes";
      if (!namesonly)
        self.RuleVar[386] = 0.0f;
      self.RuleGroup[386] = 18;
      self.RuleString[387] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[387] = 0.0f;
      self.RuleGroup[387] = -1;
      self.RuleString[388] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[388] = 0.0f;
      self.RuleGroup[388] = -1;
      self.RuleString[389] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[389] = 0.0f;
      self.RuleGroup[389] = -1;
      self.RuleString[390] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[390] = 0.0f;
      self.RuleGroup[390] = -1;
      self.RuleString[391] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[391] = 0.0f;
      self.RuleGroup[391] = -1;
      self.RuleString[392] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[392] = 0.0f;
      self.RuleGroup[392] = -1;
      self.RuleString[393] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[393] = 0.0f;
      self.RuleGroup[393] = -1;
      self.RuleString[394] = "NEW AI. Modifier for strategy calcs for attack casualty rate. >1 less <1 more. 0=dont use";
      if (!namesonly)
        self.RuleVar[394] = 0.0f;
      self.RuleGroup[394] = 18;
      self.RuleString[395] = "NEW AI. Modifier for strategy calcs for counterattack casualty rate. >1 less <1 more. 0=dont use";
      if (!namesonly)
        self.RuleVar[395] = 0.0f;
      self.RuleGroup[395] = 18;
      self.RuleString[399] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[399] = 0.0f;
      self.RuleGroup[399] = 0;
      self.RuleString[800] = "NEW AI. Allow joining of non kampfgruppes. 0=dont. 1=do.";
      if (!namesonly)
        self.RuleVar[800] = 0.0f;
      self.RuleGroup[800] = 18;
      self.RuleString[802] = "NEW AI. Unlimited Strategic Transfer for AI. 0=dont. 1=do.";
      if (!namesonly)
        self.RuleVar[802] = 0.0f;
      self.RuleGroup[802] = 18;
      self.RuleString[803] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[803] = 0.0f;
      self.RuleGroup[803] = -1;
      self.RuleString[804] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[804] = 0.0f;
      self.RuleGroup[804] = -1;
      self.RuleString[805] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[805] = 0.0f;
      self.RuleGroup[805] = -1;
      self.RuleString[806] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[806] = 0.0f;
      self.RuleGroup[806] = -1;
      self.RuleString[809] = "NEW AI. Forbid AI to blow bridges. 1=forbid. 0=allow.";
      if (!namesonly)
        self.RuleVar[809] = 0.0f;
      self.RuleGroup[809] = 18;
      self.RuleString[810] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[810] = 0.0f;
      self.RuleGroup[810] = -1;
      self.RuleString[811] = "NEW AI. Friendly Strategic Corps Movement speed-up. 0=none.. 1=double... 2=triple";
      if (!namesonly)
        self.RuleVar[811] = 0.0f;
      self.RuleGroup[811] = 18;
      self.RuleString[812] = "NEW AI. LOG OPTION. Make screenshot of areas. 0=no. 1=yes";
      if (!namesonly)
        self.RuleVar[812] = 0.0f;
      self.RuleGroup[812] = 18;
      self.RuleString[813] = "NEW AI. AI CONSERVATIVE. 0=use always. 1=strategic only. 2=tactical only";
      if (!namesonly)
        self.RuleVar[813] = 0.0f;
      self.RuleGroup[813] = 18;
      self.RuleString[826] = "NEW AI. STRATEGIC CALCS. Prohibit enemy from moving fast. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[826] = 0.0f;
      self.RuleGroup[826] = 18;
      self.RuleString[827] = "NEW AI. STRATEGIC CALCS. Prohibit making alarm groups + offensive detachments. 0=no, 1=yes, 2=attacker only alarm";
      if (!namesonly)
        self.RuleVar[827] = 0.0f;
      self.RuleGroup[827] = 18;
      self.RuleString[828] = "NEW AI. STRATEGIC CALCS. Prohibit enemy transfer+relocate. 0=no, >1 is divided by that number. the more the less transfer";
      if (!namesonly)
        self.RuleVar[828] = 0.0f;
      self.RuleGroup[828] = 18;
      self.RuleString[829] = "NEW AI. STRATEGIC CALCS. STRATEGIC. Counter attack damage modifier. 0 is no. >1 is divided by that number ";
      if (!namesonly)
        self.RuleVar[829] = 0.0f;
      self.RuleGroup[829] = 18;
      self.RuleString[830] = "NEW AI. STRATEGIC CALCS. Allow aggressive relocate of enemy. (is std if enemy is attacker/meeting) 0=no 1=yes   ";
      if (!namesonly)
        self.RuleVar[830] = 0.0f;
      self.RuleGroup[830] = 18;
      self.RuleString[831] = "NEW AI. STRATEGIC CALCS. Extra (virtual) Hex Distance between areas to make counterattack slower. (4hex p.r)  ";
      if (!namesonly)
        self.RuleVar[831] = 0.0f;
      self.RuleGroup[831] = 18;
      self.RuleString[832] = "NEW AI. STRATEGIC CALCS IMPLEMENTATION. Add 1 round Inertia to changing attack plans. 0=no, 1=yes.  ";
      if (!namesonly)
        self.RuleVar[832] = 0.0f;
      self.RuleGroup[832] = 18;
      self.RuleString[842] = "NEW AI. Allow changing of SO Air intercept stance by AI. 0=no. >0 is the multiplication of own strenght in cosidering.  ";
      if (!namesonly)
        self.RuleVar[842] = 0.0f;
      self.RuleGroup[842] = 18;
      self.RuleString[844] = "NEW AI: Games last round. Used for predicting losing scn for attacker. 0=not used.";
      if (!namesonly)
        self.RuleVar[844] = 0.0f;
      self.RuleGroup[844] = 18;
      self.RuleString[888] = "NEW AI: All Corps(5) HQs act as if they are fixed";
      if (!namesonly)
        self.RuleVar[888] = 0.0f;
      self.RuleGroup[888] = 18;
      self.RuleString[889] = "NEW AI: Work with Sub-Corps groups. 0=no, 1=yes [not fully developed, only use in few round scenarios]";
      if (!namesonly)
        self.RuleVar[889] = 0.0f;
      self.RuleGroup[889] = 18;
      self.RuleString[890] = "NEW AI/DC2 AI: Enemy uses different supply movetype. 0=no, >0 = movetype used by enemy.";
      if (!namesonly)
        self.RuleVar[890] = 0.0f;
      self.RuleGroup[890] = 18;
      self.RuleGroup2[890] = 19;
      self.RuleString[913] = "DC2 AI: AI uses dynamic OOB+regime cards. 0=no, 1=yes, -1=no, but uses regime cards";
      if (!namesonly)
        self.RuleVar[913] = 0.0f;
      self.RuleGroup[913] = 19;
      self.RuleString[914] = "DC2 AI: AI gets free units, hq switches, officer assignments. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[914] = 0.0f;
      self.RuleGroup[914] = 19;
      self.RuleString[915] = "DC2 AI: General hisvar type best for officers in ANY command (0=dont use)";
      if (!namesonly)
        self.RuleVar[915] = 0.0f;
      self.RuleGroup[915] = 19;
      self.RuleString[916] = "DC2 AI: Officer hisvar type best for officers in NOT lowest HQ command (0=dont use)";
      if (!namesonly)
        self.RuleVar[916] = 0.0f;
      self.RuleGroup[916] = 19;
      self.RuleString[917] = "DC2 AI: Disable FULL RETREAT stance to avoid encirclement. 1=disable.";
      if (!namesonly)
        self.RuleVar[917] = 0.0f;
      self.RuleGroup[917] = 19;
      self.RuleString[918] = "DC2 AI: Disable normal RETREAT stance to keep line always. 1=disable";
      if (!namesonly)
        self.RuleVar[918] = 0.0f;
      self.RuleGroup[918] = 19;
      self.RuleString[919] = "DC2 AI: Garrison I only hex with X vp or more. 0=dont do";
      if (!namesonly)
        self.RuleVar[919] = 0.0f;
      self.RuleGroup[919] = 19;
      self.RuleString[920] = "DC2 AI: Garrison I only hex within X distance of enemy. 0=dont do";
      if (!namesonly)
        self.RuleVar[920] = 0.0f;
      self.RuleGroup[920] = 19;
      self.RuleString[921] = "DC2 AI: Always garrison port hexes irrespective of VP. 0=dont do, 1=yes do";
      if (!namesonly)
        self.RuleVar[921] = 0.0f;
      self.RuleGroup[921] = 19;
      self.RuleString[922] = "DC2 AI: Garrison hex with X vp or more always (irrespective enemy dist). 0=dont do";
      if (!namesonly)
        self.RuleVar[922] = 0.0f;
      self.RuleGroup[922] = 19;
      self.RuleString[923] = "DC2 AI: Garrison II only hex with X vp or more. 0=dont do";
      if (!namesonly)
        self.RuleVar[923] = 0.0f;
      self.RuleGroup[923] = 19;
      self.RuleString[924] = "DC2 AI: Garrison II only hex within X distance of enemy. 0=dont do";
      if (!namesonly)
        self.RuleVar[924] = 0.0f;
      self.RuleGroup[924] = 19;
      self.RuleString[928] = "DC2 AI: Top Operations (Panzer Focus). 0=dont do. 1=100% are, 2=50% are, 3=33.3% are..";
      if (!namesonly)
        self.RuleVar[928] = 0.0f;
      self.RuleGroup[928] = 19;
      self.RuleString[929] = "DC2 AI: Power Points in reinforcements expected for FRIENDLY.";
      if (!namesonly)
        self.RuleVar[929] = 0.0f;
      self.RuleGroup[929] = 19;
      self.RuleString[930] = "DC2 AI: Power Points in reinforcements expected for ENEMY.";
      if (!namesonly)
        self.RuleVar[930] = 0.0f;
      self.RuleGroup[930] = 19;
      self.RuleString[931] = "DC2 AI: Max iterations into the future for strategic (0=default, 4=normal).";
      if (!namesonly)
        self.RuleVar[931] = 0.0f;
      self.RuleGroup[931] = 19;
      self.RuleString[932] = "DC2 AI: Max frontline depth  (0=default value 3).";
      if (!namesonly)
        self.RuleVar[932] = 0.0f;
      self.RuleGroup[932] = 19;
      self.RuleString[933] = "DC2 AI: Max frontline length  (0=default value 12).";
      if (!namesonly)
        self.RuleVar[933] = 0.0f;
      self.RuleGroup[933] = 19;
      self.RuleString[934] = "DC2 AI: Also allow creation of new HQ if not officers in officerpool. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[934] = 0.0f;
      self.RuleGroup[934] = 19;
      self.RuleString[935] = "DC2 AI: Maximum units per corps (for new corps HQ calculations). 0=unlimited. ";
      if (!namesonly)
        self.RuleVar[935] = 0.0f;
      self.RuleGroup[935] = 19;
      self.RuleString[936] = "DC2 AI: Maximum corps per army (for new army HQ calculations). 0=unlimited. ";
      if (!namesonly)
        self.RuleVar[936] = 0.0f;
      self.RuleGroup[936] = 19;
      self.RuleString[937] = "DC2 AI: Howmany subparts must a 'armour' unit have to be considered a TOPunit. ";
      if (!namesonly)
        self.RuleVar[937] = 0.0f;
      self.RuleGroup[937] = 19;
      self.RuleString[938] = "DC2 AI: UberRegime gives all units except airTransport+Sea to unterRegime. 1=yes. ";
      if (!namesonly)
        self.RuleVar[938] = 0.0f;
      self.RuleGroup[938] = 19;
      self.RuleString[939] = "DC2 AI: Fortress Mode for encircled troops if >= then X VP. 0=never do. ";
      if (!namesonly)
        self.RuleVar[939] = 0.0f;
      self.RuleGroup[939] = 19;
      self.RuleString[940] = "DC2 AI: Always Protect VP in frontline fronts. 0=no, 1=yes, 2=yes even if retreat front (Delay action) ";
      if (!namesonly)
        self.RuleVar[940] = 0.0f;
      self.RuleGroup[940] = 19;
      self.RuleString[991] = "DC2 AI: AI Siege Artillery simulation. 0=not used. >0 is maximum entrenchment of enemy in that hex. example: 20 is max 20 entrenchment pts. ";
      if (!namesonly)
        self.RuleVar[991] = 0.0f;
      self.RuleGroup[991] = 19;
      self.RuleString[992] = "DC2 AI: AI suplementary combat bonus. 0=not used. >0 = extra combat bonus. ";
      if (!namesonly)
        self.RuleVar[992] = 0.0f;
      self.RuleGroup[992] = 19;
      self.RuleString[993] = "DC2 AI: AI never gets it concentric bonus reduced due to using units from different HQs. Plus extra emphasis on concentric. 0=no, 1=yes ";
      if (!namesonly)
        self.RuleVar[993] = 0.0f;
      self.RuleGroup[993] = 19;
      self.RuleString[994] = "DC2 AI: More emphasis on preventing enemy concentric attacks. 0=no, 1=yes ";
      if (!namesonly)
        self.RuleVar[994] = 0.0f;
      self.RuleGroup[994] = 19;
      self.RuleString[995] = "DC2 AI: AI never gets RdnLossPer100ap used. 0=no, 1=yes, 2=only half";
      if (!namesonly)
        self.RuleVar[995] = 0.0f;
      self.RuleGroup[995] = 19;
      self.RuleString[941] = "DC2 AI: Turn on debug screenshots and logs. 0=no, 1=yes. (can create lot of files, delete often) ";
      if (!namesonly)
        self.RuleVar[941] = 0.0f;
      self.RuleGroup[941] = 19;
      self.RuleString[942] = "DC2 AI: Modify minimum attack. 0=no mod. 1=no mod. <1 is less offensive power needed. >1 more offensive power needed.";
      if (!namesonly)
        self.RuleVar[942] = 0.0f;
      self.RuleGroup[942] = 19;
      self.RuleString[943] = "DC2 AI: Modify own strength perception. 0=no mod. 1=no mod. <1 is less strong. >1 stronger.";
      if (!namesonly)
        self.RuleVar[943] = 0.0f;
      self.RuleGroup[943] = 19;
      self.RuleString[745] = "DC2 AI: Do not assign strict front HQ to OTHER front if that front has origAverageStr > x : (0=dont use)";
      if (!namesonly)
        self.RuleVar[745] = 0.0f;
      self.RuleGroup[745] = 19;
      self.RuleString[746] = "DC2 AI: Do not assign strict front HQ from ORIGIN front if that front has origAverageStr < x : (0=dont use)";
      if (!namesonly)
        self.RuleVar[746] = 0.0f;
      self.RuleGroup[746] = 19;
      self.RuleString[747] = "DC2 AI: Multiplier for distance negative effect for front HQ change to another front (0=dont use)";
      if (!namesonly)
        self.RuleVar[747] = 0.0f;
      self.RuleGroup[747] = 19;
      self.RuleString[748] = "DC2 AI: Do not allow wiping of offensive zone assignments if other closer HQs are already assigned (0=dont use, 1=do not allow wipe)";
      if (!namesonly)
        self.RuleVar[748] = 0.0f;
      self.RuleGroup[748] = 19;
      self.RuleString[958] = "DC2 AI: Superfront use by looking at hex of HQ level at type X <=0 = disabled.  type=6=hq of lowest hq,7=hq of hq of lowest hq , etc.. ";
      if (!namesonly)
        self.RuleVar[958] = 0.0f;
      self.RuleGroup[958] = 19;
      self.RuleString[959] = "DC2 AI: if Superfront use. Uses event X to set superfront numbers to hexes. -1=do not call an event all values of 960 areaslot will be 0. ";
      if (!namesonly)
        self.RuleVar[959] = 0.0f;
      self.RuleGroup[959] = 19;
      self.RuleString[960] = "DC2 AI: if Superfront use. Uses areaslot X to read superfront numbers from hexes. (0-9)";
      if (!namesonly)
        self.RuleVar[960] = 0.0f;
      self.RuleGroup[960] = 19;
      self.RuleString[961] = "DC2 AI: Stricter HQ-Front assignments. Set to 1 to use. Usefull for scenarios where re-assigning HQs for units is not possible.";
      if (!namesonly)
        self.RuleVar[961] = 0.0f;
      self.RuleGroup[961] = 19;
      self.RuleString[962] = "DC2 AI: Long term prognosis enemy movement (ap) capacity. -1=increased, 0=default, 1=half, 2=none";
      if (!namesonly)
        self.RuleVar[962] = 0.0f;
      self.RuleGroup[962] = 19;
      self.RuleString[963] = "DC2 AI: Use offensive or defensive zones? 0=no, 1=offensive zones, 2=defensive zones";
      if (!namesonly)
        self.RuleVar[963] = 0.0f;
      self.RuleGroup[963] = 19;
      self.RuleString[964] = "DC2 AI: Offensive/defensive/retreat_mod zones. Use areaslot X to read zone numbers. (0-9) ";
      if (!namesonly)
        self.RuleVar[964] = 0.0f;
      self.RuleGroup[964] = 19;
      self.RuleString[965] = "DC2 AI: Use event X to set offensive/defensive zone numbers to hexes. -1=do not call an event all values of 964 areaslot will be 0.";
      if (!namesonly)
        self.RuleVar[965] = 0.0f;
      self.RuleGroup[965] = 19;
      self.RuleString[967] = "DC2 AI: Set >0 to enable Subgroups in Strict HQs (SSHQ). >=1 = max in subgroup.  ";
      if (!namesonly)
        self.RuleVar[967] = 0.0f;
      self.RuleGroup[967] = 19;
      self.RuleString[968] = "DC2 AI: Set >0 to enable different sized 2nd,3rd,etc.. groups in Strict HQs (SSHQ). >=1 = max in 2nd,3rd,etc.. subgroup. ";
      if (!namesonly)
        self.RuleVar[968] = 0.0f;
      self.RuleGroup[968] = 19;
      self.RuleString[969] = "DC2 AI: Use MLA >0 = yes. if >0 it specifies range of MLA average construction. ";
      if (!namesonly)
        self.RuleVar[969] = 0.0f;
      self.RuleGroup[969] = 19;
      self.RuleString[970] = "DC2 AI: Minimal HQ re-assignment for regular units. (works better with StrictHQs). 0=off/1=on ";
      if (!namesonly)
        self.RuleVar[970] = 0.0f;
      self.RuleGroup[970] = 19;
      self.RuleString[979] = "DC2 AI: At what hex size does a classic defensive zone becomes a broad defensive zone? 0=off/>0=hexes. ";
      if (!namesonly)
        self.RuleVar[979] = 0.0f;
      self.RuleGroup[979] = 19;
      self.RuleString[980] = "DC2 AI: Use event X to set retreat modifier. -1=do not call an event all values of 964 areaslot will be 100. ";
      if (!namesonly)
        self.RuleVar[980] = 0.0f;
      self.RuleGroup[980] = 19;
      self.RuleString[981] = "DC2 AI: Use event X to set strength modifier. -1=do not call an event all values of 964 areaslot will be 100. ";
      if (!namesonly)
        self.RuleVar[981] = 0.0f;
      self.RuleGroup[981] = 19;
      self.RuleString[985] = "DC2 AI: Strength modifier is also minimum combat advantage modifier. 0=no. 1=yes. ";
      if (!namesonly)
        self.RuleVar[985] = 0.0f;
      self.RuleGroup[985] = 19;
      self.RuleString[986] = "DC2 AI: Offensive zones go ALL-OUT for breakthrough and terrain gain. 0=no. 1=yes. 2=extremely so. ";
      if (!namesonly)
        self.RuleVar[986] = 0.0f;
      self.RuleGroup[986] = 19;
      self.RuleString[987] = "DC2 AI: AI uses different rulevar73 hexes of 100% hq power. 0=no >0 = the value used. ";
      if (!namesonly)
        self.RuleVar[987] = 0.0f;
      self.RuleGroup[987] = 19;
      self.RuleString[982] = "DC2 AI: Favorise top-unit attacks. 0=don't. >=1 is extra strength in % point + >=1 means unit favorised as attack choice. ";
      if (!namesonly)
        self.RuleVar[982] = 0.0f;
      self.RuleGroup[982] = 19;
      self.RuleString[891] = "NEW AI: Test method. 0=dont use. >0= ID of historical unit HQ who's group is only tactical test";
      if (!namesonly)
        self.RuleVar[891] = 0.0f;
      self.RuleGroup[891] = 18;
      self.RuleString[892] = "NEW AI: Test method. 0=dont use. >0= ID of historical unit HQ who's group is only tactical test";
      if (!namesonly)
        self.RuleVar[892] = 0.0f;
      self.RuleGroup[892] = 18;
      self.RuleString[893] = "NEW AI: Test method. 0=dont use. >0= ID of historical unit HQ who's group is only tactical test";
      if (!namesonly)
        self.RuleVar[893] = 0.0f;
      self.RuleGroup[893] = 18;
      self.RuleString[989] = "NEW AI: Offensive Zone Distance Agnosticity. 0=dont use. >0 = % of AP distance more allowed of top result. When active uses enemyPower as deciding ";
      if (!namesonly)
        self.RuleVar[989] = 0.0f;
      self.RuleGroup[989] = 18;
      self.RuleString[997] = "DC2 AI: Use Different Distance Importance Weights for StrictHQ reshuffles. 0=no, 1=dist less important, 2=much less  ";
      if (!namesonly)
        self.RuleVar[997] = 0.0f;
      self.RuleGroup[997] = 19;
      self.RuleString[465] = "DC2 AI: AI only change HQs if no HQ left... 0=no keep to default behaviour, 1=yes limit AI HQ changing ";
      if (!namesonly)
        self.RuleVar[465] = 0.0f;
      self.RuleGroup[465] = 19;
      self.RuleString[600] = "DC2 AI: Maximum number of Battlegroups created by the AI per round.";
      if (!namesonly)
        self.RuleVar[600] = 0.0f;
      self.RuleGroup[600] = 19;
      self.RuleString[601] = "DC2 AI: Prevent small low-mobility units to be seen as garrisons 0/1";
      if (!namesonly)
        self.RuleVar[601] = 0.0f;
      self.RuleGroup[601] = 19;
      self.RuleString[602] = "DC2 AI: Block Product6 overruling of garrison rules 0/1";
      if (!namesonly)
        self.RuleVar[602] = 0.0f;
      self.RuleGroup[602] = 19;
      self.RuleString[603] = "DC2 AI: Assume city level X will be able to receive air supply (0=no assumption here)";
      if (!namesonly)
        self.RuleVar[603] = 0.0f;
      self.RuleGroup[603] = 19;
      self.RuleString[604] = "DC2 AI: Play Cards that put Supply Bases on hexes (0=no, 1=yes rarely, 2=yes sometimes, 4=when needed,8=often) ";
      if (!namesonly)
        self.RuleVar[604] = 0.0f;
      self.RuleGroup[604] = 19;
      self.RuleString[706] = "DC2 AI: U meta relocations every X rounds ";
      if (!namesonly)
        self.RuleVar[706] = 0.0f;
      self.RuleGroup[706] = 19;
      self.RuleString[707] = "DC2 AI: Use meta relocations stringlist slot";
      if (!namesonly)
        self.RuleVar[707] = 0.0f;
      self.RuleGroup[707] = 19;
      self.RuleString[466] = "DC4 AI: Do not wait out enemy pockets... keep attacking.. 0=no/default, 1=yes hammer them out. ";
      if (!namesonly)
        self.RuleVar[466] = 0.0f;
      self.RuleGroup[466] = 19;
      self.RuleString[502] = "Disable Action Cards (0=no 1=yes)";
      if (!namesonly)
        self.RuleVar[502] = 0.0f;
      self.RuleGroup[502] = 12;
      self.RuleString[503] = "Disable Build Infra (0=no 1=yes)";
      if (!namesonly)
        self.RuleVar[503] = 0.0f;
      self.RuleGroup[503] = 12;
      self.RuleString[505] = "Disable Blow Bridge(0=no 1=yes)";
      if (!namesonly)
        self.RuleVar[505] = 0.0f;
      self.RuleGroup[505] = 12;
      self.RuleString[507] = "Disable Load & Unload (0=no 1=yes)";
      if (!namesonly)
        self.RuleVar[507] = 0.0f;
      self.RuleGroup[507] = 12;
      self.RuleString[508] = "Obsolete".to_owned();
      if (!namesonly)
        self.RuleVar[508] = 0.0f;
      self.RuleGroup[508] = 0;
      self.RuleString[511] = "Disable Naval artillery and attacks";
      if (!namesonly)
        self.RuleVar[511] = 0.0f;
      self.RuleGroup[511] = 12;
      self.RuleString[512] = "Disable add unit";
      if (!namesonly)
        self.RuleVar[512] = 0.0f;
      self.RuleGroup[512] = 12;
      self.RuleString[514] = "OBSOLETE".to_owned();
      if (!namesonly)
        self.RuleVar[514] = 0.0f;
      self.RuleGroup[514] = 0;
      self.RuleString[515] = "Disable paradrop & airlift button";
      if (!namesonly)
        self.RuleVar[515] = 0.0f;
      self.RuleGroup[515] = 12;
      self.RuleString[516] = "Disable air supply";
      if (!namesonly)
        self.RuleVar[516] = 0.0f;
      self.RuleGroup[516] = 12;
      self.RuleString[520] = "Disable strategic transfer";
      if (!namesonly)
        self.RuleVar[520] = 0.0f;
      self.RuleGroup[520] = 12;
      self.RuleString[521] = "Disable change HQ (2=allow if no HQ at moment)";
      if (!namesonly)
        self.RuleVar[521] = 0.0f;
      self.RuleGroup[521] = 12;
      self.RuleString[522] = "Disable Air Recon";
      if (!namesonly)
        self.RuleVar[522] = 0.0f;
      self.RuleGroup[522] = 12;
      self.RuleString[523] = "Disable Supply Layer button";
      if (!namesonly)
        self.RuleVar[523] = 0.0f;
      self.RuleGroup[523] = 12;
      self.RuleString[525] = "Enable Neutral Sea Hex Sharing";
      if (!namesonly)
        self.RuleVar[525] = 0.0f;
      self.RuleGroup[525] = 12;
      self.RuleString[526] = "New Unit through historical Units";
      if (!namesonly)
        self.RuleVar[526] = 0.0f;
      self.RuleGroup[526] = 12;
      self.RuleString[527] = "New Subunits order enabled";
      if (!namesonly)
        self.RuleVar[527] = 0.0f;
      self.RuleGroup[527] = 12;
      self.RuleString[528] = "Enable Giving of Unit to Ally";
      if (!namesonly)
        self.RuleVar[528] = 0.0f;
      self.RuleGroup[528] = 12;
      self.RuleString[529] = "Enable Giving of Hex to Ally";
      if (!namesonly)
        self.RuleVar[529] = 0.0f;
      self.RuleGroup[529] = 12;
      self.RuleString[859] = "Disable battlestack remember use for land/air/art (1=disabled)";
      if (!namesonly)
        self.RuleVar[859] = 0.0f;
      self.RuleGroup[859] = 12;
      self.RuleString[531] = "Enable Changing Model of Unit (1=yes)";
      if (!namesonly)
        self.RuleVar[531] = 0.0f;
      self.RuleGroup[531] = 12;
      self.RuleString[533] = "Disable Group Orders (move/strategic) (1=yes) ";
      if (!namesonly)
        self.RuleVar[533] = 0.0f;
      self.RuleGroup[533] = 12;
      self.RuleString[534] = "Enable SF Model Design (1=yes) ";
      if (!namesonly)
        self.RuleVar[534] = 0.0f;
      self.RuleGroup[534] = 12;
      self.RuleString[808] = "OBSOLETE ";
      if (!namesonly)
        self.RuleVar[808] = 0.0f;
      self.RuleGroup[808] = 12;
      self.RuleString[815] = "Only allow execution of events in eventgroup X (0=all events allowed) ";
      if (!namesonly)
        self.RuleVar[815] = 0.0f;
      self.RuleGroup[815] = 12;
      self.RuleString[816] = "Full Hide Enemy Option ";
      if (!namesonly)
        self.RuleVar[816] = 0.0f;
      self.RuleGroup[816] = 2;
      self.RuleString[817] = "Disable Getting Handcards";
      if (!namesonly)
        self.RuleVar[817] = 0.0f;
      self.RuleGroup[817] = 12;
      self.RuleString[899] = "Minimum artillery % to include unit in 'ALL' button of artillery attack.";
      if (!namesonly)
        self.RuleVar[899] = 0.0f;
      self.RuleGroup[899] = 14;
      self.RuleString[845] = "Enable Sideways Graphics for Landscapetypes (1=yes)";
      if (!namesonly)
        self.RuleVar[845] = 0.0f;
      self.RuleGroup[845] = 14;
      self.RuleString[846] = "People Graphic offset in pixels (0=no offset)";
      if (!namesonly)
        self.RuleVar[846] = 0.0f;
      self.RuleGroup[846] = 14;
      self.RuleString[847] = "New Counter mode (0=old AT, 1=yes)";
      if (!namesonly)
        self.RuleVar[847] = 0.0f;
      self.RuleGroup[847] = 14;
      self.RuleString[849] = "Equipment offset for people after equipment draw (0=no offset)";
      if (!namesonly)
        self.RuleVar[849] = 0.0f;
      self.RuleGroup[849] = 14;
      self.RuleString[909] = "Show his.var number on side of unit (0=no, >0 hisvar type to show)";
      if (!namesonly)
        self.RuleVar[909] = 0.0f;
      self.RuleGroup[909] = 14;
      self.RuleString[976] = "GUI: DC3 - Custom DC3 GUI changes (1=yes,0=no) ";
      if (!namesonly)
        self.RuleVar[976] = 0.0f;
      self.RuleGroup[976] = 14;
      self.RuleString[540] = "GUI: Move Arrows (1=yes,0=no) ";
      if (!namesonly)
        self.RuleVar[540] = 0.0f;
      self.RuleGroup[540] = 14;
      self.RuleString[990] = "GUI: Alternate music tracks (1=yes,0=no) (Alternate version uses '_alternate' in end of filename)";
      if (!namesonly)
        self.RuleVar[990] = 0.0f;
      self.RuleGroup[990] = 14;
      self.RuleString[988] = "GUI: Help tab active? 0=no, >=1 is the ID of the stringlist used. ";
      if (!namesonly)
        self.RuleVar[988] = 0.0f;
      self.RuleGroup[988] = 14;
      self.RuleString[998] = "GUI: DC3 - Hex Textures (1=yes,0=no) ";
      if (!namesonly)
        self.RuleVar[998] = 0.0f;
      self.RuleGroup[998] = 14;
      self.RuleString[999] = "GUI: Big Counters Bigger Siluet (1=yes, 0=no) ";
      if (!namesonly)
        self.RuleVar[999] = 0.0f;
      self.RuleGroup[999] = 14;
      self.RuleString[451] = "GUI: Recoloring allowed (>1 = stringlist ID, 0=no)";
      if (!namesonly)
        self.RuleVar[451] = 0.0f;
      self.RuleGroup[451] = 14;
      self.RuleString[708] = "GUI: Show Victory State using DC4 Libraries like 'VR Basic Lib'";
      if (!namesonly)
        self.RuleVar[708] = 0.0f;
      self.RuleGroup[708] = 14;
      self.RuleString[848] = "Use landscape from which LT for Air trooptype background overrule. (0=dont)";
      if (!namesonly)
        self.RuleVar[848] = 0.0f;
      self.RuleGroup[848] = 14;
      self.RuleString[872] = "Use landscape from which LT for Navy trooptype background overrule. (0=dont)";
      if (!namesonly)
        self.RuleVar[872] = 0.0f;
      self.RuleGroup[872] = 14;
      self.RuleString[850] = "Do screenshot in zoom mode. 0=32x24, 1=64x48, 2=128x96";
      if (!namesonly)
        self.RuleVar[850] = 0.0f;
      self.RuleGroup[850] = 14;
      self.RuleString[866] = "obsolete".to_owned();
      if (!namesonly)
        self.RuleVar[866] = 0.0f;
      self.RuleGroup[866] = 14;
      self.RuleString[867] = "obsolete".to_owned();
      if (!namesonly)
        self.RuleVar[867] = 0.0f;
      self.RuleGroup[867] = 14;
      self.RuleString[878] = "Do not enforce the width+height<400 mapsize screenshot limit protection. 0=do enforce. 1=do not enforce.";
      if (!namesonly)
        self.RuleVar[878] = 0.0f;
      self.RuleGroup[878] = 14;
      self.RuleString[650] = "Extra Statistic 1. (see 700: String for name) 0=no, >0 = regimevar #";
      if (!namesonly)
        self.RuleVar[650] = 0.0f;
      self.RuleGroup[650] = 13;
      self.RuleString[651] = "Extra Statistic 2. (see 701: String for name) 0=no, >0 = regimevar #";
      if (!namesonly)
        self.RuleVar[651] = 0.0f;
      self.RuleGroup[651] = 13;
      self.RuleString[652] = "Extra Statistic 3. (see 702: String for name) 0=no, >0 = regimevar #";
      if (!namesonly)
        self.RuleVar[652] = 0.0f;
      self.RuleGroup[652] = 13;
      self.RuleString[313] = "Always show all statistics no matter of FOW or Shrowd setting. 0=off, 1=rule in effect.";
      if (!namesonly)
        self.RuleVar[313] = 0.0f;
      self.RuleGroup[313] = 13;
      self.RuleString[905] = "GUI shows Category Card Category Names. 0=no, 1=1 cat, 2=2, etc.., max 5. (see tempstrings for names)";
      if (!namesonly)
        self.RuleVar[905] = 0.0f;
      self.RuleGroup[905] = 13;
      self.RuleString[906] = "GUI: Block messages at turn startup. (0=no, 1=yes) ";
      if (!namesonly)
        self.RuleVar[906] = 0.0f;
      self.RuleGroup[906] = 13;
      self.RuleString[946] = "TroopType Intermediate Editor. Use event ID for writing SFTypes.";
      if (!namesonly)
        self.RuleVar[946] = 0.0f;
      self.RuleGroup[946] = 20;
      self.RuleString[947] = "Use Small Graphics for logo pictures in groupname 1000-1099. 0=no, 1=yes";
      if (!namesonly)
        self.RuleVar[947] = 0.0f;
      self.RuleGroup[947] = 13;
      self.RuleString[948] = "DEBUG: Switch on event debugging? 0=no, 1=yes ";
      if (!namesonly)
        self.RuleVar[948] = 0.0f;
      self.RuleGroup[948] = 13;
      self.RuleString[971] = "GUI: Use Cinematics Stringlist (0=no, >0 = ID of stringlist) ";
      if (!namesonly)
        self.RuleVar[971] = 0.0f;
      self.RuleGroup[971] = 13;
      if (DrawMod.TGame.AllowHeightMap)
      {
        self.RuleString[415] = "Use LAND Height Map Feature? 0=no, >0 = stringlist ID  ";
        if (!namesonly)
          self.RuleVar[415] = 0.0f;
        self.RuleGroup[415] = 21;
        self.RuleString[416] = "Use SEA Height Map Feature? 0=no, >0 = stringlist ID  ";
        if (!namesonly)
          self.RuleVar[416] = 0.0f;
        self.RuleGroup[416] = 21;
        self.RuleString[417] = "Use Beach Coloring Feature? 0=no, >0 = stringlist ID ";
        if (!namesonly)
          self.RuleVar[417] = 0.0f;
        self.RuleGroup[417] = 21;
        self.RuleString[418] = "Use River Canyoning? 0=no, >0 = stringlist ID  ";
        if (!namesonly)
          self.RuleVar[418] = 0.0f;
        self.RuleGroup[418] = 21;
      }
      if (DrawMod.TGame.AllowHeightMap)
      {
        self.RuleString[419] = "Use Advanced Recon Rules (LOS) System 0/1. Also enables Direct Fire. ";
        if (!namesonly)
          self.RuleVar[419] = 0.0f;
        self.RuleGroup[419] = 21;
        self.RuleString[420] = "Advanced Recon Rules. 1st Hex Divider";
        if (!namesonly)
          self.RuleVar[420] = 0.0f;
        self.RuleGroup[420] = 21;
        self.RuleString[421] = "Advanced Recon Rules. 2nd+ Hex Divider";
        if (!namesonly)
          self.RuleVar[421] = 0.0f;
        self.RuleGroup[421] = 21;
        self.RuleString[422] = "Advanced Recon Rules. Maximum Range. 0=no range";
        if (!namesonly)
          self.RuleVar[422] = 0.0f;
        self.RuleGroup[422] = 21;
        self.RuleString[423] = "Advanced Recon Rules. XP Level switch popenalty: i32 and bonus on Recon/Hide Pts";
        if (!namesonly)
          self.RuleVar[423] = 0.0f;
        self.RuleGroup[423] = 21;
        self.RuleString[424] = "Height Map Rules. Base Height Level Defending Units HitpoModifier: i32 for being lower";
        if (!namesonly)
          self.RuleVar[424] = 0.0f;
        self.RuleGroup[424] = 21;
        self.RuleString[425] = "Height Map Rules. Base Height Level Defending Units HitpoModifier: i32 for being higher";
        if (!namesonly)
          self.RuleVar[425] = 0.0f;
        self.RuleGroup[425] = 21;
        self.RuleString[426] = "Height Map Rules. Base Height Level Modifier for Movement without Road";
        if (!namesonly)
          self.RuleVar[426] = 0.0f;
        self.RuleGroup[426] = 21;
        self.RuleString[427] = "Height Map Rules. Base Height Level Modifier for Movement with Road";
        if (!namesonly)
          self.RuleVar[427] = 0.0f;
        self.RuleGroup[427] = 21;
        self.RuleString[428] = "Intercept Fire: 0=off, >=1=on, 1=1 round, 2=2 rounds, etc...";
        if (!namesonly)
          self.RuleVar[428] = 0.0f;
        self.RuleGroup[428] = 21;
        self.RuleString[429] = "Direct Fire can Target Rear-Area Troops chance. 0=no, 1=yes";
        if (!namesonly)
          self.RuleVar[429] = 0.0f;
        self.RuleGroup[429] = 21;
        self.RuleString[430] = "Indirect Ranged Fire gets bonus if LOS% on hex. 0=no, 1.0=100% bonus, 0.5=50% bonus (modified for Los%)";
        if (!namesonly)
          self.RuleVar[430] = 0.0f;
        self.RuleGroup[430] = 21;
        self.RuleString[431] = "Combat Recon: 0=off, 1=on";
        if (!namesonly)
          self.RuleVar[431] = 0.0f;
        self.RuleGroup[431] = 21;
        self.RuleString[432] = "Combat Recon: HitpoMaximum: i32 Increase Factor (*X)";
        if (!namesonly)
          self.RuleVar[432] = 0.0f;
        self.RuleGroup[432] = 21;
        self.RuleString[433] = "Combat Recon: Markers: 0=off, 1=on";
        if (!namesonly)
          self.RuleVar[433] = 0.0f;
        self.RuleGroup[433] = 21;
        self.RuleString[434] = "Simple Supply Need Rules: 0=off, 1=on";
        if (!namesonly)
          self.RuleVar[434] = 0.0f;
        self.RuleGroup[434] = 21;
        self.RuleString[435] = "Unit-based Fuel: 0=off, 1=on";
        if (!namesonly)
          self.RuleVar[435] = 0.0f;
        self.RuleGroup[435] = 21;
        self.RuleString[436] = "Intercept Fire: Reduction in Consumption and Rdn loss 0.x (0=free. 0.5=half cost)";
        if (!namesonly)
          self.RuleVar[436] = 0.0f;
        self.RuleGroup[436] = 21;
        self.RuleString[437] = "Simple Supply Need Rules: Maximum Supply Consume % loss in absolute points";
        if (!namesonly)
          self.RuleVar[437] = 0.0f;
        self.RuleGroup[437] = 21;
        self.RuleString[438] = "Simple Supply Need Rules: Maximum extra Morale Hit if hit while on reduced consumption rations";
        if (!namesonly)
          self.RuleVar[438] = 0.0f;
        self.RuleGroup[438] = 21;
        self.RuleString[467] = "Morale Hit for 1st attacked per round if not received any supplies (even if enough reserves)";
        if (!namesonly)
          self.RuleVar[467] = 0.0f;
        self.RuleGroup[467] = 9;
        self.RuleString[468] = "Readiness Hit for 1st attacked per round if not received any supplies (even if enough reserves)";
        if (!namesonly)
          self.RuleVar[468] = 0.0f;
        self.RuleGroup[468] = 9;
        self.RuleString[469] = "Morale Hit at start of turn if out-of-supply (even if enough reserves)";
        if (!namesonly)
          self.RuleVar[469] = 0.0f;
        self.RuleGroup[469] = 21;
        self.RuleString[470] = "Night Mode 0=no, 1=yes (graphical and some combat and movement effects).";
        if (!namesonly)
          self.RuleVar[470] = 0.0f;
        self.RuleGroup[470] = 21;
        self.RuleString[471] = "Use Supply Source Rules 0=off / >0 = loctype slot used.";
        if (!namesonly)
          self.RuleVar[471] = 0.0f;
        self.RuleGroup[471] = 21;
        self.RuleString[472] = "Use AP Reserve rules 0=off / >0 = AP gained per turn";
        if (!namesonly)
          self.RuleVar[472] = 0.0f;
        self.RuleGroup[472] = 21;
        self.RuleString[473] = "Maximum AP Reserve.";
        if (!namesonly)
          self.RuleVar[473] = 0.0f;
        self.RuleGroup[473] = 21;
        self.RuleString[474] = "Transport Movement Mode. 0=off >0 = minimum percentage Transport Movement Type of Unit.";
        if (!namesonly)
          self.RuleVar[474] = 0.0f;
        self.RuleGroup[474] = 21;
        self.RuleString[475] = "Battelgroup and Limited Transfer rules. 0=off, 1=on. ";
        if (!namesonly)
          self.RuleVar[475] = 0.0f;
        self.RuleGroup[475] = 21;
        self.RuleString[476] = "Minimum Power Points for Battlegroup.";
        if (!namesonly)
          self.RuleVar[476] = 0.0f;
        self.RuleGroup[476] = 21;
        self.RuleString[477] = "Maximum Power Points for Battlegroup.";
        if (!namesonly)
          self.RuleVar[477] = 0.0f;
        self.RuleGroup[477] = 21;
        self.RuleString[478] = "Battlegroup Rulevar that is unused for now.";
        if (!namesonly)
          self.RuleVar[478] = 0.0f;
        self.RuleGroup[478] = 21;
        self.RuleString[479] = "Optional Stringlist ID for Battlegroup Names. 0=none, >=1 is ID. ";
        if (!namesonly)
          self.RuleVar[479] = 0.0f;
        self.RuleGroup[479] = 21;
        self.RuleString[480] = "Counter-Battery Fire Revised Rules (they become priority targets) 0=no,1=yes. ";
        if (!namesonly)
          self.RuleVar[480] = 0.0f;
        self.RuleGroup[480] = 21;
        self.RuleString[481] = "Modified Retreat/Panic percentages. 0=no,1=yes. ";
        if (!namesonly)
          self.RuleVar[481] = 0.0f;
        self.RuleGroup[481] = 21;
        self.RuleString[482] = "Attack Type Option Available. 0=no,1=yes. ";
        if (!namesonly)
          self.RuleVar[482] = 0.0f;
        self.RuleGroup[482] = 21;
        self.RuleString[483] = "Engineers can construct bridge even if enemy on other side. 0=no,1=yes. ";
        if (!namesonly)
          self.RuleVar[483] = 0.0f;
        self.RuleGroup[483] = 21;
        self.RuleString[484] = "Different Move Modes available. 0=no, 1=yes ";
        if (!namesonly)
          self.RuleVar[484] = 0.0f;
        self.RuleGroup[484] = 21;
        self.RuleString[485] = "Intercept+Ranged Fire List feature. 0=no, 1=yes ";
        if (!namesonly)
          self.RuleVar[485] = 0.0f;
        self.RuleGroup[485] = 21;
        self.RuleString[486] = "Scrapping Rules. 0=no, >=1 is the ID of the Stringlist for instructions ";
        if (!namesonly)
          self.RuleVar[486] = 0.0f;
        self.RuleGroup[486] = 21;
        self.RuleString[487] = "New turn resets x% of Traffic Points to 0 (0=all, 5=only 5%, 50=50%, 75=75, 100=all)";
        if (!namesonly)
          self.RuleVar[487] = 0.0f;
        self.RuleGroup[487] = 21;
        self.RuleString[488] = "Fuel Weight 0.x... ";
        if (!namesonly)
          self.RuleVar[488] = 0.0f;
        self.RuleGroup[488] = 21;
        self.RuleString[489] = "Binary Fuel deliveries.. 0=no, 1=yes ";
        if (!namesonly)
          self.RuleVar[489] = 0.0f;
        self.RuleGroup[489] = 21;
        self.RuleString[490] = "Linear Carry Capacity.. 0=no, >0 = maximum extra carry cap% (for example 33 for 33%) ";
        if (!namesonly)
          self.RuleVar[490] = 0.0f;
        self.RuleGroup[490] = 21;
        self.RuleString[491] = "New Landscape Type Graphics System 0=no, 1=yes ";
        if (!namesonly)
          self.RuleVar[491] = 0.0f;
        self.RuleGroup[491] = 21;
        self.RuleString[492] = "New Trooptype Graphics System 0=no, 1=yes";
        if (!namesonly)
          self.RuleVar[492] = 0.0f;
        self.RuleGroup[492] = 21;
        self.RuleString[493] = "Allow partial-troop attacks (who can move in) 0=no, 1=yes";
        if (!namesonly)
          self.RuleVar[493] = 0.0f;
        self.RuleGroup[493] = 21;
        self.RuleString[494] = "Snow entrenchment modifier  (0= dont use , 50=half, 100=norm, 200=double). default=50";
        if (!namesonly)
          self.RuleVar[494] = 0.0f;
        self.RuleGroup[494] = 21;
        self.RuleString[495] = "Icy + slippery. For offroad and road. Increase AP cost per height lvl (0=nothing,50=50%) default=100";
        if (!namesonly)
          self.RuleVar[495] = 0.0f;
        self.RuleGroup[495] = 21;
        self.RuleString[496] = "NOT YET USED. Snowy roads not yet cleaned (0=nothing, 25=+25% , 50=+50%) default=+100 ";
        if (!namesonly)
          self.RuleVar[496] = 0.0f;
        self.RuleGroup[496] = 21;
        self.RuleString[497] = "Extra Morale impact based on power poloss: i32 and kill ratio of Unit…. 1dX …. Base = 6";
        if (!namesonly)
          self.RuleVar[497] = 0.0f;
        self.RuleGroup[497] = 21;
        self.RuleString[498] = "NOT YET USED. If enemy opposite when constructing bridge.. how much % it fails (0=none) default=50";
        if (!namesonly)
          self.RuleVar[498] = 0.0f;
        self.RuleGroup[498] = 21;
        self.RuleString[499] = "Supply Log Stringlist ID";
        if (!namesonly)
          self.RuleVar[499] = 0.0f;
        self.RuleGroup[499] = 21;
        self.RuleString[399] = "Vigor rules. 0=off , >0 is the number of points recovered for no move/no attack (15)";
        if (!namesonly)
          self.RuleVar[399] = 0.0f;
        self.RuleGroup[399] = 21;
        self.RuleString[398] = "Vigor reduction in points if did move/attack (5)";
        if (!namesonly)
          self.RuleVar[398] = 0.0f;
        self.RuleGroup[398] = 21;
        self.RuleString[397] = "Vigor rules. Loss of 1/1000th vigor per casualty % point";
        if (!namesonly)
          self.RuleVar[397] = 0.0f;
        self.RuleGroup[397] = 21;
        self.RuleString[396] = "Vigor rules. Maximum vigor loss per power point. (to avoid very small units excessive vigor loss)";
        if (!namesonly)
          self.RuleVar[396] = 0.0f;
        self.RuleGroup[396] = 21;
        self.RuleString[393] = "The turn after the night turn? 1=yes, 0=no";
        if (!namesonly)
          self.RuleVar[393] = 0.0f;
        self.RuleGroup[393] = 21;
        self.RuleString[439] = "Simple Supply Need Rules: Lowest reduced consumption ration allowed in % (25=always triest at least 25% of full consumption)";
        if (!namesonly)
          self.RuleVar[439] = 0.0f;
        self.RuleGroup[439] = 21;
        self.RuleString[455] = "Fuzzy Ownership: Hide normal Hex owner, use alternative indication for others. 0=off, 1=on";
        if (!namesonly)
          self.RuleVar[455] = 0.0f;
        self.RuleGroup[455] = 21;
        self.RuleString[456] = "Fuzzy Ownership: Are Blocked Markers visible? 0=no never, 1=on road/bridge, 2=on all hexes ";
        if (!namesonly)
          self.RuleVar[456] = 0.0f;
        self.RuleGroup[456] = 21;
        self.RuleString[458] = "Wider order bar buttons. 0=no, >0 = extra width ";
        if (!namesonly)
          self.RuleVar[458] = 0.0f;
        self.RuleGroup[458] = 21;
        self.RuleString[459] = "Use Road Traffic Points Rules 0=no, 1=yes ";
        if (!namesonly)
          self.RuleVar[459] = 0.0f;
        self.RuleGroup[459] = 21;
        self.RuleString[460] = "Use Bigger Info Window for Trooptypes 0=no, 1=yes ";
        if (!namesonly)
          self.RuleVar[460] = 0.0f;
        self.RuleGroup[460] = 21;
        self.RuleString[461] = "High HQs are Static, also OOB-wise, 0=no, 1=yes";
        if (!namesonly)
          self.RuleVar[461] = 0.0f;
        self.RuleGroup[461] = 21;
        self.RuleString[462] = "Supply Base System Active, 0=no, 1=yes  ";
        if (!namesonly)
          self.RuleVar[462] = 0.0f;
        self.RuleGroup[462] = 21;
        self.RuleString[463] = "Supply Base Supply Capture Percentage, 0=none, 50=half, 100=all ";
        if (!namesonly)
          self.RuleVar[463] = 0.0f;
        self.RuleGroup[463] = 21;
        self.RuleString[464] = "Movement Type that uses towing rules. 0=none ... >= 100 uses these rules. 100=movement type 0, 101=movtype 1, etc..";
        if (!namesonly)
          self.RuleVar[464] = 0.0f;
        self.RuleGroup[464] = 21;
      }
      self.RuleString[950] = "GUI: Run event for extra message generation when surrender is triggered for current player. (0=no, >0 event nr) ";
      if (!namesonly)
        self.RuleVar[950] = 0.0f;
      self.RuleGroup[950] = 14;
      self.RuleString[701] = "New Left/Right Click movement 0/1. ";
      if (!namesonly)
        self.RuleVar[701] = 0.0f;
      self.RuleGroup[701] = 22;
      self.RuleString[702] = "LIS: Infra setup. 0=no, >0=event Nr ";
      if (!namesonly)
        self.RuleVar[702] = 0.0f;
      self.RuleGroup[702] = 22;
      self.RuleString[703] = "LIS: Infra feedback. 0=no, >0=event Nr ";
      if (!namesonly)
        self.RuleVar[703] = 0.0f;
      self.RuleGroup[703] = 22;
      self.RuleString[704] = "LIS: Infra execute. 0=no, >0=event Nr ";
      if (!namesonly)
        self.RuleVar[704] = 0.0f;
      self.RuleGroup[704] = 22;
      self.RuleString[705] = "LIS: Infra select order. 0=no, >0=event Nr ";
      if (!namesonly)
        self.RuleVar[705] = 0.0f;
      self.RuleGroup[705] = 22;
      self.RuleString[440] = "UDS: Bottom Page UDS active. 0/1. 1=it will use the extraTab system to call eventNr's and get name for tab. ";
      if (!namesonly)
        self.RuleVar[440] = 0.0f;
      self.RuleGroup[440] = 22;
      self.RuleString[441] = "UDS: Management Tab pages Stringlist ID ";
      if (!namesonly)
        self.RuleVar[441] = 0.0f;
      self.RuleGroup[441] = 22;
      self.RuleString[442] = "Random Screen Event# (0>0 means Random Screen will be activated) ";
      if (!namesonly)
        self.RuleVar[442] = 0.0f;
      self.RuleGroup[442] = 22;
      self.RuleString[443] = "Randomscreen Tab pages stringlist ID# ";
      if (!namesonly)
        self.RuleVar[443] = 0.0f;
      self.RuleGroup[443] = 22;
      self.RuleString[444] = "Randomscreen Map Tab (0=no map shown, 1=yes show the map)";
      if (!namesonly)
        self.RuleVar[444] = 0.0f;
      self.RuleGroup[444] = 22;
      self.RuleString[446] = "Dynamic Sound System turned on (0=no, 1=yes)";
      if (!namesonly)
        self.RuleVar[446] = 0.0f;
      self.RuleGroup[446] = 22;
      self.RuleString[447] = "Dynamic Tracks stringlist ID (<=0 = none)";
      if (!namesonly)
        self.RuleVar[447] = 0.0f;
      self.RuleGroup[447] = 22;
      self.RuleString[448] = "Mix Points stringlist ID (<=0 = none)";
      if (!namesonly)
        self.RuleVar[448] = 0.0f;
      self.RuleGroup[448] = 22;
      self.RuleString[449] = "Highlights stringlist ID (<=0 = none)";
      if (!namesonly)
        self.RuleVar[449] = 0.0f;
      self.RuleGroup[449] = 22;
      self.RuleString[450] = "UDS: Small Right Bottom UDS window ";
      if (!namesonly)
        self.RuleVar[450] = 0.0f;
      self.RuleGroup[450] = 22;
      self.RuleString[401] = "GUI: Show Zone Boundaries on Map. (0=no, 1=yes), (needs setting of groupname 742,743 as well to read zone numbers) ";
      if (!namesonly)
        self.RuleVar[401] = 0.0f;
      self.RuleGroup[401] = 22;
      self.RuleString[402] = "Use event# for set units ready. 0=no, >0 = event slot called by each unit ";
      if (!namesonly)
        self.RuleVar[402] = 0.0f;
      self.RuleGroup[402] = 22;
      self.RuleString[403] = "Enable LIS systems 0/1. ";
      if (!namesonly)
        self.RuleVar[403] = 0.0f;
      self.RuleGroup[403] = 22;
      self.RuleString[404] = "LIS systems: Stringlist ID for items stringlist ";
      if (!namesonly)
        self.RuleVar[404] = 0.0f;
      self.RuleGroup[404] = 22;
      self.RuleString[405] = "LIS systems: Stringlist ID for transport mode stringlist ";
      if (!namesonly)
        self.RuleVar[405] = 0.0f;
      self.RuleGroup[405] = 22;
      self.RuleString[406] = "LIS systems: Stringlist ID for location points and free AP ";
      if (!namesonly)
        self.RuleVar[406] = 0.0f;
      self.RuleGroup[406] = 22;
      self.RuleString[407] = "LIS systems: Stringlist ID for SFType requests and uses ";
      if (!namesonly)
        self.RuleVar[407] = 0.0f;
      self.RuleGroup[407] = 22;
      self.RuleString[408] = "UDS Order Bar Active. 0=no, >0 = event number to call ";
      if (!namesonly)
        self.RuleVar[408] = 0.0f;
      self.RuleGroup[408] = 22;
      self.RuleString[412] = "LIS systems: Stringlist ID for SFType - RegimeId quality levels ";
      if (!namesonly)
        self.RuleVar[412] = 0.0f;
      self.RuleGroup[412] = 22;
      self.RuleString[413] = "Regime sprites: use composite sprite system? 1=yes, 0=no ";
      if (!namesonly)
        self.RuleVar[413] = 0.0f;
      self.RuleGroup[413] = 22;
      self.RuleString[414] = "SFType Illustrations: use composite SFType var sprite system? 0=no, >0 = stringlist ID ";
      if (!namesonly)
        self.RuleVar[414] = 0.0f;
      self.RuleGroup[414] = 22;
      self.RuleString[409] = "UDS Order Bar Stringlist ";
      if (!namesonly)
        self.RuleVar[409] = 0.0f;
      self.RuleGroup[409] = 22;
      self.RuleString[410] = "UDS Unit Tab. 0=no >0 = event number to call ";
      if (!namesonly)
        self.RuleVar[410] = 0.0f;
      self.RuleGroup[410] = 22;
      self.RuleString[411] = "Stringlist ID info to be shown in top bar. Col 0 = Regime Slot, Col 1= 'name specified in  ";
      if (!namesonly)
        self.RuleVar[411] = 0.0f;
      self.RuleGroup[411] = 22;
      self.RuleString[445] = "DEBUG: Turn on profiling of events, execs and checks (0=no, 1=yes) ";
      if (!namesonly)
        self.RuleVar[445] = 0.0f;
      self.RuleGroup[445] = 12;
      self.RuleString[978] = "GUI/Logic: On surrender make this the last round the player saw(0) or make the game continue (1)";
      if (!namesonly)
        self.RuleVar[978] = 0.0f;
      self.RuleGroup[978] = 14;
      self.RuleString[983] = "GUI/Logic: FRONT ZONES : Use 'front zones' matrix. 0=no, 1=use. ";
      if (!namesonly)
        self.RuleVar[983] = 0.0f;
      self.RuleGroup[983] = 14;
      self.RuleString[984] = "GUI/Logic: FRONT ZONES : Frontzone check is done with historical unit Type HQ (5-9): (set to <5 to use groupnames 723,724 and 725) ";
      if (!namesonly)
        self.RuleVar[984] = 0.0f;
      self.RuleGroup[984] = 14;
      self.RuleString[951] = "GUI: Use a stringlist ID for encyclopedia mouse over (col0=search txt,col1=title,col2=txt) (0=no) ";
      if (!namesonly)
        self.RuleVar[951] = 0.0f;
      self.RuleGroup[951] = 14;
      self.RuleString[952] = "Disable Air Attack Attack (0=no, 1=yes)";
      if (!namesonly)
        self.RuleVar[952] = 0.0f;
      self.RuleGroup[952] = 12;
      self.RuleString[953] = "Disable Artilley (0=no, 1=yes)";
      if (!namesonly)
        self.RuleVar[953] = 0.0f;
      self.RuleGroup[953] = 12;
      self.RuleString[954] = "Group move type (0=default same his.unit, 1=same hex of origin)";
      if (!namesonly)
        self.RuleVar[954] = 0.0f;
      self.RuleGroup[954] = 12;
      self.RuleString[955] = "Combat Logging. Battles stringlist ID# (<=0 not enabled) ";
      if (!namesonly)
        self.RuleVar[955] = 0.0f;
      self.RuleGroup[955] = 9;
      self.RuleString[956] = "Combat Logging. Historical Units stringlist ID# (<=0 not enabled) ";
      if (!namesonly)
        self.RuleVar[956] = 0.0f;
      self.RuleGroup[956] = 9;
      self.RuleString[957] = "Stringlist statistics. Use stringlist ID# (<=0 not enabled) ";
      if (!namesonly)
        self.RuleVar[957] = 0.0f;
      self.RuleGroup[957] = 13;
      self.RuleString[966] = "DEBUG: Use event-per-event debugging mode (old style) ? 0=no, 1=yes ";
      if (!namesonly)
        self.RuleVar[966] = 0.0f;
      self.RuleGroup[966] = 12;
    }

    pub fn SetDefaultTempStrings()
    {
      let mut stringCounter: i32 =  self.StringCounter;
      for (let mut index: i32 =  0; index <= stringCounter; index += 1)
        self.TempString[index] = "";
    }

    pub Clone: DataClass()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (DataClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn FindUnit(ref UnitClass uni, libName: String) -> i32
    {
      let mut unitCounter: i32 =  self.UnitCounter;
      for (let mut unit: i32 =  0; unit <= unitCounter; unit += 1)
      {
        if (self.UnitObj[unit].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.UnitObj[unit].LibId.libSlot].name, libName, false) == 0 && self.UnitObj[unit].LibId.id == uni.PreDef)
          return unit;
      }
      return -1;
    }

    pub fn FindPredef(id: i32) -> i32
    {
      let mut unitCounter: i32 =  self.UnitCounter;
      for (let mut predef: i32 =  0; predef <= unitCounter; predef += 1)
      {
        if (self.UnitObj[predef].PreDef == id)
          return predef;
      }
      return -1;
    }

    pub fn AddUnit(x: i32, y: i32, map: i32) -> i32
    {
      this += 1.UnitCounter;
      this += 1.AIUnitCounter;
      self.UnitObj = (UnitClass[]) Utils.CopyArray((Array) self.UnitObj, (Array) new UnitClass[self.UnitCounter + 1]);
      self.UnitObj[self.UnitCounter] = new UnitClass(0);
      self.UnitObj[self.UnitCounter].AIid = self.AIUnitCounter;
      if (x > -1)
      {
        self.UnitObj[self.UnitCounter].X = x;
        self.UnitObj[self.UnitCounter].Y = y;
        self.UnitObj[self.UnitCounter].Map = map;
      }
      self.UnitObj[self.UnitCounter].LoadSprites();
      if (x > -1)
        self.MapObj[map].HexObj[x, y].AddUnitToList(self.UnitCounter);
      return self.UnitCounter;
    }

    pub fn RemoveUnit(nr: i32, ref let mut tgame: GameClass = null, bool deleteRegimeMod = false)
    {
      if (self.UnitObj[nr].X > -1 & !Information.IsNothing( tgame))
        tgame.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(self.UnitObj[nr].X, self.UnitObj[nr].Y, self.UnitObj[nr].Map, -1, -1, -1, self.UnitObj[nr].Regime, nr);
      let mut x: i32 =  self.UnitObj[nr].X;
      let mut y: i32 =  self.UnitObj[nr].Y;
      let mut map: i32 =  self.UnitObj[nr].Map;
      if (self.UnitObj[nr].Historical > -1 && !self.HistoricalUnitObj[self.UnitObj[nr].Historical].Pool && Strings.Len(self.HistoricalUnitObj[self.UnitObj[nr].Historical].CommanderName) > 0)
      {
        if (self.HistoricalUnitObj[self.UnitObj[nr].Historical].PP < 0)
        {
          if (self.RegimeObj[self.UnitObj[nr].Regime].UberRegime > -1)
          {
            RegimeClass[] regimeObj = self.RegimeObj;
            RegimeClass[] regimeClassArray = regimeObj;
            let mut uberRegime: i32 =  self.RegimeObj[self.UnitObj[nr].Regime].UberRegime;
            let mut index: i32 =  uberRegime;
            regimeClassArray[index].ResPts = regimeObj[uberRegime].ResPts + self.HistoricalUnitObj[self.UnitObj[nr].Historical].PP;
          }
          else
          {
            RegimeClass[] regimeObj = self.RegimeObj;
            RegimeClass[] regimeClassArray = regimeObj;
            let mut regime: i32 =  self.UnitObj[nr].Regime;
            let mut index: i32 =  regime;
            regimeClassArray[index].ResPts = regimeObj[regime].ResPts + self.HistoricalUnitObj[self.UnitObj[nr].Historical].PP;
          }
        }
        self.HistoricalUnitObj[self.UnitObj[nr].Historical].Pool = true;
        self.HistoricalUnitObj[self.UnitObj[nr].Historical].TempRegime = self.UnitObj[nr].Regime;
        if (self.RegimeObj[self.UnitObj[nr].Regime].UberRegime > -1)
          self.HistoricalUnitObj[self.UnitObj[nr].Historical].TempRegime = self.RegimeObj[self.UnitObj[nr].Regime].UberRegime;
      }
      if (self.UnitObj[nr].SFCount > -1)
      {
        for (let mut sfCount: i32 =  self.UnitObj[nr].SFCount; sfCount >= 0; sfCount += -1)
          self.RemoveSF(self.UnitObj[nr].SFList[sfCount]);
      }
      self.UnitObj[nr].Kill();
      self.ChangeUnitNr(nr, -1, ref tgame, false, deleteRegimeMod);
      if (nr < self.UnitCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.UnitCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.UnitObj[Newnr] = self.UnitObj[Newnr + 1];
          self.ChangeUnitNr(Newnr + 1, Newnr, ref tgame, false, deleteRegimeMod);
        }
      }
      --self.UnitCounter;
      if (self.UnitCounter < -1)
        self.UnitCounter = -1;
      if (self.UnitCounter == -1)
        return;
      self.UnitObj = (UnitClass[]) Utils.CopyArray((Array) self.UnitObj, (Array) new UnitClass[self.UnitCounter + 1]);
    }

    pub void ChangeUnitNr(
      Oldnr: i32,
      Newnr: i32,
      ref tgame: GameClass,
      bool skipmap,
      bool DeleteRegimeMode = false)
    {
      if (!Information.IsNothing( tgame))
      {
        if (tgame.EditObj.UnitSelected > -1 && Oldnr == tgame.EditObj.UnitSelected)
          tgame.EditObj.UnitSelected = Newnr;
        if (tgame.EditObj.OrderUnit > -1 && Oldnr == tgame.EditObj.OrderUnit)
          tgame.EditObj.OrderUnit = Newnr;
        if (tgame.EditObj.OldUnit > -1 && Oldnr == tgame.EditObj.OldUnit)
          tgame.EditObj.OldUnit = Newnr;
        if (tgame.EditObj.se1_CardsCategory == 3 | tgame.EditObj.se1_CardsCategory == 5 && tgame.EditObj.se1_CardsTarget > -1 && Oldnr == tgame.EditObj.se1_CardsTarget)
          tgame.EditObj.se1_CardsTarget = Newnr;
      }
      if (self.UnitObj[Oldnr].PreDef > -1 & Newnr == -1)
      {
        let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
        for (let mut index1: i32 =  0; index1 <= historicalUnitCounter; index1 += 1)
        {
          let mut index2: i32 =  0;
          do
          {
            if (self.HistoricalUnitObj[index1].SubParts[index2] == self.UnitObj[Oldnr].PreDef)
              self.HistoricalUnitObj[index1].SubParts[index2] = -1;
            index2 += 1;
          }
          while (index2 <= 9);
        }
      }
      if (!skipmap)
      {
        let mut mapCounter: i32 =  self.MapCounter;
        for (let mut index3: i32 =  0; index3 <= mapCounter; index3 += 1)
        {
          let mut mapWidth: i32 =  self.MapObj[index3].MapWidth;
          for (let mut index4: i32 =  0; index4 <= mapWidth; index4 += 1)
          {
            let mut mapHeight: i32 =  self.MapObj[index3].MapHeight;
            for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
            {
              if (index4 > -1 & index5 > -1 & index3 > -1 && self.MapObj[index3].HexObj[index4, index5].UnitCounter > -1)
              {
                let mut unitCounter: i32 =  self.MapObj[index3].HexObj[index4, index5].UnitCounter;
                for (let mut index6: i32 =  0; index6 <= unitCounter; index6 += 1)
                {
                  if (self.MapObj[index3].HexObj[index4, index5].UnitList[index6] == Oldnr)
                  {
                    if (Newnr == -1)
                    {
                      self.MapObj[index3].HexObj[index4, index5].RemoveUnitFromList(Oldnr);
                      break;
                    }
                    self.MapObj[index3].HexObj[index4, index5].UnitList[index6] = Newnr;
                    break;
                  }
                }
              }
            }
          }
        }
      }
      let mut unitCounter1: i32 =  self.UnitCounter;
      for (let mut index7: i32 =  0; index7 <= unitCounter1; index7 += 1)
      {
        if (self.UnitObj[index7].HQ == Oldnr)
          self.UnitObj[index7].HQ = Newnr;
        if (self.UnitObj[index7].TempCapHQ == Oldnr)
          self.UnitObj[index7].TempCapHQ = Newnr;
        if (self.UnitObj[index7].OnBoard == Oldnr)
          self.UnitObj[index7].OnBoard = Newnr;
        if (self.UnitObj[index7].PassengerCounter > -1)
        {
          let mut passengerCounter: i32 =  self.UnitObj[index7].PassengerCounter;
          for (let mut index8: i32 =  0; index8 <= passengerCounter; index8 += 1)
          {
            if (self.UnitObj[index7].PassengerList[index8] == Oldnr)
            {
              if (Newnr == -1)
                self.UnitObj[index7].RemovePassenger(Oldnr);
              else
                self.UnitObj[index7].PassengerList[index8] = Newnr;
            }
          }
        }
        if (self.UnitObj[index7].attachedTo == Oldnr)
          self.UnitObj[index7].attachedTo = Newnr;
        if (self.UnitObj[index7].TransportCounter > -1)
        {
          let mut transportCounter: i32 =  self.UnitObj[index7].TransportCounter;
          for (let mut index9: i32 =  0; index9 <= transportCounter; index9 += 1)
          {
            if (self.UnitObj[index7].TransportList[index9] == Oldnr)
            {
              if (Newnr == -1)
                self.UnitObj[index7].RemoveTransport(Oldnr);
              else
                self.UnitObj[index7].TransportList[index9] = Newnr;
            }
          }
        }
        if (!Information.IsNothing( self.UnitObj[index7].AIPlanRef) && self.UnitObj[index7].AIPlanRef.HQ == Oldnr)
          self.UnitObj[index7].AIPlanRef.HQ = Newnr;
      }
      if (!DeleteRegimeMode && !Information.IsNothing( tgame))
      {
        if (!Information.IsNothing( tgame.AIObj))
        {
          let mut tplanCount: i32 =  tgame.AIObj.TPlanCount;
          for (let mut index: i32 =  0; index <= tplanCount; index += 1)
          {
            if (!Information.IsNothing( tgame.AIObj.TPlanObj[index]) && tgame.AIObj.TPlanObj[index].HQ == Oldnr)
              tgame.AIObj.TPlanObj[index].HQ = Newnr;
          }
        }
        if (tgame.Data.UseAI == 1)
        {
          let mut index: i32 =  1;
          do
          {
            if (tgame.NewAIObj.MoveMatrixUnit[index] == Oldnr)
              tgame.NewAIObj.MoveMatrixUnit[index] = Newnr;
            if (tgame.NewAIObj.EnemyMatrixUnit[index] == Oldnr)
              tgame.NewAIObj.EnemyMatrixUnit[index] = Newnr;
            index += 1;
          }
          while (index <= 90);
        }
      }
      let mut locCounter: i32 =  self.LocCounter;
      for (let mut index: i32 =  0; index <= locCounter; index += 1)
      {
        if (self.LocObj[index].HQ == Oldnr)
          self.LocObj[index].HQ = Newnr;
      }
    }

    pub fn AddSF(unr: i32) -> i32
    {
      this += 1.SFCounter;
      self.SFObj = (SFClass[]) Utils.CopyArray((Array) self.SFObj, (Array) new SFClass[self.SFCounter + 1]);
      self.SFObj[self.SFCounter] = new SFClass(0);
      self.SFObj[self.SFCounter].LoadSprites();
      self.UnitObj[unr].AddSF(self.SFCounter);
      return self.SFCounter;
    }

    pub fn RemoveSF(nr: i32)
    {
      self.SFObj[nr].Kill();
      self.ChangeSFNr(nr, -1);
      if (nr < self.SFCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.SFCounter - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          self.SFObj[index] = self.SFObj[index + 1];
        self.MassChangeSFNr(nr + 1);
      }
      --self.SFCounter;
      if (self.SFCounter == -1)
        return;
      self.SFObj = (SFClass[]) Utils.CopyArray((Array) self.SFObj, (Array) new SFClass[self.SFCounter + 1]);
    }

    pub fn MassChangeSFNr(FromNr: i32)
    {
      if (self.UnitCounter <= -1)
        return;
      let mut unitCounter: i32 =  self.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (self.UnitObj[index1].SFCount > -1)
        {
          let mut sfCount: i32 =  self.UnitObj[index1].SFCount;
          for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
          {
            if (self.UnitObj[index1].SFList[index2] >= FromNr)
              self.UnitObj[index1].SFList[index2] = self.UnitObj[index1].SFList[index2] - 1;
          }
        }
      }
    }

    pub fn ChangeSFNr(Oldnr: i32, Newnr: i32)
    {
      if (self.UnitCounter <= -1)
        return;
      let mut unitCounter: i32 =  self.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (self.UnitObj[index1].SFCount > -1)
        {
          let mut sfCount: i32 =  self.UnitObj[index1].SFCount;
          for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
          {
            if (self.UnitObj[index1].SFList[index2] == Oldnr)
            {
              if (Newnr == -1)
              {
                self.UnitObj[index1].RemoveSF(Oldnr);
                return;
              }
              self.UnitObj[index1].SFList[index2] = Newnr;
              return;
            }
          }
        }
      }
    }

    pub fn FindSFType(ref SFTypeClass sft, libName: String, bool thisIsHistoricalImport = false) -> i32
    {
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut sfType: i32 =  0; sfType <= sfTypeCounter; sfType += 1)
      {
        if (self.SFTypeObj[sfType].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.SFTypeObj[sfType].LibId.libSlot].name, libName, false) == 0)
        {
          if (thisIsHistoricalImport)
          {
            if (self.SFTypeObj[sfType].LibId.id == sft.LibId.id)
              return sfType;
          }
          else if (self.SFTypeObj[sfType].LibId.id == sft.Id | self.SFTypeObj[sfType].LibId.id == sft.LibId.id)
            return sfType;
        }
      }
      return -1;
    }

    pub fn FindSFType(libName: String, libId: i32) -> i32
    {
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut sfType: i32 =  0; sfType <= sfTypeCounter; sfType += 1)
      {
        if (self.SFTypeObj[sfType].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.SFTypeObj[sfType].LibId.libSlot].name, libName, false) == 0 && self.SFTypeObj[sfType].LibId.id == libId)
          return sfType;
      }
      return -1;
    }

    pub fn FindSFTypeAlien(libName: String, libId: i32, buildGround: i32) -> i32
    {
      let mut num: i32 =  -1;
      SimpleList simpleList = SimpleList::new();
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut tid: i32 =  0; tid <= sfTypeCounter; tid += 1)
      {
        if (self.SFTypeObj[tid].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.SFTypeObj[tid].LibId.libSlot].name, libName, false) == 0 && self.SFTypeObj[tid].LibId.id == libId)
        {
          num = tid;
          if (self.SFTypeObj[tid].SFTypeVar[7] == buildGround | buildGround < 1)
            simpleList.AddWeight(tid, 10000);
          else
            simpleList.AddWeight(tid, 10);
        }
      }
      if (simpleList.Counter > -1)
        return simpleList.GetRandomIdbasedOnWeight();
      return num > -1 ? num : -1;
    }

    pub fn AddSFType(bool temporary = false)
    {
      this += 1.SFTypeCounter;
      self.SFTypeObj = (SFTypeClass[]) Utils.CopyArray((Array) self.SFTypeObj, (Array) new SFTypeClass[self.SFTypeCounter + 1]);
      self.SFTypeObj[self.SFTypeCounter] = new SFTypeClass(0, self.LandscapeTypeCounter, self.ResearchCounter);
      self.SFTypeObj[self.SFTypeCounter].LoadSprites();
      if (self.Round > 0 & !temporary)
      {
        object[,] objArray1 = new object[self.SFTypeCounter + 1, self.Round + 1 + 1];
        object[,] objArray2 = new object[self.SFTypeCounter + 1, self.Round + 1 + 1];
        object[,] objArray3 = new object[self.SFTypeCounter + 1, self.Round + 1 + 1];
        object[] objArray4 = new object[self.SFTypeCounter + 1];
        let mut regimeCounter: i32 =  self.RegimeCounter;
        for (let mut index1: i32 =  0; index1 <= regimeCounter; index1 += 1)
        {
          let mut upperBound: i32 =  self.RegimeObj[index1].SKills.GetUpperBound(0);
          for (let mut index2: i32 =  0; index2 <= upperBound; index2 += 1)
          {
            let mut num: i32 =  self.Round + 1;
            for (let mut index3: i32 =  0; index3 <= num; index3 += 1)
            {
              objArray1[index2, index3] =  self.RegimeObj[index1].SKills[index2, index3];
              objArray2[index2, index3] =  self.RegimeObj[index1].SLoss[index2, index3];
              objArray3[index2, index3] =  self.RegimeObj[index1].SPresent[index2, index3];
            }
            objArray4[index2] =  self.RegimeObj[index1].SASKilled[index2];
          }
          self.RegimeObj[index1].SKills = new int[self.SFTypeCounter + 1, self.Round + 1 + 1];
          self.RegimeObj[index1].SLoss = new int[self.SFTypeCounter + 1, self.Round + 1 + 1];
          self.RegimeObj[index1].SPresent = new int[self.SFTypeCounter + 1, self.Round + 1 + 1];
          self.RegimeObj[index1].SASKilled = new int[self.SFTypeCounter + 1];
          let mut num1: i32 =  self.SFTypeCounter - 1;
          for (let mut index4: i32 =  0; index4 <= num1; index4 += 1)
          {
            let mut num2: i32 =  self.Round + 1;
            for (let mut index5: i32 =  0; index5 <= num2; index5 += 1)
            {
              self.RegimeObj[index1].SKills[index4, index5] = Conversions.ToInteger(objArray1[index4, index5]);
              self.RegimeObj[index1].SLoss[index4, index5] = Conversions.ToInteger(objArray2[index4, index5]);
              self.RegimeObj[index1].SPresent[index4, index5] = Conversions.ToInteger(objArray3[index4, index5]);
            }
            self.RegimeObj[index1].SASKilled[index4] = Conversions.ToInteger(objArray4[index4]);
          }
        }
      }
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
      {
        if (self.SFTypeObj[index].Id >= self.SFTypeIdCounter)
          self.SFTypeIdCounter = self.SFTypeObj[index].Id;
      }
      this += 1.SFTypeIdCounter;
      self.SFTypeObj[self.SFTypeCounter].Id = self.SFTypeIdCounter;
    }

    pub fn RemoveSFType(nr: i32)
    {
      self.SFTypeObj[nr].Kill();
      self.ChangeSFTypeNr(nr, -1);
      if (nr < self.SFTypeCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.SFTypeCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.SFTypeObj[Newnr] = self.SFTypeObj[Newnr + 1];
          self.ChangeSFTypeNr(Newnr + 1, Newnr);
        }
      }
      --self.SFTypeCounter;
      if (self.SFTypeCounter == -1)
        return;
      self.SFTypeObj = (SFTypeClass[]) Utils.CopyArray((Array) self.SFTypeObj, (Array) new SFTypeClass[self.SFTypeCounter + 1]);
    }

    pub fn ChangeSFTypeNr(Oldnr: i32, Newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      nr: i32;
      for (let mut index1: i32 =  0; index1 <= stringListCounter; index1 += 1)
      {
        let mut width: i32 =  self.StringListObj[index1].Width;
        for (let mut index2: i32 =  0; index2 <= width; index2 += 1)
        {
          let mut length: i32 =  self.StringListObj[index1].Length;
          for (let mut index3: i32 =  0; index3 <= length; index3 += 1)
          {
            if (self.StringListObj[index1].ColValueType[index2] == NewEnums.LibVarValueType.SFTypeId && self.StringListObj[index1].Data[index3, index2].Length > 0)
            {
              nr =  Math.Round(Conversion.Val(self.StringListObj[index1].Data[index3, index2]));
              if (nr == Oldnr)
                nr = Newnr;
              self.StringListObj[index1].Data[index3, index2] = nr.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.SFtype & self.LibVarObj[libVarCounter].instanceId.id == self.SFTypeObj[Oldnr].LibId.id & self.LibVarObj[libVarCounter].instanceId.libSlot == self.SFTypeObj[Oldnr].LibId.libSlot)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].instanceId.id = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.SFTypeId & self.LibVarObj[libVarCounter].value == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = Newnr;
        }
      }
      for (nr = self.SFCounter; nr >= 0; nr += -1)
      {
        if (self.SFObj[nr].Type == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveSF(nr);
          else
            self.SFObj[nr].Type = Newnr;
        }
      }
      for (nr = self.ResearchCounter; nr >= 0; nr += -1)
      {
        if (self.ResearchObj[nr].SFTypePic == Oldnr)
          self.ResearchObj[nr].SFTypePic = Newnr;
      }
      for (nr = self.ItemTypeCounter; nr >= 0; nr += -1)
      {
        if (self.ItemTypeObj[nr].IsSFType == Oldnr)
          self.ItemTypeObj[nr].IsSFType = Newnr;
      }
      for (nr = self.SFTypeCounter; nr >= 0; nr += -1)
      {
        if (self.SFTypeObj[nr].CopyDataFrom == Oldnr)
          self.SFTypeObj[nr].CopyDataFrom = Newnr;
      }
      for (nr = self.SFTypeCounter; nr >= 0; nr += -1)
      {
        if (self.SFTypeObj[nr].UpgradeToo == Oldnr)
          self.SFTypeObj[nr].UpgradeToo = Newnr;
        if (self.SFTypeObj[nr].ArtSFType == Oldnr)
          self.SFTypeObj[nr].ArtSFType = Newnr;
      }
    }

    pub fn ThreadBlock()
    {
      if (!DrawMod.TGame.se1ThreadBlock)
        return;
      do
        ;
      while (DrawMod.TGame.se1ThreadBlock);
      DrawMod.TGame.se1ThreadBlock = true;
    }

    pub fn ThreadRelease() => DrawMod.TGame.se1ThreadBlock = false;

    pub fn AddLandscapeType()
    {
      this += 1.LandscapeTypeCounter;
      self.LandscapeTypeObj = (LandscapeTypeClass[]) Utils.CopyArray((Array) self.LandscapeTypeObj, (Array) new LandscapeTypeClass[self.LandscapeTypeCounter + 1]);
      self.LandscapeTypeObj[self.LandscapeTypeCounter] = new LandscapeTypeClass(0);
      self.LandscapeTypeObj[self.LandscapeTypeCounter].LoadSprites();
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
      {
        self.SFTypeObj[index].CombatModAtt = (float[]) Utils.CopyArray((Array) self.SFTypeObj[index].CombatModAtt, (Array) new float[self.LandscapeTypeCounter + 1]);
        self.SFTypeObj[index].CombatModDef = (float[]) Utils.CopyArray((Array) self.SFTypeObj[index].CombatModDef, (Array) new float[self.LandscapeTypeCounter + 1]);
        self.SFTypeObj[index].ExtraRecon = (int[]) Utils.CopyArray((Array) self.SFTypeObj[index].ExtraRecon, (Array) new int[self.LandscapeTypeCounter + 1]);
        self.SFTypeObj[index].CombatModAtt[self.LandscapeTypeCounter] = 1f;
        self.SFTypeObj[index].CombatModDef[self.LandscapeTypeCounter] = 1f;
        self.SFTypeObj[index].ExtraRecon[self.LandscapeTypeCounter] = 0;
      }
    }

    pub fn RemoveLandscapeType(nr: i32)
    {
      self.LandscapeTypeObj[nr].Kill();
      self.ChangeLandscapeNr(nr, -1);
      if (nr < self.LandscapeTypeCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.LandscapeTypeCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.LandscapeTypeObj[Newnr] = self.LandscapeTypeObj[Newnr + 1];
          self.ChangeLandscapeNr(Newnr + 1, Newnr);
        }
      }
      let mut sfTypeCounter1: i32 =  self.SFTypeCounter;
      for (let mut index1: i32 =  0; index1 <= sfTypeCounter1; index1 += 1)
      {
        if (nr < self.LandscapeTypeCounter)
        {
          let mut num3: i32 =  nr;
          let mut num4: i32 =  self.LandscapeTypeCounter - 1;
          for (let mut index2: i32 =  num3; index2 <= num4; index2 += 1)
          {
            self.SFTypeObj[index1].CombatModAtt[index2] = self.SFTypeObj[index1].CombatModAtt[index2 + 1];
            self.SFTypeObj[index1].CombatModDef[index2] = self.SFTypeObj[index1].CombatModDef[index2 + 1];
            self.SFTypeObj[index1].ExtraRecon[index2] = self.SFTypeObj[index1].ExtraRecon[index2 + 1];
          }
        }
      }
      --self.LandscapeTypeCounter;
      let mut sfTypeCounter2: i32 =  self.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter2; index += 1)
      {
        if (self.LandscapeTypeCounter > -1)
        {
          self.SFTypeObj[index].CombatModAtt = (float[]) Utils.CopyArray((Array) self.SFTypeObj[index].CombatModAtt, (Array) new float[self.LandscapeTypeCounter + 1]);
          self.SFTypeObj[index].CombatModDef = (float[]) Utils.CopyArray((Array) self.SFTypeObj[index].CombatModDef, (Array) new float[self.LandscapeTypeCounter + 1]);
          self.SFTypeObj[index].ExtraRecon = (int[]) Utils.CopyArray((Array) self.SFTypeObj[index].ExtraRecon, (Array) new int[self.LandscapeTypeCounter + 1]);
        }
      }
      if (self.LandscapeTypeCounter != -1)
        self.LandscapeTypeObj = (LandscapeTypeClass[]) Utils.CopyArray((Array) self.LandscapeTypeObj, (Array) new LandscapeTypeClass[self.LandscapeTypeCounter + 1]);
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index3: i32 =  0; index3 <= mapCounter; index3 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index3].MapWidth;
        for (let mut index4: i32 =  0; index4 <= mapWidth; index4 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index3].MapHeight;
          for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
          {
            if (self.MapObj[index3].HexObj[index4, index5].LandscapeType == -1)
              self.MapObj[index3].HexObj[index4, index5].LandscapeType = 0;
          }
        }
      }
    }

    pub fn ChangeLandscapeNr(Oldnr: i32, Newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      index1: i32;
      for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
      {
        let mut width: i32 =  self.StringListObj[index2].Width;
        for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
        {
          let mut length: i32 =  self.StringListObj[index2].Length;
          for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
          {
            if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.LandscapeId && self.StringListObj[index2].Data[index4, index3].Length > 0)
            {
              index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
              if (index1 == Oldnr)
                index1 = Newnr;
              self.StringListObj[index2].Data[index4, index3] = index1.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.Landscape & self.LibVarObj[libVarCounter].instanceId.id == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].instanceId.id = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.LandscapeId & self.LibVarObj[libVarCounter].value == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = Newnr;
        }
      }
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index5: i32 =  0; index5 <= mapCounter; index5 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index5].MapWidth;
        for (let mut index6: i32 =  0; index6 <= mapWidth; index6 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index5].MapHeight;
          for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
          {
            if (self.MapObj[index5].HexObj[index6, index7].LandscapeType == Oldnr)
            {
              self.MapObj[index5].HexObj[index6, index7].LandscapeType = Newnr;
              if (Newnr == -1)
                self.MapObj[index5].HexObj[index6, index7].SpriteNr = 0;
            }
            if (self.MapObj[index5].HexObj[index6, index7].SpecialType == Oldnr)
            {
              self.MapObj[index5].HexObj[index6, index7].SpecialType = Newnr;
              if (Newnr == -1)
                self.MapObj[index5].HexObj[index6, index7].SpecialSprite = 0;
            }
          }
        }
      }
      let mut locTypeCounter: i32 =  self.LocTypeCounter;
      for (index1 = 0; index1 <= locTypeCounter; index1 += 1)
      {
        if (self.LocTypeObj[index1].OverdrawLTNr == Oldnr)
        {
          self.LocTypeObj[index1].OverdrawLTNr = Newnr;
          if (Newnr == -1)
            self.LocTypeObj[index1].OverdrawSpriteNr = 0;
        }
      }
      let mut landscapeTypeCounter1: i32 =  self.LandscapeTypeCounter;
      for (index1 = 0; index1 <= landscapeTypeCounter1; index1 += 1)
      {
        for (let mut overridesCount: i32 =  self.LandscapeTypeObj[index1].OverridesCount; overridesCount >= 0; overridesCount += -1)
        {
          if (self.LandscapeTypeObj[index1].OverridesType[overridesCount] == Oldnr)
          {
            if (Newnr == -1)
              self.LandscapeTypeObj[index1].RemoveOverride(overridesCount);
            else
              self.LandscapeTypeObj[index1].OverridesType[overridesCount] = Newnr;
          }
        }
        for (let mut overridesCount2: i32 =  self.LandscapeTypeObj[index1].OverridesCount2; overridesCount2 >= 0; overridesCount2 += -1)
        {
          if (self.LandscapeTypeObj[index1].OverridesType2[overridesCount2] == Oldnr)
          {
            if (Newnr == -1)
              self.LandscapeTypeObj[index1].RemoveOverride2(overridesCount2);
            else
              self.LandscapeTypeObj[index1].OverridesType2[overridesCount2] = Newnr;
          }
        }
      }
      let mut landscapeTypeCounter2: i32 =  self.LandscapeTypeCounter;
      for (index1 = 0; index1 <= landscapeTypeCounter2; index1 += 1)
      {
        if (self.LandscapeTypeObj[index1].ExtraExterior == Oldnr)
          self.LandscapeTypeObj[index1].ExtraExterior = Newnr;
        if (self.LandscapeTypeObj[index1].OverridesCount > -1)
        {
          for (let mut overridesCount: i32 =  self.LandscapeTypeObj[index1].OverridesCount; overridesCount >= 0; overridesCount += -1)
          {
            if (self.LandscapeTypeObj[index1].OverridesType[overridesCount] == Oldnr)
            {
              if (Newnr == -1)
                self.LandscapeTypeObj[index1].RemoveOverride(overridesCount);
              else
                self.LandscapeTypeObj[index1].OverridesType[overridesCount] = Newnr;
            }
          }
        }
      }
    }

    pub fn RemoveLandscapeBasicSprite(ltnr: i32, nr: i32)
    {
      self.LandscapeTypeObj[ltnr].BasicSpriteFileName[nr] = "";
      self.LandscapeTypeObj[ltnr].BasicSpriteFileName2[nr] = "";
      self.LandscapeTypeObj[ltnr].BasicPicFileName[nr] = "";
      BitmapStore.RemoveBitmapNr(self.LandscapeTypeObj[ltnr].BasicSpriteID[nr]);
      BitmapStore.RemoveBitmapNr(self.LandscapeTypeObj[ltnr].BasicSpriteID2[nr]);
      BitmapStore.RemoveBitmapNr(self.LandscapeTypeObj[ltnr].BasicPicID[nr]);
      self.LandscapeTypeObj[ltnr].BasicSpriteID[nr] = -1;
      self.LandscapeTypeObj[ltnr].BasicPicID[nr] = -1;
      self.LandscapeTypeObj[ltnr].PlotLast[nr] = false;
      self.ChangeLandscapeSpriteNr(ltnr, nr, -1);
      if (nr < self.LandscapeTypeObj[ltnr].BasicSpriteCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.LandscapeTypeObj[ltnr].BasicSpriteCounter - 1;
        for (let mut NewNr: i32 =  num1; NewNr <= num2; NewNr += 1)
        {
          self.LandscapeTypeObj[ltnr].BasicSpriteFileName[NewNr] = self.LandscapeTypeObj[ltnr].BasicSpriteFileName[NewNr + 1];
          self.LandscapeTypeObj[ltnr].BasicSpriteFileName2[NewNr] = self.LandscapeTypeObj[ltnr].BasicSpriteFileName2[NewNr + 1];
          self.LandscapeTypeObj[ltnr].BasicSpriteID[NewNr] = self.LandscapeTypeObj[ltnr].BasicSpriteID[NewNr + 1];
          self.LandscapeTypeObj[ltnr].BasicSpriteID2[NewNr] = self.LandscapeTypeObj[ltnr].BasicSpriteID2[NewNr + 1];
          self.LandscapeTypeObj[ltnr].BasicPicFileName[NewNr] = self.LandscapeTypeObj[ltnr].BasicPicFileName[NewNr + 1];
          self.LandscapeTypeObj[ltnr].BasicPicID[NewNr] = self.LandscapeTypeObj[ltnr].BasicPicID[NewNr + 1];
          self.LandscapeTypeObj[ltnr].PlotLast[NewNr] = self.LandscapeTypeObj[ltnr].PlotLast[NewNr + 1];
          self.ChangeLandscapeSpriteNr(ltnr, NewNr + 1, NewNr);
        }
      }
      --self.LandscapeTypeObj[ltnr].BasicSpriteCounter;
      self.LandscapeTypeObj[ltnr].BasicSpriteFileName = (string[]) Utils.CopyArray((Array) self.LandscapeTypeObj[ltnr].BasicSpriteFileName, (Array) new string[self.LandscapeTypeObj[ltnr].BasicSpriteCounter + 1]);
      self.LandscapeTypeObj[ltnr].BasicSpriteFileName2 = (string[]) Utils.CopyArray((Array) self.LandscapeTypeObj[ltnr].BasicSpriteFileName2, (Array) new string[self.LandscapeTypeObj[ltnr].BasicSpriteCounter + 1]);
      self.LandscapeTypeObj[ltnr].BasicSpriteID = (int[]) Utils.CopyArray((Array) self.LandscapeTypeObj[ltnr].BasicSpriteID, (Array) new int[self.LandscapeTypeObj[ltnr].BasicSpriteCounter + 1]);
      self.LandscapeTypeObj[ltnr].BasicSpriteID2 = (int[]) Utils.CopyArray((Array) self.LandscapeTypeObj[ltnr].BasicSpriteID2, (Array) new int[self.LandscapeTypeObj[ltnr].BasicSpriteCounter + 1]);
      self.LandscapeTypeObj[ltnr].BasicPicFileName = (string[]) Utils.CopyArray((Array) self.LandscapeTypeObj[ltnr].BasicPicFileName, (Array) new string[self.LandscapeTypeObj[ltnr].BasicSpriteCounter + 1]);
      self.LandscapeTypeObj[ltnr].BasicPicID = (int[]) Utils.CopyArray((Array) self.LandscapeTypeObj[ltnr].BasicPicID, (Array) new int[self.LandscapeTypeObj[ltnr].BasicSpriteCounter + 1]);
      self.LandscapeTypeObj[ltnr].PlotLast = (bool[]) Utils.CopyArray((Array) self.LandscapeTypeObj[ltnr].PlotLast, (Array) new bool[self.LandscapeTypeObj[ltnr].BasicSpriteCounter + 1]);
    }

    pub fn ChangeLandscapeSpriteNr(LTnr: i32, OldNr: i32, NewNr: i32)
    {
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            if (self.MapObj[index1].HexObj[index2, index3].LandscapeType == LTnr && self.MapObj[index1].HexObj[index2, index3].SpriteNr == OldNr)
              self.MapObj[index1].HexObj[index2, index3].SpriteNr = NewNr != -1 ? NewNr : 0;
            if (self.MapObj[index1].HexObj[index2, index3].SpecialType == LTnr && self.MapObj[index1].HexObj[index2, index3].SpecialSprite == OldNr)
              self.MapObj[index1].HexObj[index2, index3].SpecialSprite = NewNr != -1 ? NewNr : 0;
          }
        }
      }
      let mut locTypeCounter: i32 =  self.LocTypeCounter;
      for (let mut index: i32 =  0; index <= locTypeCounter; index += 1)
      {
        if (self.LocTypeObj[index].OverdrawLTNr == LTnr & self.LocTypeObj[index].OverdrawSpriteNr == OldNr)
          self.LocTypeObj[index].OverdrawSpriteNr = NewNr != -1 ? NewNr : 0;
      }
    }

    pub fn FindLibrary(libName: String) -> i32
    {
      let mut libraryCounter: i32 =  self.LibraryCounter;
      for (let mut library: i32 =  0; library <= libraryCounter; library += 1)
      {
        if (Operators.CompareString(self.LibraryObj[library].name, libName, false) == 0)
          return library;
      }
      return -1;
    }

    pub fn FindLibraryLowCase(libName: String) -> i32
    {
      libName = Strings.LCase(libName);
      let mut libraryCounter: i32 =  self.LibraryCounter;
      for (let mut libraryLowCase: i32 =  0; libraryLowCase <= libraryCounter; libraryLowCase += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.LibraryObj[libraryLowCase].name), libName, false) == 0)
          return libraryLowCase;
      }
      return -1;
    }

    pub fn AddLibrary()
    {
      this += 1.LibraryCounter;
      self.LibraryObj = (LibraryClass[]) Utils.CopyArray((Array) self.LibraryObj, (Array) new LibraryClass[self.LibraryCounter + 1]);
      self.LibraryObj[self.LibraryCounter] = LibraryClass::new();
    }

    pub fn RemoveLibrary(nr: i32)
    {
      self.ChangeLibraryNr(nr, -1);
      if (nr < self.LibraryCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.LibraryCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.LibraryObj[Newnr] = self.LibraryObj[Newnr + 1];
          self.ChangeLibraryNr(Newnr + 1, Newnr);
        }
      }
      --self.LibraryCounter;
      if (self.LibraryCounter == -1)
        return;
      self.LibraryObj = (LibraryClass[]) Utils.CopyArray((Array) self.LibraryObj, (Array) new LibraryClass[self.LibraryCounter + 1]);
    }

    pub fn ChangeLibraryNr(Oldnr: i32, Newnr: i32)
    {
      for (let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
      {
        if (self.HistoricalUnitObj[historicalUnitCounter].LibId.libSlot == Oldnr && Newnr == -1 && self.HistoricalUnitObj[historicalUnitCounter].OffLibId.libSlot > -1 & self.HistoricalUnitObj[historicalUnitCounter].OffLibId.libSlot != Oldnr)
        {
          self.AddHistoricalUnit();
          DrawMod.TGame.ProcessingObj.SwapOfficer(self.HistoricalUnitObj[historicalUnitCounter].TempRegime, historicalUnitCounter, self.HistoricalUnitCounter, -1);
          self.HistoricalUnitObj[self.HistoricalUnitCounter].LibId = self.HistoricalUnitObj[historicalUnitCounter].OffLibId.Clone();
        }
      }
      for (let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
      {
        if (self.HistoricalUnitObj[historicalUnitCounter].ModelMaster > -1 && self.HistoricalUnitObj[self.HistoricalUnitObj[historicalUnitCounter].ModelMaster].LibId.libSlot == Oldnr && Newnr == -1 && self.HistoricalUnitObj[historicalUnitCounter].OffLibId.libSlot > -1 & self.HistoricalUnitObj[historicalUnitCounter].OffLibId.libSlot != Oldnr)
        {
          self.AddHistoricalUnit();
          DrawMod.TGame.ProcessingObj.SwapOfficer(self.HistoricalUnitObj[historicalUnitCounter].TempRegime, historicalUnitCounter, self.HistoricalUnitCounter, -1);
          self.HistoricalUnitObj[self.HistoricalUnitCounter].LibId = self.HistoricalUnitObj[historicalUnitCounter].OffLibId.Clone();
        }
      }
      for (let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
      {
        if (self.HistoricalUnitObj[historicalUnitCounter].LibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveHistoricalUnit(historicalUnitCounter);
          else
            self.HistoricalUnitObj[historicalUnitCounter].LibId.libSlot = Newnr;
        }
      }
      for (let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
      {
        if (self.HistoricalUnitObj[historicalUnitCounter].OffLibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
          {
            DrawMod.TGame.Data.AddHistoricalUnit();
            DrawMod.TGame.ProcessingObj.SwapOfficer(self.HistoricalUnitObj[historicalUnitCounter].TempRegime, historicalUnitCounter, DrawMod.TGame.Data.HistoricalUnitCounter, -1);
            DrawMod.TGame.Data.RemoveHistoricalUnit(DrawMod.TGame.Data.HistoricalUnitCounter);
          }
          else
            self.HistoricalUnitObj[historicalUnitCounter].OffLibId.libSlot = Newnr;
        }
      }
      let mut mapWidth: i32 =  self.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          for (let mut hexLibVarCounter: i32 =  self.MapObj[0].HexObj[index1, index2].HexLibVarCounter; hexLibVarCounter >= 0; hexLibVarCounter += -1)
          {
            if (self.MapObj[0].HexObj[index1, index2].HexLibVarSlotNr[hexLibVarCounter] == Oldnr)
            {
              if (Newnr == -1)
                self.MapObj[0].HexObj[index1, index2].RemoveHexLibVar(hexLibVarCounter);
              else
                self.MapObj[0].HexObj[index1, index2].HexLibVarSlotNr[hexLibVarCounter] = Newnr;
            }
          }
        }
      }
      for (let mut sfTypeCounter: i32 =  self.SFTypeCounter; sfTypeCounter >= 0; sfTypeCounter += -1)
      {
        if (self.SFTypeObj[sfTypeCounter].LibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveSFType(sfTypeCounter);
          else
            self.SFTypeObj[sfTypeCounter].LibId.libSlot = Newnr;
        }
      }
      for (let mut peopleCounter: i32 =  self.PeopleCounter; peopleCounter >= 0; peopleCounter += -1)
      {
        if (self.PeopleObj[peopleCounter].LibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemovePeople(peopleCounter);
          else
            self.PeopleObj[peopleCounter].LibId.libSlot = Newnr;
        }
      }
      for (let mut regimeCounter: i32 =  self.RegimeCounter; regimeCounter >= 0; regimeCounter += -1)
      {
        if (self.RegimeObj[regimeCounter].libId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveRegime(regimeCounter);
          else
            self.RegimeObj[regimeCounter].libId.libSlot = Newnr;
        }
      }
      for (let mut eventCounter: i32 =  self.EventCounter; eventCounter >= 0; eventCounter += -1)
      {
        if (self.EventObj[eventCounter].LibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveEvent(eventCounter);
          else
            self.EventObj[eventCounter].LibId.libSlot = Newnr;
        }
      }
      for (let mut stringListCounter: i32 =  self.StringListCounter; stringListCounter >= 0; stringListCounter += -1)
      {
        if (self.StringListObj[stringListCounter].LibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveStringList(stringListCounter);
          else
            self.StringListObj[stringListCounter].LibId.libSlot = Newnr;
        }
      }
      for (let mut eventPicCounter: i32 =  self.EventPicCounter; eventPicCounter >= 0; eventPicCounter += -1)
      {
        if (self.eventPicLibId[eventPicCounter].libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveEventPic(eventPicCounter);
          else
            self.eventPicLibId[eventPicCounter].libSlot = Newnr;
        }
      }
      for (let mut smallPicCounter: i32 =  self.SmallPicCounter; smallPicCounter >= 0; smallPicCounter += -1)
      {
        if (self.SmallLibId[smallPicCounter].libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveSmallPic(smallPicCounter);
          else
            self.SmallLibId[smallPicCounter].libSlot = Newnr;
        }
      }
      for (let mut reinfCounter: i32 =  self.ReinfCounter; reinfCounter >= 0; reinfCounter += -1)
      {
        if (self.ReinfLibId[reinfCounter].libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveReinf(reinfCounter);
          else
            self.ReinfLibId[reinfCounter].libSlot = Newnr;
        }
      }
      for (let mut actionCardCounter: i32 =  self.ActionCardCounter; actionCardCounter >= 0; actionCardCounter += -1)
      {
        if (self.ActionCardObj[actionCardCounter].LibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveActionCard(actionCardCounter);
          else
            self.ActionCardObj[actionCardCounter].LibId.libSlot = Newnr;
        }
      }
      for (let mut unitCounter: i32 =  self.UnitCounter; unitCounter >= 0; unitCounter += -1)
      {
        if (self.UnitObj[unitCounter].PreDef > -1 && self.UnitObj[unitCounter].LibId.libSlot == Oldnr)
        {
          if (Newnr == -1)
          {
            let mut nr: i32 =  unitCounter;
            let mut gameClass: GameClass = (GameClass) null;
            ref let mut local: GameClass = ref gameClass;
            self.RemoveUnit(nr, ref local);
          }
          else
            self.UnitObj[unitCounter].LibId.libSlot = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].libId.libSlot == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].libId.libSlot = Newnr;
        }
      }
    }

    pub fn FindLibVar(ref LibVarClass tlv, libName: String) -> i32
    {
      libName = Strings.LCase(libName);
      let mut libVarCounter: i32 =  self.LibVarCounter;
      for (let mut libVar: i32 =  0; libVar <= libVarCounter; libVar += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.LibraryObj[self.LibVarObj[libVar].libId.libSlot].name), libName, false) == 0 && Operators.CompareString(Strings.LCase(self.LibVarObj[libVar].name), Strings.LCase(tlv.name), false) == 0)
          return libVar;
      }
      return -1;
    }

    pub fn FindLibVar(ref libvarname: String, libName: String) -> i32
    {
      libvarname = Strings.LCase(libvarname);
      libName = Strings.LCase(libName);
      let mut libVarCounter: i32 =  self.LibVarCounter;
      for (let mut libVar: i32 =  0; libVar <= libVarCounter; libVar += 1)
      {
        if (Operators.CompareString(Strings.LCase(self.LibraryObj[self.LibVarObj[libVar].libId.libSlot].name), libName, false) == 0 | Operators.CompareString(libName, "", false) == 0 && Operators.CompareString(Strings.LCase(self.LibVarObj[libVar].name), libvarname, false) == 0)
          return libVar;
      }
      return -1;
    }

    pub fn AddLibVar(libSlot: i32)
    {
      this += 1.LibVarCounter;
      self.LibVarObj = (LibVarClass[]) Utils.CopyArray((Array) self.LibVarObj, (Array) new LibVarClass[self.LibVarCounter + 1]);
      self.LibVarObj[self.LibVarCounter] = new LibVarClass(libSlot);
    }

    pub fn RemoveLibVar(nr: i32)
    {
      self.ChangeLibVarNr(nr, -1);
      if (nr < self.LibVarCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.LibVarCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.LibVarObj[Newnr] = self.LibVarObj[Newnr + 1];
          self.ChangeLibVarNr(Newnr + 1, Newnr);
        }
      }
      --self.LibVarCounter;
      if (self.LibVarCounter == -1)
        return;
      self.LibVarObj = (LibVarClass[]) Utils.CopyArray((Array) self.LibVarObj, (Array) new LibVarClass[self.LibVarCounter + 1]);
    }

    pub fn ChangeLibVarNr(Oldnr: i32, Newnr: i32)
    {
      let mut mapWidth: i32 =  self.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          for (let mut hexLibVarCounter: i32 =  self.MapObj[0].HexObj[index1, index2].HexLibVarCounter; hexLibVarCounter >= 0; hexLibVarCounter += -1)
          {
            if (self.MapObj[0].HexObj[index1, index2].HexLibVarSlotNr[hexLibVarCounter] == Oldnr)
            {
              if (Newnr == -1)
                self.MapObj[0].HexObj[index1, index2].RemoveHexLibVar(hexLibVarCounter);
              else
                self.MapObj[0].HexObj[index1, index2].HexLibVarSlotNr[hexLibVarCounter] = Newnr;
            }
          }
        }
      }
    }

    pub fn AddRoadType()
    {
      this += 1.RoadTypeCounter;
      self.RoadTypeObj = (RoadTypeClass[]) Utils.CopyArray((Array) self.RoadTypeObj, (Array) new RoadTypeClass[self.RoadTypeCounter + 1]);
      self.RoadTypeObj[self.RoadTypeCounter] = new RoadTypeClass(0);
      self.RoadTypeObj[self.RoadTypeCounter].LoadSprites();
    }

    pub fn RemoveRoadType(nr: i32)
    {
      self.RoadTypeObj[nr].Kill();
      self.ChangeRoadNr(nr, -1);
      if (nr < self.RoadTypeCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.RoadTypeCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.RoadTypeObj[Newnr] = self.RoadTypeObj[Newnr + 1];
          self.ChangeRoadNr(Newnr + 1, Newnr);
        }
      }
      --self.RoadTypeCounter;
      if (self.RoadTypeCounter == -1)
        return;
      self.RoadTypeObj = (RoadTypeClass[]) Utils.CopyArray((Array) self.RoadTypeObj, (Array) new RoadTypeClass[self.RoadTypeCounter + 1]);
    }

    pub fn ChangeRoadNr(Oldnr: i32, Newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      index1: i32;
      for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
      {
        let mut width: i32 =  self.StringListObj[index2].Width;
        for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
        {
          let mut length: i32 =  self.StringListObj[index2].Length;
          for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
          {
            if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.RoadId && self.StringListObj[index2].Data[index4, index3].Length > 0)
            {
              index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
              if (index1 == Oldnr)
                index1 = Newnr;
              self.StringListObj[index2].Data[index4, index3] = index1.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.Road & self.LibVarObj[libVarCounter].instanceId.id == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].instanceId.id = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.RoadId & self.LibVarObj[libVarCounter].value == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = Newnr;
        }
      }
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index5: i32 =  0; index5 <= mapCounter; index5 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index5].MapWidth;
        for (let mut index6: i32 =  0; index6 <= mapWidth; index6 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index5].MapHeight;
          for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
          {
            index1 = 0;
            do
            {
              if (self.MapObj[index5].HexObj[index6, index7].RoadType[index1] == Oldnr)
                self.MapObj[index5].HexObj[index6, index7].RoadType[index1] = Newnr;
              index1 += 1;
            }
            while (index1 <= 5);
          }
        }
      }
    }

    pub fn AddRiverType()
    {
      this += 1.RiverTypeCounter;
      self.RiverTypeObj = (RiverTypeClass[]) Utils.CopyArray((Array) self.RiverTypeObj, (Array) new RiverTypeClass[self.RiverTypeCounter + 1]);
      self.RiverTypeObj[self.RiverTypeCounter] = new RiverTypeClass(0);
      self.RiverTypeObj[self.RiverTypeCounter].LoadSprites();
    }

    pub fn RemoveRiverType(nr: i32)
    {
      self.RiverTypeObj[nr].Kill();
      self.ChangeRiverNr(nr, -1);
      if (nr < self.RiverTypeCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.RiverTypeCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.RiverTypeObj[Newnr] = self.RiverTypeObj[Newnr + 1];
          self.ChangeRiverNr(Newnr + 1, Newnr);
        }
      }
      --self.RiverTypeCounter;
      if (self.RiverTypeCounter == -1)
        return;
      self.RiverTypeObj = (RiverTypeClass[]) Utils.CopyArray((Array) self.RiverTypeObj, (Array) new RiverTypeClass[self.RiverTypeCounter + 1]);
    }

    pub fn ChangeRiverNr(Oldnr: i32, Newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      index1: i32;
      for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
      {
        let mut width: i32 =  self.StringListObj[index2].Width;
        for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
        {
          let mut length: i32 =  self.StringListObj[index2].Length;
          for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
          {
            if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.RiverId && self.StringListObj[index2].Data[index4, index3].Length > 0)
            {
              index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
              if (index1 == Oldnr)
                index1 = Newnr;
              self.StringListObj[index2].Data[index4, index3] = index1.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.River & self.LibVarObj[libVarCounter].instanceId.id == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].instanceId.id = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.RiverId & self.LibVarObj[libVarCounter].value == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = Newnr;
        }
      }
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index5: i32 =  0; index5 <= mapCounter; index5 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index5].MapWidth;
        for (let mut index6: i32 =  0; index6 <= mapWidth; index6 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index5].MapHeight;
          for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
          {
            index1 = 0;
            do
            {
              if (self.MapObj[index5].HexObj[index6, index7].RiverType[index1] == Oldnr)
                self.MapObj[index5].HexObj[index6, index7].RiverType[index1] = Newnr;
              index1 += 1;
            }
            while (index1 <= 5);
          }
        }
      }
    }

    pub fn AddArea()
    {
      this += 1.AreaCounter;
      self.AreaObj = (AreaClass[]) Utils.CopyArray((Array) self.AreaObj, (Array) new AreaClass[self.AreaCounter + 1]);
      self.AreaObj[self.AreaCounter] = AreaClass::new();
      this += 1.AreaIDCounter;
      self.AreaObj[self.AreaCounter].ID = self.AreaIDCounter;
    }

    pub fn RemoveArea(nr: i32)
    {
      if (nr < self.AreaCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.AreaCounter - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          self.AreaObj[index] = self.AreaObj[index + 1];
      }
      --self.AreaCounter;
      if (self.AreaCounter == -1)
        return;
      self.AreaObj = (AreaClass[]) Utils.CopyArray((Array) self.AreaObj, (Array) new AreaClass[self.AreaCounter + 1]);
    }

    pub fn FindHistorical(ref HistoricalUnitClass his, libName: String) -> i32
    {
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (let mut historical: i32 =  0; historical <= historicalUnitCounter; historical += 1)
      {
        if (self.HistoricalUnitObj[historical].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.HistoricalUnitObj[historical].LibId.libSlot].name, libName, false) == 0 && self.HistoricalUnitObj[historical].LibId.id == his.ID)
          return historical;
      }
      return -1;
    }

    pub fn FindHistoricalFromSameLib(ref HistoricalUnitClass his, libName: String) -> i32
    {
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (let mut historicalFromSameLib: i32 =  0; historicalFromSameLib <= historicalUnitCounter; historicalFromSameLib += 1)
      {
        if (self.HistoricalUnitObj[historicalFromSameLib].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.HistoricalUnitObj[historicalFromSameLib].LibId.libSlot].name, libName, false) == 0 && self.HistoricalUnitObj[historicalFromSameLib].LibId.id == his.LibId.id)
          return historicalFromSameLib;
      }
      return -1;
    }

    pub fn FindOffHistorical(ref HistoricalUnitClass his, libName: String) -> i32
    {
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (let mut offHistorical: i32 =  0; offHistorical <= historicalUnitCounter; offHistorical += 1)
      {
        if (self.HistoricalUnitObj[offHistorical].OffLibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.HistoricalUnitObj[offHistorical].OffLibId.libSlot].name, libName, false) == 0 && self.HistoricalUnitObj[offHistorical].OffLibId.id == his.ID)
          return offHistorical;
      }
      return -1;
    }

    pub fn FindOffHistoricalBySameLib(ref HistoricalUnitClass his, libName: String) -> i32
    {
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (let mut historicalBySameLib: i32 =  0; historicalBySameLib <= historicalUnitCounter; historicalBySameLib += 1)
      {
        if (self.HistoricalUnitObj[historicalBySameLib].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.HistoricalUnitObj[historicalBySameLib].LibId.libSlot].name, libName, false) == 0 && self.HistoricalUnitObj[historicalBySameLib].LibId.id == his.OffLibId.id)
          return historicalBySameLib;
      }
      return -1;
    }

    pub fn AddHistoricalUnit()
    {
      this += 1.HistoricalUnitCounter;
      self.HistoricalUnitObj = (HistoricalUnitClass[]) Utils.CopyArray((Array) self.HistoricalUnitObj, (Array) new HistoricalUnitClass[self.HistoricalUnitCounter + 1]);
      self.HistoricalUnitObj[self.HistoricalUnitCounter] = HistoricalUnitClass::new();
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
      {
        if (self.HistoricalUnitObj[index].ID >= self.HistoricalIDCounter)
          self.HistoricalIDCounter = self.HistoricalUnitObj[index].ID;
      }
      this += 1.HistoricalIDCounter;
      self.HistoricalUnitObj[self.HistoricalUnitCounter].ID = self.HistoricalIDCounter;
    }

    pub fn RemoveHistoricalUnit(nr: i32)
    {
      self.ThreadBlock();
      self.ChangeHistoricalUnitNr(nr, -1);
      if (nr < self.HistoricalUnitCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.HistoricalUnitCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.HistoricalUnitObj[Newnr] = self.HistoricalUnitObj[Newnr + 1];
          self.ChangeHistoricalUnitNr(Newnr + 1, Newnr);
        }
      }
      --self.HistoricalUnitCounter;
      if (self.HistoricalUnitCounter != -1)
        self.HistoricalUnitObj = (HistoricalUnitClass[]) Utils.CopyArray((Array) self.HistoricalUnitObj, (Array) new HistoricalUnitClass[self.HistoricalUnitCounter + 1]);
      self.ThreadRelease();
    }

    pub fn ChangeHistoricalUnitNr(Oldnr: i32, Newnr: i32, bool QuickMode = false)
    {
      QuickMode = true;
      index1: i32;
      if (!QuickMode)
      {
        let mut stringListCounter: i32 =  self.StringListCounter;
        for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
        {
          let mut width: i32 =  self.StringListObj[index2].Width;
          for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
          {
            if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.HistoricalUnitId | self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.HistoricalUnitModelId | self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.OfficerId)
            {
              let mut length: i32 =  self.StringListObj[index2].Length;
              for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
              {
                if (self.StringListObj[index2].Data[index4, index3].Length > 0)
                {
                  index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
                  if (index1 == Oldnr)
                    index1 = Newnr;
                  self.StringListObj[index2].Data[index4, index3] = index1.ToString();
                  self.StringListObj[index2].Data[index4, index3] = index1.ToString();
                }
              }
            }
          }
        }
        for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
        {
          if (self.HistoricalUnitObj[Oldnr].LibId.libSlot > -1 & (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.HistoricalUnit | self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.HistoricalUnitModel | self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.Officer) & self.LibVarObj[libVarCounter].instanceId.id == self.HistoricalUnitObj[Oldnr].LibId.id & self.LibVarObj[libVarCounter].instanceId.libSlot == self.HistoricalUnitObj[Oldnr].LibId.libSlot && Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          if (self.HistoricalUnitObj[Oldnr].LibId.libSlot == -1 & (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.HistoricalUnit | self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.HistoricalUnitModel | self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.Officer) & self.LibVarObj[libVarCounter].instanceId.id == Oldnr & self.LibVarObj[libVarCounter].instanceId.libSlot == -1)
          {
            if (Newnr == -1)
              self.RemoveLibVar(libVarCounter);
            else
              self.LibVarObj[libVarCounter].instanceId.id = Newnr;
          }
        }
        for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
        {
          if (self.LibVarObj[libVarCounter].instanceId.id > -1 & (self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.HistoricalUnitId | self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.HistoricalUnitModelId | self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.OfficerId) & self.LibVarObj[libVarCounter].value == Oldnr)
          {
            if (Newnr == -1)
              self.RemoveLibVar(libVarCounter);
            else
              self.LibVarObj[libVarCounter].value = Newnr;
          }
        }
      }
      if (self.Product == 7)
      {
        let mut regimeCounter: i32 =  self.RegimeCounter;
        for (let mut index5: i32 =  0; index5 <= regimeCounter; index5 += 1)
        {
          if (!self.RegimeObj[index5].AI)
          {
            let mut mapWidth: i32 =  self.MapObj[0].MapWidth;
            for (let mut index6: i32 =  0; index6 <= mapWidth; index6 += 1)
            {
              let mut mapHeight: i32 =  self.MapObj[0].MapHeight;
              for (let mut index7: i32 =  0; index7 <= mapHeight; index7 += 1)
              {
                if (self.RegimeObj[0].HistoryHis[0].Value[index6, index7] == Oldnr)
                  self.RegimeObj[0].HistoryHis[0].Value[index6, index7] = Newnr;
              }
            }
          }
        }
      }
      for (index1 = self.UnitCounter; index1 >= 0; index1 += -1)
      {
        if (self.UnitObj[index1].Historical == Oldnr)
          self.UnitObj[index1].Historical = Newnr;
      }
      for (index1 = self.HistoricalUnitCounter; index1 >= 0; index1 += -1)
      {
        if (self.HistoricalUnitObj[index1].ModelMaster == Oldnr)
          self.HistoricalUnitObj[index1].ModelMaster = Newnr;
        if (self.HistoricalUnitObj[index1].UseModelCounter == Oldnr)
          self.HistoricalUnitObj[index1].UseModelCounter = Newnr;
      }
    }

    pub fn FindRegime(ref RegimeClass reg, libName: String) -> i32
    {
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut regime: i32 =  0; regime <= regimeCounter; regime += 1)
      {
        if (self.RegimeObj[regime].libId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.RegimeObj[regime].libId.libSlot].name, libName, false) == 0 && self.RegimeObj[regime].libId.id == reg.id)
          return regime;
      }
      return -1;
    }

    pub fn FindRegimeFromSameLib(ref RegimeClass reg, libName: String) -> i32
    {
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut regimeFromSameLib: i32 =  0; regimeFromSameLib <= regimeCounter; regimeFromSameLib += 1)
      {
        if (Operators.CompareString(self.RegimeObj[regimeFromSameLib].Name, reg.Name, false) == 0)
          return regimeFromSameLib;
      }
      return -1;
    }

    pub fn AddRegime(bool noRedimNecc = false, bool tMinimumDataUsage = false)
    {
      self.ThreadBlock();
      this += 1.RegimeCounter;
      self.RegimeObj = (RegimeClass[]) Utils.CopyArray((Array) self.RegimeObj, (Array) new RegimeClass[self.RegimeCounter + 1]);
      self.RegimeObj[self.RegimeCounter] = new RegimeClass(0, self.RegimeCounter, self.ResearchCounter, this, tMinimumDataUsage);
      self.RegimeObj[self.RegimeCounter].LoadSprites();
      this += 1.RegimeIdCounter;
      self.RegimeObj[self.RegimeCounter].id = self.RegimeIdCounter;
      let mut num1: i32 =  -1;
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
      {
        if (!self.RegimeObj[index].minimumDataUsage)
          num1 = index;
      }
      let mut mapCounter1: i32 =  self.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter1; index1 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            while (self.MapObj[index1].HexObj[index2, index3].RegimeCount < self.RegimeCounter)
              self.MapObj[index1].HexObj[index2, index3].UNUSED_addnewregime(self.MapObj[index1].HexObj[index2, index3].RegimeCount + 1, !self.RegimeObj[self.MapObj[index1].HexObj[index2, index3].RegimeCount + 1].minimumDataUsage, noRedimNecc);
            self.MapObj[index1].HexObj[index2, index3].set_LastLT(self.RegimeCounter, -1);
            self.MapObj[index1].HexObj[index2, index3].set_LastReg(self.RegimeCounter, -1);
            self.MapObj[index1].HexObj[index2, index3].set_LastSpr(self.RegimeCounter, -1);
          }
        }
      }
      if (self.RegimeCounter > 0)
      {
        let mut num2: i32 =  self.RegimeCounter - 1;
        for (let mut index: i32 =  0; index <= num2; index += 1)
          self.RegimeObj[index].AddRegime();
      }
      if (noRedimNecc)
      {
        let mut mapCounter2: i32 =  self.MapCounter;
        for (let mut index4: i32 =  0; index4 <= mapCounter2; index4 += 1)
        {
          let mut mapWidth: i32 =  self.MapObj[index4].MapWidth;
          for (let mut index5: i32 =  0; index5 <= mapWidth; index5 += 1)
          {
            let mut mapHeight: i32 =  self.MapObj[index4].MapHeight;
            for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
            {
              let mut regimeFullCount: i32 =  self.MapObj[0].HexObj[index5, index6].RegimeFullCount;
              for (let mut Index: i32 =  0; Index <= regimeFullCount; Index += 1)
              {
                self.MapObj[index4].HexObj[index5, index6].set_LastLT(Index, -1);
                self.MapObj[index4].HexObj[index5, index6].set_LastReg(Index, -1);
                self.MapObj[index4].HexObj[index5, index6].set_LastSpr(Index, -1);
              }
            }
          }
        }
      }
      self.ThreadRelease();
    }

    pub fn MoveRegimeHigher(regnr: i32)
    {
      self.AddRegime();
      if (regnr < self.RegimeCounter - 1)
      {
        self.RegimeObj[self.RegimeCounter] = self.RegimeObj[regnr + 1];
        self.ChangeRegimeNr(regnr + 1, self.RegimeCounter);
        self.ChangeRegimeNr(regnr, regnr + 1);
        self.RegimeObj[regnr + 1] = self.RegimeObj[regnr];
        self.ChangeRegimeNr(self.RegimeCounter, regnr);
        self.RegimeObj[regnr] = self.RegimeObj[self.RegimeCounter];
      }
      self.RemoveRegime(self.RegimeCounter);
      self.RegimeObj[regnr].LoadSprites();
      self.RegimeObj[regnr + 1].LoadSprites();
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
      {
        let mut num: i32 =  self.RegimeObj[index].RegimeRel[regnr + 1];
        self.RegimeObj[index].RegimeRel[regnr + 1] = self.RegimeObj[index].RegimeRel[regnr];
        self.RegimeObj[index].RegimeRel[regnr] = num;
      }
    }

    pub fn MoveRegimeLower(regnr: i32)
    {
      self.AddRegime();
      if (regnr > 0)
      {
        self.RegimeObj[self.RegimeCounter] = self.RegimeObj[regnr - 1];
        self.ChangeRegimeNr(regnr - 1, self.RegimeCounter);
        self.ChangeRegimeNr(regnr, regnr - 1);
        self.RegimeObj[regnr - 1] = self.RegimeObj[regnr];
        self.ChangeRegimeNr(self.RegimeCounter, regnr);
        self.RegimeObj[regnr] = self.RegimeObj[self.RegimeCounter];
      }
      self.RemoveRegime(self.RegimeCounter);
      self.RegimeObj[regnr].LoadSprites();
      self.RegimeObj[regnr - 1].LoadSprites();
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
      {
        let mut num: i32 =  self.RegimeObj[index].RegimeRel[regnr - 1];
        self.RegimeObj[index].RegimeRel[regnr - 1] = self.RegimeObj[index].RegimeRel[regnr];
        self.RegimeObj[index].RegimeRel[regnr] = num;
      }
    }

    pub fn RemoveRegime(nr: i32)
    {
      self.ThreadBlock();
      bool minimumDataUsage = self.RegimeObj[nr].minimumDataUsage;
      self.RegimeObj[nr].Kill();
      self.ChangeRegimeNr(nr, -1);
      if (nr < self.RegimeCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.RegimeCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.RegimeObj[Newnr] = self.RegimeObj[Newnr + 1];
          self.ChangeRegimeNr(Newnr + 1, Newnr);
        }
      }
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            self.MapObj[index1].HexObj[index2, index3].removeregime(!minimumDataUsage, nr);
        }
      }
      let mut num: i32 =  self.RegimeCounter - 1;
      for (let mut index: i32 =  0; index <= num; index += 1)
        self.RegimeObj[index].Removeregime(nr);
      --self.RegimeCounter;
      if (self.RegimeCounter != -1)
        self.RegimeObj = (RegimeClass[]) Utils.CopyArray((Array) self.RegimeObj, (Array) new RegimeClass[self.RegimeCounter + 1]);
      self.ThreadRelease();
    }

    pub fn ChangeRegimeNr(Oldnr: i32, Newnr: i32, bool QuickMode = false)
    {
      QuickMode = true;
      index1: i32;
      if (!QuickMode)
      {
        let mut stringListCounter: i32 =  self.StringListCounter;
        for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
        {
          let mut width: i32 =  self.StringListObj[index2].Width;
          for (index1 = 0; index1 <= width; index1 += 1)
          {
            let mut length: i32 =  self.StringListObj[index2].Length;
            for (let mut index3: i32 =  0; index3 <= length; index3 += 1)
            {
              if (self.StringListObj[index2].ColValueType[index1] == NewEnums.LibVarValueType.RegimeId && self.StringListObj[index2].Data[index3, index1].Length > 0)
              {
                let mut num: i32 =   Math.Round(Conversion.Val(self.StringListObj[index2].Data[index3, index1]));
                if (num == Oldnr)
                  num = Newnr;
                self.StringListObj[index2].Data[index3, index1] = num.ToString();
              }
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.Regime & self.LibVarObj[libVarCounter].instanceId.id == self.RegimeObj[Oldnr].libId.id & self.LibVarObj[libVarCounter].instanceId.libSlot == self.RegimeObj[Oldnr].libId.libSlot)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].instanceId.id = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.RegimeId & self.LibVarObj[libVarCounter].value == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = Newnr;
        }
      }
      let mut mapCounter1: i32 =  self.MapCounter;
      for (let mut index4: i32 =  0; index4 <= mapCounter1; index4 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index4].MapWidth;
        for (index1 = 0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index4].MapHeight;
          for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
          {
            if (self.MapObj[index4].HexObj[index1, index5].Regime == Oldnr & Newnr == -1 && self.MapObj[index4].HexObj[index1, index5].Location > -1)
              self.LocObj[self.MapObj[index4].HexObj[index1, index5].Location].HQ = -1;
            if (self.MapObj[index4].HexObj[index1, index5].Regime == Oldnr)
              self.MapObj[index4].HexObj[index1, index5].Regime = Newnr;
            if (self.Product >= 6 & self.Round > 0)
            {
              let mut num: i32 =  self.MapObj[index4].HexObj[index1, index5].get_LastReg(Oldnr);
              if (Newnr != -1)
                self.MapObj[index4].HexObj[index1, index5].set_LastReg(Newnr, num);
              let mut regimeCounter: i32 =  self.RegimeCounter;
              for (let mut Index: i32 =  0; Index <= regimeCounter; Index += 1)
              {
                if (self.MapObj[index4].HexObj[index1, index5].get_LastReg(Index) == Oldnr)
                  self.MapObj[index4].HexObj[index1, index5].set_LastReg(Index, Newnr);
              }
            }
          }
        }
      }
      if (self.Product >= 6 & self.Round > 0)
      {
        if (!Information.IsNothing( DrawMod.TGame.EditObj.HisOwner[0]))
        {
          try
          {
            let mut mapWidth: i32 =  self.MapObj[0].MapWidth;
            for (index1 = 0; index1 <= mapWidth; index1 += 1)
            {
              let mut mapHeight: i32 =  self.MapObj[0].MapHeight;
              for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
              {
                if (DrawMod.TGame.EditObj.HisOwner[0].Value[index1, index6] == Oldnr)
                  DrawMod.TGame.EditObj.HisOwner[0].Value[index1, index6] = Newnr;
              }
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            ProjectData.ClearProjectError();
          }
        }
      }
      if (self.Product >= 6 & self.Round > 0)
      {
        let mut tRegimeFullCount: i32 =  -1;
        let mut regimeCounter: i32 =  self.RegimeCounter;
        for (let mut index7: i32 =  0; index7 <= regimeCounter; index7 += 1)
        {
          if (!self.RegimeObj[index7].minimumDataUsage)
            tRegimeFullCount = index7;
        }
        let mut mapCounter2: i32 =  self.MapCounter;
        for (let mut index8: i32 =  0; index8 <= mapCounter2; index8 += 1)
        {
          let mut mapWidth: i32 =  self.MapObj[index8].MapWidth;
          for (let mut index9: i32 =  0; index9 <= mapWidth; index9 += 1)
          {
            let mut mapHeight: i32 =  self.MapObj[index8].MapHeight;
            for (let mut index10: i32 =  0; index10 <= mapHeight; index10 += 1)
              self.MapObj[index8].HexObj[index9, index10].redimRegime(self.RegimeCounter, tRegimeFullCount);
          }
        }
      }
      for (let mut index11: i32 =  self.RegimeCounter; index11 >= 0; index11 += -1)
      {
        if (self.Round > 0)
        {
          if (self.RegimeObj[index11].UberRegime == Oldnr)
            self.RegimeObj[index11].UberRegime = Newnr;
          try
          {
            if (!Information.IsNothing( self.RegimeObj[index11].HistoryOwner[0]))
            {
              let mut mapWidth: i32 =  self.MapObj[0].MapWidth;
              for (let mut index12: i32 =  0; index12 <= mapWidth; index12 += 1)
              {
                let mut mapHeight: i32 =  self.MapObj[0].MapHeight;
                for (let mut index13: i32 =  0; index13 <= mapHeight; index13 += 1)
                {
                  if (self.RegimeObj[index11].HistoryOwner[0].Value[index12, index13] == Oldnr)
                    self.RegimeObj[index11].HistoryOwner[0].Value[index12, index13] = Newnr;
                }
              }
            }
            let mut historyStepCounter: i32 =  self.RegimeObj[index11].HistoryStepCounter;
            for (let mut index14: i32 =  0; index14 <= historyStepCounter; index14 += 1)
            {
              if (self.RegimeObj[index11].HistoryStep[index14].Ownership == Oldnr)
                self.RegimeObj[index11].HistoryStep[index14].Ownership = Newnr;
              if (self.RegimeObj[index11].HistoryStep[index14].LossAttReg == Oldnr)
                self.RegimeObj[index11].HistoryStep[index14].LossAttReg = Newnr;
              if (self.RegimeObj[index11].HistoryStep[index14].LossDefReg == Oldnr)
                self.RegimeObj[index11].HistoryStep[index14].LossDefReg = Newnr;
            }
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            index11 = index11;
            ProjectData.ClearProjectError();
          }
        }
      }
      for (let mut unitCounter: i32 =  self.UnitCounter; unitCounter >= 0; unitCounter += -1)
      {
        if (self.UnitObj[unitCounter].Regime == Oldnr)
        {
          if (Newnr == -1)
          {
            let mut nr: i32 =  unitCounter;
            let mut gameClass: GameClass = (GameClass) null;
            ref let mut local: GameClass = ref gameClass;
            self.RemoveUnit(nr, ref local, true);
          }
          else
            self.UnitObj[unitCounter].Regime = Newnr;
        }
      }
      let mut itemTypeCounter: i32 =  self.ItemTypeCounter;
      for (let mut index15: i32 =  0; index15 <= itemTypeCounter; index15 += 1)
      {
        if (self.ItemTypeObj[index15].RegimeSpecific > -1 & self.ItemTypeObj[index15].RegimeSpecific == Oldnr)
          self.ItemTypeObj[index15].RegimeSpecific = Newnr;
      }
      for (let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
      {
        if (self.HistoricalUnitObj[historicalUnitCounter].TempRegime == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveHistoricalUnit(historicalUnitCounter);
          else
            self.HistoricalUnitObj[historicalUnitCounter].TempRegime = Newnr;
        }
      }
      let mut peopleCounter: i32 =  self.PeopleCounter;
      for (let mut index16: i32 =  0; index16 <= peopleCounter; index16 += 1)
      {
        if (self.PeopleObj[index16].RegCol == Oldnr)
          self.PeopleObj[index16].RegCol = Newnr;
      }
    }

    pub fn FindStringList(ref StringListClass e, libname: String) -> i32
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      for (let mut stringList: i32 =  0; stringList <= stringListCounter; stringList += 1)
      {
        if (self.StringListObj[stringList].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.StringListObj[stringList].LibId.libSlot].name, libname, false) == 0 && Operators.CompareString(self.StringListObj[stringList].Name, e.Name, false) == 0 & self.StringListObj[stringList].LibId.id == e.ID)
          return stringList;
      }
      return -1;
    }

    pub fn AddStringList()
    {
      this += 1.StringListCounter;
      this += 1.StringIDCounter;
      let mut num: i32 =  self.StringListCounter - 1;
      for (let mut index: i32 =  0; index <= num; index += 1)
      {
        if (self.StringListObj[index].ID >= self.StringIDCounter)
          self.StringIDCounter = self.StringListObj[index].ID + 1;
      }
      self.StringListObj = (StringListClass[]) Utils.CopyArray((Array) self.StringListObj, (Array) new StringListClass[self.StringListCounter + 1]);
      self.StringListObj[self.StringListCounter] = new StringListClass(self.StringIDCounter);
    }

    pub fn RemoveStringList(nr: i32)
    {
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
      {
        if (self.RegimeObj[index].OfficerPool == nr)
          self.RegimeObj[index].OfficerPool = -1;
      }
      if (nr < self.StringListCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.StringListCounter - 1;
        for (let mut index: i32 =  num1; index <= num2; index += 1)
          self.StringListObj[index] = self.StringListObj[index + 1];
      }
      --self.StringListCounter;
      if (self.StringListCounter == -1)
        return;
      self.StringListObj = (StringListClass[]) Utils.CopyArray((Array) self.StringListObj, (Array) new StringListClass[self.StringListCounter + 1]);
    }

    pub fn AddResearch()
    {
      this += 1.ResearchCounter;
      self.ResearchObj = (ResearchClass[]) Utils.CopyArray((Array) self.ResearchObj, (Array) new ResearchClass[self.ResearchCounter + 1]);
      self.ResearchObj[self.ResearchCounter] = new ResearchClass(0);
      self.ResearchObj[self.ResearchCounter].LoadSprites();
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        self.RegimeObj[index].AddResField();
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
        self.SFTypeObj[index].AddResField();
    }

    pub fn RemoveResearch(nr: i32)
    {
      self.ResearchObj[nr].Kill();
      self.ChangeResearchNr(nr, -1);
      if (nr < self.ResearchCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.ResearchCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.ResearchObj[Newnr] = self.ResearchObj[Newnr + 1];
          self.ChangeResearchNr(Newnr + 1, Newnr);
        }
      }
      --self.ResearchCounter;
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        self.RegimeObj[index].RemoveResField(nr);
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
        self.SFTypeObj[index].RemoveResField(nr);
      if (self.ResearchCounter == -1)
        return;
      self.ResearchObj = (ResearchClass[]) Utils.CopyArray((Array) self.ResearchObj, (Array) new ResearchClass[self.ResearchCounter + 1]);
    }

    pub fn ChangeResearchNr(Oldnr: i32, Newnr: i32)
    {
      let mut itemTypeCounter: i32 =  self.ItemTypeCounter;
      for (let mut index1: i32 =  0; index1 <= itemTypeCounter; index1 += 1)
      {
        let mut index2: i32 =  0;
        do
        {
          if (self.ItemTypeObj[index1].ResFieldNeeded[index2] == Oldnr)
            self.ItemTypeObj[index1].ResFieldNeeded[index2] = Newnr;
          index2 += 1;
        }
        while (index2 <= 4);
      }
      let mut locTypeCounter: i32 =  self.LocTypeCounter;
      for (let mut index3: i32 =  0; index3 <= locTypeCounter; index3 += 1)
      {
        let mut index4: i32 =  0;
        do
        {
          if (self.LocTypeObj[index3].Research[index4] == Oldnr)
            self.LocTypeObj[index3].Research[index4] = Newnr;
          index4 += 1;
        }
        while (index4 <= 4);
      }
      let mut researchCounter: i32 =  self.ResearchCounter;
      for (let mut index: i32 =  0; index <= researchCounter; index += 1)
      {
        if (self.ResearchObj[index].PreReq == Oldnr)
          self.ResearchObj[index].PreReq = Newnr;
        if (self.ResearchObj[index].PreReq2 == Oldnr)
          self.ResearchObj[index].PreReq2 = Newnr;
      }
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index5: i32 =  0; index5 <= sfTypeCounter; index5 += 1)
      {
        let mut index6: i32 =  0;
        do
        {
          if (self.SFTypeObj[index5].ModelResearch[index6] == Oldnr)
            self.SFTypeObj[index5].ModelResearch[index6] = Newnr;
          index6 += 1;
        }
        while (index6 <= 9);
      }
    }

    pub fn AddLocType()
    {
      this += 1.LocTypeCounter;
      self.LocTypeObj = (LocationTypeClass[]) Utils.CopyArray((Array) self.LocTypeObj, (Array) new LocationTypeClass[self.LocTypeCounter + 1]);
      self.LocTypeObj[self.LocTypeCounter] = new LocationTypeClass(0);
      self.LocTypeObj[self.LocTypeCounter].LoadSprites();
    }

    pub fn RemoveLocType(nr: i32)
    {
      self.LocTypeObj[nr].Kill();
      self.ChangeLocTypeNr(nr, -1);
      if (nr < self.LocTypeCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.LocTypeCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.LocTypeObj[Newnr] = self.LocTypeObj[Newnr + 1];
          self.ChangeLocTypeNr(Newnr + 1, Newnr);
        }
      }
      --self.LocTypeCounter;
      if (self.LocTypeCounter == -1)
        return;
      self.LocTypeObj = (LocationTypeClass[]) Utils.CopyArray((Array) self.LocTypeObj, (Array) new LocationTypeClass[self.LocTypeCounter + 1]);
    }

    pub fn ChangeLocTypeNr(Oldnr: i32, Newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      nr: i32;
      for (let mut index1: i32 =  0; index1 <= stringListCounter; index1 += 1)
      {
        let mut width: i32 =  self.StringListObj[index1].Width;
        for (let mut index2: i32 =  0; index2 <= width; index2 += 1)
        {
          let mut length: i32 =  self.StringListObj[index1].Length;
          for (let mut index3: i32 =  0; index3 <= length; index3 += 1)
          {
            if (self.StringListObj[index1].ColValueType[index2] == NewEnums.LibVarValueType.LocationTypeId && self.StringListObj[index1].Data[index3, index2].Length > 0)
            {
              nr =  Math.Round(Conversion.Val(self.StringListObj[index1].Data[index3, index2]));
              if (nr == Oldnr)
                nr = Newnr;
              self.StringListObj[index1].Data[index3, index2] = nr.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.LocationType & self.LibVarObj[libVarCounter].instanceId.id == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].instanceId.id = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.LocationTypeId & self.LibVarObj[libVarCounter].value == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = Newnr;
        }
      }
      for (nr = self.LocCounter; nr >= 0; nr += -1)
      {
        if (self.LocObj[nr].Type == Oldnr)
          self.LocObj[nr].Type = Newnr;
        if (self.LocObj[nr].Type == -1)
          self.RemoveLoc(nr);
      }
    }

    pub fn AddLoc(x: i32, y: i32, map: i32)
    {
      self.ThreadBlock();
      self.LocIdCounter = Operators.AddObject(self.LocIdCounter,  1);
      this += 1.LocCounter;
      self.LocObj = (LocationClass[]) Utils.CopyArray((Array) self.LocObj, (Array) new LocationClass[self.LocCounter + 1]);
      self.LocObj[self.LocCounter] = new LocationClass(0);
      self.LocObj[self.LocCounter].LoadSprites();
      self.LocObj[self.LocCounter].X = x;
      self.LocObj[self.LocCounter].Y = y;
      self.LocObj[self.LocCounter].Map = map;
      self.LocObj[self.LocCounter].ID = Conversions.ToInteger(self.LocIdCounter);
      self.LocObj[self.LocCounter].items = ItemList::new();
      self.ThreadRelease();
    }

    pub fn AddLoc2(x: i32, y: i32, map: i32)
    {
      self.ThreadBlock();
      self.LocIdCounter = Operators.AddObject(self.LocIdCounter,  1);
      this += 1.LocCounter;
      self.LocObj = (LocationClass[]) Utils.CopyArray((Array) self.LocObj, (Array) new LocationClass[self.LocCounter + 1]);
      self.LocObj[self.LocCounter] = new LocationClass(0);
      self.LocObj[self.LocCounter].LoadSprites();
      self.LocObj[self.LocCounter].X = x;
      self.LocObj[self.LocCounter].Y = y;
      self.LocObj[self.LocCounter].LocSlot = 2;
      self.LocObj[self.LocCounter].Map = map;
      self.LocObj[self.LocCounter].ID = Conversions.ToInteger(self.LocIdCounter);
      self.ThreadRelease();
    }

    pub fn RemoveLoc(nr: i32)
    {
      self.ThreadBlock();
      self.LocObj[nr].Kill();
      self.ChangeLocNr(nr, -1);
      if (nr < self.LocCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.LocCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.LocObj[Newnr] = self.LocObj[Newnr + 1];
          self.ChangeLocNr(Newnr + 1, Newnr);
        }
      }
      --self.LocCounter;
      if (self.LocCounter != -1)
        self.LocObj = (LocationClass[]) Utils.CopyArray((Array) self.LocObj, (Array) new LocationClass[self.LocCounter + 1]);
      self.ThreadRelease();
    }

    pub fn ChangeLocNr(Oldnr: i32, Newnr: i32)
    {
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            if (self.MapObj[index1].HexObj[index2, index3].Location == Oldnr)
              self.MapObj[index1].HexObj[index2, index3].Location = Newnr;
            if (self.Product >= 6 && self.MapObj[index1].HexObj[index2, index3].Location2 == Oldnr)
              self.MapObj[index1].HexObj[index2, index3].Location2 = Newnr;
          }
        }
      }
    }

    pub fn FindEvent(ref EventClass e, libname: String) -> i32
    {
      let mut eventCounter: i32 =  self.EventCounter;
      for (let mut index: i32 =  0; index <= eventCounter; index += 1)
      {
        if (self.EventObj[index].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.EventObj[index].LibId.libSlot].name, libname, false) == 0 && Operators.CompareString(self.EventObj[index].Name, e.Name, false) == 0 & self.EventObj[index].LibId.id == e.Id)
          return index;
      }
      return -1;
    }

    pub fn FindEvent2(libname: String, id: i32) -> i32
    {
      let mut eventCounter: i32 =  self.EventCounter;
      for (let mut event2: i32 =  0; event2 <= eventCounter; event2 += 1)
      {
        if (self.EventObj[event2].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.EventObj[event2].LibId.libSlot].name, libname, false) == 0 && self.EventObj[event2].LibId.id == id)
          return event2;
      }
      return -1;
    }

    pub fn AddEvent(let mut insert: i32 =  -1)
    {
      let mut eventCounter1: i32 =  self.EventCounter;
      for (let mut index: i32 =  0; index <= eventCounter1; index += 1)
      {
        if (self.EventObj[index].Id >= self.EventIdCounter)
          self.EventIdCounter = self.EventObj[index].Id;
      }
      this += 1.EventCounter;
      self.EventObj = (EventClass[]) Utils.CopyArray((Array) self.EventObj, (Array) new EventClass[self.EventCounter + 1]);
      self.EventObj[self.EventCounter] = new EventClass(0);
      this += 1.EventIdCounter;
      self.EventObj[self.EventCounter].Id = self.EventIdCounter;
      if (insert <= -1 || insert >= self.EventCounter - 1)
        return;
      let mut eventCounter2: i32 =  self.EventCounter;
      let mut num: i32 =  insert + 2;
      for (let mut toonr: i32 =  eventCounter2; toonr >= num; toonr += -1)
      {
        self.EventObj[toonr] = self.EventObj[toonr - 1];
        self.ChangeEventNr2(toonr - 1, toonr);
      }
      self.EventObj[insert + 1] = new EventClass(0);
      self.EventObj[insert + 1].Id = self.EventIdCounter;
    }

    pub fn ChangeEventNr2(fromnr: i32, toonr: i32)
    {
      let mut actionCardCounter: i32 =  self.ActionCardCounter;
      for (let mut index: i32 =  0; index <= actionCardCounter; index += 1)
      {
        if (self.ActionCardObj[index].ExecuteEvent == fromnr)
          self.ActionCardObj[index].ExecuteEvent = toonr;
        if (self.ActionCardObj[index].PreExecuteEvent == fromnr)
          self.ActionCardObj[index].PreExecuteEvent = toonr;
      }
      let mut index1: i32 =  0;
      do
      {
        if (self.VariantEvent[index1] == fromnr)
          self.VariantEvent[index1] = toonr;
        index1 += 1;
      }
      while (index1 <= 9);
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (let mut index2: i32 =  0; index2 <= historicalUnitCounter; index2 += 1)
      {
        let mut autoEventCounter: i32 =  self.HistoricalUnitObj[index2].AutoEventCounter;
        for (let mut index3: i32 =  0; index3 <= autoEventCounter; index3 += 1)
        {
          if (self.HistoricalUnitObj[index2].AutoEvent[index3] == fromnr)
            self.HistoricalUnitObj[index2].AutoEvent[index3] = toonr;
        }
      }
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index4: i32 =  0; index4 <= sfTypeCounter; index4 += 1)
      {
        if (self.SFTypeObj[index4].ModelNewEvent == fromnr)
          self.SFTypeObj[index4].ModelNewEvent = toonr;
        let mut upperBound: i32 =  self.SFTypeObj[index4].ModelImproveEvent.GetUpperBound(0);
        for (let mut index5: i32 =  0; index5 <= upperBound; index5 += 1)
        {
          if (self.SFTypeObj[index4].ModelImproveEvent[index5] == fromnr)
            self.SFTypeObj[index4].ModelImproveEvent[index5] = toonr;
        }
        if (self.SFTypeObj[index4].ModelInitialEvent == fromnr)
          self.SFTypeObj[index4].ModelInitialEvent = toonr;
        let mut modelVariantCounter: i32 =  self.SFTypeObj[index4].ModelVariantCounter;
        for (let mut index6: i32 =  0; index6 <= modelVariantCounter; index6 += 1)
        {
          if (self.SFTypeObj[index4].ModelVariantCheck[index6] == fromnr)
            self.SFTypeObj[index4].ModelVariantCheck[index6] = toonr;
          if (self.SFTypeObj[index4].ModelVariantExec[index6] == fromnr)
            self.SFTypeObj[index4].ModelVariantExec[index6] = toonr;
        }
      }
      let mut locTypeCounter: i32 =  self.LocTypeCounter;
      for (let mut index7: i32 =  0; index7 <= locTypeCounter; index7 += 1)
      {
        if (self.LocTypeObj[index7].AIEvent == fromnr)
          self.LocTypeObj[index7].AIEvent = toonr;
        if (self.LocTypeObj[index7].AILocEvent == fromnr)
          self.LocTypeObj[index7].AILocEvent = toonr;
        if (self.LocTypeObj[index7].AIAfterBuildEvent == fromnr)
          self.LocTypeObj[index7].AIAfterBuildEvent = toonr;
      }
    }

    pub fn ChangeEventSwitchNr(fromnr: i32, toonr: i32)
    {
      let mut actionCardCounter: i32 =  self.ActionCardCounter;
      for (let mut index: i32 =  0; index <= actionCardCounter; index += 1)
      {
        if (self.ActionCardObj[index].ExecuteEvent == fromnr)
          self.ActionCardObj[index].ExecuteEvent = toonr;
        else if (self.ActionCardObj[index].ExecuteEvent == toonr)
          self.ActionCardObj[index].ExecuteEvent = fromnr;
        if (self.ActionCardObj[index].PreExecuteEvent == fromnr)
          self.ActionCardObj[index].PreExecuteEvent = toonr;
        else if (self.ActionCardObj[index].PreExecuteEvent == toonr)
          self.ActionCardObj[index].PreExecuteEvent = fromnr;
      }
      let mut index1: i32 =  0;
      do
      {
        if (self.VariantEvent[index1] == fromnr)
          self.VariantEvent[index1] = toonr;
        else if (self.VariantEvent[index1] == toonr)
          self.VariantEvent[index1] = fromnr;
        index1 += 1;
      }
      while (index1 <= 9);
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (let mut index2: i32 =  0; index2 <= historicalUnitCounter; index2 += 1)
      {
        let mut autoEventCounter: i32 =  self.HistoricalUnitObj[index2].AutoEventCounter;
        for (let mut index3: i32 =  0; index3 <= autoEventCounter; index3 += 1)
        {
          if (self.HistoricalUnitObj[index2].AutoEvent[index3] == fromnr)
            self.HistoricalUnitObj[index2].AutoEvent[index3] = toonr;
          else if (self.HistoricalUnitObj[index2].AutoEvent[index3] == toonr)
            self.HistoricalUnitObj[index2].AutoEvent[index3] = fromnr;
        }
      }
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index4: i32 =  0; index4 <= sfTypeCounter; index4 += 1)
      {
        if (self.SFTypeObj[index4].ModelNewEvent == fromnr)
          self.SFTypeObj[index4].ModelNewEvent = toonr;
        else if (self.SFTypeObj[index4].ModelNewEvent == toonr)
          self.SFTypeObj[index4].ModelNewEvent = fromnr;
        let mut researchCounter: i32 =  self.ResearchCounter;
        for (let mut index5: i32 =  0; index5 <= researchCounter; index5 += 1)
        {
          if (self.SFTypeObj[index4].ModelImproveEvent[index5] == fromnr)
            self.SFTypeObj[index4].ModelImproveEvent[index5] = toonr;
          else if (self.SFTypeObj[index4].ModelImproveEvent[index5] == toonr)
            self.SFTypeObj[index4].ModelImproveEvent[index5] = fromnr;
        }
        if (self.SFTypeObj[index4].ModelInitialEvent == fromnr)
          self.SFTypeObj[index4].ModelInitialEvent = toonr;
        else if (self.SFTypeObj[index4].ModelInitialEvent == toonr)
          self.SFTypeObj[index4].ModelInitialEvent = fromnr;
        let mut modelVariantCounter: i32 =  self.SFTypeObj[index4].ModelVariantCounter;
        for (let mut index6: i32 =  0; index6 <= modelVariantCounter; index6 += 1)
        {
          if (self.SFTypeObj[index4].ModelVariantCheck[index6] == fromnr)
            self.SFTypeObj[index4].ModelVariantCheck[index6] = toonr;
          else if (self.SFTypeObj[index4].ModelVariantCheck[index6] == toonr)
            self.SFTypeObj[index4].ModelVariantCheck[index6] = fromnr;
          if (self.SFTypeObj[index4].ModelVariantExec[index6] == fromnr)
            self.SFTypeObj[index4].ModelVariantExec[index6] = toonr;
          else if (self.SFTypeObj[index4].ModelVariantExec[index6] == toonr)
            self.SFTypeObj[index4].ModelVariantExec[index6] = fromnr;
        }
      }
    }

    pub fn RemoveEvent(nr: i32)
    {
      if (nr < self.EventCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.EventCounter - 1;
        for (let mut toonr: i32 =  num1; toonr <= num2; toonr += 1)
        {
          self.EventObj[toonr] = self.EventObj[toonr + 1];
          self.ChangeEventNr2(toonr + 1, toonr);
        }
      }
      self.ChangeEventNr2(self.EventCounter, -1);
      --self.EventCounter;
      if (self.EventCounter == -1)
        return;
      self.EventObj = (EventClass[]) Utils.CopyArray((Array) self.EventObj, (Array) new EventClass[self.EventCounter + 1]);
    }

    pub fn eventdown(nr: i32)
    {
      if (!(nr < self.EventCounter & nr > -1))
        return;
      EventClass eventClass = self.EventObj[nr].Clone();
      self.EventObj[nr] = self.EventObj[nr + 1].Clone();
      self.EventObj[nr + 1] = eventClass;
      self.ChangeEventSwitchNr(nr, nr + 1);
    }

    pub fn eventup(nr: i32)
    {
      if (nr <= 0)
        return;
      EventClass eventClass = self.EventObj[nr].Clone();
      self.EventObj[nr] = self.EventObj[nr - 1].Clone();
      self.EventObj[nr - 1] = eventClass;
      self.ChangeEventSwitchNr(nr, nr - 1);
    }

    pub fn FindPeople(ref PeopleClass ppl, libName: String) -> i32
    {
      let mut peopleCounter: i32 =  self.PeopleCounter;
      for (let mut people: i32 =  0; people <= peopleCounter; people += 1)
      {
        if (self.PeopleObj[people].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.PeopleObj[people].LibId.libSlot].name, libName, false) == 0 && self.PeopleObj[people].LibId.id == ppl.id)
          return people;
      }
      return -1;
    }

    pub fn AddPeople()
    {
      this += 1.PeopleCounter;
      self.PeopleObj = (PeopleClass[]) Utils.CopyArray((Array) self.PeopleObj, (Array) new PeopleClass[self.PeopleCounter + 1]);
      self.PeopleObj[self.PeopleCounter] = new PeopleClass(0);
      self.PeopleObj[self.PeopleCounter].LoadSprites();
      this += 1.PeopleIdCounter;
      self.PeopleObj[self.PeopleCounter].id = self.PeopleIdCounter;
    }

    pub fn RemovePeople(nr: i32)
    {
      self.PeopleObj[nr].Kill();
      self.ChangePeopleNr(nr, -1);
      if (nr < self.PeopleCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.PeopleCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.PeopleObj[Newnr] = self.PeopleObj[Newnr + 1];
          self.ChangePeopleNr(Newnr + 1, Newnr);
        }
      }
      --self.PeopleCounter;
      if (self.PeopleCounter == -1)
        return;
      self.PeopleObj = (PeopleClass[]) Utils.CopyArray((Array) self.PeopleObj, (Array) new PeopleClass[self.PeopleCounter + 1]);
    }

    pub fn ChangePeopleNr(Oldnr: i32, Newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      index1: i32;
      for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
      {
        let mut width: i32 =  self.StringListObj[index2].Width;
        for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
        {
          let mut length: i32 =  self.StringListObj[index2].Length;
          for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
          {
            if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.PeopleId && self.StringListObj[index2].Data[index4, index3].Length > 0)
            {
              index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
              if (index1 == Oldnr)
                index1 = Newnr;
              self.StringListObj[index2].Data[index4, index3] = index1.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].type == NewEnums.LibVarType.People & self.LibVarObj[libVarCounter].instanceId.id == self.PeopleObj[Oldnr].LibId.id & self.LibVarObj[libVarCounter].instanceId.libSlot == self.PeopleObj[Oldnr].LibId.libSlot)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].instanceId.id = Newnr;
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.PeopleId & self.LibVarObj[libVarCounter].value == Oldnr)
        {
          if (Newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = Newnr;
        }
      }
      if (Newnr == -1 & self.PeopleCounter > 0)
        Newnr = 0;
      let mut locCounter: i32 =  self.LocCounter;
      for (index1 = 0; index1 <= locCounter; index1 += 1)
      {
        if (self.LocObj[index1].People == Oldnr)
          self.LocObj[index1].People = Newnr;
      }
      let mut sfCounter: i32 =  self.SFCounter;
      for (index1 = 0; index1 <= sfCounter; index1 += 1)
      {
        if (self.SFObj[index1].People == Oldnr)
          self.SFObj[index1].People = Newnr;
      }
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (index1 = 0; index1 <= regimeCounter; index1 += 1)
      {
        if (self.RegimeObj[index1].People == Oldnr)
          self.RegimeObj[index1].People = Newnr;
      }
      let mut itemTypeCounter: i32 =  self.ItemTypeCounter;
      for (index1 = 0; index1 <= itemTypeCounter; index1 += 1)
      {
        if (self.ItemTypeObj[index1].PeopleMod == Oldnr)
          self.ItemTypeObj[index1].PeopleMod = Newnr;
      }
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (index1 = 0; index1 <= historicalUnitCounter; index1 += 1)
      {
        if (self.HistoricalUnitObj[index1].People == Oldnr)
          self.HistoricalUnitObj[index1].People = Newnr;
        if (self.HistoricalUnitObj[index1].UsePeopleGfx == Oldnr)
          self.HistoricalUnitObj[index1].UsePeopleGfx = Newnr;
      }
    }

    pub fn MovePeopleHigher(peopleNr: i32)
    {
      self.AddPeople();
      if (peopleNr < self.PeopleCounter - 1)
      {
        self.PeopleObj[self.PeopleCounter] = self.PeopleObj[peopleNr + 1];
        self.ChangePeopleNr(peopleNr + 1, self.PeopleCounter);
        self.ChangePeopleNr(peopleNr, peopleNr + 1);
        self.PeopleObj[peopleNr + 1] = self.PeopleObj[peopleNr];
        self.ChangePeopleNr(self.PeopleCounter, peopleNr);
        self.PeopleObj[peopleNr] = self.PeopleObj[self.PeopleCounter];
      }
      self.RemovePeople(self.PeopleCounter);
      self.PeopleObj[peopleNr].LoadSprites();
      self.PeopleObj[peopleNr + 1].LoadSprites();
    }

    pub fn MovePeopleLower(PeopleNr: i32)
    {
      self.AddPeople();
      if (PeopleNr > 0)
      {
        self.PeopleObj[self.PeopleCounter] = self.PeopleObj[PeopleNr - 1];
        self.ChangePeopleNr(PeopleNr - 1, self.PeopleCounter);
        self.ChangePeopleNr(PeopleNr, PeopleNr - 1);
        self.PeopleObj[PeopleNr - 1] = self.PeopleObj[PeopleNr];
        self.ChangePeopleNr(self.PeopleCounter, PeopleNr);
        self.PeopleObj[PeopleNr] = self.PeopleObj[self.PeopleCounter];
      }
      self.RemovePeople(self.PeopleCounter);
      self.PeopleObj[PeopleNr].LoadSprites();
      self.PeopleObj[PeopleNr - 1].LoadSprites();
    }

    pub fn AddMap(w: i32, h: i32)
    {
      this += 1.MapCounter;
      self.MapObj = (MapClass[]) Utils.CopyArray((Array) self.MapObj, (Array) new MapClass[self.MapCounter + 1]);
      self.MapObj[self.MapCounter] = new MapClass(0, self.RegimeCounter, w, h);
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        self.RegimeObj[index].AddMap(w, h);
    }

    pub fn RemoveMap(nr: i32)
    {
      self.ChangeMapNr(nr, -1);
      if (nr < self.MapCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.MapCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.MapObj[Newnr] = self.MapObj[Newnr + 1];
          self.ChangeMapNr(Newnr + 1, Newnr);
        }
      }
      --self.MapCounter;
      if (self.MapCounter != -1)
        self.MapObj = (MapClass[]) Utils.CopyArray((Array) self.MapObj, (Array) new MapClass[self.MapCounter + 1]);
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        self.RegimeObj[index].RemoveMap(nr);
    }

    pub fn ChangeMapNr(Oldnr: i32, Newnr: i32)
    {
      for (let mut unitCounter: i32 =  self.UnitCounter; unitCounter >= 0; unitCounter += -1)
      {
        if (self.UnitObj[unitCounter].Map == Oldnr & Newnr > -1)
          self.UnitObj[unitCounter].Map = Newnr;
        if (Newnr == -1)
        {
          let mut nr: i32 =  unitCounter;
          let mut gameClass: GameClass = (GameClass) null;
          ref let mut local: GameClass = ref gameClass;
          self.RemoveUnit(nr, ref local);
        }
      }
      for (let mut locCounter: i32 =  self.LocCounter; locCounter >= 0; locCounter += -1)
      {
        if (self.LocObj[locCounter].Map == Oldnr & Newnr > -1)
          self.LocObj[locCounter].Map = Newnr;
        if (Newnr == -1)
          self.RemoveLoc(locCounter);
      }
      let mut mapCounter: i32 =  self.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 =  self.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  self.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            for (let mut connectionCount: i32 =  self.MapObj[index1].HexObj[index2, index3].ConnectionCount; connectionCount >= 0; connectionCount += -1)
            {
              if (self.MapObj[index1].HexObj[index2, index3].ConnectionMap[connectionCount] == Oldnr)
              {
                if (Newnr > -1)
                  self.MapObj[index1].HexObj[index2, index3].ConnectionMap[connectionCount] = Newnr;
                else
                  self.MapObj[index1].HexObj[index2, index3].RemoveConnection(connectionCount);
              }
            }
          }
        }
      }
    }

    pub fn AddItemType()
    {
      let mut regimeCounter1: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter1; index += 1)
      {
        self.RegimeObj[index].SProd = new int[self.ItemTypeCounter + 1, self.Round + 1 + 1];
        self.RegimeObj[index].SASProdLost = (int[]) Utils.CopyArray((Array) self.RegimeObj[index].SASProdLost, (Array) new int[self.ItemTypeCounter + 1]);
      }
      this += 1.ItemTypeCounter;
      self.ItemTypeObj = (ItemTypeClass[]) Utils.CopyArray((Array) self.ItemTypeObj, (Array) new ItemTypeClass[self.ItemTypeCounter + 1]);
      self.ItemTypeObj[self.ItemTypeCounter] = new ItemTypeClass(0);
      self.ItemTypeObj[self.ItemTypeCounter].LoadSprites();
      object[,,] objArray = new object[self.RegimeCounter + 1, self.ItemTypeCounter + 1, self.Round + 1 + 1];
      let mut regimeCounter2: i32 =  self.RegimeCounter;
      for (let mut index1: i32 =  0; index1 <= regimeCounter2; index1 += 1)
      {
        let mut upperBound: i32 =  self.RegimeObj[index1].SProd.GetUpperBound(0);
        for (let mut index2: i32 =  0; index2 <= upperBound; index2 += 1)
        {
          let mut round: i32 =  self.Round;
          for (let mut index3: i32 =  0; index3 <= round; index3 += 1)
            objArray[index1, index2, index3] =  self.RegimeObj[index1].SProd[index2, index3];
        }
      }
      let mut regimeCounter3: i32 =  self.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter3; index += 1)
      {
        self.RegimeObj[index].SProd = new int[self.ItemTypeCounter + 1, self.Round + 1 + 1];
        self.RegimeObj[index].SASProdLost = (int[]) Utils.CopyArray((Array) self.RegimeObj[index].SASProdLost, (Array) new int[self.ItemTypeCounter + 1]);
      }
      let mut regimeCounter4: i32 =  self.RegimeCounter;
      for (let mut index4: i32 =  0; index4 <= regimeCounter4; index4 += 1)
      {
        let mut itemTypeCounter: i32 =  self.ItemTypeCounter;
        for (let mut index5: i32 =  0; index5 <= itemTypeCounter; index5 += 1)
        {
          let mut num: i32 =  self.Round + 1;
          for (let mut index6: i32 =  0; index6 <= num; index6 += 1)
            self.RegimeObj[index4].SProd[index5, index6] = Conversions.ToInteger(objArray[index4, index5, index6]);
        }
      }
    }

    pub fn RemoveItemType(nr: i32)
    {
      self.ItemTypeObj[nr].Kill();
      self.ChangeItemTypeNr(nr, -1);
      if (nr < self.ItemTypeCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.ItemTypeCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.ItemTypeObj[Newnr] = self.ItemTypeObj[Newnr + 1];
          self.ChangeItemTypeNr(Newnr + 1, Newnr);
        }
      }
      --self.ItemTypeCounter;
      if (self.ItemTypeCounter == -1)
        return;
      self.ItemTypeObj = (ItemTypeClass[]) Utils.CopyArray((Array) self.ItemTypeObj, (Array) new ItemTypeClass[self.ItemTypeCounter + 1]);
    }

    pub fn ChangeItemTypeNr(Oldnr: i32, Newnr: i32)
    {
      let mut itemTypeCounter: i32 =  self.ItemTypeCounter;
      for (let mut index: i32 =  0; index <= itemTypeCounter; index += 1)
      {
        if (self.ItemTypeObj[index].Blocks == Oldnr)
          self.ItemTypeObj[index].Blocks = Newnr;
      }
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
      {
        if (self.SFTypeObj[index].ModelItemType == Oldnr)
          self.SFTypeObj[index].ModelItemType = Newnr;
      }
    }

    pub fn FindActionCard(ref ActionCardClass e, origCardSlot: i32, libname: String) -> i32
    {
      if (Information.IsNothing( e))
      {
        let mut actionCardCounter: i32 =  self.ActionCardCounter;
        for (let mut actionCard: i32 =  0; actionCard <= actionCardCounter; actionCard += 1)
        {
          if (self.ActionCardObj[actionCard].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.ActionCardObj[actionCard].LibId.libSlot].name, libname, false) == 0 && self.ActionCardObj[actionCard].LibId.id == origCardSlot)
            return actionCard;
        }
      }
      else
      {
        let mut actionCardCounter: i32 =  self.ActionCardCounter;
        for (let mut actionCard: i32 =  0; actionCard <= actionCardCounter; actionCard += 1)
        {
          if (self.ActionCardObj[actionCard].LibId.libSlot > -1 && Operators.CompareString(self.LibraryObj[self.ActionCardObj[actionCard].LibId.libSlot].name, libname, false) == 0 && Operators.CompareString(self.ActionCardObj[actionCard].Title, e.Title, false) == 0 & self.ActionCardObj[actionCard].LibId.id == origCardSlot)
            return actionCard;
        }
      }
      return -1;
    }

    pub fn AddActionCard()
    {
      this += 1.ActionCardCounter;
      self.ActionCardObj = (ActionCardClass[]) Utils.CopyArray((Array) self.ActionCardObj, (Array) new ActionCardClass[self.ActionCardCounter + 1]);
      self.ActionCardObj[self.ActionCardCounter] = ActionCardClass::new();
    }

    pub fn RemoveActionCard(nr: i32, bool QuickMode = false)
    {
      self.ChangeActionCardNr(nr, -1, QuickMode);
      if (nr < self.ActionCardCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.ActionCardCounter - 1;
        for (let mut Newnr: i32 =  num1; Newnr <= num2; Newnr += 1)
        {
          self.ActionCardObj[Newnr] = self.ActionCardObj[Newnr + 1];
          self.ChangeActionCardNr(Newnr + 1, Newnr, QuickMode);
        }
      }
      --self.ActionCardCounter;
      if (self.ActionCardCounter == -1)
        return;
      self.ActionCardObj = (ActionCardClass[]) Utils.CopyArray((Array) self.ActionCardObj, (Array) new ActionCardClass[self.ActionCardCounter + 1]);
    }

    pub fn ChangeActionCardNr(Oldnr: i32, Newnr: i32, bool QuickMode = false)
    {
      index1: i32;
      if (!QuickMode)
      {
        let mut stringListCounter: i32 =  self.StringListCounter;
        for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
        {
          let mut width: i32 =  self.StringListObj[index2].Width;
          for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
          {
            let mut length: i32 =  self.StringListObj[index2].Length;
            for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
            {
              if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.ActionCardId && self.StringListObj[index2].Data[index4, index3].Length > 0)
              {
                index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
                if (index1 == Oldnr)
                  index1 = Newnr;
                self.StringListObj[index2].Data[index4, index3] = index1.ToString();
              }
            }
          }
        }
        for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
        {
          if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.ActionCardId & self.LibVarObj[libVarCounter].value == Oldnr)
          {
            if (Newnr == -1)
              self.RemoveLibVar(libVarCounter);
            else
              self.LibVarObj[libVarCounter].value = Newnr;
          }
        }
      }
      let mut regimeCounter: i32 =  self.RegimeCounter;
      for (index1 = 0; index1 <= regimeCounter; index1 += 1)
      {
        for (let mut actionCardCounter: i32 =  self.RegimeObj[index1].ActionCardCounter; actionCardCounter >= 0; actionCardCounter += -1)
        {
          if (Newnr > -1)
          {
            if (self.RegimeObj[index1].ActionCard[actionCardCounter] == Oldnr)
              self.RegimeObj[index1].ActionCard[actionCardCounter] = Newnr;
          }
          else if (self.RegimeObj[index1].ActionCard[actionCardCounter] == Oldnr)
            self.RegimeObj[index1].RemoveActionCard(actionCardCounter);
        }
      }
      if (QuickMode)
        return;
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (index1 = 0; index1 <= historicalUnitCounter; index1 += 1)
      {
        for (let mut deckCardCounter: i32 =  self.HistoricalUnitObj[index1].DeckCardCounter; deckCardCounter >= 0; deckCardCounter += -1)
        {
          if (self.HistoricalUnitObj[index1].DeckCard[deckCardCounter] == Oldnr)
          {
            if (Newnr == -1)
            {
              let mut num1: i32 =  deckCardCounter;
              let mut index5: i32 =  index1;
              if (num1 < self.HistoricalUnitObj[index5].DeckCardCounter)
              {
                let mut num2: i32 =  num1;
                let mut num3: i32 =  self.HistoricalUnitObj[index5].DeckCardCounter - 1;
                for (let mut index6: i32 =  num2; index6 <= num3; index6 += 1)
                {
                  self.HistoricalUnitObj[index5].DeckCard[index6] = self.HistoricalUnitObj[index5].DeckCard[index6 + 1];
                  self.HistoricalUnitObj[index5].DeckChance[index6] = self.HistoricalUnitObj[index5].DeckChance[index6 + 1];
                }
              }
              let mut num4: i32 =  num1 - 1;
              HistoricalUnitClass[] historicalUnitObj = self.HistoricalUnitObj;
              HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
              let mut index7: i32 =  index5;
              let mut index8: i32 =  index7;
              historicalUnitClassArray[index8].DeckCardCounter = historicalUnitObj[index7].DeckCardCounter - 1;
              if (self.HistoricalUnitObj[index5].DeckCardCounter > -1 & num4 == -1)
                ;
              if (self.HistoricalUnitObj[index5].DeckCardCounter > -1)
              {
                self.HistoricalUnitObj[index5].DeckCard = (int[]) Utils.CopyArray((Array) self.HistoricalUnitObj[index5].DeckCard, (Array) new int[self.HistoricalUnitObj[index5].DeckCardCounter + 1]);
                self.HistoricalUnitObj[index5].DeckChance = (int[]) Utils.CopyArray((Array) self.HistoricalUnitObj[index5].DeckChance, (Array) new int[self.HistoricalUnitObj[index5].DeckCardCounter + 1]);
              }
            }
            else
              self.HistoricalUnitObj[index1].DeckCard[deckCardCounter] = Newnr;
          }
        }
      }
    }

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      self.Version = 424;
      info.AddValue("Name",  self.Name);
      info.AddValue("Description",  self.Description);
      info.AddValue("Turn", self.Turn);
      info.AddValue("StepNr", self.StepNr);
      info.AddValue("Round", self.Round);
      self.MapWidth = self.MapObj[0].MapWidth;
      self.MapHeight = self.MapObj[0].MapHeight;
      info.AddValue("MapWidth", self.MapWidth);
      info.AddValue("MapHeight", self.MapHeight);
      info.AddValue("ShrowdOn", self.ShrowdOn);
      info.AddValue("FOWOn", self.FOWOn);
      info.AddValue("UncertaintyOn", self.UncertaintyOn);
      info.AddValue("ResMod", self.ResMod);
      GC.Collect();
      Application.DoEvents();
      info.AddValue("HexObj",  self.HexObj);
      info.AddValue("LandscapeTypeCounter", self.LandscapeTypeCounter);
      info.AddValue("LandscapeTypeObj",  self.LandscapeTypeObj);
      info.AddValue("RoadTypeCounter", self.RoadTypeCounter);
      info.AddValue("RoadTypeObj",  self.RoadTypeObj);
      info.AddValue("RegimeCounter", self.RegimeCounter);
      info.AddValue("RegimeObj",  self.RegimeObj);
      GC.Collect();
      Application.DoEvents();
      info.AddValue("UnitCounter", self.UnitCounter);
      info.AddValue("UnitObj",  self.UnitObj);
      info.AddValue("SFCounter", self.SFCounter);
      info.AddValue("SFObj",  self.SFObj);
      GC.Collect();
      Application.DoEvents();
      info.AddValue("SFTypeCounter", self.SFTypeCounter);
      info.AddValue("SFTypeObj",  self.SFTypeObj);
      info.AddValue("StringCounter", self.StringCounter);
      info.AddValue("TempString",  self.TempString);
      info.AddValue("LocTypeCounter", self.LocTypeCounter);
      info.AddValue("LocTypeObj",  self.LocTypeObj);
      info.AddValue("LocCounter", self.LocCounter);
      info.AddValue("LocObj",  self.LocObj);
      GC.Collect();
      Application.DoEvents();
      info.AddValue("ItemTypeCounter", self.ItemTypeCounter);
      info.AddValue("ItemTypeObj",  self.ItemTypeObj);
      info.AddValue("PeopleCounter", self.PeopleCounter);
      info.AddValue("PeopleObj",  self.PeopleObj);
      info.AddValue("GameSlot",  self.GameSlot);
      info.AddValue("GameSlotName",  self.GameSlotName);
      info.AddValue("RegimeSlotName",  self.RegimeSlotName);
      info.AddValue("RuleCounter", self.RuleCounter);
      info.AddValue("RuleVar",  self.RuleVar);
      info.AddValue("RuleGroup",  self.RuleGroup);
      info.AddValue("RiverTypeCounter", self.RiverTypeCounter);
      info.AddValue("RiverTypeObj",  self.RiverTypeObj);
      info.AddValue("HistoricalUnitCounter", self.HistoricalUnitCounter);
      info.AddValue("HistoricalUnitObj",  self.HistoricalUnitObj);
      info.AddValue("HistoricalIDCounter", self.HistoricalIDCounter);
      info.AddValue("AreaCounter", self.AreaCounter);
      info.AddValue("AreaObj",  self.AreaObj);
      info.AddValue("AreaIDCounter", self.AreaIDCounter);
      info.AddValue("BridgeObj",  self.BridgeObj);
      info.AddValue("ActionCardCounter", self.ActionCardCounter);
      info.AddValue("ActionCardObj",  self.ActionCardObj);
      GC.Collect();
      Application.DoEvents();
      info.AddValue("MapCounter", self.MapCounter);
      info.AddValue("MapObj",  self.MapObj);
      info.AddValue("ResearchCounter", self.ResearchCounter);
      info.AddValue("ResearchObj",  self.ResearchObj);
      info.AddValue("EventCounter", self.EventCounter);
      info.AddValue("EventObj",  self.EventObj);
      GC.Collect();
      Application.DoEvents();
      info.AddValue("EventPicCounter", self.EventPicCounter);
      info.AddValue("EventPicName",  self.EventPicName);
      info.AddValue("EventPicNr",  self.EventPicNr);
      info.AddValue("EventPicLibId",  self.eventPicLibId);
      info.AddValue("SmallPicCounter", self.SmallPicCounter);
      info.AddValue("SmallPicName",  self.SmallPicName);
      info.AddValue("SmallPicNr",  self.SmallPicNr);
      info.AddValue("SmallLibId",  self.SmallLibId);
      info.AddValue("ReinfCounter", self.ReinfCounter);
      info.AddValue("ReinfName",  self.ReinfName);
      info.AddValue("ReinfId",  self.ReinfId);
      info.AddValue("ReinfLibId",  self.ReinfLibId);
      info.AddValue("ReinfIdCounter", self.reinfIdCounter);
      info.AddValue("ReinfRatio",  self.ReinfRatio);
      info.AddValue("LoadPass",  self.LoadPass);
      info.AddValue("EditPass",  self.EditPass);
      info.AddValue("MasterFile",  self.MasterFile);
      info.AddValue("VPWin", self.VPWin);
      info.AddValue("Winner", self.Winner);
      info.AddValue("LastWinner", self.LastWinner);
      info.AddValue("PasswordsOn", self.PasswordsOn);
      info.AddValue("InTurn", self.InTurn);
      info.AddValue("Version", self.Version);
      GC.Collect();
      Application.DoEvents();
      info.AddValue("AlternateRound", self.AlternateRound);
      info.AddValue("AlternateRound2", self.AlternateRound2);
      info.AddValue("StartData", self.StartData);
      info.AddValue("ResCostMod", self.ResCostMod);
      info.AddValue("SupplyMultiplier", self.SupplyMultiplier);
      info.AddValue("PPMultiplier", self.PPMultiplier);
      info.AddValue("AsOn", self.ASOn);
      info.AddValue("RegimeSlotShow",  self.RegimeSlotShow);
      info.AddValue("RegimeSlotShow2",  self.RegimeSlotShow2);
      info.AddValue("RegimeSlotnato",  self.RegimeSlotNato);
      info.AddValue("RegimeSlotSmallGfx",  self.RegimeSlotSmallGfx);
      info.AddValue("GameSlotShow",  self.GameSlotShow);
      info.AddValue("GameSlotShow2",  self.GameSlotShow2);
      info.AddValue("GameSlotNato",  self.GameSlotNato);
      info.AddValue("GameSlotSmallGfx",  self.GameSlotSmallGfx);
      info.AddValue("NoPlayChoice", self.NoPlayChoice);
      info.AddValue("NoAIAdvice", self.NoAIAdvice);
      info.AddValue("PBEM", self.PBEM);
      info.AddValue("ScreenShotOn", self.ScreenShotOn);
      info.AddValue("CreatedWithShrowd", self.CreatedWithShrowd);
      info.AddValue("ShrowdPeek", self.ShrowdPeek);
      info.AddValue("Variants",  self.Variants);
      info.AddValue("MoveTypePenalty",  self.MoveTypePenalty);
      info.AddValue("UnitTypePenalty",  self.UnitTypePenalty);
      info.AddValue("WheaterColor",  self.WheaterColor);
      info.AddValue("Designer",  self.Designer);
      info.AddValue("Designer2",  self.Designer2);
      info.AddValue("MasterfileReadPeople", self.MasterfileReadPeople);
      info.AddValue("GameID", self.GameID);
      info.AddValue("AutoSave", self.AutoSave);
      self.MapLoop = self.MapObj[0].MapLoop;
      info.AddValue("MapLoop", self.MapLoop);
      info.AddValue("LoadGame",  self.LoadGame);
      info.AddValue("UseAI", self.UseAI);
      info.AddValue("SystemGfx",  self.SystemGfx);
      info.AddValue("ScenarioDir",  self.ScenarioDir);
      info.AddValue("SoundDir",  self.SoundDir);
      info.AddValue("CampaignRoom", self.CampaignRoom);
      info.AddValue("DontShowAIMove", self.DontShowAIMove);
      info.AddValue("StringListCounter", self.StringListCounter);
      info.AddValue("StringIDCounter", self.StringIDCounter);
      info.AddValue("StringListObj",  self.StringListObj);
      info.AddValue("VariantEvent",  self.VariantEvent);
      let mut num: i32 =  7;
      self.Product = !DrawMod.TGame.SuperAdminRights ? num : self.Product;
      info.AddValue("Product", self.Product);
      info.AddValue("SFModelIDCounter", self.SFModelIDCounter);
      info.AddValue("TurnString",  self.TurnString);
      info.AddValue("Loadable", self.Loadable);
      info.AddValue("Verify1", self.Verify1);
      info.AddValue("Verify2", self.Verify2);
      info.AddValue("RuleSetName",  self.RuleSetName);
      info.AddValue("DoAllied", self.DoAllied);
      info.AddValue("PermanentOverlayUse", self.PermanentOverlayUse);
      info.AddValue("PermanentOverlayName",  self.PermanentOverlayName);
      info.AddValue("PbemGameID", self.PbemGameID);
      info.AddValue("PbemPlayer1",  self.PbemPlayer1);
      info.AddValue("PbemPlayer2",  self.PbemPlayer2);
      info.AddValue("PbemGameOver", self.PbemGameOver);
      info.AddValue("PbemDrawGame", self.PbemDrawGame);
      info.AddValue("LibraryCounter", self.LibraryCounter);
      info.AddValue("LibraryObj",  self.LibraryObj);
      info.AddValue("LibVarCounter", self.LibVarCounter);
      info.AddValue("LibVarObj",  self.LibVarObj);
      info.AddValue("SimpleEditor", self.SimpleEditor);
      info.AddValue("RegimeIdCounter", self.RegimeIdCounter);
      info.AddValue("SFTypeIdCounter", self.SFTypeIdCounter);
      info.AddValue("PeopleIdCounter", self.PeopleIdCounter);
      info.AddValue("EventIdCounter", self.EventIdCounter);
      info.AddValue("LocIdCounter", RuntimeHelpers.GetObjectValue(self.LocIdCounter));
      info.AddValue("ExtraTabName",  self.ExtraTabName);
      info.AddValue("ExtraTabEvent", self.ExtraTabEvent);
      info.AddValue("ExtraTabName2",  self.ExtraTabName2);
      info.AddValue("ExtraTabEvent2", self.ExtraTabEvent2);
      info.AddValue("ExtraTabName3",  self.ExtraTabName3);
      info.AddValue("ExtraTabEvent3", self.ExtraTabEvent3);
      info.AddValue("ExtraTabName4",  self.ExtraTabName4);
      info.AddValue("ExtraTabEvent4", self.ExtraTabEvent4);
      info.AddValue("MapName",  self.MapName);
      info.AddValue("MapDesigner",  self.MapDesigner);
      info.AddValue("MapVersion", self.MapVersion);
      info.AddValue("CombatLogId", self.CombatLogId);
      info.AddValue("scenarioVersion",  self.scenarioVersion);
      info.AddValue("scenarioVersionMaster",  self.scenarioVersionMaster);
      info.AddValue("transportMovementType",  self.transportMovementType);
      info.AddValue("se1_earlyCinematicsLogin", self.se1_earlyCinematicsLogin);
      info.AddValue("specialSaveMode", self.specialSaveMode);
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    protected DataClass(SerializationInfo info, StreamingContext context)
    {
      self.GameSlot = new int[500];
      self.GameSlotName = new string[500];
      self.GameSlotShow = new bool[500];
      self.GameSlotShow2 = new bool[500];
      self.RegimeSlotName = new string[500];
      self.RegimeSlotShow = new bool[500];
      self.RegimeSlotShow2 = new int[500];
      self.RegimeSlotNato = new int[500];
      self.RegimeSlotSmallGfx = new int[500];
      self.RealTempString = new string[1000];
      self.GameSlotNato = new int[500];
      self.GameSlotSmallGfx = new int[500];
      self.MapObj = new MapClass[1];
      self.LibraryObj = new LibraryClass[1];
      self.LibVarObj = new LibVarClass[1];
      self.HexObj = new HexClass[1][1];
      self.LandscapeTypeObj = new LandscapeTypeClass[1];
      self.RoadTypeObj = new RoadTypeClass[1];
      self.RegimeObj = new RegimeClass[1];
      self.UnitObj = new UnitClass[1];
      self.SFObj = new SFClass[1];
      self.SFTypeObj = new SFTypeClass[1];
      self.LocTypeObj = new LocationTypeClass[1];
      self.LocObj = new LocationClass[1];
      self.ItemTypeObj = new ItemTypeClass[1];
      self.PeopleObj = new PeopleClass[1];
      self.StringCounter = 1500;
      self.TempString = new string[1501];
      self.RuleVar = new float[1001];
      self.RuleString = new string[1001];
      self.RuleGroup = new int[1001];
      self.RuleGroup2 = new int[1001];
      self.RuleCounter = 1000;
      self.RiverTypeObj = new RiverTypeClass[1];
      self.AreaObj = new AreaClass[1];
      self.HistoricalUnitObj = new HistoricalUnitClass[1];
      self.BridgeObj = new BridgeClass[1];
      self.ActionCardObj = new ActionCardClass[1];
      self.ResearchObj = new ResearchClass[1];
      self.EventObj = new EventClass[1];
      self.EventPicName = new string[1];
      self.EventPicNr = new int[1];
      self.eventPicLibId = new LibIdClass[1];
      self.SmallPicName = new string[1];
      self.SmallPicNr = new int[1];
      self.SmallLibId = new LibIdClass[1];
      self.ReinfName = new string[1];
      self.ReinfLibId = new LibIdClass[1];
      self.ReinfId = new int[1];
      self.StringListObj = new StringListClass[1];
      self.CheckTypeNames = new string[400];
      self.ExecTypeNames = new string[400];
      self.CheckTypeVarName = new string[300][5];
      self.CheckTypeVarCount = new int[300];
      self.CheckDesc = new string[300];
      self.CheckCategory = new int[300];
      self.CheckCategory2 = new int[300];
      self.ExecCategory = new int[400];
      self.ExecCategory2 = new int[400];
      self.ExecTypeVarName = new string[400][5];
      self.ExecTypeVarCount = new int[400];
      self.ExecDesc = new string[400];
      self.ExecTypeString = new int[400];
      self.TempVar = new int[1000];
      self.ExecCategoryName = new string[100];
      self.CheckCategoryName = new string[100];
      self.Variants = new int[12];
      self.VariantEvent = new int[12];
      self.MoveTypePenalty = new int[100];
      self.UnitTypePenalty = new int[100];
      self.WheaterColor = new int[3];
      self.ReinfRatio = new int[50];
      self.transportMovementType = new int[100];
      self.se1_earlyCinematicsLogin = 0;
      self.Version = info.GetInt32(nameof (Version));
      try
      {
        self.Product = info.GetInt32(nameof (Product));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Product = -1;
        ProjectData.ClearProjectError();
      }
      let mut num1: i32 =  7;
      if (!DrawMod.TGame.SuperAdminRights)
      {
        if (num1 != 0)
          ;
        if (num1 == 1 | num1 == 3 && self.Product != num1)
        {
          let mut num2: i32 =   Interaction.MsgBox( "This file could not be loaded. We have to Quit the application due to self. ", Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.EndApp();
        }
        if (num1 == 2 && self.Product == 1 | self.Product == 0)
        {
          if (self.Version >= 185)
          {
            let mut num3: i32 =   Interaction.MsgBox( "This file could not be loaded. We have to Quit the application due to self. ", Title: ( "Shadow Empire : Planetary Conquest"));
            ProjectData.EndApp();
          }
          else
            self.Product = 2;
        }
        if (num1 == 4 && self.Product != 4)
        {
          let mut num4: i32 =   Interaction.MsgBox( "This file might have the propper file extension but is not a real DC3 file. This file could not be loaded. We have to Quit the application due to self. ", Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.EndApp();
        }
        if (num1 == 5 && self.Product != 5)
        {
          let mut num5: i32 =   Interaction.MsgBox( "This file have the propper file extension but is not a real DCX file. This file could not be loaded. We have to Quit the application due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.EndApp();
        }
        if (num1 == 6 && self.Product != 6)
        {
          let mut num6: i32 =   Interaction.MsgBox( "This file have the propper file extension but is not a real DC4 file. This file could not be loaded. We have to Quit the application due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.EndApp();
        }
        if (num1 == 7 && self.Product != 7)
        {
          let mut num7: i32 =   Interaction.MsgBox( "This file have the propper file extension but is not a real SE1 file. This file could not be loaded. We have to Quit the application due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
          ProjectData.EndApp();
        }
      }
      if (self.Version > 424)
      {
        let mut num8: i32 =   Interaction.MsgBox( ("You are loading a scenario or saved game made with version " + Conversion.Str( (self.Version - 314)) + ". Your version is " + Conversion.Str( 110) + ". This could cause errors. You are advised to upgrade to the latest version of this Shadow Empire : Planetary Conquest."), Title: ( "You should upgrade to the latest version!"));
      }
      if (self.Version < 424)
      {
        let mut num9: i32 =   Interaction.MsgBox( ("You are loading a scenario or saved game made with a previous version " + Conversion.Str( (self.Version - 314)) + ". This is probably incompatible with the current game version " + Conversion.Str( 110) + "."));
      }
      DrawMod.TGame.Data.Version = self.Version;
      self.Name = info.GetString(nameof (Name));
      self.Description = info.GetString(nameof (Description));
      self.Turn = info.GetInt32(nameof (Turn));
      self.StepNr = info.GetInt32(nameof (StepNr));
      self.Round = info.GetInt32(nameof (Round));
      if (DrawMod.TGame.Data.Version < 181)
      {
        try
        {
          self.MapCounter = info.GetInt32(nameof (MapCounter));
          self.MapObj = self.MapCounter <= -1 ? new MapClass[1] : new MapClass[self.MapCounter + 1];
          self.MapObj = (MapClass[]) info.GetValue(nameof (MapObj), self.MapObj.GetType());
          try
          {
            self.MapWidth = info.GetInt32(nameof (MapWidth));
            self.MapHeight = info.GetInt32(nameof (MapHeight));
            DrawMod.TGame.Data.MapWidth = self.MapWidth;
            DrawMod.TGame.Data.MapHeight = self.MapHeight;
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            self.MapWidth = 0;
            self.MapHeight = 0;
            ProjectData.ClearProjectError();
          }
        }
        catch (Exception ex1)
        {
          ProjectData.SetProjectError(ex1);
          self.MapCounter = 0;
          self.MapObj = new MapClass[1];
          self.MapWidth = info.GetInt32(nameof (MapWidth));
          self.MapHeight = info.GetInt32(nameof (MapHeight));
          try
          {
            self.MapLoop = (uint) info.GetInt32(nameof (MapLoop)) > 0U;
          }
          catch (Exception ex2)
          {
            ProjectData.SetProjectError(ex2);
            self.MapLoop = false;
            ProjectData.ClearProjectError();
          }
          DrawMod.TGame.Data.MapWidth = self.MapWidth;
          DrawMod.TGame.Data.MapHeight = self.MapHeight;
          self.MapObj[0] = new MapClass(0, self.RegimeCounter, self.MapWidth, self.MapHeight);
          self.MapObj[0].HexObj = new HexClass[self.MapWidth + 1, self.MapHeight + 1];
          self.MapObj[0].HexObj = (HexClass[,]) info.GetValue(nameof (HexObj), self.HexObj.GetType());
          self.MapObj[0].Name = "Default Map";
          self.MapObj[0].MapLoop = self.MapLoop;
          self.MapObj[0].MapWidth = self.MapWidth;
          self.MapObj[0].MapHeight = self.MapHeight;
          ProjectData.ClearProjectError();
        }
      }
      else
      {
        self.MapWidth = info.GetInt32(nameof (MapWidth));
        self.MapHeight = info.GetInt32(nameof (MapHeight));
        DrawMod.TGame.Data.MapWidth = self.MapWidth;
        DrawMod.TGame.Data.MapHeight = self.MapHeight;
        self.MapCounter = info.GetInt32(nameof (MapCounter));
        self.MapObj = self.MapCounter <= -1 ? new MapClass[1] : new MapClass[self.MapCounter + 1];
        self.MapObj = (MapClass[]) info.GetValue(nameof (MapObj), self.MapObj.GetType());
      }
      GC.Collect();
      Application.DoEvents();
      self.ShrowdOn = info.GetBoolean(nameof (ShrowdOn));
      self.FOWOn = info.GetBoolean(nameof (FOWOn));
      try
      {
        self.UncertaintyOn = info.GetBoolean(nameof (UncertaintyOn));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.UncertaintyOn = false;
        ProjectData.ClearProjectError();
      }
      self.ResMod = info.GetInt32(nameof (ResMod));
      if (DrawMod.TGame.Data.Version < 158)
      {
        try
        {
          self.StringListCounter = info.GetInt32(nameof (StringListCounter));
          self.StringIDCounter = info.GetInt32(nameof (StringIDCounter));
          self.StringListObj = new StringListClass[self.StringListCounter + 1];
          self.StringListObj = (StringListClass[]) info.GetValue(nameof (StringListObj), self.StringListObj.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.StringListCounter = -1;
          self.StringIDCounter = 0;
          self.StringListObj = new StringListClass[1];
          ProjectData.ClearProjectError();
        }
      }
      else
      {
        self.StringListCounter = info.GetInt32(nameof (StringListCounter));
        self.StringIDCounter = info.GetInt32(nameof (StringIDCounter));
        self.StringListObj = new StringListClass[self.StringListCounter + 1];
        self.StringListObj = (StringListClass[]) info.GetValue(nameof (StringListObj), self.StringListObj.GetType());
      }
      GC.Collect();
      Application.DoEvents();
      self.LandscapeTypeCounter = info.GetInt32(nameof (LandscapeTypeCounter));
      self.LandscapeTypeObj = self.LandscapeTypeCounter <= -1 ? new LandscapeTypeClass[1] : new LandscapeTypeClass[self.LandscapeTypeCounter + 1];
      self.LandscapeTypeObj = (LandscapeTypeClass[]) info.GetValue(nameof (LandscapeTypeObj), self.LandscapeTypeObj.GetType());
      self.RoadTypeCounter = info.GetInt32(nameof (RoadTypeCounter));
      self.RoadTypeObj = self.RoadTypeCounter <= -1 ? new RoadTypeClass[1] : new RoadTypeClass[self.RoadTypeCounter + 1];
      self.RoadTypeObj = (RoadTypeClass[]) info.GetValue(nameof (RoadTypeObj), self.RoadTypeObj.GetType());
      self.RegimeCounter = info.GetInt32(nameof (RegimeCounter));
      self.RegimeObj = self.RegimeCounter <= -1 ? new RegimeClass[1] : new RegimeClass[self.RegimeCounter + 1];
      GC.Collect();
      Application.DoEvents();
      self.RegimeObj = (RegimeClass[]) info.GetValue(nameof (RegimeObj), self.RegimeObj.GetType());
      self.UnitCounter = info.GetInt32(nameof (UnitCounter));
      self.UnitObj = self.UnitCounter <= -1 ? new UnitClass[1] : new UnitClass[self.UnitCounter + 1];
      self.UnitObj = (UnitClass[]) info.GetValue(nameof (UnitObj), self.UnitObj.GetType());
      self.SFCounter = info.GetInt32(nameof (SFCounter));
      self.SFObj = self.SFCounter <= -1 ? new SFClass[1] : new SFClass[self.SFCounter + 1];
      GC.Collect();
      Application.DoEvents();
      self.SFObj = (SFClass[]) info.GetValue(nameof (SFObj), self.SFObj.GetType());
      self.SFTypeCounter = info.GetInt32(nameof (SFTypeCounter));
      self.SFTypeObj = self.SFTypeCounter <= -1 ? new SFTypeClass[1] : new SFTypeClass[self.SFTypeCounter + 1];
      self.SFTypeObj = (SFTypeClass[]) info.GetValue(nameof (SFTypeObj), self.SFTypeObj.GetType());
      try
      {
        self.MapLoop = info.GetBoolean(nameof (MapLoop));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.MapLoop = false;
        ProjectData.ClearProjectError();
      }
      GC.Collect();
      Application.DoEvents();
      self.StringCounter = info.GetInt32(nameof (StringCounter));
      self.TempString = new string[self.StringCounter + 1];
      self.TempString = (string[]) info.GetValue(nameof (TempString), self.TempString.GetType());
      if (self.StringCounter < 1500)
        self.StringCounter = 1500;
      self.TempString = (string[]) Utils.CopyArray((Array) self.TempString, (Array) new string[self.StringCounter + 1]);
      self.LocTypeCounter = info.GetInt32(nameof (LocTypeCounter));
      self.LocTypeObj = new LocationTypeClass[self.LocTypeCounter + 1];
      self.LocTypeObj = (LocationTypeClass[]) info.GetValue(nameof (LocTypeObj), self.LocTypeObj.GetType());
      self.LocCounter = info.GetInt32(nameof (LocCounter));
      self.LocObj = new LocationClass[self.LocCounter + 1];
      self.LocObj = (LocationClass[]) info.GetValue(nameof (LocObj), self.LocObj.GetType());
      self.ActionCardCounter = info.GetInt32(nameof (ActionCardCounter));
      if (self.ActionCardCounter > -1)
      {
        self.ActionCardObj = new ActionCardClass[self.ActionCardCounter + 1];
        self.ActionCardObj = (ActionCardClass[]) info.GetValue(nameof (ActionCardObj), self.ActionCardObj.GetType());
      }
      else
        self.ActionCardObj = new ActionCardClass[1];
      self.ItemTypeCounter = info.GetInt32(nameof (ItemTypeCounter));
      self.ItemTypeObj = new ItemTypeClass[self.ItemTypeCounter + 1];
      self.ItemTypeObj = (ItemTypeClass[]) info.GetValue(nameof (ItemTypeObj), self.ItemTypeObj.GetType());
      self.PeopleCounter = info.GetInt32(nameof (PeopleCounter));
      self.PeopleObj = new PeopleClass[self.PeopleCounter + 1];
      self.PeopleObj = (PeopleClass[]) info.GetValue(nameof (PeopleObj), self.PeopleObj.GetType());
      self.GameSlot = (int[]) info.GetValue(nameof (GameSlot), self.GameSlot.GetType());
      self.GameSlotName = (string[]) info.GetValue(nameof (GameSlotName), self.GameSlotName.GetType());
      self.RegimeSlotName = (string[]) info.GetValue(nameof (RegimeSlotName), self.RegimeSlotName.GetType());
      self.RiverTypeCounter = info.GetInt32(nameof (RiverTypeCounter));
      self.RiverTypeObj = new RiverTypeClass[self.RiverTypeCounter + 1];
      self.RiverTypeObj = (RiverTypeClass[]) info.GetValue(nameof (RiverTypeObj), self.RiverTypeObj.GetType());
      self.BridgeObj = new BridgeClass[1];
      self.BridgeObj = (BridgeClass[]) info.GetValue(nameof (BridgeObj), self.BridgeObj.GetType());
      GC.Collect();
      Application.DoEvents();
      self.ResearchCounter = info.GetInt32(nameof (ResearchCounter));
      self.ResearchObj = self.ResearchCounter <= -1 ? new ResearchClass[1] : new ResearchClass[self.ResearchCounter + 1];
      self.ResearchObj = (ResearchClass[]) info.GetValue(nameof (ResearchObj), self.ResearchObj.GetType());
      if (DrawMod.TGame.Data.Version < 182)
      {
        try
        {
          self.HistoricalUnitCounter = info.GetInt32(nameof (HistoricalUnitCounter));
          self.HistoricalUnitObj = self.HistoricalUnitCounter <= -1 ? new HistoricalUnitClass[1] : new HistoricalUnitClass[self.HistoricalUnitCounter + 1];
          self.HistoricalUnitObj = (HistoricalUnitClass[]) info.GetValue(nameof (HistoricalUnitObj), self.HistoricalUnitObj.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.HistoricalUnitCounter = -1;
          self.HistoricalUnitObj = new HistoricalUnitClass[1];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.HistoricalIDCounter = info.GetInt32(nameof (HistoricalIDCounter));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.HistoricalIDCounter = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.AreaCounter = info.GetInt32(nameof (AreaCounter));
          self.AreaObj = self.AreaCounter <= -1 ? new AreaClass[1] : new AreaClass[self.AreaCounter + 1];
          self.AreaIDCounter = info.GetInt32(nameof (AreaIDCounter));
          self.AreaObj = (AreaClass[]) info.GetValue(nameof (AreaObj), self.AreaObj.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.AreaCounter = -1;
          self.AreaObj = new AreaClass[1];
          self.AreaIDCounter = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.RegimeSlotShow2 = (int[]) info.GetValue(nameof (RegimeSlotShow2), self.RegimeSlotShow2.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.RegimeSlotShow2 = new int[500];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.RegimeSlotNato = (int[]) info.GetValue("RegimeSlotnato", self.RegimeSlotNato.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.RegimeSlotNato = new int[500];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.GameSlotNato = (int[]) info.GetValue("RegimeSlotnato", self.GameSlotNato.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.GameSlotNato = new int[500];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.RegimeSlotSmallGfx = (int[]) info.GetValue(nameof (RegimeSlotSmallGfx), self.RegimeSlotSmallGfx.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.RegimeSlotSmallGfx = new int[500];
          let mut index: i32 =  0;
          do
          {
            self.RegimeSlotSmallGfx[index] = -1;
            index += 1;
          }
          while (index <= 499);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.GameSlotSmallGfx = (int[]) info.GetValue(nameof (RegimeSlotSmallGfx), self.GameSlotSmallGfx.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.GameSlotSmallGfx = new int[500];
          let mut index: i32 =  0;
          do
          {
            self.GameSlotSmallGfx[index] = -1;
            index += 1;
          }
          while (index <= 499);
          ProjectData.ClearProjectError();
        }
        self.GameSlotShow = (bool[]) info.GetValue(nameof (GameSlotShow), self.GameSlotShow.GetType());
        try
        {
          self.GameSlotShow2 = (bool[]) info.GetValue(nameof (GameSlotShow2), self.GameSlotShow2.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Variants = (int[]) info.GetValue(nameof (Variants), self.Variants.GetType());
          self.VariantEvent = (int[]) info.GetValue(nameof (VariantEvent), self.VariantEvent.GetType());
        }
        catch (Exception ex3)
        {
          ProjectData.SetProjectError(ex3);
          int[] numArray = new int[10];
          self.Variants = new int[10];
          self.VariantEvent = new int[10];
          self.Variants = (int[]) info.GetValue(nameof (Variants), numArray.GetType());
          try
          {
            self.VariantEvent = (int[]) info.GetValue(nameof (VariantEvent), numArray.GetType());
          }
          catch (Exception ex4)
          {
            ProjectData.SetProjectError(ex4);
            let mut index: i32 =  0;
            do
            {
              self.VariantEvent[index] = -1;
              index += 1;
            }
            while (index <= 9);
            ProjectData.ClearProjectError();
          }
          ProjectData.ClearProjectError();
        }
      }
      else
      {
        GC.Collect();
        Application.DoEvents();
        self.HistoricalUnitCounter = info.GetInt32(nameof (HistoricalUnitCounter));
        self.HistoricalUnitObj = self.HistoricalUnitCounter <= -1 ? new HistoricalUnitClass[1] : new HistoricalUnitClass[self.HistoricalUnitCounter + 1];
        self.HistoricalUnitObj = (HistoricalUnitClass[]) info.GetValue(nameof (HistoricalUnitObj), self.HistoricalUnitObj.GetType());
        self.HistoricalIDCounter = info.GetInt32(nameof (HistoricalIDCounter));
        self.AreaCounter = info.GetInt32(nameof (AreaCounter));
        self.AreaObj = self.AreaCounter <= -1 ? new AreaClass[1] : new AreaClass[self.AreaCounter + 1];
        self.AreaIDCounter = info.GetInt32(nameof (AreaIDCounter));
        self.AreaObj = (AreaClass[]) info.GetValue(nameof (AreaObj), self.AreaObj.GetType());
        self.RegimeSlotShow2 = (int[]) info.GetValue(nameof (RegimeSlotShow2), self.RegimeSlotShow2.GetType());
        self.RegimeSlotNato = (int[]) info.GetValue("RegimeSlotnato", self.RegimeSlotNato.GetType());
        self.GameSlotNato = (int[]) info.GetValue("RegimeSlotnato", self.GameSlotNato.GetType());
        try
        {
          self.RegimeSlotSmallGfx = (int[]) info.GetValue(nameof (RegimeSlotSmallGfx), self.RegimeSlotSmallGfx.GetType());
          self.GameSlotSmallGfx = (int[]) info.GetValue(nameof (RegimeSlotSmallGfx), self.GameSlotSmallGfx.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.RegimeSlotSmallGfx = new int[500];
          self.GameSlotSmallGfx = new int[500];
          let mut index: i32 =  0;
          do
          {
            self.RegimeSlotSmallGfx[index] = -1;
            self.GameSlotSmallGfx[index] = -1;
            index += 1;
          }
          while (index <= 499);
          ProjectData.ClearProjectError();
        }
        self.GameSlotShow = (bool[]) info.GetValue(nameof (GameSlotShow), self.GameSlotShow.GetType());
        self.GameSlotShow2 = (bool[]) info.GetValue(nameof (GameSlotShow2), self.GameSlotShow2.GetType());
        self.Variants = (int[]) info.GetValue(nameof (Variants), self.Variants.GetType());
        self.VariantEvent = (int[]) info.GetValue(nameof (VariantEvent), self.VariantEvent.GetType());
      }
      self.EventCounter = info.GetInt32(nameof (EventCounter));
      self.EventObj = self.EventCounter <= -1 ? new EventClass[1] : new EventClass[self.EventCounter + 1];
      self.EventObj = (EventClass[]) info.GetValue(nameof (EventObj), self.EventObj.GetType());
      self.EventPicCounter = info.GetInt32(nameof (EventPicCounter));
      if (self.EventPicCounter > -1)
      {
        self.EventPicName = new string[self.EventPicCounter + 1];
        self.EventPicNr = new int[self.EventPicCounter + 1];
        self.eventPicLibId = new LibIdClass[self.EventPicCounter + 1];
      }
      else
      {
        self.EventPicName = new string[1];
        self.EventPicNr = new int[1];
        self.eventPicLibId = new LibIdClass[1];
      }
      self.EventPicName = (string[]) info.GetValue(nameof (EventPicName), self.EventPicName.GetType());
      try
      {
        self.eventPicLibId = (LibIdClass[]) info.GetValue("EventPicLibId", self.eventPicLibId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.eventPicLibId = self.EventPicCounter <= -1 ? new LibIdClass[1] : new LibIdClass[self.EventPicCounter + 1];
        let mut eventPicCounter: i32 =  self.EventPicCounter;
        for (let mut index: i32 =  0; index <= eventPicCounter; index += 1)
          self.eventPicLibId[index] = LibIdClass::new();
        ProjectData.ClearProjectError();
      }
      try
      {
        self.SmallPicCounter = info.GetInt32(nameof (SmallPicCounter));
        if (self.SmallPicCounter > -1)
        {
          self.SmallPicName = new string[self.SmallPicCounter + 1];
          self.SmallPicNr = new int[self.SmallPicCounter + 1];
          self.SmallLibId = new LibIdClass[self.SmallPicCounter + 1];
        }
        else
        {
          self.SmallPicName = new string[1];
          self.SmallPicNr = new int[1];
          self.SmallLibId = new LibIdClass[1];
        }
        self.SmallPicName = (string[]) info.GetValue(nameof (SmallPicName), self.SmallPicName.GetType());
        self.SmallLibId = (LibIdClass[]) info.GetValue(nameof (SmallLibId), self.SmallLibId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SmallPicCounter = -1;
        self.SmallPicName = new string[1];
        self.SmallPicNr = new int[1];
        self.SmallLibId = new LibIdClass[1];
        ProjectData.ClearProjectError();
      }
      try
      {
        self.ReinfCounter = info.GetInt32(nameof (ReinfCounter));
        self.reinfIdCounter = info.GetInt32("ReinfIdCounter");
        if (self.ReinfCounter > -1)
        {
          self.ReinfName = new string[self.ReinfCounter + 1];
          self.ReinfId = new int[self.ReinfCounter + 1];
          self.ReinfLibId = new LibIdClass[self.ReinfCounter + 1];
          self.ReinfRatio = new int[self.ReinfCounter + 1];
        }
        else
        {
          self.ReinfName = new string[1];
          self.ReinfId = new int[1];
          self.ReinfLibId = new LibIdClass[1];
          self.ReinfRatio = new int[1];
        }
        self.ReinfName = (string[]) info.GetValue(nameof (ReinfName), self.ReinfName.GetType());
        self.ReinfLibId = (LibIdClass[]) info.GetValue(nameof (ReinfLibId), self.ReinfLibId.GetType());
        self.ReinfId = (int[]) info.GetValue(nameof (ReinfId), self.ReinfId.GetType());
        try
        {
          self.ReinfRatio = (int[]) info.GetValue(nameof (ReinfRatio), self.ReinfRatio.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut reinfCounter: i32 =  self.ReinfCounter;
          for (let mut index: i32 =  0; index <= reinfCounter; index += 1)
            self.ReinfRatio[index] = 1;
          ProjectData.ClearProjectError();
        }
        if (self.ReinfRatio.GetUpperBound(0) < self.ReinfName.GetUpperBound(0))
          self.ReinfRatio = (int[]) Utils.CopyArray((Array) self.ReinfRatio, (Array) new int[self.ReinfCounter + 1]);
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.ReinfCounter = -1;
        self.reinfIdCounter = -1;
        self.ReinfName = new string[1];
        self.ReinfId = new int[1];
        self.ReinfLibId = new LibIdClass[1];
        self.ReinfRatio = new int[1];
        ProjectData.ClearProjectError();
      }
      self.LoadPass = info.GetString(nameof (LoadPass));
      self.EditPass = info.GetString(nameof (EditPass));
      self.MasterFile = info.GetString(nameof (MasterFile));
      self.VPWin = info.GetInt32(nameof (VPWin));
      self.Winner = info.GetInt32(nameof (Winner));
      try
      {
        self.LastWinner = info.GetInt32(nameof (LastWinner));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LastWinner = -1;
        ProjectData.ClearProjectError();
      }
      self.RuleVar = (float[]) info.GetValue(nameof (RuleVar), self.RuleVar.GetType());
      self.RuleGroup = (int[]) info.GetValue(nameof (RuleGroup), self.RuleGroup.GetType());
      self.SetDefaultRules(true);
      self.PasswordsOn = info.GetBoolean(nameof (PasswordsOn));
      self.InTurn = info.GetBoolean(nameof (InTurn));
      self.AlternateRound = info.GetInt32(nameof (AlternateRound));
      try
      {
        self.AlternateRound2 = info.GetInt32(nameof (AlternateRound2));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.AlternateRound2 = -1;
        ProjectData.ClearProjectError();
      }
      self.StartData = info.GetDateTime(nameof (StartData));
      self.ResCostMod = info.GetSingle(nameof (ResCostMod));
      self.SupplyMultiplier = info.GetSingle(nameof (SupplyMultiplier));
      self.PPMultiplier = info.GetSingle(nameof (PPMultiplier));
      self.ASOn = info.GetBoolean("AsOn");
      self.RegimeSlotShow = (bool[]) info.GetValue(nameof (RegimeSlotShow), self.RegimeSlotShow.GetType());
      GC.Collect();
      Application.DoEvents();
      self.NoPlayChoice = info.GetBoolean(nameof (NoPlayChoice));
      self.NoAIAdvice = info.GetBoolean(nameof (NoAIAdvice));
      self.PBEM = info.GetBoolean(nameof (PBEM));
      self.ScreenShotOn = info.GetBoolean(nameof (ScreenShotOn));
      self.CreatedWithShrowd = info.GetBoolean(nameof (CreatedWithShrowd));
      self.ShrowdPeek = info.GetBoolean(nameof (ShrowdPeek));
      if (self.Version >= 405)
      {
        try
        {
          self.LibraryCounter = info.GetInt32(nameof (LibraryCounter));
          self.LibraryObj = self.LibraryCounter <= -1 ? new LibraryClass[1] : new LibraryClass[self.LibraryCounter + 1];
          self.LibraryObj = (LibraryClass[]) info.GetValue(nameof (LibraryObj), self.LibraryObj.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.LibraryCounter = -1;
          self.LibraryObj = new LibraryClass[1];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.LibVarCounter = info.GetInt32(nameof (LibVarCounter));
          self.LibVarObj = self.LibVarCounter <= -1 ? new LibVarClass[1] : new LibVarClass[self.LibVarCounter + 1];
          self.LibVarObj = (LibVarClass[]) info.GetValue(nameof (LibVarObj), self.LibVarObj.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.LibVarCounter = -1;
          self.LibVarObj = new LibVarClass[1];
          ProjectData.ClearProjectError();
        }
      }
      else
      {
        self.LibraryCounter = -1;
        self.LibraryObj = new LibraryClass[1];
        self.LibVarCounter = -1;
        self.LibVarObj = new LibVarClass[1];
      }
      if (self.Variants.GetUpperBound(0) <= 9 | self.Version < 170)
      {
        self.Variants = (int[]) Utils.CopyArray((Array) self.Variants, (Array) new int[12]);
        self.VariantEvent = (int[]) Utils.CopyArray((Array) self.VariantEvent, (Array) new int[12]);
        self.VariantEvent[10] = -1;
        self.VariantEvent[11] = -1;
        self.Variants[10] = -1;
        self.Variants[11] = -1;
      }
      if (self.Version < 174)
      {
        self.RuleVar[820] = -1f;
        self.RuleVar[822] = -1f;
      }
      if (self.Version < 203)
      {
        if ( self.RuleVar[857] == 0.0)
          self.RuleVar[857] = self.RuleVar[99];
        if ( self.RuleVar[858] == 0.0)
          self.RuleVar[858] = self.RuleVar[3];
      }
      self.MoveTypePenalty = (int[]) info.GetValue(nameof (MoveTypePenalty), self.MoveTypePenalty.GetType());
      self.UnitTypePenalty = (int[]) info.GetValue(nameof (UnitTypePenalty), self.UnitTypePenalty.GetType());
      self.WheaterColor = (int[]) info.GetValue(nameof (WheaterColor), self.WheaterColor.GetType());
      self.Designer = info.GetString(nameof (Designer));
      self.Designer2 = info.GetString(nameof (Designer2));
      self.MasterfileReadPeople = info.GetBoolean(nameof (MasterfileReadPeople));
      try
      {
        self.SystemGfx = info.GetString(nameof (SystemGfx));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SystemGfx = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.ScenarioDir = info.GetString(nameof (ScenarioDir));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.ScenarioDir = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.SoundDir = info.GetString(nameof (SoundDir));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SoundDir = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.UseAI = info.GetInt32(nameof (UseAI));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.UseAI = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.DontShowAIMove = info.GetBoolean(nameof (DontShowAIMove));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.DontShowAIMove = false;
        ProjectData.ClearProjectError();
      }
      self.SetEventNames();
      try
      {
        self.GameID = info.GetInt32(nameof (GameID));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.GameID = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.LoadGame = info.GetString(nameof (LoadGame));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LoadGame = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.CampaignRoom = info.GetInt32(nameof (CampaignRoom));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.CampaignRoom = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.SFModelIDCounter = info.GetInt32(nameof (SFModelIDCounter));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SFModelIDCounter = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.TurnString = info.GetString(nameof (TurnString));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.TurnString = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Loadable = info.GetBoolean(nameof (Loadable));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Loadable = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.RuleSetName = info.GetString(nameof (RuleSetName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.RuleSetName = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.DoAllied = info.GetBoolean(nameof (DoAllied));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.DoAllied = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.PermanentOverlayUse = info.GetBoolean(nameof (PermanentOverlayUse));
        self.PermanentOverlayName = info.GetString(nameof (PermanentOverlayName));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.PermanentOverlayName = "systemgraphics/trans.bmp";
        self.PermanentOverlayUse = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.PbemGameID = info.GetInt32(nameof (PbemGameID));
        self.PbemPlayer1 = info.GetString(nameof (PbemPlayer1));
        self.PbemPlayer2 = info.GetString(nameof (PbemPlayer2));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.PbemGameID = 0;
        self.PbemPlayer1 = "";
        self.PbemPlayer2 = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.PbemGameOver = info.GetInt32(nameof (PbemGameOver));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.PbemGameOver = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.PbemDrawGame = info.GetInt32(nameof (PbemDrawGame));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.PbemDrawGame = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.SimpleEditor = info.GetBoolean(nameof (SimpleEditor));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SimpleEditor = false;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.RegimeIdCounter = info.GetInt32(nameof (RegimeIdCounter));
        self.SFTypeIdCounter = info.GetInt32(nameof (SFTypeIdCounter));
        self.PeopleIdCounter = info.GetInt32(nameof (PeopleIdCounter));
        self.EventIdCounter = info.GetInt32(nameof (EventIdCounter));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.RegimeIdCounter = -1;
        self.SFTypeIdCounter = -1;
        self.PeopleIdCounter = -1;
        self.EventIdCounter = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.LocIdCounter =  info.GetInt32("locIdCounter");
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LocIdCounter =  -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.ExtraTabName = info.GetString(nameof (ExtraTabName));
        self.ExtraTabEvent = info.GetInt32(nameof (ExtraTabEvent));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.ExtraTabEvent = -1;
        self.ExtraTabName = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.ExtraTabName2 = info.GetString(nameof (ExtraTabName2));
        self.ExtraTabEvent2 = info.GetInt32(nameof (ExtraTabEvent2));
        self.ExtraTabName3 = info.GetString(nameof (ExtraTabName3));
        self.ExtraTabEvent3 = info.GetInt32(nameof (ExtraTabEvent3));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.ExtraTabEvent2 = -1;
        self.ExtraTabName2 = "";
        self.ExtraTabEvent3 = -1;
        self.ExtraTabName3 = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.ExtraTabName4 = info.GetString(nameof (ExtraTabName4));
        self.ExtraTabEvent4 = info.GetInt32(nameof (ExtraTabEvent4));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.ExtraTabEvent4 = -1;
        self.ExtraTabName4 = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.MapName = info.GetString(nameof (MapName));
        self.MapDesigner = info.GetString(nameof (MapDesigner));
        self.MapVersion = info.GetInt32(nameof (MapVersion));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.MapName = "Unnamed".to_owned();
        self.MapDesigner = "Unkown".to_owned();
        self.MapVersion = 1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.CombatLogId = info.GetInt32(nameof (CombatLogId));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.CombatLogId = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.scenarioVersion = info.GetString(nameof (scenarioVersion));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.scenarioVersion = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.scenarioVersionMaster = info.GetString(nameof (scenarioVersionMaster));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.scenarioVersionMaster = "";
        ProjectData.ClearProjectError();
      }
      try
      {
        self.transportMovementType = (int[]) info.GetValue(nameof (transportMovementType), self.transportMovementType.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        let mut index: i32 =  0;
        do
        {
          self.transportMovementType[index] = 0;
          index += 1;
        }
        while (index <= 99);
        ProjectData.ClearProjectError();
      }
      try
      {
        self.se1_earlyCinematicsLogin = info.GetInt32(nameof (se1_earlyCinematicsLogin));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.se1_earlyCinematicsLogin = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.specialSaveMode = info.GetInt32(nameof (specialSaveMode));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.specialSaveMode = 0;
        ProjectData.ClearProjectError();
      }
      if ( self.RuleVar[316] == 0.0)
        self.RuleVar[316] = 1f;
      if ( self.RuleVar[317] == 0.0)
        self.RuleVar[317] = 1f;
      if ( self.RuleVar[319] == 0.0)
        self.RuleVar[319] = 1f;
      if ( DrawMod.TGame.Data.RuleVar[344] == 1.0 & DrawMod.TGame.EditObj.HideUnit == 0)
        DrawMod.TGame.EditObj.HideUnit = 2;
      let mut index1: i32 =  440;
      do
      {
        if (Operators.CompareString(self.RuleString[index1], "", false) == 0)
          self.RuleVar[index1] = 0.0f;
        index1 += 1;
      }
      while (index1 <= 500);
      self.Version = 424;
    }

    pub fn serialize(fileloc: String)
    {
      FileStream serializationStream = new FileStream(fileloc, FileMode.Create, FileAccess.Write, FileShare.ReadWrite);
      GC.Collect();
      Application.DoEvents();
      BinaryFormatter::new().Serialize((Stream) serializationStream,  this);
      serializationStream.Close();
      GC.Collect();
      Application.DoEvents();
    }

    pub static deserialize: DataClass(fileloc: String)
    {
      FileStream serializationStream = new FileStream(fileloc, FileMode.Open, FileAccess.Read, FileShare.None);
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      GC.Collect();
      Application.DoEvents();
      dataClass1: DataClass = (DataClass) binaryFormatter.Deserialize((Stream) serializationStream);
      serializationStream.Flush();
      serializationStream.Close();
      if ( dataClass1.RuleVar[306] == 0.0)
        dataClass1.RuleVar[306] = 0.3f;
      if (Operators.CompareString(dataClass1.Designer, dataClass1.Designer2, false) == 0)
        dataClass1.Designer2 = "";
      let mut sfTypeCounter1: i32 =  dataClass1.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter1; index += 1)
      {
        if (dataClass1.SFTypeObj[index].ModelLastState.GetUpperBound(0) < dataClass1.ResearchCounter)
        {
          dataClass1.SFTypeObj[index].ModelLastState = (int[]) Utils.CopyArray((Array) dataClass1.SFTypeObj[index].ModelLastState, (Array) new int[dataClass1.ResearchCounter + 1]);
          dataClass1.SFTypeObj[index].ModelPossibleImp = (int[]) Utils.CopyArray((Array) dataClass1.SFTypeObj[index].ModelPossibleImp, (Array) new int[dataClass1.ResearchCounter + 1]);
          dataClass1.SFTypeObj[index].ModelImproveEvent = (int[]) Utils.CopyArray((Array) dataClass1.SFTypeObj[index].ModelImproveEvent, (Array) new int[dataClass1.ResearchCounter + 1]);
          dataClass1.SFTypeObj[index].ModelAutoImprovement = (bool[]) Utils.CopyArray((Array) dataClass1.SFTypeObj[index].ModelAutoImprovement, (Array) new bool[dataClass1.ResearchCounter + 1]);
        }
      }
      let mut locTypeCounter: i32 =  dataClass1.LocTypeCounter;
      for (let mut index1: i32 =  0; index1 <= locTypeCounter; index1 += 1)
      {
        let mut index2: i32 =  0;
        do
        {
          if (dataClass1.LocTypeObj[index1].Research[index2] > dataClass1.ResearchCounter)
            dataClass1.LocTypeObj[index1].Research[index2] = -1;
          index2 += 1;
        }
        while (index2 <= 4);
      }
      if (dataClass1.RegimeIdCounter == -1)
      {
        dataClass1.RegimeIdCounter = 0;
        let mut regimeCounter: i32 =  dataClass1.RegimeCounter;
        for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        {
          if (dataClass1.RegimeObj[index].id >= dataClass1.RegimeIdCounter)
            dataClass1.RegimeIdCounter = dataClass1.RegimeObj[index].id;
        }
      }
      let mut regimeCounter1: i32 =  dataClass1.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter1; index += 1)
      {
        if (dataClass1.RegimeObj[index].id == -1)
        {
          dataClass1 += 1.RegimeIdCounter;
          dataClass1.RegimeObj[index].id = dataClass1.RegimeIdCounter;
        }
      }
      if (dataClass1.PeopleIdCounter == -1)
      {
        dataClass1.PeopleIdCounter = 0;
        let mut peopleCounter: i32 =  dataClass1.PeopleCounter;
        for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
        {
          if (dataClass1.PeopleObj[index].id >= dataClass1.PeopleIdCounter)
            dataClass1.PeopleIdCounter = dataClass1.PeopleObj[index].id;
        }
      }
      let mut peopleCounter1: i32 =  dataClass1.PeopleCounter;
      for (let mut index: i32 =  0; index <= peopleCounter1; index += 1)
      {
        if (dataClass1.PeopleObj[index].id == -1)
        {
          dataClass1 += 1.PeopleIdCounter;
          dataClass1.PeopleObj[index].id = dataClass1.PeopleIdCounter;
        }
      }
      if (dataClass1.EventIdCounter == -1)
      {
        dataClass1.EventIdCounter = 0;
        let mut eventCounter: i32 =  dataClass1.EventCounter;
        for (let mut index: i32 =  0; index <= eventCounter; index += 1)
        {
          if (dataClass1.EventObj[index].Id >= dataClass1.EventIdCounter)
            dataClass1.EventIdCounter = dataClass1.EventObj[index].Id;
        }
      }
      let mut eventCounter1: i32 =  dataClass1.EventCounter;
      for (let mut index: i32 =  0; index <= eventCounter1; index += 1)
      {
        if (dataClass1.EventObj[index].Id == -1)
        {
          dataClass1 += 1.EventIdCounter;
          dataClass1.EventObj[index].Id = dataClass1.EventIdCounter;
        }
      }
      if (Operators.ConditionalCompareObjectEqual(dataClass1.LocIdCounter,  -1, false))
      {
        dataClass1.LocIdCounter =  0;
        let mut locCounter: i32 =  dataClass1.LocCounter;
        for (let mut index: i32 =  0; index <= locCounter; index += 1)
        {
          if (Operators.ConditionalCompareObjectGreaterEqual( dataClass1.LocObj[index].ID, dataClass1.LocIdCounter, false))
            dataClass1.LocIdCounter =  dataClass1.LocObj[index].ID;
        }
      }
      let mut locCounter1: i32 =  dataClass1.LocCounter;
      for (let mut index: i32 =  0; index <= locCounter1; index += 1)
      {
        if (dataClass1.LocObj[index].ID == -1)
        {
          dataClass2: DataClass = dataClass1;
          dataClass2.LocIdCounter = Operators.AddObject(dataClass2.LocIdCounter,  1);
          dataClass1.LocObj[index].ID = Conversions.ToInteger(dataClass1.LocIdCounter);
        }
      }
      if (dataClass1.SFTypeIdCounter == -1)
      {
        dataClass1.SFTypeIdCounter = 0;
        let mut sfTypeCounter2: i32 =  dataClass1.SFTypeCounter;
        for (let mut index: i32 =  0; index <= sfTypeCounter2; index += 1)
        {
          if (dataClass1.SFTypeObj[index].Id >= dataClass1.SFTypeIdCounter)
            dataClass1.SFTypeIdCounter = dataClass1.SFTypeObj[index].Id;
        }
      }
      let mut sfTypeCounter3: i32 =  dataClass1.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter3; index += 1)
      {
        if (dataClass1.SFTypeObj[index].Id == -1)
        {
          dataClass1 += 1.SFTypeIdCounter;
          dataClass1.SFTypeObj[index].Id = dataClass1.SFTypeIdCounter;
        }
      }
      if (dataClass1.reinfIdCounter == -1)
        dataClass1.reinfIdCounter = 0;
      let mut reinfCounter1: i32 =  dataClass1.ReinfCounter;
      for (let mut index: i32 =  0; index <= reinfCounter1; index += 1)
      {
        if (dataClass1.ReinfId[index] > dataClass1.reinfIdCounter)
          dataClass1.reinfIdCounter = dataClass1.ReinfId[index];
      }
      if (dataClass1.ReinfCounter == -1)
      {
        let mut sfTypeCounter4: i32 =  dataClass1.SFTypeCounter;
        for (let mut index3: i32 =  0; index3 <= sfTypeCounter4; index3 += 1)
        {
          if (dataClass1.SFTypeObj[index3].ReinforcementType > -1)
          {
            str: String = dataClass1.TempString[750 + dataClass1.SFTypeObj[index3].ReinforcementType];
            bool flag = false;
            let mut reinfCounter2: i32 =  dataClass1.ReinfCounter;
            for (let mut index4: i32 =  0; index4 <= reinfCounter2; index4 += 1)
            {
              if (Operators.CompareString(dataClass1.ReinfName[index4], str, false) == 0)
              {
                flag = true;
                dataClass1.SFTypeObj[index3].ReinforcementType = index4;
                break;
              }
            }
            if (!flag)
            {
              dataClass1.AddReinf(str);
              dataClass1.SFTypeObj[index3].ReinforcementType = dataClass1.ReinfCounter;
            }
          }
          if (dataClass1.SFTypeObj[index3].ReinforcementType2 > -1)
          {
            str: String = dataClass1.TempString[750 + dataClass1.SFTypeObj[index3].ReinforcementType2];
            bool flag = false;
            let mut reinfCounter3: i32 =  dataClass1.ReinfCounter;
            for (let mut index5: i32 =  0; index5 <= reinfCounter3; index5 += 1)
            {
              if (Operators.CompareString(dataClass1.ReinfName[index5], str, false) == 0)
              {
                flag = true;
                dataClass1.SFTypeObj[index3].ReinforcementType2 = index5;
                break;
              }
            }
            if (!flag)
            {
              dataClass1.AddReinf(str);
              dataClass1.SFTypeObj[index3].ReinforcementType2 = dataClass1.ReinfCounter;
            }
          }
          if (dataClass1.SFTypeObj[index3].ReinforcementType3 > -1)
          {
            str: String = dataClass1.TempString[750 + dataClass1.SFTypeObj[index3].ReinforcementType3];
            bool flag = false;
            let mut reinfCounter4: i32 =  dataClass1.ReinfCounter;
            for (let mut index6: i32 =  0; index6 <= reinfCounter4; index6 += 1)
            {
              if (Operators.CompareString(dataClass1.ReinfName[index6], str, false) == 0)
              {
                flag = true;
                dataClass1.SFTypeObj[index3].ReinforcementType3 = index6;
                break;
              }
            }
            if (!flag)
            {
              dataClass1.AddReinf(str);
              dataClass1.SFTypeObj[index3].ReinforcementType3 = dataClass1.ReinfCounter;
            }
          }
        }
        let mut index: i32 =  750;
        do
        {
          dataClass1.TempString[index] = "";
          index += 1;
        }
        while (index <= 799);
      }
      if (dataClass1.Product == -1 & dataClass1.Version < 130)
        dataClass1.RuleSetName = "Imported old AT scenario";
      if (dataClass1.Product == 4 & Strings.InStr(dataClass1.Name.ToLower(), "barbarossa") > 0)
        dataClass1.RuleVar[999] = 1f;
      if (dataClass1.Product <= 5)
      {
        let mut index: i32 =  401;
        do
        {
          dataClass1.RuleVar[index] = 0.0f;
          index += 1;
        }
        while (index <= 499);
        dataClass1.RuleVar[540] = 0.0f;
      }
      return dataClass1;
    }

    pub fn LoadGraphics(tformref: Form1)
    {
      BitmapStore.FlagForDelete();
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading Scn Gfx 4/6";
      Application.DoEvents();
      if (self.LandscapeTypeCounter > -1)
      {
        let mut landscapeTypeCounter: i32 =  self.LandscapeTypeCounter;
        for (let mut Number: i32 =  0; Number <= landscapeTypeCounter; Number += 1)
        {
          self.LandscapeTypeObj[Number].LoadSprites();
          str: String = Strings.Trim(Conversion.Str( Number)) + " of " + Strings.Trim(Conversion.Str( self.LandscapeTypeCounter));
          if (!Information.IsNothing( tformref))
            tformref.Label1.Text = "Loading Scn Gfx 4/6, step " + str;
          Application.DoEvents();
        }
      }
      if (self.RoadTypeCounter > -1)
      {
        let mut roadTypeCounter: i32 =  self.RoadTypeCounter;
        for (let mut index: i32 =  0; index <= roadTypeCounter; index += 1)
          self.RoadTypeObj[index].LoadSprites();
      }
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading Scn Gfx 5/6";
      Application.DoEvents();
      if (self.SFTypeCounter > -1)
      {
        let mut sfTypeCounter: i32 =  self.SFTypeCounter;
        for (let mut Number: i32 =  0; Number <= sfTypeCounter; Number += 1)
        {
          str: String = Strings.Trim(Conversion.Str( Number)) + " of " + Strings.Trim(Conversion.Str( self.SFTypeCounter));
          if (!Information.IsNothing( tformref))
            tformref.Label1.Text = "Loading Scn Gfx 5/6, step " + str;
          Application.DoEvents();
          self.SFTypeObj[Number].LoadSprites();
        }
      }
      if (!Information.IsNothing( tformref))
        tformref.Label1.Text = "Loading Scn Gfx 6/6";
      Application.DoEvents();
      if (self.RegimeCounter > -1)
      {
        let mut regimeCounter: i32 =  self.RegimeCounter;
        for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
          self.RegimeObj[index].LoadSprites();
      }
      if (self.RiverTypeCounter > -1)
      {
        let mut riverTypeCounter: i32 =  self.RiverTypeCounter;
        for (let mut index: i32 =  0; index <= riverTypeCounter; index += 1)
          self.RiverTypeObj[index].LoadSprites();
      }
      if (self.PeopleCounter > -1)
      {
        let mut peopleCounter: i32 =  self.PeopleCounter;
        for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
          self.PeopleObj[index].LoadSprites();
      }
      if (self.HistoricalUnitCounter > -1)
      {
        let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
        for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
          self.HistoricalUnitObj[index].LoadSprites();
      }
      self.BridgeObj[0].LoadSprites();
      self.EventPicLoadSprites();
      self.SmallPicLoadSprites();
      self.LoadSprites();
      BitmapStore.DeleteFlaggedBitmaps();
    }

    pub fn LoadSprites()
    {
      if (self.PermanentOverlayUse)
      {
        if (self.PermanentOverlaySpriteID > 0)
          BitmapStore.ReloadFile(self.PermanentOverlaySpriteID, self.PermanentOverlayName);
        else
          self.PermanentOverlaySpriteID = BitmapStore.AddFile(self.PermanentOverlayName, false);
      }
      else
        self.PermanentOverlaySpriteID = -1;
    }

    pub fn EventPicKill()
    {
      let mut eventPicCounter: i32 =  self.EventPicCounter;
      num1: i32;
      let mut num2: i32 =  num1;
      for (let mut index: i32 =  eventPicCounter; index >= num2; index += -1)
        BitmapStore.RemoveBitmapNr(self.EventPicNr[index]);
    }

    pub fn EventPicReplaceprite(nr: i32, filename: String)
    {
      self.EventPicName[nr] = filename;
      self.EventPicNr[nr] = BitmapStore.ReloadFile(self.EventPicNr[nr], filename);
    }

    pub fn EventPicLoadSprites()
    {
      if (self.EventPicCounter == -1)
        return;
      let mut eventPicCounter: i32 =  self.EventPicCounter;
      for (let mut index: i32 =  0; index <= eventPicCounter; index += 1)
        self.EventPicNr[index] = BitmapStore.AddFile(self.EventPicName[index], false);
    }

    pub fn RemoveEventPic(nr: i32)
    {
      self.EventChangeEventNr(nr, -1);
      if (nr < self.EventPicCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.EventPicCounter - 1;
        for (let mut newnr: i32 =  num1; newnr <= num2; newnr += 1)
        {
          self.EventPicName[newnr] = self.EventPicName[newnr + 1];
          self.EventPicNr[newnr] = self.EventPicNr[newnr + 1];
          self.eventPicLibId[newnr] = self.eventPicLibId[newnr + 1];
          self.EventChangeEventNr(newnr + 1, newnr);
        }
        --self.EventPicCounter;
        self.EventPicName = (string[]) Utils.CopyArray((Array) self.EventPicName, (Array) new string[self.EventPicCounter + 1]);
        self.EventPicNr = (int[]) Utils.CopyArray((Array) self.EventPicNr, (Array) new int[self.EventPicCounter + 1]);
      }
      else
      {
        self.EventPicName[nr] = "";
        self.EventPicNr[nr] = -1;
        --self.EventPicCounter;
      }
    }

    pub fn FindEventPic(teventpicName: String, eventPicOrigSlot: i32, libname: String) -> i32
    {
      let mut eventPicCounter: i32 =  self.EventPicCounter;
      for (let mut eventPic: i32 =  0; eventPic <= eventPicCounter; eventPic += 1)
      {
        if (self.eventPicLibId[eventPic].libSlot > -1 && Operators.CompareString(self.LibraryObj[self.eventPicLibId[eventPic].libSlot].name, libname, false) == 0 && (Operators.CompareString(self.EventPicName[eventPic], teventpicName, false) == 0 | Operators.CompareString(teventpicName, "", false) == 0) & self.eventPicLibId[eventPic].id == eventPicOrigSlot)
          return eventPic;
      }
      return -1;
    }

    pub fn FindEventPic(eventPicOrigSlot: i32, libname: String) -> i32
    {
      let mut eventPicCounter: i32 =  self.EventPicCounter;
      for (let mut eventPic: i32 =  0; eventPic <= eventPicCounter; eventPic += 1)
      {
        if (self.eventPicLibId[eventPic].libSlot > -1 && Operators.CompareString(self.LibraryObj[self.eventPicLibId[eventPic].libSlot].name, libname, false) == 0 && self.eventPicLibId[eventPic].id == eventPicOrigSlot)
          return eventPic;
      }
      return -1;
    }

    pub fn AddEventPic(filename: String)
    {
      this += 1.EventPicCounter;
      self.EventPicName = (string[]) Utils.CopyArray((Array) self.EventPicName, (Array) new string[self.EventPicCounter + 1]);
      self.EventPicNr = (int[]) Utils.CopyArray((Array) self.EventPicNr, (Array) new int[self.EventPicCounter + 1]);
      self.eventPicLibId = (LibIdClass[]) Utils.CopyArray((Array) self.eventPicLibId, (Array) new LibIdClass[self.EventPicCounter + 1]);
      self.EventPicName[self.EventPicCounter] = filename;
      self.eventPicLibId[self.EventPicCounter] = LibIdClass::new();
      self.EventPicNr[self.EventPicCounter] = BitmapStore.AddFile(filename, false);
    }

    pub fn EventChangeEventNr(oldnr: i32, newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      index1: i32;
      for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
      {
        let mut width: i32 =  self.StringListObj[index2].Width;
        for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
        {
          let mut length: i32 =  self.StringListObj[index2].Length;
          for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
          {
            if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.EventPicId && self.StringListObj[index2].Data[index4, index3].Length > 0)
            {
              index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
              if (index1 == oldnr)
                index1 = newnr;
              self.StringListObj[index2].Data[index4, index3] = index1.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.EventPicId & self.LibVarObj[libVarCounter].value == oldnr)
        {
          if (newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = newnr;
        }
      }
      let mut actionCardCounter: i32 =  self.ActionCardCounter;
      for (index1 = 0; index1 <= actionCardCounter; index1 += 1)
      {
        if (self.ActionCardObj[index1].EventPicNr == oldnr)
          self.ActionCardObj[index1].EventPicNr = newnr;
        if (self.ActionCardObj[index1].AlternateEventPicNr == oldnr)
          self.ActionCardObj[index1].AlternateEventPicNr = newnr;
      }
    }

    pub fn SmallPicKill()
    {
      let mut smallPicCounter: i32 =  self.SmallPicCounter;
      num1: i32;
      let mut num2: i32 =  num1;
      for (let mut index: i32 =  smallPicCounter; index >= num2; index += -1)
        BitmapStore.RemoveBitmapNr(self.SmallPicNr[index]);
    }

    pub fn SmallPicReplaceprite(nr: i32, filename: String)
    {
      self.SmallPicName[nr] = filename;
      self.SmallPicNr[nr] = BitmapStore.ReloadFile(self.SmallPicNr[nr], filename, IsBig: true);
    }

    pub fn SmallPicLoadSprites()
    {
      if (self.SmallPicCounter == -1)
        return;
      let mut smallPicCounter: i32 =  self.SmallPicCounter;
      for (let mut index: i32 =  0; index <= smallPicCounter; index += 1)
        self.SmallPicNr[index] = BitmapStore.AddFile(self.SmallPicName[index], false, true);
    }

    pub fn RemoveSmallPic(nr: i32)
    {
      self.SmallChangeEventNr(nr, -1);
      if (nr < self.SmallPicCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.SmallPicCounter - 1;
        for (let mut newnr: i32 =  num1; newnr <= num2; newnr += 1)
        {
          self.SmallPicName[newnr] = self.SmallPicName[newnr + 1];
          self.SmallPicNr[newnr] = self.SmallPicNr[newnr + 1];
          self.SmallLibId[newnr] = self.SmallLibId[newnr + 1];
          self.SmallChangeEventNr(newnr + 1, newnr);
        }
        --self.SmallPicCounter;
        self.SmallPicName = (string[]) Utils.CopyArray((Array) self.SmallPicName, (Array) new string[self.SmallPicCounter + 1]);
        self.SmallPicNr = (int[]) Utils.CopyArray((Array) self.SmallPicNr, (Array) new int[self.SmallPicCounter + 1]);
        self.SmallLibId = (LibIdClass[]) Utils.CopyArray((Array) self.SmallLibId, (Array) new LibIdClass[self.SmallPicCounter + 1]);
      }
      else
      {
        self.SmallPicName[nr] = "";
        self.SmallPicNr[nr] = -1;
        --self.SmallPicCounter;
      }
    }

    pub fn RemoveReinf(nr: i32)
    {
      self.ReinfChangeEventNr(nr, -1);
      if (nr < self.ReinfCounter)
      {
        let mut num1: i32 =  nr;
        let mut num2: i32 =  self.ReinfCounter - 1;
        for (let mut newnr: i32 =  num1; newnr <= num2; newnr += 1)
        {
          self.ReinfName[newnr] = self.ReinfName[newnr + 1];
          self.ReinfId[newnr] = self.ReinfId[newnr + 1];
          self.ReinfLibId[newnr] = self.ReinfLibId[newnr + 1];
          self.ReinfRatio[newnr] = self.ReinfRatio[newnr + 1];
          self.ReinfChangeEventNr(newnr + 1, newnr);
        }
        --self.ReinfCounter;
        self.ReinfName = (string[]) Utils.CopyArray((Array) self.ReinfName, (Array) new string[self.ReinfCounter + 1]);
        self.ReinfId = (int[]) Utils.CopyArray((Array) self.ReinfId, (Array) new int[self.ReinfCounter + 1]);
        self.ReinfLibId = (LibIdClass[]) Utils.CopyArray((Array) self.ReinfLibId, (Array) new LibIdClass[self.ReinfCounter + 1]);
        self.ReinfRatio = (int[]) Utils.CopyArray((Array) self.ReinfRatio, (Array) new int[self.ReinfCounter + 1]);
      }
      else
      {
        self.ReinfName[nr] = "";
        self.ReinfId[nr] = -1;
        self.ReinfLibId[nr] = LibIdClass::new();
        --self.ReinfCounter;
        self.ReinfRatio[nr] = 1;
      }
    }

    pub fn FindSmallPic(tsmallpicName: String, smallId: i32, libname: String) -> i32
    {
      let mut smallPicCounter: i32 =  self.SmallPicCounter;
      for (let mut smallPic: i32 =  0; smallPic <= smallPicCounter; smallPic += 1)
      {
        if (self.SmallLibId[smallPic].libSlot > -1 && Operators.CompareString(self.LibraryObj[self.SmallLibId[smallPic].libSlot].name, libname, false) == 0 && Operators.CompareString(self.SmallPicName[smallPic], tsmallpicName, false) == 0 & self.SmallLibId[smallPic].id == smallId)
          return smallPic;
      }
      return -1;
    }

    pub fn FindSmallPic(smallId: i32, libname: String) -> i32
    {
      let mut smallPicCounter: i32 =  self.SmallPicCounter;
      for (let mut smallPic: i32 =  0; smallPic <= smallPicCounter; smallPic += 1)
      {
        if (self.SmallLibId[smallPic].libSlot > -1 && Operators.CompareString(self.LibraryObj[self.SmallLibId[smallPic].libSlot].name, libname, false) == 0 && self.SmallLibId[smallPic].id == smallId)
          return smallPic;
      }
      return -1;
    }

    pub fn FindReinf(treinfName: String, reinfId: i32, libname: String) -> i32
    {
      let mut reinfCounter: i32 =  self.ReinfCounter;
      for (let mut reinf: i32 =  0; reinf <= reinfCounter; reinf += 1)
      {
        if (self.ReinfLibId[reinf].libSlot > -1 && Operators.CompareString(self.LibraryObj[self.ReinfLibId[reinf].libSlot].name, libname, false) == 0 && ((Operators.CompareString(treinfName, "", false) == 0 | Operators.CompareString(self.ReinfName[reinf], treinfName, false) == 0) & self.ReinfLibId[reinf].id == reinfId || Operators.CompareString(treinfName, "", false) == 0 & self.ReinfLibId[reinf].id == reinfId))
          return reinf;
      }
      return -1;
    }

    pub fn AddSmallPic(filename: String)
    {
      num: i32;
      num += 1;
      this += 1.SmallPicCounter;
      self.SmallPicName = (string[]) Utils.CopyArray((Array) self.SmallPicName, (Array) new string[self.SmallPicCounter + 1]);
      self.SmallPicNr = (int[]) Utils.CopyArray((Array) self.SmallPicNr, (Array) new int[self.SmallPicCounter + 1]);
      self.SmallLibId = (LibIdClass[]) Utils.CopyArray((Array) self.SmallLibId, (Array) new LibIdClass[self.SmallPicCounter + 1]);
      self.SmallLibId[self.SmallPicCounter] = LibIdClass::new();
      self.SmallPicName[self.SmallPicCounter] = filename;
      self.SmallPicNr[self.SmallPicCounter] = BitmapStore.AddFile(filename, false, true);
    }

    pub fn AddReinf(name: String)
    {
      num: i32;
      num += 1;
      this += 1.ReinfCounter;
      self.ReinfName = (string[]) Utils.CopyArray((Array) self.ReinfName, (Array) new string[self.ReinfCounter + 1]);
      self.ReinfId = (int[]) Utils.CopyArray((Array) self.ReinfId, (Array) new int[self.ReinfCounter + 1]);
      self.ReinfLibId = (LibIdClass[]) Utils.CopyArray((Array) self.ReinfLibId, (Array) new LibIdClass[self.ReinfCounter + 1]);
      self.ReinfRatio = (int[]) Utils.CopyArray((Array) self.ReinfRatio, (Array) new int[self.ReinfCounter + 1]);
      self.ReinfLibId[self.ReinfCounter] = LibIdClass::new();
      this += 1.reinfIdCounter;
      self.ReinfRatio[self.ReinfCounter] = 1;
      self.ReinfName[self.ReinfCounter] = name;
      self.ReinfId[self.ReinfCounter] = self.reinfIdCounter;
    }

    pub fn SmallChangeEventNr(oldnr: i32, newnr: i32)
    {
      let mut stringListCounter: i32 =  self.StringListCounter;
      index1: i32;
      for (let mut index2: i32 =  0; index2 <= stringListCounter; index2 += 1)
      {
        let mut width: i32 =  self.StringListObj[index2].Width;
        for (let mut index3: i32 =  0; index3 <= width; index3 += 1)
        {
          let mut length: i32 =  self.StringListObj[index2].Length;
          for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
          {
            if (self.StringListObj[index2].ColValueType[index3] == NewEnums.LibVarValueType.SmallGfxId && self.StringListObj[index2].Data[index4, index3].Length > 0)
            {
              index1 =  Math.Round(Conversion.Val(self.StringListObj[index2].Data[index4, index3]));
              if (index1 == oldnr)
                index1 = newnr;
              self.StringListObj[index2].Data[index4, index3] = index1.ToString();
            }
          }
        }
      }
      for (let mut libVarCounter: i32 =  self.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
      {
        if (self.LibVarObj[libVarCounter].instanceId.id > -1 & self.LibVarObj[libVarCounter].valueType == NewEnums.LibVarValueType.SmallGfxId & self.LibVarObj[libVarCounter].value == oldnr)
        {
          if (newnr == -1)
            self.RemoveLibVar(libVarCounter);
          else
            self.LibVarObj[libVarCounter].value = newnr;
        }
      }
      let mut historicalUnitCounter: i32 =  self.HistoricalUnitCounter;
      for (index1 = 0; index1 <= historicalUnitCounter; index1 += 1)
      {
        if (self.HistoricalUnitObj[index1].SmallGfx == oldnr)
          self.HistoricalUnitObj[index1].SmallGfx = newnr;
        let mut index5: i32 =  0;
        do
        {
          if (self.HistoricalUnitObj[index1].DesignationSmall[index5] == oldnr)
            self.HistoricalUnitObj[index1].DesignationSmall[index5] = newnr;
          index5 += 1;
        }
        while (index5 <= 9);
        let mut hisVarCount: i32 =  self.HistoricalUnitObj[index1].HisVarCount;
        for (let mut index6: i32 =  0; index6 <= hisVarCount; index6 += 1)
        {
          if (self.HistoricalUnitObj[index1].HisVarSmall[index6] == oldnr)
            self.HistoricalUnitObj[index1].HisVarSmall[index6] = newnr;
        }
      }
      index1 = 0;
      do
      {
        if (self.RegimeSlotSmallGfx[index1] == oldnr)
          self.RegimeSlotSmallGfx[index1] = newnr;
        if (self.GameSlotSmallGfx[index1] == oldnr)
          self.GameSlotSmallGfx[index1] = newnr;
        index1 += 1;
      }
      while (index1 <= 499);
      let mut actionCardCounter: i32 =  self.ActionCardCounter;
      for (index1 = 0; index1 <= actionCardCounter; index1 += 1)
      {
        if (self.ActionCardObj[index1].SmallGfx == oldnr)
          self.ActionCardObj[index1].SmallGfx = newnr;
      }
      let mut locTypeCounter: i32 =  self.LocTypeCounter;
      for (index1 = 0; index1 <= locTypeCounter; index1 += 1)
      {
        if (self.LocTypeObj[index1].SmallGraphic == oldnr)
          self.LocTypeObj[index1].SmallGraphic = newnr;
      }
      if ( self.RuleVar[947] != 1.0)
        return;
      index1 = 1000;
      do
      {
        if (self.TempString[index1].Length > 0 && Conversion.Val(self.TempString[index1]) ==  oldnr)
          self.TempString[index1] = newnr != -1 ? newnr.ToString() : "";
        index1 += 1;
      }
      while (index1 <= 1099);
    }

    pub fn ReinfChangeEventNr(oldnr: i32, newnr: i32)
    {
      let mut sfTypeCounter: i32 =  self.SFTypeCounter;
      for (let mut index: i32 =  0; index <= sfTypeCounter; index += 1)
      {
        if (self.SFTypeObj[index].ReinforcementType == oldnr)
          self.SFTypeObj[index].ReinforcementType = newnr;
        if (self.SFTypeObj[index].ReinforcementType2 == oldnr)
          self.SFTypeObj[index].ReinforcementType2 = newnr;
        if (self.SFTypeObj[index].ReinforcementType3 == oldnr)
          self.SFTypeObj[index].ReinforcementType3 = newnr;
      }
    }
  }
// }

impl DataClass {
        pub fn GetLibVarUseId(libVarId: i32, slotId: i32) -> i32
    {
      let mut index: i32 =  slotId;
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.Hex || self.LibVarObj[libVarId].type == NewEnums.LibVarType.General)
        return libVarId;
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.HistoricalUnit)
      {
        num2 = self.HistoricalUnitObj[index].LibId.libSlot;
        num1 = self.HistoricalUnitObj[index].LibId.id;
        if (num1 == -1)
          num1 = index;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.HistoricalUnitModel)
      {
        num2 = self.HistoricalUnitObj[index].LibId.libSlot;
        num1 = self.HistoricalUnitObj[index].LibId.id;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.Officer)
      {
        if (self.HistoricalUnitObj[index].OffLibId.id > -1)
        {
          num2 = self.HistoricalUnitObj[index].OffLibId.libSlot;
          num1 = self.HistoricalUnitObj[index].OffLibId.id;
        }
        else
        {
          num2 = self.HistoricalUnitObj[index].LibId.libSlot;
          num1 = self.HistoricalUnitObj[index].LibId.id;
        }
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.Landscape)
      {
        num2 = -1;
        num1 = index;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.LocationType)
      {
        num2 = -1;
        num1 = index;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.People)
      {
        num2 = self.PeopleObj[index].LibId.libSlot;
        num1 = self.PeopleObj[index].LibId.id;
        if (num1 == -1)
          num1 = index;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.Regime)
      {
        num2 = self.RegimeObj[index].libId.libSlot;
        num1 = self.RegimeObj[index].libId.id;
        if (num1 == -1)
          num1 = index;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.River)
      {
        num2 = -1;
        num1 = index;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.Road)
      {
        num2 = -1;
        num1 = index;
      }
      if (self.LibVarObj[libVarId].type == NewEnums.LibVarType.SFtype)
      {
        num2 = self.SFTypeObj[index].LibId.libSlot;
        num1 = self.SFTypeObj[index].LibId.id;
        if (num1 == -1)
          num1 = index;
      }
      if (index > -1)
      {
        let mut libVarCounter: i32 =  self.LibVarCounter;
        for (let mut libVarUseId: i32 =  0; libVarUseId <= libVarCounter; libVarUseId += 1)
        {
          if (self.LibVarObj[libVarId].libId.libSlot == self.LibVarObj[libVarUseId].libId.libSlot && Operators.CompareString(self.LibVarObj[libVarId].name, self.LibVarObj[libVarUseId].name, false) == 0 && num2 == self.LibVarObj[libVarUseId].instanceId.libSlot && num1 == self.LibVarObj[libVarUseId].instanceId.id)
            return libVarUseId;
        }
      }
      return libVarId;
    }

        pub DataClass(let mut twidth: i32 =  25, let mut theight: i32 =  25, bool DontLoadGraphics = false)
    {
      self.GameSlot = new int[500];
      self.GameSlotName = new string[500];
      self.GameSlotShow = new bool[500];
      self.GameSlotShow2 = new bool[500];
      self.RegimeSlotName = new string[500];
      self.RegimeSlotShow = new bool[500];
      self.RegimeSlotShow2 = new int[500];
      self.RegimeSlotNato = new int[500];
      self.RegimeSlotSmallGfx = new int[500];
      self.RealTempString = new string[1000];
      self.GameSlotNato = new int[500];
      self.GameSlotSmallGfx = new int[500];
      self.MapObj = new MapClass[1];
      self.LibraryObj = new LibraryClass[1];
      self.LibVarObj = new LibVarClass[1];
      self.HexObj = new HexClass[1][1];
      self.LandscapeTypeObj = new LandscapeTypeClass[1];
      self.RoadTypeObj = new RoadTypeClass[1];
      self.RegimeObj = new RegimeClass[1];
      self.UnitObj = new UnitClass[1];
      self.SFObj = new SFClass[1];
      self.SFTypeObj = new SFTypeClass[1];
      self.LocTypeObj = new LocationTypeClass[1];
      self.LocObj = new LocationClass[1];
      self.ItemTypeObj = new ItemTypeClass[1];
      self.PeopleObj = new PeopleClass[1];
      self.StringCounter = 1500;
      self.TempString = new string[1501];
      self.RuleVar = new float[1001];
      self.RuleString = new string[1001];
      self.RuleGroup = new int[1001];
      self.RuleGroup2 = new int[1001];
      self.RuleCounter = 1000;
      self.RiverTypeObj = new RiverTypeClass[1];
      self.AreaObj = new AreaClass[1];
      self.HistoricalUnitObj = new HistoricalUnitClass[1];
      self.BridgeObj = new BridgeClass[1];
      self.ActionCardObj = new ActionCardClass[1];
      self.ResearchObj = new ResearchClass[1];
      self.EventObj = new EventClass[1];
      self.EventPicName = new string[1];
      self.EventPicNr = new int[1];
      self.eventPicLibId = new LibIdClass[1];
      self.SmallPicName = new string[1];
      self.SmallPicNr = new int[1];
      self.SmallLibId = new LibIdClass[1];
      self.ReinfName = new string[1];
      self.ReinfLibId = new LibIdClass[1];
      self.ReinfId = new int[1];
      self.StringListObj = new StringListClass[1];
      self.CheckTypeNames = new string[400];
      self.ExecTypeNames = new string[400];
      self.CheckTypeVarName = new string[300][5];
      self.CheckTypeVarCount = new int[300];
      self.CheckDesc = new string[300];
      self.CheckCategory = new int[300];
      self.CheckCategory2 = new int[300];
      self.ExecCategory = new int[400];
      self.ExecCategory2 = new int[400];
      self.ExecTypeVarName = new string[400][5];
      self.ExecTypeVarCount = new int[400];
      self.ExecDesc = new string[400];
      self.ExecTypeString = new int[400];
      self.TempVar = new int[1000];
      self.ExecCategoryName = new string[100];
      self.CheckCategoryName = new string[100];
      self.Variants = new int[12];
      self.VariantEvent = new int[12];
      self.MoveTypePenalty = new int[100];
      self.UnitTypePenalty = new int[100];
      self.WheaterColor = new int[3];
      self.ReinfRatio = new int[50];
      self.transportMovementType = new int[100];
      self.se1_earlyCinematicsLogin = 0;
      self.UncertaintyOn = false;
      self.Name = "New Scenario";
      self.Description = "This is a blank scenario.";
      self.Round = 0;
      self.Turn = 0;
      self.scenarioVersion = "";
      self.scenarioVersionMaster = "";
      self.MapDesigner = "Unknown".to_owned();
      self.MapName = "Unnamed".to_owned();
      self.MapVersion = 1;
      self.se1_earlyCinematicsLogin = 0;
      self.PermanentOverlayName = "systemgraphics/trans.bmp";
      self.PermanentOverlayUse = false;
      self.Version = 424;
      self.ExtraTabName = "";
      self.ExtraTabEvent = -1;
      self.ExtraTabName2 = "";
      self.ExtraTabEvent2 = -1;
      self.ExtraTabName3 = "";
      self.ExtraTabEvent3 = -1;
      self.ExtraTabName4 = "";
      self.ExtraTabEvent4 = -1;
      self.LandscapeTypeCounter = 0;
      self.LandscapeTypeObj = new LandscapeTypeClass[self.LandscapeTypeCounter + 1];
      self.LandscapeTypeObj[0] = new LandscapeTypeClass(0);
      self.MapObj = new MapClass[1];
      self.MapCounter = 0;
      self.MapObj[0] = new MapClass(0, 0, twidth, theight);
      self.MapObj[0].MapWidth = twidth;
      self.MapObj[0].MapHeight = theight;
      self.HexObj = new HexClass[self.MapWidth + 1, self.MapHeight + 1];
      let mut mapWidth: i32 =  self.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  self.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          self.MapObj[0].HexObj[index1, index2] = new HexClass(0, 0, 0);
      }
      self.RoadTypeCounter = 0;
      self.RoadTypeObj[0] = new RoadTypeClass(0);
      self.LibraryCounter = -1;
      self.LibVarCounter = -1;
      self.RegimeCounter = -1;
      self.UnitCounter = -1;
      self.SFCounter = -1;
      self.SFTypeCounter = -1;
      self.LocTypeCounter = -1;
      self.LocCounter = -1;
      self.ItemTypeCounter = -1;
      self.PeopleCounter = -1;
      self.ActionCardCounter = -1;
      self.ActionCardObj[0] = (ActionCardClass) null;
      self.RiverTypeCounter = 0;
      self.RiverTypeObj[0] = new RiverTypeClass(0);
      self.BridgeObj[0] = new BridgeClass(0);
      self.StringListCounter = -1;
      let mut index3: i32 =  0;
      do
      {
        self.GameSlot[index3] = -1;
        self.GameSlotName[index3] = "Empty".to_owned();
        self.RegimeSlotName[index3] = "Empty".to_owned();
        self.GameSlotSmallGfx[index3] = -1;
        self.RegimeSlotSmallGfx[index3] = -1;
        index3 += 1;
      }
      while (index3 <= 499);
      self.ResearchCounter = -1;
      self.EventCounter = -1;
      self.EventPicCounter = -1;
      self.SmallPicCounter = -1;
      self.ReinfCounter = -1;
      self.HistoricalUnitCounter = -1;
      self.AreaCounter = -1;
      self.Winner = -1;
      self.LastWinner = -1;
      self.VPWin = -1;
      self.ASOn = true;
      self.ResCostMod = 1f;
      self.SupplyMultiplier = 1f;
      self.PPMultiplier = 1f;
      self.AlternateRound = -1;
      self.AlternateRound2 = -1;
      self.Turn = -1;
      let mut index4: i32 =  0;
      do
      {
        self.Variants[index4] = -1;
        self.VariantEvent[index4] = -1;
        index4 += 1;
      }
      while (index4 <= 11);
      let mut index5: i32 =  0;
      do
      {
        self.MoveTypePenalty[index5] = 100;
        self.UnitTypePenalty[index5] = 100;
        index5 += 1;
      }
      while (index5 <= 99);
      self.Designer = "";
      self.Designer2 = "";
      self.CampaignRoom = -1;
      self.MasterfileReadPeople = false;
      self.MapLoop = false;
      self.SetDefaultTempStrings();
      self.SetDefaultRules();
      self.SetEventNames();
      if (!DontLoadGraphics)
        self.LoadGraphics((Form1) null);
      let mut index6: i32 =  0;
      do
      {
        self.ReinfRatio[index6] = 1;
        index6 += 1;
      }
      while (index6 <= 49);
      let mut index7: i32 =  0;
      do
      {
        self.transportMovementType[index7] = 0;
        index7 += 1;
      }
      while (index7 <= 99);
      self.RuleVar[839] = 1f;
      self.RuleVar[501] = 1f;
      self.RuleVar[503] = 1f;
      self.RuleVar[504] = 1f;
      self.RuleVar[506] = 1f;
      self.RuleVar[509] = 0.0f;
      self.RuleVar[510] = 1f;
      self.RuleVar[517] = 1f;
      self.RuleVar[513] = 1f;
      self.RuleVar[847] = 1f;
      self.RuleVar[518] = 1f;
      self.RuleVar[519] = 1f;
      self.RuleVar[524] = 0.0f;
      self.RuleVar[307] = 1f;
      self.RuleVar[530] = 0.0f;
      self.RuleVar[861] = 0.0f;
      self.RuleVar[862] = 0.0f;
      self.RuleVar[316] = 1f;
      self.RuleVar[317] = 1f;
      self.RuleVar[344] = 1f;
      self.RuleVar[843] = -1f;
      self.RuleVar[340] = -1f;
      self.RuleVar[337] = 1f;
      self.RuleVar[884] = 1f;
      self.RuleVar[143] = 10f;
      self.RuleVar[144] = 10f;
      self.RuleVar[145] = 10f;
      self.RuleVar[146] = 10f;
      self.RuleVar[339] = 250f;
      self.RuleVar[887] = 1f;
      self.UseAI = 1;
      self.MasterFile = "";
      self.EditPass = "";
      self.LoadPass = "";
      self.PbemGameID = 0;
      self.PbemPlayer1 = "";
      self.PbemPlayer2 = "";
      self.PbemGameOver = 0;
      self.Product = 7;
      self.HistoricalIDCounter = 0;
    }

}
