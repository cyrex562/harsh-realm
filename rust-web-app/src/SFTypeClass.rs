// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SFTypeClass
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

namespace WindowsApplication1
{
  [Serializable]
  pub class SFTypeClass : ISerializable
  {
    pub Name: String;
    pub Description: String;
    pub Id: i32;
    pub PicFileName: String;
    pub PicSpriteID: i32;
    pub SidewaysFileName: String;
    pub SidewaysSpriteID: i32;
    pub SymbolFileName: String;
    pub SymbolSpriteID: i32;
    pub SymbolColBigFileName: String;
    pub SymbolColBigSpriteID: i32;
    pub SymbolColBigFileName2: String;
    pub SymbolColBigSprite2ID: i32;
    pub SymbolFileName2: String;
    pub SymbolSprite2ID: i32;
    pub SymbolGroup: i32;
    pub SymbolWeight: i32;
    pub SymbolOverrule: bool;
    pub ExtraCounter: i32;
    pub ExtraPicFileName: Vec<String>;
    pub ExtraPicSpriteID: Vec<i32>;
    pub ExtraSidewaysFileName: Vec<String>;
    pub ExtraSidewaysSpriteID: Vec<i32>;
    pub ExtraSymbolFileName: Vec<String>;
    pub ExtraSymbolSpriteID: Vec<i32>;
    pub ExtraSymbolFileName2: Vec<String>;
    pub ExtraSymbolSprite2ID: Vec<i32>;
    pub ExtraSymbolColBigFileName: Vec<String>;
    pub ExtraSymbolColBigSpriteID: Vec<i32>;
    pub ExtraSymbolColBigFileName2: Vec<String>;
    pub ExtraSymbolColBigSprite2ID: Vec<i32>;
    pub ExtraCode: Vec<i32>;
    pub ExtraName: Vec<String>;
    pub BaseColor: i32;
    pub MoveType: i32;
    pub SupplyCarry: i32;
    pub PeopleGroup: Vec<bool>;
    pub Cap: i32;
    pub RailCap: i32;
    pub BasicSupplyNeed: i32;
    pub UnitGroup: i32;
    pub Theater: i32;
    pub Weight: i32;
    pub Frontage: i32;
    pub EntrenchPower: i32;
    pub DefPower: i32;
    pub Initiative: i32;
    pub InitiativeDef: i32;
    pub MaxAttacked: i32;
    pub Attacks: i32;
    pub BackBench: bool;
    pub ArtRange: i32;
    pub FavTarget: Vec<i32>;
    pub FavArtTarget: Vec<i32>;
    pub FavTargetTries: i32;
    pub AttackPower: Vec<i32>;
    pub AttackPowerDef: Vec<i32>;
    pub object[] AttackArt;
    pub CanDoParadrop: bool;
    pub KillPercent: i32;
    pub EquipPercent: i32;
    pub RetreatPercent: i32;
    pub RdnLossPerAttack: i32;
    pub AutoDestroy: bool;
    pub AutoDestroy2: bool;
    pub CarryCap: i32;
    pub AirCarrierCap: i32;
    pub PowerPts: i32;
    pub AntiStrucPts: i32;
    pub ReconPts: i32;
    pub HidePts: i32;
    pub ZOCPts: i32;
    pub float ApMod;
    pub EP: i32;
    pub MoveWAV: String;
    pub BattleWAV: String;
    pub MoveRedux: i32;
    pub targettedByRangedChance: i32;
    pub CopyDataFrom: i32;
    pub CopyDataFromBackup: i32;
    pub float[] CombatModAtt;
    pub float[] CombatModDef;
    pub ExtraRecon: Vec<i32>;
    pub StaffPts: i32;
    pub float StaffCombatMod;
    pub float StaffMoraleMod;
    pub UpgradeToo: i32;
    pub UpgradeCost: i32;
    pub UpgradeXP: i32;
    pub AARange: i32;
    pub BlowBridgePts: i32;
    pub KilltoRetreatChance: i32;
    pub AIRoleScore: Vec<i32>;
    pub AIMobNeedScore: i32;
    pub AntiSupply: i32;
    pub AntiSupplySea: i32;
    pub AntiSupplyRange: i32;
    pub ReadinessLoss: i32;
    pub KillIsRegVar: i32;
    pub FreeAir: bool;
    pub SlotNumber: i32;
    pub Ratio: i32;
    pub Unique: bool;
    pub DepletingHitpointRule: i32;
    pub StartCombatRound: i32;
    pub EndCombatRound: i32;
    pub AirAPRule: i32;
    pub ReinforcementType: i32;
    pub ReinforcementType2: i32;
    pub ReinforcementType3: i32;
    pub DontReturnFromHQ: bool;
    pub ConsiderCarry: bool;
    pub float FirstRoundPenaltyMod;
    pub ModelIsBase: bool;
    pub ModelLevel: i32;
    pub ModelName: String;
    pub ModelMark: i32;
    pub ModelLastState: Vec<i32>;
    pub ModelVersion: i32;
    pub ModelID: i32;
    pub ModelPossibleImp: Vec<i32>;
    pub ModelCostType: i32;
    pub ModelCost: i32;
    pub float ModelCostPerLevel;
    pub float ModelCostPerSameModel;
    pub ModelBaseModel: i32;
    pub ModelResearch: Vec<i32>;
    pub ModelNewEvent: i32;
    pub ModelNameList: i32;
    pub ModelAllowUpgrade: bool;
    pub ModelImproveEvent: Vec<i32>;
    pub ModelAllowImprovements: bool;
    pub float ModelImproveCostMod;
    pub float ModelCostPerSameImp;
    pub ModelAutoImprovement: Vec<bool>;
    pub ModelExtraResearch: i32;
    pub ModelRegime: i32;
    pub ModelInitialForAll: bool;
    pub ModelInitialEvent: i32;
    pub ModelItemType: i32;
    pub float ModelSFTypeUpgrade;
    pub TempUpgradeCost: i32;
    pub TempImproveCost: i32;
    pub TempNewCost: i32;
    pub TempFieldCount: i32;
    pub TempNewLevels: i32;
    pub TempUpgradeLevels: i32;
    pub TempImprovementFields: i32;
    pub TempAlterationCost: i32;
    pub TempAlterationCount: i32;
    pub TempAlterationPossible: Vec<bool>;
    pub float TempAvgCombatMatrixAtt;
    pub float TempAvgCombatMatrixDef;
    pub LogoString: Vec<String>;
    pub PreventCounter: i32;
    pub PreventHitOn: Vec<i32>;
    pub PreventHitFrom: Vec<i32>;
    pub PreventChance: Vec<i32>;
    pub PreventPoints: Vec<i32>;
    pub PreventPriority: Vec<i32>;
    pub MaxPreventPointsUsed: i32;
    pub MaxPreventPointsGiven: i32;
    pub HitPoints: Vec<i32>;
    pub HitPointsDef: Vec<i32>;
    pub FuelRegimeVar: i32;
    pub FuelForMove: i32;
    pub float OutOfFuelMove;
    pub FuelForAttack: i32;
    pub float OutOfFuelAttack;
    pub float OutOfFuelDefense;
    pub FuelForAttackDef: i32;
    pub SupplyForAttack: i32;
    pub float OutOfSupplyAttack;
    pub float OutOfSupplyDefense;
    pub SupplyForAttackDef: i32;
    pub FuelCarry: i32;
    pub ModelVariantCounter: i32;
    pub ModelVariantName: Vec<String>;
    pub ModelVariantCheck: Vec<i32>;
    pub ModelVariantExec: Vec<i32>;
    pub artCode: Vec<i32>;
    pub SFTypeVar: Vec<i32>;
    pub ArtSFType: i32;
    pub UsePeopleGraphics: i32;
    pub DontShowInList: bool;
    pub StockpileUsedPerRound: i32;
    pub StockpileMax: i32;
    pub StockpileMaxIn: i32;
    pub float StockpileDepletedMod;
    pub SupplyMaxIn: i32;
    pub float ChanceOnDeathIfMakeHit;
    pub directRange: i32;
    pub directModFirstHex: i32;
    pub directModPerHex: i32;
    pub heightLevelDiff: i32;
    pub scrapable: i32;
    pub manpower: i32;
    pub manpowerCarry: i32;
    pub LibIdClass LibId;

