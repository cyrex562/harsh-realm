// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CustomCombatCalls
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class CustomCombatCalls
  {
    pub SimpleList SL;
    pub SimpleList MODL;
    pub SimpleList logLeaderBonus;
    pub SimpleList logLeaderBonusDef;
    pub SimpleList otherBonus;
    pub SimpleList otherBonusDef;
    pub SimpleList SkillCacheList;
    pub slotModelsAdditive: i32;
    pub slotModelsClean: i32;
    pub slotModels: i32;
    pub slotGameKeys: i32;
    pub slotLeaders: i32;
    pub slotRegimes: i32;
    pub slotZones: i32;
    pub slotUnitFeats: i32;
    pub slotCharSkill: i32;
    pub slotChar: i32;
    pub slotAssets: i32;
    pub slotAssetTypes: i32;
    pub slotBehaviour: i32;
    pub slotSkill: i32;
    pub slotRegimeKey: i32;
    pub slotMM: i32;
    pub slotSkillTypes: i32;
    pub rememberRadPts: i32;
    pub airEnabled: bool;
    pub float SKILLEFFECT;
    pub EntrBunkerInit: bool;
    pub float EntrBunkerMod;
    pub float EntrAssetMod;

    pub CustomCombatCalls()
    {
      this.SKILLEFFECT = 0.5f;
      this.EntrBunkerMod = 1f;
      this.EntrAssetMod = 1f;
      if ((int) Math.Round(Conversion.Val(DrawMod.TGame.Data.Designer)) < 103)
        this.SKILLEFFECT = 1f;
      this.SL = SimpleList::new();
      this.MODL = SimpleList::new();
      this.logLeaderBonus = SimpleList::new();
      this.logLeaderBonusDef = SimpleList::new();
      this.otherBonus = SimpleList::new();
      this.otherBonusDef = SimpleList::new();
      this.SkillCacheList = SimpleList::new();
      this.slotModelsAdditive = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 363, 0, 0));
      this.slotModelsClean = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 229, 0, 0));
      this.slotModels = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 228, 0, 0));
      this.slotGameKeys = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0));
      this.slotLeaders = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0));
      this.slotRegimes = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
      this.slotZones = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
      this.slotUnitFeats = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 402, 0, 0));
      this.slotCharSkill = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 199, 0, 0));
      this.slotChar = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 196, 0, 0));
      this.slotAssets = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 148, 0, 0));
      this.slotAssetTypes = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 144, 0, 0));
      this.slotBehaviour = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 404, 0, 0));
      this.slotSkill = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 199, 0, 0));
      this.slotRegimeKey = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0));
      this.slotMM = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 446, 0, 0));
      this.slotSkillTypes = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 203, 0, 0));
      this.EntrBunkerInit = false;
      this.airEnabled = DrawMod.TGame.EventRelatedObj.Helper_AirEnabled();
    }

    pub HasCustumCalls: bool() => true;

    pub fn IndividualMORMod(ref cc: CombatClass, inr: i32, morMod: i32, let mut sfnr: i32 =  -1) -> i32
    {
      people: i32;
      if (inr > -1)
      {
        people = DrawMod.TGame.Data.SFObj[cc.IList[inr].ISFNr].People;
      }
      else
      {
        if (sfnr <= -1)
          return morMod;
        people = DrawMod.TGame.Data.SFObj[sfnr].People;
      }
      if (DrawMod.TGame.Data.PeopleObj[people].tv0 == (int) Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotGameKeys].GetData(0, 37, 2))))
        morMod = 0;
      return morMod;
    }

    pub fn IndividualXPMod(ref cc: CombatClass, inr: i32, xpToBeGiven: i32, let mut sfnr: i32 =  -1) -> i32
    {
      people: i32;
      if (inr > -1)
      {
        people = DrawMod.TGame.Data.SFObj[cc.IList[inr].ISFNr].People;
      }
      else
      {
        if (sfnr <= -1)
          return xpToBeGiven;
        people = DrawMod.TGame.Data.SFObj[sfnr].People;
      }
      if (DrawMod.TGame.Data.PeopleObj[people].tv0 == (int) Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotGameKeys].GetData(0, 37, 2))))
        xpToBeGiven = 0;
      return xpToBeGiven;
    }

    pub fn CachedSkillRoll(game: GameClass, charId: i32, skillId: i32) -> i32
    {
      let mut nr: i32 =  this.SkillCacheList.FindNr(charId, skillId);
      tdata2: i32;
      if (nr < 0)
      {
        tdata2 = game.EventRelatedObj.CheckHardcoded_SkillRoll(charId, skillId, false, 0, false);
        this.SkillCacheList.Add(charId, 1, skillId, tdata2, CheckExistence: false);
      }
      else
        tdata2 = this.SkillCacheList.Data2[nr];
      return tdata2;
    }

    pub fn EntrenchModifier(ref cc: CombatClass, indNr: i32, curX: i32, curY: i32, entr: i32) -> i32
    {
      if (cc.IList[indNr].IAttacker == 0 & (int) Math.Round(Conversion.Val(cc.game.Data.Designer)) >= 112)
      {
        if (!this.EntrBunkerInit)
        {
          data: DataClass = cc.game.Data;
          str: String = "bunkerPoints".to_owned();
          ref local: String = ref str;
          let mut libVar: i32 =  data.FindLibVar(ref local, "SE_Data");
          let mut hexLibVarValue: i32 =  cc.game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].GetHexLibVarValue(libVar);
          this.EntrAssetMod = 1f;
          let mut num1: i32 =  0;
          let mut length: i32 =  cc.game.Data.StringListObj[this.slotAssets].Length;
          for (let mut index: i32 =  0; index <= length; index += 1)
          {
            let mut num2: i32 =  (int) Math.Round(Conversion.Val(cc.game.Data.StringListObj[this.slotAssets].Data[index, 3]));
            let mut num3: i32 =  (int) Math.Round(Conversion.Val(cc.game.Data.StringListObj[this.slotAssets].Data[index, 4]));
            if (num2 == cc.TargetX & num3 == cc.TargetY)
            {
              let mut idValue: i32 =  (int) Math.Round(Conversion.Val(cc.game.Data.StringListObj[this.slotAssets].Data[index, 1]));
              let mut num4: i32 =  (int) Math.Round(Conversion.Val(cc.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue, 2)));
              let mut num5: i32 =  (int) Math.Round(Conversion.Val(cc.game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue, 13)));
              if (num5 < 1)
                num5 = 1;
              num1 += num4 * (5 + num5);
            }
          }
          if (num1 > 0)
            hexLibVarValue += num1;
          this.EntrBunkerInit = true;
          this.EntrBunkerMod = 1f;
          if (hexLibVarValue > 0)
          {
            let mut num6: i32 =  0;
            let mut icounter: i32 =  cc.ICounter;
            for (let mut index: i32 =  0; index <= icounter; index += 1)
            {
              if (cc.IList[index].IAttacker == 0)
                num6 += cc.game.Data.SFTypeObj[cc.IList[index].ISFType].PowerPts * 10;
            }
            let mut num7: i32 =  (int) Math.Round(Math.Sqrt( hexLibVarValue) * 10.0);
            if (num6 < num7)
              num6 = num7;
            while (num6 > 0 & hexLibVarValue > 0)
            {
              float num8 =  hexLibVarValue /  num6;
              if ( num8 > 1.0)
                num8 = 1f;
              hexLibVarValue -= num6;
              this.EntrBunkerMod += num8 * 0.25f;
            }
          }
        }
        entr = (int) Math.Round( ( entr * this.EntrBunkerMod));
      }
      return entr;
    }

    pub fn AIHelpCombatChanger(ref cc: CombatClass, forNr: i32, againstNr: i32, tHelpCombat: i32) -> i32
    {
      if (tHelpCombat > 0 && cc.game.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[cc.IList[forNr].IUnr].Regime].AI && cc.game.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[cc.IList[againstNr].IUnr].Regime].AI)
      {
        let mut id1: i32 =  cc.game.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[cc.IList[forNr].IUnr].Regime].id;
        let mut id2: i32 =  cc.game.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[cc.IList[forNr].IUnr].Regime].id;
        if ((int) Math.Round(Conversion.Val(cc.game.Data.StringListObj[this.slotRegimes].GetData(0, id1, 1))) > 1 & (int) Math.Round(Conversion.Val(cc.game.Data.StringListObj[this.slotRegimes].GetData(0, id2, 1))) == 1)
          tHelpCombat = 0;
      }
      return tHelpCombat;
    }

    pub void PersonalCombatRoll(
      ref cc: CombatClass,
      game: GameClass,
      unr: i32,
      inr: i32,
      randomizedDiff: i32,
      wasKilledByString: String,
      bool unitLossCheck,
      bool hexLossCheck)
    {
      let mut index1: i32 =  -1;
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      if (unr > -1)
      {
        index1 = game.Data.UnitObj[unr].Historical;
        num1 = game.Data.RegimeObj[game.Data.UnitObj[unr].Regime].id;
        num2 = game.Data.HistoricalUnitObj[index1].ID;
      }
      let mut idValue3_1: i32 =  -1;
      let mut idValue3_2: i32 =  -1;
      let mut index2: i32 =  -1;
      let mut index3: i32 =  -1;
      let mut index4: i32 =  -1;
      let mut index5: i32 =  -1;
      bool flag1 = false;
      bool flag2 = false;
      if (cc.CombatType2 == 16 | cc.CombatType == 13 | cc.CombatType == 5)
        hexLossCheck = false;
      if (index1 > -1)
      {
        if (game.Data.HistoricalUnitObj[index1].Type == 8)
        {
          flag2 = true;
          idValue3_2 = num2;
          index5 = unr;
          index2 = index1;
        }
        else if (game.Data.UnitObj[unr].IsHQ)
        {
          flag1 = true;
          idValue3_1 = num2;
          index4 = unr;
          index3 = index1;
        }
      }
      if (idValue3_1 > 0)
      {
        let mut hq: i32 =  game.Data.UnitObj[index4].HQ;
        if (hq > -1)
        {
          index5 = hq;
          index2 = game.Data.UnitObj[index5].Historical;
          idValue3_2 = game.Data.HistoricalUnitObj[index2].ID;
        }
      }
      else if (idValue3_2 <= 0 && unr > -1 && game.Data.UnitObj[unr].HQ > -1)
      {
        index4 = game.Data.UnitObj[unr].HQ;
        index3 = game.Data.UnitObj[index4].Historical;
        idValue3_1 = game.Data.HistoricalUnitObj[index3].ID;
        if (index4 > -1)
        {
          index3 = game.Data.UnitObj[index4].Historical;
          if (game.Data.HistoricalUnitObj[index3].Type == 8)
          {
            index5 = index4;
            index2 = game.Data.UnitObj[index5].Historical;
            idValue3_2 = game.Data.HistoricalUnitObj[index2].ID;
            index4 = -1;
            idValue3_1 = -1;
            index3 = -1;
          }
          if (index4 > -1)
          {
            index5 = game.Data.UnitObj[index4].HQ;
            if (index5 > -1)
            {
              index2 = game.Data.UnitObj[index5].Historical;
              idValue3_2 = game.Data.HistoricalUnitObj[index2].ID;
            }
          }
        }
      }
      str1: String = "";
      str2: String;
      num3: i32;
      if (idValue3_1 > 0)
      {
        let mut integer: i32 =  Conversions.ToInteger(game.Data.StringListObj[this.slotLeaders].GetData3(5, num1, 6, 3, 7, idValue3_1, 0));
        if (integer > 0)
        {
          let mut num4: i32 =  0;
          let mut unitCounter: i32 =  game.Data.UnitCounter;
          for (let mut index6: i32 =  0; index6 <= unitCounter; index6 += 1)
          {
            if (game.Data.UnitObj[index6].HQ == index4 & !flag1 | index6 == index4 & flag1)
            {
              let mut sfCount: i32 =  game.Data.UnitObj[index6].SFCount;
              for (let mut index7: i32 =  0; index7 <= sfCount; index7 += 1)
              {
                let mut sf: i32 =  game.Data.UnitObj[index6].SFList[index7];
                num4 += game.Data.SFObj[sf].Qty;
              }
            }
          }
          data1: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer, 46);
          let mut row: i32 =  game.Data.StringListObj[this.slotLeaders].FindRow(0, integer);
          texty: String;
          if (DrawMod.RandyNumber.Next(0, num4 * 10) <= 10 & (flag1 | Conversions.ToDouble(data1) >  DrawMod.RandyNumber.Next(1, 100)) | unitLossCheck & flag1)
          {
            let mut num5: i32 =  game.EventRelatedObj.CheckHardcoded_SkillRoll(integer, 10, false, 0, true);
            this.SL.AddWeight(integer, 1, 10, CheckData1Existence: true);
            let mut num6: i32 =  DrawMod.RandyNumber.Next(0, randomizedDiff) + 70;
            if (num5 < num6)
            {
              data2: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer, 3);
              data3: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer, 4);
              characterJobTitle: String = game.EventRelatedObj.Helper_GetCharacterJobTitle(integer);
              if (inr > -1)
              {
                if (str1.Length > 0)
                  str1 += " ";
                str2 = characterJobTitle + " " + data2 + " " + data3 + " was killed in action during fighting in the front line.";
                str1 += str2;
                game.EventRelatedObj.AddLeaderTempLog(integer, str2);
              }
              else if (unitLossCheck)
              {
                if (str1.Length > 0)
                  str1 += " ";
                str2 = characterJobTitle + " " + data2 + " " + data3 + " was killed in action during the destruction of " + game.Data.UnitObj[unr].Name + ".";
                str1 += str2;
                game.EventRelatedObj.AddLeaderTempLog(integer, str2);
              }
              game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader KIA", str2, integer, DetailType.CharacterDeath);
              game.Data.HistoricalUnitObj[index3].SetHisVarValue(61, 0);
              game.EventRelatedObj.HelperResetAdvisorFor(integer, false, num1);
              game.Data.StringListObj[this.slotLeaders].Data[row, 5] = 0.ToString();
              game.Data.StringListObj[this.slotLeaders].Data[row, 6] = 0.ToString();
              game.Data.StringListObj[this.slotLeaders].Data[row, 7] = 0.ToString();
              game.Data.StringListObj[this.slotLeaders].Data[row, 25] = game.Data.Round.ToString();
              game.Data.StringListObj[this.slotLeaders].Data[row, 26] = num1.ToString();
              game.EventRelatedObj.Helper_RemoveAllTraceOfLeader(integer);
            }
            else if (unitLossCheck & flag1)
            {
              game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader Survived Unit Destruction", texty, integer, DetailType.CharacterNews);
              game.Data.HistoricalUnitObj[index3].SetHisVarValue(61, 0);
              string[,] data4 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index8: i32 =  row;
              num3 = 1;
              str3: String = num3.ToString();
              data4[index8, 6] = str3;
              string[,] data5 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index9: i32 =  row;
              num3 = 0;
              str4: String = num3.ToString();
              data5[index9, 7] = str4;
            }
          }
          else if (unitLossCheck & flag1)
          {
            game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader Survived Unit Destruction", texty, integer, DetailType.CharacterNews);
            game.Data.HistoricalUnitObj[index3].SetHisVarValue(61, 0);
            string[,] data6 = game.Data.StringListObj[this.slotLeaders].Data;
            let mut index10: i32 =  row;
            num3 = 1;
            str5: String = num3.ToString();
            data6[index10, 6] = str5;
            string[,] data7 = game.Data.StringListObj[this.slotLeaders].Data;
            let mut index11: i32 =  row;
            num3 = 0;
            str6: String = num3.ToString();
            data7[index11, 7] = str6;
          }
        }
      }
      if (idValue3_2 > 0)
      {
        if (num1 == 4)
        {
          let mut num7: i32 =  num7;
        }
        let mut integer: i32 =  Conversions.ToInteger(game.Data.StringListObj[this.slotLeaders].GetData3(5, num1, 6, 4, 7, idValue3_2, 0));
        if (integer > 0)
        {
          let mut num8: i32 =  0;
          let mut unitCounter: i32 =  game.Data.UnitCounter;
          for (let mut index12: i32 =  0; index12 <= unitCounter; index12 += 1)
          {
            if (game.Data.UnitObj[index12].HQ == index5 & !flag2 | index12 == index5 & flag2)
            {
              let mut sfCount: i32 =  game.Data.UnitObj[index12].SFCount;
              for (let mut index13: i32 =  0; index13 <= sfCount; index13 += 1)
              {
                let mut sf: i32 =  game.Data.UnitObj[index12].SFList[index13];
                num8 += game.Data.SFObj[sf].Qty;
              }
            }
            else if (game.Data.UnitObj[index12].HQ > -1 && game.Data.UnitObj[game.Data.UnitObj[index12].HQ].HQ > -1)
            {
              let mut hq: i32 =  game.Data.UnitObj[game.Data.UnitObj[index12].HQ].HQ;
              let mut sfCount: i32 =  game.Data.UnitObj[hq].SFCount;
              for (let mut index14: i32 =  0; index14 <= sfCount; index14 += 1)
              {
                let mut sf: i32 =  game.Data.UnitObj[hq].SFList[index14];
                num8 += game.Data.SFObj[sf].Qty;
              }
            }
          }
          data8: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer, 46);
          let mut row: i32 =  game.Data.StringListObj[this.slotLeaders].FindRow(0, integer);
          if (DrawMod.RandyNumber.Next(0, num8 * 10) <= 5 & (flag2 | Conversions.ToDouble(data8) >  DrawMod.RandyNumber.Next(1, 100)) | unitLossCheck & flag2)
          {
            let mut num9: i32 =  game.EventRelatedObj.CheckHardcoded_SkillRoll(integer, 10, false, 0, true);
            this.SL.AddWeight(integer, 1, 10, CheckData1Existence: true);
            let mut num10: i32 =  DrawMod.RandyNumber.Next(0, randomizedDiff) + 70;
            if (num9 < num10)
            {
              data9: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer, 3);
              data10: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer, 4);
              characterJobTitle: String = game.EventRelatedObj.Helper_GetCharacterJobTitle(integer);
              if (inr > -1)
              {
                if (str1.Length > 0)
                  str1 += " ";
                str2 = characterJobTitle + " " + data9 + " " + data10 + " was killed in action during fighting in the front line.";
                str1 += str2;
                game.EventRelatedObj.AddLeaderTempLog(integer, str2);
              }
              else if (unitLossCheck)
              {
                if (str1.Length > 0)
                  str1 += " ";
                str2 = characterJobTitle + " " + data9 + " " + data10 + " was killed in action during the destruction of " + game.Data.UnitObj[unr].Name + ".";
                str1 += str2;
                game.EventRelatedObj.AddLeaderTempLog(integer, str2);
              }
              game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader KIA", str2, integer, DetailType.CharacterDeath);
              game.Data.HistoricalUnitObj[index2].SetHisVarValue(61, 0);
              game.EventRelatedObj.HelperResetAdvisorFor(integer, false, num1);
              string[,] data11 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index15: i32 =  row;
              num3 = 0;
              str7: String = num3.ToString();
              data11[index15, 5] = str7;
              string[,] data12 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index16: i32 =  row;
              num3 = 0;
              str8: String = num3.ToString();
              data12[index16, 6] = str8;
              string[,] data13 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index17: i32 =  row;
              num3 = 0;
              str9: String = num3.ToString();
              data13[index17, 7] = str9;
              game.Data.StringListObj[this.slotLeaders].Data[row, 25] = game.Data.Round.ToString();
              game.Data.StringListObj[this.slotLeaders].Data[row, 26] = num1.ToString();
              game.EventRelatedObj.Helper_RemoveAllTraceOfLeader(integer);
            }
            else if (unitLossCheck & flag2)
            {
              game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader Survived Unit Destruction", str2, integer, DetailType.CharacterNews);
              game.Data.HistoricalUnitObj[index2].SetHisVarValue(61, 0);
              string[,] data14 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index18: i32 =  row;
              num3 = 1;
              str10: String = num3.ToString();
              data14[index18, 6] = str10;
              string[,] data15 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index19: i32 =  row;
              num3 = 0;
              str11: String = num3.ToString();
              data15[index19, 7] = str11;
            }
          }
          else if (unitLossCheck & flag2)
          {
            game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader Survived Unit Destruction", str2, integer, DetailType.CharacterNews);
            game.Data.HistoricalUnitObj[index2].SetHisVarValue(61, 0);
            string[,] data16 = game.Data.StringListObj[this.slotLeaders].Data;
            let mut index20: i32 =  row;
            num3 = 1;
            str12: String = num3.ToString();
            data16[index20, 6] = str12;
            string[,] data17 = game.Data.StringListObj[this.slotLeaders].Data;
            let mut index21: i32 =  row;
            num3 = 0;
            str13: String = num3.ToString();
            data17[index21, 7] = str13;
          }
        }
      }
      if (hexLossCheck)
      {
        let mut defenderRegime: i32 =  cc.DefenderRegime;
        num1 = game.Data.RegimeObj[defenderRegime].id;
        let mut location: i32 =  game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].Location;
        if (location > -1)
        {
          let mut id1: i32 =  game.Data.LocObj[location].ID;
          let mut id2: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotRegimes].GetData(0, num1, 12)));
          let mut locationById: i32 =  game.HandyFunctionsObj.GetLocationByID(id2);
          let mut integer1: i32 =  Conversions.ToInteger(game.Data.StringListObj[this.slotZones].GetData(6, id1, 0));
          let mut integer2: i32 =  Conversions.ToInteger(game.Data.StringListObj[this.slotLeaders].GetData3(5, num1, 6, 10, 7, integer1, 0));
          if (integer2 > 0)
          {
            let mut row: i32 =  game.Data.StringListObj[this.slotLeaders].FindRow(0, integer2);
            let mut num11: i32 =  game.EventRelatedObj.CheckHardcoded_SkillRoll(integer2, 10, false, 0, true);
            this.SL.AddWeight(integer2, 1, 10, CheckData1Existence: true);
            let mut num12: i32 =  DrawMod.RandyNumber.Next(0, randomizedDiff) + 70;
            if (num11 < num12)
            {
              data18: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer2, 3);
              data19: String = game.Data.StringListObj[this.slotLeaders].GetData(0, integer2, 4);
              characterJobTitle: String = game.EventRelatedObj.Helper_GetCharacterJobTitle(integer2);
              if (str1.Length > 0)
                str1 += " ";
              str14: String = characterJobTitle + " " + data18 + " " + data19 + " was killed in action during the loss of " + game.Data.LocObj[location].Name + ".";
              str1 += str14;
              game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader KIA", str14, integer2, DetailType.CharacterDeath);
              game.EventRelatedObj.AddLeaderTempLog(integer2, str14);
              game.EventRelatedObj.HelperResetAdvisorFor(integer2, true, num1);
              string[,] data20 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index22: i32 =  row;
              num3 = 0;
              str15: String = num3.ToString();
              data20[index22, 5] = str15;
              string[,] data21 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index23: i32 =  row;
              num3 = 0;
              str16: String = num3.ToString();
              data21[index23, 6] = str16;
              string[,] data22 = game.Data.StringListObj[this.slotLeaders].Data;
              let mut index24: i32 =  row;
              num3 = 0;
              str17: String = num3.ToString();
              data22[index24, 7] = str17;
              game.Data.StringListObj[this.slotLeaders].Data[row, 25] = game.Data.Round.ToString();
              game.Data.StringListObj[this.slotLeaders].Data[row, 26] = num1.ToString();
              game.EventRelatedObj.Helper_RemoveAllTraceOfLeader(integer2);
            }
          }
          if (locationById == location)
          {
            let mut length: i32 =  game.Data.StringListObj[this.slotLeaders].Length;
            for (let mut index25: i32 =  0; index25 <= length; index25 += 1)
            {
              if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotLeaders].Data[index25, 5])) == num1)
              {
                let mut num13: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotLeaders].Data[index25, 6]));
                if (num13 == 5 | num13 == 6)
                {
                  let mut num14: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotLeaders].Data[index25, 0]));
                  let mut row: i32 =  game.Data.StringListObj[this.slotLeaders].FindRow(0, num14);
                  let mut num15: i32 =  game.EventRelatedObj.CheckHardcoded_SkillRoll(num14, 10, false, 0, true);
                  this.SL.AddWeight(num14, 1, 10, CheckData1Existence: true);
                  let mut num16: i32 =  DrawMod.RandyNumber.Next(0, randomizedDiff) + 70;
                  if (num15 < num16)
                  {
                    data23: String = game.Data.StringListObj[this.slotLeaders].GetData(0, num14, 3);
                    data24: String = game.Data.StringListObj[this.slotLeaders].GetData(0, num14, 4);
                    characterJobTitle: String = game.EventRelatedObj.Helper_GetCharacterJobTitle(num14);
                    if (str1.Length > 0)
                      str1 += " ";
                    str18: String = characterJobTitle + " " + data23 + " " + data24 + " was killed in action during the loss of the capital city (" + game.Data.LocObj[location].Name + ").";
                    str1 += str18;
                    game.EventRelatedObj.AddLeaderTempLog(num14, str18);
                    game.EventRelatedObj.Helper_AddMessageToUpcomingOrCurrentUDS(num1, "Leader KIA", str18, num14, DetailType.CharacterDeath);
                    game.EventRelatedObj.HelperResetAdvisorFor(num14, true, num1);
                    string[,] data25 = game.Data.StringListObj[this.slotLeaders].Data;
                    let mut index26: i32 =  row;
                    num3 = 0;
                    str19: String = num3.ToString();
                    data25[index26, 5] = str19;
                    string[,] data26 = game.Data.StringListObj[this.slotLeaders].Data;
                    let mut index27: i32 =  row;
                    num3 = 0;
                    str20: String = num3.ToString();
                    data26[index27, 6] = str20;
                    string[,] data27 = game.Data.StringListObj[this.slotLeaders].Data;
                    let mut index28: i32 =  row;
                    num3 = 0;
                    str21: String = num3.ToString();
                    data27[index28, 7] = str21;
                    game.Data.StringListObj[this.slotLeaders].Data[row, 25] = game.Data.Round.ToString();
                    game.Data.StringListObj[this.slotLeaders].Data[row, 26] = num1.ToString();
                    game.EventRelatedObj.Helper_RemoveAllTraceOfLeader(num14);
                  }
                }
              }
            }
          }
        }
      }
      if (str1.Length <= 0)
        return;
      if (Operators.CompareString(game.EditObj.CombatOneSentenceCustom, "", false) != 0)
        str1 += " ";
      game.EditObj.CombatOneSentenceCustom += str1;
      cc.AddBiggy(str1);
      if (hexLossCheck)
      {
        data: DataClass = game.Data;
        str22: String = "Zones".to_owned();
        ref local: String = ref str22;
        let mut libVar: i32 =  data.FindLibVar(ref local, "SE_Data");
        let mut hexLibVarValue: i32 =  game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].GetHexLibVarValue(libVar);
        texty: String = game.EventRelatedObj.CheckHexName(cc.TargetX, cc.TargetY, 0, 0) + " was conquered by " + game.Data.RegimeObj[cc.AttackerRegime].Name + ". " + str1;
        let mut instanceId: i32 =  hexLibVarValue;
        DrawMod.TGame.EventRelatedObj.Helper_AddDetailedReport(DetailType.Combat, instanceId, texty, game.Data.RegimeObj[cc.DefenderRegime].id);
      }
      else
      {
        if (unr <= -1)
          return;
        if (game.Data.UnitObj[unr].Regime == cc.DefenderRegime)
        {
          data: DataClass = game.Data;
          str23: String = "Zones".to_owned();
          ref local: String = ref str23;
          let mut libVar: i32 =  data.FindLibVar(ref local, "SE_Data");
          let mut hexLibVarValue: i32 =  game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].GetHexLibVarValue(libVar);
          DrawMod.TGame.EventRelatedObj.Helper_AddDetailedReport(DetailType.Combat, hexLibVarValue, str1, game.Data.RegimeObj[cc.DefenderRegime].id);
        }
        else
        {
          if (game.Data.UnitObj[unr].Regime != cc.AttackerRegime)
            return;
          data: DataClass = game.Data;
          str24: String = "Zones".to_owned();
          ref local: String = ref str24;
          let mut libVar: i32 =  data.FindLibVar(ref local, "SE_Data");
          let mut hexLibVarValue: i32 =  game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].GetHexLibVarValue(libVar);
          texty: String = "Attackers from " + game.Data.RegimeObj[cc.AttackerRegime].Name + " lost a Leader: " + str1;
          DrawMod.TGame.EventRelatedObj.Helper_AddDetailedReport(DetailType.Combat, hexLibVarValue, texty, game.Data.RegimeObj[cc.DefenderRegime].id);
        }
      }
    }

    pub fn PreCombatCall(ref cc: CombatClass, game: GameClass)
    {
      let mut ucounter1: i32 =  cc.UCounter;
      for (let mut index: i32 =  0; index <= ucounter1; index += 1)
      {
        let mut unr: i32 =  cc.UList[index].UNr;
        if (cc.UList[index].Uattacker == 0)
        {
          let mut historical: i32 =  game.Data.UnitObj[unr].Historical;
          if (game.Data.HistoricalUnitObj[historical].Type >= 8)
          {
            cc.UList[index].UCanRetreat.x = -1;
            cc.UList[index].UCanRetreat.y = -1;
            cc.UList[index].UCanRetreat.onmap = false;
          }
        }
      }
      if (this.slotUnitFeats <= -1)
        return;
      let mut highestValue: i32 =  game.Data.StringListObj[this.slotUnitFeats].GetHighestValue(0);
      let mut ucounter2: i32 =  cc.UCounter;
      for (let mut index1: i32 =  0; index1 <= ucounter2; index1 += 1)
      {
        let mut unr: i32 =  cc.UList[index1].UNr;
        let mut historical: i32 =  game.Data.UnitObj[unr].Historical;
        SimpleList simpleList1 = SimpleList::new();
        let mut num1: i32 =  100 + highestValue;
        num2: i32;
        for (let mut typ: i32 =  101; typ <= num1; typ += 1)
        {
          let mut tweight: i32 =  game.Data.HistoricalUnitObj[historical].GiveHisVarValue(typ);
          if (tweight > 0)
          {
            let mut tdata1: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotUnitFeats].GetData(0, typ - 100, 11)));
            let mut tdata2: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotUnitFeats].GetData(0, typ - 100, 13)));
            simpleList1.Add(typ - 100, tweight, tdata1, tdata2);
            if (tweight > num2)
              num2 = tweight;
          }
        }
        if (simpleList1.Counter > -1)
        {
          int[,,] numArray = new int[simpleList1.Counter + 1, num2 + 1, 100];
          SimpleList simpleList2 = SimpleList::new();
          let mut counter1: i32 =  simpleList1.Counter;
          for (let mut index2: i32 =  0; index2 <= counter1; index2 += 1)
          {
            let mut icounter: i32 =  cc.ICounter;
            for (let mut tid: i32 =  0; tid <= icounter; tid += 1)
            {
              if (Information.IsNothing( cc.IList[tid].IunitFeat))
                cc.IList[tid].IunitFeat = SimpleList::new();
              if (cc.IList[tid].IUnr == unr && simpleList1.Data1[index2] == -1 | simpleList1.Data1[index2] == game.Data.SFTypeObj[cc.IList[tid].ISFType].UnitGroup)
              {
                let mut num3: i32 =  game.Data.SFTypeObj[cc.IList[tid].ISFType].AttackPower[game.Data.SFTypeObj[cc.IList[tid].ISFType].UnitGroup];
                if (game.Data.SFTypeObj[cc.IList[tid].ISFType].ArtRange < 1 & game.Data.SFTypeObj[cc.IList[tid].ISFType].BackBench)
                  num3 = (int) Math.Round( num3 / 10.0);
                let mut num4: i32 =  0;
                let mut counter2: i32 =  cc.IList[tid].IunitFeat.Counter;
                for (let mut index3: i32 =  0; index3 <= counter2; index3 += 1)
                {
                  if (cc.IList[tid].IunitFeat.Data1[index3] == tid)
                    num4 = 1;
                }
                if (num4 == 0)
                  simpleList2.Add(tid, 10 * num3);
              }
            }
            let mut index4: i32 =  simpleList1.Weight[index2];
            let mut num5: i32 =  index4;
            for (let mut index5: i32 =  1; index5 <= num5; index5 += 1)
            {
              let mut randomIdbasedOnWeight: i32 =  simpleList2.GetRandomIdbasedOnWeight();
              if (randomIdbasedOnWeight > -1)
              {
                if (Information.IsNothing( cc.IList[randomIdbasedOnWeight].IunitFeat))
                  cc.IList[randomIdbasedOnWeight].IunitFeat = SimpleList::new();
                cc.IList[randomIdbasedOnWeight].IunitFeat.Add(simpleList1.Id[index2], 1, cc.IList[randomIdbasedOnWeight].IID);
                cc.IList[randomIdbasedOnWeight].IunitFeatStart = simpleList1.Id[index2];
                numArray[index2, index4, 1] = randomIdbasedOnWeight;
                simpleList2.Remove(randomIdbasedOnWeight);
              }
            }
          }
          SimpleList simpleList3 = SimpleList::new();
          let mut counter3: i32 =  simpleList1.Counter;
          for (let mut index6: i32 =  0; index6 <= counter3; index6 += 1)
          {
            let mut icounter: i32 =  cc.ICounter;
            for (let mut tid: i32 =  0; tid <= icounter; tid += 1)
            {
              if (Information.IsNothing( cc.IList[tid].IunitFeat))
                cc.IList[tid].IunitFeat = SimpleList::new();
              if (cc.IList[tid].IUnr == unr && simpleList1.Data1[index6] == -1 | simpleList1.Data1[index6] == game.Data.SFTypeObj[cc.IList[tid].ISFType].UnitGroup && cc.IList[tid].IunitFeat.FindNr(simpleList1.Id[index6]) == -1)
              {
                let mut num6: i32 =  game.Data.SFTypeObj[cc.IList[tid].ISFType].AttackPower[game.Data.SFTypeObj[cc.IList[tid].ISFType].UnitGroup];
                if (game.Data.SFTypeObj[cc.IList[tid].ISFType].ArtRange < 1 & game.Data.SFTypeObj[cc.IList[tid].ISFType].BackBench)
                  num6 = (int) Math.Round( num6 / 10.0);
                simpleList3.Add(tid, 10 * num6);
              }
            }
            let mut index7: i32 =  simpleList1.Weight[index6];
            let mut num7: i32 =  index7;
            for (let mut index8: i32 =  1; index8 <= num7; index8 += 1)
            {
              if (simpleList1.Data2[index6] >= 2)
              {
                let mut num8: i32 =  simpleList1.Data2[index6];
                for (let mut index9: i32 =  2; index9 <= num8; index9 += 1)
                {
                  let mut randomIdbasedOnWeight: i32 =  simpleList3.GetRandomIdbasedOnWeight();
                  if (randomIdbasedOnWeight > -1)
                  {
                    let mut index10: i32 =  numArray[index6, index7, 1];
                    if (Information.IsNothing( cc.IList[randomIdbasedOnWeight].IunitFeat))
                      cc.IList[randomIdbasedOnWeight].IunitFeat = SimpleList::new();
                    cc.IList[randomIdbasedOnWeight].IunitFeat.Add(simpleList1.Id[index6], 1, cc.IList[index10].IID);
                    simpleList3.Remove(randomIdbasedOnWeight);
                  }
                }
              }
            }
          }
        }
      }
    }

    pub fn DoStructuralDamageCall(ref cc: CombatClass, game: GameClass, damPts: i32)
    {
      let mut num1: i32 =  (int) Math.Round( cc.CombatRound / 3.0) + 1;
      s: String = "";
      data1: DataClass = game.Data;
      str1: String = "Zones".to_owned();
      ref local1: String = ref str1;
      let mut libVar1: i32 =  data1.FindLibVar(ref local1, "SE_Data");
      let mut hexLibVarValue1: i32 =  game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].GetHexLibVarValue(libVar1);
      if (damPts < 1)
        return;
      data2: DataClass = DrawMod.TGame.Data;
      str2: String = "bunkerPoints".to_owned();
      ref local2: String = ref str2;
      let mut libVar2: i32 =  data2.FindLibVar(ref local2, "SE_Data");
      let mut hexLibVarValue2: i32 =  game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].GetHexLibVarValue(libVar2);
      float num2 = 0.0f;
      let mut val2: i32 =  damPts;
      if (hexLibVarValue2 > 0 & libVar2 > -1)
      {
        let mut val1: i32 =  hexLibVarValue2;
        while (val1 > 0 & damPts > 0)
        {
          float num3 =  Math.Min(val1, val2) /  val2;
          if ( num3 > 1.0)
            num3 = 1f;
          let mut num4: i32 =  (int) Math.Round(Math.Ceiling( damPts * 0.5 *  num3));
          num2 += num3;
          val1 -= val2;
          if (0 > val1)
            val1 = 0;
          damPts -= num4 + 1;
          if (0 > damPts)
            damPts = 0;
        }
        s = s + hexLibVarValue2.ToString() + " Bunker Points halved the structural damage " + Math.Round( num2, 1).ToString() + "x times letting: " + damPts.ToString() + " structural damage through.";
      }
      if (damPts > 0)
      {
        if (cc.CombatType == 5)
        {
          data3: DataClass = game.Data;
          str3: String = "aiStrucDam".to_owned();
          ref local3: String = ref str3;
          let mut libVar3: i32 =  data3.FindLibVar(ref local3, "SE_Data");
          if (libVar3 > -1)
          {
            let mut tValue: i32 =  game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].GetHexLibVarValue(libVar3) + damPts;
            game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].SetHexLibVarValue(libVar3, tValue);
          }
        }
        SimpleList simpleList = SimpleList::new();
        let mut length: i32 =  game.Data.StringListObj[this.slotAssets].Length;
        num5: i32;
        for (let mut index: i32 =  0; index <= length; index += 1)
        {
          let mut num6: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[index, 3]));
          let mut num7: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[index, 4]));
          if (num6 == cc.TargetX & num7 == cc.TargetY)
          {
            let mut num8: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[index, 2]));
            let mut tdata2: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[index, 6]));
            num5 = (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[index, 2]));
            let mut num9: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[index, 9]));
            let mut idValue: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[index, 1]));
            let mut tdata3: i32 =  game.Data.StringListObj[this.slotAssetTypes].Width < 29 ? 0 : (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue, 29)));
            simpleList.Add(num9, num8 * 10 + 10, num9, tdata2, tdata3);
          }
        }
        if (simpleList.Counter > -1)
        {
          let mut num10: i32 =  (int) Math.Round( damPts /  Math.Max(1, simpleList.Counter)) + (int) Math.Round( damPts / 3.0);
          if (num10 < 1)
            num10 = 1;
          let mut num11: i32 =  0;
          let mut counter1: i32 =  simpleList.Counter;
          for (let mut index: i32 =  0; index <= counter1; index += 1)
          {
            let mut num12: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].GetData(9, simpleList.Data1[index], 6)));
            let mut num13: i32 =  num10;
            if (num13 > 100)
              num13 = 100 + (int) Math.Round( (num13 - 100) / 2.0);
            if (num13 > 200)
              num13 = 200 + (int) Math.Round( (num13 - 200) / 3.0);
            if (num13 > 300)
              num13 = 300 + (int) Math.Round( (num13 - 300) / 4.0);
            if (num13 > 400)
              num13 = 400 + (int) Math.Round( (num13 - 400) / 5.0);
            if (num13 > 500)
              num13 = 500 + (int) Math.Round( (num13 - 500) / 6.0);
            if (num13 > 600)
              num13 = 600 + (int) Math.Round( (num13 - 600) / 7.0);
            if (num13 > 700)
              num13 = 700;
            if (simpleList.Data3[index] > 0)
            {
              num13 = (int) Math.Round( num13 -  (num13 * simpleList.Data3[index]) / 100.0);
              if (num13 < 0)
                num13 = 0;
            }
            let mut setValue: i32 =  num12 + num13;
            num11 += num13;
            game.Data.StringListObj[this.slotAssets].SetData(9, simpleList.Data1[index], 6, setValue);
          }
          s = s + "Attack did " + num11.ToString() + " of damage to the Assets in the hex. ";
          let mut counter2: i32 =  simpleList.Counter;
          for (let mut index: i32 =  0; index <= counter2; index += 1)
          {
            let mut row1: i32 =  game.Data.StringListObj[this.slotAssets].FindRow(9, simpleList.Data1[index]);
            if (row1 > -1)
            {
              let mut idValue1: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[row1, 1]));
              let mut nr: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue1, 2)));
              let mut num14: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[row1, 6]));
              let mut idValue2: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[row1, 3]));
              let mut idValue2_1: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[row1, 4]));
              num5 = (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[row1, 2]));
              let mut tval: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].Data[row1, 9]));
              let mut integer: i32 =  Conversions.ToInteger(game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue1, 14));
              str4: String = game.Data.StringListObj[this.slotAssetTypes].GetData(0, idValue1, 1);
              if (nr > 0)
                str4 = str4 + " " + game.HandyFunctionsObj.GetRomanNumerical(nr);
              let mut num15: i32 =  0;
              if (num14 > 200 * nr)
              {
                let mut num16: i32 =  (int) Math.Round(Math.Ceiling( (num14 - 200 * nr) / 10.0));
                if (DrawMod.RandyNumber.Next(0, 100) < num16)
                {
                  num15 = 1;
                  s = s + str4 + " was completely obliterated. ";
                  let mut row2: i32 =  game.Data.StringListObj[this.slotAssets].FindRow(9, tval);
                  game.Data.StringListObj[this.slotAssets].RemoveRow(row2);
                  if (nr >= 1)
                  {
                    let mut idValue2_2: i32 =  nr + 1;
                    let mut idValue3: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssetTypes].GetData2(14, integer, 2, idValue2_2, 0)));
                    if (idValue3 > 0)
                    {
                      let mut num17: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].GetData3(3, idValue2, 4, idValue2_1, 1, idValue3, 9)));
                      if (num17 > 0 && (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotAssets].GetData(9, num17, 8))) >= 1)
                      {
                        let mut row3: i32 =  game.Data.StringListObj[this.slotAssets].FindRow(9, num17);
                        game.Data.StringListObj[this.slotAssets].RemoveRow(row3);
                      }
                    }
                  }
                }
              }
              if (num15 == 0)
              {
                let mut num18: i32 =  num14 - simpleList.Data2[index];
                s = s + str4 + " survived the attack but suffered " + num18.ToString() + " damage points. ";
              }
            }
          }
        }
      }
      num19: i32;
      num20: i32;
      num21: i32;
      if (this.rememberRadPts > 0)
      {
        if (hexLibVarValue1 > 0)
        {
          HelperEconomyData hed = new HelperEconomyData(ref game, "SE_Data")
          {
            zoneId = hexLibVarValue1
          };
          hed.zoneName = game.Data.StringListObj[hed.slotZones].GetData(0, hexLibVarValue1, 7);
          hed.locId = (int) Math.Round(Conversion.Val(game.Data.StringListObj[hed.slotZones].GetData(0, hexLibVarValue1, 6)));
          hed.locNr = -1;
          if (hed.locId > 0)
            hed.locNr = game.HandyFunctionsObj.GetLocationByID(hed.locId);
          hed.origRegimeId = (int) Math.Round(Conversion.Val(game.Data.StringListObj[hed.slotZones].GetData(0, hexLibVarValue1, 8)));
          hed.regid = hed.origRegimeId;
          hed.origRegimeNr = -1;
          hed.Input(hed.origRegimeId);
          let mut pop1: i32 =  hed.pop;
          let mut worker1: i32 =  hed.worker;
          let mut freeFolk1: i32 =  hed.freeFolk;
          game.EventRelatedObj.ZoneEconomy_ExposureEffects(ref hed, "SE_Data", cc.TargetX, cc.TargetY, this.rememberRadPts * 3);
          let mut pop2: i32 =  hed.pop;
          let mut worker2: i32 =  hed.worker;
          let mut freeFolk2: i32 =  hed.freeFolk;
          num22: i32;
          if (pop1 > pop2)
            num22 = pop1 - pop2;
          num23: i32;
          if (worker1 > worker2)
            num23 = worker1 - worker2;
          num24: i32;
          if (freeFolk1 > freeFolk2)
            num24 = freeFolk1 - freeFolk2;
          hed.Output(hed.origRegimeId);
          num19 = num22 * 100;
          num20 = num23 * 100;
          num21 = num24 * 100;
        }
        if (num19 > 0 | num20 > 0 | num21 > 0)
          s = num19.ToString() + " Population and " + num20.ToString() + " Workers and " + num21.ToString() + " Free Folk lost in strike. " + s;
        s = "Attack by " + game.Data.RegimeObj[cc.AttackerRegime].Name + " made use of Atomic weapons. " + this.rememberRadPts.ToString() + " of RAD points has been added to Hex. " + s;
      }
      if (s.Length <= 0)
        return;
      game.EditObj.CombatOneSentenceCustom += s;
      cc.AddBiggy(s);
      str5: String = s.Replace(". ", ".\r\n");
      texty1: String = game.EventRelatedObj.CheckHexName(cc.TargetX, cc.TargetY, 0, 0) + "(" + cc.TargetX.ToString() + "," + cc.TargetY.ToString() + ") was attacked.\r\n" + str5;
      DrawMod.TGame.EventRelatedObj.Helper_AddDetailedReport(DetailType.Combat, hexLibVarValue1, texty1, game.Data.RegimeObj[cc.DefenderRegime].id);
      if (!(this.rememberRadPts >= 200 & !game.Data.RegimeObj[cc.DefenderRegime].AI & hexLibVarValue1 > 0))
        return;
      texty2: String = game.EventRelatedObj.CheckHexName(cc.TargetX, cc.TargetY, 0, 0) + "(" + cc.TargetX.ToString() + "," + cc.TargetY.ToString() + ") was attacked.\r\nAttack by " + game.Data.RegimeObj[cc.AttackerRegime].Name + " made use of Atomic weapons. " + this.rememberRadPts.ToString() + " of RAD points has been added to Hex. ";
      if (num19 > 0 | num20 > 0 | num21 > 0)
        texty2 = texty2 + num19.ToString() + " Population and " + num20.ToString() + " Workers and " + num21.ToString() + " Free Folk lost in strike. ";
      game.EventRelatedObj.Helper_AddDetailedReportPlusData(DetailType.Nuke, hexLibVarValue1, texty2, game.Data.RegimeObj[cc.DefenderRegime].id, -1);
    }

    pub fn EndCombatCall(ref cc: CombatClass, game: GameClass)
    {
      let mut num1: i32 =  (int) Math.Round( cc.CombatRound / 3.0) + 1;
      SimpleList simpleList1 = SimpleList::new();
      let mut ucounter: i32 =  cc.UCounter;
      for (let mut index: i32 =  0; index <= ucounter; index += 1)
      {
        let mut unr: i32 =  cc.UList[index].UNr;
        if (!Information.IsNothing( game.Data.UnitObj[unr].tempSFTypeBitmap))
        {
          game.Data.UnitObj[unr].tempSFTypeBitmap.Dispose();
          game.Data.UnitObj[unr].tempSFTypeBitmap = (Bitmap) null;
        }
      }
      let mut num2: i32 =  0;
      let mut counter1: i32 =  this.SL.Counter;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
      {
        if (this.SL.Weight[index] > 0)
          this.SL.Weight[index] = (int) Math.Round(Math.Sqrt(Math.Sqrt( this.SL.Weight[index])));
        simpleList1.AddWeight(this.SL.Id[index], DrawMod.RandyNumber.Next(1, 100));
        num2 += 1;
      }
      simpleList1.ReverseSortHighSpeed();
      if (this.SL.Counter > -1)
      {
        let mut num3: i32 =  num1;
        for (let mut index: i32 =  1; index <= num3; index += 1)
        {
          let mut nr: i32 =  this.SL.FindNr(this.SL.GetRandomIdbasedOnWeight());
          let mut idValue: i32 =  this.SL.Id[nr];
          let mut idValue2: i32 =  this.SL.Data1[nr];
          let mut setValue: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotCharSkill].GetData2(0, idValue, 1, idValue2, 4))) + 1;
          game.Data.StringListObj[this.slotCharSkill].SetData2(0, idValue, 1, idValue2, 4, setValue, true);
        }
      }
      let mut num4: i32 =  -1;
      SimpleList simpleList2 = SimpleList::new();
      let mut counter2: i32 =  simpleList1.Counter;
      for (let mut index1: i32 =  0; index1 <= counter2; index1 += 1)
      {
        let mut num5: i32 =  simpleList1.Id[index1];
        let mut num6: i32 =  simpleList1.Weight[index1];
        let mut id: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData(0, num5, 5)));
        let mut regimeById: i32 =  game.HandyFunctionsObj.GetRegimeByID(id);
        let mut num7: i32 =  0;
        num8: i32;
        if (cc.AttackerRegime == regimeById)
        {
          if (simpleList2.FindNr(num5) == -1)
          {
            if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData(0, num5, 6))) == 3)
              simpleList2.Add(num5, 100);
            else
              simpleList2.Add(num5, 10);
          }
          num8 = cc.BattleEnded != 1 ? num7 + 10 : num7 + 20;
        }
        else
          num8 = cc.BattleEnded != 2 ? num7 + 10 : num7 + 20;
        let mut num9: i32 =  0;
        let mut combatRound: i32 =  cc.CombatRound;
        for (let mut index2: i32 =  1; index2 <= combatRound; index2 += 1)
        {
          if (index2 <= 10)
            num9 += (int) Math.Round(Math.Ceiling( index2 / 3.0));
        }
        let mut xpGain: i32 =  num8 + num9;
        if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData(0, num5, 6))) == 4)
          xpGain = (int) Math.Round( xpGain / 3.0);
        let mut num10: i32 =  game.EventRelatedObj.Helper_ModifyXPGain(xpGain, num5);
        let mut setValue: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData(0, num5, 17))) + num10;
        game.EventRelatedObj.AddLeaderTempLog(num5, "Gained " + num10.ToString() + " XP during combat in " + game.HandyFunctionsObj.GetHexName(cc.TargetX, cc.TargetY, 0));
        game.Data.StringListObj[this.slotChar].SetData(0, num5, 17, setValue);
      }
      if (cc.BattleEnded == 1)
        this.PersonalCombatRoll(ref cc, game, -1, -1, 0, "", false, true);
      if (simpleList2.Counter > -1)
        num4 = simpleList2.GetRandomIdbasedOnWeight();
      let mut num11: i32 =  1;
      if (num4 > -1)
        num11 = (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData(0, num4, 15)));
      let mut num12: i32 =  new Random(num4).Next(1, 5);
      if (cc.dontUseSfx)
        return;
      if (cc.CombatType == 3)
      {
        if (num4 <= -1)
          return;
        let mut num13: i32 =  new Random(num4).Next(1, 21);
        Soundfile: String;
        if (num11 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num12.ToString() + "/acknowledged" + num13.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/acknowledged" + num13.ToString() + ".wav";
        if (!(!game.AIRunning & !game.Data.RegimeObj[game.Data.Turn].AI))
          return;
        SoundMod.PlayAWave(Soundfile, ref game.EditObj);
      }
      else if (cc.CombatType == 13 | cc.CombatType == 5)
      {
        if (num4 <= -1)
          return;
        let mut num14: i32 =  0;
        let mut num15: i32 =  0;
        let mut icounter: i32 =  cc.ICounter;
        for (let mut index: i32 =  0; index <= icounter; index += 1)
        {
          if (cc.IList[index].IAttacker == 1 && game.Data.SFTypeObj[cc.IList[index].ISFType].Theater == 2)
          {
            if (cc.IList[index].IKilled > 0)
              num15 += 1;
            else
              num14 += 1;
          }
        }
        Soundfile: String;
        if (num15 >= num14)
        {
          let mut num16: i32 =  new Random(num4).Next(1, 3);
          Soundfile = game.AppPath + "sound/air/voiceovers/mayday" + num16.ToString() + ".wav";
        }
        else
        {
          let mut num17: i32 =  new Random(num4).Next(1, 3);
          Soundfile = game.AppPath + "sound/air/voiceovers/endofmission" + num17.ToString() + ".wav";
        }
        if (!(!game.AIRunning & !game.Data.RegimeObj[game.Data.Turn].AI))
          return;
        SoundMod.PlayAWave(Soundfile, ref game.EditObj);
      }
      else if (cc.BattleEnded == 1)
      {
        this.PersonalCombatRoll(ref cc, game, -1, -1, 0, "", false, true);
        if (num4 <= -1)
          return;
        let mut num18: i32 =  new Random(num4).Next(1, 11);
        Soundfile: String;
        if (num11 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num12.ToString() + "/battlewon" + num18.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/battlewon" + num18.ToString() + ".wav";
        if (!(!game.AIRunning & !game.Data.RegimeObj[game.Data.Turn].AI))
          return;
        SoundMod.PlayAWave(Soundfile, ref game.EditObj);
      }
      else
      {
        if (num4 <= -1)
          return;
        let mut num19: i32 =  new Random(num4).Next(1, 7);
        Soundfile: String;
        if (num11 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num12.ToString() + "/battlelost" + num19.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/battlelost" + num19.ToString() + ".wav";
        if (!(!game.AIRunning & !game.Data.RegimeObj[game.Data.Turn].AI))
          return;
        SoundMod.PlayAWave(Soundfile, ref game.EditObj);
      }
    }

    pub fn EndBattleCall(ref cc: CombatClass, game: GameClass)
    {
      let mut index1: i32 =  (int) Math.Round( game.Data.RuleVar[407]) + 5;
      let mut index2: i32 =  (int) Math.Round( game.Data.RuleVar[407]) + 9;
      let mut index3: i32 =  (int) Math.Round( game.Data.RuleVar[407]) + 0;
      let mut index4: i32 =  (int) Math.Round( game.Data.RuleVar[407]) + 7;
      let mut index5: i32 =  (int) Math.Round( game.Data.RuleVar[407]) + 2;
      let mut index6: i32 =  (int) Math.Round( game.Data.RuleVar[407]) + 8;
      let mut ucounter1: i32 =  cc.UCounter;
      for (let mut index7: i32 =  0; index7 <= ucounter1; index7 += 1)
      {
        if (cc.UList[index7].UDead == 0)
        {
          let mut unr: i32 =  cc.UList[index7].UNr;
          SimpleList simpleList = SimpleList::new();
          let mut sfCount: i32 =  game.Data.UnitObj[unr].SFCount;
          for (let mut index8: i32 =  0; index8 <= sfCount; index8 += 1)
          {
            let mut sf: i32 =  game.Data.UnitObj[unr].SFList[index8];
            let mut type: i32 =  game.Data.SFObj[sf].Type;
            if (game.Data.SFTypeObj[type].SFTypeVar[index2] > 0)
              simpleList.AddWeight(game.Data.SFTypeObj[type].SFTypeVar[index1], game.Data.SFTypeObj[type].SFTypeVar[index2] * game.Data.SFObj[sf].Qty);
            if (game.Data.SFTypeObj[type].SFTypeVar[index4] > 0)
              simpleList.AddWeight(game.Data.SFTypeObj[type].SFTypeVar[index3], game.Data.SFTypeObj[type].SFTypeVar[index4] * game.Data.SFObj[sf].Qty);
            if (game.Data.SFTypeObj[type].SFTypeVar[index6] > 0)
              simpleList.AddWeight(game.Data.SFTypeObj[type].SFTypeVar[index5], game.Data.SFTypeObj[type].SFTypeVar[index6] * game.Data.SFObj[sf].Qty);
          }
          if (game.Data.UnitObj[unr].Historical > -1 && game.Data.HistoricalUnitObj[game.Data.UnitObj[unr].Historical].Type < 8)
          {
            let mut counter: i32 =  game.Data.UnitObj[unr].items.list.Counter;
            for (let mut index9: i32 =  0; index9 <= counter; index9 += 1)
            {
              let mut weight: i32 =  simpleList.FindWeight(game.Data.UnitObj[unr].items.list.Id[index9]);
              if (game.Data.UnitObj[unr].items.list.Weight[index9] > weight)
                game.Data.UnitObj[unr].items.list.Weight[index9] = weight;
            }
          }
        }
      }
      if (this.slotUnitFeats > -1)
      {
        game.Data.StringListObj[this.slotUnitFeats].GetHighestValue(0);
        let mut ucounter2: i32 =  cc.UCounter;
        for (let mut index10: i32 =  0; index10 <= ucounter2; index10 += 1)
        {
          let mut unr: i32 =  cc.UList[index10].UNr;
          let mut historical: i32 =  game.Data.UnitObj[unr].Historical;
          let mut icounter: i32 =  cc.ICounter;
          for (let mut index11: i32 =  0; index11 <= icounter; index11 += 1)
          {
            if (cc.IList[index11].IUnr == unr && !Information.IsNothing( cc.IList[index11].IunitFeat) && cc.IList[index11].IunitFeatStart > 0 & cc.IList[index11].IunitFeat.Counter > -1 && cc.IList[index11].IunitFeat.Id[0] < 1 | cc.IList[index11].IunitFeatDeadRound > 0 | cc.IList[index11].ICapitulate | cc.IList[index11].IKilled > 0)
            {
              let mut num: i32 =  game.Data.HistoricalUnitObj[historical].GiveHisVarValue(100 + cc.IList[index11].IunitFeatStart) - 1;
              game.Data.HistoricalUnitObj[historical].SetHisVarValue(100 + cc.IList[index11].IunitFeatStart, num);
            }
          }
        }
      }
      let mut ucounter3: i32 =  cc.UCounter;
      for (let mut index12: i32 =  0; index12 <= ucounter3; index12 += 1)
        game.EventRelatedObj.Helper_RemoveExcessUnitFeats(cc.UList[index12].UNr);
      let mut counter1: i32 =  this.MODL.Counter;
      for (let mut index13: i32 =  0; index13 <= counter1; index13 += 1)
      {
        let mut idValue: i32 =  this.MODL.Id[index13];
        if (idValue > 0)
        {
          let mut num1: i32 =  (int) Math.Round(Math.Sqrt( this.MODL.Weight[index13]));
          if (num1 > 10)
            num1 = 10;
          if (num1 > 0)
          {
            let mut setValue1: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotModelsClean].GetData2(0, idValue, 1, 7, 2)));
            let mut setValue2: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotModelsAdditive].GetData2(0, idValue, 1, 7, 2)));
            let mut num2: i32 =  120 + (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotModels].GetData(0, idValue, 4))) * 10;
            let mut num3: i32 =  num1;
            for (let mut index14: i32 =  1; index14 <= num3; index14 += 1)
            {
              let mut num4: i32 =  num2 - setValue1;
              if (num4 > 40)
              {
                setValue1 += 1;
                setValue2 += 1;
              }
              else if (num4 > 0 && DrawMod.RandyNumber.Next(0, 40) < num4 && DrawMod.RandyNumber.Next(0, 40) < num4)
              {
                setValue1 += 1;
                setValue2 += 1;
              }
            }
            game.Data.StringListObj[this.slotModelsClean].SetData2(0, idValue, 1, 7, 2, setValue1);
            game.Data.StringListObj[this.slotModelsAdditive].SetData2(0, idValue, 1, 7, 2, setValue2);
          }
        }
      }
    }

    pub fn StartCombatRound(ref cc: CombatClass, game: GameClass, combatRound: i32)
    {
      let mut num1: i32 =  -1;
      let mut jobSpecificId1: i32 =  -1;
      let mut jobSpecificId2: i32 =  -1;
      let mut num2: i32 =  -1;
      SimpleList simpleList1 = SimpleList::new();
      let mut icounter1: i32 =  cc.ICounter;
      num3: i32;
      num4: i32;
      num5: i32;
      num6: i32;
      num7: i32;
      num8: i32;
      num9: i32;
      for (let mut index1: i32 =  0; index1 <= icounter1; index1 += 1)
      {
        let mut index2: i32 =  cc.IList[index1].IUnr;
        bool flag = false;
        if (cc.IList[index1].IRetreat > 0)
          flag = true;
        if (cc.IList[index1].IKilled > 0)
          flag = true;
        if (cc.IList[index1].IRetreated > 0)
          flag = true;
        if (!flag)
        {
          let mut historical1: i32 =  game.Data.UnitObj[index2].Historical;
          let mut id: i32 =  game.Data.RegimeObj[game.Data.UnitObj[cc.IList[index1].IUnr].Regime].id;
          if (historical1 > -1)
            num1 = game.Data.HistoricalUnitObj[historical1].ID;
          for (; index2 > -1; index2 = game.Data.UnitObj[index2].HQ)
          {
            let mut historical2: i32 =  game.Data.UnitObj[index2].Historical;
            if (game.Data.UnitObj[index2].IsHQ)
            {
              if (game.Data.HistoricalUnitObj[historical2].Type == 8)
                jobSpecificId2 = game.Data.HistoricalUnitObj[historical2].ID;
              else if (game.Data.HistoricalUnitObj[historical2].Type >= 5)
                jobSpecificId1 = game.Data.HistoricalUnitObj[historical2].ID;
            }
          }
          let mut characterId1: i32 =  game.EventRelatedObj.Helper_GetCharacterId(id, 3, jobSpecificId1, -1, this.slotChar);
          let mut characterId2: i32 =  game.EventRelatedObj.Helper_GetCharacterId(id, 4, jobSpecificId2, -1, this.slotChar);
          if (id == game.Data.RegimeObj[game.Data.Turn].id)
          {
            num3 += (int) Math.Round( (cc.IList[index1].ILisFuelMod * 100f));
            num4 += 1;
            num5 += (int) Math.Round( (cc.IList[index1].ILisAmmoMod * 100f));
            num6 += 1;
            num7 += 1;
            if (game.Data.SFTypeObj[cc.IList[index1].ISFType].UnitGroup == 3)
              num8 += 1;
            if (game.Data.SFTypeObj[cc.IList[index1].ISFType].UnitGroup == 2)
              num9 += 1;
            if (characterId1 > -1 & simpleList1.FindNr(characterId1) == -1)
              simpleList1.Add(characterId1, 100);
            if (characterId2 > -1 & simpleList1.FindNr(characterId2) == -1)
              simpleList1.Add(characterId2, 10);
          }
          if (characterId1 > -1)
          {
            let mut num10: i32 =  this.CachedSkillRoll(game, characterId1, 11);
            if (game.HandyFunctionsObj.Gethqpow(cc.IList[index1].IUnr) > DrawMod.RandyNumber.Next(0, 100))
            {
              this.SL.AddWeight(characterId1, 1, 11, CheckData1Existence: true);
              let mut mor: i32 =  game.Data.SFObj[cc.IList[index1].ISFNr].Mor;
              if (cc.IList[index1].IMor < mor)
              {
                if (num10 > 200)
                {
                  Individual[] ilist = cc.IList;
                  Individual[] individualArray = ilist;
                  let mut index3: i32 =  index1;
                  let mut index4: i32 =  index3;
                  individualArray[index4].IMor = ilist[index3].IMor + 4;
                }
                else if (num10 > 150)
                {
                  Individual[] ilist = cc.IList;
                  Individual[] individualArray = ilist;
                  let mut index5: i32 =  index1;
                  let mut index6: i32 =  index5;
                  individualArray[index6].IMor = ilist[index5].IMor + 2;
                }
                else if (num10 > 100)
                {
                  Individual[] ilist = cc.IList;
                  Individual[] individualArray = ilist;
                  let mut index7: i32 =  index1;
                  let mut index8: i32 =  index7;
                  individualArray[index8].IMor = ilist[index7].IMor + 1;
                }
                if (cc.IList[index1].IMor > mor)
                  cc.IList[index1].IMor = mor;
              }
            }
          }
        }
      }
      let mut icounter2: i32 =  cc.ICounter;
      index9: i32;
      for (let mut index10: i32 =  0; index10 <= icounter2; index10 += 1)
      {
        if (Information.IsNothing( cc.IList[index10].IunitFeat))
          cc.IList[index10].IunitFeat = SimpleList::new();
        txt: String = "";
        let mut counter: i32 =  cc.IList[index10].IunitFeat.Counter;
        for (let mut index11: i32 =  0; index11 <= counter; index11 += 1)
        {
          if (cc.IList[index10].IunitFeat.Id[index11] > 0)
          {
            data: String;
            if (cc.IList[cc.FindISlot(cc.IList[index10].IunitFeat.Data1[index11])].IKilled == 0 & cc.IList[cc.FindISlot(cc.IList[index10].IunitFeat.Data1[index11])].IRetreated == 0 & cc.IList[cc.FindISlot(cc.IList[index10].IunitFeat.Data1[index11])].IRetreat == 0)
            {
              let mut idValue: i32 =  cc.IList[index10].IunitFeat.Id[index11];
              data = game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue, 2);
              let mut length: i32 =  game.Data.StringListObj[this.slotBehaviour].Length;
              for (let mut index12: i32 =  0; index12 <= length; index12 += 1)
              {
                let mut num11: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 0]));
                if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 1])) == idValue)
                {
                  let mut num12: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 3]));
                  let mut num13: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 4]));
                  let mut num14: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 5]));
                  let mut num15: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 6]));
                  let mut num16: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 7]));
                  index13: i32;
                  str1: String = game.Data.StringListObj[this.slotBehaviour].Data[index12, index13];
                  let mut val2: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 8]));
                  let mut num17: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index12, 9]));
                  bool flag1 = false;
                  if (cc.CombatType == 1 & num12 == 1)
                    flag1 = true;
                  if (cc.CombatType == 3 & num13 == 1)
                    flag1 = true;
                  if (num13 == 1 & cc.game.Data.SFTypeObj[cc.IList[index10].ISFType].ArtRange > 0)
                    flag1 = true;
                  if (num14 != 11)
                    flag1 = false;
                  bool flag2 = false;
                  if (cc.IList[index10].IRetreat > 0)
                    flag2 = true;
                  if (cc.IList[index10].IKilled > 0)
                    flag2 = true;
                  if (cc.IList[index10].IRetreated > 0)
                    flag2 = true;
                  if (flag2)
                    flag1 = false;
                  if (flag1)
                  {
                    if (num11 == 17 && cc.IList[index10].IAttacker != 1)
                    {
                      index9 = game.Data.SFObj[cc.IList[index10].ISFNr].CurrentEntrench;
                      let mut ientrench: i32 =  cc.IList[index10].IEntrench;
                      let mut val1: i32 =  index9 - ientrench;
                      if (val1 > 0)
                      {
                        let mut num18: i32 =  Math.Min(val1, val2);
                        txt = data + " restored " + num18.ToString() + " entrenchment points.";
                        Individual[] ilist = cc.IList;
                        Individual[] individualArray = ilist;
                        let mut index14: i32 =  index10;
                        let mut index15: i32 =  index14;
                        individualArray[index15].IEntrench = ilist[index14].IEntrench + num18;
                      }
                    }
                    if (num11 == 26 && cc.IList[index10].IBreakTrough < 1)
                    {
                      str2: String = data + " gives " + num16.ToString() + "% chance on a breakthrough. ";
                      if (DrawMod.RandyNumber.Next(0, 100) < num16)
                      {
                        txt = str2 + "Succes!";
                        cc.IList[index10].IBreakTrough = combatRound;
                      }
                      else
                        txt = str2 + "Failed.";
                    }
                    if (num11 == 22)
                    {
                      index9 = game.Data.SFObj[cc.IList[index10].ISFNr].Mor;
                      let mut imor: i32 =  cc.IList[index10].IMor;
                      let mut val1: i32 =  index9 - imor;
                      if (val1 > 0)
                      {
                        let mut num19: i32 =  Math.Min(val1, val2);
                        txt = data + " restored " + num19.ToString() + " morale points.";
                        Individual[] ilist = cc.IList;
                        Individual[] individualArray = ilist;
                        let mut index16: i32 =  index10;
                        let mut index17: i32 =  index16;
                        individualArray[index17].IMor = ilist[index16].IMor + num19;
                      }
                    }
                    if (num11 == 5)
                    {
                      index9 = game.Data.SFObj[cc.IList[index10].ISFNr].Rdn;
                      let mut irdn: i32 =  cc.IList[index10].IRdn;
                      let mut val1: i32 =  index9 - irdn;
                      if (val1 > 0)
                      {
                        let mut num20: i32 =  Math.Min(val1, val2);
                        txt = data + " restored " + num20.ToString() + " readiness points.";
                        Individual[] ilist = cc.IList;
                        Individual[] individualArray = ilist;
                        let mut index18: i32 =  index10;
                        let mut index19: i32 =  index18;
                        individualArray[index19].IRdn = ilist[index18].IRdn + num20;
                      }
                    }
                    if (num11 == 29 & cc.UList[cc.IList[index10].IUlistNr].URetreat < 1)
                    {
                      SimpleList simpleList2 = SimpleList::new();
                      SimpleList simpleList3 = SimpleList::new();
                      let mut icounter3: i32 =  cc.ICounter;
                      for (let mut tid: i32 =  0; tid <= icounter3; tid += 1)
                      {
                        if (cc.IList[tid].IUnr == cc.IList[index10].IUnr & index10 != tid)
                        {
                          if (cc.IList[tid].IRetreated < 1 & cc.IList[tid].IRetreat > 0)
                          {
                            if (cc.IList[tid].IKilled < 1)
                              simpleList2.Add(tid, 10);
                          }
                          else if (cc.IList[tid].IRetreat < 1 & cc.IList[tid].IKilled < 1)
                            simpleList3.Add(tid, 100 - cc.IList[tid].IMor);
                        }
                      }
                      if (simpleList2.Counter > -1 & simpleList3.Counter > -1)
                      {
                        index9 = simpleList2.GetRandomIdbasedOnWeight();
                        txt = data + " killed our own #" + index9.ToString() + " individual to instill discipline and morale on troops still fighting.";
                        cc.IList[index9].IKilled = cc.CombatRound;
                        let mut num21: i32 =  num17;
                        for (let mut index20: i32 =  1; index20 <= num21; index20 += 1)
                        {
                          if (simpleList3.Counter > -1)
                          {
                            let mut randomIdbasedOnWeight: i32 =  simpleList3.GetRandomIdbasedOnWeight();
                            simpleList3.Remove(randomIdbasedOnWeight);
                            if (randomIdbasedOnWeight > -1)
                            {
                              Individual[] ilist = cc.IList;
                              Individual[] individualArray = ilist;
                              let mut index21: i32 =  randomIdbasedOnWeight;
                              let mut index22: i32 =  index21;
                              individualArray[index22].IMor = ilist[index21].IMor + val2;
                              if (100 < cc.IList[randomIdbasedOnWeight].IMor)
                                cc.IList[randomIdbasedOnWeight].IMor = 100;
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
            if (txt.Length > 0)
              cc.AddReport(0, data + " effect", txt, 10000 + index10, cc.CombatRound);
          }
        }
      }
      if (cc.CombatRound != 1 || !(!game.AIRunning & !game.Data.RegimeObj[game.Data.Turn].AI & !cc.dontUseSfx))
        return;
      if (simpleList1.Counter > -1)
        num2 = simpleList1.GetRandomIdbasedOnWeight();
      if (num2 <= -1)
        return;
      bool flag3;
      if (cc.CombatType == 3)
        flag3 = true;
      if (DrawMod.RandyNumber.Next(1, num7 + 1) <= (int) Math.Round( num9 / 2.0))
        flag3 = true;
      bool flag4;
      if (DrawMod.RandyNumber.Next(1, num7 + 1) <= (int) Math.Round( num8 / 2.0))
        flag4 = true;
      let mut num22: i32 =  (int) Math.Round( num5 /  num6);
      let mut num23: i32 =  (int) Math.Round( num3 /  num4);
      bool flag5;
      if (num22 < DrawMod.RandyNumber.Next(50, 85))
        flag5 = true;
      bool flag6;
      if (num23 < DrawMod.RandyNumber.Next(50, 85))
        flag6 = true;
      if (DrawMod.RandyNumber.Next(0, 100) < 50)
      {
        flag5 = false;
        flag6 = false;
      }
      bool flag7;
      if ( cc.ConcentricBonus >  DrawMod.RandyNumber.Next(40, 120))
        flag7 = true;
      if (cc.CombatType == 1)
        ;
      bool flag8;
      if (cc.CombatType == 11)
      {
        flag8 = true;
        flag6 = false;
        flag5 = false;
      }
      if (cc.CombatType == 3)
        flag6 = false;
      if (flag3 & flag4)
      {
        if (cc.CombatType == 3)
          flag4 = false;
        else
          flag3 = false;
      }
      bool flag9;
      if (cc.CombatType == 13 & cc.CombatType2 != 16)
        flag9 = true;
      bool flag10;
      if (cc.CombatType == 5)
        flag10 = true;
      Random random = new Random(num2);
      let mut num24: i32 =  1;
      if (num2 > -1)
        num24 = (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData(0, num2, 15)));
      let mut num25: i32 =  new Random(num2).Next(1, 5);
      Soundfile: String;
      if (flag9)
        Soundfile = game.AppPath + "sound/air/voiceovers/airrecon.wav";
      else if (flag10)
        Soundfile = game.AppPath + "sound/air/voiceovers/airstrike.wav";
      else if (flag7)
      {
        index9 = random.Next(1, 3);
        if (num24 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num25.ToString() + "/concentric" + index9.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/concentric" + index9.ToString() + ".wav";
      }
      else if (flag5)
      {
        index9 = 1;
        if (num24 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num25.ToString() + "/ammo" + index9.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/ammo" + index9.ToString() + ".wav";
      }
      else if (flag6)
      {
        index9 = random.Next(1, 3);
        if (num24 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num25.ToString() + "/fuel" + index9.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/fuel" + index9.ToString() + ".wav";
      }
      else if (flag8)
      {
        index9 = random.Next(1, 8);
        if (num24 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num25.ToString() + "/surprise" + index9.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/surprise" + index9.ToString() + ".wav";
      }
      else if (flag3)
      {
        index9 = random.Next(1, 7);
        if (num24 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num25.ToString() + "/fire" + index9.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/fire" + index9.ToString() + ".wav";
      }
      else if (flag4)
      {
        index9 = random.Next(1, 5);
        if (num24 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num25.ToString() + "/tank" + index9.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/tank" + index9.ToString() + ".wav";
      }
      else
      {
        index9 = random.Next(1, 13);
        if (num24 == 2)
          Soundfile = game.AppPath + "sound/voiceovers_female" + num25.ToString() + "/general" + index9.ToString() + ".ogg";
        else
          Soundfile = game.AppPath + "sound/voiceovers/general" + index9.ToString() + ".wav";
      }
      if (Soundfile.Length <= 0)
        return;
      SoundMod.PlayAWave(Soundfile, ref game.EditObj);
    }

    pub fn NumberOfMods() -> i32 => this.airEnabled ? 22 : 15;

    pub GetModName: String(nr: i32)
    {
      switch (nr)
      {
        case 1:
          return "Skill Tank Tactics";
        case 2:
          return "Skill Gun Tactics";
        case 3:
          return "Skill Infantry Tactics";
        case 4:
          return "Skill Escape Artist";
        case 5:
          return "Skill Demolitions";
        case 6:
          return "Skill Offensive Tactics";
        case 7:
          return "Skill Defensive Tactics";
        case 8:
          return "Skill Operational Command";
        case 9:
          return "Skill High Command";
        case 10:
          return "Posture Combat Bonus";
        case 11:
          return "Profile Combat Bonus";
        case 12:
          return "Unit Feat Mod";
        case 13:
          return "Skill Deception";
        case 14:
          return "Weapon Matrix Mod";
        case 15:
          return "Callibre Matrix Mod";
        case 16:
          return "Air Combat Range";
        case 17:
          return "Dogfight Modifier";
        case 18:
          return "Skill Air Tactics";
        case 19:
          return "Skill Air Intercepts";
        case 20:
          return "Skill Air Offensive";
        case 21:
          return "Skill Ground Attack";
        case 22:
          return "Skill Anti-Air Tactics";
        default:
          modName: String;
          return modName;
      }
    }

    pub IndividualCombatCall_HasNoEarlyCombatRoundPenalties: i32(
      ref cc: CombatClass,
      game: GameClass,
      attnr: i32,
      defnr: i32,
      bool isCounterAttack,
      ref s9: String)
    {
      let mut num1: i32 =  -1;
      let mut index1: i32 =  -1;
      let mut unitGroup: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup;
      let mut iunr: i32 =  cc.IList[defnr].IUnr;
      let mut historical: i32 =  game.Data.UnitObj[iunr].Historical;
      let mut id: i32 =  game.Data.RegimeObj[game.Data.UnitObj[cc.IList[defnr].IUnr].Regime].id;
      if (historical > -1)
        num1 = game.Data.HistoricalUnitObj[historical].ID;
      if (index1 > -1 && game.Data.HistoricalUnitObj[index1].GiveHisVarValue(30) < 0)
        return 0;
      let mut num2: i32 =  100;
      let mut num3: i32 =  1;
      do
      {
        let mut index2: i32 =  -1;
        let mut index3: i32 =  -1;
        if (num3 == 1)
        {
          index2 = attnr;
          index3 = defnr;
        }
        if (num3 == 2)
        {
          index2 = defnr;
          index3 = attnr;
        }
        if (!Information.IsNothing( cc.IList[index2].IunitFeat))
        {
          let mut counter: i32 =  cc.IList[index2].IunitFeat.Counter;
          for (let mut index4: i32 =  0; index4 <= counter; index4 += 1)
          {
            if (cc.IList[index2].IunitFeat.Id[index4] > 0 && cc.IList[cc.FindISlot(cc.IList[index2].IunitFeat.Data1[index4])].IKilled == 0 & cc.IList[cc.FindISlot(cc.IList[index2].IunitFeat.Data1[index4])].IRetreated == 0 & cc.IList[cc.FindISlot(cc.IList[index2].IunitFeat.Data1[index4])].IRetreat == 0)
            {
              let mut idValue: i32 =  cc.IList[index2].IunitFeat.Id[index4];
              data: String = game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue, 2);
              let mut length: i32 =  game.Data.StringListObj[this.slotBehaviour].Length;
              for (let mut index5: i32 =  0; index5 <= length; index5 += 1)
              {
                let mut num4: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 0]));
                if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 1])) == idValue)
                {
                  let mut num5: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 3]));
                  let mut num6: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 4]));
                  let mut num7: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 5]));
                  let mut num8: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 6]));
                  let mut num9: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 7]));
                  index6: i32;
                  str: String = game.Data.StringListObj[this.slotBehaviour].Data[index5, index6];
                  let mut num10: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 8]));
                  let mut num11: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 9]));
                  bool flag = false;
                  if (cc.CombatType == 1 & num5 == 1)
                    flag = true;
                  if (cc.CombatType == 3 & num6 == 1)
                    flag = true;
                  if (num6 == 1 & cc.game.Data.SFTypeObj[cc.IList[index2].ISFType].ArtRange > 0)
                    flag = true;
                  if (num7 != 5)
                    flag = false;
                  if (num8 > -1 & num8 != game.Data.SFTypeObj[cc.IList[index3].ISFType].UnitGroup)
                    flag = false;
                  if (flag && num4 == 1 && index2 == attnr)
                  {
                    num2 -= (int) Math.Round( (num2 * num9) / 100.0);
                    s9 = data + " diminished battle startup penalty with " + num9.ToString() + "%.";
                  }
                }
              }
            }
          }
        }
        num3 += 1;
      }
      while (num3 <= 2);
      return num2;
    }

    pub fn UnitCombatCall_AvoidPanic(game: GameClass, unr: i32, ref s9: String) -> i32
    {
      let mut num: i32 =  -1;
      let mut idValue2: i32 =  -1;
      let mut historical1: i32 =  game.Data.UnitObj[unr].Historical;
      let mut id: i32 =  game.Data.RegimeObj[game.Data.UnitObj[unr].Regime].id;
      if (historical1 > -1)
        num = game.Data.HistoricalUnitObj[historical1].ID;
      unr = game.Data.UnitObj[unr].HQ;
      if (unr > -1)
      {
        let mut historical2: i32 =  game.Data.UnitObj[unr].Historical;
        if (game.Data.UnitObj[unr].IsHQ && game.Data.HistoricalUnitObj[historical2].Type >= 5)
          idValue2 = game.Data.HistoricalUnitObj[historical2].ID;
      }
      if (idValue2 > 0)
      {
        let mut charId: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData2(6, 3, 7, idValue2, 0)));
        if (charId > 0 && game.HandyFunctionsObj.Gethqpow(unr) > DrawMod.RandyNumber.Next(0, 100) && this.CachedSkillRoll(game, charId, 16) >= 100)
        {
          s9 = "Operational Commander managed to avoid the unit panicking by succesfull Against the Odds roll.";
          return 1;
        }
      }
      return 0;
    }

    pub fn UnitAirBridgeBonus(game: GameClass, unr: i32) -> i32
    {
      let mut num1: i32 =  -1;
      let mut idValue2: i32 =  -1;
      let mut historical1: i32 =  game.Data.UnitObj[unr].Historical;
      let mut id: i32 =  game.Data.RegimeObj[game.Data.UnitObj[unr].Regime].id;
      if (historical1 > -1)
        num1 = game.Data.HistoricalUnitObj[historical1].ID;
      unr = game.Data.UnitObj[unr].HQ;
      if (unr > -1)
      {
        let mut historical2: i32 =  game.Data.UnitObj[unr].Historical;
        if (game.Data.UnitObj[unr].IsHQ && game.Data.HistoricalUnitObj[historical2].Type >= 5)
          idValue2 = game.Data.HistoricalUnitObj[historical2].ID;
      }
      if (idValue2 > 0)
      {
        let mut charId: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotChar].GetData2(6, 3, 7, idValue2, 0)));
        if (charId > 0 && game.HandyFunctionsObj.Gethqpow(unr) > DrawMod.RandyNumber.Next(0, 100))
        {
          let mut num2: i32 =  this.CachedSkillRoll(game, charId, 55);
          if (num2 > 100)
            return num2 - 100;
        }
      }
      return 0;
    }

    pub IndividualCombatCall_RiverTypeAndLandscapeTypeModifier: i32(
      ref cc: CombatClass,
      game: GameClass,
      attnr: i32,
      defnr: i32,
      bool isCounterAttack,
      ref s9: String,
      riverHpMod: i32,
      landscapeAttMod: i32,
      landscapeDefMod: i32,
      riverType: i32)
    {
      let mut num1: i32 =  -1;
      let mut unitGroup: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup;
      let mut iunr: i32 =  cc.IList[defnr].IUnr;
      let mut historical: i32 =  game.Data.UnitObj[iunr].Historical;
      let mut id: i32 =  game.Data.RegimeObj[game.Data.UnitObj[cc.IList[defnr].IUnr].Regime].id;
      if (historical > -1)
        num1 = game.Data.HistoricalUnitObj[historical].ID;
      let mut num2: i32 =  100;
      if (riverHpMod != 0)
        num2 = riverHpMod;
      if (landscapeAttMod != 0)
        num2 = landscapeAttMod;
      if (landscapeDefMod != 0)
        num2 = landscapeDefMod;
      let mut num3: i32 =  1;
      do
      {
        let mut index1: i32 =  -1;
        let mut index2: i32 =  -1;
        if (num3 == 1)
        {
          index1 = attnr;
          index2 = defnr;
        }
        if (num3 == 2)
        {
          index1 = defnr;
          index2 = attnr;
        }
        if (!Information.IsNothing( cc.IList[index1].IunitFeat))
        {
          let mut counter: i32 =  cc.IList[index1].IunitFeat.Counter;
          for (let mut index3: i32 =  0; index3 <= counter; index3 += 1)
          {
            if (cc.IList[index1].IunitFeat.Id[index3] > 0 && cc.IList[cc.FindISlot(cc.IList[index1].IunitFeat.Data1[index3])].IKilled == 0 & cc.IList[cc.FindISlot(cc.IList[index1].IunitFeat.Data1[index3])].IRetreated == 0 & cc.IList[cc.FindISlot(cc.IList[index1].IunitFeat.Data1[index3])].IRetreat == 0)
            {
              let mut idValue: i32 =  cc.IList[index1].IunitFeat.Id[index3];
              data: String = game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue, 2);
              let mut length: i32 =  game.Data.StringListObj[this.slotBehaviour].Length;
              for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
              {
                let mut num4: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 0]));
                if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 1])) == idValue)
                {
                  let mut num5: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 3]));
                  let mut num6: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 4]));
                  let mut num7: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 5]));
                  let mut num8: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 6]));
                  let mut num9: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 7]));
                  index5: i32;
                  str1: String = game.Data.StringListObj[this.slotBehaviour].Data[index4, index5];
                  let mut num10: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 8]));
                  let mut num11: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 9]));
                  bool flag = false;
                  if (cc.CombatType == 1 & num5 == 1)
                    flag = true;
                  if (cc.CombatType == 3 & num6 == 1)
                    flag = true;
                  if (num6 == 1 & cc.game.Data.SFTypeObj[cc.IList[index1].ISFType].ArtRange > 0)
                    flag = true;
                  if (num7 != 12)
                    flag = false;
                  if (num8 > -1 & num8 != game.Data.SFTypeObj[cc.IList[index2].ISFType].UnitGroup)
                    flag = false;
                  if (flag)
                  {
                    num12: i32;
                    num13: i32;
                    if (num4 == 36 && index1 == attnr && num9 == -1 | game.Data.MapObj[0].HexObj[cc.TargetX, cc.TargetY].LandscapeType == num9)
                    {
                      let mut num14: i32 =  num2;
                      if (num2 < num11)
                      {
                        num2 += num10;
                        if (num2 > num11)
                          num2 = num11;
                        ref local: String = ref s9;
                        strArray1: Vec<String> = new string[6]
                        {
                          data,
                          " changed landscape modifier from ",
                          null,
                          null,
                          null,
                          null
                        };
                        strArray2: Vec<String> = strArray1;
                        num12 = num14 - 100;
                        str2: String = num12.ToString();
                        strArray2[2] = str2;
                        strArray1[3] = "% to ";
                        strArray3: Vec<String> = strArray1;
                        num13 = num2 - 100;
                        str3: String = num13.ToString();
                        strArray3[4] = str3;
                        strArray1[5] = "%.";
                        str4: String = string.Concat(strArray1);
                        local = str4;
                      }
                    }
                    if (num4 == 35 && index1 == defnr && num9 == -1 | riverType == num9)
                    {
                      let mut num15: i32 =  num2;
                      if (num2 < 100)
                      {
                        num2 += num10;
                        if (num2 > 100)
                          num2 = 100;
                        ref local: String = ref s9;
                        strArray4: Vec<String> = new string[6]
                        {
                          data,
                          "  changed river modifier from ",
                          null,
                          null,
                          null,
                          null
                        };
                        strArray5: Vec<String> = strArray4;
                        num13 = num15 - 100;
                        str5: String = num13.ToString();
                        strArray5[2] = str5;
                        strArray4[3] = "% to ";
                        strArray6: Vec<String> = strArray4;
                        num12 = num2 - 100;
                        str6: String = num12.ToString();
                        strArray6[4] = str6;
                        strArray4[5] = "%.";
                        str7: String = string.Concat(strArray4);
                        local = str7;
                      }
                    }
                  }
                }
              }
            }
          }
        }
        num3 += 1;
      }
      while (num3 <= 2);
      return num2;
    }

    pub IndividualCombatCall_SmallSizeBGmodifierApplies: bool(
      ref cc: CombatClass,
      game: GameClass,
      attnr: i32,
      defnr: i32)
    {
      let mut isfType: i32 =  cc.IList[attnr].ISFType;
      let mut id: i32 =  game.Data.SFTypeObj[isfType].Id;
      let mut integer: i32 =  Conversions.ToInteger(game.Data.StringListObj[this.slotModels].GetData(5, id, 2));
      return game.HandyFunctionsObj.GetRegimeByID(integer) > 0;
    }

    pub IndividualCombatCall_ResultModifier: i32(
      result: i32,
      ref cc: CombatClass,
      game: GameClass,
      attnr: i32,
      defnr: i32,
      bool isCounterAttack,
      float attval,
      float defval,
      ref s9: String)
    {
      let mut num1: i32 =  -1;
      let mut id1: i32 =  -1;
      let mut num2: i32 =  -1;
      let mut num3: i32 =  -1;
      let mut id2: i32 =  -1;
      let mut num4: i32 =  -1;
      let mut unitGroup1: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup;
      let mut iunr1: i32 =  cc.IList[attnr].IUnr;
      let mut historical1: i32 =  game.Data.UnitObj[iunr1].Historical;
      let mut id3: i32 =  game.Data.RegimeObj[game.Data.UnitObj[cc.IList[attnr].IUnr].Regime].id;
      if (historical1 > -1)
        num1 = game.Data.HistoricalUnitObj[historical1].ID;
      for (let mut index: i32 =  iunr1; index > -1; index = game.Data.UnitObj[index].HQ)
      {
        let mut historical2: i32 =  game.Data.UnitObj[index].Historical;
        if (game.Data.UnitObj[index].IsHQ)
        {
          if (game.Data.HistoricalUnitObj[historical2].Type == 8)
            num2 = game.Data.HistoricalUnitObj[historical2].ID;
          else if (game.Data.HistoricalUnitObj[historical2].Type >= 5)
            id1 = game.Data.HistoricalUnitObj[historical2].ID;
        }
      }
      let mut num5: i32 =  -1;
      if (id1 > -1)
        num5 = game.HandyFunctionsObj.GetHistoricalUnitByID(id1);
      let mut unitGroup2: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup;
      let mut iunr2: i32 =  cc.IList[defnr].IUnr;
      let mut historical3: i32 =  game.Data.UnitObj[iunr2].Historical;
      let mut id4: i32 =  game.Data.RegimeObj[game.Data.UnitObj[cc.IList[defnr].IUnr].Regime].id;
      if (historical3 > -1)
        num3 = game.Data.HistoricalUnitObj[historical3].ID;
      for (let mut index: i32 =  iunr2; index > -1; index = game.Data.UnitObj[index].HQ)
      {
        let mut historical4: i32 =  game.Data.UnitObj[index].Historical;
        if (game.Data.UnitObj[index].IsHQ)
        {
          if (game.Data.HistoricalUnitObj[historical4].Type == 8)
            num4 = game.Data.HistoricalUnitObj[historical4].ID;
          else if (game.Data.HistoricalUnitObj[historical4].Type >= 5)
            id2 = game.Data.HistoricalUnitObj[historical4].ID;
        }
      }
      let mut num6: i32 =  -1;
      if (id2 > -1)
        num6 = game.HandyFunctionsObj.GetHistoricalUnitByID(id2);
      s9 = "";
      let mut num7: i32 =  1;
      tid1: i32;
      do
      {
        let mut index1: i32 =  -1;
        let mut index2: i32 =  -1;
        if (num7 == 1)
        {
          index1 = attnr;
          index2 = defnr;
        }
        if (num7 == 2)
        {
          index1 = defnr;
          index2 = attnr;
        }
        if (!Information.IsNothing( cc.IList[index1].IunitFeat))
        {
          let mut counter: i32 =  cc.IList[index1].IunitFeat.Counter;
          for (let mut index3: i32 =  0; index3 <= counter; index3 += 1)
          {
            if (cc.IList[index1].IunitFeat.Id[index3] > 0 && cc.IList[cc.FindISlot(cc.IList[index1].IunitFeat.Data1[index3])].IKilled == 0 & cc.IList[cc.FindISlot(cc.IList[index1].IunitFeat.Data1[index3])].IRetreated == 0 & cc.IList[cc.FindISlot(cc.IList[index1].IunitFeat.Data1[index3])].IRetreat == 0)
            {
              let mut idValue: i32 =  cc.IList[index1].IunitFeat.Id[index3];
              data: String = game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue, 2);
              let mut length: i32 =  game.Data.StringListObj[this.slotBehaviour].Length;
              for (let mut index4: i32 =  0; index4 <= length; index4 += 1)
              {
                let mut num8: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 0]));
                if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 1])) == idValue)
                {
                  let mut num9: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 3]));
                  let mut num10: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 4]));
                  let mut num11: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 5]));
                  let mut num12: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 6]));
                  let mut num13: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 7]));
                  index5: i32;
                  idValue2: String = game.Data.StringListObj[this.slotBehaviour].Data[index4, index5];
                  let mut Expression: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 8]));
                  let mut num14: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index4, 9]));
                  bool flag = false;
                  if (cc.CombatType == 1 & num9 == 1)
                    flag = true;
                  if (cc.CombatType == 3 & num10 == 1)
                    flag = true;
                  if (num10 == 1 & cc.game.Data.SFTypeObj[cc.IList[index1].ISFType].ArtRange > 0)
                    flag = true;
                  if (num11 != 4)
                    flag = false;
                  if (num12 > -1 & num12 != game.Data.SFTypeObj[cc.IList[index2].ISFType].UnitGroup)
                    flag = false;
                  if (flag)
                  {
                    if (num8 == 14 && index1 == defnr && result == 3)
                    {
                      s9 = data + " tried to prevent RETREAT result. " + num13.ToString() + "% chance. ";
                      if (DrawMod.RandyNumber.Next(0, 100) < num13)
                      {
                        result = 0;
                        s9 += "Succes!";
                      }
                      else
                        s9 += "Failed.";
                    }
                    if (num8 == 24 && index1 == defnr && result == 1)
                    {
                      s9 = data + " tried to prevent KILL result. " + num13.ToString() + "% chance. ";
                      if (DrawMod.RandyNumber.Next(0, 100) < num13)
                      {
                        result = 3;
                        s9 += "Succes!";
                      }
                      else
                        s9 += "Failed.";
                    }
                    if (num8 == 27 && index1 == attnr && result > 0)
                    {
                      tid1 = (int) Math.Round( (cc.IList[defnr].IRdn * num13) / 100.0);
                      Individual[] ilist1 = cc.IList;
                      Individual[] individualArray1 = ilist1;
                      let mut index6: i32 =  defnr;
                      let mut index7: i32 =  index6;
                      individualArray1[index7].IRdn = ilist1[index6].IRdn - tid1;
                      Individual[] ilist2 = cc.IList;
                      Individual[] individualArray2 = ilist2;
                      let mut index8: i32 =  attnr;
                      let mut index9: i32 =  index8;
                      individualArray2[index9].IRdn = ilist2[index8].IRdn + tid1;
                      if (100 > cc.IList[attnr].IRdn)
                        cc.IList[attnr].IRdn = 100;
                      s9 = data + " took " + tid1.ToString() + " readiness from target and added it to own readiness.";
                    }
                    if (num8 == 31 && index1 == attnr && result > 0)
                    {
                      tid1 = (int) Math.Round( (cc.IList[defnr].IXp * num13) / 100.0);
                      Individual[] ilist = cc.IList;
                      Individual[] individualArray = ilist;
                      let mut index10: i32 =  defnr;
                      let mut index11: i32 =  index10;
                      individualArray[index11].IXp = ilist[index10].IXp - tid1;
                      s9 = data + " destroyed " + tid1.ToString() + " experience of target.";
                    }
                    if (num8 == 6 && index1 == attnr && result == 0 | result == 4)
                    {
                      s9 = data + " tried to make enemy RETREAT. " + num13.ToString() + "% chance. ";
                      if (DrawMod.RandyNumber.Next(0, 100) < num13)
                      {
                        result = 3;
                        s9 += "Succes!";
                      }
                      else
                        s9 += "Failed.";
                    }
                    if (num8 == 4 && index1 == attnr && result == 1)
                    {
                      tid1 = game.Data.SFTypeObj[cc.IList[defnr].ISFType].HitPoints[game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup];
                      let mut num15: i32 =  (int) Math.Round( ((int) Math.Round(Conversion.Val( Expression)) * tid1) / 1000.0) + 1;
                      if (num15 > 0)
                      {
                        s9 = data + " took " + num15.ToString() + " " + idValue2 + " of plunder from killed target.";
                        let mut setValue: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotRegimeKey].GetData2(0, id3, 1, idValue2, 2))) + num15;
                        game.Data.StringListObj[this.slotRegimeKey].SetData2(0, id3, 1, idValue2, 2, setValue);
                      }
                    }
                    if (num8 == 28 && index1 == attnr && result != 1)
                    {
                      s9 = data + " tried to KILL the target. " + num13.ToString() + "% chance. ";
                      if (DrawMod.RandyNumber.Next(0, 100) < num13)
                      {
                        result = 1;
                        s9 += "Succes!";
                      }
                      else
                        s9 += "Failed.";
                    }
                  }
                }
              }
            }
          }
        }
        num7 += 1;
      }
      while (num7 <= 2);
      if (result == 3)
      {
        let mut people: i32 =  game.Data.SFObj[cc.IList[defnr].ISFNr].People;
        if (people > -1 && DrawMod.TGame.Data.PeopleObj[people].tv0 == (int) Math.Round(Conversion.Val(DrawMod.TGame.Data.StringListObj[this.slotGameKeys].GetData(0, 37, 2))))
          result = game.Data.SFTypeObj[cc.IList[attnr].ISFType].KillPercent * 2 < DrawMod.RandyNumber.Next(0, 100) ? 4 : 1;
      }
      if (result == 1)
        this.PersonalCombatRoll(ref cc, game, cc.IList[defnr].IUnr, defnr, 50, "", false, false);
      if (result != 0)
      {
        tid1 = game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[81];
        let mut tid2: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[81];
        if (tid1 > 0)
          this.MODL.AddWeight(tid1, 1);
        if (tid2 > 0)
          this.MODL.AddWeight(tid2, 5);
      }
      if (result != 0)
      {
        int[] sfTypeVar1 = game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar;
        int[] numArray1 = sfTypeVar1;
        let mut index12: i32 =  4;
        let mut index13: i32 =  index12;
        let mut num16: i32 =  sfTypeVar1[index12] + 1;
        numArray1[index13] = num16;
        if (game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup == 3 | game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup == 4)
        {
          int[] sfTypeVar2 = game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar;
          int[] numArray2 = sfTypeVar2;
          let mut index14: i32 =  5;
          let mut index15: i32 =  index14;
          let mut num17: i32 =  sfTypeVar2[index14] + 1;
          numArray2[index15] = num17;
        }
        else if (game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup == 8 | game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup == 9)
        {
          int[] sfTypeVar3 = game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar;
          int[] numArray3 = sfTypeVar3;
          let mut index16: i32 =  6;
          let mut index17: i32 =  index16;
          let mut num18: i32 =  sfTypeVar3[index16] + 1;
          numArray3[index17] = num18;
        }
        int[] sfTypeVar4 = game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar;
        int[] numArray4 = sfTypeVar4;
        let mut index18: i32 =  1;
        let mut index19: i32 =  index18;
        let mut num19: i32 =  sfTypeVar4[index18] + 1;
        numArray4[index19] = num19;
        if (game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup == 3 | game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup == 4)
        {
          int[] sfTypeVar5 = game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar;
          int[] numArray5 = sfTypeVar5;
          let mut index20: i32 =  2;
          let mut index21: i32 =  index20;
          let mut num20: i32 =  sfTypeVar5[index20] + 1;
          numArray5[index21] = num20;
        }
        else if (game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup == 8 | game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup == 9)
        {
          int[] sfTypeVar6 = game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar;
          int[] numArray6 = sfTypeVar6;
          let mut index22: i32 =  3;
          let mut index23: i32 =  index22;
          let mut num21: i32 =  sfTypeVar6[index22] + 1;
          numArray6[index23] = num21;
        }
      }
      if (result == 0 && cc.game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[82] > 0 && cc.game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[8] > 100000)
      {
        let mut isfType: i32 =  cc.IList[defnr].ISFType;
        if (cc.game.Data.SFTypeObj[isfType].SFTypeVar[8] > 0)
        {
          let mut num22: i32 =  (int) Math.Round(Conversion.Val(Strings.Mid(cc.game.Data.SFTypeObj[isfType].SFTypeVar[8].ToString(), 5, 1)));
          if (num22 > 0)
          {
            tid1 = cc.IList[attnr].IMor;
            tid1 = (int) Math.Round(Math.Ceiling( (tid1 * num22) / 20.0));
            Individual[] ilist = cc.IList;
            Individual[] individualArray = ilist;
            let mut index24: i32 =  attnr;
            let mut index25: i32 =  index24;
            individualArray[index25].IMor = ilist[index24].IMor - tid1;
            if (cc.IList[attnr].IMor < 1)
              cc.IList[attnr].IMor = 1;
            s9 += "Morale loss for attacker due to Bane Fauna Feat. ";
          }
        }
      }
      if (result != 0 && cc.game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[82] > 0 && cc.game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[8] > 100000)
      {
        let mut isfType: i32 =  cc.IList[defnr].ISFType;
        if (cc.game.Data.SFTypeObj[isfType].SFTypeVar[8] > 0)
        {
          let mut num23: i32 =  (int) Math.Round(Conversion.Val(Strings.Mid(cc.game.Data.SFTypeObj[isfType].SFTypeVar[8].ToString(), 6, 1)));
          if (num23 > 0 && DrawMod.RandyNumber.Next(0, 100) < 40 + num23 * 10)
          {
            s9 += "Regenerate Fauna Feat cancels effects of hit. ";
            result = 0;
          }
        }
      }
      return result;
    }

    pub float IndividualCombatCall_AttValModder(
      modNr: i32,
      ref cc: CombatClass,
      game: GameClass,
      attnr: i32,
      defnr: i32,
      bool isCounterAttack,
      float attval,
      ref s9: String)
    {
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      let mut jobSpecificId: i32 =  -1;
      let mut unitGroup1: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup;
      let mut index1: i32 =  cc.IList[attnr].IUnr;
      let mut historical1: i32 =  game.Data.UnitObj[index1].Historical;
      let mut id1: i32 =  game.Data.RegimeObj[game.Data.UnitObj[cc.IList[attnr].IUnr].Regime].id;
      if (historical1 > -1)
        num1 = game.Data.HistoricalUnitObj[historical1].ID;
      let mut num3: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotRegimes].GetData(0, id1, 1)));
      bool flag1 = false;
      if (game.Data.HistoricalUnitObj[historical1].GiveHisVarValue(11) > 0)
        flag1 = true;
      if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotRegimes].GetData(0, id1, 1))) > 1)
        flag1 = true;
      for (; index1 > -1; index1 = game.Data.UnitObj[index1].HQ)
      {
        let mut historical2: i32 =  game.Data.UnitObj[index1].Historical;
        if (game.Data.UnitObj[index1].IsHQ)
        {
          if (game.Data.HistoricalUnitObj[historical2].Type == 8)
            jobSpecificId = game.Data.HistoricalUnitObj[historical2].ID;
          else if (game.Data.HistoricalUnitObj[historical2].Type >= 5)
            num2 = game.Data.HistoricalUnitObj[historical2].ID;
        }
      }
      if (modNr == 12 && cc.game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[82] > 0 && cc.IList[attnr].IUnr > -1 && DrawMod.TGame.Data.RegimeObj[DrawMod.TGame.Data.UnitObj[cc.IList[attnr].IUnr].Regime].AI)
        attval *= 2f;
      if (modNr == 12)
      {
        if (cc.game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[82] > 0 && cc.game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[8] > 100000)
        {
          let mut isfType: i32 =  cc.IList[attnr].ISFType;
          if (cc.game.Data.SFTypeObj[isfType].SFTypeVar[8] > 0)
          {
            let mut num4: i32 =  (int) Math.Round(Conversion.Val(Strings.Mid(cc.game.Data.SFTypeObj[isfType].SFTypeVar[8].ToString(), 3, 1)));
            if (num4 > 0 && cc.CombatRound == 1 & cc.IList[attnr].IAttacker == 0)
            {
              let mut num5: i32 =  (int) Math.Round( (attval *  num4));
              s9 = s9 + "Traps set by critter adds " + num5.ToString() + " attack points.";
              attval +=  num5;
            }
          }
        }
        if (cc.game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[82] > 0 && cc.game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[8] > 100000)
        {
          let mut isfType1: i32 =  cc.IList[defnr].ISFType;
          if (cc.game.Data.SFTypeObj[isfType1].SFTypeVar[8] > 0)
          {
            let mut num6: i32 =  (int) Math.Round(Conversion.Val(Strings.Mid(cc.game.Data.SFTypeObj[isfType1].SFTypeVar[8].ToString(), 4, 1)));
            if (num6 > 0)
            {
              let mut isfType2: i32 =  cc.IList[attnr].ISFType;
              let mut num7: i32 =  cc.game.Data.SFTypeObj[isfType2].AttackPower[cc.game.Data.SFTypeObj[isfType1].UnitGroup];
              if (cc.game.Data.SFTypeObj[isfType1].HitPoints[cc.game.Data.SFTypeObj[isfType2].UnitGroup] > num7 & DrawMod.RandyNumber.Next(0, 100) < 50 + num6 * 10)
              {
                s9 += "Amorphous feat of critter negates the attack.";
                attval = 0.0f;
              }
            }
          }
        }
      }
      let mut num8: i32 =  0;
      let mut num9: i32 =  -1;
      let mut num10: i32 =  1;
      do
      {
        let mut index2: i32 =  -1;
        let mut index3: i32 =  -1;
        if (num10 == 1)
        {
          index2 = attnr;
          index3 = defnr;
        }
        if (num10 == 2)
        {
          index2 = defnr;
          index3 = attnr;
        }
        if (!Information.IsNothing( cc.IList[index2].IunitFeat))
        {
          let mut counter: i32 =  cc.IList[index2].IunitFeat.Counter;
          for (let mut index4: i32 =  0; index4 <= counter; index4 += 1)
          {
            if (cc.IList[index2].IunitFeat.Id[index4] > 0 && cc.IList[cc.FindISlot(cc.IList[index2].IunitFeat.Data1[index4])].IKilled == 0 & cc.IList[cc.FindISlot(cc.IList[index2].IunitFeat.Data1[index4])].IRetreated == 0 & cc.IList[cc.FindISlot(cc.IList[index2].IunitFeat.Data1[index4])].IRetreat == 0)
            {
              let mut idValue: i32 =  cc.IList[index2].IunitFeat.Id[index4];
              data: String = game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue, 2);
              let mut length: i32 =  game.Data.StringListObj[this.slotBehaviour].Length;
              for (let mut index5: i32 =  0; index5 <= length; index5 += 1)
              {
                let mut num11: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 0]));
                if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 1])) == idValue)
                {
                  let mut num12: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 3]));
                  let mut num13: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 4]));
                  let mut num14: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 5]));
                  let mut num15: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 6]));
                  let mut num16: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 7]));
                  index6: i32;
                  str1: String = game.Data.StringListObj[this.slotBehaviour].Data[index5, index6];
                  let mut maxValue: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 8]));
                  let mut num17: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index5, 9]));
                  bool flag2 = false;
                  if (cc.CombatType == 1 & num12 == 1)
                    flag2 = true;
                  if (cc.CombatType == 3 & num13 == 1)
                    flag2 = true;
                  if (num13 == 1 & cc.game.Data.SFTypeObj[cc.IList[index2].ISFType].ArtRange > 0)
                    flag2 = true;
                  if (!(num14 == 1 | num14 == 3))
                    flag2 = false;
                  if (num15 > -1 & num15 != game.Data.SFTypeObj[cc.IList[index3].ISFType].UnitGroup)
                    flag2 = false;
                  if (flag2)
                  {
                    if (modNr == 12)
                    {
                      if (num11 == 2 && index2 == defnr)
                      {
                        s9 = s9 + "Defenders " + data + " gives " + num16.ToString() + "% chance to foil our attack. ";
                        if (DrawMod.RandyNumber.Next(0, 100) > num16)
                        {
                          s9 += "Succes!";
                          attval = 0.0f;
                        }
                        else
                          s9 += "Failed.";
                      }
                      if (num11 == 16 && index2 == attnr)
                      {
                        let mut num18: i32 =  (int) Math.Round( (cc.IList[defnr].IEntrench * num16) / 100.0);
                        Individual[] ilist = cc.IList;
                        Individual[] individualArray = ilist;
                        let mut index7: i32 =  defnr;
                        let mut index8: i32 =  index7;
                        individualArray[index8].IEntrench = ilist[index7].IEntrench - num18;
                        s9 = s9 + data + " diminished targets entrenchment with " + num18.ToString() + ".";
                      }
                      if (num11 == 33 && index2 == attnr & cc.IList[attnr].IAttacker < 1 & maxValue >= cc.CombatRound & cc.CombatRound <= num17)
                      {
                        let mut num19: i32 =  num16;
                        attval +=  num19;
                        s9 = s9 + data + " adds " + num19.ToString() + " attack points.";
                      }
                      if (num11 == 7 && index2 == defnr)
                      {
                        let mut num20: i32 =  (int) Math.Round( attval *  num16 / 100.0);
                        attval -=  num20;
                        s9 = s9 + "Defenders " + data + " diminished attackers attack with " + num16.ToString() + "%.";
                      }
                      if (num11 == 23 && index2 == attnr)
                      {
                        s9 = s9 + data + " has " + num16.ToString() + "% chance to make a succesfull attack. ";
                        if (DrawMod.RandyNumber.Next(0, 100) < num16)
                        {
                          s9 += "Succes. ";
                          let mut num21: i32 =  DrawMod.RandyNumber.Next(0, maxValue);
                          s9 = s9 + " Did " + num21.ToString() + " points of attack dammage.";
                          attval +=  num21;
                        }
                        else
                          s9 += "Failure. ";
                        if (num17 > 0 && DrawMod.RandyNumber.Next(0, 100) < num17)
                        {
                          s9 = s9 + " The " + data + " was destroyed in the process.";
                          cc.IList[attnr].IunitFeat.Id[index4] = 0;
                          cc.IList[attnr].IunitFeatDeadRound = cc.CombatRound;
                        }
                      }
                      if (num11 == 13 && index2 == attnr & cc.IList[attnr].AttackCount < 1 & !isCounterAttack)
                      {
                        cc.AddXp(attnr, num16);
                        s9 = s9 + " The " + data + " provided 10 experience gain chances.";
                      }
                      if (num11 == 25 && index2 == attnr && num17 < 1 | num17 <= cc.IList[attnr].IMor)
                      {
                        if (num16 > 0)
                        {
                          let mut num22: i32 =  (int) Math.Round( attval *  num16 / 100.0);
                          attval +=  num22;
                          s9 = data + " increased our attack with " + num16.ToString() + "%. ";
                        }
                        if (maxValue > 0)
                        {
                          let mut num23: i32 =  maxValue;
                          attval +=  num23;
                          s9 = s9 + data + " increased our attack with +" + maxValue.ToString() + " points.";
                        }
                      }
                      if (num11 == 34 && index2 == attnr && maxValue >= cc.CombatRound && game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[42] < num17 && num16 > 0)
                      {
                        let mut num24: i32 =  num16;
                        attval +=  num24;
                        s9 = data + " increased our attack with +" + num16.ToString() + " points.";
                      }
                      if (num11 == 15 && index2 == attnr)
                      {
                        let mut num25: i32 =  (int) Math.Round( (game.Data.SFObj[cc.IList[attnr].ISFNr].Rdn * num16) / 100.0);
                        let mut num26: i32 =  num25 - cc.IList[attnr].IRdn;
                        if (cc.IList[attnr].IRdn < num25)
                          cc.IList[attnr].IRdn = num25;
                        if (num26 > 0)
                          s9 = data + " increased our readiness with " + num26.ToString() + "points back to " + num16.ToString() + "% of original readiness.";
                      }
                    }
                    else if (modNr != 12 && num11 == 12 & index2 == attnr)
                    {
                      str2: String = num16 <= 0 ? "on all skills." : "on skill '" + game.Data.StringListObj[this.slotSkillTypes].GetData(0, num16, 1) + "'";
                      s9 = s9 + data + " gives a skill modification of " + maxValue.ToString() + " points " + str2;
                      num9 = num16;
                      num8 = maxValue;
                    }
                  }
                }
              }
            }
          }
        }
        num10 += 1;
      }
      while (num10 <= 2);
      let mut index9: i32 =  -1;
      if (num2 > -1)
        index9 = game.HandyFunctionsObj.GetHistoricalUnitByID(num2);
      let mut characterId1: i32 =  game.EventRelatedObj.Helper_GetCharacterId(id1, 3, num2, -1, this.slotChar);
      let mut characterId2: i32 =  game.EventRelatedObj.Helper_GetCharacterId(id1, 4, jobSpecificId, -1, this.slotChar);
      if (characterId1 > 0 & game.HandyFunctionsObj.Gethqpow(cc.IList[attnr].IUnr) > DrawMod.RandyNumber.Next(0, 100))
      {
        float num27 = attval;
        let mut tdata2: i32 =  0;
        if (this.airEnabled)
        {
          if (modNr == 18 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater == 2 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater == 2)
          {
            let mut num28: i32 =  this.CachedSkillRoll(game, characterId1, 51);
            if (51 == num9 | num9 == -1)
              num28 += num8;
            this.SL.AddWeight(characterId1, 1, 51, CheckData1Existence: true);
            if (num28 > 100)
              attval +=  ( attval *  (num28 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 51;
          }
          if (cc.IList[attnr].IAttacker == 0 && modNr == 19 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater == 2 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater == 2)
          {
            let mut num29: i32 =  this.CachedSkillRoll(game, characterId1, 52);
            if (52 == num9 | num9 == -1)
              num29 += num8;
            this.SL.AddWeight(characterId1, 1, 52, CheckData1Existence: true);
            if (num29 > 100)
              attval +=  ( attval *  (num29 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 52;
          }
          if (cc.IList[attnr].IAttacker == 1 & cc.CombatType2 != 16 && modNr == 20 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater == 2 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater == 2)
          {
            let mut num30: i32 =  this.CachedSkillRoll(game, characterId1, 53);
            if (53 == num9 | num9 == -1)
              num30 += num8;
            this.SL.AddWeight(characterId1, 1, 53, CheckData1Existence: true);
            if (num30 > 100)
              attval +=  ( attval *  (num30 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 53;
          }
          if (modNr == 21 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater == 2 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater < 2)
          {
            let mut num31: i32 =  this.CachedSkillRoll(game, characterId1, 54);
            if (54 == num9 | num9 == -1)
              num31 += num8;
            this.SL.AddWeight(characterId1, 1, 54, CheckData1Existence: true);
            if (num31 > 100)
              attval +=  ( attval *  (num31 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 54;
          }
          if (modNr == 22 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater < 2 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater == 2)
          {
            let mut num32: i32 =  this.CachedSkillRoll(game, characterId1, 56);
            if (56 == num9 | num9 == -1)
              num32 += num8;
            this.SL.AddWeight(characterId1, 1, 56, CheckData1Existence: true);
            if (num32 > 100)
              attval +=  ( attval *  (num32 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 56;
          }
        }
        if (game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater < 2)
        {
          if (modNr == 8)
          {
            let mut num33: i32 =  this.CachedSkillRoll(game, characterId1, 24);
            if (24 == num9 | num9 == -1)
              num33 += num8;
            this.SL.AddWeight(characterId1, 1, 24, CheckData1Existence: true);
            if (num33 > 100)
              attval +=  ( attval *  (num33 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 24;
          }
          if (modNr == 1 & unitGroup1 == 3 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].ArtRange < 1)
          {
            let mut num34: i32 =  this.CachedSkillRoll(game, characterId1, 31);
            if (31 == num9 | num9 == -1)
              num34 += num8;
            this.SL.AddWeight(characterId1, 1, 31, CheckData1Existence: true);
            if (num34 > 100)
              attval +=  ( attval *  (num34 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 31;
          }
          if (modNr == 2 & (unitGroup1 == 1 | unitGroup1 == 3 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].ArtRange > 0))
          {
            let mut num35: i32 =  this.CachedSkillRoll(game, characterId1, 32);
            if (32 == num9 | num9 == -1)
              num35 += num8;
            this.SL.AddWeight(characterId1, 1, 32, CheckData1Existence: true);
            if (num35 > 100)
              attval +=  ( attval *  (num35 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 32;
          }
          if (modNr == 13 & cc.IList[attnr].IAttacker == 0)
          {
            let mut num36: i32 =  this.CachedSkillRoll(game, characterId1, 4);
            if (4 == num9 | num9 == -1)
              num36 += num8;
            this.SL.AddWeight(characterId1, 1, 4, CheckData1Existence: true);
            let mut num37: i32 =  num36 - 100;
            if (num37 > 0)
              attval +=  (int) Math.Round( attval * ( cc.IList[attnr].IEntrench / 100.0) * ( num37 / 100.0));
            tdata2 = 4;
          }
          if (modNr == 3 & unitGroup1 == 0)
          {
            let mut num38: i32 =  this.CachedSkillRoll(game, characterId1, 33);
            if (33 == num9 | num9 == -1)
              num38 += num8;
            this.SL.AddWeight(characterId1, 1, 33, CheckData1Existence: true);
            if (num38 > 100)
              attval +=  ( attval *  (num38 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 33;
          }
          if (modNr == 6 & cc.IList[attnr].IAttacker == 1)
          {
            let mut num39: i32 =  this.CachedSkillRoll(game, characterId1, 36);
            if (36 == num9 | num9 == -1)
              num39 += num8;
            this.SL.AddWeight(characterId1, 1, 36, CheckData1Existence: true);
            if (num39 > 100)
              attval +=  ( attval *  (num39 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 36;
          }
          if (modNr == 5 & cc.IList[attnr].IAttacker == 1)
          {
            let mut num40: i32 =  this.CachedSkillRoll(game, characterId1, 23);
            if (23 == num9 | num9 == -1)
              num40 += num8;
            if (num40 > 100)
            {
              let mut num41: i32 =  num40 - 100;
              this.SL.AddWeight(characterId1, 1, 23, CheckData1Existence: true);
              if (cc.IList[defnr].IEntrench > 0)
              {
                let mut num42: i32 =  (int) Math.Round( (cc.IList[defnr].IEntrench * num41) / 200.0);
                cc.IList[defnr].IEntrench -= num42;
                if (0 > cc.IList[defnr].IEntrench)
                  cc.IList[defnr].IEntrench = 0;
              }
            }
            tdata2 = 23;
          }
        }
        let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num27) * 100.0) / num27));
        if (tweight > 0)
          tweight = tweight;
        if (tdata2 > 0)
        {
          let mut nr: i32 =  this.logLeaderBonus.FindNr(characterId1, tdata2: tdata2);
          if (nr <= -1)
          {
            this.logLeaderBonus.AddWeight(characterId1, tweight, 1, tdata2, CheckExistence: false);
          }
          else
          {
            int[] weight = this.logLeaderBonus.Weight;
            int[] numArray1 = weight;
            let mut index10: i32 =  nr;
            let mut index11: i32 =  index10;
            let mut num43: i32 =  weight[index10] + tweight;
            numArray1[index11] = num43;
            int[] data1 = this.logLeaderBonus.Data1;
            int[] numArray2 = data1;
            let mut index12: i32 =  nr;
            let mut index13: i32 =  index12;
            let mut num44: i32 =  data1[index12] + 1;
            numArray2[index13] = num44;
          }
        }
      }
      if (characterId2 > 0)
      {
        float num45 = attval;
        let mut tdata2: i32 =  0;
        if (modNr == 9)
        {
          let mut num46: i32 =  this.CachedSkillRoll(game, characterId2, 18);
          if (18 == num9 | num9 == -1)
            num46 += num8;
          this.SL.AddWeight(characterId2, 1, 18, CheckData1Existence: true);
          if (num46 > 100)
            attval +=  ( attval *  (num46 - 100) / 100.0) * this.SKILLEFFECT;
          tdata2 = 18;
        }
        let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num45) * 100.0) / num45));
        if (tdata2 > 0)
        {
          let mut nr: i32 =  this.logLeaderBonus.FindNr(characterId2, tdata2: tdata2);
          if (nr <= -1)
          {
            this.logLeaderBonus.AddWeight(characterId2, tweight, 1, tdata2, CheckExistence: false);
          }
          else
          {
            int[] weight = this.logLeaderBonus.Weight;
            int[] numArray3 = weight;
            let mut index14: i32 =  nr;
            let mut index15: i32 =  index14;
            let mut num47: i32 =  weight[index14] + tweight;
            numArray3[index15] = num47;
            int[] data1 = this.logLeaderBonus.Data1;
            int[] numArray4 = data1;
            let mut index16: i32 =  nr;
            let mut index17: i32 =  index16;
            let mut num48: i32 =  data1[index16] + 1;
            numArray4[index17] = num48;
          }
        }
      }
      if (modNr == 10)
      {
        float num49 = attval;
        let mut iunr: i32 =  cc.IList[attnr].IUnr;
        if (index9 > -1)
        {
          if (cc.AttackerRegime == game.Data.UnitObj[iunr].Regime)
          {
            let mut unitGroup2: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup;
            if (unitGroup2 == 0)
            {
              let mut num50: i32 =  game.Data.HistoricalUnitObj[index9].GiveHisVarValue(22);
              attval *=  (100 + num50) / 100f;
            }
            else if (unitGroup2 == 1 | unitGroup2 == 6)
            {
              let mut num51: i32 =  game.Data.HistoricalUnitObj[index9].GiveHisVarValue(23);
              attval *=  (100 + num51) / 100f;
            }
            else if (unitGroup2 == 2 | unitGroup2 == 3)
            {
              let mut num52: i32 =  game.Data.HistoricalUnitObj[index9].GiveHisVarValue(24);
              attval *=  (100 + num52) / 100f;
            }
          }
          if (cc.DefenderRegime == game.Data.UnitObj[iunr].Regime)
          {
            let mut unitGroup3: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].UnitGroup;
            if (unitGroup3 == 0)
            {
              let mut num53: i32 =  game.Data.HistoricalUnitObj[index9].GiveHisVarValue(25);
              attval *=  (100 + num53) / 100f;
            }
            else if (unitGroup3 == 1 | unitGroup3 == 6)
            {
              let mut num54: i32 =  game.Data.HistoricalUnitObj[index9].GiveHisVarValue(26);
              attval *=  (100 + num54) / 100f;
            }
            else if (unitGroup3 == 2 | unitGroup3 == 3)
            {
              let mut num55: i32 =  game.Data.HistoricalUnitObj[index9].GiveHisVarValue(27);
              attval *=  (100 + num55) / 100f;
            }
          }
          let mut num56: i32 =  game.Data.HistoricalUnitObj[index9].GiveHisVarValue(40);
          if (num56 == 1 & cc.IList[attnr].IAttacker == 1)
            attval *= cc.ConcentricBonus;
          if (num56 == 2)
          {
            let mut num57: i32 =  cc.IList[attnr].IMor - cc.IList[defnr].IMor;
            if (num57 > 0)
              attval *=  (100 + num57) / 100f;
          }
          if ( num49 > 0.0)
          {
            let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num49) * 100.0) / num49));
            let mut nr: i32 =  this.otherBonus.FindNr(cc.IList[attnr].IAttacker, tdata2: 1);
            if (nr <= -1)
            {
              this.otherBonus.AddWeight(cc.IList[attnr].IAttacker, tweight, 1, 1, CheckExistence: false);
            }
            else
            {
              int[] weight = this.otherBonus.Weight;
              int[] numArray5 = weight;
              let mut index18: i32 =  nr;
              let mut index19: i32 =  index18;
              let mut num58: i32 =  weight[index18] + tweight;
              numArray5[index19] = num58;
              int[] data1 = this.otherBonus.Data1;
              int[] numArray6 = data1;
              let mut index20: i32 =  nr;
              let mut index21: i32 =  index20;
              let mut num59: i32 =  data1[index20] + 1;
              numArray6[index21] = num59;
            }
          }
        }
      }
      if (modNr == 11)
      {
        float num60 = attval;
        let mut regime: i32 =  game.Data.UnitObj[cc.IList[attnr].IUnr].Regime;
        let mut id2: i32 =  game.Data.RegimeObj[regime].id;
        let mut num61: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotRegimeKey].GetData2(0, id2, 1, "combatBonus", 2)));
        attval +=  ( attval *  num61 / 100.0);
        if ( num60 > 0.0)
        {
          let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num60) * 100.0) / num60));
          let mut nr: i32 =  this.otherBonus.FindNr(cc.IList[attnr].IAttacker, tdata2: 2);
          if (nr <= -1)
          {
            this.otherBonus.AddWeight(cc.IList[attnr].IAttacker, tweight, 1, 2, CheckExistence: false);
          }
          else
          {
            int[] weight = this.otherBonus.Weight;
            int[] numArray7 = weight;
            let mut index22: i32 =  nr;
            let mut index23: i32 =  index22;
            let mut num62: i32 =  weight[index22] + tweight;
            numArray7[index23] = num62;
            int[] data1 = this.otherBonus.Data1;
            int[] numArray8 = data1;
            let mut index24: i32 =  nr;
            let mut index25: i32 =  index24;
            let mut num63: i32 =  data1[index24] + 1;
            numArray8[index25] = num63;
          }
        }
      }
      if (modNr == 16 && game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater == 2 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater == 2)
      {
        float num64 = attval;
        let mut regime: i32 =  game.Data.UnitObj[cc.IList[attnr].IUnr].Regime;
        let mut id3: i32 =  game.Data.RegimeObj[regime].id;
        let mut num65: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[15];
        let mut num66: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[16];
        let mut num67: i32 =  100;
        if (cc.CombatRound == 1)
          num67 = num65;
        if (cc.CombatRound == 2)
          num67 = num66;
        if (num67 < 0)
          num67 = 0;
        attval =  ( attval *  num67 / 100.0);
        if ( num64 > 0.0 & num67 != 0)
        {
          let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num64) * 100.0) / num64));
          let mut nr: i32 =  this.otherBonus.FindNr(cc.IList[attnr].IAttacker, tdata2: 2);
          if (nr <= -1)
          {
            this.otherBonus.AddWeight(cc.IList[attnr].IAttacker, tweight, 1, 2, CheckExistence: false);
          }
          else
          {
            int[] weight = this.otherBonus.Weight;
            int[] numArray9 = weight;
            let mut index26: i32 =  nr;
            let mut index27: i32 =  index26;
            let mut num68: i32 =  weight[index26] + tweight;
            numArray9[index27] = num68;
            int[] data1 = this.otherBonus.Data1;
            int[] numArray10 = data1;
            let mut index28: i32 =  nr;
            let mut index29: i32 =  index28;
            let mut num69: i32 =  data1[index28] + 1;
            numArray10[index29] = num69;
          }
        }
      }
      if (modNr == 17 && game.Data.SFTypeObj[cc.IList[attnr].ISFType].Theater == 2 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater == 2)
      {
        float num70 = attval;
        let mut regime: i32 =  game.Data.UnitObj[cc.IList[attnr].IUnr].Regime;
        let mut id4: i32 =  game.Data.RegimeObj[regime].id;
        float d =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[13] /  game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[13];
        if ( d > 1.0)
          d =  Math.Sqrt( d);
        if (cc.CombatRound == 1)
          d =  Math.Sqrt(Math.Sqrt( d));
        if (cc.CombatRound == 2)
          d =  Math.Sqrt( d);
        attval *= d;
        if ( num70 > 0.0 &  d != 0.0)
        {
          let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num70) * 100.0) / num70));
          let mut nr: i32 =  this.otherBonus.FindNr(cc.IList[attnr].IAttacker, tdata2: 2);
          if (nr <= -1)
          {
            this.otherBonus.AddWeight(cc.IList[attnr].IAttacker, tweight, 1, 2, CheckExistence: false);
          }
          else
          {
            int[] weight = this.otherBonus.Weight;
            int[] numArray11 = weight;
            let mut index30: i32 =  nr;
            let mut index31: i32 =  index30;
            let mut num71: i32 =  weight[index30] + tweight;
            numArray11[index31] = num71;
            int[] data1 = this.otherBonus.Data1;
            int[] numArray12 = data1;
            let mut index32: i32 =  nr;
            let mut index33: i32 =  index32;
            let mut num72: i32 =  data1[index32] + 1;
            numArray12[index33] = num72;
          }
        }
      }
      if (modNr == 14)
      {
        float num73 = attval;
        let mut num74: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[37];
        let mut num75: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[38];
        let mut num76: i32 =  100;
        if (num74 == 0)
        {
          if (num75 == 1)
            num76 = 66;
          if (num75 == 2)
            num76 = 33;
          if (num75 == 3)
            num76 = 66;
        }
        if (num74 == 1)
        {
          if (num75 == 3)
            num76 = 150;
          if (num75 == 4)
            num76 = 50;
        }
        if (num74 == 2)
        {
          if (num75 == 3)
            num76 = 50;
          if (num75 == 4)
            num76 = 33;
        }
        if (num74 == 3)
        {
          if (num75 == 3)
            num76 = 25;
          if (num75 == 4)
            num76 = 50;
        }
        if (num74 == 4)
        {
          if (num75 == 3)
            num76 = 66;
          if (num75 == 4)
            num76 = 75;
        }
        if (num74 == 5)
        {
          if (num75 == 1)
            num76 = 50;
          if (num75 == 2)
            num76 = 75;
        }
        attval =  ( attval *  num76 / 100.0);
        if ( num73 > 0.0)
        {
          let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num73) * 100.0) / num73));
          let mut nr: i32 =  this.otherBonus.FindNr(cc.IList[attnr].IAttacker, tdata2: 4);
          if (nr <= -1)
          {
            this.otherBonus.AddWeight(cc.IList[attnr].IAttacker, tweight, 1, 4, CheckExistence: false);
          }
          else
          {
            int[] weight = this.otherBonus.Weight;
            int[] numArray13 = weight;
            let mut index34: i32 =  nr;
            let mut index35: i32 =  index34;
            let mut num77: i32 =  weight[index34] + tweight;
            numArray13[index35] = num77;
            int[] data1 = this.otherBonus.Data1;
            int[] numArray14 = data1;
            let mut index36: i32 =  nr;
            let mut index37: i32 =  index36;
            let mut num78: i32 =  data1[index36] + 1;
            numArray14[index37] = num78;
          }
        }
      }
      if (modNr == 15)
      {
        float num79 = attval;
        let mut num80: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[37];
        let mut num81: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[38];
        let mut idValue1: i32 =  game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[41];
        if (this.airEnabled && idValue1 < 1 | game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater == 2 & game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[20] > 0)
          idValue1 = game.Data.SFTypeObj[cc.IList[attnr].ISFType].SFTypeVar[20];
        let mut idValue2: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].SFTypeVar[42];
        let mut num82: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotMM].GetData(0, idValue1, 1)));
        if (num82 < 1)
          num82 = 1;
        if (game.Data.SFTypeObj[cc.IList[attnr].ISFType].ArtRange > 0)
          num82 = (int) Math.Round( num82 / 3.0);
        let mut num83: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotMM].GetData(0, idValue2, 1)));
        let mut num84: i32 =  100;
        if (num80 != 6 & num80 != 0)
        {
          if (num81 != 4)
          {
            float num85 = 1f;
            if (num83 > 0)
              num85 =  num82 /  num83;
            if ( num85 > 10.0)
              num85 = 10f;
            if ( num85 < 1.0)
              num85 +=  ((1.0 -  num85) * 0.1);
            if ( num85 > 1.0)
              num85 = 1f;
            num84 = (int) Math.Round( (100f * num85));
          }
        }
        attval =  ( attval *  num84 / 100.0);
        if ( num79 > 0.0)
        {
          let mut tweight: i32 =  (int) Math.Round( ( (( attval -  num79) * 100.0) / num79));
          let mut nr: i32 =  this.otherBonus.FindNr(cc.IList[attnr].IAttacker, tdata2: 5);
          if (nr <= -1)
          {
            this.otherBonus.AddWeight(cc.IList[attnr].IAttacker, tweight, 1, 5, CheckExistence: false);
          }
          else
          {
            int[] weight = this.otherBonus.Weight;
            int[] numArray15 = weight;
            let mut index38: i32 =  nr;
            let mut index39: i32 =  index38;
            let mut num86: i32 =  weight[index38] + tweight;
            numArray15[index39] = num86;
            int[] data1 = this.otherBonus.Data1;
            int[] numArray16 = data1;
            let mut index40: i32 =  nr;
            let mut index41: i32 =  index40;
            let mut num87: i32 =  data1[index40] + 1;
            numArray16[index41] = num87;
          }
        }
      }
      return attval;
    }

    pub GetUnitFeatName: String(game: GameClass, id: i32)
    {
      unitFeatName: String = game.Data.StringListObj[this.slotUnitFeats].GetData(0, id, 2);
      if (unitFeatName.Length < 1)
        unitFeatName = "Unknown Unit Feat";
      return unitFeatName;
    }

    pub float IndividualCombatCall_DefValModder(
      modNr: i32,
      ref cc: CombatClass,
      game: GameClass,
      attnr2: i32,
      defnr: i32,
      bool isCounterAttack,
      float defval,
      ref s9: String)
    {
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      let mut jobSpecificId: i32 =  -1;
      let mut unitGroup1: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup;
      let mut index1: i32 =  cc.IList[defnr].IUnr;
      let mut historical1: i32 =  game.Data.UnitObj[index1].Historical;
      let mut id: i32 =  game.Data.RegimeObj[game.Data.UnitObj[cc.IList[defnr].IUnr].Regime].id;
      if (historical1 > -1)
        num1 = game.Data.HistoricalUnitObj[historical1].ID;
      bool flag1 = false;
      if (game.Data.HistoricalUnitObj[historical1].GiveHisVarValue(11) > 0)
        flag1 = true;
      if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotRegimes].GetData(0, id, 1))) > 1)
        flag1 = true;
      let mut num3: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotRegimes].GetData(0, id, 1)));
      for (; index1 > -1; index1 = game.Data.UnitObj[index1].HQ)
      {
        let mut historical2: i32 =  game.Data.UnitObj[index1].Historical;
        if (game.Data.UnitObj[index1].IsHQ)
        {
          if (game.Data.HistoricalUnitObj[historical2].Type == 8)
            jobSpecificId = game.Data.HistoricalUnitObj[historical2].ID;
          else if (game.Data.HistoricalUnitObj[historical2].Type >= 5)
            num2 = game.Data.HistoricalUnitObj[historical2].ID;
        }
      }
      let mut index2: i32 =  -1;
      if (num2 > -1)
        index2 = game.HandyFunctionsObj.GetHistoricalUnitByID(num2);
      let mut characterId1: i32 =  game.EventRelatedObj.Helper_GetCharacterId(id, 3, num2, -1, this.slotChar);
      let mut characterId2: i32 =  game.EventRelatedObj.Helper_GetCharacterId(id, 4, jobSpecificId, -1, this.slotChar);
      let mut num4: i32 =  0;
      let mut num5: i32 =  -1;
      let mut num6: i32 =  1;
      index3: i32;
      num7: i32;
      do
      {
        let mut index4: i32 =  -1;
        let mut index5: i32 =  -1;
        if (num6 == 1)
        {
          index4 = index3;
          index5 = defnr;
        }
        if (num6 == 2)
        {
          index4 = defnr;
          index5 = index3;
        }
        if (!Information.IsNothing( cc.IList[index4].IunitFeat))
        {
          let mut counter: i32 =  cc.IList[index4].IunitFeat.Counter;
          for (let mut index6: i32 =  0; index6 <= counter; index6 += 1)
          {
            if (cc.IList[index4].IunitFeat.Id[index6] > 0 && cc.IList[cc.FindISlot(cc.IList[index4].IunitFeat.Data1[index6])].IKilled == 0 & cc.IList[cc.FindISlot(cc.IList[index4].IunitFeat.Data1[index6])].IRetreated == 0 & cc.IList[cc.FindISlot(cc.IList[index4].IunitFeat.Data1[index6])].IRetreat == 0)
            {
              let mut idValue1: i32 =  cc.IList[index4].IunitFeat.Id[index6];
              data: String = game.Data.StringListObj[this.slotUnitFeats].GetData(0, idValue1, 2);
              let mut length: i32 =  game.Data.StringListObj[this.slotBehaviour].Length;
              for (let mut index7: i32 =  0; index7 <= length; index7 += 1)
              {
                let mut num8: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 0]));
                if ((int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 1])) == idValue1)
                {
                  let mut num9: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 3]));
                  let mut num10: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 4]));
                  let mut num11: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 5]));
                  let mut num12: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 6]));
                  let mut idValue2: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 7]));
                  index8: i32;
                  str1: String = game.Data.StringListObj[this.slotBehaviour].Data[index7, index8];
                  let mut num13: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 8]));
                  let mut num14: i32 =  (int) Math.Round(Conversion.Val(game.Data.StringListObj[this.slotBehaviour].Data[index7, 9]));
                  bool flag2 = false;
                  if (cc.CombatType == 1 & num9 == 1)
                    flag2 = true;
                  if (cc.CombatType == 3 & num10 == 1)
                    flag2 = true;
                  if (num10 == 1 & cc.game.Data.SFTypeObj[cc.IList[index4].ISFType].ArtRange > 0)
                    flag2 = true;
                  if (!(num11 == 3 | num11 == 2))
                    flag2 = false;
                  if (num12 > -1 & num12 != game.Data.SFTypeObj[cc.IList[index5].ISFType].UnitGroup)
                    flag2 = false;
                  if (flag2)
                  {
                    if (modNr == 12)
                    {
                      if (num8 == 11 && index4 == defnr)
                      {
                        s9 = "";
                        if (idValue2 > 0)
                        {
                          let mut num15: i32 =  (int) Math.Round( defval *  idValue2 / 100.0);
                          defval +=  num15;
                          s9 = data + " provided " + idValue2.ToString() + "% extra defensive points. ";
                        }
                        if (num13 > 0)
                        {
                          let mut num16: i32 =  num13;
                          defval +=  num16;
                          s9 = s9 + data + " provided " + num13.ToString() + " extra defensive points.";
                        }
                      }
                      if (num8 == 37 && index4 == defnr)
                      {
                        s9 = "";
                        if (idValue2 > 0)
                        {
                          let mut ientrench: i32 =  cc.IList[defnr].IEntrench;
                          num7 = (int) Math.Round( (ientrench * idValue2) / 100.0);
                          let mut num17: i32 =  (int) Math.Round( defval *  ientrench / 100.0);
                          defval +=  num17;
                          s9 = data + " provided " + num17.ToString() + "% extra defensive points. ";
                        }
                      }
                      if (num8 == 15 && index4 == defnr)
                      {
                        let mut num18: i32 =  (int) Math.Round( (game.Data.SFObj[cc.IList[defnr].ISFNr].Rdn * idValue2) / 100.0);
                        let mut num19: i32 =  num18 - cc.IList[defnr].IRdn;
                        if (cc.IList[defnr].IRdn < num18)
                          cc.IList[defnr].IRdn = num18;
                        if (num19 > 0)
                          s9 = data + " increased our readiness with " + num19.ToString() + "points back to " + idValue2.ToString() + "% of original readiness.";
                      }
                    }
                    else if (modNr != 12 && num8 == 12 & index4 == defnr)
                    {
                      str2: String = idValue2 <= 0 ? "on all skills." : "on skill '" + game.Data.StringListObj[this.slotSkillTypes].GetData(0, idValue2, 1) + "'";
                      s9 = data + " gives a skill modification of " + num13.ToString() + " points " + str2;
                      num5 = idValue2;
                      num4 = num13;
                    }
                  }
                }
              }
            }
          }
        }
        num6 += 1;
      }
      while (num6 <= 2);
      if (characterId1 > 0 & game.HandyFunctionsObj.Gethqpow(cc.IList[defnr].IUnr) > DrawMod.RandyNumber.Next(0, 100))
      {
        float num20 = defval;
        let mut tdata2: i32 =  0;
        if (game.Data.SFTypeObj[cc.IList[defnr].ISFType].Theater < 2)
        {
          if (modNr == 8)
          {
            let mut num21: i32 =  this.CachedSkillRoll(game, characterId1, 24);
            if (24 == num5 | num5 == -1)
              num21 += num4;
            this.SL.AddWeight(characterId1, 1, 24, CheckData1Existence: true);
            if (num21 > 100)
              defval +=  ( defval *  (num21 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 24;
          }
          if (modNr == 1 & unitGroup1 == 3 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].ArtRange < 1)
          {
            let mut num22: i32 =  this.CachedSkillRoll(game, characterId1, 31);
            if (31 == num5 | num5 == -1)
              num22 += num4;
            this.SL.AddWeight(characterId1, 1, 31, CheckData1Existence: true);
            if (num22 > 100)
              defval +=  ( defval *  (num22 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 31;
          }
          if (modNr == 2 & (unitGroup1 == 1 | unitGroup1 == 3 & game.Data.SFTypeObj[cc.IList[defnr].ISFType].ArtRange > 0))
          {
            let mut num23: i32 =  this.CachedSkillRoll(game, characterId1, 32);
            if (32 == num5 | num5 == -1)
              num23 += num4;
            this.SL.AddWeight(characterId1, 1, 32, CheckData1Existence: true);
            if (num23 > 100)
              defval +=  ( defval *  (num23 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 32;
          }
          if (modNr == 3 & unitGroup1 == 0)
          {
            let mut num24: i32 =  this.CachedSkillRoll(game, characterId1, 33);
            if (33 == num5 | num5 == -1)
              num24 += num4;
            this.SL.AddWeight(characterId1, 1, 33, CheckData1Existence: true);
            if (num24 > 100)
              defval +=  ( defval *  (num24 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 33;
          }
          if (modNr == 7 & cc.IList[defnr].IAttacker == 0)
          {
            let mut num25: i32 =  this.CachedSkillRoll(game, characterId1, 37);
            if (37 == num5 | num5 == -1)
              num25 += num4;
            this.SL.AddWeight(characterId1, 1, 37, CheckData1Existence: true);
            if (num25 > 100)
              defval +=  ( defval *  (num25 - 100) / 100.0) * this.SKILLEFFECT;
            tdata2 = 37;
          }
          if (modNr == 4 & cc.IList[defnr].IRetreat > 0)
          {
            let mut num26: i32 =  this.CachedSkillRoll(game, characterId1, 13);
            if (13 == num5 | num5 == -1)
              num26 += num4;
            if (num26 > 100)
            {
              this.SL.AddWeight(characterId1, 1, 13, CheckData1Existence: true);
              defval += defval * 3f;
            }
            tdata2 = 13;
          }
        }
        let mut tweight: i32 =  (int) Math.Round( ( (( defval -  num20) * 100.0) / num20));
        if (tweight > 0)
          tweight = tweight;
        if (tdata2 > 0)
        {
          let mut nr: i32 =  this.logLeaderBonusDef.FindNr(characterId1, tdata2: tdata2);
          if (nr <= -1)
          {
            this.logLeaderBonusDef.AddWeight(characterId1, tweight, 1, tdata2, CheckExistence: false);
          }
          else
          {
            int[] weight = this.logLeaderBonusDef.Weight;
            int[] numArray1 = weight;
            let mut index9: i32 =  nr;
            let mut index10: i32 =  index9;
            let mut num27: i32 =  weight[index9] + tweight;
            numArray1[index10] = num27;
            int[] data1 = this.logLeaderBonusDef.Data1;
            int[] numArray2 = data1;
            let mut index11: i32 =  nr;
            let mut index12: i32 =  index11;
            let mut num28: i32 =  data1[index11] + 1;
            numArray2[index12] = num28;
          }
        }
      }
      if (characterId2 > 0)
      {
        float num29 = defval;
        let mut tdata2: i32 =  0;
        if (modNr == 9)
        {
          let mut num30: i32 =  this.CachedSkillRoll(game, characterId2, 18);
          if (18 == num5 | num5 == -1)
            num30 += num4;
          this.SL.AddWeight(characterId2, 1, 18, CheckData1Existence: true);
          if (num30 > 100)
            defval +=  ( defval *  (num30 - 100) / 100.0) * this.SKILLEFFECT;
          tdata2 = 18;
        }
        let mut tweight: i32 =  (int) Math.Round( ( (( defval -  num29) * 100.0) / num29));
        if (tdata2 > 0)
        {
          let mut nr: i32 =  this.logLeaderBonusDef.FindNr(characterId2, tdata2: tdata2);
          if (nr <= -1)
          {
            this.logLeaderBonusDef.AddWeight(characterId2, tweight, 1, tdata2, CheckExistence: false);
          }
          else
          {
            int[] weight = this.logLeaderBonusDef.Weight;
            int[] numArray3 = weight;
            let mut index13: i32 =  nr;
            let mut index14: i32 =  index13;
            let mut num31: i32 =  weight[index13] + tweight;
            numArray3[index14] = num31;
            int[] data1 = this.logLeaderBonusDef.Data1;
            int[] numArray4 = data1;
            let mut index15: i32 =  nr;
            let mut index16: i32 =  index15;
            let mut num32: i32 =  data1[index15] + 1;
            numArray4[index16] = num32;
          }
        }
      }
      if (modNr == 10)
      {
        float num33 = defval;
        let mut iunr: i32 =  cc.IList[defnr].IUnr;
        if (index2 > -1)
        {
          if (cc.AttackerRegime == game.Data.UnitObj[iunr].Regime)
          {
            let mut unitGroup2: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup;
            if (unitGroup2 == 0)
              num7 = game.Data.HistoricalUnitObj[index2].GiveHisVarValue(22);
            else if (unitGroup2 == 1 | unitGroup2 == 6)
              num7 = game.Data.HistoricalUnitObj[index2].GiveHisVarValue(23);
            else if (unitGroup2 == 2 | unitGroup2 == 3)
              num7 = game.Data.HistoricalUnitObj[index2].GiveHisVarValue(24);
          }
          if (cc.DefenderRegime == game.Data.UnitObj[iunr].Regime)
          {
            let mut unitGroup3: i32 =  game.Data.SFTypeObj[cc.IList[defnr].ISFType].UnitGroup;
            if (unitGroup3 == 0)
              num7 = game.Data.HistoricalUnitObj[index2].GiveHisVarValue(25);
            else if (unitGroup3 == 1 | unitGroup3 == 6)
              num7 = game.Data.HistoricalUnitObj[index2].GiveHisVarValue(26);
            else if (unitGroup3 == 2 | unitGroup3 == 3)
              num7 = game.Data.HistoricalUnitObj[index2].GiveHisVarValue(27);
          }
          let mut num34: i32 =  game.Data.HistoricalUnitObj[index2].GiveHisVarValue(40);
          if (num34 == 4 && DrawMod.RandyNumber.Next(0, 1000) < 200)
            defval = 0.0f;
          if (num34 == 3 & cc.IList[defnr].IRetreat == cc.CombatRound - 1 & cc.IList[defnr].IRetreat > 0 && DrawMod.RandyNumber.Next(0, 1000) < 50)
            defval = 0.0f;
          if (num34 == 5 & cc.IList[defnr].IRetreat == cc.CombatRound - 1 & cc.IList[defnr].IRetreat > 0 && DrawMod.RandyNumber.Next(0, 1000) < 750)
            defval = 0.0f;
          if (num34 == 6 & cc.IList[defnr].IRetreat == cc.CombatRound - 1 & cc.IList[defnr].IRetreat > 0 && DrawMod.RandyNumber.Next(0, 1000) < 500)
            defval = 0.0f;
          let mut tweight: i32 =  (int) Math.Round( ( (( defval -  num33) * 100.0) / num33));
          let mut nr: i32 =  this.otherBonusDef.FindNr(cc.IList[index3].IAttacker, tdata2: 1);
          if (nr <= -1)
          {
            this.otherBonusDef.AddWeight(cc.IList[index3].IAttacker, tweight, 1, 1, CheckExistence: false);
          }
          else
          {
            int[] weight = this.otherBonusDef.Weight;
            int[] numArray5 = weight;
            let mut index17: i32 =  nr;
            let mut index18: i32 =  index17;
            let mut num35: i32 =  weight[index17] + tweight;
            numArray5[index18] = num35;
            int[] data1 = this.otherBonusDef.Data1;
            int[] numArray6 = data1;
            let mut index19: i32 =  nr;
            let mut index20: i32 =  index19;
            let mut num36: i32 =  data1[index19] + 1;
            numArray6[index20] = num36;
          }
        }
      }
      return defval;
    }

    pub fn UnitLost(ref cc: CombatClass, game: GameClass, unr: i32) => this.PersonalCombatRoll(ref cc, game, unr, -1, 75, "", true, false);

    pub IndividualCombatCall_FirstAttackOfRound: i32(
      ref cc: CombatClass,
      game: GameClass,
      attnr: i32,
      defnr: i32,
      ref s9: String)
    {
      if (cc.previewMode)
        return 0;
      let mut isfType: i32 =  cc.IList[attnr].ISFType;
      let mut num1: i32 =  game.Data.SFTypeObj[isfType].SFTypeVar[95];
      if ( cc.IList[attnr].ILisAmmoMod < 1.0)
        num1 = (int) Math.Round( ( num1 * cc.IList[attnr].ILisAmmoMod));
      if (num1 < 1)
        return 0;
      data: DataClass = game.Data;
      str: String = "rad".to_owned();
      ref local: String = ref str;
      let mut libVar: i32 =  data.FindLibVar(ref local, "SE_Data");
      let mut num2: i32 =  cc.TargetX - 4;
      let mut num3: i32 =  cc.TargetX + 4;
      for (let mut index1: i32 =  num2; index1 <= num3; index1 += 1)
      {
        let mut num4: i32 =  cc.TargetY - 4;
        let mut num5: i32 =  cc.TargetY + 4;
        for (let mut index2: i32 =  num4; index2 <= num5; index2 += 1)
        {
          let mut x2: i32 =  index1;
          let mut y2: i32 =  index2;
          if (x2 > game.Data.MapObj[0].MapWidth)
            x2 -= game.Data.MapObj[0].MapWidth + 1;
          if (x2 < 0)
            x2 = game.Data.MapObj[0].MapWidth + 1 + x2;
          if (y2 >= 0 & y2 <= game.Data.MapObj[0].MapHeight)
          {
            let mut num6: i32 =  game.HandyFunctionsObj.Distance(cc.TargetX, cc.TargetY, 0, x2, y2, 0, 5);
            let mut num7: i32 =  0;
            if (num6 < 5)
            {
              if (num6 == 0)
                num7 = num1;
              if (num6 == 1)
                num7 = (int) Math.Round( num1 / 4.0);
              if (num6 == 2)
                num7 = (int) Math.Round( num1 / 16.0);
              if (num6 == 3)
                num7 = (int) Math.Round( num1 / 64.0);
              if (num6 == 4)
                num7 = (int) Math.Round( num1 / 256.0);
            }
            if (num7 > 0)
            {
              let mut hexLibVarValue: i32 =  game.Data.MapObj[0].HexObj[x2, y2].GetHexLibVarValue(libVar);
              let mut num8: i32 =  hexLibVarValue + num7;
              let mut num9: i32 =  0;
              if (hexLibVarValue <= 1000)
              {
                num9 = 1000 - hexLibVarValue;
                if (num7 < num9)
                  num9 = num7;
                num7 -= num9;
              }
              if (num8 > 1000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 2000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 3000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 4000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 5000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 6000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 7000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 8000)
                num7 = (int) Math.Round( num7 / 2.0);
              if (hexLibVarValue + num7 > 9000)
                num7 = (int) Math.Round( num7 / 2.0);
              let mut num10: i32 =  num7 + num9;
              let mut tValue: i32 =  hexLibVarValue + num10;
              game.Data.MapObj[0].HexObj[x2, y2].SetHexLibVarValue(libVar, tValue);
            }
          }
        }
      }
      s9 = "Hex irradiated with " + num1.ToString() + " RAD.";
      if (num1 > this.rememberRadPts)
        this.rememberRadPts = num1;
      return num1;
    }

    pub fn AlterCombatLastRound(ref cc: CombatClass) -> i32 => cc.CombatType == 13 ? 3 : -1;
  }
}
