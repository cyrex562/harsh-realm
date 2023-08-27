// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.HelperEconomyData
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;

namespace WindowsApplication1
{
  pub class HelperEconomyData
  {
    pub slotMilitiaUnits: i32;
    pub slotZoneRegKeys: i32;
    pub slotModels: i32;
    pub slotMilitiaTroops: i32;
    pub slotMilitiaNames: i32;
    pub slotProdType: i32;
    pub slotZones: i32;
    pub slotZoneKeys: i32;
    pub slotAssetTypes: i32;
    pub slotAssets: i32;
    pub slotNeighbours: i32;
    pub slotRegimes: i32;
    pub slotConstructionCost: i32;
    pub slotUpkeepCost: i32;
    pub slotProdCost: i32;
    pub assetZoneId: i32;
    pub city: i32;
    pub phase: i32;
    pub superphase: i32;
    pub assetLackOfUpkeep: i32;
    pub assetInConstr: i32;
    pub assetConstrRoundsLeft: i32;
    pub assetTypeId: i32;
    pub level: i32;
    pub ax: i32;
    pub ay: i32;
    pub assetProdId: i32;
    pub costProdId: i32;
    pub costAssetTypeId: i32;
    pub costType: i32;
    pub costItemType: i32;
    pub costQty: i32;
    pub prodItemType: i32;
    pub prodQty: i32;
    pub ProdTypeType: i32;
    pub zoneValue: i32;
    pub workerHapiness: i32;
    pub popHapiness: i32;
    pub popHunger: i32;
    pub recruitPenalty: i32;
    pub workerHunger: i32;
    pub popLoyalty: i32;
    pub zoneKey: String;
    pub zoneName: String;
    pub workerSalaryNeed: i32;
    pub workerSalaryBuyFood: i32;
    pub slotRegFeats: i32;
    pub workerSalaryGiven: i32;
    pub recruitBonus: i32;
    pub colonistBonus: i32;
    pub privateMetal: i32;
    pub privateWater: i32;
    pub privateOil: i32;
    pub cas: i32;
    pub worker: i32;
    pub pop: i32;
    pub recruits: i32;
    pub colonists: i32;
    pub food: i32;
    pub workerPoints: i32;
    pub popPoints: i32;
    pub privateFood: i32;
    pub rebelCredits: i32;
    pub netPrivateIncome: i32;
    pub emergencyFoodGiven: i32;
    pub freeFolk: i32;
    pub hiddenEconomyIncome: i32;
    pub popCredits: i32;
    pub workerCredits: i32;
    pub occupationMode: i32;
    pub rebelStrength: i32;
    pub rebelManpower: i32;
    pub rebelRegime: i32;
    pub highestLoyalty: i32;
    pub highestLoyaltyRegime: i32;
    pub publicCredits: i32;
    pub workerSpendingIncome: i32;
    pub sellingFoodIncome: i32;
    pub privateAssetIncome: i32;
    pub buyingFoodExpenses: i32;
    pub buyingLuxuriesExpenses: i32;
    pub BuyingAssetsExpenses: i32;
    pub BuyingPopExpenses: i32;
    pub publicBudget: i32;
    pub publicBudgetGiven: i32;
    pub slotOrg: i32;
    pub slotAssetPresentation: i32;
    pub privateLuxury: i32;
    pub garrisonFacility: i32;
    pub culture: i32;
    pub unrest: i32;
    pub danger: i32;
    pub fear: i32;
    pub qol: i32;
    pub health: i32;
    pub entertainment: i32;
    pub education: i32;
    pub security: i32;
    pub luxury: i32;
    pub qol_health: i32;
    pub qol_entertainment: i32;
    pub qol_education: i32;
    pub qol_security: i32;
    pub qol_luxury: i32;
    pub maxWater: i32;
    pub maxEnergy: i32;
    pub maxAmmo: i32;
    pub maxFuel: i32;
    pub maxFood: i32;
    pub maxIP: i32;
    pub orig_food: i32;
    pub orig_privateFood: i32;
    pub foodConsumed: i32;
    pub privateFoodConsumed: i32;
    pub zoneId: i32;
    pub zoneNr: i32;
    pub locNr: i32;
    pub locId: i32;
    pub reg: i32;
    pub regid: i32;
    pub regtype: i32;
    pub regCulture: i32;
    pub regCultureGroup: i32;
    pub militiaManpower: i32;
    pub militiaEquipment: i32;
    pub militancy: i32;
    pub origRegimeId: i32;
    pub origRegimeNr: i32;
    pub currentRegimeId: i32;
    pub currentRegimeNr: i32;
    pub water: i32;
    pub oxygen: i32;
    pub zoneCultureId: i32;
    pub AIMatrix zones;
    pub AIMatrix hexFreeFolk;
    pub AIMatrix temperatures;
    pub AIMatrix rad;
    pub privateCreditsGrowth: i32;
    pub governorCharId: i32;
    pub regimeCredits: i32;
    pub administrativeStrain: i32;
    pub slotCulture: i32;
    pub slotTradeLog: i32;
    pub slotCultureKeys: i32;
    pub slotCultureGroup: i32;
    pub slotGfxSet: i32;
    pub slotStats: i32;
    pub slotCulturePlanet: i32;
    pub slotBaseValues: i32;
    pub slotZoneOrders: i32;
    pub slotCharacter: i32;
    pub slotRegReg: i32;
    pub slotFlags: i32;
    pub slotFlagInstructions: i32;
    pub slotGameKeys: i32;
    pub slotTraders: i32;
    pub slotTraderZones: i32;
    pub slotTraderItems: i32;
    pub slotRegimeKeys: i32;
    pub slotItemType: i32;
    pub bioHazard: i32;
    pub atmosHazard: i32;
    pub tempExposureWorkerLoss: i32;
    pub tempExposurePopLoss: i32;
    pub tempExposureFreeFolkLoss: i32;

