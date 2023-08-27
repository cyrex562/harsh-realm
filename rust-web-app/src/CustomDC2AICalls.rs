// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.CustomDC2AICalls
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;

namespace WindowsApplication1
{
  pub class CustomDC2AICalls
  {
    pub ai: DC2AIClass;
    pub data: DataClass;
    pub slotRegimeKeys: i32;
    pub slotOobType: i32;
    pub slotRegimes: i32;
    pub slotregregkeys: i32;
    pub dataLib: String;
    pub bool[,] tempActuallyNotAtWarForMove;
    pub bool[,] tempActuallyNotAtWarForAttack;

    pub CustomDC2AICalls(ref tai: DC2AIClass)
    {
      this.dataLib = "SE_Data".to_owned();
      this.ai = tai;
      this.data = this.ai.game.Data;
      this.slotRegimes = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 143, 0, 0));
      this.slotRegimeKeys = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 210, 0, 0));
      this.slotregregkeys = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 275, 0, 0));
      this.slotOobType = DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 236, 0, 0));
      this.tempActuallyNotAtWarForMove = new bool[this.data.RegimeCounter + 1, this.data.RegimeCounter + 1];
      this.tempActuallyNotAtWarForAttack = new bool[this.data.RegimeCounter + 1, this.data.RegimeCounter + 1];
      let mut regimeCounter1: i32 =  this.data.RegimeCounter;
      for (let mut index1: i32 =  0; index1 <= regimeCounter1; index1 += 1)
      {
        let mut regimeCounter2: i32 =  this.data.RegimeCounter;
        for (let mut index2: i32 =  0; index2 <= regimeCounter2; index2 += 1)
        {
          let mut num1: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[index1].id, 1, this.data.RegimeObj[index2].id, 2, "dipClear", 3)));
          if ( Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[index2].id, 1, this.data.RegimeObj[index1].id, 2, "dipClear", 3))) == 1 & num1 == 0)
            num1 = 1;
          let mut num2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[index1].id, 1)));
          this.tempActuallyNotAtWarForMove[index1, index2] = false;
          this.tempActuallyNotAtWarForAttack[index1, index2] = false;
          if (this.data.RegimeObj[index1].RegimeRel[index2] == 0 && num2 == 2)
          {
            if (num1 < 1)
            {
              let mut num3: i32 =  Math.Max( Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[index1].id, 1, this.data.RegimeObj[index2].id, 2, "relation", 3))),  Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[index2].id, 1, this.data.RegimeObj[index1].id, 2, "relation", 3))));
              this.tempActuallyNotAtWarForAttack[index1, index2] = true;
              this.tempActuallyNotAtWarForMove[index1, index2] = num3 >= 35;
            }
            else
            {
              this.tempActuallyNotAtWarForMove[index1, index2] = false;
              this.tempActuallyNotAtWarForAttack[index1, index2] = false;
            }
          }
          this.tempActuallyNotAtWarForMove[index2, index1] = this.tempActuallyNotAtWarForMove[index1, index2];
          this.tempActuallyNotAtWarForAttack[index2, index1] = this.tempActuallyNotAtWarForAttack[index1, index2];
        }
      }
      if (Operators.CompareString(this.data.RegimeObj[this.data.Turn].Name, "Abaro Territory", false) != 0)
        ;
    }

    pub fn CustomAfterInitialization() => this.PlayPostures();

    pub float CustomAllowedAsStrategicReserve(byvalunr: i32) => -1f;

    pub float StrategicReserveModForUnit(unr: i32)
    {
      let mut historical1: i32 =  this.data.UnitObj[unr].Historical;
      let mut hq: i32 =  this.data.UnitObj[unr].HQ;
      if (this.data.HistoricalUnitObj[historical1].GiveHisVarValue(11) > 0 && !this.data.UnitObj[unr].IsHQ)
        return 0.2f;
      if (this.data.UnitObj[unr].TempCategory == 5)
        return 1f;
      if (this.data.UnitObj[unr].IsHQ)
        return 99999f;
      if (hq > -1)
      {
        let mut historical2: i32 =  this.data.UnitObj[hq].Historical;
        let mut idValue: i32 =  this.data.HistoricalUnitObj[historical1].GiveHisVarValue(1);
        if (idValue > 0 & historical2 > -1)
        {
          if ( Math.Round(Conversion.Val(this.data.StringListObj[this.slotOobType].GetData(0, idValue, 4))) == 1)
          {
            if (this.data.HistoricalUnitObj[historical2].Type >= 7)
              return 2f;
            return this.data.HistoricalUnitObj[historical2].Type >= 5 ? 30f : 9999f;
          }
          return this.data.HistoricalUnitObj[historical2].Type >= 7 ? 30f : 9999f;
        }
      }
      return 9999f;
    }

    pub fn PlayPostures()
    {
      str1: String = "8101_AI_Play_Postures".to_owned();
      let mut id1: i32 =  this.data.RegimeObj[this.data.Turn].id;
      let mut stringListById1: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 277, 0, 0));
      let mut stringListById2: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 169, 0, 0));
      let mut stringListById3: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 168, 0, 0));
      let mut num1: i32 =  50;
      let mut num2: i32 =  100;
      this.ai.ClearLog();
      this.ai.AddLog("");
      this.ai.AddLog(str1);
      this.ai.AddLog("");
      this.ai.AddLog("OHQs available:");
      SimpleList simpleList1 = SimpleList::new();
      let mut unitCounter1: i32 =  this.data.UnitCounter;
      for (let mut tid: i32 =  0; tid <= unitCounter1; tid += 1)
      {
        if (this.data.UnitObj[tid].HQ > -1 & this.data.UnitObj[tid].IsHQ && this.data.UnitObj[tid].Regime == this.data.Turn & this.data.UnitObj[tid].PreDef == -1 & this.data.UnitObj[tid].Historical > -1 && this.data.HistoricalUnitObj[this.data.UnitObj[tid].Historical].TempVar1 < 1)
        {
          simpleList1.Add(tid, 1);
          let mut num3: i32 =  this.data.HistoricalUnitObj[this.data.UnitObj[tid].Historical].GiveHisVarValue(21);
          this.ai.AddLog(Conversions.ToString(Operators.CompareString("-OHQ: " + this.data.UnitObj[tid].Name + " (posture=" + num3.ToString(), ")", false) == 0));
        }
      }
      this.ai.AddLog("");
      this.data.StringListObj[stringListById2].SetData(0, "REGIMEID", 1, id1, true);
      this.data.StringListObj[stringListById2].SetData(0, "SOURCEREGIMEID", 1, id1, true);
      this.data.StringListObj[stringListById2].SetData(0, "ROUND", 1, this.data.Round, true);
      SimpleList simpleList2 = SimpleList::new();
      let mut length: i32 =  this.data.StringListObj[stringListById1].Length;
      num4: i32;
      for (let mut index: i32 =  0; index <= length; index += 1)
      {
        num4 =  Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 0]));
        if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 12])) == 7 &  Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 7])) == 3)
        {
          str2: String = this.data.StringListObj[stringListById1].Data[index, 1];
          str3: String = this.data.StringListObj[stringListById1].Data[index, 6];
          num5: i32;
          if (str3.Length > 0)
          {
            eventRelatedObj: EventRelatedClass = DrawMod.TGame.EventRelatedObj;
            let mut id2: i32 =  this.data.StringListObj[stringListById3].ID;
            let mut id3: i32 =  this.data.StringListObj[stringListById2].ID;
            logicString: String = str3;
            Random random = (Random) null;
            ref Random local = ref random;
            num5 = eventRelatedObj.CheckLogicStringStart(id2, id3, logicString, 0, ref local);
          }
          else
            num5 = 1;
          if (num5 >= 1)
          {
            simpleList2.Add(num4, 1, 1);
            this.ai.AddLog("-Posture available: #" + num4.ToString() + " : " + str2);
          }
        }
      }
      this.ai.AddLog("");
      let mut counter1: i32 =  simpleList1.Counter;
      for (let mut index1: i32 =  0; index1 <= counter1; index1 += 1)
      {
        let mut index2: i32 =  simpleList1.Id[index1];
        let mut num6: i32 =  this.data.HistoricalUnitObj[this.data.UnitObj[index2].Historical].GiveHisVarValue(21);
        let mut num7: i32 =  0;
        if (num6 > 0)
        {
          if (DrawMod.RandyNumber.Next(0, 100) < num1)
            num7 = 1;
        }
        else if (DrawMod.RandyNumber.Next(0, 100) < num2)
          num7 = 1;
        if (num7 == 1)
        {
          let mut num8: i32 =  0;
          let mut num9: i32 =  0;
          let mut num10: i32 =  0;
          let mut num11: i32 =  0;
          let mut num12: i32 =  0;
          let mut num13: i32 =  0;
          let mut num14: i32 =  0;
          let mut unitCounter2: i32 =  this.data.UnitCounter;
          for (let mut index3: i32 =  0; index3 <= unitCounter2; index3 += 1)
          {
            if (this.data.UnitObj[index3].HQ == index2)
            {
              let mut sfCount: i32 =  this.data.UnitObj[index3].SFCount;
              for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
              {
                let mut sf: i32 =  this.data.UnitObj[index3].SFList[index4];
                let mut type: i32 =  this.data.SFObj[sf].Type;
                let mut qty: i32 =  this.data.SFObj[sf].Qty;
                let mut unitGroup: i32 =  this.data.SFTypeObj[type].UnitGroup;
                let mut num15: i32 =  (this.data.SFTypeObj[type].SFTypeVar[30] + this.data.SFTypeObj[type].SFTypeVar[31] + (this.data.SFTypeObj[type].SFTypeVar[32] + this.data.SFTypeObj[type].SFTypeVar[33])) * qty;
                let mut num16: i32 =  this.data.SFObj[sf].Xp * qty;
                num14 += num16;
                num10 += qty;
                if (unitGroup == 0)
                {
                  if (this.data.SFTypeObj[type].ArtRange > 0)
                  {
                    num13 += num15;
                  }
                  else
                  {
                    switch (unitGroup)
                    {
                      case 0:
                        num9 += num15;
                        break;
                      case 1:
                        num13 += num15;
                        break;
                      case 2:
                        num12 += num15;
                        break;
                      default:
                        if (unitGroup == 3 | unitGroup == 4)
                        {
                          num11 += num15;
                          break;
                        }
                        break;
                    }
                  }
                  num8 += num15;
                }
              }
            }
          }
          let mut num17: i32 =   Math.Round( num14 /  num10);
          let mut tweight1: i32 =   Math.Round( (100 * num13) /  num8);
          let mut tweight2: i32 =   Math.Round( (100 * num9) /  num8);
          let mut tweight3: i32 =   Math.Round( (100 * num12) /  num8);
          let mut tweight4: i32 =   Math.Round( (100 * num11) /  num8);
          simpleList2.AddWeight(601, tweight2);
          simpleList2.AddWeight(602, tweight2);
          simpleList2.AddWeight(603, tweight2);
          simpleList2.AddWeight(604, tweight2);
          simpleList2.AddWeight(607, tweight2);
          simpleList2.AddWeight(608, tweight2);
          simpleList2.AddWeight(609, tweight2);
          simpleList2.AddWeight(610, tweight2);
          simpleList2.AddWeight(611, tweight2);
          simpleList2.AddWeight(612, tweight2);
          simpleList2.AddWeight(615, tweight2 * 2);
          simpleList2.AddWeight(616, tweight2);
          simpleList2.AddWeight(619, tweight2);
          simpleList2.AddWeight(621, tweight2 * 2);
          simpleList2.AddWeight(622, tweight2 * 2);
          simpleList2.AddWeight(607, tweight4);
          simpleList2.AddWeight(608, tweight4);
          simpleList2.AddWeight(609, tweight4);
          simpleList2.AddWeight(610, tweight4);
          simpleList2.AddWeight(612, tweight4);
          simpleList2.AddWeight(616, tweight4);
          simpleList2.AddWeight(618, tweight4 * 2);
          simpleList2.AddWeight(619, tweight4);
          simpleList2.AddWeight(603, tweight1);
          simpleList2.AddWeight(607, tweight1);
          simpleList2.AddWeight(609, tweight1);
          simpleList2.AddWeight(610, tweight1);
          simpleList2.AddWeight(611, tweight1);
          simpleList2.AddWeight(616, tweight1);
          simpleList2.AddWeight(610, tweight3);
          let mut counter2: i32 =  simpleList2.Counter;
          for (let mut index5: i32 =  0; index5 <= counter2; index5 += 1)
          {
            let mut num18: i32 =  new Random( Math.Round( this.data.GameID / 1000.0 *  id1 *  simpleList2.Id[index5])).Next(3, 8);
            simpleList2.Weight[index5] =  Math.Round( (simpleList2.Weight[index5] * num18) / 5.0);
          }
          let mut index6: i32 =  -1;
          let mut counter3: i32 =  this.ai.frontList.Counter;
          for (let mut index7: i32 =  0; index7 <= counter3; index7 += 1)
          {
            if (this.ai.frontList.Front[index7].strictHQs.counter > -1)
            {
              let mut counter4: i32 =  this.ai.frontList.Front[index7].strictHQs.counter;
              for (let mut index8: i32 =  0; index8 <= counter4; index8 += 1)
              {
                if (this.ai.frontList.Front[index7].strictHQs.unr[index8] == index2)
                  index6 = index7;
              }
            }
          }
          if (index6 > -1)
          {
            AIFront aiFront = this.ai.frontList.Front[index6];
            if (aiFront.FrontType == 11)
            {
              simpleList2.MultiplyWeight(601, 2f);
              simpleList2.MultiplyWeight(604, 2f);
              simpleList2.MultiplyWeight(607, 2f);
              simpleList2.MultiplyWeight(616, 2f);
              simpleList2.MultiplyWeight(622, 2f);
            }
            else if (aiFront.FrontType == 12)
            {
              simpleList2.MultiplyWeight(608, 2f);
              simpleList2.MultiplyWeight(611, 2f);
              simpleList2.MultiplyWeight(615, 4f);
              simpleList2.MultiplyWeight(619, 2f);
              simpleList2.MultiplyWeight(621, 4f);
            }
            else if (aiFront.FrontType == 1)
            {
              if (aiFront.Stance == 3)
              {
                simpleList2.MultiplyWeight(608, 2f);
                simpleList2.MultiplyWeight(611, 2f);
                simpleList2.MultiplyWeight(615, 2f);
                simpleList2.MultiplyWeight(619, 2f);
                simpleList2.MultiplyWeight(621, 2f);
              }
              else if (aiFront.Stance == 2)
              {
                simpleList2.MultiplyWeight(601, 2f);
                simpleList2.MultiplyWeight(604, 2f);
                simpleList2.MultiplyWeight(607, 2f);
                simpleList2.MultiplyWeight(616, 2f);
                simpleList2.MultiplyWeight(622, 2f);
              }
              else if (aiFront.Stance == 1)
              {
                simpleList2.MultiplyWeight(602, 2f);
                simpleList2.MultiplyWeight(618, 2f);
              }
              else if (aiFront.Stance == 4)
              {
                simpleList2.MultiplyWeight(602, 2f);
                simpleList2.MultiplyWeight(618, 2f);
                simpleList2.MultiplyWeight(603, 2.5f);
                simpleList2.MultiplyWeight(610, 2.5f);
                simpleList2.MultiplyWeight(612, 2.5f);
              }
            }
            else if (aiFront.FrontType == 2 | aiFront.FrontType == 3)
            {
              simpleList2.MultiplyWeight(603, 2f);
              simpleList2.MultiplyWeight(610, 2f);
              simpleList2.MultiplyWeight(612, 2f);
            }
            if (aiFront.vpScoreAveragePercent > 80)
            {
              simpleList2.MultiplyWeight(604, 2.5f);
              simpleList2.MultiplyWeight(611, 2.5f);
              simpleList2.MultiplyWeight(616, 2.5f);
              simpleList2.MultiplyWeight(621, 2.5f);
              simpleList2.MultiplyWeight(622, 2.5f);
            }
            else if (aiFront.vpScoreAveragePercent > 70)
            {
              simpleList2.MultiplyWeight(604, 2f);
              simpleList2.MultiplyWeight(611, 2f);
              simpleList2.MultiplyWeight(616, 2f);
              simpleList2.MultiplyWeight(621, 2f);
              simpleList2.MultiplyWeight(622, 2f);
            }
            else if (aiFront.vpScoreAveragePercent > 60)
            {
              simpleList2.MultiplyWeight(604, 1.5f);
              simpleList2.MultiplyWeight(611, 1.5f);
              simpleList2.MultiplyWeight(616, 1.5f);
              simpleList2.MultiplyWeight(621, 1.5f);
              simpleList2.MultiplyWeight(622, 1.5f);
            }
            if (aiFront.Stance == 3)
            {
              if ( aiFront.UnitCountRatio > 3.0)
                simpleList2.MultiplyWeight(619, 3f);
              else if ( aiFront.UnitCountRatio > 2.0)
                simpleList2.MultiplyWeight(619, 2f);
              else if ( aiFront.UnitCountRatio > 1.5)
                simpleList2.MultiplyWeight(619, 1.75f);
              else if ( aiFront.UnitCountRatio > 1.32000005245209)
                simpleList2.MultiplyWeight(619, 1.5f);
              else if ( aiFront.UnitCountRatio > 1.0)
                simpleList2.MultiplyWeight(619, 1.25f);
              else if ( aiFront.UnitCountRatio < 0.66)
                simpleList2.MultiplyWeight(619, 0.5f);
            }
            if ( aiFront.OrigAverageStrength < 5.0)
            {
              if ( aiFront.OrigAverageStrength >= 4.0)
              {
                simpleList2.MultiplyWeight(621, 0.66f);
                simpleList2.MultiplyWeight(623, 0.66f);
              }
              else if ( aiFront.OrigAverageStrength >= 3.33)
              {
                simpleList2.MultiplyWeight(615, 0.66f);
                simpleList2.MultiplyWeight(617, 0.66f);
                simpleList2.MultiplyWeight(621, 0.2f);
                simpleList2.MultiplyWeight(623, 0.2f);
              }
              else
              {
                simpleList2.MultiplyWeight(615, 0.2f);
                simpleList2.MultiplyWeight(617, 0.2f);
                simpleList2.MultiplyWeight(621, 0.01f);
                simpleList2.MultiplyWeight(623, 0.01f);
              }
            }
            if (aiFront.Stance == 3 | aiFront.Stance == 2)
            {
              let mut num19: i32 =  DrawMod.TGame.EventRelatedObj.CheckEnemyTroopsCloseBy(12, this.data.UnitObj[index2].X, this.data.UnitObj[index2].Y, 0);
              if (this.data.Turn == 5)
                index1 = index1;
              if (num19 < 1)
              {
                simpleList2.MultiplyWeight(609, 10f);
              }
              else
              {
                let mut num20: i32 =  DrawMod.TGame.EventRelatedObj.CheckEnemyTroopsCloseBy(8, this.data.UnitObj[index2].X, this.data.UnitObj[index2].Y, 0);
                if (num20 < 1)
                  simpleList2.MultiplyWeight(609, 5f);
                else if (num20 >= 10)
                  simpleList2.MultiplyWeight(609, 0.1f);
              }
            }
            if (num17 > 40)
              simpleList2.MultiplyWeight(609, 0.01f);
            else if (num17 > 30)
              simpleList2.MultiplyWeight(609, 0.1f);
            else if (num17 > 20)
              simpleList2.MultiplyWeight(609, 0.5f);
          }
          for (let mut counter5: i32 =  simpleList2.Counter; counter5 >= 0; counter5 += -1)
          {
            if (simpleList2.Data1[counter5] < 1)
              simpleList2.RemoveNr(counter5);
          }
          simpleList2.ReverseSortHighSpeed();
          simpleList2.removeWeight0orLower();
          if (simpleList2.Counter > -1)
          {
            let mut num21: i32 =  simpleList2.Id[0];
            num4 = num21;
            let mut historical: i32 =  this.data.UnitObj[index2].Historical;
            this.ai.AddLog("Chosing Posture for " + this.data.UnitObj[index2].Name);
            let mut counter6: i32 =  simpleList2.Counter;
            for (let mut index9: i32 =  0; index9 <= counter6; index9 += 1)
              this.ai.AddLog("Posture #" + simpleList2.Id[index9].ToString() + " has weight = " + simpleList2.Weight[index9].ToString());
            if (this.data.HistoricalUnitObj[historical].GiveHisVarValue(21) != num4)
            {
              this.data.HistoricalUnitObj[historical].SetHisVarValue(21, num4);
              this.ai.AddLog("Assigned Posture #" + num21.ToString() + " to " + this.data.UnitObj[index2].Name);
              let mut eventByLib: i32 =  DrawMod.TGame.EventRelatedObj.CheckGetEventByLib("SE_Data", 541, 0, 0);
              this.data.AddActionCard();
              let mut actionCardCounter: i32 =  this.data.ActionCardCounter;
              this.data.ActionCardObj[actionCardCounter].TempVar0 = num4;
              this.data.ActionCardObj[actionCardCounter].ExecuteEvent = eventByLib;
              let mut row: i32 =  this.data.StringListObj[stringListById1].FindRow(0, num4);
              let mut num22: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[row, 7]));
              this.data.RegimeObj[this.data.Turn].AddActionCard(this.data.ActionCardCounter);
              DrawMod.TGame.EditObj.DoCardSlot = this.data.RegimeObj[this.data.Turn].ActionCardCounter;
              this.data.StringListObj[stringListById2].SetData(0, "REGID", 1, id1, true);
              this.data.StringListObj[stringListById2].SetData(0, "HISID", 1, this.data.HistoricalUnitObj[historical].ID, true);
              this.data.StringListObj[stringListById2].SetData(0, "REGIMEID", 1, id1, true);
              this.data.StringListObj[stringListById2].SetData(0, "SOURCEREGIMEID", 1, id1, true);
              this.data.StringListObj[stringListById2].SetData(0, "ROUND", 1, this.data.Round, true);
              DrawMod.TGame.SelectX = this.data.UnitObj[index2].X;
              DrawMod.TGame.SelectY = this.data.UnitObj[index2].Y;
              DrawMod.TGame.EventRelatedObj.ExecHardcodedSet(46, this.data.HistoricalUnitObj[historical].ID, 0, 0, "");
              this.data.HistoricalUnitObj[historical].SetHisVarValue(21, num4);
              DrawMod.TGame.EventRelatedObj.DoCheckSpecificEvent(eventByLib, num4, -1, -1, -1);
              DrawMod.TGame.EventRelatedObj.IO_AddClear();
              this.ai.AddLog("Played card: ID#" + num4.ToString() + " : " + this.data.StringListObj[stringListById1].GetData(0, num4, 1) + " on: " + this.data.HistoricalUnitObj[historical].Name);
              this.data.RemoveActionCard(this.data.ActionCardCounter);
            }
            else
              this.ai.AddLog("Already had Posture #" + num21.ToString() + " assigned to " + this.data.UnitObj[index2].Name);
          }
        }
      }
      this.ai.WriteLog(str1);
      this.ai.ClearLog();
    }

    pub float CustomRuleTheater_MinimalAttackModifier(x: i32, y: i32, float currentMinimal)
    {
      float num1 = 1f;
      let mut num2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1)));
      let mut regime: i32 =  this.data.MapObj[0].HexObj[x, y].Regime;
      if (num2 == 1 & regime > -1)
      {
        let mut num3: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[regime].id, 1)));
        if (num3 == 2 | num3 == 3)
          num1 = 0.5f;
      }
      return num1;
    }

    pub fn CustomHelpCombatModifier(tHelpCombat: i32, forRegimeNr: i32) -> i32
    {
      let mut num1: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1)));
      let mut num2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[forRegimeNr].id, 1)));
      if (num1 == 1 && num2 > 1)
        tHelpCombat = 0;
      return tHelpCombat;
    }

    pub TargetRegimeRelationIsActuallyNotWar: bool(regnr: i32, targetregnr: i32, bool forMove)
    {
      if (forMove)
      {
        if (this.tempActuallyNotAtWarForMove.GetUpperBound(0) >= regnr & this.tempActuallyNotAtWarForMove.GetUpperBound(1) >= targetregnr && regnr > -1 & targetregnr > -1)
          return this.tempActuallyNotAtWarForMove[regnr, targetregnr];
      }
      else if (this.tempActuallyNotAtWarForAttack.GetUpperBound(0) >= regnr & this.tempActuallyNotAtWarForAttack.GetUpperBound(1) >= targetregnr && regnr > -1 & targetregnr > -1)
        return this.tempActuallyNotAtWarForAttack[regnr, targetregnr];
      return false;
    }

    pub CustomRuleHQtoFrontAssign_AllowHQGroupsWithoutHQ: bool(unr: i32)
    {
      let mut index: i32 =  this.data.Turn;
      if (unr > -1)
        index = this.data.UnitObj[unr].Regime;
      let mut num: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[index].id, 1)));
      return num == 2 | num == 3;
    }

    pub CustomDoStrategicIterations: bool() =>  Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1))) == 1;

    pub CustomIsMinor: bool()
    {
      switch ( Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1))))
      {
        case 1:
          return false;
        case 2:
          return true;
        default:
          return false;
      }
    }

    pub CustomIsMajor: bool() =>  Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1))) == 1;

    pub fn CustomStrategicReserveMultiplier() -> i32 => 1;

    pub CustomStrategicReserveDelegateToFrontline: bool(phase: i32) => false;

    pub object CustomRuleInitVars()
    {
      let mut num: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1)));
      if (num == 2 | num == 3)
      {
        this.ai.VAR_SUPPLY_MAXIMUM_RANGE = 1000;
        this.ai.VAR_SUPPLY_75PERCENT_RANGE = 700;
        this.ai.VAR_SUPPLY_50PERCENT_RANGE = 800;
        this.ai.VAR_SUPPLY_25PERCENT_RANGE = 900;
      }
      if (num == 1)
      {
        this.ai.VAR_DC4_ATTACKUNIT_IS_IMPORTANT = true;
        this.ai.VAR_USE_STRATEGIC_OPS_WITH_STRICT_HQFRONT = true;
        this.ai.VAR_DC4_STRATEGIC_DEFENSE_OF_SUPPLY_SOURCE = true;
      }
      object obj;
      return obj;
    }

    pub float CustomRuleTheaterModifiers_VpModifier(x: i32, y: i32) => 10f;

    pub CustomRule_MakeFrontsFromDefensiveZones_NoUnitsAssignedNeeded: bool() => true;

    pub void CustomRuleHQtoFrontAssign_CustomScripting_BeforeHqsToFrontAssigns(
      ref AIUnitList hqlist,
      ref AIFrontList fronts)
    {
    }

    pub float CustomRuleInitFrontlines_UnitRatioWeightModifier(unr: i32)
    {
      switch ( Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.UnitObj[unr].Regime].id, 1))))
      {
        case 1:
          return 1f;
        case 2:
          return 0.5f;
        case 4:
          return 0.25f;
        default:
          return 0.66f;
      }
    }

    pub CustomRuleInitFrontlines_MLAalreadySet: bool() =>  Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1))) == 1;

    pub fn CustomRuleInitFrontlines_ResetMatrixes()
    {
      let mut num1: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1)));
      let mut stringListById1: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 123, 0, 0));
      let mut stringListById2: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 160, 0, 0));
      let mut stringListById3: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 236, 0, 0));
      let mut stringListById4: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID(this.dataLib, 148, 0, 0));
      let mut stringListById5: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 144, 0, 0));
      let mut id1: i32 =  this.data.RegimeObj[this.data.Turn].id;
      if (num1 != 1)
        return;
      this.ai.AddLog("CUSTOM MATRIXES AND ZONES (OFF/DEF)");
      this.ai.AddLog("");
      this.ai.VAR_ZONES_TYPE = 3;
      this.ai.VAR_USE_BROAD_DEFENSIVE_ZONES = false;
      this.ai.VAR_BROAD_DEFENSIVE_ZONE_HEX_MINIMUM = 1;
      AIMatrix mask1 = this.ai.SetOwnerMatrix(0, 0, this.data.MapObj[0].MapWidth, this.data.MapObj[0].MapHeight);
      AIMatrix aiMatrix1 = mask1.Clone();
      aiMatrix1.RemoveValuesByMask(mask1, 1);
      aiMatrix1.ExpandAndAddValueForAnyRegime(199);
      aiMatrix1.SetAllValuesSubtractWith(2);
      let mut num2: i32 =  0;
      AIMatrix aiMatrix2 = new AIMatrix(ref this.ai.game.DC2AIObj);
      aiMatrix2.SetAllValuesToWithMask(1, ref mask1, 1);
      AIMatrix aiMatrix3 = aiMatrix2.DetectAndMakeEdgeMatrix(false);
      aiMatrix3.RemoveValuesByLandscapeAIBlock(0);
      aiMatrix3.ExpandSpecificValueForAnyRegime(1, 1);
      let mut mapWidth1: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth1; index1 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (aiMatrix3.Value[index1, index2] > 0)
            num2 += 1;
        }
      }
      let mut num3: i32 =   Math.Round( num2 * 0.25);
      if (3 > num3)
        num3 = 3;
      let mut num4: i32 =  0;
      let mut num5: i32 =  0;
      let mut num6: i32 =  0;
      let mut num7: i32 =  0;
      let mut num8: i32 =  0;
      let mut num9: i32 =  0;
      let mut unitCounter1: i32 =  this.data.UnitCounter;
      num10: i32;
      for (let mut unr: i32 =  0; unr <= unitCounter1; unr += 1)
      {
        if (this.data.UnitObj[unr].Regime == this.data.Turn & this.data.UnitObj[unr].PreDef == -1)
        {
          if (this.data.UnitObj[unr].AIAttack != 1)
          {
            let mut historical: i32 =  this.data.UnitObj[unr].Historical;
            if (historical > -1 && this.data.HistoricalUnitObj[historical].GiveHisVarValue(11) < 1)
            {
              num4 += 1;
              if (this.ai.GetAIRolePercent(unr, 8) > 33)
                num5 += 1;
              else if (this.ai.GetAIRolePercent(unr, 10) > 33)
                num7 += 1;
              else if (this.ai.GetAIRolePercent(unr, 6) > 33)
                num6 += 1;
              else
                num8 += 1;
              num10 = -1;
              if (!this.data.UnitObj[unr].AIReserve)
                num9 += 1;
              else
                num9 = num9;
            }
          }
          else
            num10 = num10;
        }
      }
      float val2_1 =  num9 /  num3;
      this.ai.AddLog("UnitsPerFrontHex: " + val2_1.ToString());
      this.ai.AddLog("Units: " + num4.ToString());
      this.ai.AddLog("Frontage: " + num3.ToString());
      if (this.data.Round < 10)
        val2_1 += 0.18f;
      else if (this.data.Round < 20)
        val2_1 += 0.14f;
      else if (this.data.Round < 30)
        val2_1 += 0.1f;
      else if (this.data.Round < 40)
        val2_1 += 0.6f;
      else if (this.data.Round < 50)
        val2_1 += 0.2f;
      this.ai.AddLog("Adjusted UnitsPerFrontHex: " + val2_1.ToString());
      this.ai.AddLog("");
      this.ai.MLAMatrix = new AIMatrix(ref this.ai);
      let mut mapWidth2: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index3: i32 =  0; index3 <= mapWidth2; index3 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
        {
          this.ai.VAR_MATRIX_ZONES.Value[index3, index4] = 0;
          this.ai.VAR_MATRIX_RETREAT.Value[index3, index4] = 100;
          this.ai.VAR_MATRIX_STRENGTH.Value[index3, index4] = 100;
          this.ai.MLAMatrix.Value[index3, index4] = 1;
        }
      }
      SimpleList simpleList1 = SimpleList::new();
      let mut unitCounter2: i32 =  this.ai.game.Data.UnitCounter;
      for (let mut tid: i32 =  0; tid <= unitCounter2; tid += 1)
      {
        if (this.ai.game.Data.UnitObj[tid].Regime == this.ai.game.Data.Turn)
        {
          let mut historical: i32 =  this.data.UnitObj[tid].Historical;
          if (historical > -1)
          {
            if (this.data.UnitObj[tid].PreDef == -1 & this.data.UnitObj[tid].IsHQ && historical > -1 && this.data.HistoricalUnitObj[historical].Type == 5 && this.data.HistoricalUnitObj[historical].AIlist > 0)
            {
              simpleList1.Add(tid, this.data.HistoricalUnitObj[historical].AIlist);
              this.ai.AddLog("Added " + this.data.UnitObj[tid].Name + " to OldAIList: " + this.data.HistoricalUnitObj[historical].AIlist.ToString());
            }
            this.data.HistoricalUnitObj[historical].AIlist = 0;
          }
        }
      }
      this.ai.AddLog("");
      SimpleList simpleList2 = SimpleList::new();
      SimpleList simpleList3 = SimpleList::new();
      SimpleList simpleList4 = SimpleList::new();
      let mut idValue1: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, id1, 12)));
      let mut num11: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].GetData(6, idValue1, 0)));
      let mut length1: i32 =  this.data.StringListObj[stringListById1].Length;
      for (let mut index5: i32 =  0; index5 <= length1; index5 += 1)
      {
        if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index5, 8])) == id1)
        {
          let mut num12: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index5, 0]));
          str: String = this.data.StringListObj[stringListById1].Data[index5, 7];
          let mut tweight1: i32 =  0;
          let mut length2: i32 =  this.data.StringListObj[stringListById4].Length;
          for (let mut index6: i32 =  0; index6 <= length2; index6 += 1)
          {
            if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById4].Data[index6, 0])) == num12)
            {
              let mut idValue2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById4].Data[index6, 1]));
              let mut num13: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById5].GetData(0, idValue2, 2)));
              if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById5].GetData(0, idValue2, 5))) == 1)
                tweight1 += 3 * (num13 + 1);
              else
                tweight1 += num13 + 1;
            }
          }
          simpleList3.AddWeight(num12, tweight1);
          let mut num14: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById2].GetData2(0, num12, 1, "pop", 2)));
          let mut num15: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById2].GetData2(0, num12, 1, "worker", 2)));
          let mut num16: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById2].GetData2(0, num12, 1, "cas", 2)));
          if (num16 < 1)
            num16 = 100;
          let mut num17: i32 =  num16 + 25;
          let mut tweight2: i32 =   Math.Round( ((num14 + num15) * num17) / 100.0);
          this.ai.AddLog(str + " got score1: " + tweight1.ToString() + ", score2: " + tweight2.ToString());
          simpleList4.AddWeight(num12, tweight2);
        }
      }
      simpleList3.Percentify();
      simpleList4.Percentify();
      let mut length3: i32 =  this.data.StringListObj[stringListById1].Length;
      for (let mut index: i32 =  0; index <= length3; index += 1)
      {
        if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 8])) == id1)
        {
          let mut tid: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 0]));
          let mut weight1: i32 =  simpleList3.FindWeight(tid);
          let mut weight2: i32 =  simpleList4.FindWeight(tid);
          let mut num18: i32 =  0;
          if (num11 == tid)
            num18 = 100;
          let mut tweight: i32 =   Math.Round( (weight1 + weight2 + num18) / 3.0);
          simpleList2.AddWeight(tid, tweight);
        }
      }
      simpleList2.ReverseSort();
      let mut num19: i32 =  simpleList2.Counter + 1;
      let mut num20: i32 =  0;
      let mut num21: i32 =  2;
      let mut num22: i32 =  1;
      let mut counter1: i32 =  simpleList2.Counter;
      for (let mut index: i32 =  0; index <= counter1; index += 1)
      {
        num20 += 1;
        if (num20 > num21)
        {
          num20 = 0;
          num22 += 1;
          num21 += 1;
        }
      }
      if (num22 > simpleList2.Counter + 1)
        num22 = simpleList2.Counter + 1;
      this.ai.AddLog("");
      this.ai.AddLog("IMPORTANT FRIENDLY ZONES");
      let mut num23: i32 =  num22;
      tid1: i32;
      for (tid1 = 1; tid1 <= num23; tid1 += 1)
      {
        let mut idValue3: i32 =  simpleList2.Id[tid1 - 1];
        data: String = this.data.StringListObj[stringListById1].GetData(0, idValue3, 7);
        this.ai.AddLog("#" + tid1.ToString() + " : " + data);
      }
      this.ai.AddLog("");
      SimpleList simpleList5 = SimpleList::new();
      SimpleList simpleList6 = SimpleList::new();
      SimpleList simpleList7 = SimpleList::new();
      SimpleList simpleList8 = SimpleList::new();
      SimpleList simpleList9 = SimpleList::new();
      let mut regimeCounter1: i32 =  this.data.RegimeCounter;
      for (tid1 = 1; tid1 <= regimeCounter1; tid1 += 1)
      {
        if (tid1 != this.data.Turn)
        {
          let mut num24: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[tid1].id, 1)));
          let mut id2: i32 =  this.data.RegimeObj[tid1].id;
          let mut num25: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[tid1].id, 2, "aiIntention", 3)));
          let mut num26: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[tid1].id, 2, "relation", 3)));
          AIMatrix aiMatrix4 = mask1.Clone();
          let mut mapWidth3: i32 =  this.data.MapObj[0].MapWidth;
          for (let mut index7: i32 =  0; index7 <= mapWidth3; index7 += 1)
          {
            let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
            for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
              aiMatrix4.Value[index7, index8] = this.data.MapObj[0].HexObj[index7, index8].Regime == tid1 ? 1 : 0;
          }
          aiMatrix4.ExpandAndAddValueForAnyRegime(49);
          aiMatrix4.SetAllValuesSubtractWith(1);
          AIMatrix aiMatrix5 = new AIMatrix(ref this.ai);
          let mut mapWidth4: i32 =  this.data.MapObj[0].MapWidth;
          for (let mut index9: i32 =  0; index9 <= mapWidth4; index9 += 1)
          {
            let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
            for (let mut index10: i32 =  0; index10 <= mapHeight; index10 += 1)
              aiMatrix5.Value[index9, index10] = this.data.MapObj[0].HexObj[index9, index10].Regime == this.data.Turn ? 1 : 0;
          }
          switch (num24)
          {
            case 1:
              aiMatrix5.ExpandAndAddValueForAnyRegime(20);
              break;
            case 2:
              aiMatrix5.ExpandAndAddValueForAnyRegime(10);
              break;
            default:
              aiMatrix5.ExpandAndAddValueForAnyRegime(5);
              break;
          }
          aiMatrix5.SetAllValuesSubtractWith(1);
          let mut num27: i32 =  0;
          let mut num28: i32 =  0;
          let mut mapWidth5: i32 =  this.data.MapObj[0].MapWidth;
          for (let mut index11: i32 =  0; index11 <= mapWidth5; index11 += 1)
          {
            let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
            for (let mut index12: i32 =  0; index12 <= mapHeight; index12 += 1)
            {
              if (this.data.MapObj[0].HexObj[index11, index12].Regime == this.data.Turn && aiMatrix3.Value[index11, index12] > 0 & aiMatrix4.Value[index11, index12] == 1)
                num27 += 10;
              if (this.data.MapObj[0].HexObj[index11, index12].Regime == tid1 && aiMatrix5.Value[index11, index12] > 0 & aiMatrix5.Value[index11, index12] < 21 && this.data.MapObj[0].HexObj[index11, index12].UnitCounter > -1)
              {
                let mut unitCounter3: i32 =  this.data.MapObj[0].HexObj[index11, index12].UnitCounter;
                for (let mut index13: i32 =  0; index13 <= unitCounter3; index13 += 1)
                {
                  let mut unit: i32 =  this.data.MapObj[0].HexObj[index11, index12].UnitList[index13];
                  num28 += this.data.UnitObj[unit].TempUnitPowerAbs;
                }
              }
            }
          }
          let mut tweight3: i32 =   Math.Round( (num27 * num28) / 100.0);
          simpleList6.AddWeight(id2, tweight3);
          let mut tweight4: i32 =  0;
          let mut length4: i32 =  this.data.StringListObj[stringListById1].Length;
          for (let mut index: i32 =  0; index <= length4; index += 1)
          {
            if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 8])) == id1)
            {
              let mut tid2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 0]));
              let mut nr1: i32 =  simpleList2.FindNr(tid2);
              if (nr1 > -1 & nr1 < num22)
              {
                let mut id3: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index, 6]));
                if (id3 > 0)
                {
                  let mut locationById: i32 =  DrawMod.TGame.HandyFunctionsObj.GetLocationByID(id3);
                  if (locationById > -1)
                  {
                    let mut num29: i32 =  aiMatrix4.Value[this.data.LocObj[locationById].X, this.data.LocObj[locationById].Y];
                    let mut nr2: i32 =  simpleList2.FindNr(tid2);
                    if (nr2 > -1 && simpleList2.Data1[nr2] > num29)
                      simpleList2.Data1[nr2] = num29;
                    tweight4 += num29;
                  }
                }
              }
            }
          }
          simpleList7.Add(id2, tweight4);
          let mut tweight5: i32 =  10;
          if (num24 == 1)
            tweight5 = 100;
          if (num24 == 2)
            tweight5 = 50;
          if (num24 == 3)
            tweight5 = 50;
          if (this.data.RegimeObj[this.data.Turn].RegimeRel[tid1] == 1)
            tweight5 =  Math.Round( tweight3 / 5.0);
          simpleList8.Add(id2, tweight5);
          let mut tweight6: i32 =  new Random( Math.Round( (this.data.RegimeObj[tid1].id * this.data.GameID) / 10.0)).Next(1, 101);
          simpleList9.Add(id2, tweight6);
        }
      }
      simpleList6.Percentify();
      simpleList7.Percentify();
      simpleList8.Percentify();
      simpleList9.Percentify();
      let mut regimeCounter2: i32 =  this.data.RegimeCounter;
      index14: i32;
      for (tid1 = 1; tid1 <= regimeCounter2; tid1 += 1)
      {
        if (tid1 != this.data.Turn)
        {
          let mut num30: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[tid1].id, 1)));
          let mut id4: i32 =  this.data.RegimeObj[tid1].id;
          let mut val1: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[tid1].id, 2, "aiIntention", 3)));
          let mut val2_2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[tid1].id, 2, "relation", 3)));
          let mut num31: i32 =  this.data.RegimeObj[this.data.Turn].RegimeRel[tid1];
          if (num30 != 4)
          {
            let mut tweight: i32 =  0;
            index14 = simpleList6.FindWeight(id4);
            let mut num32: i32 =  100 - simpleList7.FindWeight(id4);
            let mut weight: i32 =  simpleList8.FindWeight(id4);
            let mut num33: i32 =   Math.Round( simpleList9.FindWeight(id4) / 3.0);
            tweight =  Math.Round( (index14 + num32 + weight + num33) / 4.0);
            let mut num34: i32 =  100 - Math.Max(val1, val2_2);
            if (num34 < 0)
              num34 = 0;
            if (num31 > 0)
            {
              tweight =  Math.Round( (tweight * 5 * num34) / 100.0);
              tweight =  Math.Round( (tweight * num34) / 100.0);
            }
            else
              tweight *= 100;
            this.ai.AddLog(this.data.RegimeObj[tid1].Name + " got score1(frontage+power): " + index14.ToString() + ", score2(distance): " + num32.ToString() + ", score3(rel): " + weight.ToString() + ", score4(rnd):" + num33.ToString() + " => finalscore: " + tweight.ToString());
            if (index14 > 0)
              simpleList5.Add(id4, tweight);
          }
        }
      }
      simpleList5.ReverseSort();
      index14 = simpleList5.Counter + 1;
      let mut num35: i32 =  0;
      let mut num36: i32 =  3;
      let mut num37: i32 =  1;
      let mut counter2: i32 =  simpleList5.Counter;
      for (tid1 = 0; tid1 <= counter2; tid1 += 1)
      {
        num35 += 1;
        if (num35 > num36)
        {
          num35 = 0;
          num37 += 1;
          num36 += 1;
        }
      }
      let mut num38: i32 =   Math.Round(Math.Ceiling( num37 *  Math.Min(1f, val2_1)));
      if (num38 < 1)
        num38 = 1;
      if (num38 > simpleList5.Counter + 1)
        num38 = simpleList5.Counter + 1;
      this.ai.AddLog("");
      this.ai.AddLog("IMPORTANT ENEMIES");
      let mut num39: i32 =  num38;
      for (tid1 = 1; tid1 <= num39; tid1 += 1)
      {
        index14 = simpleList5.Id[tid1 - 1];
        name: String = this.data.RegimeObj[DrawMod.TGame.HandyFunctionsObj.GetRegimeByID(index14)].Name;
        this.ai.AddLog("#" + tid1.ToString() + " : " + name);
      }
      this.ai.AddLog("");
      SimpleList simpleList10 = SimpleList::new();
      SimpleList simpleList11 = SimpleList::new();
      let mut unitCounter4: i32 =  this.data.UnitCounter;
      for (tid1 = 0; tid1 <= unitCounter4; tid1 += 1)
      {
        if (this.data.UnitObj[tid1].Regime == this.ai.game.Data.Turn & this.data.UnitObj[tid1].PreDef == -1 & this.data.UnitObj[tid1].IsHQ)
        {
          let mut historical: i32 =  this.data.UnitObj[tid1].Historical;
          if (historical > -1 && this.data.HistoricalUnitObj[historical].Type == 5)
          {
            index14 = 0;
            let mut num40: i32 =  0;
            let mut unitCounter5: i32 =  this.data.UnitCounter;
            for (let mut index15: i32 =  0; index15 <= unitCounter5; index15 += 1)
            {
              if (this.data.UnitObj[index15].Regime == this.ai.game.Data.Turn & this.data.UnitObj[index15].PreDef == -1 & !this.data.UnitObj[index15].IsHQ && this.data.UnitObj[index15].HQ == tid1)
              {
                index14 += 1;
                if (this.data.UnitObj[index15].TempTopUnit)
                  num40 += this.data.UnitObj[index15].TempUnitPowerAbs * 10;
                else
                  num40 += this.data.UnitObj[index15].TempUnitPowerAbs;
              }
            }
            if (index14 > 2)
            {
              index14 = num40;
              if (simpleList1.FindWeight(tid1) > 0)
                index14 *= 3;
              simpleList10.AddWeight(tid1, index14);
            }
          }
        }
      }
      simpleList10.ReverseSort();
      let mut unitCounter6: i32 =  this.data.UnitCounter;
      for (tid1 = 0; tid1 <= unitCounter6; tid1 += 1)
      {
        if (this.data.UnitObj[tid1].Regime == this.ai.game.Data.Turn & this.data.UnitObj[tid1].PreDef == -1 & this.data.UnitObj[tid1].IsHQ)
        {
          let mut historical: i32 =  this.data.UnitObj[tid1].Historical;
          if (historical > -1 && this.data.HistoricalUnitObj[historical].Type == 5)
          {
            index14 = 0;
            let mut num41: i32 =  0;
            let mut num42: i32 =  0;
            let mut unitCounter7: i32 =  this.data.UnitCounter;
            for (let mut index16: i32 =  0; index16 <= unitCounter7; index16 += 1)
            {
              if (this.data.UnitObj[index16].Regime == this.ai.game.Data.Turn & this.data.UnitObj[index16].PreDef == -1 & !this.data.UnitObj[index16].IsHQ && this.data.UnitObj[index16].HQ == tid1)
              {
                index14 += 1;
                if (this.data.UnitObj[index16].TempTopUnit)
                  num41 += 1;
              }
            }
            let mut idValue4: i32 =  this.data.HistoricalUnitObj[historical].GiveHisVarValue(1);
            if (idValue4 > 0)
            {
              let mut num43: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById3].GetData(0, idValue4, 2)));
              num42 = 2;
              if (num43 > 0)
                num42 = num43;
            }
            if (index14 > 3)
            {
              index14 = 100000;
              if (num41 > 0)
                index14 =  Math.Round( index14 /  ((num41 + 1) * (num41 + 1) * (num41 + 1)));
              let mut num44: i32 =  num42 * num42;
              if (num44 > 0)
                index14 =  Math.Round( index14 /  num44);
              if (simpleList1.FindWeight(tid1) > 0)
                index14 *= 10;
              simpleList11.AddWeight(tid1, index14);
            }
          }
        }
      }
      simpleList11.ReverseSort();
      let mut num45: i32 =  0;
      let mut num46: i32 =  2;
      let mut num47: i32 =  1;
      let mut counter3: i32 =  simpleList10.Counter;
      for (tid1 = 0; tid1 <= counter3; tid1 += 1)
      {
        num45 += 1;
        if (num45 > num46)
        {
          num45 = 0;
          num47 += 1;
          num46 += 1;
        }
      }
      this.ai.AddLog("");
      this.ai.AddLog("SPARE OHQs (if Offensive)");
      let mut num48: i32 =  num47;
      for (tid1 = 1; tid1 <= num48; tid1 += 1)
      {
        index14 = simpleList10.Id[tid1 - 1];
        name: String = this.data.UnitObj[index14].Name;
        this.ai.AddLog("#" + tid1.ToString() + " : " + name);
      }
      this.ai.AddLog("");
      this.ai.AddLog("");
      this.ai.AddLog("SPARE OHQs (if Defensive)");
      let mut num49: i32 =  num47;
      for (tid1 = 1; tid1 <= num49; tid1 += 1)
      {
        index14 = simpleList11.Id[tid1 - 1];
        name: String = this.data.UnitObj[index14].Name;
        this.ai.AddLog("#" + tid1.ToString() + " : " + name);
      }
      this.ai.AddLog("");
      let mut num50: i32 =  0;
      let mut num51: i32 =  0;
      let mut num52: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimeKeys].GetData2(0, id1, 1, "majorAiOffensiveMode", 2)));
      let mut setValue1: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimeKeys].GetData2(0, id1, 1, "majorAiOffensiveModeFixed", 2)));
      if (num52 < 1)
        num52 = 50;
      if (setValue1 < 1)
        setValue1 = 50;
      let mut num53: i32 =  0;
      let mut num54: i32 =  0;
      let mut num55: i32 =  num22 - 1;
      for (tid1 = 0; tid1 <= num55; tid1 += 1)
      {
        index14 =  Math.Round(Conversion.Val(this.data.StringListObj[stringListById2].GetData2(0, simpleList2.Id[tid1], 1, "majorAiEnemyDistance", 2)));
        if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById2].GetData2(0, simpleList2.Id[tid1], 1, "majorAiRegId", 2))) == id1)
        {
          num50 += index14;
          num51 += simpleList2.Data1[tid1];
          if (num51 > num50)
          {
            let mut num56: i32 =  num51 - num50;
            if (num56 > num53)
              num53 = num56;
          }
          if (num51 < num50)
          {
            let mut num57: i32 =  num50 - num51;
            if (num57 > num54)
              num54 = num57;
          }
        }
      }
      if (num50 == 0)
        num50 = num51;
      let mut num58: i32 =  num52;
      let mut setValue2: i32 =  !(num50 > 0 & num51 > 0) ? num58 + 1 : (  Math.Round( num51 /  num50) < 0.5 ? (num54 >= 1 ? (num54 >= 2 ? (num54 >= 3 ? num58 - 7 : num58 - 6) : num58 - 4) : num58 - 1) : (num53 >= 1 ? (num53 >= 2 ? (num53 >= 3 ? num58 + 7 : num58 + 6) : num58 + 4) : num58 + 1));
      this.ai.VAR_OFFENSIVE_ZONE_IS_ALL_OUT_MODE = 0;
      let mut num59: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimeKeys].GetData2(0, id1, 1, "pathWar_Offensive", 2)));
      let mut num60: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimeKeys].GetData2(0, id1, 1, "pathWar_Defensive", 2)));
      let mut num61: i32 =  num59 - num60;
      this.ai.VAR_OFFENSIVE_ZONE_IS_ALL_OUT_MODE = setValue2 < 80 ? (setValue2 < 70 ? (setValue2 <= 60 ? (setValue2 < 40 ? (setValue2 < 30 ? 0 : (num59 < 70 ? 0 : 1)) : (num59 < 50 ? 0 : 1)) : (num59 < 70 ? 1 : 2)) : (num59 < 50 ? 1 : 2)) : 2;
      if (setValue2 < 20)
        setValue2 = 20;
      if (setValue2 > 80)
        setValue2 = 80;
      this.ai.AddLog("OldOffensiveMod: " + num52.ToString());
      this.ai.AddLog("currentOffensiveMode: " + setValue2.ToString());
      this.ai.AddLog("OldOffensiveModFixed: " + setValue1.ToString());
      if (Math.Abs(setValue1 - setValue2) >= 15)
      {
        setValue1 = setValue2;
        this.ai.AddLog("OldOffensiveModFixed IS CHANGED TO CURRENT OFFENSIVE MODE");
      }
      else
        this.ai.AddLog("OldOffensiveModFixed IS UNCHANGED");
      let mut num62: i32 =  setValue1;
      num63: i32;
      num64: i32;
      if (num62 >= 50)
      {
        num63 =  Math.Round(Math.Ceiling( (num47 * num62) / 100.0));
        num64 = num47 - num63;
      }
      else
      {
        --num47;
        if (num47 < 1)
          num47 = 0;
        num64 =  Math.Round(Math.Ceiling( (num47 * (100 - num62)) / 100.0));
        num63 = num47 - num64;
      }
      if (num63 > 1 & num64 == 0)
      {
        num64 = 1;
        --num63;
      }
      if (num61 > 50)
      {
        if (num47 <= 5 & num64 > 0)
        {
          --num64;
          num63 += 1;
        }
        if (num47 <= 5 & num64 > 0)
        {
          --num64;
          num63 += 1;
        }
        if (num47 <= 5 & num64 > 0)
        {
          --num64;
          num63 += 1;
        }
      }
      else if (num61 > 30)
      {
        if (num47 <= 4 & num64 > 0)
        {
          --num64;
          num63 += 1;
        }
        if (num47 <= 4 & num64 > 0)
        {
          --num64;
          num63 += 1;
        }
      }
      else if (num61 > 15)
      {
        if (num47 <= 3 & num64 > 0)
        {
          --num64;
          num63 += 1;
        }
      }
      else if (num61 > 0 && num47 <= 2 & num64 > 0)
      {
        --num64;
        num63 += 1;
      }
      this.ai.AddLog("Offensive OHQs: " + num63.ToString() + ", Defensive OHQs: " + num64.ToString());
      this.ai.AddLog("");
      this.ai.AddLog("FIND BEST OFFENSIVE MATCHES");
      if (num63 > num47)
        num63 = num47;
      if (simpleList10.Counter + 1 < num63)
        num63 = simpleList10.Counter + 1;
      int[] numArray1 = new int[this.data.LocCounter + 10 + 1];
      let mut counter4: i32 =  simpleList10.Counter;
      for (tid1 = 0; tid1 <= counter4; tid1 += 1)
      {
        num65: i32;
        if (num65 < num63 && simpleList10.Data2[tid1] < 1 & simpleList11.FindData(simpleList10.Id[tid1], 2) < 1)
        {
          let mut num66: i32 =  0;
          let mut num67: i32 =  0;
          let mut index17: i32 =  0;
          let mut index18: i32 =  0;
          let mut id5: i32 =  0;
          let mut x: i32 =  this.data.UnitObj[simpleList10.Id[tid1]].X;
          let mut y: i32 =  this.data.UnitObj[simpleList10.Id[tid1]].Y;
          AIMatrix aiMatrix6 = new AIMatrix(ref this.ai);
          aiMatrix6.SetAllValuesTo(0);
          aiMatrix6.Value[x, y] = 1;
          aiMatrix6.ExpandAndAddValueForSameRegime(49);
          aiMatrix6.ExpandAndAddValueForAnyRegime(99, true);
          aiMatrix6.SetValueXToValueY(0, 9999);
          this.ai.AddLog("For " + this.data.UnitObj[simpleList10.Id[tid1]].Name + ":");
          let mut num68: i32 =  num38 - 1;
          locationById: i32;
          for (let mut index19: i32 =  0; index19 <= num68; index19 += 1)
          {
            let mut idValue5: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, simpleList5.Id[index19], 12)));
            let mut num69: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].GetData(6, idValue5, 0)));
            let mut length5: i32 =  this.data.StringListObj[stringListById1].Length;
            for (let mut index20: i32 =  0; index20 <= length5; index20 += 1)
            {
              if ( Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index20, 8])) == simpleList5.Id[index19])
              {
                let mut num70: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index20, 0]));
                let mut num71: i32 =  50;
                if (num69 == num70)
                  num71 = 100;
                let mut id6: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].Data[index20, 6]));
                if (id6 > 0)
                {
                  locationById = DrawMod.TGame.HandyFunctionsObj.GetLocationByID(id6);
                  if (locationById > -1)
                  {
                    let mut num72: i32 =  aiMatrix6.Value[this.data.LocObj[locationById].X, this.data.LocObj[locationById].Y];
                    if (num72 < 199)
                    {
                      let mut num73: i32 =   Math.Round( (num71 * 1000) /  num72);
                      if (simpleList1.FindWeight(simpleList10.Id[tid1]) == id6 && this.data.RegimeObj[this.data.MapObj[0].HexObj[this.data.LocObj[locationById].X, this.data.LocObj[locationById].Y].Regime].RegimeRel[this.data.Turn] == 0)
                        num73 *= 3;
                      if (numArray1[locationById] > 0)
                        num73 =  Math.Round( num73 /  (numArray1[locationById] + 1));
                      this.ai.AddLog("*" + this.data.LocObj[locationById].Name + " = " + num73.ToString());
                      if (num73 > num67)
                      {
                        num67 = num73;
                        num66 = num70;
                        id5 = id6;
                        index17 = this.data.LocObj[locationById].X;
                        index18 = this.data.LocObj[locationById].Y;
                      }
                    }
                  }
                }
              }
            }
          }
          if (num67 > 0)
          {
            locationById = DrawMod.TGame.HandyFunctionsObj.GetLocationByID(id5);
            int[] numArray2 = numArray1;
            int[] numArray3 = numArray2;
            let mut index21: i32 =  locationById;
            let mut index22: i32 =  index21;
            let mut num74: i32 =  numArray2[index21] + 1;
            numArray3[index22] = num74;
            this.ai.AddLog("====> Assigned to: ==> " + this.data.LocObj[locationById].Name);
            this.ai.AddLog("");
            num65 += 1;
            simpleList10.Data2[tid1] = 1;
            this.ai.VAR_MATRIX_ZONES.Value[index17, index18] = id5;
            this.data.HistoricalUnitObj[this.data.UnitObj[simpleList10.Id[tid1]].Historical].AIlist = id5;
          }
          else
          {
            this.ai.AddLog("====> NOT ASSIGNED " + this.data.LocObj[locationById].Name);
            this.ai.AddLog("");
          }
        }
      }
      this.ai.AddLog("");
      this.ai.AddLog("");
      this.ai.AddLog("FIND BEST DEFENSIVE ZONE MATCHES");
      if (num64 > num47)
        num64 = num47;
      if (simpleList11.Counter + 1 < num64)
        num64 = simpleList11.Counter + 1;
      int[] numArray4 = new int[this.data.LocCounter + 10 + 1];
      let mut num75: i32 =  0;
      let mut counter5: i32 =  simpleList11.Counter;
      for (tid1 = 0; tid1 <= counter5; tid1 += 1)
      {
        if (num75 < num64 && simpleList11.Data2[tid1] < 1 & simpleList10.FindData(simpleList11.Id[tid1], 2) < 1)
        {
          let mut num76: i32 =  0;
          let mut num77: i32 =  0;
          let mut index23: i32 =  0;
          let mut index24: i32 =  0;
          let mut id7: i32 =  0;
          let mut x: i32 =  this.data.UnitObj[simpleList11.Id[tid1]].X;
          let mut y: i32 =  this.data.UnitObj[simpleList11.Id[tid1]].Y;
          AIMatrix aiMatrix7 = new AIMatrix(ref this.ai);
          aiMatrix7.SetAllValuesTo(0);
          aiMatrix7.Value[x, y] = 1;
          aiMatrix7.ExpandAndAddValueForSameRegime(69);
          aiMatrix7.SetValueXToValueY(0, 9999);
          this.ai.AddLog("For " + this.data.UnitObj[simpleList11.Id[tid1]].Name + ":");
          let mut num78: i32 =  num22 - 1;
          for (let mut index25: i32 =  0; index25 <= num78; index25 += 1)
          {
            let mut idValue6: i32 =  simpleList2.Id[index25];
            let mut num79: i32 =  simpleList2.Weight[index25];
            let mut id8: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].GetData(0, idValue6, 6)));
            if (id8 > 0)
            {
              let mut locationById: i32 =  DrawMod.TGame.HandyFunctionsObj.GetLocationByID(id8);
              if (locationById > -1)
              {
                let mut num80: i32 =  aiMatrix7.Value[this.data.LocObj[locationById].X, this.data.LocObj[locationById].Y];
                if (num80 < 199)
                {
                  let mut num81: i32 =   Math.Round( (num79 * 1000) /  num80);
                  if (simpleList1.FindWeight(simpleList11.Id[tid1] + 200000) - 200000 == id8)
                    num81 *= 3;
                  if (numArray4[locationById] > 0)
                    num81 *= numArray4[locationById] + 1;
                  this.ai.AddLog("*" + this.data.LocObj[locationById].Name + " = " + num81.ToString());
                  if (num81 > num77)
                  {
                    num77 = num81;
                    num76 = idValue6;
                    id7 = id8;
                    index23 = this.data.LocObj[locationById].X;
                    index24 = this.data.LocObj[locationById].Y;
                  }
                }
              }
            }
          }
          if (num77 > 0)
          {
            let mut locationById: i32 =  DrawMod.TGame.HandyFunctionsObj.GetLocationByID(id7);
            int[] numArray5 = numArray4;
            int[] numArray6 = numArray5;
            let mut index26: i32 =  locationById;
            let mut index27: i32 =  index26;
            let mut num82: i32 =  numArray5[index26] + 1;
            numArray6[index27] = num82;
            this.ai.AddLog("====> Assigned to: ==> " + this.data.LocObj[locationById].Name);
            this.ai.AddLog("");
            num75 += 1;
            simpleList11.Data2[tid1] = 1;
            AIMatrix aiMatrix8 = new AIMatrix(ref this.ai);
            aiMatrix8.SetAllValuesTo(0);
            aiMatrix8.Value[index23, index24] = 1;
            aiMatrix8.ExpandAndAddValueForSameRegime(4);
            let mut mapWidth6: i32 =  this.data.MapObj[0].MapWidth;
            for (let mut index28: i32 =  0; index28 <= mapWidth6; index28 += 1)
            {
              let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
              for (let mut index29: i32 =  0; index29 <= mapHeight; index29 += 1)
              {
                if (this.data.MapObj[0].HexObj[index28, index29].Regime == this.data.Turn && aiMatrix8.Value[index28, index29] > 0)
                  this.ai.VAR_MATRIX_ZONES.Value[index28, index29] = 200000 + id7;
              }
            }
            this.data.HistoricalUnitObj[this.data.UnitObj[simpleList11.Id[tid1]].Historical].AIlist = 200000 + id7;
          }
        }
      }
      this.ai.AddLog("");
      let mut counter6: i32 =  simpleList2.Counter;
      for (tid1 = 0; tid1 <= counter6; tid1 += 1)
      {
        this.data.StringListObj[stringListById2].SetData2(0, simpleList2.Id[tid1], 1, "majorAiRegId", 2, this.data.RegimeObj[this.data.Turn].id, true);
        this.data.StringListObj[stringListById2].SetData2(0, simpleList2.Id[tid1], 1, "majorAiEnemyDistance", 2, simpleList2.Data1[tid1], true);
      }
      this.data.StringListObj[this.slotRegimeKeys].SetData2(0, id1, 1, "majorAiOffensiveMode", 2, setValue2, true);
      this.data.StringListObj[this.slotRegimeKeys].SetData2(0, id1, 1, "majorAiOffensiveModeFixed", 2, setValue1, true);
      AIMatrix addvalue = new AIMatrix(ref this.ai);
      let mut mapWidth7: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index30: i32 =  0; index30 <= mapWidth7; index30 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index31: i32 =  0; index31 <= mapHeight; index31 += 1)
          addvalue.Value[index30, index31] = this.data.MapObj[0].HexObj[index30, index31].Regime != this.data.Turn ? 0 : 1;
      }
      let mut num83: i32 =  0;
      maxy1: i32;
      if ( val2_1 < 0.2)
        maxy1 = 2;
      else if ( num83 > 0.4)
      {
        maxy1 = 99;
      }
      else
      {
        maxy1 =  Math.Round(7.0 * (( val2_1 - 0.2) / 0.2));
        if (maxy1 > 6)
          maxy1 = 6;
        if (maxy1 < 3)
          maxy1 = 3;
      }
      AIMatrix mask2 = new AIMatrix(ref this.ai);
      let mut mapWidth8: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index32: i32 =  0; index32 <= mapWidth8; index32 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index33: i32 =  0; index33 <= mapHeight; index33 += 1)
        {
          if (this.data.MapObj[0].HexObj[index32, index33].Regime > -1)
          {
            index14 = simpleList5.FindNr(this.data.RegimeObj[this.data.MapObj[0].HexObj[index32, index33].Regime].id);
            if (index14 > -1)
              mask2.Value[index32, index33] = index14 >= num38 ? 0 : 1;
          }
        }
      }
      mask2.AddValue(addvalue, 1);
      if (maxy1 > 0)
        addvalue.ExpandValueForAnyRegimeWithinMask(ref mask2, maxy1);
      let mut num84: i32 =  0;
      maxy2: i32;
      if ( val2_1 < 0.4)
        maxy2 = 0;
      else if ( num84 > 0.8)
      {
        maxy2 = 99;
      }
      else
      {
        maxy2 =  Math.Round(6.0 * (( val2_1 - 0.4) / 0.4));
        if (maxy2 > 5)
          maxy2 = 5;
        if (maxy2 < 1)
          maxy2 = 1;
      }
      if (maxy2 > 0)
        addvalue.ExpandValueForAnyRegime(maxy2);
      this.ai.MLAMatrix = addvalue;
      AIMatrix aiMatrix9 = new AIMatrix(ref this.ai);
      let mut mapWidth9: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index34: i32 =  0; index34 <= mapWidth9; index34 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index35: i32 =  0; index35 <= mapHeight; index35 += 1)
          aiMatrix9.Value[index34, index35] = 0;
      }
      let mut num85: i32 =  num22 - 1;
      for (let mut index36: i32 =  0; index36 <= num85; index36 += 1)
      {
        let mut idValue7: i32 =  simpleList2.Id[index36];
        let mut num86: i32 =  simpleList2.Weight[index36];
        let mut id9: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[stringListById1].GetData(0, idValue7, 6)));
        if (id9 > 0)
        {
          let mut locationById: i32 =  DrawMod.TGame.HandyFunctionsObj.GetLocationByID(id9);
          if (locationById > -1)
            aiMatrix9.Value[this.data.LocObj[locationById].X, this.data.LocObj[locationById].Y] = 1;
        }
      }
      aiMatrix9.ExpandAndAddValueForAnyRegime(6);
      let mut mapWidth10: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index37: i32 =  0; index37 <= mapWidth10; index37 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index38: i32 =  0; index38 <= mapHeight; index38 += 1)
        {
          if (aiMatrix9.Value[index37, index38] == 1)
            this.ai.VAR_MATRIX_RETREAT.Value[index37, index38] = 25;
          else if (aiMatrix9.Value[index37, index38] == 2)
            this.ai.VAR_MATRIX_RETREAT.Value[index37, index38] = 40;
          else if (aiMatrix9.Value[index37, index38] == 3)
            this.ai.VAR_MATRIX_RETREAT.Value[index37, index38] = 50;
          else if (aiMatrix9.Value[index37, index38] == 4)
            this.ai.VAR_MATRIX_RETREAT.Value[index37, index38] = 65;
          else if (aiMatrix9.Value[index37, index38] == 5)
            this.ai.VAR_MATRIX_RETREAT.Value[index37, index38] = 80;
        }
      }
      int[] numArray7 = new int[this.data.RegimeCounter + 1];
      int[] numArray8 = new int[this.data.RegimeCounter + 1];
      int[] numArray9 = new int[this.data.RegimeCounter + 1];
      let mut regimeCounter3: i32 =  this.data.RegimeCounter;
      for (tid1 = 1; tid1 <= regimeCounter3; tid1 += 1)
      {
        if (tid1 != this.data.Turn)
        {
          numArray7[tid1] =  Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[tid1].id, 1)));
          numArray8[tid1] =  Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[tid1].id, 2, "aiIntention", 3)));
          numArray9[tid1] =  Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[tid1].id, 2, "relation", 3)));
        }
      }
      AIMatrix aiMatrix10 = new AIMatrix(ref this.ai);
      let mut mapWidth11: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index39: i32 =  0; index39 <= mapWidth11; index39 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index40: i32 =  0; index40 <= mapHeight; index40 += 1)
        {
          aiMatrix10.Value[index39, index40] = 0;
          index14 = this.data.MapObj[0].HexObj[index39, index40].Regime;
          if (index14 > -1 && index14 != this.data.Turn)
          {
            let mut num87: i32 =  100;
            if (this.data.RegimeObj[this.data.Turn].RegimeRel[index14] == 0)
              num87 = numArray7[index14] != 1 ? 100 : 50;
            else if (numArray7[index14] == 1)
              num87 = !(numArray8[index14] < 21 | numArray9[index14] < 20) ? (!(numArray8[index14] < 29 | numArray9[index14] < 29) ? (!(numArray8[index14] < 37 | numArray9[index14] < 37) ? (!(numArray8[index14] < 45 | numArray9[index14] < 45) ? 400 : 250) : 150) : 100) : 80;
            else if (numArray7[index14] != 4)
              num87 = !(numArray8[index14] < 21 | numArray9[index14] < 14) ? (!(numArray8[index14] < 29 | numArray9[index14] < 20) ? (!(numArray8[index14] < 37 | numArray9[index14] < 25) ? (!(numArray8[index14] < 45 | numArray9[index14] < 30) ? 800 : 400) : 250) : 170) : 125;
            else if (numArray7[index14] == 4)
              num87 = 400;
            if (this.data.RegimeObj[this.data.Turn].RegimeRel[index14] == 0)
            {
              let mut nr: i32 =  simpleList5.FindNr(this.data.RegimeObj[index14].id);
              if (nr > -1 & nr < num38)
              {
                num87 =  Math.Round( num87 / 2.0);
                if (num87 < 25)
                  num87 = 25;
              }
            }
            if (this.data.MapObj[0].HexObj[index39, index40].UnitCounter <= -1)
              num87 = 0;
            aiMatrix10.Value[index39, index40] = num87;
          }
        }
      }
      aiMatrix10.ExpandValueForSameRegime(19);
      let mut mapWidth12: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index41: i32 =  0; index41 <= mapWidth12; index41 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index42: i32 =  0; index42 <= mapHeight; index42 += 1)
        {
          if (this.data.MapObj[0].HexObj[index41, index42].Regime != this.data.Turn & this.data.MapObj[0].HexObj[index41, index42].Regime > -1 && aiMatrix10.Value[index41, index42] == 0)
            aiMatrix10.Value[index41, index42] = 400;
        }
      }
      aiMatrix10.ExpandValueForAnyRegime(1);
      let mut mapWidth13: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index43: i32 =  0; index43 <= mapWidth13; index43 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index44: i32 =  0; index44 <= mapHeight; index44 += 1)
        {
          if (this.data.MapObj[0].HexObj[index43, index44].Regime != this.data.Turn)
            aiMatrix10.Value[index43, index44] = 0;
        }
      }
      aiMatrix10.ExpandValueForSameRegime(99);
      AIMatrix aiMatrix11 = aiMatrix10.AverageValuesForSameRegime(2, mask1);
      aiMatrix11.ExpandValueForAnyRegime(20);
      if (this.ai.VAR_DEBUG_ON)
        this.ai.Screenshot("STRENGTH_MATRIX", ref aiMatrix11.Value);
      let mut mapWidth14: i32 =  this.data.MapObj[0].MapWidth;
      for (let mut index45: i32 =  0; index45 <= mapWidth14; index45 += 1)
      {
        let mut mapHeight: i32 =  this.data.MapObj[0].MapHeight;
        for (let mut index46: i32 =  0; index46 <= mapHeight; index46 += 1)
          this.ai.VAR_MATRIX_STRENGTH.Value[index45, index46] = aiMatrix11.Value[index45, index46];
      }
      this.ai.VAR_STRENGTH_MOD_IS_ALSO_COMBAT_ADV_MOD = false;
      this.ai.WriteLog("0009_CustomMatrixes");
    }

    pub fn CustomRuleHQtoFrontAssign_howmanySSHQperDefensiveZone(defzoneNumber: i32) -> i32 => 9;

    pub float CustomRuleHQtoFrontAssign_ModifyScore1(
      ref AIMatrix frontlines,
      ref AIFront tempFront,
      hq: i32,
      sshq: i32,
      curEnm: i32,
      curFr: i32)
    {
      return 1f;
    }

    pub CustomRuleHQToFrontAssign_SetScore: i32(
      totalScore: i32,
      float score1,
      float score2,
      float score3)
    {
      let mut num1: i32 =   Math.Round( score1);
      let mut num2: i32 =   Math.Round( score2);
      let mut num3: i32 =   Math.Round( score3);
      if (num1 > 30 & num1 < 999)
        num1 =  Math.Round(30.0 +  (num1 - 30) * 0.8);
      if (num1 > 60 & num1 < 999)
        num1 =  Math.Round(60.0 +  (num1 - 60) * 0.6);
      if (num1 > 90 & num1 < 999)
        num1 =  Math.Round(90.0 +  (num1 - 90) * 0.4);
      if (num1 > 120 & num1 < 999)
        num1 =  Math.Round(120.0 +  (num1 - 120) * 0.2);
      if (num2 > 20 & num2 < 999)
        num2 =  Math.Round(20.0 +  (num2 - 20) * 0.66);
      if (num2 > 40 & num2 < 999)
        num2 =  Math.Round(40.0 +  (num2 - 40) * 0.33);
      if (num2 > 60 & num2 < 999)
        num2 =  Math.Round(60.0 +  (num2 - 60) * 0.33);
      if (num2 > 80 & num2 < 999)
        num2 =  Math.Round(80.0 +  (num2 - 80) * 0.2);
      return  Math.Round( (num1 * num2) *  score3 / 4.0);
    }

    pub float GetUnitPowerModifier(unr: i32)
    {
      let mut num1: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 156, 0, 0))].GetData(0, 42, 2)));
      let mut regime: i32 =  this.data.UnitObj[unr].Regime;
      float unitPowerModifier;
      if (DrawMod.TGame.HandyFunctionsObj.IsAlliedOrSelf(regime, this.data.Turn))
      {
        unitPowerModifier = 1f;
      }
      else
      {
        if (this.data.RegimeObj[this.data.Turn].RegimeRel[regime] == 0)
        {
          unitPowerModifier = 1f;
        }
        else
        {
          let mut num2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[regime].id, 2, "aiIntention", 3)));
          let mut num3: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotregregkeys].GetData3(0, this.data.RegimeObj[this.data.Turn].id, 1, this.data.RegimeObj[regime].id, 2, "relation", 3)));
          if (num3 > 0)
            num3 = num3;
          if (num3 > num2)
            num3 =  Math.Round( (num3 + num2) / 2.0);
          else if (num2 > num3 & this.data.RegimeObj[regime].AI && num1 > 10)
            num3 =  Math.Round( (num3 + num2) / 2.0);
          float num4 = 1f -  num3 / 100f;
          unitPowerModifier = !(num2 < 10 | num3 < 10) ? (!(num2 < 20 | num3 < 20) ? (!(num2 < 25 | num3 < 25) ? (!(num2 < 30 | num3 < 30) ? (!(num2 < 35 | num3 < 35) ? (!(num2 < 40 | num3 < 40) ? (!(num2 < 40 | num3 < 40) ? (!(num2 < 50 | num3 < 50) ? (!(num2 < 60 | num3 < 60) ? num4 * 0.05f : num4 * 0.1f) : num4 * 0.15f) : num4 * 0.23f) : num4 * 0.3f) : num4 * 0.4f) : num4 * 0.52f) : num4 * 0.65f) : num4 * 0.8f) : num4 * 1f;
        }
        switch ( Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[regime].id, 1))))
        {
          case 2:
            unitPowerModifier *= 0.7f;
            break;
          case 4:
            unitPowerModifier *= 0.35f;
            break;
        }
      }
      if ( unitPowerModifier < 0.1)
        unitPowerModifier = 0.1f;
      return unitPowerModifier;
    }

    pub CustomRuleHQtoFrontAssign_UnitInCorrectFront_SeeSSHQasSeperateHQ: bool(unr: i32) => this.data.UnitObj[unr].IsHQ ? this.data.UnitObj[unr].HQ == -1 : this.data.UnitObj[unr].HQ > -1 && this.data.UnitObj[this.data.UnitObj[unr].HQ].HQ <= -1;

    pub fn CustomRuleWorld_ExtraTroopsOnHex(x: i32, y: i32, curTroops: i32) -> i32
    {
      let mut regime: i32 =  this.data.MapObj[0].HexObj[x, y].Regime;
      let mut num1: i32 =  0;
      if (regime > 0 && this.data.RegimeObj[this.data.Turn].RegimeRel[regime] == 0 & this.data.Turn != regime &&  Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[this.data.Turn].id, 1))) == 1)
      {
        let mut num2: i32 =   Math.Round(Conversion.Val(this.data.StringListObj[this.slotRegimes].GetData(0, this.data.RegimeObj[regime].id, 1)));
        if (curTroops < 1 & DrawMod.RandyNumber.Next(0, 100) < 20)
          num1 = 5;
      }
      return num1;
    }

    pub HasCustumCalls: bool() => true;
  }
}