    pub fn GetObjectData(SerializationInfo info, StreamingContext context)
    {
      info.AddValue("Name",  self.Name);
      info.AddValue("Description",  self.Description);
      info.AddValue("PicFileName",  self.PicFileName);
      info.AddValue("SymbolFileName",  self.SymbolFileName);
      info.AddValue("SymbolFileName2",  self.SymbolFileName2);
      info.AddValue("SymbolGroup", self.SymbolGroup);
      info.AddValue("SymbolWeight", self.SymbolWeight);
      info.AddValue("MoveType", self.MoveType);
      info.AddValue("SupplyCarry", self.SupplyCarry);
      info.AddValue("PeopleGroup",  self.PeopleGroup);
      info.AddValue("LandCap", self.Cap);
      info.AddValue("BasicSupplyNeed", self.BasicSupplyNeed);
      info.AddValue("UnitGroup", self.UnitGroup);
      info.AddValue("Theater", self.Theater);
      info.AddValue("Weight", self.Weight);
      info.AddValue("DefPower", self.DefPower);
      info.AddValue("Initiative", self.Initiative);
      info.AddValue("Attacks", self.Attacks);
      info.AddValue("MaxAttacked", self.MaxAttacked);
      info.AddValue("Frontage", self.Frontage);
      info.AddValue("Backbench", self.BackBench);
      info.AddValue("ArtRange", self.ArtRange);
      info.AddValue("FavTarget",  self.FavTarget);
      info.AddValue("FavTargetTries", self.FavTargetTries);
      info.AddValue("AttackPower",  self.AttackPower);
      info.AddValue("AttackPowerDef",  self.AttackPowerDef);
      info.AddValue("SymbolOverrule", self.SymbolOverrule);
      info.AddValue("DepletingHitpointRule", self.DepletingHitpointRule);
      info.AddValue("StartCombatRound", self.StartCombatRound);
      info.AddValue("EndCombatRound", self.EndCombatRound);
      info.AddValue("CarryCap", self.CarryCap);
      info.AddValue("EntrenchPower", self.EntrenchPower);
      info.AddValue("PowerPts", self.PowerPts);
      info.AddValue("ReconPts", self.ReconPts);
      info.AddValue("HidePts", self.HidePts);
      info.AddValue("ZOCPts", self.ZOCPts);
      info.AddValue("CanDoParadrop", self.CanDoParadrop);
      info.AddValue("AntiStrucPts", self.AntiStrucPts);
      info.AddValue("AirCarrierCap", self.AirCarrierCap);
      info.AddValue("ApMod", self.ApMod);
      info.AddValue("KillPercent", self.KillPercent);
      info.AddValue("EquipPercent", self.EquipPercent);
      info.AddValue("RetreatPercent", self.RetreatPercent);
      info.AddValue("InitiativeDef", self.InitiativeDef);
      info.AddValue("RdnLossPerAttack", self.RdnLossPerAttack);
      info.AddValue("AutoDestroy", self.AutoDestroy);
      info.AddValue("EP", self.EP);
      info.AddValue("MoveWAV",  self.MoveWAV);
      info.AddValue("BattleWAV",  self.BattleWAV);
      info.AddValue("CopyDataFrom", self.CopyDataFrom);
      info.AddValue("CopyDataFromBackup", self.CopyDataFromBackup);
      info.AddValue("CombatModAtt",  self.CombatModAtt);
      info.AddValue("CombatModDef",  self.CombatModDef);
      info.AddValue("StaffPts", self.StaffPts);
      info.AddValue("StaffMoraleMod", self.StaffMoraleMod);
      info.AddValue("StaffCombatMod", self.StaffCombatMod);
      info.AddValue("ÜpgradeToo", self.UpgradeToo);
      info.AddValue("UpgradeCost", self.UpgradeCost);
      info.AddValue("UpgradeXP", self.UpgradeXP);
      info.AddValue("MoveRedux", self.MoveRedux);
      info.AddValue("AARange", self.AARange);
      info.AddValue("FavArtTarget",  self.FavArtTarget);
      info.AddValue("AttackArt",  self.AttackArt);
      info.AddValue("BlowBridgePts", self.BlowBridgePts);
      info.AddValue("AIRoleScore",  self.AIRoleScore);
      info.AddValue("AIMobNeedScore", self.AIMobNeedScore);
      info.AddValue("KilltoRetreatChance", self.KilltoRetreatChance);
      info.AddValue("AntiSupply", self.AntiSupply);
      info.AddValue("AntiSupplySea", self.AntiSupplySea);
      info.AddValue("AntiSupplyRange", self.AntiSupplyRange);
      info.AddValue("ReadinessLoss", self.ReadinessLoss);
      info.AddValue("RailCap", self.RailCap);
      info.AddValue("KillIsRegVar", self.KillIsRegVar);
      info.AddValue("ExtraCounter", self.ExtraCounter);
      info.AddValue("ExtraPicFileName",  self.ExtraPicFileName);
      info.AddValue("ExtraSymbolFileName",  self.ExtraSymbolFileName);
      info.AddValue("ExtraSymbolFileName2",  self.ExtraSymbolFileName2);
      info.AddValue("ExtraCode",  self.ExtraCode);
      info.AddValue("AutoDestroy2", self.AutoDestroy2);
      info.AddValue("ExtraName",  self.ExtraName);
      info.AddValue("SlotNumber", self.SlotNumber);
      info.AddValue("FreeAir", self.FreeAir);
      info.AddValue("Ratio", self.Ratio);
      info.AddValue("ExtraRecon",  self.ExtraRecon);
      info.AddValue("Unique", self.Unique);
      info.AddValue("AirAPRule", self.AirAPRule);
      info.AddValue("ReinforcementType", self.ReinforcementType);
      info.AddValue("ReinforcementType2", self.ReinforcementType2);
      info.AddValue("ReinforcementType3", self.ReinforcementType3);
      info.AddValue("DontReturnFromHQ", self.DontReturnFromHQ);
      info.AddValue("ConsiderCarry", self.ConsiderCarry);
      info.AddValue("FirstRoundPenaltyMod", self.FirstRoundPenaltyMod);
      info.AddValue("rcount", self.ModelLastState.GetUpperBound(0));
      info.AddValue("ModelIsBase", self.ModelIsBase);
      info.AddValue("ModelLevel", self.ModelLevel);
      info.AddValue("ModelName",  self.ModelName);
      info.AddValue("ModelMark", self.ModelMark);
      info.AddValue("ModelLastState",  self.ModelLastState);
      info.AddValue("ModelVersion", self.ModelVersion);
      info.AddValue("ModelID", self.ModelID);
      info.AddValue("ModelPossibleImp",  self.ModelPossibleImp);
      info.AddValue("ModelCostType", self.ModelCostType);
      info.AddValue("ModelCost", self.ModelCost);
      info.AddValue("ModelCostPerLevel", self.ModelCostPerLevel);
      info.AddValue("ModelCostPerSameModel", self.ModelCostPerSameModel);
      info.AddValue("ModelBaseModel", self.ModelBaseModel);
      info.AddValue("ModelResearch",  self.ModelResearch);
      info.AddValue("ModelNewEvent", self.ModelNewEvent);
      info.AddValue("ModelNameList", self.ModelNameList);
      info.AddValue("ModelAllowUpgrade", self.ModelAllowUpgrade);
      info.AddValue("ModelImproveEvent",  self.ModelImproveEvent);
      info.AddValue("ModelAllowImprovements", self.ModelAllowImprovements);
      info.AddValue("ModelImproveCostMod", self.ModelImproveCostMod);
      info.AddValue("ModelCostPerSameImp", self.ModelCostPerSameImp);
      info.AddValue("ModelAutoImprovement",  self.ModelAutoImprovement);
      info.AddValue("ModelRegime", self.ModelRegime);
      info.AddValue("ModelInitialForAll", self.ModelInitialForAll);
      info.AddValue("ModelInitialEvent", self.ModelInitialEvent);
      info.AddValue("ModelItemType", self.ModelItemType);
      info.AddValue("LogoString",  self.LogoString);
      info.AddValue("PreventCounter", self.PreventCounter);
      info.AddValue("MaxPreventPointsUsed", self.MaxPreventPointsUsed);
      info.AddValue("MaxPreventPointsGiven", self.MaxPreventPointsGiven);
      info.AddValue("PreventHitOn",  self.PreventHitOn);
      info.AddValue("PreventHitFrom",  self.PreventHitFrom);
      info.AddValue("PreventPriority",  self.PreventPriority);
      info.AddValue("PreventChance",  self.PreventChance);
      info.AddValue("PreventPoints",  self.PreventPoints);
      info.AddValue("HitPoints",  self.HitPoints);
      info.AddValue("HitPointsDef",  self.HitPointsDef);
      info.AddValue("ModelExtraResearch", self.ModelExtraResearch);
      info.AddValue("FuelRegimeVar", self.FuelRegimeVar);
      info.AddValue("FuelForMove", self.FuelForMove);
      info.AddValue("OutOfFuelMove", self.OutOfFuelMove);
      info.AddValue("FuelForAttack", self.FuelForAttack);
      info.AddValue("OutOfFuelAttack", self.OutOfFuelAttack);
      info.AddValue("OutOfFuelDefense", self.OutOfFuelDefense);
      info.AddValue("FuelForAttackDef", self.FuelForAttackDef);
      info.AddValue("SupplyForAttack", self.SupplyForAttack);
      info.AddValue("OutOfSupplyAttack", self.OutOfSupplyAttack);
      info.AddValue("OutOfSupplyDefense", self.OutOfSupplyDefense);
      info.AddValue("SupplyForAttackDef", self.SupplyForAttackDef);
      info.AddValue("FuelCarry", self.FuelCarry);
      info.AddValue("ModelSFTypeUpgrade", self.ModelSFTypeUpgrade);
      info.AddValue("ModelVariantCounter", self.ModelVariantCounter);
      info.AddValue("ModelVariantCheck",  self.ModelVariantCheck);
      info.AddValue("ModelVariantExec",  self.ModelVariantExec);
      info.AddValue("ModelVariantName",  self.ModelVariantName);
      info.AddValue("SFTypeVar",  self.SFTypeVar);
      info.AddValue("artCode",  self.artCode);
      info.AddValue("ArtSFType", self.ArtSFType);
      info.AddValue("UsePeopleGraphics", self.UsePeopleGraphics);
      info.AddValue("SidewaysFileName",  self.SidewaysFileName);
      info.AddValue("DontShowInList", self.DontShowInList);
      info.AddValue("BaseColor", self.BaseColor);
      info.AddValue("ExtraSidewaysFileName",  self.ExtraSidewaysFileName);
      info.AddValue("ExtraSymbolColBigFileName",  self.ExtraSymbolColBigFileName);
      info.AddValue("SymbolColBigFileName",  self.SymbolColBigFileName);
      info.AddValue("ExtraSymbolColBigFileName2",  self.ExtraSymbolColBigFileName2);
      info.AddValue("SymbolColBigFileName2",  self.SymbolColBigFileName2);
      info.AddValue("StockpileUsedPerRound", self.StockpileUsedPerRound);
      info.AddValue("StockpileMax", self.StockpileMax);
      info.AddValue("StockpileMaxIn", self.StockpileMaxIn);
      info.AddValue("SupplyMaxIn", self.SupplyMaxIn);
      info.AddValue("StockpileDepletedMod", self.StockpileDepletedMod);
      info.AddValue("ChanceOnDeathIfMakeHit", self.ChanceOnDeathIfMakeHit);
      info.AddValue("LibId",  self.LibId);
      info.AddValue("Id", self.Id);
      info.AddValue("heightLevelDiff", self.heightLevelDiff);
      info.AddValue("directRange", self.directRange);
      info.AddValue("directModFirstHex", self.directModFirstHex);
      info.AddValue("directModPerHex", self.directModPerHex);
      info.AddValue("targettedByRangedChance", self.targettedByRangedChance);
      info.AddValue("scrapable", self.scrapable);
    }

