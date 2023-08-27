// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.Shadow_MinorAntAI
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;

namespace WindowsApplication1
{
  pub class Shadow_MinorAntAI
  {
    pub ai: DC2AIClass;
    pub data: DataClass;
    pub MapClass map;
    pub RegimeId: i32;
    pub slotMilitiaUnits: i32;
    pub slotZoneOrders: i32;
    pub slotResult: i32;
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
    pub slotBaseValues: i32;
    pub slotSHQitems: i32;
    pub slotPoolItems: i32;
    pub slotOldRegimeKeys: i32;
    pub slotAssetPresentation: i32;
    pub slotRegimeRes: i32;
    pub slotResearchTypes: i32;
    pub slotFlags: i32;
    pub slotFlagInstructions: i32;
    pub slotGameKeys: i32;
    pub slotTraders: i32;
    pub slotTraderZones: i32;
    pub slotTraderItems: i32;
    pub slotRegimeKeys: i32;
    pub slotItemType: i32;
    pub slotToeTypes: i32;
    pub slotOldShqItems: i32;
    pub slotZoneSeasons: i32;
    pub slotRegimeOobs: i32;
    pub slotOobTypes: i32;
    pub slotModelTypes: i32;
    pub slotRegimeModels: i32;
    pub AIMatrix areas;
    pub SimpleList GroupList;
    pub SimpleList UnitList;
    pub VAR_AGRESSION: i32;
    pub VAR_ORGANISATION: i32;
    pub VAR_CALCULATION: i32;
    pub SimpleList faunaAreaList;
    pub SimpleList faunaTypeList;
    pub SimpleList faunaAreaNeighbourList;
    pub SimpleList faunaTypeListMigrateHistory;
    pub SimpleList faunaTypeListCrusaderHistory;

    pub Shadow_MinorAntAI( tai: DC2AIClass)
    {
      self.ai = tai;
      self.data = tai.game.Data;
      self.map = tai.game.Data.MapObj[0];
      self.RegimeId = tai.game.Data.RegimeObj[tai.game.Data.Turn].id;
    }

    pub fn Run()
    {
      HelperEconomyData hed = new HelperEconomyData( self.ai.game, "SE_Data");
      if (self.ai.game.Data.Round == 1)
        self.ai.game.EventRelatedObj.ExecMakeProduction("SE_Data", -1, 1, 0, "");
      hed.reg = self.ai.game.Data.Turn;
      hed.regid = self.ai.game.Data.RegimeObj[hed.reg].id;
      hed.currentRegimeNr = hed.reg;
      hed.currentRegimeId = hed.regid;
      let mut length1: i32 = self.ai.game.Data.StringListObj[hed.slotZones].Length;
      for (let mut index: i32 = 0; index <= length1; index += 1)
      {
        let mut num: i32 =  Math.Round(Conversion.Val(self.ai.game.Data.StringListObj[hed.slotZones].Data[index, 0]));
        if ( Math.Round(Conversion.Val(self.ai.game.Data.StringListObj[hed.slotZones].Data[index, 8])) == self.ai.game.Data.RegimeObj[self.ai.game.Data.Turn].id)
        {
          hed.zoneId = num;
          hed.Input();
          self.ai.game.EventRelatedObj.ZoneEconomy_EarlyPrivateEconomy( hed, "Se_Data");
          hed.Output();
        }
      }
      self.ai.game.EventRelatedObj.ExecMakeProduction("SE_Data", -1, 0, 0, "");
      hed.reg = self.ai.game.Data.Turn;
      hed.regid = self.ai.game.Data.RegimeObj[hed.reg].id;
      hed.currentRegimeNr = hed.reg;
      hed.currentRegimeId = hed.regid;
      let mut length2: i32 = self.ai.game.Data.StringListObj[hed.slotZones].Length;
      for (let mut index: i32 = 0; index <= length2; index += 1)
      {
        let mut num: i32 =  Math.Round(Conversion.Val(self.ai.game.Data.StringListObj[hed.slotZones].Data[index, 0]));
        if ( Math.Round(Conversion.Val(self.ai.game.Data.StringListObj[hed.slotZones].Data[index, 8])) == self.ai.game.Data.RegimeObj[self.ai.game.Data.Turn].id)
        {
          hed.zoneId = num;
          hed.Input();
          self.ai.game.EventRelatedObj.ZoneEconomy_PrivateEconomy( hed, "SE_Data");
          self.ai.game.EventRelatedObj.ZoneEconomy_Militia_and_Minors( hed, "SE_Data");
          hed.Output();
        }
      }
      self.ai.game.EventRelatedObj.ExecMakeMaximization(false);
      if (self.ai.game.Data.Round <= 1)
        return;
      float num1 = self.data.RuleVar[941];
      self.data.RuleVar[941] = 1f;
      bool varDebugOn = self.ai.VAR_DEBUG_ON;
      if (self.ai.game.EventRelatedObj.Helper_IsDebug())
        self.ai.VAR_DEBUG_ON = true;
      self.VAR_AGRESSION = 50;
      self.VAR_ORGANISATION = 50;
      self.VAR_CALCULATION = 50;
      self.ai.SetTempHexNeighbours();
      self.ai.MakeCombatMatrix(false);
      self.ai.SetTempUnitPowerAndVarDefensiveMod();
      if ( Math.Round(Conversion.Val(self.ai.game.Data.Designer)) >= 66)
        self.Global_ReproductionAndStuff();
      for (let mut unitCounter: i32 = self.data.UnitCounter; unitCounter >= 0; unitCounter += -1)
      {
        let mut x: i32 = self.data.UnitObj[unitCounter].X;
        let mut y: i32 = self.data.UnitObj[unitCounter].Y;
        if (self.data.UnitObj[unitCounter].PreDef == -1 & x > -1 & self.data.UnitObj[unitCounter].Regime == self.data.Turn)
        {
          self.data.UnitObj[unitCounter].HQ = -1;
          switch (self.data.HistoricalUnitObj[self.data.UnitObj[unitCounter].Historical].GiveHisVarValue(91))
          {
            case 100:
              self.ExecuteAnts_AttackRandomly(unitCounter);
              self.ExecuteAnts_MoveRandomly_FreeFolk(unitCounter);
              continue;
            case 200:
              self.ExecuteAnts_Animal_SpecialRules(unitCounter);
              self.ExecuteAnts_AttackRandomly(unitCounter);
              self.ExecuteAnts_MoveRandomly_Animal(unitCounter);
              continue;
            case 300:
              self.ExecuteAnts_Animal_SpecialRules(unitCounter);
              self.ExecuteAnts_AdvancedAnimal_Behaviour(unitCounter);
              self.ExecuteAnts_Advanced_AttackRandomly(unitCounter);
              continue;
            default:
              continue;
          }
        }
      }
      self.data.RuleVar[941] = num1;
      self.ai.VAR_DEBUG_ON = varDebugOn;
      DrawMod.TGame.EditObj.TargetX = -1;
      DrawMod.TGame.EditObj.TargetY = -1;
    }