    pub HelperEconomyData(ref tgame: GameClass, dataLib: String)
    {
      libName1: String = "SE_Trade".to_owned();
      this.zones = new AIMatrix(ref tgame.DC2AIObj);
      this.hexFreeFolk = new AIMatrix(ref tgame.DC2AIObj);
      this.temperatures = new AIMatrix(ref tgame.DC2AIObj);
      this.rad = new AIMatrix(ref tgame.DC2AIObj);
      let mut mapWidth: i32 =  tgame.Data.MapObj[0].MapWidth;
      let mut mapHeight: i32 =  tgame.Data.MapObj[0].MapHeight;
      this.atmosHazard =  Math.Round(Conversion.Val(tgame.Data.StringListObj[this.slotGameKeys].GetData(0, 1, 2)));
      this.bioHazard =  Math.Round(Conversion.Val(tgame.Data.StringListObj[this.slotGameKeys].GetData(0, 2, 2)));
      data1: DataClass = tgame.Data;
      str1: String = "Zones".to_owned();
      ref local1: String = ref str1;
      libName2: String = dataLib;
      let mut libVar1: i32 =  data1.FindLibVar(ref local1, libName2);
      data2: DataClass = tgame.Data;
      str2: String = "Temperature".to_owned();
      ref local2: String = ref str2;
      libName3: String = dataLib;
      let mut libVar2: i32 =  data2.FindLibVar(ref local2, libName3);
      data3: DataClass = tgame.Data;
      str3: String = nameof (rad);
      ref local3: String = ref str3;
      libName4: String = dataLib;
      let mut libVar3: i32 =  data3.FindLibVar(ref local3, libName4);
      data4: DataClass = tgame.Data;
      str4: String = "freefolk".to_owned();
      ref local4: String = ref str4;
      libName5: String = dataLib;
      let mut libVar4: i32 =  data4.FindLibVar(ref local4, libName5);
      let mut num1: i32 =  mapWidth;
      for (let mut index1: i32 =  0; index1 <= num1; index1 += 1)
      {
        let mut num2: i32 =  mapHeight;
        for (let mut index2: i32 =  0; index2 <= num2; index2 += 1)
        {
          this.zones.Value[index1, index2] = tgame.Data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar1);
          this.temperatures.Value[index1, index2] = tgame.Data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar2);
          this.rad.Value[index1, index2] = tgame.Data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar3);
          this.hexFreeFolk.Value[index1, index2] = tgame.Data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar4);
        }
      }
      this.slotModels = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 228, 0, 0));
      this.slotAssetPresentation = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 166, 0, 0));
      this.slotZones = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 123, 0, 0));
      this.slotOrg = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 184, 0, 0));
      this.slotZoneOrders = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 289, 0, 0));
      this.slotZoneKeys = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 160, 0, 0));
      this.slotGameKeys = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 156, 0, 0));
      this.slotNeighbours = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 157, 0, 0));
      this.slotRegimes = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 143, 0, 0));
      this.slotAssets = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 148, 0, 0));
      this.slotAssetTypes = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 144, 0, 0));
      this.slotConstructionCost = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 150, 0, 0));
      this.slotUpkeepCost = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 145, 0, 0));
      this.slotProdCost = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 146, 0, 0));
      this.slotProdType = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 147, 0, 0));
      this.slotMilitiaUnits = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 170, 0, 0));
      this.slotMilitiaTroops = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 171, 0, 0));
      this.slotMilitiaNames = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 172, 0, 0));
      this.slotBaseValues = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(libName1, 251, 0, 0));
      this.slotTraders = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(libName1, 252, 0, 0));
      this.slotTraderZones = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(libName1, 253, 0, 0));
      this.slotTraderItems = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(libName1, 254, 0, 0));
      this.slotRegimeKeys = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 210, 0, 0));
      this.slotItemType = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 149, 0, 0));
      this.slotFlagInstructions = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 169, 0, 0));
      this.slotFlags = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 168, 0, 0));
      this.slotRegReg = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 275, 0, 0));
      this.slotCharacter = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 196, 0, 0));
      this.slotCulture = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 306, 0, 0));
      this.slotCultureGroup = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 310, 0, 0));
      this.slotGfxSet = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 311, 0, 0));
      this.slotCulturePlanet = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 312, 0, 0));
      this.slotCultureKeys = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 308, 0, 0));
      this.slotStats = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 292, 0, 0));
      this.slotTradeLog = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 400, 0, 0));
      this.slotRegFeats = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 388, 0, 0));
      this.slotZoneRegKeys = tgame.HandyFunctionsObj.GetStringListByID(tgame.EventRelatedObj.CheckStringlistID(dataLib, 288, 0, 0));
      if (tgame.Data.Turn <= -1)
        return;
      this.regid = tgame.Data.RegimeObj[tgame.Data.Turn].id;
      this.regtype =  Math.Round(Conversion.Val(tgame.Data.StringListObj[this.slotRegimes].GetData(0, this.regid, 1)));
    }

    pub fn Input(let mut useRegimeId: i32 =  -1)
    {
      if (DrawMod.TGame.Data.Turn == 1)
        this.water = this.water;
      this.worker = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "worker", 2))));
      this.pop = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "pop", 2))));
      this.workerPoints = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerPoints", 2))));
      this.popPoints = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "popPoints", 2))));
      this.privateFood = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "privateFood", 2))));
      this.privateMetal = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "privateMetal", 2))));
      this.privateOil = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "privateOil", 2))));
      this.privateWater = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "privateWater", 2))));
      this.orig_privateFood = this.privateFood;
      this.popHunger = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "popHunger", 2))));
      this.popHapiness = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "popHapiness", 2))));
      this.popLoyalty = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "popLoyalty", 2))));
      this.workerHunger = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerHunger", 2))));
      this.workerHapiness = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerHapiness", 2))));
      this.militiaManpower = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "militiaManpower", 2))));
      this.militiaEquipment = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "militiaEquipment", 2))));
      this.militancy = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "militancy", 2))));
      this.cas = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "cas", 2))));
      this.regtype =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotRegimes].GetData(0, this.regid, 1)));
      this.regCulture =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotRegimes].GetData(0, this.regid, 2)));
      this.regCultureGroup =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotCulture].GetData(0, this.regCulture, 1)));
      this.privateCreditsGrowth = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "privateCreditsGrowth", 2))));
      this.zoneCultureId =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZones].GetData(0, this.zoneId, 9)));
      this.locNr = -1;
      this.locId =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZones].GetData(0, this.zoneId, 6)));
      if (this.locId > 0)
        this.locNr = DrawMod.TGame.HandyFunctionsObj.GetLocationByID(this.locId);
      this.rebelCredits = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "rebelCredits", 2))));
      this.popCredits = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "popCredits", 2))));
      this.hiddenEconomyIncome = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "hiddenEconomyIncome", 2))));
      this.privateAssetIncome = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "privateAssetIncome", 2))));
      this.workerSpendingIncome = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerSpendingIncome", 2))));
      this.sellingFoodIncome = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "sellingFoodIncome", 2))));
      this.buyingFoodExpenses = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "buyingFoodExpenses", 2))));
      this.buyingLuxuriesExpenses = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "buyingLuxuriesExpenses", 2))));
      this.BuyingAssetsExpenses = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "BuyingAssetsExpenses", 2))));
      this.BuyingPopExpenses = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "BuyingPopExpenses", 2))));
      this.netPrivateIncome = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "netPrivateIncome", 2))));
      this.recruitPenalty = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "recruitPenalty", 2))));
      this.workerCredits = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerCredits", 2))));
      this.occupationMode = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "occupationMode", 2))));
      this.rebelStrength = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "rebelStrength", 2))));
      this.rebelManpower = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "rebelManpower", 2))));
      this.administrativeStrain = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "administrativeStrain", 2))));
      this.rebelRegime = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "rebelRegime", 2))));
      this.highestLoyalty = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "highestLoyalty", 2))));
      this.highestLoyaltyRegime = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "highestLoyaltyRegime", 2))));
      this.publicCredits = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "publicCredits", 2))));
      this.publicBudget = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "publicBudget", 2))));
      this.privateLuxury = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "privateLuxury", 2))));
      this.garrisonFacility = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "garrisonFacility", 2))));
      this.culture = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "culture", 2))));
      this.unrest = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "unrest", 2))));
      this.fear = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "fear", 2))));
      this.danger = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "danger", 2))));
      this.qol = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "qol", 2))));
      this.city = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "city", 2))));
      this.health = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "health", 2))));
      this.entertainment = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "entertainment", 2))));
      this.education = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "education", 2))));
      this.security = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "security", 2))));
      this.freeFolk = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "freeFolk", 2))));
      this.luxury = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "luxury", 2))));
      this.qol_health = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "qol_health", 2))));
      this.qol_entertainment = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "qol_entertainment", 2))));
      this.qol_education = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "qol_education", 2))));
      this.qol_security = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "qol_security", 2))));
      this.qol_luxury = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "qol_luxury", 2))));
      this.workerSalaryNeed = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerSalaryNeed", 2))));
      this.workerSalaryGiven = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerSalaryGiven", 2))));
      this.workerSalaryBuyFood = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "workerSalaryBuyFood", 2))));
      this.colonistBonus = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "colonistBonus", 2))));
      this.recruitBonus = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "recruitBonus", 2))));
      this.publicBudgetGiven = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "publicBudgetGiven", 2))));
      this.emergencyFoodGiven = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "emergencyFoodGiven", 2))));
      this.governorCharId =  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotCharacter].GetData3(5, this.regid, 6, 10, 7, this.zoneId, 0)));
      if (DrawMod.TGame.Data.Turn > -1)
        this.regimeCredits = useRegimeId <= -1 ? Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotRegimeKeys].GetData2(0, this.currentRegimeId, 1, "credits", 2)))) : Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotRegimeKeys].GetData2(0, useRegimeId, 1, "credits", 2))));
      if (this.locNr > -1)
      {
        if (Information.IsNothing( DrawMod.TGame.Data.LocObj[this.locNr].items))
          DrawMod.TGame.Data.LocObj[this.locNr].items = ItemList::new();
        let mut nr1: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(7);
        this.food = nr1 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr1];
        this.orig_food = this.food;
        let mut nr2: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(5);
        this.water = nr2 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr2];
        let mut nr3: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(6);
        this.oxygen = nr3 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr3];
        let mut nr4: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(12);
        this.colonists = nr4 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr4];
        let mut nr5: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(9);
        this.recruits = nr5 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr5];
        let mut nr6: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(16);
        this.maxEnergy = nr6 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr6];
        let mut nr7: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(17);
        this.maxAmmo = nr7 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr7];
        let mut nr8: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(18);
        this.maxFuel = nr8 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr8];
        let mut nr9: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(19);
        this.maxFood = nr9 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr9];
        let mut nr10: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(22);
        this.maxIP = nr10 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr10];
        let mut nr11: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(20);
        this.maxWater = nr11 <= -1 ? 0 : DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr11];
      }
      else
        this.food = 0;
      this.tempExposureFreeFolkLoss = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "tempExposureFreeFolkLoss", 2))));
      this.tempExposurePopLoss = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "tempExposurePopLoss", 2))));
      this.tempExposureWorkerLoss = Math.Max(0,  Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].GetData2(0, this.zoneId, 1, "tempExposureWorkerLoss", 2))));
    }

    pub fn Output(let mut useRegimeId: i32 =  -1)
    {
      if (this.locNr > -1)
      {
        let mut nr1: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(5);
        if (nr1 < 0)
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(5, this.water);
        else
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr1] = this.water;
        let mut nr2: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(6);
        if (nr2 < 0)
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(6, this.oxygen);
        else
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr2] = this.oxygen;
        let mut index: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(7);
        if (this.zoneId == 41)
          index = index;
        if (index < 0)
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(7, this.food);
        else
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[index] = this.food;
        let mut nr3: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(12);
        if (nr3 < 0)
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(12, this.colonists);
        else
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr3] = this.colonists;
        let mut nr4: i32 =  DrawMod.TGame.Data.LocObj[this.locNr].items.list.FindNr(9);
        if (nr4 < 0)
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(9, this.recruits);
        else
          DrawMod.TGame.Data.LocObj[this.locNr].items.list.Weight[nr4] = this.recruits;
        DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(16, this.maxEnergy);
        DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(17, this.maxAmmo);
        DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(18, this.maxFuel);
        DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(19, this.maxFood);
        DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(22, this.maxIP);
        DrawMod.TGame.Data.LocObj[this.locNr].items.list.Add(20, this.maxWater);
        DrawMod.TGame.Data.LocObj[this.locNr].items.list.removeWeight0orLower();
      }
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerSalaryNeed", 2, this.workerSalaryNeed, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerSalaryGiven", 2, this.workerSalaryGiven, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerSalaryBuyFood", 2, this.workerSalaryBuyFood, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "recruitBonus", 2, this.recruitBonus, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "colonistBonus", 2, this.colonistBonus, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "occupationMode", 2, this.occupationMode, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "worker", 2, this.worker, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "pop", 2, this.pop, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerPoints", 2, this.workerPoints, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "popPoints", 2, this.popPoints, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "privateFood", 2, this.privateFood, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "privateMetal", 2, this.privateMetal, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "privateOil", 2, this.privateOil, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "privateWater", 2, this.privateWater, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "popHunger", 2, this.popHunger, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "popHapiness", 2, this.popHapiness, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "popLoyalty", 2, this.popLoyalty, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerHunger", 2, this.workerHunger, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerHapiness", 2, this.workerHapiness, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "militiaManpower", 2, this.militiaManpower, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "militiaEquipment", 2, this.militiaEquipment, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "militancy", 2, this.militancy, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "cas", 2, this.cas, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "recruitPenalty", 2, this.recruitPenalty, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "city", 2, this.city, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "privateCreditsGrowth", 2, this.privateCreditsGrowth, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "emergencyFoodGiven", 2, this.emergencyFoodGiven, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "rebelCredits", 2, this.rebelCredits, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "popCredits", 2, this.popCredits, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "hiddenEconomyIncome", 2, this.hiddenEconomyIncome, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "privateAssetIncome", 2, this.privateAssetIncome, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerSpendingIncome", 2, this.workerSpendingIncome, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "sellingFoodIncome", 2, this.sellingFoodIncome, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "buyingFoodExpenses", 2, this.buyingFoodExpenses, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "buyingLuxuriesExpenses", 2, this.buyingLuxuriesExpenses, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "BuyingAssetsExpenses", 2, this.BuyingAssetsExpenses, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "BuyingPopExpenses", 2, this.BuyingPopExpenses, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "netPrivateIncome", 2, this.netPrivateIncome, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "administrativeStrain", 2, this.administrativeStrain, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "workerCredits", 2, this.workerCredits, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "occupationMode", 2, this.occupationMode, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "rebelStrength", 2, this.rebelStrength, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "rebelManpower", 2, this.rebelManpower, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "rebelRegime", 2, this.rebelRegime, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "highestLoyalty", 2, this.highestLoyalty, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "highestLoyaltyRegime", 2, this.highestLoyaltyRegime, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "publicCredits", 2, this.publicCredits, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "publicBudget", 2, this.publicBudget, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "publicBudgetGiven", 2, this.publicBudgetGiven, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "privateLuxury", 2, this.privateLuxury, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "garrisonFacility", 2, this.garrisonFacility, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "culture", 2, this.culture, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "unrest", 2, this.unrest, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "danger", 2, this.danger, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "fear", 2, this.fear, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "qol", 2, this.qol, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "health", 2, this.health, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "entertainment", 2, this.entertainment, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "education", 2, this.education, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "security", 2, this.security, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "freeFolk", 2, this.freeFolk, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "luxury", 2, this.luxury, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "qol_health", 2, this.qol_health, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "qol_entertainment", 2, this.qol_entertainment, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "qol_education", 2, this.qol_education, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "qol_security", 2, this.qol_security, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "qol_luxury", 2, this.qol_luxury, true);
      if (DrawMod.TGame.Data.Turn > -1)
      {
        if (useRegimeId > -1)
          DrawMod.TGame.Data.StringListObj[this.slotRegimeKeys].SetData2(0, useRegimeId, 1, "credits", 2, this.regimeCredits, true);
        else
          DrawMod.TGame.Data.StringListObj[this.slotRegimeKeys].SetData2(0, this.currentRegimeId, 1, "credits", 2, this.regimeCredits, true);
      }
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "tempExposureFreeFolkLoss", 2, this.tempExposureFreeFolkLoss, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "tempExposurePopLoss", 2, this.tempExposurePopLoss, true);
      DrawMod.TGame.Data.StringListObj[this.slotZoneKeys].SetData2(0, this.zoneId, 1, "tempExposureWorkerLoss", 2, this.tempExposureWorkerLoss, true);
      DrawMod.TGame.Data.StringListObj[this.slotZones].SetData(0, this.zoneId, 9, this.zoneCultureId);
    }
  }
}