    protected SFTypeClass(SerializationInfo info, StreamingContext context)
    {
      self.ExtraPicFileName = new string[1];
      self.ExtraPicSpriteID = new int[1];
      self.ExtraSidewaysFileName = new string[1];
      self.ExtraSidewaysSpriteID = new int[1];
      self.ExtraSymbolFileName = new string[1];
      self.ExtraSymbolSpriteID = new int[1];
      self.ExtraSymbolFileName2 = new string[1];
      self.ExtraSymbolSprite2ID = new int[1];
      self.ExtraSymbolColBigFileName = new string[1];
      self.ExtraSymbolColBigSpriteID = new int[1];
      self.ExtraSymbolColBigFileName2 = new string[1];
      self.ExtraSymbolColBigSprite2ID = new int[1];
      self.ExtraCode = new int[1];
      self.ExtraName = new string[1];
      self.PeopleGroup = new bool[100];
      self.FavTarget = new int[100];
      self.FavArtTarget = new int[100];
      self.AttackPower = new int[100];
      self.AttackPowerDef = new int[100];
      self.AttackArt = new object[100];
      self.CombatModAtt = new float[1];
      self.CombatModDef = new float[1];
      self.ExtraRecon = new int[1];
      self.AIRoleScore = new int[50];
      self.ModelLastState = new int[2];
      self.ModelPossibleImp = new int[2];
      self.ModelResearch = new int[10];
      self.ModelImproveEvent = new int[2];
      self.ModelAutoImprovement = new bool[2];
      self.TempAlterationPossible = new bool[2];
      self.LogoString = new string[100];
      self.PreventHitOn = new int[2];
      self.PreventHitFrom = new int[2];
      self.PreventChance = new int[2];
      self.PreventPoints = new int[2];
      self.PreventPriority = new int[2];
      self.HitPoints = new int[100];
      self.HitPointsDef = new int[100];
      self.ModelVariantName = new string[1];
      self.ModelVariantCheck = new int[1];
      self.ModelVariantExec = new int[1];
      self.artCode = new int[10];
      self.SFTypeVar = new int[100];
      self.Name = info.GetString(nameof (Name));
      self.Description = info.GetString(nameof (Description));
      self.PicFileName = info.GetString(nameof (PicFileName));
      self.SymbolFileName = info.GetString(nameof (SymbolFileName));
      try
      {
        self.SymbolFileName2 = info.GetString(nameof (SymbolFileName2));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.SymbolFileName2 = self.SymbolFileName;
        ProjectData.ClearProjectError();
      }
      self.SymbolGroup = info.GetInt32(nameof (SymbolGroup));
      self.SymbolWeight = info.GetInt32(nameof (SymbolWeight));
      self.MoveType = info.GetInt32(nameof (MoveType));
      self.SupplyCarry = info.GetInt32(nameof (SupplyCarry));
      self.PeopleGroup = (bool[]) info.GetValue(nameof (PeopleGroup), self.PeopleGroup.GetType());
      self.PeopleGroup = (bool[]) Utils.CopyArray((Array) self.PeopleGroup, (Array) new bool[100]);
      self.Cap = info.GetInt32("LandCap");
      self.BasicSupplyNeed = info.GetInt32(nameof (BasicSupplyNeed));
      self.UnitGroup = info.GetInt32(nameof (UnitGroup));
      self.Theater = info.GetInt32(nameof (Theater));
      self.Weight = info.GetInt32(nameof (Weight));
      self.DefPower = info.GetInt32(nameof (DefPower));
      self.Initiative = info.GetInt32(nameof (Initiative));
      self.Attacks = info.GetInt32(nameof (Attacks));
      self.MaxAttacked = info.GetInt32(nameof (MaxAttacked));
      self.Frontage = info.GetInt32(nameof (Frontage));
      self.BackBench = (uint) info.GetInt32("Backbench") > 0U;
      self.ArtRange = info.GetInt32(nameof (ArtRange));
      self.FavTarget = (int[]) info.GetValue(nameof (FavTarget), self.FavTarget.GetType());
      self.FavTarget = (int[]) Utils.CopyArray((Array) self.FavTarget, (Array) new int[100]);
      self.FavTargetTries = info.GetInt32(nameof (FavTargetTries));
      self.AttackPower = (int[]) info.GetValue(nameof (AttackPower), self.AttackPower.GetType());
      self.AttackPower = (int[]) Utils.CopyArray((Array) self.AttackPower, (Array) new int[100]);
      self.AttackPowerDef = (int[]) info.GetValue(nameof (AttackPowerDef), self.AttackPowerDef.GetType());
      self.AttackPowerDef = (int[]) Utils.CopyArray((Array) self.AttackPowerDef, (Array) new int[100]);
      self.SymbolOverrule = info.GetBoolean(nameof (SymbolOverrule));
      self.CarryCap = info.GetInt32(nameof (CarryCap));
      self.EntrenchPower = info.GetInt32(nameof (EntrenchPower));
      self.PowerPts = info.GetInt32(nameof (PowerPts));
      self.ReconPts = info.GetInt32(nameof (ReconPts));
      self.HidePts = info.GetInt32(nameof (HidePts));
      self.ZOCPts = info.GetInt32(nameof (ZOCPts));
      self.CanDoParadrop = info.GetBoolean(nameof (CanDoParadrop));
      self.AntiStrucPts = info.GetInt32(nameof (AntiStrucPts));
      self.AirCarrierCap = info.GetInt32(nameof (AirCarrierCap));
      self.ApMod = info.GetSingle(nameof (ApMod));
      self.KillPercent = info.GetInt32(nameof (KillPercent));
      self.RetreatPercent = info.GetInt32(nameof (RetreatPercent));
      self.EquipPercent = info.GetInt32(nameof (EquipPercent));
      self.InitiativeDef = info.GetInt32(nameof (InitiativeDef));
      self.RdnLossPerAttack = info.GetInt32(nameof (RdnLossPerAttack));
      self.AutoDestroy = info.GetBoolean(nameof (AutoDestroy));
      self.CombatModAtt = (float[]) info.GetValue(nameof (CombatModAtt), self.CombatModAtt.GetType());
      self.CombatModDef = (float[]) info.GetValue(nameof (CombatModDef), self.CombatModDef.GetType());
      if (DrawMod.TGame.Data.Version < 158)
      {
        try
        {
          self.AutoDestroy2 = info.GetBoolean(nameof (AutoDestroy2));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.AutoDestroy2 = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ExtraRecon = (int[]) info.GetValue(nameof (ExtraRecon), self.ExtraRecon.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ExtraRecon = new int[self.CombatModDef.GetUpperBound(0) + 1];
          ProjectData.ClearProjectError();
        }
      }
      else
      {
        self.AutoDestroy2 = info.GetBoolean(nameof (AutoDestroy2));
        self.ExtraRecon = (int[]) info.GetValue(nameof (ExtraRecon), self.ExtraRecon.GetType());
      }
      if (self.ExtraRecon.GetUpperBound(0) == 0)
        self.ExtraRecon = new int[self.CombatModDef.GetUpperBound(0) + 1];
      self.EP = info.GetInt32(nameof (EP));
      self.MoveWAV = info.GetString(nameof (MoveWAV));
      self.BattleWAV = info.GetString(nameof (BattleWAV));
      self.CopyDataFrom = info.GetInt32(nameof (CopyDataFrom));
      try
      {
        self.CopyDataFromBackup = info.GetInt32(nameof (CopyDataFromBackup));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.CopyDataFromBackup = -1;
        ProjectData.ClearProjectError();
      }
      self.StaffPts = info.GetInt32(nameof (StaffPts));
      self.StaffCombatMod = info.GetSingle(nameof (StaffCombatMod));
      self.StaffMoraleMod = info.GetSingle(nameof (StaffMoraleMod));
      self.UpgradeToo = info.GetInt32("ÜpgradeToo");
      self.UpgradeCost = info.GetInt32(nameof (UpgradeCost));
      self.UpgradeXP = info.GetInt32(nameof (UpgradeXP));
      self.MoveRedux = info.GetInt32(nameof (MoveRedux));
      self.AARange = info.GetInt32(nameof (AARange));
      self.FavArtTarget = (int[]) info.GetValue(nameof (FavArtTarget), self.FavArtTarget.GetType());
      self.AttackArt = (object[]) info.GetValue(nameof (AttackArt), self.AttackArt.GetType());
      self.AIRoleScore = (int[]) info.GetValue(nameof (AIRoleScore), self.AIRoleScore.GetType());
      self.AIMobNeedScore = info.GetInt32(nameof (AIMobNeedScore));
      self.BlowBridgePts = info.GetInt32(nameof (BlowBridgePts));
      self.KilltoRetreatChance = info.GetInt32(nameof (KilltoRetreatChance));
      self.AntiSupply = info.GetInt32(nameof (AntiSupply));
      self.AntiSupplyRange = info.GetInt32(nameof (AntiSupplyRange));
      self.AntiSupplySea = info.GetInt32(nameof (AntiSupplySea));
      self.ReadinessLoss = info.GetInt32(nameof (ReadinessLoss));
      self.RailCap = info.GetInt32(nameof (RailCap));
      self.KillIsRegVar = info.GetInt32(nameof (KillIsRegVar));
      let mut index1: i32 = 0;
      do
      {
        if (Information.IsNothing(RuntimeHelpers.GetObjectValue(self.AttackArt[index1])))
          self.AttackArt[index1] =  0;
        index1 += 1;
      }
      while (index1 <= 99);
      self.ExtraCounter = info.GetInt32(nameof (ExtraCounter));
      if (self.ExtraCounter > -1)
      {
        self.ExtraPicFileName = new string[self.ExtraCounter + 1];
        self.ExtraPicSpriteID = new int[self.ExtraCounter + 1];
        self.ExtraSymbolFileName = new string[self.ExtraCounter + 1];
        self.ExtraSymbolSpriteID = new int[self.ExtraCounter + 1];
        self.ExtraSymbolFileName2 = new string[self.ExtraCounter + 1];
        self.ExtraSymbolSprite2ID = new int[self.ExtraCounter + 1];
        self.ExtraCode = new int[self.ExtraCounter + 1];
        self.ExtraName = new string[self.ExtraCounter + 1];
        self.ExtraSidewaysFileName = new string[self.ExtraCounter + 1];
        self.ExtraSidewaysSpriteID = new int[self.ExtraCounter + 1];
        self.ExtraSymbolColBigFileName = new string[self.ExtraCounter + 1];
        self.ExtraSymbolColBigSpriteID = new int[self.ExtraCounter + 1];
        self.ExtraSymbolColBigFileName2 = new string[self.ExtraCounter + 1];
        self.ExtraSymbolColBigSprite2ID = new int[self.ExtraCounter + 1];
        self.ExtraPicFileName = (string[]) info.GetValue(nameof (ExtraPicFileName), self.ExtraPicFileName.GetType());
        self.ExtraSymbolFileName = (string[]) info.GetValue(nameof (ExtraSymbolFileName), self.ExtraSymbolFileName.GetType());
        self.ExtraCode = (int[]) info.GetValue(nameof (ExtraCode), self.ExtraCode.GetType());
        if (DrawMod.TGame.Data.Version < 130)
        {
          let mut extraCounter1: i32 = self.ExtraCounter;
          for (let mut index2: i32 = 0; index2 <= extraCounter1; index2 += 1)
            self.ExtraName[index2] = self.Name;
          let mut extraCounter2: i32 = self.ExtraCounter;
          for (let mut index3: i32 = 0; index3 <= extraCounter2; index3 += 1)
          {
            self.ExtraSymbolFileName2 = (string[]) self.ExtraSymbolFileName.Clone();
            self.ExtraSidewaysFileName = (string[]) self.ExtraPicFileName.Clone();
            self.ExtraSymbolColBigFileName[index3] = "systemgraphics/trans.bmp";
            self.ExtraSymbolColBigFileName2[index3] = "systemgraphics/trans.bmp";
          }
        }
        else
        {
          try
          {
            self.ExtraSymbolFileName2 = (string[]) info.GetValue(nameof (ExtraSymbolFileName2), self.ExtraSymbolFileName2.GetType());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            self.ExtraSymbolFileName2 = (string[]) self.ExtraSymbolFileName.Clone();
            ProjectData.ClearProjectError();
          }
          try
          {
            self.ExtraSidewaysFileName = (string[]) info.GetValue(nameof (ExtraSidewaysFileName), self.ExtraSidewaysFileName.GetType());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            self.ExtraSidewaysFileName = (string[]) self.ExtraPicFileName.Clone();
            ProjectData.ClearProjectError();
          }
          try
          {
            self.ExtraSymbolColBigFileName = (string[]) info.GetValue(nameof (ExtraSymbolColBigFileName), self.ExtraSymbolColBigFileName.GetType());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            let mut extraCounter: i32 = self.ExtraCounter;
            for (let mut index4: i32 = 0; index4 <= extraCounter; index4 += 1)
              self.ExtraSymbolColBigFileName[index4] = "systemgraphics/trans.bmp";
            ProjectData.ClearProjectError();
          }
          try
          {
            self.ExtraSymbolColBigFileName2 = (string[]) info.GetValue(nameof (ExtraSymbolColBigFileName2), self.ExtraSymbolColBigFileName2.GetType());
          }
          catch (Exception ex)
          {
            ProjectData.SetProjectError(ex);
            let mut extraCounter: i32 = self.ExtraCounter;
            for (let mut index5: i32 = 0; index5 <= extraCounter; index5 += 1)
              self.ExtraSymbolColBigFileName2[index5] = "systemgraphics/trans.bmp";
            ProjectData.ClearProjectError();
          }
          if (DrawMod.TGame.Data.Version < 158)
          {
            try
            {
              self.ExtraName = (string[]) info.GetValue(nameof (ExtraName), self.ExtraName.GetType());
            }
            catch (Exception ex)
            {
              ProjectData.SetProjectError(ex);
              let mut extraCounter: i32 = self.ExtraCounter;
              for (let mut index6: i32 = 0; index6 <= extraCounter; index6 += 1)
                self.ExtraName[index6] = self.Name;
              ProjectData.ClearProjectError();
            }
          }
          else
            self.ExtraName = (string[]) info.GetValue(nameof (ExtraName), self.ExtraName.GetType());
        }
      }
      else
      {
        self.ExtraPicFileName = new string[1];
        self.ExtraPicSpriteID = new int[1];
        self.ExtraSymbolFileName = new string[1];
        self.ExtraSymbolSpriteID = new int[1];
        self.ExtraSymbolColBigFileName = new string[1];
        self.ExtraSymbolColBigSpriteID = new int[1];
        self.ExtraSymbolColBigFileName2 = new string[1];
        self.ExtraSymbolColBigSprite2ID = new int[1];
        self.ExtraCode = new int[1];
        self.ExtraName = new string[1];
        self.ExtraSidewaysFileName = new string[1];
        self.ExtraSidewaysSpriteID = new int[1];
      }
      let mut index7: i32 = 0;
      do
      {
        self.HitPoints[index7] = self.DefPower;
        index7 += 1;
      }
      while (index7 <= 99);
      let mut index8: i32 = 0;
      do
      {
        self.HitPointsDef[index8] = self.HitPoints[index8];
        index8 += 1;
      }
      while (index8 <= 99);
      if (DrawMod.TGame.Data.Version < 130)
      {
        self.ConsiderCarry = false;
        self.FreeAir = false;
        self.SlotNumber = -1;
        self.Ratio = 1;
        self.Unique = false;
        self.AirAPRule = -1;
        self.ReinforcementType = -1;
        self.ReinforcementType2 = -1;
        self.ReinforcementType3 = -1;
        self.DontReturnFromHQ = false;
        let mut index9: i32 = 0;
        do
        {
          self.ModelResearch[index9] = -1;
          index9 += 1;
        }
        while (index9 <= 9);
        self.ArtSFType = -1;
        let mut index10: i32 = 0;
        do
        {
          self.HitPointsDef[index10] = self.HitPoints[index10];
          index10 += 1;
        }
        while (index10 <= 99);
        self.FuelRegimeVar = -1;
        self.OutOfSupplyAttack = 1f;
        self.OutOfFuelDefense = 1f;
        self.OutOfFuelMove = 1f;
        self.OutOfFuelAttack = 1f;
        self.OutOfFuelDefense = 1f;
        self.UsePeopleGraphics = 0;
        self.ArtSFType = -1;
        self.SidewaysFileName = "systemgraphics/trans.bmp";
        self.FirstRoundPenaltyMod = 1f;
        self.ModelRegime = -1;
        self.ModelInitialEvent = -1;
        self.ModelItemType = -1;
        self.ModelInitialForAll = false;
        self.ModelExtraResearch = -1;
        self.LogoString = new string[100];
        self.PreventCounter = -1;
        self.ModelSFTypeUpgrade = 1f;
        self.ModelVariantCounter = -1;
        self.ModelVariantName = new string[1];
        self.ModelVariantExec = new int[1];
        self.ModelVariantCheck = new int[1];
        self.SFTypeVar = new int[100];
        self.ArtSFType = -1;
        self.UsePeopleGraphics = 0;
        self.SidewaysFileName = "systemgraphics/trans.bmp";
        self.SymbolColBigFileName = "systemgraphics/trans.bmp";
        self.SymbolColBigFileName2 = "systemgraphics/trans.bmp";
        self.DontShowInList = false;
        self.artCode = new int[10];
      }
      else if (DrawMod.TGame.Data.Version < 162)
      {
        try
        {
          self.ConsiderCarry = info.GetBoolean(nameof (ConsiderCarry));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ConsiderCarry = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.SlotNumber = info.GetInt32(nameof (SlotNumber));
          self.FreeAir = info.GetBoolean(nameof (FreeAir));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.FreeAir = false;
          self.SlotNumber = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Ratio = info.GetInt32(nameof (Ratio));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Ratio = 1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.Unique = info.GetBoolean(nameof (Unique));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.Unique = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.AirAPRule = info.GetInt32(nameof (AirAPRule));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.AirAPRule = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ReinforcementType = info.GetInt32(nameof (ReinforcementType));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ReinforcementType = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ReinforcementType2 = info.GetInt32(nameof (ReinforcementType2));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ReinforcementType2 = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ReinforcementType3 = info.GetInt32(nameof (ReinforcementType3));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ReinforcementType3 = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.DontReturnFromHQ = info.GetBoolean(nameof (DontReturnFromHQ));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.DontReturnFromHQ = false;
          ProjectData.ClearProjectError();
        }
        let mut index11: i32 = 0;
        do
        {
          self.ModelResearch[index11] = -1;
          index11 += 1;
        }
        while (index11 <= 9);
        self.ArtSFType = -1;
        let mut index12: i32 = 0;
        do
        {
          self.HitPointsDef[index12] = self.HitPoints[index12];
          index12 += 1;
        }
        while (index12 <= 99);
        self.FuelRegimeVar = -1;
        self.OutOfSupplyAttack = 1f;
        self.OutOfSupplyDefense = 1f;
        self.OutOfFuelMove = 1f;
        self.OutOfFuelAttack = 1f;
        self.OutOfFuelDefense = 1f;
        self.UsePeopleGraphics = 0;
        self.ArtSFType = -1;
        self.SidewaysFileName = "systemgraphics/trans.bmp";
        self.FirstRoundPenaltyMod = 1f;
        self.ModelRegime = -1;
        self.ModelInitialEvent = -1;
        self.ModelItemType = -1;
        self.ModelInitialForAll = false;
        self.ModelExtraResearch = -1;
        self.LogoString = new string[100];
        self.PreventCounter = -1;
        self.ModelSFTypeUpgrade = 1f;
        self.ModelVariantCounter = -1;
        self.ModelVariantName = new string[1];
        self.ModelVariantExec = new int[1];
        self.ModelVariantCheck = new int[1];
        self.SFTypeVar = new int[100];
        self.ArtSFType = -1;
        self.UsePeopleGraphics = 0;
        self.SidewaysFileName = "systemgraphics/trans.bmp";
        self.SymbolColBigFileName = "systemgraphics/trans.bmp";
        self.SymbolColBigFileName2 = "systemgraphics/trans.bmp";
        self.DontShowInList = false;
        self.artCode = new int[10];
      }
      else
      {
        self.ConsiderCarry = info.GetBoolean(nameof (ConsiderCarry));
        self.SlotNumber = info.GetInt32(nameof (SlotNumber));
        self.FreeAir = info.GetBoolean(nameof (FreeAir));
        self.Ratio = info.GetInt32(nameof (Ratio));
        self.Unique = info.GetBoolean(nameof (Unique));
        self.AirAPRule = info.GetInt32(nameof (AirAPRule));
        self.ReinforcementType = info.GetInt32(nameof (ReinforcementType));
        try
        {
          self.ReinforcementType2 = info.GetInt32(nameof (ReinforcementType2));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ReinforcementType2 = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ReinforcementType3 = info.GetInt32(nameof (ReinforcementType3));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ReinforcementType3 = -1;
          ProjectData.ClearProjectError();
        }
        self.DontReturnFromHQ = info.GetBoolean(nameof (DontReturnFromHQ));
        try
        {
          self.FirstRoundPenaltyMod = info.GetSingle(nameof (FirstRoundPenaltyMod));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.FirstRoundPenaltyMod = 1f;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ChanceOnDeathIfMakeHit = info.GetSingle(nameof (ChanceOnDeathIfMakeHit));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ChanceOnDeathIfMakeHit = 0.0f;
          ProjectData.ClearProjectError();
        }
        try
        {
          index8 = info.GetInt32("rcount");
          self.ModelLastState = new int[index8 + 1];
          self.ModelPossibleImp = new int[index8 + 1];
          self.ModelImproveEvent = new int[index8 + 1];
          self.ModelIsBase = info.GetBoolean(nameof (ModelIsBase));
          self.ModelLevel = info.GetInt32(nameof (ModelLevel));
          self.ModelName = info.GetString(nameof (ModelName));
          self.ModelMark = info.GetInt32(nameof (ModelMark));
          self.ModelLastState = (int[]) info.GetValue(nameof (ModelLastState), self.ModelLastState.GetType());
          self.ModelVersion = info.GetInt32(nameof (ModelVersion));
          self.ModelID = info.GetInt32(nameof (ModelID));
          self.ModelPossibleImp = (int[]) info.GetValue(nameof (ModelPossibleImp), self.ModelPossibleImp.GetType());
          self.ModelCostType = info.GetInt32(nameof (ModelCostType));
          self.ModelCost = info.GetInt32(nameof (ModelCost));
          self.ModelCostPerLevel = info.GetSingle(nameof (ModelCostPerLevel));
          self.ModelCostPerSameModel = info.GetSingle(nameof (ModelCostPerSameModel));
          self.ModelBaseModel = info.GetInt32(nameof (ModelBaseModel));
          self.ModelResearch = (int[]) info.GetValue(nameof (ModelResearch), self.ModelResearch.GetType());
          self.ModelNewEvent = info.GetInt32(nameof (ModelNewEvent));
          self.ModelNameList = info.GetInt32(nameof (ModelNameList));
          self.ModelAllowUpgrade = info.GetBoolean(nameof (ModelAllowUpgrade));
          self.ModelImproveEvent = (int[]) info.GetValue(nameof (ModelImproveEvent), self.ModelImproveEvent.GetType());
          self.ModelAllowImprovements = info.GetBoolean(nameof (ModelAllowImprovements));
          self.ModelImproveCostMod = info.GetSingle(nameof (ModelImproveCostMod));
          self.ModelCostPerSameImp = info.GetSingle(nameof (ModelCostPerSameImp));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut index13: i32 = 0;
          do
          {
            self.ModelResearch[index13] = -1;
            index13 += 1;
          }
          while (index13 <= 9);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ModelRegime = info.GetInt32(nameof (ModelRegime));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ModelRegime = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ModelAutoImprovement = new bool[index8 + 1];
          self.ModelAutoImprovement = (bool[]) info.GetValue(nameof (ModelAutoImprovement), self.ModelAutoImprovement.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ModelInitialForAll = info.GetBoolean(nameof (ModelInitialForAll));
          self.ModelInitialEvent = info.GetInt32(nameof (ModelInitialEvent));
          self.ModelItemType = info.GetInt32(nameof (ModelItemType));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ModelInitialEvent = -1;
          self.ModelItemType = -1;
          self.ModelInitialForAll = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ModelExtraResearch = info.GetInt32(nameof (ModelExtraResearch));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ModelExtraResearch = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.LogoString = (string[]) info.GetValue(nameof (LogoString), self.LogoString.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.LogoString = new string[100];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.PreventCounter = info.GetInt32(nameof (PreventCounter));
          self.MaxPreventPointsUsed = info.GetInt32(nameof (MaxPreventPointsUsed));
          self.MaxPreventPointsGiven = info.GetInt32(nameof (MaxPreventPointsGiven));
          self.PreventHitOn = new int[self.PreventCounter + 1];
          self.PreventHitFrom = new int[self.PreventCounter + 1];
          self.PreventPriority = new int[self.PreventCounter + 1];
          self.PreventChance = new int[self.PreventCounter + 1];
          self.PreventPoints = new int[self.PreventCounter + 1];
          self.PreventHitOn = (int[]) info.GetValue(nameof (PreventHitOn), self.PreventHitOn.GetType());
          self.PreventHitFrom = (int[]) info.GetValue(nameof (PreventHitFrom), self.PreventHitFrom.GetType());
          self.PreventPriority = (int[]) info.GetValue(nameof (PreventPriority), self.PreventPriority.GetType());
          self.PreventChance = (int[]) info.GetValue(nameof (PreventChance), self.PreventChance.GetType());
          self.PreventPoints = (int[]) info.GetValue(nameof (PreventPoints), self.PreventPoints.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.PreventCounter = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.HitPoints = (int[]) info.GetValue(nameof (HitPoints), self.HitPoints.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut index14: i32 = 0;
          do
          {
            self.HitPoints[index14] = self.DefPower;
            index14 += 1;
          }
          while (index14 <= 99);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.HitPointsDef = (int[]) info.GetValue(nameof (HitPointsDef), self.HitPointsDef.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          let mut index15: i32 = 0;
          do
          {
            self.HitPointsDef[index15] = self.HitPoints[index15];
            index15 += 1;
          }
          while (index15 <= 99);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.FuelRegimeVar = info.GetInt32(nameof (FuelRegimeVar));
          self.FuelForMove = info.GetInt32(nameof (FuelForMove));
          self.OutOfFuelMove = info.GetSingle(nameof (OutOfFuelMove));
          self.FuelForAttack = info.GetInt32(nameof (FuelForAttack));
          self.OutOfFuelAttack = info.GetSingle(nameof (OutOfFuelAttack));
          self.OutOfFuelDefense = info.GetSingle(nameof (OutOfFuelDefense));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.FuelRegimeVar = -1;
          self.FuelForMove = 0;
          self.OutOfFuelMove = 1f;
          self.FuelForAttack = 0;
          self.OutOfFuelAttack = 1f;
          self.OutOfFuelDefense = 1f;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.FuelForAttackDef = info.GetInt32(nameof (FuelForAttackDef));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.FuelForAttackDef = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.SupplyForAttackDef = info.GetInt32(nameof (SupplyForAttackDef));
          self.SupplyForAttack = info.GetInt32(nameof (SupplyForAttack));
          self.OutOfSupplyAttack = info.GetSingle(nameof (OutOfSupplyAttack));
          self.OutOfSupplyDefense = info.GetSingle(nameof (OutOfSupplyDefense));
          self.FuelCarry = info.GetInt32(nameof (FuelCarry));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.SupplyForAttackDef = 0;
          self.SupplyForAttack = 0;
          self.OutOfSupplyAttack = 1f;
          self.OutOfSupplyDefense = 1f;
          self.FuelCarry = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ModelSFTypeUpgrade = info.GetSingle(nameof (ModelSFTypeUpgrade));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ModelSFTypeUpgrade = 1f;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ModelVariantCounter = info.GetInt32(nameof (ModelVariantCounter));
          if (self.ModelVariantCounter > -1)
          {
            self.ModelVariantName = new string[self.ModelVariantCounter + 1];
            self.ModelVariantExec = new int[self.ModelVariantCounter + 1];
            self.ModelVariantCheck = new int[self.ModelVariantCounter + 1];
          }
          else
          {
            self.ModelVariantName = new string[1];
            self.ModelVariantExec = new int[1];
            self.ModelVariantCheck = new int[1];
          }
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ModelVariantCounter = -1;
          self.ModelVariantName = new string[1];
          self.ModelVariantExec = new int[1];
          self.ModelVariantCheck = new int[1];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ModelVariantName = (string[]) info.GetValue(nameof (ModelVariantName), self.ModelVariantName.GetType());
          self.ModelVariantCheck = (int[]) info.GetValue(nameof (ModelVariantCheck), self.ModelVariantCheck.GetType());
          self.ModelVariantExec = (int[]) info.GetValue(nameof (ModelVariantExec), self.ModelVariantExec.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.SFTypeVar = (int[]) info.GetValue(nameof (SFTypeVar), self.SFTypeVar.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.SFTypeVar = new int[100];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.artCode = (int[]) info.GetValue(nameof (artCode), self.artCode.GetType());
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.artCode = new int[10];
          ProjectData.ClearProjectError();
        }
        try
        {
          self.ArtSFType = info.GetInt32(nameof (ArtSFType));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.ArtSFType = -1;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.UsePeopleGraphics = info.GetInt32(nameof (UsePeopleGraphics));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.UsePeopleGraphics = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.SidewaysFileName = info.GetString(nameof (SidewaysFileName));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.SidewaysFileName = "systemgraphics/trans.bmp";
          ProjectData.ClearProjectError();
        }
        try
        {
          self.SymbolColBigFileName = info.GetString(nameof (SymbolColBigFileName));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.SymbolColBigFileName = "systemgraphics/trans.bmp";
          ProjectData.ClearProjectError();
        }
        try
        {
          self.SymbolColBigFileName2 = info.GetString(nameof (SymbolColBigFileName2));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.SymbolColBigFileName2 = "systemgraphics/trans.bmp";
          ProjectData.ClearProjectError();
        }
        try
        {
          self.DontShowInList = info.GetBoolean(nameof (DontShowInList));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.DontShowInList = false;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.StockpileDepletedMod = info.GetSingle(nameof (StockpileDepletedMod));
          self.StockpileMax = info.GetInt32(nameof (StockpileMax));
          self.StockpileMaxIn = info.GetInt32(nameof (StockpileMaxIn));
          self.SupplyMaxIn = info.GetInt32(nameof (SupplyMaxIn));
          self.StockpileUsedPerRound = info.GetInt32(nameof (StockpileUsedPerRound));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          ProjectData.ClearProjectError();
        }
        try
        {
          self.DepletingHitpointRule = info.GetInt32(nameof (DepletingHitpointRule));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.DepletingHitpointRule = 0;
          ProjectData.ClearProjectError();
        }
        try
        {
          self.StartCombatRound = info.GetInt32(nameof (StartCombatRound));
          self.EndCombatRound = info.GetInt32(nameof (EndCombatRound));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.StartCombatRound = 0;
          self.EndCombatRound = 0;
          ProjectData.ClearProjectError();
        }
      }
      try
      {
        self.LibId = LibIdClass::new();
        self.LibId = (LibIdClass) info.GetValue(nameof (LibId), self.LibId.GetType());
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.LibId = LibIdClass::new();
        self.LibId.id = -1;
        self.LibId.libSlot = -1;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.Id = info.GetInt32(nameof (Id));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.Id = -1;
        ProjectData.ClearProjectError();
      }
      if (DrawMod.TGame.Data.Version < 130)
      {
        self.BaseColor = 0;
      }
      else
      {
        try
        {
          self.BaseColor = info.GetInt32(nameof (BaseColor));
        }
        catch (Exception ex)
        {
          ProjectData.SetProjectError(ex);
          self.BaseColor = 0;
          ProjectData.ClearProjectError();
        }
      }
      try
      {
        self.heightLevelDiff = info.GetInt32(nameof (heightLevelDiff));
        self.directRange = info.GetInt32(nameof (directRange));
        self.directModFirstHex = info.GetInt32(nameof (directModFirstHex));
        self.directModPerHex = info.GetInt32(nameof (directModPerHex));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.directRange = 0;
        self.heightLevelDiff = 0;
        self.directModFirstHex = 0;
        self.directModPerHex = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.targettedByRangedChance = info.GetInt32(nameof (targettedByRangedChance));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.targettedByRangedChance = 0;
        ProjectData.ClearProjectError();
      }
      try
      {
        self.scrapable = info.GetInt32(nameof (scrapable));
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        self.scrapable = 0;
        ProjectData.ClearProjectError();
      }
      if (self.Ratio != 0)
        return;
      self.Ratio = 1;
    }

    pub SFTypeClass(hardcoded: i32, ltcount: i32, rcount: i32)
    {
      self.ExtraPicFileName = new string[1];
      self.ExtraPicSpriteID = new int[1];
      self.ExtraSidewaysFileName = new string[1];
      self.ExtraSidewaysSpriteID = new int[1];
      self.ExtraSymbolFileName = new string[1];
      self.ExtraSymbolSpriteID = new int[1];
      self.ExtraSymbolFileName2 = new string[1];
      self.ExtraSymbolSprite2ID = new int[1];
      self.ExtraSymbolColBigFileName = new string[1];
      self.ExtraSymbolColBigSpriteID = new int[1];
      self.ExtraSymbolColBigFileName2 = new string[1];
      self.ExtraSymbolColBigSprite2ID = new int[1];
      self.ExtraCode = new int[1];
      self.ExtraName = new string[1];
      self.PeopleGroup = new bool[100];
      self.FavTarget = new int[100];
      self.FavArtTarget = new int[100];
      self.AttackPower = new int[100];
      self.AttackPowerDef = new int[100];
      self.AttackArt = new object[100];
      self.CombatModAtt = new float[1];
      self.CombatModDef = new float[1];
      self.ExtraRecon = new int[1];
      self.AIRoleScore = new int[50];
      self.ModelLastState = new int[2];
      self.ModelPossibleImp = new int[2];
      self.ModelResearch = new int[10];
      self.ModelImproveEvent = new int[2];
      self.ModelAutoImprovement = new bool[2];
      self.TempAlterationPossible = new bool[2];
      self.LogoString = new string[100];
      self.PreventHitOn = new int[2];
      self.PreventHitFrom = new int[2];
      self.PreventChance = new int[2];
      self.PreventPoints = new int[2];
      self.PreventPriority = new int[2];
      self.HitPoints = new int[100];
      self.HitPointsDef = new int[100];
      self.ModelVariantName = new string[1];
      self.ModelVariantCheck = new int[1];
      self.ModelVariantExec = new int[1];
      self.artCode = new int[10];
      self.SFTypeVar = new int[100];
      self.ModelVariantCounter = -1;
      self.ArtSFType = -1;
      self.scrapable = 0;
      self.manpower = 0;
      self.manpowerCarry = 0;
      self.targettedByRangedChance = 0;
      self.LibId = LibIdClass::new();
      self.SidewaysFileName = "systemgraphics/trans.bmp";
      self.Name = "Default SubFormation Type";
      self.PicFileName = "systemgraphics/trans.bmp";
      self.SymbolFileName = "systemgraphics/trans.bmp";
      self.SymbolFileName2 = "systemgraphics/trans.bmp";
      self.SymbolColBigFileName = "systemgraphics/trans.bmp";
      self.SymbolColBigFileName2 = "systemgraphics/trans.bmp";
      self.SymbolGroup = -1;
      self.FirstRoundPenaltyMod = 1f;
      self.SymbolWeight = 0;
      self.ModelExtraResearch = -1;
      self.CopyDataFrom = -1;
      self.CopyDataFromBackup = -1;
      self.KillIsRegVar = -1;
      self.ModelRegime = -1;
      self.PreventCounter = -1;
      self.ReinforcementType = -1;
      self.ReinforcementType2 = -1;
      self.ReinforcementType3 = -1;
      self.StaffPts = 0;
      self.ModelSFTypeUpgrade = 1f;
      self.ApMod = 1f;
      self.FuelRegimeVar = -1;
      self.FuelForMove = 0;
      self.OutOfFuelMove = 1f;
      self.FuelForAttack = 0;
      self.FuelForAttackDef = 0;
      self.SupplyForAttack = 0;
      self.SupplyForAttackDef = 0;
      self.OutOfSupplyAttack = 1f;
      self.OutOfSupplyDefense = 1f;
      self.OutOfFuelAttack = 1f;
      self.OutOfFuelDefense = 1f;
      self.DepletingHitpointRule = 0;
      self.StartCombatRound = 0;
      self.EndCombatRound = 0;
      let mut index1: i32 = 0;
      do
      {
        self.PeopleGroup[index1] = false;
        index1 += 1;
      }
      while (index1 <= 19);
      if (ltcount > -1)
      {
        self.CombatModDef = new float[ltcount + 1];
        self.CombatModAtt = new float[ltcount + 1];
        self.ExtraRecon = new int[ltcount + 1];
        let mut num: i32 = ltcount;
        for (let mut index2: i32 = 0; index2 <= num; index2 += 1)
        {
          self.CombatModDef[index2] = 1f;
          self.CombatModAtt[index2] = 1f;
          self.ExtraRecon[index2] = 0;
        }
      }
      let mut index3: i32 = 0;
      do
      {
        self.AttackArt[index3] =  0;
        index3 += 1;
      }
      while (index3 <= 99);
      self.UpgradeToo = -1;
      self.UpgradeCost = 0;
      self.UpgradeXP = 0;
      self.ExtraCounter = -1;
      self.SlotNumber = -1;
      self.AirAPRule = -1;
      self.Ratio = 1;
      if (rcount > -1)
      {
        self.ModelLastState = new int[rcount + 1];
        self.ModelPossibleImp = new int[rcount + 1];
        self.ModelImproveEvent = new int[rcount + 1];
        self.ModelAutoImprovement = new bool[rcount + 1];
      }
      let mut index4: i32 = 0;
      do
      {
        self.ModelResearch[index4] = -1;
        index4 += 1;
      }
      while (index4 <= 9);
      self.ModelItemType = -1;
      self.ModelInitialEvent = -1;
      self.directRange = 0;
      self.directModFirstHex = 0;
      self.directModPerHex = 0;
      self.heightLevelDiff = 0;
    }

    pub fn AddResField()
    {
      self.ModelLastState = (int[]) Utils.CopyArray((Array) self.ModelLastState, (Array) new int[self.ModelLastState.GetUpperBound(0) + 1 + 1]);
      self.ModelPossibleImp = (int[]) Utils.CopyArray((Array) self.ModelPossibleImp, (Array) new int[self.ModelLastState.GetUpperBound(0) + 1 + 1]);
      self.ModelImproveEvent = (int[]) Utils.CopyArray((Array) self.ModelImproveEvent, (Array) new int[self.ModelLastState.GetUpperBound(0) + 1 + 1]);
      self.ModelAutoImprovement = (bool[]) Utils.CopyArray((Array) self.ModelAutoImprovement, (Array) new bool[self.ModelLastState.GetUpperBound(0) + 1 + 1]);
    }

    pub fn RemoveResField(nr: i32)
    {
      if (nr < self.ModelLastState.GetUpperBound(0))
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.ModelLastState.GetUpperBound(0) - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
        {
          self.ModelLastState[index] = self.ModelLastState[index + 1];
          self.ModelPossibleImp[index] = self.ModelPossibleImp[index + 1];
          self.ModelImproveEvent[index] = self.ModelImproveEvent[index + 1];
        }
      }
      if (self.ModelLastState.GetUpperBound(0) <= 0)
        return;
      self.ModelLastState = (int[]) Utils.CopyArray((Array) self.ModelLastState, (Array) new int[self.ModelLastState.GetUpperBound(0) - 1 + 1]);
      self.ModelPossibleImp = (int[]) Utils.CopyArray((Array) self.ModelPossibleImp, (Array) new int[self.ModelPossibleImp.GetUpperBound(0) - 1 + 1]);
      self.ModelImproveEvent = (int[]) Utils.CopyArray((Array) self.ModelImproveEvent, (Array) new int[self.ModelImproveEvent.GetUpperBound(0) - 1 + 1]);
    }

    pub SFTypeClass Clone()
    {
      BinaryFormatter binaryFormatter = BinaryFormatter::new();
      MemoryStream serializationStream = MemoryStream::new();
      binaryFormatter.Serialize((Stream) serializationStream,  this);
      serializationStream.Position = 0L;
      return (SFTypeClass) binaryFormatter.Deserialize((Stream) serializationStream);
    }

    pub fn ReplaceSymbolSprite(s: String)
    {
      self.SymbolFileName = s;
      self.SymbolSpriteID = BitmapStore.ReloadFile(self.SymbolSpriteID, s, IsBig: true);
    }

    pub fn ReplaceColBigSymbolSprite(s: String)
    {
      self.SymbolColBigFileName = s;
      self.SymbolColBigSpriteID = BitmapStore.ReloadFile(self.SymbolColBigSpriteID, s);
    }

    pub fn ReplaceColBigSymbolSprite2(s: String)
    {
      self.SymbolColBigFileName2 = s;
      self.SymbolColBigSprite2ID = BitmapStore.ReloadFile(self.SymbolColBigSprite2ID, s);
    }

    pub fn ReplaceSymbolSprite2(s: String)
    {
      self.SymbolFileName2 = s;
      self.SymbolSprite2ID = BitmapStore.ReloadFile(self.SymbolSprite2ID, s);
    }

    pub fn ReplacePicSprite(s: String)
    {
      self.PicFileName = s;
      self.PicSpriteID = BitmapStore.ReloadFile(self.PicSpriteID, s);
    }

    pub fn ReplaceSidewaysSprite(s: String)
    {
      self.SidewaysFileName = s;
      self.SidewaysSpriteID = BitmapStore.ReloadFile(self.SidewaysSpriteID, s);
    }

    pub fn Kill()
    {
      BitmapStore.RemoveBitmapNr(self.SymbolSpriteID);
      BitmapStore.RemoveBitmapNr(self.SymbolSprite2ID);
      BitmapStore.RemoveBitmapNr(self.PicSpriteID);
      BitmapStore.RemoveBitmapNr(self.SidewaysSpriteID);
      BitmapStore.RemoveBitmapNr(self.SymbolColBigSpriteID);
      BitmapStore.RemoveBitmapNr(self.SymbolColBigSprite2ID);
      let mut extraCounter: i32 = self.ExtraCounter;
      for (let mut index: i32 = 0; index <= extraCounter; index += 1)
      {
        BitmapStore.RemoveBitmapNr(self.ExtraPicSpriteID[index]);
        BitmapStore.RemoveBitmapNr(self.ExtraSidewaysSpriteID[index]);
        BitmapStore.RemoveBitmapNr(self.ExtraSymbolSpriteID[index]);
        BitmapStore.RemoveBitmapNr(self.ExtraSymbolSprite2ID[index]);
        BitmapStore.RemoveBitmapNr(self.ExtraSymbolColBigSpriteID[index]);
        BitmapStore.RemoveBitmapNr(self.ExtraSymbolColBigSprite2ID[index]);
      }
    }

    pub fn LoadSprites()
    {
      self.SymbolSpriteID = BitmapStore.AddFile(self.SymbolFileName, false, true);
      self.SymbolSprite2ID = BitmapStore.AddFile(self.SymbolFileName2, false);
      self.PicSpriteID = BitmapStore.AddFile(self.PicFileName, false);
      self.SidewaysSpriteID = BitmapStore.AddFile(self.SidewaysFileName, false);
      self.SymbolColBigSpriteID = BitmapStore.AddFile(self.SymbolColBigFileName, false);
      self.SymbolColBigSprite2ID = BitmapStore.AddFile(self.SymbolColBigFileName2, false);
      let mut extraCounter: i32 = self.ExtraCounter;
      for (let mut index: i32 = 0; index <= extraCounter; index += 1)
      {
        self.ExtraPicSpriteID[index] = BitmapStore.AddFile(self.ExtraPicFileName[index], false);
        self.ExtraSidewaysSpriteID[index] = BitmapStore.AddFile(self.ExtraSidewaysFileName[index], false);
        self.ExtraSymbolSpriteID[index] = BitmapStore.AddFile(self.ExtraSymbolFileName[index], false, true);
        self.ExtraSymbolSprite2ID[index] = BitmapStore.AddFile(self.ExtraSymbolFileName2[index], false);
        self.ExtraSymbolColBigSpriteID[index] = BitmapStore.AddFile(self.ExtraSymbolColBigFileName[index], false);
        self.ExtraSymbolColBigSprite2ID[index] = BitmapStore.AddFile(self.ExtraSymbolColBigFileName2[index], false);
      }
    }

    pub fn AddExtraSprite()
    {
      this += 1.ExtraCounter;
      self.ExtraPicFileName = (string[]) Utils.CopyArray((Array) self.ExtraPicFileName, (Array) new string[self.ExtraCounter + 1]);
      self.ExtraPicSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraPicSpriteID, (Array) new int[self.ExtraCounter + 1]);
      self.ExtraSidewaysFileName = (string[]) Utils.CopyArray((Array) self.ExtraSidewaysFileName, (Array) new string[self.ExtraCounter + 1]);
      self.ExtraSidewaysSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSidewaysSpriteID, (Array) new int[self.ExtraCounter + 1]);
      self.ExtraSymbolFileName = (string[]) Utils.CopyArray((Array) self.ExtraSymbolFileName, (Array) new string[self.ExtraCounter + 1]);
      self.ExtraSymbolFileName2 = (string[]) Utils.CopyArray((Array) self.ExtraSymbolFileName2, (Array) new string[self.ExtraCounter + 1]);
      self.ExtraSymbolSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolSpriteID, (Array) new int[self.ExtraCounter + 1]);
      self.ExtraSymbolSprite2ID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolSprite2ID, (Array) new int[self.ExtraCounter + 1]);
      self.ExtraSymbolColBigFileName = (string[]) Utils.CopyArray((Array) self.ExtraSymbolColBigFileName, (Array) new string[self.ExtraCounter + 1]);
      self.ExtraSymbolColBigSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolColBigSpriteID, (Array) new int[self.ExtraCounter + 1]);
      self.ExtraSymbolColBigFileName2 = (string[]) Utils.CopyArray((Array) self.ExtraSymbolColBigFileName2, (Array) new string[self.ExtraCounter + 1]);
      self.ExtraSymbolColBigSprite2ID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolColBigSprite2ID, (Array) new int[self.ExtraCounter + 1]);
      self.ExtraCode = (int[]) Utils.CopyArray((Array) self.ExtraCode, (Array) new int[self.ExtraCounter + 1]);
      self.ExtraName = (string[]) Utils.CopyArray((Array) self.ExtraName, (Array) new string[self.ExtraCounter + 1]);
      self.ExtraPicFileName[self.ExtraCounter] = self.PicFileName;
      self.ExtraPicSpriteID[self.ExtraCounter] = BitmapStore.AddFile(self.ExtraPicFileName[self.ExtraCounter], false);
      self.ExtraSidewaysFileName[self.ExtraCounter] = self.PicFileName;
      self.ExtraSidewaysSpriteID[self.ExtraCounter] = BitmapStore.AddFile(self.ExtraPicFileName[self.ExtraCounter], false);
      self.ExtraSymbolFileName[self.ExtraCounter] = self.SymbolFileName;
      self.ExtraSymbolSpriteID[self.ExtraCounter] = BitmapStore.AddFile(self.ExtraSymbolFileName[self.ExtraCounter], false, true);
      self.ExtraSymbolFileName2[self.ExtraCounter] = "systemgraphics/trans.bmp";
      self.ExtraSymbolSprite2ID[self.ExtraCounter] = BitmapStore.AddFile(self.ExtraSymbolFileName2[self.ExtraCounter], false);
      self.ExtraSymbolColBigFileName[self.ExtraCounter] = "systemgraphics/trans.bmp";
      self.ExtraSymbolColBigSpriteID[self.ExtraCounter] = BitmapStore.AddFile(self.ExtraSymbolColBigFileName[self.ExtraCounter], false);
      self.ExtraSymbolColBigFileName2[self.ExtraCounter] = "systemgraphics/trans.bmp";
      self.ExtraSymbolColBigSprite2ID[self.ExtraCounter] = BitmapStore.AddFile(self.ExtraSymbolColBigFileName2[self.ExtraCounter], false);
      num1: i32;
      if (self.ExtraCounter > 0)
      {
        num1 = 1;
        let mut num2: i32 = self.ExtraCounter - 1;
        for (let mut index: i32 = 0; index <= num2; index += 1)
        {
          if (self.ExtraCode[index] >= num1)
            num1 = self.ExtraCode[index] + 1;
        }
      }
      else
        num1 = 1;
      self.ExtraCode[self.ExtraCounter] = num1;
    }

    pub fn RemoveExtraSprite(nr: i32)
    {
      if (self.ExtraCounter > nr)
      {
        let mut num1: i32 = nr;
        let mut num2: i32 = self.ExtraCounter - 1;
        for (let mut index: i32 = num1; index <= num2; index += 1)
        {
          self.ExtraPicFileName[nr] = self.ExtraPicFileName[nr + 1];
          self.ExtraPicSpriteID[nr] = self.ExtraPicSpriteID[nr + 1];
          self.ExtraSymbolFileName[nr] = self.ExtraSymbolFileName[nr + 1];
          self.ExtraSymbolFileName2[nr] = self.ExtraSymbolFileName2[nr + 1];
          self.ExtraSymbolSpriteID[nr] = self.ExtraSymbolSpriteID[nr + 1];
          self.ExtraSymbolSprite2ID[nr] = self.ExtraSymbolSprite2ID[nr + 1];
          self.ExtraSidewaysFileName[nr] = self.ExtraSidewaysFileName[nr + 1];
          self.ExtraSidewaysSpriteID[nr] = self.ExtraSidewaysSpriteID[nr + 1];
          self.ExtraSymbolColBigFileName[nr] = self.ExtraSymbolColBigFileName[nr + 1];
          self.ExtraSymbolColBigSpriteID[nr] = self.ExtraSymbolColBigSpriteID[nr + 1];
          self.ExtraSymbolColBigFileName2[nr] = self.ExtraSymbolColBigFileName2[nr + 1];
          self.ExtraSymbolColBigSprite2ID[nr] = self.ExtraSymbolColBigSprite2ID[nr + 1];
          self.ExtraCode[nr] = self.ExtraCode[nr + 1];
        }
        --self.ExtraCounter;
        self.ExtraPicFileName = (string[]) Utils.CopyArray((Array) self.ExtraPicFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraPicSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraPicSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolFileName = (string[]) Utils.CopyArray((Array) self.ExtraSymbolFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolFileName2 = (string[]) Utils.CopyArray((Array) self.ExtraSymbolFileName2, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolSprite2ID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolSprite2ID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraCode = (int[]) Utils.CopyArray((Array) self.ExtraCode, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSidewaysFileName = (string[]) Utils.CopyArray((Array) self.ExtraSidewaysFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSidewaysSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSidewaysSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigFileName = (string[]) Utils.CopyArray((Array) self.ExtraSymbolColBigFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolColBigSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigFileName2 = (string[]) Utils.CopyArray((Array) self.ExtraSymbolColBigFileName2, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigSprite2ID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolColBigSprite2ID, (Array) new int[self.ExtraCounter + 1]);
      }
      else
      {
        --self.ExtraCounter;
        self.ExtraPicFileName = (string[]) Utils.CopyArray((Array) self.ExtraPicFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraPicSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraPicSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolFileName = (string[]) Utils.CopyArray((Array) self.ExtraSymbolFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolFileName2 = (string[]) Utils.CopyArray((Array) self.ExtraSymbolFileName2, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolSprite2ID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolSprite2ID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraCode = (int[]) Utils.CopyArray((Array) self.ExtraCode, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSidewaysFileName = (string[]) Utils.CopyArray((Array) self.ExtraSidewaysFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSidewaysSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSidewaysSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigFileName = (string[]) Utils.CopyArray((Array) self.ExtraSymbolColBigFileName, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigSpriteID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolColBigSpriteID, (Array) new int[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigFileName2 = (string[]) Utils.CopyArray((Array) self.ExtraSymbolColBigFileName2, (Array) new string[self.ExtraCounter + 1]);
        self.ExtraSymbolColBigSprite2ID = (int[]) Utils.CopyArray((Array) self.ExtraSymbolColBigSprite2ID, (Array) new int[self.ExtraCounter + 1]);
      }
    }

    pub fn ReplaceExtraPic(nr: i32, filename: String)
    {
      self.ExtraPicFileName[nr] = filename;
      self.ExtraPicSpriteID[nr] = BitmapStore.ReloadFile(self.ExtraPicSpriteID[nr], self.ExtraPicFileName[nr]);
    }

    pub fn ReplaceExtraSideways(nr: i32, filename: String)
    {
      self.ExtraSidewaysFileName[nr] = filename;
      self.ExtraSidewaysSpriteID[nr] = BitmapStore.ReloadFile(self.ExtraSidewaysSpriteID[nr], self.ExtraSidewaysFileName[nr]);
    }

    pub fn ReplaceExtraSymbol(nr: i32, filename: String)
    {
      self.ExtraSymbolFileName[nr] = filename;
      self.ExtraSymbolSpriteID[nr] = BitmapStore.ReloadFile(self.ExtraSymbolSpriteID[nr], self.ExtraSymbolFileName[nr], IsBig: true);
    }

    pub fn ReplaceExtraColBigSymbol(nr: i32, filename: String)
    {
      self.ExtraSymbolColBigFileName[nr] = filename;
      self.ExtraSymbolColBigSpriteID[nr] = BitmapStore.ReloadFile(self.ExtraSymbolColBigSpriteID[nr], self.ExtraSymbolColBigFileName[nr]);
    }

    pub fn ReplaceExtraColBigSymbol2(nr: i32, filename: String)
    {
      self.ExtraSymbolColBigFileName2[nr] = filename;
      self.ExtraSymbolColBigSprite2ID[nr] = BitmapStore.ReloadFile(self.ExtraSymbolColBigSprite2ID[nr], self.ExtraSymbolColBigFileName2[nr]);
    }

    pub fn ReplaceExtraSymbol2(nr: i32, filename: String)
    {
      self.ExtraSymbolFileName2[nr] = filename;
      self.ExtraSymbolSprite2ID[nr] = BitmapStore.ReloadFile(self.ExtraSymbolSprite2ID[nr], self.ExtraSymbolFileName2[nr]);
    }
  }
}