    pub fn Global_ReproductionAndStuff()
    {
      data: DataClass = self.data;
      str: String = "hexName";
       local: String =  str;
      let mut libVar: i32 = data.FindLibVar( local, "SE_Data");
      let mut stringListById1: i32 = self.ai.game.HandyFunctionsObj.GetStringListByID(self.ai.game.EventRelatedObj.CheckStringlistID("SE_Data", 242, 0, 0));
      let mut stringListById2: i32 = self.ai.game.HandyFunctionsObj.GetStringListByID(self.ai.game.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0));
      let mut stringListById3: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
      let mut stringListById4: i32 = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 382, 0, 0));
      let mut num1: i32 =  Math.Round(Conversion.Val(self.ai.game.Data.StringListObj[stringListById2].GetData(0, 42, 2)));
      int[] numArray1 = new int[200];
      let mut length: i32 = self.data.StringListObj[stringListById1].Length;
      for (let mut index1: i32 = 0; index1 <= length; index1 += 1)
      {
        let mut index2: i32 =  Math.Round(Conversion.Val(self.data.StringListObj[stringListById1].Data[index1, 0]));
        let mut num2: i32 = self.data.StringListObj[stringListById1].Width < 4 ? 20 :  Math.Round(Conversion.Val(self.data.StringListObj[stringListById1].Data[index1, 4]));
        numArray1[index2] = num2;
      }
      int[] numArray2 = new int[99 + self.ai.game.Data.RegimeCounter + 1];
      let mut regimeCounter: i32 = self.ai.game.Data.RegimeCounter;
      for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
        numArray2[index] =  Math.Round(Conversion.Val(self.data.StringListObj[stringListById3].GetData(0, self.data.RegimeObj[index].id, 1)));
      self.faunaAreaNeighbourList = SimpleList::new();
      self.faunaAreaList = SimpleList::new();
      SimpleList simpleList1 = SimpleList::new();
      self.faunaTypeList = SimpleList::new();
      self.faunaTypeListMigrateHistory = SimpleList::new();
      self.faunaTypeListCrusaderHistory = SimpleList::new();
      let mut mapWidth1: i32 = self.data.MapObj[0].MapWidth;
      for (let mut cx: i32 = 0; cx <= mapWidth1; cx += 1)
      {
        let mut mapHeight: i32 = self.data.MapObj[0].MapHeight;
        for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
        {
          let mut hexLibVarValue1: i32 = self.data.MapObj[0].HexObj[cx, cy].GetHexLibVarValue(libVar);
          if (hexLibVarValue1 > 0 &&  Math.Round(Conversion.Val(self.data.StringListObj[stringListById4].GetData(0, hexLibVarValue1, 2))) == 5)
          {
            let mut landscapeType: i32 = self.data.MapObj[0].HexObj[cx, cy].LandscapeType;
            if (!self.data.LandscapeTypeObj[landscapeType].IsSea)
            {
              let mut tweight1: i32 = numArray1[landscapeType];
              if (num1 < 10)
                tweight1 =  Math.Round(Math.Ceiling( tweight1 / 15.0));
              else if (num1 == 10)
                tweight1 =  Math.Round(Math.Ceiling( tweight1 / 10.0));
              else if (num1 > 10)
                tweight1 =  Math.Round(Math.Ceiling( tweight1 / 6.0));
              self.faunaAreaList.AddWeight(hexLibVarValue1, tweight1);
              if (self.data.MapObj[0].HexObj[cx, cy].Regime > -1 && numArray2[self.data.MapObj[0].HexObj[cx, cy].Regime] == 1)
                simpleList1.AddWeight(hexLibVarValue1, 1);
              let mut tfacing: i32 = 1;
              do
              {
                Coordinate coordinate = self.ai.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                if (coordinate.onmap)
                {
                  let mut hexLibVarValue2: i32 = self.data.MapObj[0].HexObj[coordinate.x, coordinate.y].GetHexLibVarValue(libVar);
                  if (hexLibVarValue2 > 0 & hexLibVarValue1 != hexLibVarValue2 &&  Math.Round(Conversion.Val(self.data.StringListObj[stringListById4].GetData(0, hexLibVarValue2, 2))) == 5 && !self.data.LandscapeTypeObj[self.data.MapObj[0].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                    self.faunaAreaNeighbourList.AddWeight(hexLibVarValue1, 1, hexLibVarValue2, CheckData1Existence: true);
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
              if (self.data.MapObj[0].HexObj[cx, cy].Regime == self.data.Turn)
              {
                let mut unitCounter: i32 = self.data.MapObj[0].HexObj[cx, cy].UnitCounter;
                for (let mut index: i32 = 0; index <= unitCounter; index += 1)
                {
                  let mut unit: i32 = self.data.MapObj[0].HexObj[cx, cy].UnitList[index];
                  let mut historical: i32 = self.data.UnitObj[unit].Historical;
                  if (historical > -1)
                  {
                    if (self.data.HistoricalUnitObj[historical].GiveHisVarValue(91) == 300 && self.data.HistoricalUnitObj[historical].GiveHisVarValue(92) < 1 && self.data.UnitObj[unit].SFCount > -1)
                    {
                      let mut sf: i32 = self.data.UnitObj[unit].SFList[0];
                      let mut type: i32 = self.data.SFObj[sf].Type;
                      let mut qty: i32 = self.data.SFObj[sf].Qty;
                      self.faunaTypeList.AddWeight(hexLibVarValue1, qty, type, CheckData1Existence: true);
                      let mut tweight2: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(95);
                      self.faunaTypeListMigrateHistory.AddWeight(hexLibVarValue1, tweight2, type, CheckData1Existence: true);
                      let mut tweight3: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(96);
                      self.faunaTypeListCrusaderHistory.AddWeight(hexLibVarValue1, tweight3, type, CheckData1Existence: true);
                    }
                    let mut num3: i32 =  Math.Round(Math.Ceiling( self.data.HistoricalUnitObj[historical].GiveHisVarValue(95) * 0.9)) - 1;
                    self.data.HistoricalUnitObj[historical].SetHisVarValue(95, num3);
                    let mut num4: i32 =  Math.Round(Math.Ceiling( self.data.HistoricalUnitObj[historical].GiveHisVarValue(96) * 0.9)) - 1;
                    self.data.HistoricalUnitObj[historical].SetHisVarValue(96, num4);
                  }
                }
              }
            }
          }
        }
      }
      let mut counter1: i32 = self.faunaAreaList.Counter;
      for (let mut index3: i32 = 0; index3 <= counter1; index3 += 1)
      {
        let mut tid: i32 = self.faunaAreaList.Id[index3];
        let mut num5: i32 = Convert.ToInt32(Math.Ceiling(new Decimal(self.faunaAreaList.Weight[index3])));
        if (num1 < 10 && DrawMod.RandyNumber.Next(0, 100) < 50)
          num5 = 0;
        if (num5 > 0)
        {
          let mut num6: i32 = 0;
          SimpleList simpleList2 = SimpleList::new();
          let mut sfTypeCounter1: i32 = self.data.SFTypeCounter;
          for (let mut index4: i32 = 0; index4 <= sfTypeCounter1; index4 += 1)
          {
            if (self.data.SFTypeObj[index4].SFTypeVar[82] > 0)
            {
              let mut weight: i32 = self.faunaTypeList.FindWeight(tid, index4);
              if (weight > 0)
              {
                num6 += weight * self.data.SFTypeObj[index4].Weight;
                simpleList2.AddWeight(index4, weight * self.data.SFTypeObj[index4].Weight);
              }
            }
          }
          if (num6 > 0)
          {
            simpleList2.Percentify();
            let mut num7: i32 = num5 - num6;
            if (num7 > 0)
            {
              let mut sfTypeCounter2: i32 = self.data.SFTypeCounter;
              for (let mut index5: i32 = 0; index5 <= sfTypeCounter2; index5 += 1)
              {
                if (self.data.SFTypeObj[index5].SFTypeVar[82] > 0)
                {
                  let mut weight: i32 = self.faunaTypeList.FindWeight(tid, index5);
                  if (weight > 0)
                  {
                    SimpleList behaviourStats = self.ai.game.EventRelatedObj.AlienFauna_GetBehaviourStats(index5);
                    if ( Math.Round( (num7 * simpleList2.FindWeight(index5)) / 100.0) > self.data.SFTypeObj[index5].Weight)
                    {
                      float d =  (weight * behaviourStats.Weight[1]) / 100f;
                      let mut num8: i32 =  Math.Round(Math.Floor( d));
                      float num9 = d - Convert.ToSingle(Math.Floor(new Decimal(num8)));
                      if ( num9 > 0.0 &&  Math.Round( (num9 * 100f)) >= DrawMod.RandyNumber.Next(0, 100))
                        num8 = 1;
                      if (num8 > 0)
                      {
                        SimpleList simpleList3 = SimpleList::new();
                        let mut mapWidth2: i32 = self.data.MapObj[0].MapWidth;
                        for (let mut index6: i32 = 0; index6 <= mapWidth2; index6 += 1)
                        {
                          let mut mapHeight: i32 = self.data.MapObj[0].MapHeight;
                          for (let mut index7: i32 = 0; index7 <= mapHeight; index7 += 1)
                          {
                            if (self.data.MapObj[0].HexObj[index6, index7].GetHexLibVarValue(libVar) == tid)
                            {
                              let mut unitCounter: i32 = self.data.MapObj[0].HexObj[index6, index7].UnitCounter;
                              for (let mut index8: i32 = 0; index8 <= unitCounter; index8 += 1)
                              {
                                let mut unit: i32 = self.data.MapObj[0].HexObj[index6, index7].UnitList[index8];
                                if (self.ai.game.HandyFunctionsObj.HasUnitSFType(unit, index5))
                                {
                                  let mut qty: i32 = self.data.SFObj[self.data.UnitObj[unit].SFList[0]].Qty;
                                  simpleList3.AddWeight(unit, qty);
                                }
                              }
                            }
                          }
                        }
                        if (simpleList3.Counter > -1)
                        {
                          let mut num10: i32 = 0;
                          while (num8 > 0 & num10 < 99)
                          {
                            num10 += 1;
                            let mut sf: i32 = self.data.UnitObj[simpleList3.GetRandomIdbasedOnWeight()].SFList[0];
                            let mut qty: i32 = self.data.SFObj[sf].Qty;
                            SFClass[] sfObj = self.data.SFObj;
                            SFClass[] sfClassArray = sfObj;
                            let mut index9: i32 = sf;
                            let mut index10: i32 = index9;
                            sfClassArray[index10].Xp = sfObj[index9].Xp -  Math.Round(Math.Ceiling( (self.data.SFObj[sf].Xp * 1) /  qty));
                            let mut num11: i32 = qty + 1;
                            --num8;
                            self.data.SFObj[sf].Qty = num11;
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
      let mut sfTypeCounter3: i32 = self.data.SFTypeCounter;
      for (let mut index11: i32 = 0; index11 <= sfTypeCounter3; index11 += 1)
      {
        if (self.data.SFTypeObj[index11].SFTypeVar[82] > 0)
        {
          let mut counter2: i32 = self.faunaAreaList.Counter;
          for (let mut index12: i32 = 0; index12 <= counter2; index12 += 1)
          {
            let mut tid1: i32 = self.faunaAreaList.Id[index12];
            let mut num12: i32 = self.faunaTypeList.FindWeight(tid1, index11) - self.faunaTypeListMigrateHistory.FindWeight(tid1, index11);
            SimpleList simpleList4 = SimpleList::new();
            if (num12 > 0)
            {
              SimpleList behaviourStats = self.ai.game.EventRelatedObj.AlienFauna_GetBehaviourStats(index11);
              if (behaviourStats.Weight[2] > DrawMod.RandyNumber.Next(0, 100))
              {
                if (num12 > behaviourStats.Weight[0])
                {
                  let mut counter3: i32 = self.faunaAreaNeighbourList.Counter;
                  for (let mut index13: i32 = 0; index13 <= counter3; index13 += 1)
                  {
                    if (self.faunaAreaNeighbourList.Id[index13] == tid1)
                    {
                      let mut tid2: i32 = self.faunaAreaNeighbourList.Data1[index13];
                      let mut weight1: i32 = self.faunaTypeList.FindWeight(tid2, index11);
                      let mut weight2: i32 = self.faunaAreaList.FindWeight(tid2);
                      if (num12 > weight1 &&  (num12 + weight1) <  weight2 /  (1 + self.data.SFTypeObj[index11].Weight) &&  Math.Round( (num12 * behaviourStats.Weight[2]) / 100.0) > weight1)
                        simpleList4.AddWeight(tid2, weight1);
                    }
                  }
                }
                if (simpleList4.Counter > -1)
                {
                  simpleList4.ReverseSortHighSpeed();
                  let mut num13: i32 = simpleList4.Weight[0];
                  let mut counter4: i32 = simpleList4.Counter;
                  for (let mut index14: i32 = 0; index14 <= counter4; index14 += 1)
                  {
                    simpleList4.Weight[index14] = num13 - simpleList4.Weight[index14];
                    if (simpleList4.Weight[index14] < 1)
                      simpleList4.Weight[index14] = 1;
                    int[] weight = simpleList4.Weight;
                    int[] numArray3 = weight;
                    let mut index15: i32 = index14;
                    let mut index16: i32 = index15;
                    let mut num14: i32 = weight[index15] + 1;
                    numArray3[index16] = num14;
                  }
                  let mut randomIdbasedOnWeight1: i32 = simpleList4.GetRandomIdbasedOnWeight();
                  SimpleList simpleList5 = SimpleList::new();
                  let mut mapWidth3: i32 = self.data.MapObj[0].MapWidth;
                  for (let mut index17: i32 = 0; index17 <= mapWidth3; index17 += 1)
                  {
                    let mut mapHeight: i32 = self.data.MapObj[0].MapHeight;
                    for (let mut index18: i32 = 0; index18 <= mapHeight; index18 += 1)
                    {
                      if (self.data.MapObj[0].HexObj[index17, index18].GetHexLibVarValue(libVar) == tid1)
                      {
                        let mut unitCounter: i32 = self.data.MapObj[0].HexObj[index17, index18].UnitCounter;
                        for (let mut index19: i32 = 0; index19 <= unitCounter; index19 += 1)
                        {
                          let mut unit: i32 = self.data.MapObj[0].HexObj[index17, index18].UnitList[index19];
                          if (self.ai.game.HandyFunctionsObj.HasUnitSFType(unit, index11))
                          {
                            let mut qty: i32 = self.data.SFObj[self.data.UnitObj[unit].SFList[0]].Qty;
                            if (qty >  Math.Round( behaviourStats.Weight[0] * 0.8) & qty >= 2)
                              simpleList5.AddWeight(unit, qty);
                          }
                        }
                      }
                    }
                  }
                  if (simpleList5.Counter > -1)
                  {
                    let mut randomIdbasedOnWeight2: i32 = simpleList5.GetRandomIdbasedOnWeight();
                    let mut x: i32 = self.ai.game.Data.UnitObj[randomIdbasedOnWeight2].X;
                    let mut y: i32 = self.ai.game.Data.UnitObj[randomIdbasedOnWeight2].Y;
                    let mut sf: i32 = self.data.UnitObj[randomIdbasedOnWeight2].SFList[0];
                    let mut num15: i32 =  Math.Round( self.data.SFObj[sf].Qty / 2.0);
                    self.ai.game.EventRelatedObj.Helper_AddSpecificUnit(self.data.UnitObj[randomIdbasedOnWeight2].Name, x, y, 0, 1, index11, num15, self.data.SFObj[sf].Xp, self.data.SFObj[sf].Mor);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(91, 300);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].Counter = -1;
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].CounterString = Strings.Left(self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].Name, 3);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(92, tid1);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(93, randomIdbasedOnWeight1);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(94, 2);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(95, 0);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(96, 0);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(95, num15);
                    SFClass[] sfObj = self.data.SFObj;
                    SFClass[] sfClassArray = sfObj;
                    let mut index20: i32 = sf;
                    let mut index21: i32 = index20;
                    sfClassArray[index21].Qty = sfObj[index20].Qty - num15;
                    self.data.SFObj[self.ai.game.Data.UnitObj[self.ai.game.Data.UnitCounter].SFList[0]].Rdn = self.data.SFObj[sf].Rdn;
                    self.faunaTypeListMigrateHistory.AddWeight(tid1, num15);
                  }
                }
              }
            }
          }
        }
      }
      let mut sfTypeCounter4: i32 = self.data.SFTypeCounter;
      for (let mut index22: i32 = 0; index22 <= sfTypeCounter4; index22 += 1)
      {
        if (self.data.SFTypeObj[index22].SFTypeVar[82] > 0)
        {
          let mut counter5: i32 = self.faunaAreaList.Counter;
          for (let mut index23: i32 = 0; index23 <= counter5; index23 += 1)
          {
            let mut tid3: i32 = self.faunaAreaList.Id[index23];
            let mut num16: i32 = self.faunaTypeList.FindWeight(tid3, index22) - self.faunaTypeListCrusaderHistory.FindWeight(tid3, index22);
            let mut weight3: i32 = simpleList1.FindWeight(tid3, index22);
            SimpleList simpleList6 = SimpleList::new();
            if (num16 > 0)
            {
              SimpleList behaviourStats = self.ai.game.EventRelatedObj.AlienFauna_GetBehaviourStats(index22);
              if (behaviourStats.Weight[4] > DrawMod.RandyNumber.Next(0, 100))
              {
                if (num16 >  Math.Round( behaviourStats.Weight[0] * 0.5))
                {
                  let mut counter6: i32 = self.faunaAreaNeighbourList.Counter;
                  for (let mut index24: i32 = 0; index24 <= counter6; index24 += 1)
                  {
                    if (self.faunaAreaNeighbourList.Id[index24] == tid3)
                    {
                      let mut tid4: i32 = self.faunaAreaNeighbourList.Data1[index24];
                      let mut weight4: i32 = self.faunaTypeList.FindWeight(tid4, index22);
                      let mut weight5: i32 = simpleList1.FindWeight(tid4, index22);
                      if (num16 > weight4 & weight5 > 0 &&  Math.Round( (num16 * behaviourStats.Weight[4]) / 100.0) > weight4 &  (weight5 * behaviourStats.Weight[4]) / 100.0 >  weight3)
                        simpleList6.AddWeight(tid4, weight5);
                    }
                  }
                }
                if (simpleList6.Counter > -1)
                {
                  let mut randomIdbasedOnWeight3: i32 = simpleList6.GetRandomIdbasedOnWeight();
                  SimpleList simpleList7 = SimpleList::new();
                  let mut mapWidth4: i32 = self.data.MapObj[0].MapWidth;
                  for (let mut index25: i32 = 0; index25 <= mapWidth4; index25 += 1)
                  {
                    let mut mapHeight: i32 = self.data.MapObj[0].MapHeight;
                    for (let mut index26: i32 = 0; index26 <= mapHeight; index26 += 1)
                    {
                      if (self.data.MapObj[0].HexObj[index25, index26].GetHexLibVarValue(libVar) == tid3)
                      {
                        let mut unitCounter: i32 = self.data.MapObj[0].HexObj[index25, index26].UnitCounter;
                        for (let mut index27: i32 = 0; index27 <= unitCounter; index27 += 1)
                        {
                          let mut unit: i32 = self.data.MapObj[0].HexObj[index25, index26].UnitList[index27];
                          if (self.ai.game.HandyFunctionsObj.HasUnitSFType(unit, index22))
                          {
                            let mut qty: i32 = self.data.SFObj[self.data.UnitObj[unit].SFList[0]].Qty;
                            if (qty >  Math.Round( behaviourStats.Weight[0] * 0.8) & qty >= 2)
                              simpleList7.AddWeight(unit, qty);
                          }
                        }
                      }
                    }
                  }
                  if (simpleList7.Counter > -1)
                  {
                    let mut randomIdbasedOnWeight4: i32 = simpleList7.GetRandomIdbasedOnWeight();
                    let mut x: i32 = self.ai.game.Data.UnitObj[randomIdbasedOnWeight4].X;
                    let mut y: i32 = self.ai.game.Data.UnitObj[randomIdbasedOnWeight4].Y;
                    let mut sf: i32 = self.data.UnitObj[randomIdbasedOnWeight4].SFList[0];
                    let mut num17: i32 =  Math.Round( self.data.SFObj[sf].Qty / 2.0);
                    self.ai.game.EventRelatedObj.Helper_AddSpecificUnit(self.data.UnitObj[randomIdbasedOnWeight4].Name, x, y, 0, 1, index22, num17, self.data.SFObj[sf].Xp, self.data.SFObj[sf].Mor);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(91, 300);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].Counter = -1;
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].CounterString = Strings.Left(self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].Name, 3);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(92, tid3);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(93, randomIdbasedOnWeight3);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(94, 1);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(95, 0);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(96, 0);
                    self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(95, num17);
                    SFClass[] sfObj = self.data.SFObj;
                    SFClass[] sfClassArray = sfObj;
                    let mut index28: i32 = sf;
                    let mut index29: i32 = index28;
                    sfClassArray[index29].Qty = sfObj[index28].Qty - num17;
                    self.data.SFObj[self.ai.game.Data.UnitObj[self.ai.game.Data.UnitCounter].SFList[0]].Rdn = self.data.SFObj[sf].Rdn;
                    self.faunaTypeListMigrateHistory.AddWeight(tid3, num17);
                  }
                }
              }
            }
          }
        }
      }
      let mut unitCounter1: i32 = self.ai.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
      {
        let mut historical: i32 = self.data.UnitObj[index].Historical;
        if (historical > -1 && self.data.UnitObj[index].Regime == 1 && self.data.UnitObj[index].SFCount > -1 && self.data.SFTypeObj[self.data.SFObj[self.data.UnitObj[index].SFList[0]].Type].SFTypeVar[82] > 0)
        {
          let mut num18: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(93);
          if (self.data.HistoricalUnitObj[historical].GiveHisVarValue(94) == 2 && self.data.MapObj[0].HexObj[self.ai.game.Data.UnitObj[index].X, self.ai.game.Data.UnitObj[index].Y].GetHexLibVarValue(libVar) == num18)
          {
            self.data.HistoricalUnitObj[historical].SetHisVarValue(92, num18);
            self.data.HistoricalUnitObj[historical].SetHisVarValue(93, 0);
            self.data.HistoricalUnitObj[historical].SetHisVarValue(94, 0);
          }
        }
      }
      let mut unitCounter2: i32 = self.ai.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter2; index += 1)
      {
        let mut historical: i32 = self.data.UnitObj[index].Historical;
        if (historical > -1 && self.data.UnitObj[index].Regime == 1 && self.data.UnitObj[index].SFCount > -1 && self.data.SFTypeObj[self.data.SFObj[self.data.UnitObj[index].SFList[0]].Type].SFTypeVar[82] > 0)
        {
          let mut tid: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(93);
          if (self.data.HistoricalUnitObj[historical].GiveHisVarValue(94) == 1 && simpleList1.FindWeight(tid) < 1)
          {
            let mut hexLibVarValue: i32 = self.data.MapObj[0].HexObj[self.ai.game.Data.UnitObj[index].X, self.ai.game.Data.UnitObj[index].Y].GetHexLibVarValue(libVar);
            if (hexLibVarValue > 0)
            {
              self.data.HistoricalUnitObj[historical].SetHisVarValue(92, hexLibVarValue);
              self.data.HistoricalUnitObj[historical].SetHisVarValue(93, 0);
              self.data.HistoricalUnitObj[historical].SetHisVarValue(94, 0);
            }
          }
        }
      }
      let mut unitCounter3: i32 = self.ai.game.Data.UnitCounter;
      for (let mut index30: i32 = 0; index30 <= unitCounter3; index30 += 1)
      {
        let mut historical: i32 = self.data.UnitObj[index30].Historical;
        if (historical > -1 && self.data.UnitObj[index30].Regime == 1 && self.data.UnitObj[index30].SFCount > -1)
        {
          let mut sf1: i32 = self.data.UnitObj[index30].SFList[0];
          let mut type: i32 = self.data.SFObj[sf1].Type;
          let mut qty: i32 = self.data.SFObj[sf1].Qty;
          let mut tid: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(92);
          let mut num19: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(93);
          let mut num20: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(94);
          if (self.data.SFTypeObj[type].SFTypeVar[82] > 0 && self.ai.game.EventRelatedObj.AlienFauna_GetBehaviourStats(type).Weight[0] <  Math.Round( qty * 0.66))
          {
            let mut x: i32 = self.ai.game.Data.UnitObj[index30].X;
            let mut y: i32 = self.ai.game.Data.UnitObj[index30].Y;
            let mut sf2: i32 = self.data.UnitObj[index30].SFList[0];
            let mut num21: i32 =  Math.Round( self.data.SFObj[sf2].Qty / 2.0);
            self.ai.game.EventRelatedObj.Helper_AddSpecificUnit(self.data.UnitObj[index30].Name, x, y, 0, 1, type, num21, self.data.SFObj[sf2].Xp, self.data.SFObj[sf2].Mor);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(91, 300);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].Counter = -1;
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].CounterString = Strings.Left(self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].Name, 3);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(92, tid);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(93, num19);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(94, num20);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(95, 0);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(96, 0);
            self.ai.game.Data.HistoricalUnitObj[self.ai.game.Data.HistoricalUnitCounter].SetHisVarValue(95, num21);
            SFClass[] sfObj = self.data.SFObj;
            SFClass[] sfClassArray = sfObj;
            let mut index31: i32 = sf2;
            let mut index32: i32 = index31;
            sfClassArray[index32].Qty = sfObj[index31].Qty - num21;
            self.data.SFObj[self.ai.game.Data.UnitObj[self.ai.game.Data.UnitCounter].SFList[0]].Rdn = self.data.SFObj[sf2].Rdn;
            self.faunaTypeListMigrateHistory.AddWeight(tid, num21);
          }
        }
      }
    }

    pub fn ExecuteAnts_AdvancedAnimal_Behaviour(unr: i32)
    {
      let mut historical: i32 = self.data.UnitObj[unr].Historical;
      if (historical == -1)
        return;
      let mut sfCount: i32 = self.data.UnitObj[unr].SFCount;
      type: i32;
      for (let mut index: i32 = 0; index <= sfCount; index += 1)
      {
        let mut sf: i32 = self.data.UnitObj[unr].SFList[index];
        type = self.data.SFObj[sf].Type;
        let mut qty: i32 = self.data.SFObj[sf].Qty;
      }
      if (type < 0)
        return;
      data: DataClass = self.data;
      str: String = "hexName";
       local1: String =  str;
      let mut libVar: i32 = data.FindLibVar( local1, "SE_Data");
      SimpleList behaviourStats = self.ai.game.EventRelatedObj.AlienFauna_GetBehaviourStats(type);
      let mut num1: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(92);
      let mut num2: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(93);
      let mut num3: i32 = self.data.HistoricalUnitObj[historical].GiveHisVarValue(94);
      let mut num4: i32 = num1;
      if (num3 == 1 | num3 == 2 && num2 > 0)
        num4 = num2;
      AIMatrix aiMatrix1 = new AIMatrix( self.ai);
      AIMatrix aiMatrix2 = new AIMatrix( self.ai);
      AIMatrix aiMatrix3 = new AIMatrix( self.ai);
      let mut mapWidth1: i32 = self.data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 = self.data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          if (self.data.MapObj[0].HexObj[index1, index2].GetHexLibVarValue(libVar) == num4)
            aiMatrix1.Value[index1, index2] = 1;
          if (self.data.MapObj[0].HexObj[index1, index2].Regime > -1 && self.ai.game.Data.RegimeObj[self.data.Turn].RegimeRel[self.data.MapObj[0].HexObj[index1, index2].Regime] == 0)
          {
            aiMatrix3.Value[index1, index2] = 1;
            if (self.data.MapObj[0].HexObj[index1, index2].UnitCounter > -1 | self.data.MapObj[0].HexObj[index1, index2].Location > -1)
              aiMatrix2.Value[index1, index2] = 1;
          }
        }
      }
      aiMatrix1.ExpandAndAddValueForAnyRegime(20);
      aiMatrix1.SetValueXToValueY(0, 21);
      aiMatrix2.ExpandAndAddValueForAnyRegime(behaviourStats.Weight[3]);
      aiMatrix2.SetValueXToValueY(0, behaviourStats.Weight[3] + 1);
      let mut tempUnitPower: i32 = self.data.UnitObj[unr].TempUnitPower;
      let mut x1: i32 = self.data.UnitObj[unr].X;
      let mut y1: i32 = self.data.UnitObj[unr].Y;
      if (!(self.data.UnitObj[unr].PreDef == -1 & x1 > -1 & self.data.UnitObj[unr].Regime == self.data.Turn))
        return;
      handyFunctionsObj: HandyFunctionsclass = DrawMod.TGame.HandyFunctionsObj;
      let mut unr1: i32 = unr;
      let mut x2: i32 = x1;
      let mut y2: i32 = y1;
      CustomDC2AICalls customDc2AiCalls = (CustomDC2AICalls) null;
       CustomDC2AICalls local2 =  customDc2AiCalls;
      handyFunctionsObj.MakeMovePrediction3(unr1, x2, y2, 0, ismove: true, tcustomAi: ( local2));
      let mut lowestAp: i32 = self.ai.game.HandyFunctionsObj.GetLowestAp(unr);
      let mut x2_1: i32 = -1;
      let mut num5: i32 = 0;
      let mut mapWidth2: i32 = self.map.MapWidth;
      y2_1: i32;
      for (let mut cx: i32 = 0; cx <= mapWidth2; cx += 1)
      {
        let mut mapHeight: i32 = self.map.MapHeight;
        for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
        {
          if (DrawMod.TGame.EditObj.TempValue[0].Value[cx, cy] <= lowestAp)
          {
            let mut num6: i32 = DrawMod.RandyNumber.Next(0, 30);
            if (!(DrawMod.RandyNumber.Next(0, 100) < behaviourStats.Weight[12] | DrawMod.RandyNumber.Next(0, 100) < 25))
            {
              if (behaviourStats.Weight[12] < 30)
                num6 += DrawMod.RandyNumber.Next(160, 320);
              else if (behaviourStats.Weight[12] < 50)
                num6 += DrawMod.RandyNumber.Next(80, 160);
              else if (behaviourStats.Weight[12] < 70)
                num6 += DrawMod.RandyNumber.Next(40, 80);
              else if (behaviourStats.Weight[12] < 90)
                num6 += DrawMod.RandyNumber.Next(20, 40);
              else
                num6 += DrawMod.RandyNumber.Next(0, 30);
            }
            let mut hexLibVarValue: i32 = self.data.MapObj[0].HexObj[cx, cy].GetHexLibVarValue(libVar);
            let mut num7: i32 = DrawMod.TGame.EditObj.TempValue[0].Value[cx, cy] != 0 ? num6 +  Math.Round( Math.Max(0, 100 - DrawMod.TGame.EditObj.TempValue[0].Value[cx, cy]) / 5.0) : num6 + 20;
            if (num3 > 0)
            {
              let mut num8: i32 = aiMatrix1.Value[cx, cy];
              let mut num9: i32 = aiMatrix1.Value[x1, y1] - num8;
              num7 += num9 * 50;
            }
            let mut num10: i32 = 0;
            let mut num11: i32 = 0;
            let mut num12: i32 = 0;
            let mut num13: i32 = 0;
            let mut tfacing1: i32 = 1;
            do
            {
              Coordinate coordinate1 = self.ai.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing1);
              if (coordinate1.onmap && self.data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].Regime >= 2)
              {
                if (self.data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].Location > -1)
                  num13 += 1;
                let mut unitCounter: i32 = self.data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].UnitCounter;
                for (let mut index: i32 = 0; index <= unitCounter; index += 1)
                {
                  let mut unit: i32 = self.data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].UnitList[index];
                  num10 += self.data.UnitObj[unit].TempUnitPower;
                  num11 += 1;
                  let mut num14: i32 = 0;
                  let mut tfacing2: i32 = 1;
                  do
                  {
                    Coordinate coordinate2 = self.ai.game.HandyFunctionsObj.HexNeighbour(coordinate1.x, coordinate1.y, 0, tfacing2);
                    if (coordinate2.onmap && self.data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].Regime == 1 & !(coordinate1.x == cx & coordinate1.x == cy) && self.data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].UnitCounter > -1)
                      num14 += 1;
                    tfacing2 += 1;
                  }
                  while (tfacing2 <= 6);
                  if (num14 > num12)
                    num12 = num14;
                }
              }
              tfacing1 += 1;
            }
            while (tfacing1 <= 6);
            let mut num15: i32 = num12;
            let mut num16: i32 = num10;
            let mut num17: i32 = num11;
            let mut num18: i32 = num13;
            if (num17 > 0)
              num7 +=  Math.Round(100.0 * ( behaviourStats.Weight[9] / 100.0));
            else if (aiMatrix2.Value[x1, y1] > aiMatrix2.Value[cx, cy])
            {
              let mut num19: i32 = aiMatrix2.Value[cx, cy];
              let mut num20: i32 = aiMatrix2.Value[x1, y1] - num19;
              num7 += Math.Min(70,  Math.Round( (num20 * 30) * ( behaviourStats.Weight[9] / 100.0)));
            }
            else if (aiMatrix3.Value[cx, cy] > 0)
              num7 +=  Math.Round( (25 * behaviourStats.Weight[9]) / 100.0);
            if (behaviourStats.Weight[6] > 0 && self.data.MapObj[0].HexObj[cx, cy].UnitCounter == -1 && num12 > 0)
              num7 += num15 * 50;
            if (behaviourStats.Weight[11] > 0)
            {
              if (num16 > tempUnitPower * 4)
                num7 =  Math.Round( num7 * 0.1);
              else if (num16 > tempUnitPower * 3)
                num7 =  Math.Round( num7 * 0.3);
              else if (num16 > tempUnitPower * 2)
                num7 =  Math.Round( num7 * 0.5);
              else if (num16 > tempUnitPower * 1)
                num7 =  Math.Round( num7 * 0.8);
            }
            if (behaviourStats.Weight[2] > DrawMod.RandyNumber.Next(0, 100) | behaviourStats.Weight[1] > DrawMod.RandyNumber.Next(0, 100) && x1 == cx & y1 == cy & num17 > 0)
              num7 +=  Math.Round( (40f * Math.Min(1f,  tempUnitPower /  (num16 + 1))));
            if (behaviourStats.Weight[7] > 0)
            {
              if (num18 > 0)
                num7 += Math.Min(70, 30 * num18);
              if (self.data.MapObj[0].HexObj[cx, cy].Location > -1)
                num7 += 50;
            }
            else
            {
              if (num18 > 0)
                num7 += Math.Min(30, 10 * num18);
              if (self.data.MapObj[0].HexObj[cx, cy].Location > -1)
                num7 += 15;
            }
            if (behaviourStats.Weight[8] > 0)
            {
              let mut val2: i32 = self.data.LandscapeTypeObj[self.data.MapObj[0].HexObj[cx, cy].LandscapeType].HidePts - self.data.LandscapeTypeObj[self.data.MapObj[0].HexObj[x1, y1].LandscapeType].HidePts;
              if (val2 > 0 | num3 < 1)
                num7 += Math.Min(45, val2);
            }
            index3: i32;
            if (behaviourStats.Weight[10] > 0 && self.data.MapObj[index3].HexObj[x1, y1].UnitCounter < 2 && self.data.MapObj[index3].HexObj[cx, cy].UnitCounter < 1)
            {
              if (!(cx == x1 & cy == y1))
              {
                if (self.data.MapObj[index3].HexObj[cx, cy].UnitCounter > -1)
                  num7 += 25;
              }
              else if (self.data.MapObj[index3].HexObj[cx, cy].UnitCounter > 0)
                num7 += 25;
            }
            if (num3 > 0)
            {
              if (hexLibVarValue != num2)
                num7 =  Math.Round( num7 / 2.0);
            }
            else if (hexLibVarValue != num1)
              num7 =  Math.Round( num7 / 5.0);
            if (num7 > num5)
            {
              num5 = num7;
              x2_1 = cx;
              y2_1 = cy;
            }
          }
        }
      }
      if (x2_1 <= -1)
        return;
      DrawMod.TGame.ProcessingObj.ExecuteMovement(unr, x1, y1, 0, x2_1, y2_1, 0);
    }

    pub fn ExecuteAnts_Animal_SpecialRules(unr: i32)
    {
      let mut sfCount: i32 = self.data.UnitObj[unr].SFCount;
      for (let mut index: i32 = 0; index <= sfCount; index += 1)
      {
        let mut sf: i32 = self.data.UnitObj[unr].SFList[index];
        let mut type: i32 = self.data.SFObj[sf].Type;
        if (self.data.SFTypeObj[type].SFTypeVar[8] > 0)
        {
          let mut num: i32 =  Math.Round(Conversion.Val(Strings.Mid(self.data.SFTypeObj[type].SFTypeVar[8].ToString(), 2, 1)));
          if (num > 0 && self.data.SFObj[sf].Mor > 50 - num * 8)
            self.data.SFObj[sf].Mor = 50 - num * 8;
        }
      }
    }

    pub fn ExecuteAnts_MoveRandomly_FreeFolk(unr: i32)
    {
      name: String = "ANT_120_Ant_AI";
      self.ai.ClearLog();
      data: DataClass = self.data;
      str: String = "freefolk";
       local1: String =  str;
      let mut libVar: i32 = data.FindLibVar( local1, "SE_Data");
      let mut x1: i32 = self.data.UnitObj[unr].X;
      let mut y1: i32 = self.data.UnitObj[unr].Y;
      if (self.data.UnitObj[unr].PreDef == -1 & x1 > -1 & self.data.UnitObj[unr].Regime == self.data.Turn)
      {
        handyFunctionsObj: HandyFunctionsclass = DrawMod.TGame.HandyFunctionsObj;
        let mut unr1: i32 = unr;
        let mut x2: i32 = x1;
        let mut y2: i32 = y1;
        CustomDC2AICalls customDc2AiCalls = (CustomDC2AICalls) null;
         CustomDC2AICalls local2 =  customDc2AiCalls;
        handyFunctionsObj.MakeMovePrediction3(unr1, x2, y2, 0, ismove: true, tcustomAi: ( local2));
        let mut x2_1: i32 = -1;
        let mut num1: i32 = 50;
        if (self.map.HexObj[x1, y1].GetHexLibVarValue(libVar) > 0)
          num1 = 999999;
        if (DrawMod.RandyNumber.Next(0, 100) < 33)
          num1 = 0;
        let mut mapWidth: i32 = self.map.MapWidth;
        y2_1: i32;
        for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 = self.map.MapHeight;
          for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
          {
            if (DrawMod.TGame.EditObj.TempValue[0].Value[index1, index2] < 999)
            {
              if (self.map.HexObj[index1, index2].Regime > 5)
                index1 = index1;
              let mut hexLibVarValue: i32 = self.map.HexObj[index1, index2].GetHexLibVarValue(libVar);
              let mut num2: i32 = DrawMod.RandyNumber.Next(0, 100);
              if (DrawMod.RandyNumber.Next(0, 100) > 50 & hexLibVarValue > 0)
                num2 += hexLibVarValue;
              if (num2 > num1)
              {
                num1 = num2;
                x2_1 = index1;
                y2_1 = index2;
              }
            }
          }
        }
        if (x2_1 > -1)
          DrawMod.TGame.ProcessingObj.ExecuteMovement(unr, x1, y1, 0, x2_1, y2_1, 0);
      }
      self.ai.WriteLog(name);
    }

    pub fn ExecuteAnts_MoveRandomly_Animal(unr: i32)
    {
      name: String = "ANT_120_Ant_AI";
      self.ai.ClearLog();
      let mut x1: i32 = self.data.UnitObj[unr].X;
      let mut y1: i32 = self.data.UnitObj[unr].Y;
      if (self.data.UnitObj[unr].PreDef == -1 & x1 > -1 & self.data.UnitObj[unr].Regime == self.data.Turn)
      {
        handyFunctionsObj: HandyFunctionsclass = DrawMod.TGame.HandyFunctionsObj;
        let mut unr1: i32 = unr;
        let mut x2: i32 = x1;
        let mut y2: i32 = y1;
        CustomDC2AICalls customDc2AiCalls = (CustomDC2AICalls) null;
         CustomDC2AICalls local =  customDc2AiCalls;
        handyFunctionsObj.MakeMovePrediction3(unr1, x2, y2, 0, ismove: true, tcustomAi: ( local));
        let mut x2_1: i32 = -1;
        let mut num1: i32 = 50;
        let mut mapWidth: i32 = self.map.MapWidth;
        y2_1: i32;
        for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 = self.map.MapHeight;
          for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
          {
            if (DrawMod.TGame.EditObj.TempValue[0].Value[index1, index2] < 999)
            {
              if (self.map.HexObj[index1, index2].Regime > 5)
                index1 = index1;
              let mut num2: i32 = DrawMod.RandyNumber.Next(0, 100);
              if (num2 > num1)
              {
                num1 = num2;
                x2_1 = index1;
                y2_1 = index2;
              }
            }
          }
        }
        if (x2_1 > -1)
          DrawMod.TGame.ProcessingObj.ExecuteMovement(unr, x1, y1, 0, x2_1, y2_1, 0);
      }
      self.ai.WriteLog(name);
    }

    pub fn ExecuteAnts_Advanced_AttackRandomly(unr: i32)
    {
      let mut sfCount: i32 = self.data.UnitObj[unr].SFCount;
      type: i32;
      for (let mut index: i32 = 0; index <= sfCount; index += 1)
      {
        let mut sf: i32 = self.data.UnitObj[unr].SFList[index];
        type = self.data.SFObj[sf].Type;
        let mut qty: i32 = self.data.SFObj[sf].Qty;
      }
      SimpleList behaviourStats = self.ai.game.EventRelatedObj.AlienFauna_GetBehaviourStats(type);
      let mut x1: i32 = self.data.UnitObj[unr].X;
      let mut y1: i32 = self.data.UnitObj[unr].Y;
      if (!(self.data.UnitObj[unr].PreDef == -1 & x1 > -1 & self.data.UnitObj[unr].Regime == self.data.Turn))
        return;
      handyFunctionsObj: HandyFunctionsclass = DrawMod.TGame.HandyFunctionsObj;
      let mut unr1: i32 = unr;
      let mut x2: i32 = x1;
      let mut y2: i32 = y1;
      CustomDC2AICalls customDc2AiCalls = (CustomDC2AICalls) null;
       CustomDC2AICalls local =  customDc2AiCalls;
      handyFunctionsObj.MakeMovePrediction3(unr1, x2, y2, 0, attack: true, ismove: true, tcustomAi: ( local));
      let mut num1: i32 = -1;
      let mut num2: i32 = 105 - behaviourStats.Weight[5];
      if (behaviourStats.Weight[5] < 1)
        num2 = 60;
      let mut tfacing: i32 = 1;
      num3: i32;
      do
      {
        Coordinate coordinate = DrawMod.TGame.HandyFunctionsObj.HexNeighbour(x1, y1, 0, tfacing);
        if (coordinate.onmap)
        {
          let mut x3: i32 = coordinate.x;
          let mut y3: i32 = coordinate.y;
          if (DrawMod.TGame.EditObj.TempValue[0].Value[x3, y3] < 999 && DrawMod.TGame.HandyFunctionsObj.VisibleEnemyUnitsInHex(x3, y3, 0, self.data.Turn, true))
          {
            let mut num4: i32 = DrawMod.RandyNumber.Next(0, 100);
            if (num4 > num2)
            {
              num2 = num4;
              num1 = x3;
              num3 = y3;
            }
          }
        }
        tfacing += 1;
      }
      while (tfacing <= 6);
      if (num1 <= -1)
        return;
      DrawMod.TGame.EditObj.TempUnitList = new WindowsApplication1.UnitList();
      DrawMod.TGame.EditObj.TempUnitList.add(unr);
      Coordinate Target = Coordinate::new();
      Target.onmap = true;
      Target.x = num1;
      Target.y = num3;
      DrawMod.TGame.TempCombat = new CombatClass(DrawMod.TGame);
      if (DrawMod.TGame.Data.LandscapeTypeObj[self.map.HexObj[Target.x, Target.y].LandscapeType].IsSea)
        DrawMod.TGame.TempCombat.Init(Target, 1, DrawMod.TGame.EditObj.TempUnitList, 12);
      else
        DrawMod.TGame.TempCombat.Init(Target, 1, DrawMod.TGame.EditObj.TempUnitList, 2);
      DrawMod.TGame.TempCombat.DoBattle();
      DrawMod.TGame.TempCombat.EndBattle();
    }

    pub fn ExecuteAnts_AttackRandomly(unr: i32)
    {
      let mut x1: i32 = self.data.UnitObj[unr].X;
      let mut y1: i32 = self.data.UnitObj[unr].Y;
      if (!(self.data.UnitObj[unr].PreDef == -1 & x1 > -1 & self.data.UnitObj[unr].Regime == self.data.Turn))
        return;
      handyFunctionsObj: HandyFunctionsclass = DrawMod.TGame.HandyFunctionsObj;
      let mut unr1: i32 = unr;
      let mut x2: i32 = x1;
      let mut y2: i32 = y1;
      CustomDC2AICalls customDc2AiCalls = (CustomDC2AICalls) null;
       CustomDC2AICalls local =  customDc2AiCalls;
      handyFunctionsObj.MakeMovePrediction3(unr1, x2, y2, 0, attack: true, ismove: true, tcustomAi: ( local));
      let mut num1: i32 = -1;
      let mut num2: i32 = 60;
      let mut tfacing: i32 = 1;
      num3: i32;
      do
      {
        Coordinate coordinate = DrawMod.TGame.HandyFunctionsObj.HexNeighbour(x1, y1, 0, tfacing);
        if (coordinate.onmap)
        {
          let mut x3: i32 = coordinate.x;
          let mut y3: i32 = coordinate.y;
          if (DrawMod.TGame.EditObj.TempValue[0].Value[x3, y3] < 999 && DrawMod.TGame.HandyFunctionsObj.VisibleEnemyUnitsInHex(x3, y3, 0, self.data.Turn, true))
          {
            let mut num4: i32 = DrawMod.RandyNumber.Next(0, 100);
            if (num4 > num2)
            {
              num2 = num4;
              num1 = x3;
              num3 = y3;
            }
          }
        }
        tfacing += 1;
      }
      while (tfacing <= 6);
      if (num1 <= -1)
        return;
      DrawMod.TGame.EditObj.TempUnitList = new WindowsApplication1.UnitList();
      DrawMod.TGame.EditObj.TempUnitList.add(unr);
      Coordinate Target = Coordinate::new();
      Target.onmap = true;
      Target.x = num1;
      Target.y = num3;
      DrawMod.TGame.TempCombat = new CombatClass(DrawMod.TGame);
      if (DrawMod.TGame.Data.LandscapeTypeObj[self.map.HexObj[Target.x, Target.y].LandscapeType].IsSea)
        DrawMod.TGame.TempCombat.Init(Target, 1, DrawMod.TGame.EditObj.TempUnitList, 12);
      else
        DrawMod.TGame.TempCombat.Init(Target, 1, DrawMod.TGame.EditObj.TempUnitList, 2);
      DrawMod.TGame.TempCombat.DoBattle();
      DrawMod.TGame.TempCombat.EndBattle();
    }

    pub fn ExecuteSubGroup_AreaDefense(groupId: i32)
    {
      name: String = "ANT_120_Ant_Group_" + groupId.ToString() + "_Execute_SubGroup_AreaDefense";
      self.ai.ClearLog();
      let mut counter1: i32 = self.UnitList.Counter;
      num1: i32;
      for (let mut index: i32 = 0; index <= counter1; index += 1)
      {
        if (self.UnitList.Data1[index] == groupId && self.UnitList.Data2[index] == 2)
          num1 += 1;
      }
      if (num1 < 1)
        return;
      AIMatrix aiMatrix1 = new AIMatrix( self.ai);
      bool[] flagArray = new bool[self.data.UnitCounter + 1];
      bool flag = true;
      while (flag)
      {
        flag = false;
        let mut counter2: i32 = self.UnitList.Counter;
        for (let mut index1: i32 = 0; index1 <= counter2; index1 += 1)
        {
          if (self.UnitList.Data1[index1] == groupId && self.UnitList.Data2[index1] == 2)
          {
            let mut historicalUnitById: i32 = DrawMod.TGame.HandyFunctionsObj.GetHistoricalUnitByID(self.UnitList.Id[index1]);
            let mut unitByHistorical: i32 = DrawMod.TGame.HandyFunctionsObj.GetUnitByHistorical(historicalUnitById);
            let mut x: i32 = self.data.UnitObj[unitByHistorical].X;
            let mut y: i32 = self.data.UnitObj[unitByHistorical].Y;
            if (!flagArray[unitByHistorical])
            {
              AIMatrix aiMatrix2 = new AIMatrix( self.ai);
              AIMatrix aiMatrix3 = new AIMatrix( self.ai);
              AIMatrix aiMatrix4 = new AIMatrix( self.ai);
              let mut mapWidth1: i32 = self.map.MapWidth;
              for (let mut index2: i32 = 0; index2 <= mapWidth1; index2 += 1)
              {
                let mut mapHeight: i32 = self.map.MapHeight;
                for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                {
                  if (self.areas.Value[index2, index3] == groupId)
                  {
                    let mut unitCounter: i32 = self.map.HexObj[index2, index3].UnitCounter;
                    for (let mut index4: i32 = 0; index4 <= unitCounter; index4 += 1)
                    {
                      index1 = self.map.HexObj[index2, index3].UnitList[index4];
                      if (index1 != unitByHistorical)
                      {
                        let mut regime: i32 = self.map.HexObj[index2, index3].Regime;
                        if (regime == self.data.Turn)
                        {
                          numArray1: Vec<i32> = aiMatrix2.Value;
                          numArray2: Vec<i32> = numArray1;
                          let mut index5: i32 = index2;
                          let mut index6: i32 = index5;
                          let mut index7: i32 = index3;
                          let mut index8: i32 = index7;
                          let mut num2: i32 = numArray1[index5, index7] + self.data.UnitObj[index1].TempUnitPower;
                          numArray2[index6, index8] = num2;
                        }
                        else if (regime > -1 && !self.data.RegimeObj[regime].Sleep & self.data.RegimeObj[regime].RegimeRel[self.data.Turn] == 0)
                        {
                          numArray3: Vec<i32> = aiMatrix3.Value;
                          numArray4: Vec<i32> = numArray3;
                          let mut index9: i32 = index2;
                          let mut index10: i32 = index9;
                          let mut index11: i32 = index3;
                          let mut index12: i32 = index11;
                          let mut num3: i32 = numArray3[index9, index11] + self.data.UnitObj[index1].TempUnitPower;
                          numArray4[index10, index12] = num3;
                        }
                      }
                    }
                  }
                }
              }
              aiMatrix3.ExpandAndRemovePercentageForAnyRegime(10, 0.8f);
              aiMatrix2.ExpandAndRemovePercentageForAnyRegime(10, 0.8f);
              let mut num4: i32 = 0;
              let mut mapWidth2: i32 = self.map.MapWidth;
              for (let mut index13: i32 = 0; index13 <= mapWidth2; index13 += 1)
              {
                let mut mapHeight: i32 = self.map.MapHeight;
                for (let mut index14: i32 = 0; index14 <= mapHeight; index14 += 1)
                {
                  if (self.areas.Value[index13, index14] != groupId)
                  {
                    aiMatrix2.Value[index13, index14] = 0;
                    aiMatrix3.Value[index13, index14] = 0;
                    aiMatrix4.Value[index13, index14] = 0;
                  }
                  else
                  {
                    aiMatrix4.Value[index13, index14] = aiMatrix3.Value[index13, index14] * 3 - aiMatrix2.Value[index13, index14];
                    if (aiMatrix4.Value[index13, index14] < num4)
                      num4 = aiMatrix4.Value[index13, index14];
                  }
                }
              }
              if (num4 < 0)
                aiMatrix4.AddValue(Math.Abs(num4));
              aiMatrix4.Percentify();
              let mut mapWidth3: i32 = self.map.MapWidth;
              for (let mut index15: i32 = 0; index15 <= mapWidth3; index15 += 1)
              {
                let mut mapHeight: i32 = self.map.MapHeight;
                for (let mut index16: i32 = 0; index16 <= mapHeight; index16 += 1)
                {
                  if (self.areas.Value[index15, index16] != groupId)
                    aiMatrix4.Value[index15, index16] = 0;
                }
              }
              DrawMod.TGame.HandyFunctionsObj.MakeMovePrediction(unitByHistorical, x, y, 0, ismove: true);
              let mut x2: i32 = -1;
              let mut num5: i32 = -999999;
              let mut num6: i32 = aiMatrix4.Value[x, y];
              let mut mapWidth4: i32 = self.map.MapWidth;
              y2: i32;
              for (let mut index17: i32 = 0; index17 <= mapWidth4; index17 += 1)
              {
                let mut mapHeight: i32 = self.map.MapHeight;
                for (let mut index18: i32 = 0; index18 <= mapHeight; index18 += 1)
                {
                  if (DrawMod.TGame.EditObj.TempValue[0].Value[index17, index18] < 9999 & self.areas.Value[index17, index18] == groupId)
                  {
                    let mut num7: i32 = aiMatrix4.Value[index17, index18];
                    if (num7 > num5)
                    {
                      num5 = num7;
                      x2 = index17;
                      y2 = index18;
                    }
                  }
                }
              }
              if ( num5 >  num6 * 1.1)
              {
                DrawMod.TGame.ProcessingObj.ExecuteMovement(unitByHistorical, x, y, 0, x2, y2, 0);
                flag = true;
                break;
              }
              flagArray[unitByHistorical] = true;
            }
          }
        }
      }
      self.ai.WriteLog(name);
    }

    pub fn SetGroups()
    {
      name: String = "ANT_100_Ant_Groups";
      self.ai.ClearLog();
      data: DataClass = self.data;
      str1: String = "freeFolk";
       local: String =  str1;
      let mut libVar: i32 = data.FindLibVar( local, "SE_Data");
      self.areas = new AIMatrix( self.ai);
      self.GroupList = SimpleList::new();
      self.UnitList = SimpleList::new();
      let mut locCounter: i32 = self.data.LocCounter;
      for (let mut index: i32 = 0; index <= locCounter; index += 1)
      {
        let mut x: i32 = self.data.LocObj[index].X;
        let mut y: i32 = self.data.LocObj[index].Y;
        if (self.map.HexObj[x, y].Regime == self.data.Turn)
        {
          self.GroupList.Add(x * 1000 + y, 1, x, y);
          self.areas.Value[x, y] = x * 1000 + y;
        }
      }
      self.areas.ExpandUniquesValuesForSameRegime(10);
      let mut mapWidth: i32 = self.map.MapWidth;
      for (let mut tdata1: i32 = 0; tdata1 <= mapWidth; tdata1 += 1)
      {
        let mut mapHeight: i32 = self.map.MapHeight;
        for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
        {
          if (self.map.HexObj[tdata1, tdata2].Regime == self.data.Turn && self.areas.Value[tdata1, tdata2] == 0 && self.map.HexObj[tdata1, tdata2].GetHexLibVarValue(libVar) > 0)
          {
            self.GroupList.Add(tdata1 * 1000 + tdata2, 1, tdata1, tdata2);
            self.areas.Value[tdata1, tdata2] = tdata1 * 1000 + tdata2;
          }
        }
      }
      self.areas.ExpandUniquesValuesForSameRegime(10);
      self.areas.ExpandValueForAnyRegime(5);
      let mut unitCounter1: i32 = self.data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
      {
        if (self.data.UnitObj[index].Regime == self.data.Turn & self.data.UnitObj[index].X > -1)
        {
          let mut x: i32 = self.data.UnitObj[index].X;
          let mut y: i32 = self.data.UnitObj[index].Y;
          if (self.areas.Value[x, y] == 0)
          {
            self.GroupList.Add(x * 1000 + y, 1, x, y);
            self.areas.Value[x, y] = x * 1000 + y;
            self.areas.ExpandSpecificValueForSameRegime(x * 1000 + y, 10);
          }
        }
      }
      self.areas.ExpandValueForAnyRegime(10);
      let mut unitCounter2: i32 = self.data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter2; index += 1)
      {
        if (self.data.UnitObj[index].Regime == self.data.Turn & self.data.UnitObj[index].Historical > -1)
        {
          let mut x: i32 = self.data.UnitObj[index].X;
          let mut y: i32 = self.data.UnitObj[index].Y;
          if (x > -1 && self.areas.Value[x, y] > 0 && self.GroupList.FindNr(self.areas.Value[x, y]) > -1)
            self.UnitList.Add(self.data.HistoricalUnitObj[self.data.UnitObj[index].Historical].ID, 1, self.areas.Value[x, y]);
        }
      }
      let mut counter1: i32 = self.GroupList.Counter;
      for (let mut index1: i32 = 0; index1 <= counter1; index1 += 1)
      {
        let mut num1: i32 = self.GroupList.Data1[index1];
        let mut num2: i32 = self.GroupList.Data2[index1];
        let mut num3: i32 = 0;
        let mut counter2: i32 = self.UnitList.Counter;
        for (let mut index2: i32 = 0; index2 <= counter2; index2 += 1)
        {
          if (self.UnitList.Data1[index2] == self.GroupList.Id[index1] && DrawMod.TGame.HandyFunctionsObj.GetHistoricalUnitByID(self.UnitList.Id[index2]) > -1)
            num3 += 1;
        }
        let mut num4: i32 = 0;
        let mut counter3: i32 = self.UnitList.Counter;
        for (let mut index3: i32 = 0; index3 <= counter3; index3 += 1)
        {
          if (self.UnitList.Data1[index3] == self.GroupList.Id[index1] && DrawMod.TGame.HandyFunctionsObj.GetHistoricalUnitByID(self.UnitList.Id[index3]) > -1)
          {
            num4 += 1;
            self.UnitList.Data2[index3] =  num4 > Math.Ceiling( num3 / 2.0) ? 1 : 2;
          }
        }
      }
      let mut counter4: i32 = self.GroupList.Counter;
      for (let mut index4: i32 = 0; index4 <= counter4; index4 += 1)
      {
        let mut x: i32 = self.GroupList.Data1[index4];
        let mut y: i32 = self.GroupList.Data2[index4];
        str2: String = DrawMod.TGame.HandyFunctionsObj.GetHexName(x, y, 0) + " (" + x.ToString() + "," + y.ToString() + ")";
        self.ai.AddLog("Group #" + self.GroupList.Id[index4].ToString() + " : " + str2);
        let mut counter5: i32 = self.UnitList.Counter;
        for (let mut index5: i32 = 0; index5 <= counter5; index5 += 1)
        {
          if (self.UnitList.Data1[index5] == self.GroupList.Id[index4])
          {
            let mut historicalUnitById: i32 = DrawMod.TGame.HandyFunctionsObj.GetHistoricalUnitByID(self.UnitList.Id[index5]);
            if (historicalUnitById > -1)
            {
              let mut unitByHistorical: i32 = DrawMod.TGame.HandyFunctionsObj.GetUnitByHistorical(historicalUnitById);
              str3: String = "";
              if (self.UnitList.Data2[index5] == 0)
                str3 = "WANDER";
              if (self.UnitList.Data2[index5] == 1)
                str3 = "DEFEND_AREA";
              if (self.UnitList.Data2[index5] == 2)
                str3 = "DEFEND_CITY";
              if (self.UnitList.Data2[index5] == 3)
                str3 = "HELP";
              if (self.UnitList.Data2[index5] == 4)
                str3 = "ATTACK";
              self.ai.AddLog("-" + self.data.UnitObj[unitByHistorical].Name + ", " + str3);
            }
          }
        }
      }
      self.ai.WriteLog(name);
    }
  }
}
