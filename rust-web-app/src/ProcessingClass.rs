// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ProcessingClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class ProcessingClass
  {
     game: GameClass;
     int[,,] cacheLIShistory;
     int[,,] cacheLISpoints;
     int[,,] cacheLIStotalHistory;
     int[,,] cacheLISorganic;
     int[,,] cacheLISorganicPercentage;
     int[,,] cacheLISpull;
     int[,,] tempLISwithoutLogExt;
    pub plogcounter: i32;
    pub plog: Vec<String>;

    pub ProcessingClass(tgame: GameClass)
    {
      this.cacheLIShistory = new int[2, 2, 2];
      this.cacheLISpoints = new int[2, 2, 2];
      this.cacheLIStotalHistory = new int[2, 2, 2];
      this.cacheLISorganic = new int[2, 2, 2];
      this.cacheLISorganicPercentage = new int[2, 2, 2];
      this.cacheLISpull = new int[2, 2, 2];
      this.tempLISwithoutLogExt = new int[2, 2, 2];
      this.plog = new string[1];
      this.game = tgame;
      this.plogcounter = -1;
    }

    pub AutoMoveChange: String(
      unr: i32,
      bool setAutoMove,
      bool subordinates,
      targetX: i32,
      targetY: i32)
    {
      let mut num1: i32 = 0;
      let mut num2: i32 = -1;
      if (unr > -1)
        num2 = !this.game.Data.UnitObj[unr].IsHQ ? this.game.Data.UnitObj[unr].HQ : unr;
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
      {
        if (index == unr | this.game.Data.UnitObj[index].HQ == num2 & subordinates && this.game.Data.UnitObj[index].PreDef == -1 & this.game.Data.UnitObj[index].X > -1)
        {
          if (setAutoMove)
          {
            if (!(this.game.Data.UnitObj[index].X == targetX & this.game.Data.UnitObj[index].Y == targetY))
            {
              this.game.Data.UnitObj[index].autoMoveX = targetX;
              this.game.Data.UnitObj[index].autoMoveY = targetY;
              num1 += 1;
            }
            else
            {
              this.game.Data.UnitObj[index].autoMoveX = -1;
              this.game.Data.UnitObj[index].autoMoveY = -1;
            }
          }
          else
          {
            this.game.Data.UnitObj[index].autoMoveX = -1;
            this.game.Data.UnitObj[index].autoMoveY = -1;
            num1 += 1;
          }
        }
      }
      str1: String = "We changed auto-move setting of " + this.game.Data.UnitObj[unr].Name;
      str2: String = num1 <= 1 ? (num1 != 0 ? str1 + ".\r\n" : str1 + ". No units where affected.") : str1 + " and " + (num1 - 1).ToString() + " other subordinate unit(s).\r\n";
      if (setAutoMove)
        str2 += this.AutoMoveExecute(unr, subordinates);
      return str2;
    }

    pub AutoMoveExecute: String(unr: i32, bool alsoSubordinates, bool calledFromStartTurn = false)
    {
      str: String = "";
      if (Information.IsNothing( this.game.EditObj.TempValue[0]))
        this.game.HandyFunctionsObj.RedimTempValue(9999);
      if (Information.IsNothing( this.game.EditObj.TempValueSpecial[0]))
        this.game.HandyFunctionsObj.RedimTempValueSpecial(0);
      if (Information.IsNothing( this.game.EditObj.TempValueSpecial2[0]))
        this.game.HandyFunctionsObj.RedimTempValueSpecial2(0);
      if (Information.IsNothing( this.game.EditObj.TempValue2[0]))
        this.game.HandyFunctionsObj.RedimTempValue2(9999);
      if (Information.IsNothing( this.game.EditObj.TempCameFrom[0]))
        this.game.HandyFunctionsObj.RedimTempCameFrom();
      if (Information.IsNothing( this.game.EditObj.TempAttack[0]))
        this.game.HandyFunctionsObj.RedimTempAttack(true);
      MapMatrix2 mapMatrix2_1 = this.game.EditObj.TempValue[0].Clone();
      MapMatrix2Coordinate matrix2Coordinate = this.game.EditObj.TempCameFrom[0].Clone();
      MapMatrix2 mapMatrix2_2 = this.game.EditObj.TempValueSpecial[0].Clone();
      MapMatrix2 mapMatrix2_3 = this.game.EditObj.TempValueSpecial2[0].Clone();
      MapMatrix2 mapMatrix2_4 = this.game.EditObj.TempValue2[0].Clone();
      MapMatrix2Plus6 mapMatrix2Plus6 = this.game.EditObj.TempAttack[0].Clone();
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      let mut num1: i32 = -1;
      if (unr > -1)
        num1 = !this.game.Data.UnitObj[unr].IsHQ ? this.game.Data.UnitObj[unr].HQ : unr;
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
      {
        if (unr == -1 | index == unr | this.game.Data.UnitObj[index].HQ == num1 & alsoSubordinates && this.game.Data.UnitObj[index].PreDef == -1 & this.game.Data.UnitObj[index].X > -1 & this.game.Data.UnitObj[index].Regime == this.game.Data.Turn && this.game.Data.UnitObj[index].autoMoveX > -1 && this.game.Data.UnitObj[index].Historical > -1)
        {
          if (this.game.Data.UnitObj[index].autoMoveX > this.game.Data.MapObj[0].MapWidth | this.game.Data.UnitObj[index].autoMoveY > this.game.Data.MapObj[0].MapHeight)
          {
            str = str + this.game.Data.UnitObj[index].Name + " auto-move cancelled because impossible destination.\r\n";
            this.game.Data.UnitObj[index].autoMoveX = -1;
            this.game.Data.UnitObj[index].autoMoveY = -1;
          }
          else
          {
            simpleList1.Add(this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index].Historical].ID, 1, this.game.Data.UnitObj[index].HistoricalSubPart, this.game.Data.UnitObj[index].autoMoveX, this.game.Data.UnitObj[index].autoMoveY, CheckExistence: false);
            simpleList2.AddWeight(this.game.Data.UnitObj[index].autoMoveX * 1000 + this.game.Data.UnitObj[index].autoMoveY, 1, this.game.Data.UnitObj[index].autoMoveX, this.game.Data.UnitObj[index].autoMoveY);
          }
        }
      }
      let mut counter1: i32 = simpleList2.Counter;
      for (let mut index1: i32 = 0; index1 <= counter1; index1 += 1)
      {
        let mut targetX: i32 = simpleList2.Data1[index1];
        let mut targetY: i32 = simpleList2.Data2[index1];
        AIMatrix aiMatrix = this.game.HandyFunctionsObj.AutoMoveLongRangeMatrix(targetX, targetY, this.game.Data.MapObj[0].MapWidth, this.game.Data.MapObj[0].MapHeight);
        let mut counter2: i32 = simpleList1.Counter;
        for (let mut index2: i32 = 0; index2 <= counter2; index2 += 1)
        {
          unr = this.game.HandyFunctionsObj.GetUnitByHistorical(this.game.HandyFunctionsObj.GetHistoricalUnitByID(simpleList1.Id[index2]), simpleList1.Data1[index2]);
          if (unr > -1 && this.game.Data.UnitObj[unr].attachedTo == -1)
          {
            let mut autoMoveX: i32 = this.game.Data.UnitObj[unr].autoMoveX;
            let mut autoMoveY: i32 = this.game.Data.UnitObj[unr].autoMoveY;
            if (autoMoveX == targetX & autoMoveY == targetY && !(autoMoveX == this.game.Data.UnitObj[unr].X & autoMoveY == this.game.Data.UnitObj[unr].Y))
            {
              this.game.HandyFunctionsObj.MakeMovePrediction(unr, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, increaseap: Math.Max(100 - this.game.HandyFunctionsObj.GetLowestAp(unr), 0), ismove: true);
              let mut num2: i32 = 99999;
              let mut index3: i32 = -1;
              bool flag1 = false;
              let mut lowestAp: i32 = this.game.HandyFunctionsObj.GetLowestAp(unr);
              let mut mapWidth: i32 = this.game.Data.MapObj[0].MapWidth;
              index4: i32;
              for (let mut index5: i32 = 0; index5 <= mapWidth; index5 += 1)
              {
                let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
                for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
                {
                  if (aiMatrix.Value[index5, index6] > 0 && aiMatrix.Value[index5, index6] < num2)
                  {
                    if (this.game.EditObj.TempValue[0].Value[index5, index6] <= lowestAp)
                    {
                      if (this.game.Data.MapObj[0].HexObj[index5, index6].UnitCounter < 15)
                      {
                        num2 = aiMatrix.Value[index5, index6];
                        index3 = index5;
                        index4 = index6;
                      }
                    }
                    else
                      flag1 = true;
                  }
                }
              }
              if (index3 > -1)
              {
                if (index3 > -1 & this.game.EditObj.TempValue[0].Value[index3, index4] < 9999 & this.game.EditObj.TempCameFrom[0].Value[index3, index4].onmap)
                {
                  Coordinate coordinate;
                  coordinate.x = index3;
                  coordinate.y = index4;
                  coordinate.onmap = true;
                  bool flag2 = false;
                  let mut num3: i32 = 0;
                  while (coordinate.onmap)
                  {
                    num3 += 1;
                    if (lowestAp >= this.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y])
                    {
                      coordinate.onmap = false;
                      flag2 = true;
                    }
                    else
                      coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y];
                    if (num3 > 9999)
                      break;
                  }
                  if (flag2 && !(coordinate.x == this.game.Data.UnitObj[unr].X & coordinate.y == this.game.Data.UnitObj[unr].Y))
                  {
                    OrderResult orderResult = this.ExecuteMovement(unr, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0, coordinate.x, coordinate.y, 0, true, calledFromStartTurn);
                    if (orderResult.BattleIntercept)
                      str = str + "Tried moving " + this.game.Data.UnitObj[unr].Name + " to " + coordinate.x.ToString() + "," + coordinate.y.ToString() + " but suffered intercept fire on " + orderResult.BattleX.ToString() + "," + orderResult.BattleY.ToString() + ".\r\n";
                    else if (orderResult.BattleUnit > -1)
                      str = str + "Tried moving " + this.game.Data.UnitObj[unr].Name + " to " + coordinate.x.ToString() + "," + coordinate.y.ToString() + " but ran into an ambush on " + orderResult.BattleX.ToString() + "," + orderResult.BattleY.ToString() + ".\r\n";
                    else
                      str = str + "Moved " + this.game.Data.UnitObj[unr].Name + " to " + coordinate.x.ToString() + "," + coordinate.y.ToString() + ".\r\n";
                  }
                  if (this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0, autoMoveX, autoMoveY, 0, 2) <= 2)
                  {
                    str = str + this.game.Data.UnitObj[unr].Name + " arrived at (or close to) destination.\r\n";
                    this.game.Data.UnitObj[unr].autoMoveX = -1;
                    this.game.Data.UnitObj[unr].autoMoveY = -1;
                  }
                }
              }
              else if (!flag1)
              {
                if (this.game.Data.UnitObj[unr].attachedTo == -1)
                {
                  str = str + this.game.Data.UnitObj[unr].Name + " did not see a movement path to " + autoMoveX.ToString() + "," + autoMoveY.ToString() + " and dropped out of auto-move.\r\n";
                  this.game.Data.UnitObj[unr].autoMoveX = -1;
                  this.game.Data.UnitObj[unr].autoMoveY = -1;
                }
              }
              else
                str = str + this.game.Data.UnitObj[unr].Name + " did not have the AP to advance at this moment.\r\n";
            }
          }
        }
      }
      this.game.EditObj.TempValue[0] = mapMatrix2_1;
      this.game.EditObj.TempCameFrom[0] = matrix2Coordinate;
      this.game.EditObj.TempValueSpecial[0] = mapMatrix2_2;
      this.game.EditObj.TempValueSpecial2[0] = mapMatrix2_3;
      this.game.EditObj.TempValue2[0] = mapMatrix2_4;
      this.game.EditObj.TempAttack[0] = mapMatrix2Plus6;
      return str;
    }

    pub fn MakeNewSFTypeModel(sftypenr: i32)
    {
      let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID(this.game.Data.SFTypeObj[sftypenr].ModelNameList);
      if (stringListById == -1)
        return;
      let mut num1: i32 = 0;
      let mut num2: i32 = 0;
      Right: String;
      while (num1 == 0 & num2 < 999)
      {
        let mut randomFromStringList: i32 = this.GetRandomFromStringList(stringListById);
        num2 += 1;
        Right = this.game.Data.StringListObj[stringListById].Data[randomFromStringList, 1];
        let mut num3: i32 = -1;
        let mut sfTypeCounter: i32 = this.game.Data.SFTypeCounter;
        for (let mut index: i32 = 0; index <= sfTypeCounter; index += 1)
        {
          if (Operators.CompareString(this.game.Data.SFTypeObj[index].ModelName, Right, false) == 0)
          {
            num3 = index;
            break;
          }
        }
        if (num3 == -1)
          num1 = 1;
      }
      this.game.Data.AddSFType();
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter] = this.game.Data.SFTypeObj[sftypenr].Clone();
      let mut tv10: i32 = -1;
      if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType > -1)
      {
        let mut modelItemType: i32 = this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType;
        this.game.Data.AddItemType();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter] = this.game.Data.ItemTypeObj[modelItemType].Clone();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].RegimeSpecific = this.game.Data.Turn;
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].IsSFType = this.game.Data.SFTypeCounter;
        this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType = this.game.Data.ItemTypeCounter;
        tv10 = this.game.Data.ItemTypeCounter;
      }
      let mut sfTypeCounter1: i32 = this.game.Data.SFTypeCounter;
      let mut tempNewLevels: i32 = this.game.Data.SFTypeObj[sftypenr].TempNewLevels;
      let mut tv8_1: i32 = 1;
      this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[sftypenr].ModelNewEvent, tv9: sfTypeCounter1, tv7: tempNewLevels, tv8: tv8_1, tv10: tv10);
      let mut sfTypeCounter2: i32 = this.game.Data.SFTypeCounter;
      let mut tv7: i32 = 0;
      let mut tv8_2: i32 = 0;
      let mut researchCounter: i32 = this.game.Data.ResearchCounter;
      for (let mut index: i32 = 0; index <= researchCounter; index += 1)
      {
        if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] == 0 & this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index] > 0 && this.game.Data.RegimeObj[this.game.Data.Turn].ResField[index])
        {
          this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index], tv9: sfTypeCounter2, tv7: tv7, tv8: tv8_2, tv10: tv10);
          this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] = 1;
        }
      }
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelName = Right;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].Name = Right;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelIsBase = false;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelBaseModel = sftypenr;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLevel = this.game.Data.SFTypeObj[sftypenr].TempNewLevels;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelMark = 1;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelVersion = 1;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelRegime = this.game.Data.Turn;
      this += 1.game.Data.SFModelIDCounter;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelID = this.game.Data.SFModelIDCounter;
      if (tv10 > -1)
        this.game.Data.ItemTypeObj[tv10].Name = Right;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].LoadSprites();
      if (this.game.Data.SFTypeObj[sftypenr].ModelCostType == -1)
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut turn: i32 = this.game.Data.Turn;
        let mut index: i32 = turn;
        regimeClassArray[index].ResPts = regimeObj[turn].ResPts - this.game.Data.SFTypeObj[sftypenr].TempNewCost;
      }
      else
      {
        int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
        int[] numArray = regimeSlot;
        let mut modelCostType: i32 = this.game.Data.SFTypeObj[sftypenr].ModelCostType;
        let mut index: i32 = modelCostType;
        let mut num4: i32 = regimeSlot[modelCostType] - this.game.Data.SFTypeObj[sftypenr].TempNewCost;
        numArray[index] = num4;
      }
    }

    pub fn AttachUnit(attachUnr: i32, transportUnr: i32)
    {
      this.game.Data.UnitObj[transportUnr].AddTransport(attachUnr);
      this.game.Data.UnitObj[attachUnr].moveMode = this.game.Data.UnitObj[transportUnr].moveMode;
      this.game.Data.UnitObj[attachUnr].attachedTo = transportUnr;
      this.game.Data.UnitObj[attachUnr].FreeCombatX = -1;
      this.game.Data.UnitObj[attachUnr].FreeCombatY = -1;
    }

    pub fn DetachUnit(detachUnr: i32, transportUnr: i32)
    {
      this.game.Data.UnitObj[transportUnr].RemoveTransport(detachUnr);
      this.game.Data.UnitObj[detachUnr].attachedTo = -1;
    }

    pub fn MakeInitialModels()
    {
      let mut sfTypeCounter: i32 = this.game.Data.SFTypeCounter;
      for (let mut sftypenr: i32 = 0; sftypenr <= sfTypeCounter; sftypenr += 1)
      {
        if (this.game.Data.SFTypeObj[sftypenr].ModelInitialForAll)
        {
          let mut regimeCounter: i32 = this.game.Data.RegimeCounter;
          for (let mut regnr: i32 = 0; regnr <= regimeCounter; regnr += 1)
            this.MakeInitialSFTypeModel(sftypenr, regnr);
          this.game.Data.SFTypeObj[sftypenr].ModelRegime = -2;
        }
      }
    }

    pub fn MakeInitialSFTypeModel(sftypenr: i32, regnr: i32)
    {
      this.game.Data.AddSFType();
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter] = this.game.Data.SFTypeObj[sftypenr].Clone();
      let mut tv10: i32 = -1;
      if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType > -1)
      {
        let mut modelItemType: i32 = this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType;
        this.game.Data.AddItemType();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter] = this.game.Data.ItemTypeObj[modelItemType].Clone();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].RegimeSpecific = regnr;
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].IsSFType = this.game.Data.SFTypeCounter;
        this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType = this.game.Data.ItemTypeCounter;
        tv10 = this.game.Data.ItemTypeCounter;
      }
      if (this.game.Data.SFTypeObj[sftypenr].ModelInitialEvent > -1)
      {
        let mut sfTypeCounter: i32 = this.game.Data.SFTypeCounter;
        let mut tv7: i32 = 0;
        let mut tv8: i32 = 0;
        this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[sftypenr].ModelInitialEvent, tv9: sfTypeCounter, tv7: tv7, tv8: tv8, tv10: tv10);
      }
      let mut sfTypeCounter1: i32 = this.game.Data.SFTypeCounter;
      let mut tv7_1: i32 = 0;
      let mut tv8_1: i32 = 0;
      let mut researchCounter: i32 = this.game.Data.ResearchCounter;
      for (let mut index: i32 = 0; index <= researchCounter; index += 1)
      {
        if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] == 0 & this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index] > 0 && this.game.Data.RegimeObj[regnr].ResField[index])
        {
          this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index], tv9: sfTypeCounter1, tv7: tv7_1, tv8: tv8_1, tv10: tv10);
          this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] = 1;
        }
      }
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelRegime = regnr;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelIsBase = false;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].LoadSprites();
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index1: i32 = 0; index1 <= unitCounter; index1 += 1)
      {
        if (this.game.Data.UnitObj[index1].PreDef == -1 & this.game.Data.UnitObj[index1].Regime == regnr)
        {
          let mut sfCount: i32 = this.game.Data.UnitObj[index1].SFCount;
          for (let mut index2: i32 = 0; index2 <= sfCount; index2 += 1)
          {
            let mut sf: i32 = this.game.Data.UnitObj[index1].SFList[index2];
            if (this.game.Data.SFObj[sf].Type == sftypenr)
              this.game.Data.SFObj[sf].Type = this.game.Data.SFTypeCounter;
          }
        }
      }
    }

    pub fn MakeSFTypeModelImprovement(sftypenr: i32)
    {
      modelName: String = this.game.Data.SFTypeObj[sftypenr].ModelName;
      this.game.Data.AddSFType();
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter] = this.game.Data.SFTypeObj[sftypenr].Clone();
      let mut tv10: i32 = -1;
      if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType > -1)
      {
        let mut modelItemType: i32 = this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType;
        this.game.Data.AddItemType();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter] = this.game.Data.ItemTypeObj[modelItemType].Clone();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].RegimeSpecific = this.game.Data.Turn;
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].IsSFType = this.game.Data.SFTypeCounter;
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].Blocks = this.game.Data.SFTypeObj[sftypenr].ModelItemType;
        this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType = this.game.Data.ItemTypeCounter;
        tv10 = this.game.Data.ItemTypeCounter;
      }
      let mut sfTypeCounter1: i32 = this.game.Data.SFTypeCounter;
      let mut tv7: i32 = 0;
      let mut tv8: i32 = 0;
      let mut num1: i32 = 0;
      let mut researchCounter: i32 = this.game.Data.ResearchCounter;
      for (let mut index: i32 = 0; index <= researchCounter; index += 1)
      {
        if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] == 0 & this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index] > 0 && this.game.Data.RegimeObj[this.game.Data.Turn].ResField[index])
        {
          if (this.game.Data.ResearchObj[index].UpgradeCost == -1)
            num1 = -1;
          else if (num1 > -1)
            num1 += this.game.Data.ResearchObj[index].UpgradeCost;
          this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index], tv9: sfTypeCounter1, tv7: tv7, tv8: tv8, tv10: tv10);
          this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] = 1;
        }
      }
      if (num1 > -1)
      {
        this.game.Data.SFTypeObj[sftypenr].UpgradeToo = this.game.Data.SFTypeCounter;
        this.game.Data.SFTypeObj[sftypenr].UpgradeCost =  Math.Round( ( num1 * this.game.Data.SFTypeObj[sftypenr].ModelSFTypeUpgrade));
      }
      else
      {
        this.game.Data.SFTypeObj[sftypenr].UpgradeToo = -1;
        this.game.Data.SFTypeObj[sftypenr].UpgradeCost = 0;
      }
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelName = modelName;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].Name = modelName;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelIsBase = false;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelVersion = this.game.Data.SFTypeObj[sftypenr].ModelVersion + 1;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelRegime = this.game.Data.Turn;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelID = this.game.Data.SFTypeObj[sftypenr].ModelID;
      if (tv10 > -1)
        this.game.Data.ItemTypeObj[tv10].Name = modelName;
      let mut num2: i32 = 1;
      let mut sfTypeCounter2: i32 = this.game.Data.SFTypeCounter;
      for (let mut index: i32 = 0; index <= sfTypeCounter2; index += 1)
      {
        if (this.game.Data.SFTypeObj[index].ModelID == this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelID && this.game.Data.SFTypeObj[index].ModelMark > num2)
          num2 = this.game.Data.SFTypeObj[index].ModelMark;
      }
      let mut sfTypeCounter3: i32 = this.game.Data.SFTypeCounter;
      for (let mut index: i32 = 0; index <= sfTypeCounter3; index += 1)
      {
        if (this.game.Data.SFTypeObj[index].ModelID == this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelID && this.game.Data.SFTypeObj[index].ModelMark == this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelMark)
        {
          str: String;
          if (num2 > 1)
            str = Conversions.ToString(Operators.AddObject( this.game.HandyFunctionsObj.GetRomanNumerical(this.game.Data.SFTypeObj[index].ModelMark), NewLateBinding.LateGet( null, typeof (Strings), "LCase", new object[1]
            {
              RuntimeHelpers.GetObjectValue(this.game.HandyFunctionsObj.GetAlphabetLetter(this.game.Data.SFTypeObj[index].ModelVersion))
            }, (string[]) null, (System.Type[]) null, (bool[]) null)));
          else
            str = Conversions.ToString(Operators.AddObject( "", NewLateBinding.LateGet( null, typeof (Strings), "UCase", new object[1]
            {
              RuntimeHelpers.GetObjectValue(this.game.HandyFunctionsObj.GetAlphabetLetter(this.game.Data.SFTypeObj[index].ModelVersion))
            }, (string[]) null, (System.Type[]) null, (bool[]) null)));
          this.game.Data.SFTypeObj[index].Name = modelName + " " + str;
          if (this.game.Data.SFTypeObj[index].ModelItemType > -1)
            this.game.Data.ItemTypeObj[this.game.Data.SFTypeObj[index].ModelItemType].Name = this.game.Data.SFTypeObj[index].Name;
        }
      }
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].LoadSprites();
      if (this.game.Data.SFTypeObj[sftypenr].ModelCostType == -1)
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut turn: i32 = this.game.Data.Turn;
        let mut index: i32 = turn;
        regimeClassArray[index].ResPts = regimeObj[turn].ResPts - this.game.Data.SFTypeObj[sftypenr].TempImproveCost;
      }
      else
      {
        int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
        int[] numArray = regimeSlot;
        let mut modelCostType: i32 = this.game.Data.SFTypeObj[sftypenr].ModelCostType;
        let mut index: i32 = modelCostType;
        let mut num3: i32 = regimeSlot[modelCostType] - this.game.Data.SFTypeObj[sftypenr].TempImproveCost;
        numArray[index] = num3;
      }
    }

    pub fn MakeSFTypeModelUpgrade(sftypenr: i32)
    {
      modelName: String = this.game.Data.SFTypeObj[sftypenr].ModelName;
      this.game.Data.AddSFType();
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter] = this.game.Data.SFTypeObj[sftypenr].Clone();
      let mut tv10: i32 = -1;
      if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType > -1)
      {
        let mut modelItemType: i32 = this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType;
        this.game.Data.AddItemType();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter] = this.game.Data.ItemTypeObj[modelItemType].Clone();
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].RegimeSpecific = this.game.Data.Turn;
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].IsSFType = this.game.Data.SFTypeCounter;
        this.game.Data.ItemTypeObj[this.game.Data.ItemTypeCounter].Blocks = this.game.Data.SFTypeObj[sftypenr].ModelItemType;
        this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelItemType = this.game.Data.ItemTypeCounter;
        tv10 = this.game.Data.ItemTypeCounter;
      }
      let mut sfTypeCounter1: i32 = this.game.Data.SFTypeCounter;
      let mut tempUpgradeLevels: i32 = this.game.Data.SFTypeObj[sftypenr].TempUpgradeLevels;
      let mut tv8_1: i32 = 0;
      this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[sftypenr].ModelNewEvent, tv9: sfTypeCounter1, tv7: tempUpgradeLevels, tv8: tv8_1, tv10: tv10);
      let mut sfTypeCounter2: i32 = this.game.Data.SFTypeCounter;
      let mut tv7: i32 = 0;
      let mut tv8_2: i32 = 0;
      let mut researchCounter: i32 = this.game.Data.ResearchCounter;
      for (let mut index: i32 = 0; index <= researchCounter; index += 1)
      {
        if (this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] == 0 & this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index] > 0 && this.game.Data.RegimeObj[this.game.Data.Turn].ResField[index])
        {
          this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelImproveEvent[index], tv9: sfTypeCounter2, tv7: tv7, tv8: tv8_2, tv10: tv10);
          this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelLastState[index] = 1;
        }
      }
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelName = modelName;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].Name = modelName;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelIsBase = false;
      SFTypeClass[] sfTypeObj1 = this.game.Data.SFTypeObj;
      SFTypeClass[] sfTypeClassArray1 = sfTypeObj1;
      let mut sfTypeCounter3: i32 = this.game.Data.SFTypeCounter;
      let mut index1: i32 = sfTypeCounter3;
      sfTypeClassArray1[index1].ModelLevel = sfTypeObj1[sfTypeCounter3].ModelLevel + this.game.Data.SFTypeObj[sftypenr].TempUpgradeLevels;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelMark = this.game.Data.SFTypeObj[sftypenr].ModelMark + 1;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelVersion = 1;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelRegime = this.game.Data.Turn;
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelID = this.game.Data.SFTypeObj[sftypenr].ModelID;
      if (tv10 > -1)
        this.game.Data.ItemTypeObj[tv10].Name = modelName;
      let mut num1: i32 = 1;
      let mut sfTypeCounter4: i32 = this.game.Data.SFTypeCounter;
      for (let mut index2: i32 = 0; index2 <= sfTypeCounter4; index2 += 1)
      {
        if (this.game.Data.SFTypeObj[index2].ModelID == this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelID && this.game.Data.SFTypeObj[index2].ModelVersion > num1)
          num1 = this.game.Data.SFTypeObj[index2].ModelVersion;
      }
      let mut sfTypeCounter5: i32 = this.game.Data.SFTypeCounter;
      for (let mut index3: i32 = 0; index3 <= sfTypeCounter5; index3 += 1)
      {
        if (this.game.Data.SFTypeObj[index3].ModelID == this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].ModelID)
        {
          romanNumerical: String = this.game.HandyFunctionsObj.GetRomanNumerical(this.game.Data.SFTypeObj[index3].ModelMark);
          this.game.Data.SFTypeObj[index3].Name = modelName + " " + romanNumerical;
          if (num1 > 1)
          {
            SFTypeClass[] sfTypeObj2 = this.game.Data.SFTypeObj;
            SFTypeClass[] sfTypeClassArray2 = sfTypeObj2;
            let mut index4: i32 = index3;
            let mut index5: i32 = index4;
            sfTypeClassArray2[index5].Name = Conversions.ToString(Operators.AddObject( sfTypeObj2[index4].Name, NewLateBinding.LateGet( null, typeof (Strings), "LCase", new object[1]
            {
              RuntimeHelpers.GetObjectValue(this.game.HandyFunctionsObj.GetAlphabetLetter(this.game.Data.SFTypeObj[index3].ModelVersion))
            }, (string[]) null, (System.Type[]) null, (bool[]) null)));
          }
          if (this.game.Data.SFTypeObj[index3].ModelItemType > -1)
            this.game.Data.ItemTypeObj[this.game.Data.SFTypeObj[index3].ModelItemType].Name = this.game.Data.SFTypeObj[index3].Name;
        }
      }
      this.game.Data.SFTypeObj[this.game.Data.SFTypeCounter].LoadSprites();
      if (this.game.Data.SFTypeObj[sftypenr].ModelCostType == -1)
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut turn: i32 = this.game.Data.Turn;
        let mut index6: i32 = turn;
        regimeClassArray[index6].ResPts = regimeObj[turn].ResPts - this.game.Data.SFTypeObj[sftypenr].TempUpgradeCost;
      }
      else
      {
        int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
        int[] numArray = regimeSlot;
        let mut modelCostType: i32 = this.game.Data.SFTypeObj[sftypenr].ModelCostType;
        let mut index7: i32 = modelCostType;
        let mut num2: i32 = regimeSlot[modelCostType] - this.game.Data.SFTypeObj[sftypenr].TempUpgradeCost;
        numArray[index7] = num2;
      }
    }

    pub fn SetUberOff()
    {
      let mut mapCounter: i32 = this.game.Data.MapCounter;
      for (let mut index1: i32 = 0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
          {
            if (this.game.Data.MapObj[index1].HexObj[index2, index3].TempOwner > -1)
              this.game.Data.MapObj[index1].HexObj[index2, index3].Regime = this.game.Data.MapObj[index1].HexObj[index2, index3].TempOwner;
          }
        }
      }
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
      {
        if (this.game.Data.UnitObj[index].TempOwner > -1)
          this.game.Data.UnitObj[index].Regime = this.game.Data.UnitObj[index].TempOwner;
      }
    }

    pub fn SetUberOn()
    {
      let mut mapCounter: i32 = this.game.Data.MapCounter;
      for (let mut index1: i32 = 0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
          {
            this.game.Data.MapObj[index1].HexObj[index2, index3].TempOwner = -1;
            let mut regime: i32 = this.game.Data.MapObj[index1].HexObj[index2, index3].Regime;
            if (regime > -1 && this.game.Data.RegimeObj[regime].UberRegime == this.game.Data.Turn)
            {
              this.game.Data.MapObj[index1].HexObj[index2, index3].TempOwner = this.game.Data.MapObj[index1].HexObj[index2, index3].Regime;
              this.game.Data.MapObj[index1].HexObj[index2, index3].Regime = this.game.Data.RegimeObj[regime].UberRegime;
            }
          }
        }
      }
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
      {
        let mut regime: i32 = this.game.Data.UnitObj[index].Regime;
        this.game.Data.UnitObj[index].TempOwner = -1;
        if (regime > -1 && this.game.Data.RegimeObj[regime].UberRegime == this.game.Data.Turn & this.game.Data.RegimeObj[regime].UberRegime > -1)
        {
          this.game.Data.UnitObj[index].TempOwner = regime;
          this.game.Data.UnitObj[index].Regime = this.game.Data.RegimeObj[regime].UberRegime;
        }
      }
    }

    pub fn GetRandomFromStringList(nr: i32, let mut coluse: i32 = 0) -> i32
    {
      let mut length1: i32 = this.game.Data.StringListObj[nr].Length;
      num1: i32;
      for (let mut index: i32 = 0; index <= length1; index += 1)
      {
        if (coluse == -1)
          num1 += 1;
        else
          num1 =  Math.Round( num1 + Conversion.Val(this.game.Data.StringListObj[nr].Data[index, coluse]));
      }
      if (num1 < 1)
        return -1;
      let mut num2: i32 =  Math.Round( (1f + Conversion.Int(VBMath.Rnd() *  num1)));
      let mut num3: i32 = 0;
      let mut length2: i32 = this.game.Data.StringListObj[nr].Length;
      for (let mut randomFromStringList: i32 = 0; randomFromStringList <= length2; randomFromStringList += 1)
      {
        if (coluse == -1)
          num3 += 1;
        else
          num3 =  Math.Round( num3 + Conversion.Val(this.game.Data.StringListObj[nr].Data[randomFromStringList, coluse]));
        if (num2 <= num3)
          return randomFromStringList;
      }
      return -1;
    }

    pub fn GetRandomFromStringList2(nr: i32, let mut coluse: i32 = 0, let mut blockcoluse: i32 = -1) -> i32
    {
      let mut length1: i32 = this.game.Data.StringListObj[nr].Length;
      num1: i32;
      for (let mut index: i32 = 0; index <= length1; index += 1)
      {
        bool flag = true;
        if (blockcoluse > -1 && Conversion.Val(this.game.Data.StringListObj[nr].Data[index, blockcoluse]) >= 1.0)
          flag = false;
        if (flag)
        {
          if (coluse == -1)
            num1 += 1;
          else
            num1 =  Math.Round( num1 + Conversion.Val(this.game.Data.StringListObj[nr].Data[index, coluse]));
        }
      }
      if (num1 < 1)
        return -1;
      let mut num2: i32 =  Math.Round( (1f + Conversion.Int(VBMath.Rnd() *  num1)));
      let mut num3: i32 = 0;
      let mut length2: i32 = this.game.Data.StringListObj[nr].Length;
      for (let mut randomFromStringList2: i32 = 0; randomFromStringList2 <= length2; randomFromStringList2 += 1)
      {
        bool flag = true;
        if (blockcoluse > -1 && Conversion.Val(this.game.Data.StringListObj[nr].Data[randomFromStringList2, blockcoluse]) >= 1.0)
          flag = false;
        if (flag)
        {
          if (coluse == -1)
            num3 += 1;
          else
            num3 =  Math.Round( num3 + Conversion.Val(this.game.Data.StringListObj[nr].Data[randomFromStringList2, coluse]));
          if (num2 <= num3)
            return randomFromStringList2;
        }
      }
      return -1;
    }

    pub fn SwapOfficerOtherData(regnr: i32, his1: i32, his2: i32, unr: i32,  tData: DataClass)
    {
      bool flag;
      if (Information.IsNothing( tData.HistoricalUnitObj[his1].CommanderName))
        flag = true;
      else if (tData.HistoricalUnitObj[his1].CommanderName.Length < 1)
        flag = true;
      if (tData.Round > 0)
      {
        if (!(tData.RegimeObj[tData.Turn].AI &  tData.RuleVar[914] == 1.0) && regnr > -1)
        {
          RegimeClass[] regimeObj1 = tData.RegimeObj;
          RegimeClass[] regimeClassArray1 = regimeObj1;
          let mut index1: i32 = regnr;
          let mut index2: i32 = index1;
          regimeClassArray1[index2].ResPts =  Math.Round( ( regimeObj1[index1].ResPts - tData.RuleVar[904]));
          if (tData.HistoricalUnitObj[his1].PP < 0)
          {
            RegimeClass[] regimeObj2 = tData.RegimeObj;
            RegimeClass[] regimeClassArray2 = regimeObj2;
            let mut index3: i32 = regnr;
            let mut index4: i32 = index3;
            regimeClassArray2[index4].ResPts = regimeObj2[index3].ResPts - Math.Abs(tData.HistoricalUnitObj[his1].PP);
          }
          if (his2 > -1 && tData.HistoricalUnitObj[his2].PP > 0)
          {
            RegimeClass[] regimeObj3 = tData.RegimeObj;
            RegimeClass[] regimeClassArray3 = regimeObj3;
            let mut index5: i32 = regnr;
            let mut index6: i32 = index5;
            regimeClassArray3[index6].ResPts = regimeObj3[index5].ResPts - Math.Abs(tData.HistoricalUnitObj[his2].PP);
          }
        }
        if ( tData.RuleVar[907] > 0.0)
        {
          let mut hisVarCount1: i32 = tData.HistoricalUnitObj[his1].HisVarCount;
          for (let mut index: i32 = 0; index <= hisVarCount1; index += 1)
          {
            if ( tData.HistoricalUnitObj[his1].HisVarType[index] ==  tData.RuleVar[907])
              tData.HistoricalUnitObj[his1].HisVarValue[index] = 0;
          }
          if (his2 > -1)
          {
            let mut hisVarCount2: i32 = tData.HistoricalUnitObj[his2].HisVarCount;
            for (let mut index: i32 = 0; index <= hisVarCount2; index += 1)
            {
              if ( tData.HistoricalUnitObj[his2].HisVarType[index] ==  tData.RuleVar[907])
                tData.HistoricalUnitObj[his2].HisVarValue[index] = 0;
            }
          }
        }
      }
      if (his1 == -1)
      {
        tData.UnitObj[unr].Historical = his2;
        tData.HistoricalUnitObj[his2].Pool = false;
      }
      else
      {
        commanderName: String = tData.HistoricalUnitObj[his1].CommanderName;
        tData.HistoricalUnitObj[his1].CommanderName = tData.HistoricalUnitObj[his2].CommanderName;
        tData.HistoricalUnitObj[his2].CommanderName = commanderName;
        descript: String = tData.HistoricalUnitObj[his1].Descript;
        tData.HistoricalUnitObj[his1].Descript = tData.HistoricalUnitObj[his2].Descript;
        tData.HistoricalUnitObj[his2].Descript = descript;
        commanderFileName: String = tData.HistoricalUnitObj[his1].CommanderFileName;
        tData.HistoricalUnitObj[his1].CommanderFileName = tData.HistoricalUnitObj[his2].CommanderFileName;
        tData.HistoricalUnitObj[his2].CommanderFileName = commanderFileName;
        let mut commanderSpriteId: i32 = tData.HistoricalUnitObj[his1].CommanderSpriteID;
        tData.HistoricalUnitObj[his1].CommanderSpriteID = tData.HistoricalUnitObj[his2].CommanderSpriteID;
        tData.HistoricalUnitObj[his2].CommanderSpriteID = commanderSpriteId;
        overdrawFileName: String = tData.HistoricalUnitObj[his1].OverdrawFileName;
        tData.HistoricalUnitObj[his1].OverdrawFileName = tData.HistoricalUnitObj[his2].OverdrawFileName;
        tData.HistoricalUnitObj[his2].OverdrawFileName = overdrawFileName;
        let mut overdrawSpriteId: i32 = tData.HistoricalUnitObj[his1].OverdrawSpriteID;
        tData.HistoricalUnitObj[his1].OverdrawSpriteID = tData.HistoricalUnitObj[his2].OverdrawSpriteID;
        tData.HistoricalUnitObj[his2].OverdrawSpriteID = overdrawSpriteId;
        let mut staffSize: i32 = tData.HistoricalUnitObj[his1].StaffSize;
        tData.HistoricalUnitObj[his1].StaffSize = tData.HistoricalUnitObj[his2].StaffSize;
        tData.HistoricalUnitObj[his2].StaffSize = staffSize;
        let mut combatMod: i32 = tData.HistoricalUnitObj[his1].CombatMod;
        tData.HistoricalUnitObj[his1].CombatMod = tData.HistoricalUnitObj[his2].CombatMod;
        tData.HistoricalUnitObj[his2].CombatMod = combatMod;
        let mut moraleMod: i32 = tData.HistoricalUnitObj[his1].MoraleMod;
        tData.HistoricalUnitObj[his1].MoraleMod = tData.HistoricalUnitObj[his2].MoraleMod;
        tData.HistoricalUnitObj[his2].MoraleMod = moraleMod;
        let mut xp: i32 = tData.HistoricalUnitObj[his1].Xp;
        tData.HistoricalUnitObj[his1].Xp = tData.HistoricalUnitObj[his2].Xp;
        tData.HistoricalUnitObj[his2].Xp = xp;
        let mut tempVar1: i32 = tData.HistoricalUnitObj[his1].TempVar1;
        tData.HistoricalUnitObj[his1].TempVar1 = tData.HistoricalUnitObj[his2].TempVar1;
        tData.HistoricalUnitObj[his2].TempVar1 = tempVar1;
        let mut tempVar2: i32 = tData.HistoricalUnitObj[his1].TempVar2;
        tData.HistoricalUnitObj[his1].TempVar2 = tData.HistoricalUnitObj[his2].TempVar2;
        tData.HistoricalUnitObj[his2].TempVar2 = tempVar2;
        let mut tempVar3: i32 = tData.HistoricalUnitObj[his1].TempVar3;
        tData.HistoricalUnitObj[his1].TempVar3 = tData.HistoricalUnitObj[his2].TempVar3;
        tData.HistoricalUnitObj[his2].TempVar3 = tempVar3;
        let mut pp: i32 = tData.HistoricalUnitObj[his1].PP;
        tData.HistoricalUnitObj[his1].PP = tData.HistoricalUnitObj[his2].PP;
        tData.HistoricalUnitObj[his2].PP = pp;
        let mut people: i32 = tData.HistoricalUnitObj[his1].People;
        tData.HistoricalUnitObj[his1].People = tData.HistoricalUnitObj[his2].People;
        tData.HistoricalUnitObj[his2].People = people;
        let mut tempRegime: i32 = tData.HistoricalUnitObj[his1].TempRegime;
        tData.HistoricalUnitObj[his1].TempRegime = tData.HistoricalUnitObj[his2].TempRegime;
        tData.HistoricalUnitObj[his2].TempRegime = tempRegime;
        object[] objArray1 = new object[1];
        object[] objArray2 = new object[1];
        if (tData.HistoricalUnitObj[his1].AutoEventCounter > -1)
        {
          objArray1 = new object[tData.HistoricalUnitObj[his1].AutoEventCounter + 1];
          objArray2 = new object[tData.HistoricalUnitObj[his1].AutoEventCounter + 1];
        }
        let mut autoEventCounter1: i32 = tData.HistoricalUnitObj[his1].AutoEventCounter;
        for (let mut index: i32 = 0; index <= autoEventCounter1; index += 1)
        {
          objArray1[index] =  tData.HistoricalUnitObj[his1].AutoEvent[index];
          objArray2[index] =  tData.HistoricalUnitObj[his1].AutoChance[index];
        }
        let mut autoEventCounter2: i32 = tData.HistoricalUnitObj[his1].AutoEventCounter;
        tData.HistoricalUnitObj[his1].AutoEventCounter = tData.HistoricalUnitObj[his2].AutoEventCounter;
        tData.HistoricalUnitObj[his2].AutoEventCounter = autoEventCounter2;
        if (tData.HistoricalUnitObj[his2].AutoEventCounter > -1)
        {
          tData.HistoricalUnitObj[his1].AutoEvent = new int[tData.HistoricalUnitObj[his1].AutoEventCounter + 1];
          tData.HistoricalUnitObj[his1].AutoChance = new int[tData.HistoricalUnitObj[his1].AutoEventCounter + 1];
        }
        else
        {
          tData.HistoricalUnitObj[his1].AutoEvent = new int[1];
          tData.HistoricalUnitObj[his1].AutoChance = new int[1];
        }
        let mut autoEventCounter3: i32 = tData.HistoricalUnitObj[his1].AutoEventCounter;
        for (let mut index: i32 = 0; index <= autoEventCounter3; index += 1)
        {
          tData.HistoricalUnitObj[his1].AutoEvent[index] = tData.HistoricalUnitObj[his2].AutoEvent[index];
          tData.HistoricalUnitObj[his1].AutoChance[index] = tData.HistoricalUnitObj[his2].AutoChance[index];
        }
        if (tData.HistoricalUnitObj[his2].AutoEventCounter > -1)
        {
          tData.HistoricalUnitObj[his2].AutoEvent = new int[tData.HistoricalUnitObj[his2].AutoEventCounter + 1];
          tData.HistoricalUnitObj[his2].AutoChance = new int[tData.HistoricalUnitObj[his2].AutoEventCounter + 1];
        }
        else
        {
          tData.HistoricalUnitObj[his2].AutoEvent = new int[1];
          tData.HistoricalUnitObj[his2].AutoChance = new int[1];
        }
        let mut autoEventCounter4: i32 = tData.HistoricalUnitObj[his2].AutoEventCounter;
        for (let mut index: i32 = 0; index <= autoEventCounter4; index += 1)
        {
          tData.HistoricalUnitObj[his2].AutoEvent[index] = Conversions.ToInteger(objArray1[index]);
          tData.HistoricalUnitObj[his2].AutoChance[index] = Conversions.ToInteger(objArray2[index]);
        }
        object[] objArray3 = new object[tData.HistoricalUnitObj[his1].DeckCardCounter + 1];
        object[] objArray4 = new object[tData.HistoricalUnitObj[his1].DeckCardCounter + 1];
        let mut deckCardCounter1: i32 = tData.HistoricalUnitObj[his1].DeckCardCounter;
        for (let mut index: i32 = 0; index <= deckCardCounter1; index += 1)
        {
          objArray3[index] =  tData.HistoricalUnitObj[his1].DeckCard[index];
          objArray4[index] =  tData.HistoricalUnitObj[his1].DeckChance[index];
        }
        let mut deckCardCounter2: i32 = tData.HistoricalUnitObj[his1].DeckCardCounter;
        tData.HistoricalUnitObj[his1].DeckCardCounter = tData.HistoricalUnitObj[his2].DeckCardCounter;
        tData.HistoricalUnitObj[his2].DeckCardCounter = deckCardCounter2;
        tData.HistoricalUnitObj[his1].DeckCard = new int[tData.HistoricalUnitObj[his1].DeckCardCounter + 1];
        tData.HistoricalUnitObj[his1].DeckChance = new int[tData.HistoricalUnitObj[his1].DeckCardCounter + 1];
        let mut deckCardCounter3: i32 = tData.HistoricalUnitObj[his1].DeckCardCounter;
        for (let mut index: i32 = 0; index <= deckCardCounter3; index += 1)
        {
          tData.HistoricalUnitObj[his1].DeckCard[index] = tData.HistoricalUnitObj[his2].DeckCard[index];
          tData.HistoricalUnitObj[his1].DeckChance[index] = tData.HistoricalUnitObj[his2].DeckChance[index];
        }
        tData.HistoricalUnitObj[his2].DeckCard = new int[tData.HistoricalUnitObj[his2].DeckCardCounter + 1];
        tData.HistoricalUnitObj[his2].DeckChance = new int[tData.HistoricalUnitObj[his2].DeckCardCounter + 1];
        let mut deckCardCounter4: i32 = tData.HistoricalUnitObj[his2].DeckCardCounter;
        for (let mut index: i32 = 0; index <= deckCardCounter4; index += 1)
        {
          tData.HistoricalUnitObj[his2].DeckCard[index] = Conversions.ToInteger(objArray3[index]);
          tData.HistoricalUnitObj[his2].DeckChance[index] = Conversions.ToInteger(objArray4[index]);
        }
        object[] objArray5 = new object[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        object[] objArray6 = new object[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        object[] objArray7 = new object[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        object[] objArray8 = new object[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        let mut hisVarCount3: i32 = tData.HistoricalUnitObj[his1].HisVarCount;
        for (let mut index: i32 = 0; index <= hisVarCount3; index += 1)
        {
          objArray5[index] =  tData.HistoricalUnitObj[his1].HisVarType[index];
          objArray6[index] =  tData.HistoricalUnitObj[his1].HisVarValue[index];
          objArray8[index] =  tData.HistoricalUnitObj[his1].HisVarNato[index];
          objArray7[index] =  tData.HistoricalUnitObj[his1].HisVarSmall[index];
        }
        let mut hisVarCount4: i32 = tData.HistoricalUnitObj[his1].HisVarCount;
        tData.HistoricalUnitObj[his1].HisVarCount = tData.HistoricalUnitObj[his2].HisVarCount;
        tData.HistoricalUnitObj[his2].HisVarCount = hisVarCount4;
        tData.HistoricalUnitObj[his1].HisVarType = new int[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        tData.HistoricalUnitObj[his1].HisVarValue = new int[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        tData.HistoricalUnitObj[his1].HisVarNato = new int[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        tData.HistoricalUnitObj[his1].HisVarSmall = new int[tData.HistoricalUnitObj[his1].HisVarCount + 1];
        let mut hisVarCount5: i32 = tData.HistoricalUnitObj[his1].HisVarCount;
        for (let mut index: i32 = 0; index <= hisVarCount5; index += 1)
        {
          tData.HistoricalUnitObj[his1].HisVarType[index] = tData.HistoricalUnitObj[his2].HisVarType[index];
          tData.HistoricalUnitObj[his1].HisVarValue[index] = tData.HistoricalUnitObj[his2].HisVarValue[index];
          tData.HistoricalUnitObj[his1].HisVarNato[index] = tData.HistoricalUnitObj[his2].HisVarNato[index];
          tData.HistoricalUnitObj[his1].HisVarSmall[index] = tData.HistoricalUnitObj[his2].HisVarSmall[index];
        }
        tData.HistoricalUnitObj[his2].HisVarType = new int[tData.HistoricalUnitObj[his2].HisVarCount + 1];
        tData.HistoricalUnitObj[his2].HisVarValue = new int[tData.HistoricalUnitObj[his2].HisVarCount + 1];
        tData.HistoricalUnitObj[his2].HisVarNato = new int[tData.HistoricalUnitObj[his2].HisVarCount + 1];
        tData.HistoricalUnitObj[his2].HisVarSmall = new int[tData.HistoricalUnitObj[his2].HisVarCount + 1];
        let mut hisVarCount6: i32 = tData.HistoricalUnitObj[his2].HisVarCount;
        for (let mut index: i32 = 0; index <= hisVarCount6; index += 1)
        {
          tData.HistoricalUnitObj[his2].HisVarType[index] = Conversions.ToInteger(objArray5[index]);
          tData.HistoricalUnitObj[his2].HisVarValue[index] = Conversions.ToInteger(objArray6[index]);
          tData.HistoricalUnitObj[his2].HisVarNato[index] = Conversions.ToInteger(objArray8[index]);
          tData.HistoricalUnitObj[his2].HisVarSmall[index] = Conversions.ToInteger(objArray7[index]);
        }
        if ( tData.RuleVar[355] == 0.0)
        {
          tData.HistoricalUnitObj[his1].HandCardCounter = -1;
          tData.HistoricalUnitObj[his1].HandCard = new int[1];
          tData.HistoricalUnitObj[his2].HandCardCounter = -1;
          tData.HistoricalUnitObj[his2].HandCard = new int[1];
        }
        else if ( tData.RuleVar[355] == 1.0)
        {
          object[] objArray9 = new object[tData.HistoricalUnitObj[his1].HandCardCounter + 1];
          let mut handCardCounter1: i32 = tData.HistoricalUnitObj[his1].HandCardCounter;
          for (let mut index: i32 = 0; index <= handCardCounter1; index += 1)
            objArray9[index] =  tData.HistoricalUnitObj[his1].HandCard[index];
          let mut handCardCounter2: i32 = tData.HistoricalUnitObj[his1].HandCardCounter;
          tData.HistoricalUnitObj[his1].HandCardCounter = tData.HistoricalUnitObj[his2].HandCardCounter;
          tData.HistoricalUnitObj[his2].HandCardCounter = handCardCounter2;
          tData.HistoricalUnitObj[his1].HandCard = new int[tData.HistoricalUnitObj[his1].HandCardCounter + 1];
          let mut handCardCounter3: i32 = tData.HistoricalUnitObj[his1].HandCardCounter;
          for (let mut index: i32 = 0; index <= handCardCounter3; index += 1)
            tData.HistoricalUnitObj[his1].HandCard[index] = tData.HistoricalUnitObj[his2].HandCard[index];
          tData.HistoricalUnitObj[his2].HandCard = new int[tData.HistoricalUnitObj[his2].HandCardCounter + 1];
          let mut handCardCounter4: i32 = tData.HistoricalUnitObj[his2].HandCardCounter;
          for (let mut index: i32 = 0; index <= handCardCounter4; index += 1)
            tData.HistoricalUnitObj[his2].HandCard[index] = Conversions.ToInteger(objArray9[index]);
        }
        if (!flag)
          return;
        tData.RemoveHistoricalUnit(his2);
      }
    }

    pub fn SwapOfficer(regnr: i32, his1: i32, his2: i32, unr: i32)
    {
      bool flag;
      if (Information.IsNothing( this.game.Data.HistoricalUnitObj[his1].CommanderName))
        flag = true;
      else if (this.game.Data.HistoricalUnitObj[his1].CommanderName.Length < 1)
        flag = true;
      if (this.game.Data.Round > 0)
      {
        if (!(this.game.Data.RegimeObj[this.game.Data.Turn].AI &  this.game.Data.RuleVar[914] == 1.0))
        {
          RegimeClass[] regimeObj1 = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray1 = regimeObj1;
          let mut index1: i32 = regnr;
          let mut index2: i32 = index1;
          regimeClassArray1[index2].ResPts =  Math.Round( ( regimeObj1[index1].ResPts - this.game.Data.RuleVar[904]));
          if (this.game.Data.HistoricalUnitObj[his1].PP < 0)
          {
            RegimeClass[] regimeObj2 = this.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray2 = regimeObj2;
            let mut index3: i32 = regnr;
            let mut index4: i32 = index3;
            regimeClassArray2[index4].ResPts = regimeObj2[index3].ResPts - Math.Abs(this.game.Data.HistoricalUnitObj[his1].PP);
          }
          if (his2 > -1 && this.game.Data.HistoricalUnitObj[his2].PP > 0)
          {
            RegimeClass[] regimeObj3 = this.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray3 = regimeObj3;
            let mut index5: i32 = regnr;
            let mut index6: i32 = index5;
            regimeClassArray3[index6].ResPts = regimeObj3[index5].ResPts - Math.Abs(this.game.Data.HistoricalUnitObj[his2].PP);
          }
        }
        if ( this.game.Data.RuleVar[907] > 0.0)
        {
          let mut hisVarCount1: i32 = this.game.Data.HistoricalUnitObj[his1].HisVarCount;
          for (let mut index: i32 = 0; index <= hisVarCount1; index += 1)
          {
            if ( this.game.Data.HistoricalUnitObj[his1].HisVarType[index] ==  this.game.Data.RuleVar[907])
              this.game.Data.HistoricalUnitObj[his1].HisVarValue[index] = 0;
          }
          if (his2 > -1)
          {
            let mut hisVarCount2: i32 = this.game.Data.HistoricalUnitObj[his2].HisVarCount;
            for (let mut index: i32 = 0; index <= hisVarCount2; index += 1)
            {
              if ( this.game.Data.HistoricalUnitObj[his2].HisVarType[index] ==  this.game.Data.RuleVar[907])
                this.game.Data.HistoricalUnitObj[his2].HisVarValue[index] = 0;
            }
          }
        }
      }
      if (his1 == -1)
      {
        this.game.Data.UnitObj[unr].Historical = his2;
        this.game.Data.HistoricalUnitObj[his2].Pool = false;
      }
      else
      {
        commanderName: String = this.game.Data.HistoricalUnitObj[his1].CommanderName;
        this.game.Data.HistoricalUnitObj[his1].CommanderName = this.game.Data.HistoricalUnitObj[his2].CommanderName;
        this.game.Data.HistoricalUnitObj[his2].CommanderName = commanderName;
        descript: String = this.game.Data.HistoricalUnitObj[his1].Descript;
        this.game.Data.HistoricalUnitObj[his1].Descript = this.game.Data.HistoricalUnitObj[his2].Descript;
        this.game.Data.HistoricalUnitObj[his2].Descript = descript;
        commanderFileName: String = this.game.Data.HistoricalUnitObj[his1].CommanderFileName;
        this.game.Data.HistoricalUnitObj[his1].CommanderFileName = this.game.Data.HistoricalUnitObj[his2].CommanderFileName;
        this.game.Data.HistoricalUnitObj[his2].CommanderFileName = commanderFileName;
        let mut commanderSpriteId: i32 = this.game.Data.HistoricalUnitObj[his1].CommanderSpriteID;
        this.game.Data.HistoricalUnitObj[his1].CommanderSpriteID = this.game.Data.HistoricalUnitObj[his2].CommanderSpriteID;
        this.game.Data.HistoricalUnitObj[his2].CommanderSpriteID = commanderSpriteId;
        overdrawFileName: String = this.game.Data.HistoricalUnitObj[his1].OverdrawFileName;
        this.game.Data.HistoricalUnitObj[his1].OverdrawFileName = this.game.Data.HistoricalUnitObj[his2].OverdrawFileName;
        this.game.Data.HistoricalUnitObj[his2].OverdrawFileName = overdrawFileName;
        let mut overdrawSpriteId: i32 = this.game.Data.HistoricalUnitObj[his1].OverdrawSpriteID;
        this.game.Data.HistoricalUnitObj[his1].OverdrawSpriteID = this.game.Data.HistoricalUnitObj[his2].OverdrawSpriteID;
        this.game.Data.HistoricalUnitObj[his2].OverdrawSpriteID = overdrawSpriteId;
        let mut staffSize: i32 = this.game.Data.HistoricalUnitObj[his1].StaffSize;
        this.game.Data.HistoricalUnitObj[his1].StaffSize = this.game.Data.HistoricalUnitObj[his2].StaffSize;
        this.game.Data.HistoricalUnitObj[his2].StaffSize = staffSize;
        let mut combatMod: i32 = this.game.Data.HistoricalUnitObj[his1].CombatMod;
        this.game.Data.HistoricalUnitObj[his1].CombatMod = this.game.Data.HistoricalUnitObj[his2].CombatMod;
        this.game.Data.HistoricalUnitObj[his2].CombatMod = combatMod;
        let mut moraleMod: i32 = this.game.Data.HistoricalUnitObj[his1].MoraleMod;
        this.game.Data.HistoricalUnitObj[his1].MoraleMod = this.game.Data.HistoricalUnitObj[his2].MoraleMod;
        this.game.Data.HistoricalUnitObj[his2].MoraleMod = moraleMod;
        let mut xp: i32 = this.game.Data.HistoricalUnitObj[his1].Xp;
        this.game.Data.HistoricalUnitObj[his1].Xp = this.game.Data.HistoricalUnitObj[his2].Xp;
        this.game.Data.HistoricalUnitObj[his2].Xp = xp;
        let mut tempVar1: i32 = this.game.Data.HistoricalUnitObj[his1].TempVar1;
        this.game.Data.HistoricalUnitObj[his1].TempVar1 = this.game.Data.HistoricalUnitObj[his2].TempVar1;
        this.game.Data.HistoricalUnitObj[his2].TempVar1 = tempVar1;
        let mut tempVar2: i32 = this.game.Data.HistoricalUnitObj[his1].TempVar2;
        this.game.Data.HistoricalUnitObj[his1].TempVar2 = this.game.Data.HistoricalUnitObj[his2].TempVar2;
        this.game.Data.HistoricalUnitObj[his2].TempVar2 = tempVar2;
        let mut tempVar3: i32 = this.game.Data.HistoricalUnitObj[his1].TempVar3;
        this.game.Data.HistoricalUnitObj[his1].TempVar3 = this.game.Data.HistoricalUnitObj[his2].TempVar3;
        this.game.Data.HistoricalUnitObj[his2].TempVar3 = tempVar3;
        let mut pp: i32 = this.game.Data.HistoricalUnitObj[his1].PP;
        this.game.Data.HistoricalUnitObj[his1].PP = this.game.Data.HistoricalUnitObj[his2].PP;
        this.game.Data.HistoricalUnitObj[his2].PP = pp;
        let mut people: i32 = this.game.Data.HistoricalUnitObj[his1].People;
        this.game.Data.HistoricalUnitObj[his1].People = this.game.Data.HistoricalUnitObj[his2].People;
        this.game.Data.HistoricalUnitObj[his2].People = people;
        let mut tempRegime: i32 = this.game.Data.HistoricalUnitObj[his1].TempRegime;
        this.game.Data.HistoricalUnitObj[his1].TempRegime = this.game.Data.HistoricalUnitObj[his2].TempRegime;
        this.game.Data.HistoricalUnitObj[his2].TempRegime = tempRegime;
        object[] objArray1 = new object[1];
        object[] objArray2 = new object[1];
        if (this.game.Data.HistoricalUnitObj[his1].AutoEventCounter > -1)
        {
          objArray1 = new object[this.game.Data.HistoricalUnitObj[his1].AutoEventCounter + 1];
          objArray2 = new object[this.game.Data.HistoricalUnitObj[his1].AutoEventCounter + 1];
        }
        let mut autoEventCounter1: i32 = this.game.Data.HistoricalUnitObj[his1].AutoEventCounter;
        for (let mut index: i32 = 0; index <= autoEventCounter1; index += 1)
        {
          objArray1[index] =  this.game.Data.HistoricalUnitObj[his1].AutoEvent[index];
          objArray2[index] =  this.game.Data.HistoricalUnitObj[his1].AutoChance[index];
        }
        let mut autoEventCounter2: i32 = this.game.Data.HistoricalUnitObj[his1].AutoEventCounter;
        this.game.Data.HistoricalUnitObj[his1].AutoEventCounter = this.game.Data.HistoricalUnitObj[his2].AutoEventCounter;
        this.game.Data.HistoricalUnitObj[his2].AutoEventCounter = autoEventCounter2;
        if (this.game.Data.HistoricalUnitObj[his2].AutoEventCounter > -1)
        {
          this.game.Data.HistoricalUnitObj[his1].AutoEvent = new int[this.game.Data.HistoricalUnitObj[his1].AutoEventCounter + 1];
          this.game.Data.HistoricalUnitObj[his1].AutoChance = new int[this.game.Data.HistoricalUnitObj[his1].AutoEventCounter + 1];
        }
        else
        {
          this.game.Data.HistoricalUnitObj[his1].AutoEvent = new int[1];
          this.game.Data.HistoricalUnitObj[his1].AutoChance = new int[1];
        }
        let mut autoEventCounter3: i32 = this.game.Data.HistoricalUnitObj[his1].AutoEventCounter;
        for (let mut index: i32 = 0; index <= autoEventCounter3; index += 1)
        {
          this.game.Data.HistoricalUnitObj[his1].AutoEvent[index] = this.game.Data.HistoricalUnitObj[his2].AutoEvent[index];
          this.game.Data.HistoricalUnitObj[his1].AutoChance[index] = this.game.Data.HistoricalUnitObj[his2].AutoChance[index];
        }
        if (this.game.Data.HistoricalUnitObj[his2].AutoEventCounter > -1)
        {
          this.game.Data.HistoricalUnitObj[his2].AutoEvent = new int[this.game.Data.HistoricalUnitObj[his2].AutoEventCounter + 1];
          this.game.Data.HistoricalUnitObj[his2].AutoChance = new int[this.game.Data.HistoricalUnitObj[his2].AutoEventCounter + 1];
        }
        else
        {
          this.game.Data.HistoricalUnitObj[his2].AutoEvent = new int[1];
          this.game.Data.HistoricalUnitObj[his2].AutoChance = new int[1];
        }
        let mut autoEventCounter4: i32 = this.game.Data.HistoricalUnitObj[his2].AutoEventCounter;
        for (let mut index: i32 = 0; index <= autoEventCounter4; index += 1)
        {
          this.game.Data.HistoricalUnitObj[his2].AutoEvent[index] = Conversions.ToInteger(objArray1[index]);
          this.game.Data.HistoricalUnitObj[his2].AutoChance[index] = Conversions.ToInteger(objArray2[index]);
        }
        object[] objArray3 = new object[this.game.Data.HistoricalUnitObj[his1].DeckCardCounter + 1];
        object[] objArray4 = new object[this.game.Data.HistoricalUnitObj[his1].DeckCardCounter + 1];
        let mut deckCardCounter1: i32 = this.game.Data.HistoricalUnitObj[his1].DeckCardCounter;
        for (let mut index: i32 = 0; index <= deckCardCounter1; index += 1)
        {
          objArray3[index] =  this.game.Data.HistoricalUnitObj[his1].DeckCard[index];
          objArray4[index] =  this.game.Data.HistoricalUnitObj[his1].DeckChance[index];
        }
        let mut deckCardCounter2: i32 = this.game.Data.HistoricalUnitObj[his1].DeckCardCounter;
        this.game.Data.HistoricalUnitObj[his1].DeckCardCounter = this.game.Data.HistoricalUnitObj[his2].DeckCardCounter;
        this.game.Data.HistoricalUnitObj[his2].DeckCardCounter = deckCardCounter2;
        this.game.Data.HistoricalUnitObj[his1].DeckCard = new int[this.game.Data.HistoricalUnitObj[his1].DeckCardCounter + 1];
        this.game.Data.HistoricalUnitObj[his1].DeckChance = new int[this.game.Data.HistoricalUnitObj[his1].DeckCardCounter + 1];
        let mut deckCardCounter3: i32 = this.game.Data.HistoricalUnitObj[his1].DeckCardCounter;
        for (let mut index: i32 = 0; index <= deckCardCounter3; index += 1)
        {
          this.game.Data.HistoricalUnitObj[his1].DeckCard[index] = this.game.Data.HistoricalUnitObj[his2].DeckCard[index];
          this.game.Data.HistoricalUnitObj[his1].DeckChance[index] = this.game.Data.HistoricalUnitObj[his2].DeckChance[index];
        }
        this.game.Data.HistoricalUnitObj[his2].DeckCard = new int[this.game.Data.HistoricalUnitObj[his2].DeckCardCounter + 1];
        this.game.Data.HistoricalUnitObj[his2].DeckChance = new int[this.game.Data.HistoricalUnitObj[his2].DeckCardCounter + 1];
        let mut deckCardCounter4: i32 = this.game.Data.HistoricalUnitObj[his2].DeckCardCounter;
        for (let mut index: i32 = 0; index <= deckCardCounter4; index += 1)
        {
          this.game.Data.HistoricalUnitObj[his2].DeckCard[index] = Conversions.ToInteger(objArray3[index]);
          this.game.Data.HistoricalUnitObj[his2].DeckChance[index] = Conversions.ToInteger(objArray4[index]);
        }
        object[] objArray5 = new object[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        object[] objArray6 = new object[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        object[] objArray7 = new object[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        object[] objArray8 = new object[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        let mut hisVarCount3: i32 = this.game.Data.HistoricalUnitObj[his1].HisVarCount;
        for (let mut index: i32 = 0; index <= hisVarCount3; index += 1)
        {
          objArray5[index] =  this.game.Data.HistoricalUnitObj[his1].HisVarType[index];
          objArray6[index] =  this.game.Data.HistoricalUnitObj[his1].HisVarValue[index];
          objArray8[index] =  this.game.Data.HistoricalUnitObj[his1].HisVarNato[index];
          objArray7[index] =  this.game.Data.HistoricalUnitObj[his1].HisVarSmall[index];
        }
        let mut hisVarCount4: i32 = this.game.Data.HistoricalUnitObj[his1].HisVarCount;
        this.game.Data.HistoricalUnitObj[his1].HisVarCount = this.game.Data.HistoricalUnitObj[his2].HisVarCount;
        this.game.Data.HistoricalUnitObj[his2].HisVarCount = hisVarCount4;
        this.game.Data.HistoricalUnitObj[his1].HisVarType = new int[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        this.game.Data.HistoricalUnitObj[his1].HisVarValue = new int[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        this.game.Data.HistoricalUnitObj[his1].HisVarNato = new int[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        this.game.Data.HistoricalUnitObj[his1].HisVarSmall = new int[this.game.Data.HistoricalUnitObj[his1].HisVarCount + 1];
        let mut hisVarCount5: i32 = this.game.Data.HistoricalUnitObj[his1].HisVarCount;
        for (let mut index: i32 = 0; index <= hisVarCount5; index += 1)
        {
          this.game.Data.HistoricalUnitObj[his1].HisVarType[index] = this.game.Data.HistoricalUnitObj[his2].HisVarType[index];
          this.game.Data.HistoricalUnitObj[his1].HisVarValue[index] = this.game.Data.HistoricalUnitObj[his2].HisVarValue[index];
          this.game.Data.HistoricalUnitObj[his1].HisVarNato[index] = this.game.Data.HistoricalUnitObj[his2].HisVarNato[index];
          this.game.Data.HistoricalUnitObj[his1].HisVarSmall[index] = this.game.Data.HistoricalUnitObj[his2].HisVarSmall[index];
        }
        this.game.Data.HistoricalUnitObj[his2].HisVarType = new int[this.game.Data.HistoricalUnitObj[his2].HisVarCount + 1];
        this.game.Data.HistoricalUnitObj[his2].HisVarValue = new int[this.game.Data.HistoricalUnitObj[his2].HisVarCount + 1];
        this.game.Data.HistoricalUnitObj[his2].HisVarNato = new int[this.game.Data.HistoricalUnitObj[his2].HisVarCount + 1];
        this.game.Data.HistoricalUnitObj[his2].HisVarSmall = new int[this.game.Data.HistoricalUnitObj[his2].HisVarCount + 1];
        let mut hisVarCount6: i32 = this.game.Data.HistoricalUnitObj[his2].HisVarCount;
        for (let mut index: i32 = 0; index <= hisVarCount6; index += 1)
        {
          this.game.Data.HistoricalUnitObj[his2].HisVarType[index] = Conversions.ToInteger(objArray5[index]);
          this.game.Data.HistoricalUnitObj[his2].HisVarValue[index] = Conversions.ToInteger(objArray6[index]);
          this.game.Data.HistoricalUnitObj[his2].HisVarNato[index] = Conversions.ToInteger(objArray8[index]);
          this.game.Data.HistoricalUnitObj[his2].HisVarSmall[index] = Conversions.ToInteger(objArray7[index]);
        }
        if ( this.game.Data.RuleVar[355] == 0.0)
        {
          this.game.Data.HistoricalUnitObj[his1].HandCardCounter = -1;
          this.game.Data.HistoricalUnitObj[his1].HandCard = new int[1];
          this.game.Data.HistoricalUnitObj[his2].HandCardCounter = -1;
          this.game.Data.HistoricalUnitObj[his2].HandCard = new int[1];
        }
        else if ( this.game.Data.RuleVar[355] == 1.0)
        {
          object[] objArray9 = new object[this.game.Data.HistoricalUnitObj[his1].HandCardCounter + 1];
          let mut handCardCounter1: i32 = this.game.Data.HistoricalUnitObj[his1].HandCardCounter;
          for (let mut index: i32 = 0; index <= handCardCounter1; index += 1)
            objArray9[index] =  this.game.Data.HistoricalUnitObj[his1].HandCard[index];
          let mut handCardCounter2: i32 = this.game.Data.HistoricalUnitObj[his1].HandCardCounter;
          this.game.Data.HistoricalUnitObj[his1].HandCardCounter = this.game.Data.HistoricalUnitObj[his2].HandCardCounter;
          this.game.Data.HistoricalUnitObj[his2].HandCardCounter = handCardCounter2;
          this.game.Data.HistoricalUnitObj[his1].HandCard = new int[this.game.Data.HistoricalUnitObj[his1].HandCardCounter + 1];
          let mut handCardCounter3: i32 = this.game.Data.HistoricalUnitObj[his1].HandCardCounter;
          for (let mut index: i32 = 0; index <= handCardCounter3; index += 1)
            this.game.Data.HistoricalUnitObj[his1].HandCard[index] = this.game.Data.HistoricalUnitObj[his2].HandCard[index];
          this.game.Data.HistoricalUnitObj[his2].HandCard = new int[this.game.Data.HistoricalUnitObj[his2].HandCardCounter + 1];
          let mut handCardCounter4: i32 = this.game.Data.HistoricalUnitObj[his2].HandCardCounter;
          for (let mut index: i32 = 0; index <= handCardCounter4; index += 1)
            this.game.Data.HistoricalUnitObj[his2].HandCard[index] = Conversions.ToInteger(objArray9[index]);
        }
        if (!flag)
          return;
        this.game.Data.RemoveHistoricalUnit(his2);
      }
    }

    pub fn RecruitOfficer(regnr: i32, bool MakePay, let mut OverWriteHis: i32 = -1, let mut OverWritePool: i32 = -1)
    {
      if (MakePay & this.game.Data.Round > 0)
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index1: i32 = regnr;
        let mut index2: i32 = index1;
        regimeClassArray[index2].ResPts =  Math.Round( ( regimeObj[index1].ResPts - this.game.Data.RuleVar[345]));
      }
      let mut id: i32 = this.game.Data.RegimeObj[regnr].OfficerPool;
      if (OverWritePool > -1)
        id = OverWritePool;
      let mut stringListById1: i32 = this.game.HandyFunctionsObj.GetStringListByID(id);
      if (stringListById1 == -1)
        return;
      let mut randomFromStringList1: i32 = this.GetRandomFromStringList(stringListById1);
      if (randomFromStringList1 == -1)
        return;
      index3: i32;
      if (OverWriteHis == -1)
      {
        this.game.Data.AddHistoricalUnit();
        index3 = this.game.Data.HistoricalUnitCounter;
        this.game.Data.HistoricalUnitObj[index3].Pool = true;
      }
      else
      {
        index3 = OverWriteHis;
        this.game.Data.HistoricalUnitObj[index3].Pool = false;
      }
      this.game.Data.HistoricalUnitObj[index3].TempRegime = regnr;
      this.game.Data.HistoricalUnitObj[index3].OverdrawFileName = this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 1];
      this.game.Data.HistoricalUnitObj[index3].PP =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 2]));
      this.game.Data.HistoricalUnitObj[index3].StaffSize =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 3]));
      this.game.Data.HistoricalUnitObj[index3].CombatMod =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 4]));
      this.game.Data.HistoricalUnitObj[index3].MoraleMod =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 5]));
      this.game.Data.HistoricalUnitObj[index3].Descript = this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 13];
      this.game.Data.HistoricalUnitObj[index3].TempVar1 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 14]));
      this.game.Data.HistoricalUnitObj[index3].TempVar2 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 15]));
      this.game.Data.HistoricalUnitObj[index3].TempVar3 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 16]));
      let mut stringListById2: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 6])));
      if (stringListById2 > -1)
      {
        let mut randomFromStringList2: i32 = this.GetRandomFromStringList(stringListById2);
        this.game.Data.HistoricalUnitObj[index3].CommanderFileName = this.game.Data.StringListObj[stringListById2].Data[randomFromStringList2, 1];
        if (this.game.Data.StringListObj[stringListById2].Data[randomFromStringList2, 2].Length > 1)
          this.game.Data.HistoricalUnitObj[index3].OverdrawFileName = this.game.Data.StringListObj[stringListById2].Data[randomFromStringList2, 2];
      }
      let mut num1: i32 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 8]));
      for (let mut index4: i32 = 1; index4 <= num1; index4 += 1)
      {
        let mut stringListById3: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 7])));
        if (stringListById3 > -1)
        {
          let mut randomFromStringList3: i32 = this.GetRandomFromStringList(stringListById3);
          HistoricalUnitClass[] historicalUnitObj1 = this.game.Data.HistoricalUnitObj;
          HistoricalUnitClass[] historicalUnitClassArray1 = historicalUnitObj1;
          let mut index5: i32 = index3;
          let mut index6: i32 = index5;
          historicalUnitClassArray1[index6].DeckCardCounter = historicalUnitObj1[index5].DeckCardCounter + 1;
          this.game.Data.HistoricalUnitObj[index3].DeckCard = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[index3].DeckCard, (Array) new int[this.game.Data.HistoricalUnitObj[index3].DeckCardCounter + 1]);
          this.game.Data.HistoricalUnitObj[index3].DeckChance = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[index3].DeckChance, (Array) new int[this.game.Data.HistoricalUnitObj[index3].DeckCardCounter + 1]);
          this.game.Data.HistoricalUnitObj[index3].DeckCard[this.game.Data.HistoricalUnitObj[index3].DeckCardCounter] = Conversions.ToInteger(this.game.Data.StringListObj[stringListById3].Data[randomFromStringList3, 1]);
          this.game.Data.HistoricalUnitObj[index3].DeckChance[this.game.Data.HistoricalUnitObj[index3].DeckCardCounter] =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[randomFromStringList3, 2]));
          if (Strings.InStr(this.game.Data.HistoricalUnitObj[index3].Descript, this.game.Data.StringListObj[stringListById3].Data[randomFromStringList3, 3]) <= 0)
          {
            HistoricalUnitClass[] historicalUnitObj2 = this.game.Data.HistoricalUnitObj;
            HistoricalUnitClass[] historicalUnitClassArray2 = historicalUnitObj2;
            let mut index7: i32 = index3;
            let mut index8: i32 = index7;
            historicalUnitClassArray2[index8].Descript = historicalUnitObj2[index7].Descript + " " + this.game.Data.StringListObj[stringListById3].Data[randomFromStringList3, 3];
          }
        }
      }
      let mut num2: i32 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 10]));
      for (let mut index9: i32 = 1; index9 <= num2; index9 += 1)
      {
        let mut stringListById4: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 9])));
        if (stringListById4 > -1)
        {
          let mut randomFromStringList4: i32 = this.GetRandomFromStringList(stringListById4);
          HistoricalUnitClass[] historicalUnitObj3 = this.game.Data.HistoricalUnitObj;
          HistoricalUnitClass[] historicalUnitClassArray3 = historicalUnitObj3;
          let mut index10: i32 = index3;
          let mut index11: i32 = index10;
          historicalUnitClassArray3[index11].AutoEventCounter = historicalUnitObj3[index10].AutoEventCounter + 1;
          this.game.Data.HistoricalUnitObj[index3].AutoEvent = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[index3].AutoEvent, (Array) new int[this.game.Data.HistoricalUnitObj[index3].AutoEventCounter + 1]);
          this.game.Data.HistoricalUnitObj[index3].AutoChance = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[index3].AutoChance, (Array) new int[this.game.Data.HistoricalUnitObj[index3].AutoEventCounter + 1]);
          this.game.Data.HistoricalUnitObj[index3].AutoEvent[this.game.Data.HistoricalUnitObj[index3].AutoEventCounter] = Conversions.ToInteger(this.game.Data.StringListObj[stringListById4].Data[randomFromStringList4, 1]);
          this.game.Data.HistoricalUnitObj[index3].AutoChance[this.game.Data.HistoricalUnitObj[index3].AutoEventCounter] =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById4].Data[randomFromStringList4, 2]));
          if (Strings.InStr(this.game.Data.HistoricalUnitObj[index3].Descript, this.game.Data.StringListObj[stringListById4].Data[randomFromStringList4, 3]) <= 0)
          {
            HistoricalUnitClass[] historicalUnitObj4 = this.game.Data.HistoricalUnitObj;
            HistoricalUnitClass[] historicalUnitClassArray4 = historicalUnitObj4;
            let mut index12: i32 = index3;
            let mut index13: i32 = index12;
            historicalUnitClassArray4[index13].Descript = historicalUnitObj4[index12].Descript + " " + this.game.Data.StringListObj[stringListById4].Data[randomFromStringList4, 3];
          }
        }
      }
      let mut stringListById5: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 11])));
      if (stringListById5 > -1)
      {
        let mut randomFromStringList5: i32 = this.GetRandomFromStringList(stringListById5);
        this.game.Data.HistoricalUnitObj[index3].CommanderName = this.game.Data.StringListObj[stringListById5].Data[randomFromStringList5, 1];
      }
      let mut stringListById6: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[randomFromStringList1, 12])));
      if (stringListById6 > -1)
      {
        let mut randomFromStringList6: i32 = this.GetRandomFromStringList(stringListById6);
        HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
        HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
        let mut index14: i32 = index3;
        let mut index15: i32 = index14;
        historicalUnitClassArray[index15].CommanderName = historicalUnitObj[index14].CommanderName + " " + this.game.Data.StringListObj[stringListById6].Data[randomFromStringList6, 1];
      }
      this.game.Data.HistoricalUnitObj[index3].LoadSprites();
      if ( this.game.Data.RuleVar[843] <= 0.0)
        return;
      this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
    }

    pub fn InitialAPPenalty(regnr: i32, bool resetAP)
    {
      let mut mapCounter: i32 = this.game.Data.MapCounter;
      for (let mut index1: i32 = 0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
          {
            if (resetAP)
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_APPenalty(regnr, 0);
            if (resetAP)
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_BattleStack(regnr, 0);
            if (resetAP)
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_BattleStackAir(regnr, 0);
            if (resetAP)
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_BattleStackArt(regnr, 0);
            if (resetAP)
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_BattlePenalty(regnr, 0);
            if (this.game.Data.MapObj[index1].HexObj[index2, index3].Regime != regnr)
            {
              if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[index1].HexObj[index2, index3].LandscapeType].IsSea)
              {
                let mut num: i32 = 0;
                if (this.game.Data.MapObj[index1].HexObj[index2, index3].Regime > -1)
                {
                  if (this.game.Data.RegimeObj[regnr].RegimeRel[this.game.Data.MapObj[index1].HexObj[index2, index3].Regime] == 2)
                    num = 1;
                  if (this.game.Data.RegimeObj[regnr].UberRegime > -1 & this.game.Data.MapObj[index1].HexObj[index2, index3].Regime == this.game.Data.RegimeObj[regnr].UberRegime &&  this.game.Data.MapObj[index1].HexObj[index2, index3].get_APPenalty(regnr) >  this.game.Data.RuleVar[4])
                    this.game.Data.MapObj[index1].HexObj[index2, index3].set_APPenalty(regnr,  Math.Round( this.game.Data.RuleVar[4]));
                }
                if (num == 0)
                  this.game.Data.MapObj[index1].HexObj[index2, index3].set_APPenalty(regnr,  Math.Round( this.game.Data.RuleVar[4]));
              }
            }
            else if ( this.game.Data.MapObj[index1].HexObj[index2, index3].get_APPenalty(regnr) >  this.game.Data.RuleVar[4])
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_APPenalty(regnr,  Math.Round( this.game.Data.RuleVar[4]));
          }
        }
      }
    }

    pub fn PlayCard(regnr: i32, cardinhandnr: i32, bool dontDelete = false)
    {
      object obj1 =  this.game.Data.RegimeObj[regnr].ActionCard[cardinhandnr];
      if (this.game.Data.Product <= 6)
        this.game.Data.RegimeObj[regnr].RemoveActionCard(cardinhandnr);
      if (this.game.Data.Product == 7)
      {
        if (this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].customCostType == 1 & this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].customCostQty > 0)
        {
          let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 210, 0, 0));
          let mut setValue: i32 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].GetData2(0, this.game.Data.RegimeObj[this.game.Data.Turn].id, 1, "fp", 2))) - this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].customCostQty;
          this.game.Data.StringListObj[stringListById].SetData2(0, this.game.Data.RegimeObj[this.game.Data.Turn].id, 1, "fp", 2, setValue, true);
        }
        else
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut index1: i32 = regnr;
          let mut index2: i32 = index1;
          regimeClassArray[index2].ResPts = regimeObj[index1].ResPts - this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].PPCost;
        }
      }
      else
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index3: i32 = regnr;
        let mut index4: i32 = index3;
        regimeClassArray[index4].ResPts = regimeObj[index3].ResPts - this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].PPCost;
      }
      object obj2 = !this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].UnitSelect ?  -1 :  this.game.EditObj.UnitSelected;
      this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].ExecuteEvent, this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].TempVar0, this.game.Data.ActionCardObj[Conversions.ToInteger(obj1)].TempVar1, -1, Conversions.ToInteger(obj2));
      if (this.game.Data.Product > 6 & !dontDelete)
        this.game.Data.RegimeObj[regnr].RemoveActionCard(cardinhandnr);
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
        this.game.Data.UnitObj[index].LastAP = -1;
    }

    pub fn PlayCardByUnit(unr: i32, cardnr: i32)
    {
      let mut regime: i32 = this.game.Data.UnitObj[unr].Regime;
      let mut historical: i32 = this.game.Data.UnitObj[unr].Historical;
      let mut tv3: i32 = !this.game.Data.ActionCardObj[cardnr].UnitSelect ? -1 : this.game.EditObj.UnitSelected;
      this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.ActionCardObj[cardnr].ExecuteEvent, this.game.Data.ActionCardObj[cardnr].TempVar0, this.game.Data.ActionCardObj[cardnr].TempVar1, unr, tv3);
      let mut handCardCounter: i32 = this.game.Data.HistoricalUnitObj[historical].HandCardCounter;
      for (let mut index1: i32 = 0; index1 <= handCardCounter; index1 += 1)
      {
        if (this.game.Data.HistoricalUnitObj[historical].HandCard[index1] == cardnr)
        {
          let mut num1: i32 = index1;
          let mut num2: i32 = this.game.Data.HistoricalUnitObj[historical].HandCardCounter - 1;
          for (let mut index2: i32 = num1; index2 <= num2; index2 += 1)
            this.game.Data.HistoricalUnitObj[historical].HandCard[index2] = this.game.Data.HistoricalUnitObj[historical].HandCard[index2 + 1];
          HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
          HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
          let mut index3: i32 = historical;
          let mut index4: i32 = index3;
          historicalUnitClassArray[index4].HandCardCounter = historicalUnitObj[index3].HandCardCounter - 1;
          if (this.game.Data.HistoricalUnitObj[historical].HandCardCounter > -1)
          {
            this.game.Data.HistoricalUnitObj[historical].HandCard = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[historical].HandCard, (Array) new int[this.game.Data.HistoricalUnitObj[historical].HandCardCounter + 1]);
            break;
          }
          break;
        }
      }
      RegimeClass[] regimeObj = this.game.Data.RegimeObj;
      RegimeClass[] regimeClassArray = regimeObj;
      let mut index5: i32 = regime;
      let mut index6: i32 = index5;
      regimeClassArray[index6].ResPts = regimeObj[index5].ResPts - this.game.Data.ActionCardObj[cardnr].PPCost;
      if (this.game.Data.ActionCardObj[cardnr].HisVarCostType > -1)
      {
        let mut hisVarCount: i32 = this.game.Data.HistoricalUnitObj[historical].HisVarCount;
        for (let mut index7: i32 = 0; index7 <= hisVarCount; index7 += 1)
        {
          if (this.game.Data.HistoricalUnitObj[historical].HisVarType[index7] == this.game.Data.ActionCardObj[cardnr].HisVarCostType)
          {
            int[] hisVarValue = this.game.Data.HistoricalUnitObj[historical].HisVarValue;
            int[] numArray = hisVarValue;
            let mut index8: i32 = index7;
            let mut index9: i32 = index8;
            let mut num: i32 = hisVarValue[index8] - this.game.Data.ActionCardObj[cardnr].HisVarCostQty;
            numArray[index9] = num;
          }
        }
      }
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index10: i32 = 0; index10 <= unitCounter; index10 += 1)
        this.game.Data.UnitObj[index10].LastAP = -1;
    }

    pub fn PlayCard(cardnr: i32)
    {
      let mut tv3: i32 = !this.game.Data.ActionCardObj[cardnr].UnitSelect ? -1 : this.game.EditObj.UnitSelected;
      this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.ActionCardObj[cardnr].ExecuteEvent, this.game.Data.ActionCardObj[cardnr].TempVar0, this.game.Data.ActionCardObj[cardnr].TempVar1, -1, tv3);
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
        this.game.Data.UnitObj[index].LastAP = -1;
    }

    pub fn PlayCardPreEvent(cardnr: i32)
    {
      let mut tv2: i32 = !this.game.Data.ActionCardObj[cardnr].UnitSelect ? this.game.EditObj.UnitSelected : this.game.EditObj.UnitSelected;
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
        this.game.Data.UnitObj[index].TempUnitSelectable = false;
      this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.ActionCardObj[cardnr].PreExecuteEvent, this.game.Data.ActionCardObj[cardnr].TempVar0, this.game.Data.ActionCardObj[cardnr].TempVar1, tv2, -1);
    }

    pub fn PlayCardPreEvent(regnr: i32, cardinhandnr: i32)
    {
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
        this.game.Data.UnitObj[index].TempUnitSelectable = false;
      let mut index1: i32 = this.game.Data.RegimeObj[regnr].ActionCard[cardinhandnr];
      if (this.game.Data.ActionCardObj[index1].PreExecuteEvent <= -1)
        return;
      this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.ActionCardObj[index1].PreExecuteEvent, this.game.Data.ActionCardObj[index1].TempVar0, this.game.Data.ActionCardObj[index1].TempVar1);
    }

    pub void AddNewUnitBasedOnHistorical(
      x: i32,
      y: i32,
      map: i32,
      reg: i32,
      his: i32,
      let mut subpart: i32 = -1,
      let mut OverWriteUnr: i32 = -1,
      bool freePPnoUnit = false,
      bool DontCreateUnits = false,
      bool populateUnit = false)
    {
      if (subpart == -1)
      {
        if (!(this.game.Data.RegimeObj[reg].AI &  this.game.Data.RuleVar[914] == 1.0) && !freePPnoUnit)
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut turn: i32 = this.game.Data.Turn;
          let mut index: i32 = turn;
          regimeClassArray[index].ResPts = regimeObj[turn].ResPts - this.game.Data.HistoricalUnitObj[his].PP;
        }
      }
      else
      {
        let mut num: i32 = 0;
        let mut index1: i32 = 0;
        do
        {
          if (this.game.Data.HistoricalUnitObj[his].SubParts[index1] > -1)
            num += 1;
          index1 += 1;
        }
        while (index1 <= 9);
        if (!(this.game.Data.RegimeObj[this.game.Data.Turn].AI &  this.game.Data.RuleVar[914] == 1.0) && !freePPnoUnit)
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut turn: i32 = this.game.Data.Turn;
          let mut index2: i32 = turn;
          regimeClassArray[index2].ResPts = regimeObj[turn].ResPts -  Math.Round( this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[his].ModelMaster].PP /  num);
        }
      }
      let mut num1: i32 = 9999;
      let mut num2: i32 = -1;
      let mut index3: i32 = his;
      let mut unitCounter1: i32 = this.game.Data.UnitCounter;
      for (let mut index4: i32 = 0; index4 <= unitCounter1; index4 += 1)
      {
        if (this.game.Data.UnitObj[index4].Regime == this.game.Data.HistoricalUnitObj[index3].TempRegime && this.game.Data.UnitObj[index4].PreDef == -1 && this.game.Data.UnitObj[index4].IsHQ && this.game.Data.UnitObj[index4].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index4].Historical].Type > this.game.Data.HistoricalUnitObj[index3].Type)
        {
          let mut num3: i32 = this.game.HandyFunctionsObj.Distance(this.game.Data.UnitObj[index4].X, this.game.Data.UnitObj[index4].Y, this.game.Data.UnitObj[index4].Map, x, y, this.game.EditObj.EventMap);
          if (num3 < num1)
          {
            num1 = num3;
            num2 = index4;
          }
        }
      }
      if (subpart == -1)
      {
        this.game.Data.AddHistoricalUnit();
        index3 = this.game.Data.HistoricalUnitCounter;
        let mut index5: i32 = his;
        if (this.game.Data.HistoricalUnitObj[his].UseModelCounter > -1)
          index5 = this.game.Data.HistoricalUnitObj[his].UseModelCounter;
        let mut num4: i32 = 0;
        if (this.game.Data.HistoricalUnitObj[index5].PercentOldName >  Math.Round( (VBMath.Rnd() * 100f)) & this.game.Data.HistoricalUnitObj[index5].NameCounter > 0)
          num4 = 1;
        nameCounter1: i32;
        if (num4 == 1)
        {
          SimpleList simpleList1 = SimpleList::new();
          let mut historicalUnitCounter: i32 = this.game.Data.HistoricalUnitCounter;
          for (let mut index6: i32 = 0; index6 <= historicalUnitCounter; index6 += 1)
          {
            if (this.game.Data.HistoricalUnitObj[index6].TempRegime == this.game.Data.Turn && this.game.Data.HistoricalUnitObj[index6].ModelMaster > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[index6].ModelMaster].UseModelCounter == index5 | this.game.Data.HistoricalUnitObj[index6].ModelMaster == index5)
            {
              simpleList1.Add(this.game.Data.HistoricalUnitObj[index6].NameCounter, 1);
              simpleList1.Add( Math.Round(Conversion.Val(this.game.Data.HistoricalUnitObj[index6].CounterString)), 1);
            }
          }
          SimpleList simpleList2 = SimpleList::new();
          let mut nameCounter2: i32 = this.game.Data.HistoricalUnitObj[index5].NameCounter;
          for (let mut tid: i32 = 1; tid <= nameCounter2; tid += 1)
          {
            if (simpleList1.FindNr(tid) == -1)
              simpleList2.Add(tid, 1);
          }
          if (simpleList2.Counter > -1)
          {
            nameCounter1 = simpleList2.Id[ Math.Round( Conversion.Int(VBMath.Rnd() *  (simpleList2.Counter + 1)))];
            num4 = 1;
          }
          else
            num4 = 0;
        }
        if (num4 == 0)
        {
          if (this.game.Data.HistoricalUnitObj[index5].NameCounter > -1)
          {
            HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
            HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
            let mut index7: i32 = index5;
            let mut index8: i32 = index7;
            historicalUnitClassArray[index8].NameCounter = historicalUnitObj[index7].NameCounter + 1;
          }
          nameCounter1 = this.game.Data.HistoricalUnitObj[index5].NameCounter;
        }
        this.game.Data.HistoricalUnitObj[index3].Counter = nameCounter1;
        str1: String;
        str2: String;
        if (nameCounter1 > -1)
        {
          str3: String = Strings.Trim(Conversion.Str( nameCounter1));
          str4: String = !((nameCounter1 + 10) % 10 == 1 & (nameCounter1 + 100) % 100 != 11) ? (!((nameCounter1 + 10) % 10 == 2 & (nameCounter1 + 100) % 100 != 12) ? (!((nameCounter1 + 10) % 10 == 3 & (nameCounter1 + 100) % 100 != 13) ? str3 + "th" : str3 + "rd") : str3 + "nd") : str3 + "st";
          if (this.game.Data.HistoricalUnitObj[his].UseRomans)
          {
            str1 = this.game.HandyFunctionsObj.GetRomanNumerical(nameCounter1);
            str4 = str1;
            if (Strings.Len(this.game.Data.HistoricalUnitObj[his].CounterString) > 0 && Conversion.Val(this.game.Data.HistoricalUnitObj[his].CounterString) != -1.0)
              str1 = str1 + " " + this.game.Data.HistoricalUnitObj[his].CounterString;
          }
          else
          {
            str1 = Strings.Trim(Conversion.Str( nameCounter1));
            if (Strings.Len(this.game.Data.HistoricalUnitObj[his].CounterString) > 0 && Conversion.Val(this.game.Data.HistoricalUnitObj[his].CounterString) != -1.0)
              str1 += this.game.Data.HistoricalUnitObj[his].CounterString;
          }
          str2 = str4 + " " + this.game.Data.HistoricalUnitObj[his].Name;
        }
        else
        {
          str2 = this.game.Data.HistoricalUnitObj[his].Name;
          str1 = this.game.Data.HistoricalUnitObj[his].CounterString;
        }
        this.game.Data.HistoricalUnitObj[index3].Name = str2;
        this.game.Data.HistoricalUnitObj[index3].CounterString = str1;
        this.game.Data.HistoricalUnitObj[index3].SmallGfx = this.game.Data.HistoricalUnitObj[his].SmallGfx;
        this.game.Data.HistoricalUnitObj[index3].Counter = this.game.Data.HistoricalUnitObj[his].Counter;
        this.game.Data.HistoricalUnitObj[index3].Green = this.game.Data.HistoricalUnitObj[his].Green;
        this.game.Data.HistoricalUnitObj[index3].Red = this.game.Data.HistoricalUnitObj[his].Red;
        this.game.Data.HistoricalUnitObj[index3].Blue = this.game.Data.HistoricalUnitObj[his].Blue;
        this.game.Data.HistoricalUnitObj[index3].TempVar1 = this.game.Data.HistoricalUnitObj[his].TempVar1;
        this.game.Data.HistoricalUnitObj[index3].TempVar2 = this.game.Data.HistoricalUnitObj[his].TempVar2;
        this.game.Data.HistoricalUnitObj[index3].TempVar3 = this.game.Data.HistoricalUnitObj[his].TempVar3;
        this.game.Data.HistoricalUnitObj[index3].People = this.game.Data.HistoricalUnitObj[his].People;
        this.game.Data.HistoricalUnitObj[index3].UsePeopleGfx = this.game.Data.HistoricalUnitObj[his].UsePeopleGfx;
        this.game.Data.HistoricalUnitObj[index3].Type = this.game.Data.HistoricalUnitObj[his].Type;
        this.game.Data.HistoricalUnitObj[index3].NoSplit = this.game.Data.HistoricalUnitObj[his].NoSplit;
        this.game.Data.HistoricalUnitObj[index3].TempRegime = reg;
        this.game.Data.HistoricalUnitObj[index3].ModelMaster = his;
        this.game.Data.HistoricalUnitObj[index3].NameCounter = this.game.Data.HistoricalUnitObj[his].NameCounter;
        if ( this.game.Data.RuleVar[343] > 0.0 & this.game.Data.HistoricalUnitObj[index3].Type >= 5)
        {
          let mut historicalUnitCounter: i32 = this.game.Data.HistoricalUnitCounter;
          let mut num5: i32 = 0;
          while (num5 <= historicalUnitCounter)
            num5 += 1;
        }
      }
      let mut index9: i32 = 0;
      do
      {
        if (subpart == -1 | subpart == index9 && subpart == -1)
        {
          this.game.Data.HistoricalUnitObj[index3].SubParts[index9] = this.game.Data.HistoricalUnitObj[his].SubParts[index9];
          this.game.Data.HistoricalUnitObj[index3].Designation[index9] = this.game.Data.HistoricalUnitObj[his].Designation[index9];
          this.game.Data.HistoricalUnitObj[index3].DesignationSmall[index9] = this.game.Data.HistoricalUnitObj[his].DesignationSmall[index9];
        }
        index9 += 1;
      }
      while (index9 <= 9);
      SimpleList simpleList3 = SimpleList::new();
      SimpleList simpleList4 = SimpleList::new();
      str5: String;
      Number1: i32;
      if (OverWriteUnr > -1)
      {
        if (this.game.Data.UnitObj[OverWriteUnr].Historical == -1)
        {
          str5 = "Unit is currently a to be disbanded formation.";
          simpleList3.Add(OverWriteUnr, 0);
          Number1 = 1;
        }
        else if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[OverWriteUnr].Historical].ModelMaster == -1)
        {
          str5 = "Unit is currently an ad hoc formation." + " Current unit consists of " + Conversion.Str( Number1) + " subunits.";
          let mut unitCounter2: i32 = this.game.Data.UnitCounter;
          for (let mut tid: i32 = 0; tid <= unitCounter2; tid += 1)
          {
            if (this.game.Data.UnitObj[tid].PreDef == -1 && this.game.Data.UnitObj[tid].Historical == this.game.Data.UnitObj[OverWriteUnr].Historical)
            {
              num6: i32;
              num6 += 1;
              simpleList3.Add(tid, 0);
            }
          }
        }
        else
        {
          str6: String = "Current Model is " + this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[OverWriteUnr].Historical].ModelMaster].Name;
          let mut unitCounter3: i32 = this.game.Data.UnitCounter;
          Number2: i32;
          for (let mut tid: i32 = 0; tid <= unitCounter3; tid += 1)
          {
            if (this.game.Data.UnitObj[tid].PreDef == -1 && this.game.Data.UnitObj[tid].Historical == this.game.Data.UnitObj[OverWriteUnr].Historical)
            {
              Number2 += 1;
              simpleList3.Add(tid, 0);
            }
          }
          str5 = str6 + " Current unit consists of " + Conversion.Str( Number2) + " subunits.";
        }
      }
      if (subpart == -1 & OverWriteUnr > -1 & !DontCreateUnits)
      {
        for (let mut counter: i32 = simpleList3.Counter; counter >= 0; counter += -1)
        {
          let mut index10: i32 = simpleList3.Id[counter];
          let mut historical: i32 = this.game.Data.UnitObj[index10].Historical;
          let mut historicalSubPart: i32 = this.game.Data.UnitObj[index10].HistoricalSubPart;
          let mut num7: i32 = 0;
          let mut tid1: i32 = 0;
          while (!(historical > -1 & historicalSubPart > -1) || simpleList4.FindNr(tid1) != -1 || this.game.Data.HistoricalUnitObj[index3].SubParts[tid1] <= -1 || this.game.Data.HistoricalUnitObj[historical].SubParts[historicalSubPart] != this.game.Data.HistoricalUnitObj[index3].SubParts[tid1])
          {
            tid1 += 1;
            if (tid1 > 9)
              goto label_76;
          }
          this.game.Data.UnitObj[index10].HistoricalSubPart = tid1;
          simpleList4.Add(tid1, 0);
          this.game.Data.UnitObj[index10].Historical = index3;
          simpleList3.Remove(index10);
          num7 = 1;
label_76:
          if (num7 == 0)
          {
            let mut tid2: i32 = 0;
            while (!(historical > -1 & historicalSubPart > -1) || simpleList4.FindNr(tid2) != -1 || this.game.Data.HistoricalUnitObj[index3].Designation[tid2] <= -1 || this.game.Data.HistoricalUnitObj[historical].Designation[historicalSubPart] != this.game.Data.HistoricalUnitObj[index3].Designation[tid2])
            {
              tid2 += 1;
              if (tid2 > 9)
                goto label_81;
            }
            this.game.Data.UnitObj[index10].HistoricalSubPart = tid2;
            simpleList4.Add(tid2, 0);
            this.game.Data.UnitObj[index10].Historical = index3;
            simpleList3.Remove(index10);
            num7 = 1;
          }
label_81:
          if (num7 == 0)
          {
            SimpleList simpleList5 = SimpleList::new();
            let mut tid3: i32 = 0;
            do
            {
              if (simpleList4.FindNr(tid3) == -1 && this.game.Data.HistoricalUnitObj[index3].SubParts[tid3] > -1)
                simpleList5.Add(tid3, this.game.HandyFunctionsObj.PowerPointsDifferent(index10, this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index3].SubParts[tid3])));
              tid3 += 1;
            }
            while (tid3 <= 9);
            simpleList5.Sort();
            if (simpleList5.Counter > -1)
            {
              let mut tid4: i32 = simpleList5.Id[0];
              this.game.Data.UnitObj[index10].HistoricalSubPart = tid4;
              simpleList4.Add(tid4, 0);
              this.game.Data.UnitObj[index10].Historical = index3;
              simpleList3.Remove(index10);
              num7 = 1;
            }
          }
          if (num7 == 1)
          {
            this.game.Data.UnitObj[index10].Name = this.game.Data.HistoricalUnitObj[index3].Name;
            this.game.Data.UnitObj[index10].IsHQ = this.game.Data.UnitObj[this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index3].SubParts[this.game.Data.UnitObj[index10].HistoricalSubPart])].IsHQ;
            this.game.Data.UnitObj[index10].StartPower = this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index3].SubParts[counter]), true);
            this.game.Data.UnitObj[index10].Regime = this.game.Data.HistoricalUnitObj[index3].TempRegime;
            let mut num8: i32 = 0;
            let mut unitCounter4: i32 = this.game.Data.UnitCounter;
            for (let mut index11: i32 = 0; index11 <= unitCounter4; index11 += 1)
            {
              if (this.game.Data.UnitObj[index11].Historical == this.game.Data.UnitObj[index10].Historical)
                num8 += 1;
            }
            this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index10].Historical].StartSize = num8;
          }
        }
      }
      else
      {
        if (DontCreateUnits)
          return;
        bool flag = false;
        let mut index12: i32 = 0;
        do
        {
          if (this.game.Data.HistoricalUnitObj[index3].SubParts[index12] > -1)
            flag = true;
          index12 += 1;
        }
        while (index12 <= 9);
        if (this.game.Data.Product < 6)
          flag = true;
        let mut index13: i32 = 0;
        do
        {
          if (subpart == -1 | subpart == index13 && this.game.Data.HistoricalUnitObj[index3].SubParts[index13] > -1 | index13 == 0 & !flag & this.game.Data.Product >= 6)
          {
            unr: i32;
            if (OverWriteUnr == -1)
            {
              this.game.Data.AddUnit(x, y, map);
              unr = this.game.Data.UnitCounter;
            }
            else
              unr = OverWriteUnr;
            if (populateUnit)
              this.game.HandyFunctionsObj.CopyUnit(unr, this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index3].SubParts[index13]));
            this.game.Data.UnitObj[unr].Name = this.game.Data.HistoricalUnitObj[index3].Name;
            this.game.Data.UnitObj[unr].Historical = index3;
            if (flag)
            {
              this.game.Data.UnitObj[unr].IsHQ = this.game.Data.UnitObj[this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index3].SubParts[index13])].IsHQ;
              this.game.Data.UnitObj[unr].HistoricalSubPart = index13;
            }
            else
            {
              this.game.Data.UnitObj[unr].IsHQ = false;
              this.game.Data.UnitObj[unr].HistoricalSubPart = 0;
            }
            this.game.Data.UnitObj[unr].Regime = this.game.Data.HistoricalUnitObj[index3].TempRegime;
            this.game.Data.UnitObj[unr].StartPower = this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index3].SubParts[index13]), true);
            if (this.game.Data.Product == 6 & this.game.Data.Round >= 1)
            {
              let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
              for (let mut index14: i32 = 0; index14 <= sfCount; index14 += 1)
                this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index14]].Ap = 0;
            }
            if (OverWriteUnr == -1)
              this.game.Data.UnitObj[unr].HQ = num2;
            if (subpart > -1)
            {
              let mut num9: i32 = this.game.Data.UnitCounter - 1;
              for (let mut index15: i32 = 0; index15 <= num9; index15 += 1)
              {
                if (this.game.Data.UnitObj[index15].Historical == index3 & this.game.Data.UnitObj[index15].HQ > -1)
                {
                  this.game.Data.UnitObj[unr].HQ = this.game.Data.UnitObj[index15].HQ;
                  this.game.Data.UnitObj[unr].SODefendPercent = this.game.Data.UnitObj[index15].SODefendPercent;
                  if (OverWriteUnr == -1)
                    this.game.Data.UnitObj[unr].HQ = num2;
                  this.game.Data.UnitObj[unr].SOSupReqPercent = this.game.Data.UnitObj[index15].SOSupReqPercent;
                  this.game.Data.UnitObj[unr].SOReplacementPercent = this.game.Data.UnitObj[index15].SOReplacementPercent;
                  this.game.Data.UnitObj[unr].SOInterceptRdnStop = this.game.Data.UnitObj[index15].SOInterceptRdnStop;
                  break;
                }
              }
            }
          }
          index13 += 1;
        }
        while (index13 <= 9);
        let mut num10: i32 = 0;
        let mut unitCounter5: i32 = this.game.Data.UnitCounter;
        for (let mut index16: i32 = 0; index16 <= unitCounter5; index16 += 1)
        {
          if (this.game.Data.UnitObj[index16].Historical == index3)
            num10 += 1;
        }
        this.game.Data.HistoricalUnitObj[index3].StartSize = num10;
      }
    }

    pub fn AutoConquerNeutral(regnr: i32)
    {
      MapMatrix2[] mapMatrix2Array = new MapMatrix2[this.game.Data.MapCounter + 1];
      let mut mapCounter1: i32 = this.game.Data.MapCounter;
      for (let mut index: i32 = 0; index <= mapCounter1; index += 1)
        mapMatrix2Array[index] = new MapMatrix2(this.game.Data.MapObj[index].MapWidth, this.game.Data.MapObj[index].MapHeight);
      this += 1.game.Data.StepNr;
      let mut mapCounter2: i32 = this.game.Data.MapCounter;
      for (let mut cmap: i32 = 0; cmap <= mapCounter2; cmap += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[cmap].MapWidth;
        for (let mut cx: i32 = 0; cx <= mapWidth; cx += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[cmap].MapHeight;
          for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
          {
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime == regnr)
            {
              let mut tfacing: i32 = 1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing);
                if (coordinate.onmap && this.game.Data.MapObj[cmap].HexObj[coordinate.x, coordinate.y].Regime == -1 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[cmap].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                  mapMatrix2Array[cmap].Value[coordinate.x, coordinate.y] = 1;
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      let mut mapCounter3: i32 = this.game.Data.MapCounter;
      for (let mut map: i32 = 0; map <= mapCounter3; map += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[map].MapWidth;
        for (let mut x: i32 = 0; x <= mapWidth; x += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[map].MapHeight;
          for (let mut y: i32 = 0; y <= mapHeight; y += 1)
          {
            if (mapMatrix2Array[map].Value[x, y] == 1)
            {
              this.game.Data.MapObj[map].HexObj[x, y].Regime = regnr;
              this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, regnr);
            }
          }
        }
      }
    }

    pub fn SetExtraStat(regnr: i32)
    {
      let mut index1: i32 = 0;
      do
      {
        this.game.EditObj.uds_subtab[index1] = -1;
        let mut index2: i32 = 0;
        do
        {
          this.game.EditObj.uds_page[index1, index2] = -1;
          index2 += 1;
        }
        while (index2 <= 18);
        index1 += 1;
      }
      while (index1 <= 8);
      this.game.EditObj.statsTab_tab = -1;
      this.game.EditObj.statsTab_item = -1;
      if ( this.game.Data.RuleVar[650] > 0.0)
        this.game.Data.RegimeObj[regnr].ExtraStat[0, this.game.Data.Round] = this.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( this.game.Data.RuleVar[650])];
      if ( this.game.Data.RuleVar[651] > 0.0)
        this.game.Data.RegimeObj[regnr].ExtraStat[1, this.game.Data.Round] = this.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( this.game.Data.RuleVar[651])];
      if ( this.game.Data.RuleVar[652] <= 0.0)
        return;
      this.game.Data.RegimeObj[regnr].ExtraStat[2, this.game.Data.Round] = this.game.Data.RegimeObj[regnr].RegimeSlot[ Math.Round( this.game.Data.RuleVar[652])];
    }

    pub fn SetInitialReconAndZOC(regnr: i32)
    {
      let mut mapCounter1: i32 = this.game.Data.MapCounter;
      for (let mut index1: i32 = 0; index1 <= mapCounter1; index1 += 1)
      {
        this.game.Data.MapObj[index1].CanSee = false;
        let mut mapWidth: i32 = this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
          {
            if (this.game.Data.ShrowdOn)
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_SeeNow(regnr, 0);
            else
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_SeeNow(regnr, 1);
            this.game.Data.MapObj[index1].HexObj[index2, index3].set_ReconPts(regnr, 0);
            this.game.Data.MapObj[index1].HexObj[index2, index3].set_ZocPts(regnr, 0);
            this.game.Data.MapObj[index1].HexObj[index2, index3].MaxRecon = 0;
            this.game.Data.MapObj[index1].HexObj[index2, index3].MaxLos = 0;
            this.game.Data.MapObj[index1].HexObj[index2, index3].MaxObstruct = 0;
          }
        }
      }
      let mut mapCounter2: i32 = this.game.Data.MapCounter;
      for (let mut map: i32 = 0; map <= mapCounter2; map += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[map].MapWidth;
        for (let mut x: i32 = 0; x <= mapWidth; x += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[map].MapHeight;
          for (let mut y: i32 = 0; y <= mapHeight; y += 1)
            this.game.HandyFunctionsObj.SetHexReconAndZOC(x, y, map, regnr);
        }
      }
      let mut mapCounter3: i32 = this.game.Data.MapCounter;
      for (let mut cmap: i32 = 0; cmap <= mapCounter3; cmap += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[cmap].MapWidth;
        for (let mut cx: i32 = 0; cx <= mapWidth; cx += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[cmap].MapHeight;
          for (let mut cy: i32 = 0; cy <= mapHeight; cy += 1)
          {
            let mut num: i32 = 0;
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].get_LastLT(regnr) > -1)
              this.game.Data.MapObj[cmap].CanSee = true;
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime == regnr)
              num = 1;
            if (this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime > -1 && this.game.Data.RegimeObj[regnr].RegimeRel[this.game.Data.MapObj[cmap].HexObj[cx, cy].Regime] == 2 &  this.game.Data.RuleVar[328] == 1.0)
              num = 1;
            if (num == 1)
            {
              if ( this.game.Data.RuleVar[8] >  this.game.Data.MapObj[cmap].HexObj[cx, cy].get_ReconPts(regnr))
              {
                this.game.Data.MapObj[cmap].HexObj[cx, cy].set_ReconPts(regnr,  Math.Round( this.game.Data.RuleVar[8]));
                this.game.Data.MapObj[cmap].CanSee = true;
              }
              let mut tfacing: i32 = 1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, cmap, tfacing);
                if (coordinate.onmap &&  this.game.Data.RuleVar[8] >  this.game.Data.MapObj[cmap].HexObj[coordinate.x, coordinate.y].get_ReconPts(regnr))
                {
                  this.game.Data.MapObj[cmap].HexObj[coordinate.x, coordinate.y].set_ReconPts(regnr,  Math.Round( this.game.Data.RuleVar[8]));
                  this.game.Data.MapObj[cmap].CanSee = true;
                }
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      let mut mapCounter4: i32 = this.game.Data.MapCounter;
      for (let mut index4: i32 = 0; index4 <= mapCounter4; index4 += 1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[index4].MapWidth;
        for (let mut index5: i32 = 0; index5 <= mapWidth; index5 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[index4].MapHeight;
          for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
          {
            if (this.game.Data.MapObj[index4].HexObj[index5, index6].get_ReconPts(regnr) >= 1)
            {
              this.game.Data.MapObj[index4].HexObj[index5, index6].set_SeeNow(regnr, 1);
              this.game.Data.MapObj[index4].HexObj[index5, index6].set_LastLT(regnr, this.game.Data.MapObj[index4].HexObj[index5, index6].LandscapeType);
              this.game.Data.MapObj[index4].HexObj[index5, index6].set_LastSpr(regnr, this.game.Data.MapObj[index4].HexObj[index5, index6].SpriteNr);
              this.game.Data.MapObj[index4].HexObj[index5, index6].set_LastReg(regnr, this.game.Data.MapObj[index4].HexObj[index5, index6].Regime);
            }
            this.game.Data.MapObj[index4].HexObj[index5, index6].MaxRecon = this.game.Data.MapObj[index4].HexObj[index5, index6].get_ReconPts(regnr);
          }
        }
      }
      this.SpottedAndIdentifiedUpdate(regnr);
    }

    pub fn SpottedAndIdentifiedUpdate(regnr: i32, let mut tunr: i32 = -1, bool improveOnly = false)
    {
      if (!( this.game.Data.RuleVar[419] > 0.0 & this.game.Data.Product >= 6 & this.game.Data.Round > 0))
        return;
      if (tunr == -1)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
          {
            let mut unitCounter: i32 = this.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
            for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
            {
              let mut unit: i32 = this.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3];
              if (this.game.Data.UnitObj[unit].Regime != regnr)
              {
                if (index1 == 59 & index2 == 19)
                  index1 = index1;
                let mut val1: i32 = this.game.Data.MapObj[0].HexObj[index1, index2].get_ReconPts(regnr);
                if (this.game.Data.Turn == regnr)
                  val1 = Math.Max(val1, this.game.Data.MapObj[0].HexObj[index1, index2].MaxRecon);
                Coordinate reconMinusHide;
                if (this.game.Data.Round == 0 | !this.game.Data.FOWOn)
                  reconMinusHide.x = 3;
                else
                  reconMinusHide = this.game.HandyFunctionsObj.GetReconMinusHide(unit, this.game.Data.Turn);
                if (reconMinusHide.y >= 1 & !this.game.Data.UnitObj[unit].Spotted)
                  this.game.Data.UnitObj[unit].Spotted = true;
                else if (this.game.Data.UnitObj[unit].Spotted & (reconMinusHide.y < 1 | val1 < 1) && !improveOnly)
                  this.game.Data.UnitObj[unit].Spotted = false;
                if ( reconMinusHide.y >=  this.game.Data.RuleVar[55] & !this.game.Data.UnitObj[unit].Identified)
                  this.game.Data.UnitObj[unit].Identified = true;
                else if (this.game.Data.UnitObj[unit].Identified & (reconMinusHide.y < 1 | val1 < 1) && !improveOnly)
                  this.game.Data.UnitObj[unit].Identified = false;
              }
            }
          }
        }
      }
      else
      {
        if (tunr <= -1)
          return;
        let mut unr: i32 = tunr;
        let mut x: i32 = this.game.Data.UnitObj[unr].X;
        let mut y: i32 = this.game.Data.UnitObj[unr].Y;
        if (this.game.Data.UnitObj[unr].Regime == regnr)
          return;
        let mut val1: i32 = this.game.Data.MapObj[0].HexObj[x, y].get_ReconPts(regnr);
        if (this.game.Data.Turn == regnr)
          val1 = Math.Max(val1, this.game.Data.MapObj[0].HexObj[x, y].MaxRecon);
        Coordinate reconMinusHide;
        if (this.game.Data.Round == 0 | !this.game.Data.FOWOn)
          reconMinusHide.x = 3;
        else
          reconMinusHide = this.game.HandyFunctionsObj.GetReconMinusHide(unr, this.game.Data.Turn);
        if (reconMinusHide.y >= 1 & !this.game.Data.UnitObj[unr].Spotted)
          this.game.Data.UnitObj[unr].Spotted = true;
        else if (this.game.Data.UnitObj[unr].Spotted & (reconMinusHide.y < 1 | val1 < 1) && !improveOnly)
          this.game.Data.UnitObj[unr].Spotted = false;
        if ( reconMinusHide.y >=  this.game.Data.RuleVar[55] & !this.game.Data.UnitObj[unr].Identified)
        {
          this.game.Data.UnitObj[unr].Identified = true;
        }
        else
        {
          if (!(this.game.Data.UnitObj[unr].Identified & (reconMinusHide.y < 1 | val1 < 1)) || improveOnly)
            return;
          this.game.Data.UnitObj[unr].Identified = false;
        }
      }
    }

    pub fn IntialZOCConquestCheck(regnr: i32)
    {
      let mut num: i32 = 0;
      do
      {
        let mut mapCounter: i32 = this.game.Data.MapCounter;
        for (let mut map: i32 = 0; map <= mapCounter; map += 1)
        {
          let mut mapWidth: i32 = this.game.Data.MapObj[map].MapWidth;
          for (let mut x: i32 = 0; x <= mapWidth; x += 1)
          {
            let mut mapHeight: i32 = this.game.Data.MapObj[map].MapHeight;
            for (let mut y: i32 = 0; y <= mapHeight; y += 1)
              this.game.HandyFunctionsObj.DoZOCConquest(x, y, map, regnr);
          }
        }
        num += 1;
      }
      while (num <= 3);
    }

    pub fn DoEntrench(regnr: i32, bool onlyauto = false)
    {
      if (this.game.Data.UnitCounter < 0)
        return;
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index1: i32 = 0; index1 <= unitCounter; index1 += 1)
      {
        if (this.game.Data.UnitObj[index1].Regime == regnr & this.game.Data.UnitObj[index1].X > -1 & this.game.Data.UnitObj[index1].PreDef <= -1 && this.game.Data.UnitObj[index1].SFCount > -1)
        {
          let mut sfCount: i32 = this.game.Data.UnitObj[index1].SFCount;
          for (let mut index2: i32 = 0; index2 <= sfCount; index2 += 1)
          {
            let mut sf: i32 = this.game.Data.UnitObj[index1].SFList[index2];
            let mut ap: i32 = this.game.Data.SFObj[sf].Ap;
            let mut type: i32 = this.game.Data.SFObj[sf].Type;
            let mut unitGroup: i32 = this.game.Data.SFTypeObj[type].UnitGroup;
            if (this.game.Data.SFTypeObj[type].Theater == 0)
            {
              let mut landscapeType: i32 = this.game.Data.MapObj[this.game.Data.UnitObj[index1].Map].HexObj[this.game.Data.UnitObj[index1].X, this.game.Data.UnitObj[index1].Y].LandscapeType;
              let mut location: i32 = this.game.Data.MapObj[this.game.Data.UnitObj[index1].Map].HexObj[this.game.Data.UnitObj[index1].X, this.game.Data.UnitObj[index1].Y].Location;
              if (!this.game.Data.LandscapeTypeObj[landscapeType].IsSea)
              {
                let mut num1: i32 = Conversion.Int(this.game.Data.SFTypeObj[type].EntrenchPower);
                if (this.game.Data.Product == 6 & this.game.Data.SFTypeObj[type].EP <= 0)
                {
                  Coordinate coordinate = this.game.HandyFunctionsObj.EPandPowerInHex(this.game.Data.UnitObj[index1].X, this.game.Data.UnitObj[index1].Y);
                  if (coordinate.x > 0 & coordinate.y > 0)
                  {
                    float num2 =  coordinate.x /  coordinate.y;
                    let mut num3: i32 = coordinate.data1 - num1;
                    if (num3 > 0)
                    {
                      if ( num2 < 1.0)
                        num3 =  Math.Round( ( num3 * num2));
                      num1 += num3;
                    }
                  }
                }
                if ( this.game.Data.RuleVar[494] > 0.0)
                {
                  data: DataClass = this.game.Data;
                  str: String = "Snow";
                   local: String =  str;
                  let mut libVar: i32 = data.FindLibVar( local, "");
                  if (this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index1].X, this.game.Data.UnitObj[index1].Y].GetHexLibVarValue(libVar) > 0)
                    num1 =  Math.Round( num1 *  this.game.Data.RuleVar[494] / 100.0);
                }
                if (onlyauto)
                  num1 = 0;
                SFClass[] sfObj = this.game.Data.SFObj;
                SFClass[] sfClassArray = sfObj;
                let mut index3: i32 = sf;
                let mut index4: i32 = index3;
                sfClassArray[index4].CurrentEntrench = sfObj[index3].CurrentEntrench + num1;
                let mut num4: i32 =  Math.Round( this.game.Data.LandscapeTypeObj[landscapeType].DefBonus[unitGroup]);
                if (location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].PictureLT > -1)
                  num4 =  Math.Round( ( num4 + this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].PictureLT].DefBonus[unitGroup]));
                if (num4 > this.game.Data.SFObj[sf].CurrentEntrench)
                  this.game.Data.SFObj[sf].CurrentEntrench = num4;
                let mut num5: i32 =  Math.Round( this.game.Data.LandscapeTypeObj[landscapeType].DefBonusMax[unitGroup]);
                if (location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].PictureLT > -1)
                  num5 =  Math.Round( ( num5 + this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[location].Type].PictureLT].DefBonusMax[unitGroup]));
                if (num5 < this.game.Data.SFObj[sf].CurrentEntrench)
                  this.game.Data.SFObj[sf].CurrentEntrench = num5;
                if (9999 < this.game.Data.SFObj[sf].CurrentEntrench)
                  this.game.Data.SFObj[sf].CurrentEntrench = 9999;
              }
            }
            if (this.game.Data.Product >= 6)
              this.game.Data.SFObj[sf].initialEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
          }
        }
      }
    }

    pub fn DoMorale(regnr: i32)
    {
      let mut peopleGroup: i32 = this.game.Data.PeopleObj[this.game.Data.RegimeObj[regnr].People].PeopleGroup;
      if (this.game.Data.UnitCounter < 0)
        return;
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut unr1: i32 = 0; unr1 <= unitCounter; unr1 += 1)
      {
        if (this.game.Data.UnitObj[unr1].Regime == regnr & this.game.Data.UnitObj[unr1].PreDef <= -1)
        {
          bool flag = false;
          if (this.game.Data.Product >= 6 &&  this.game.Data.RuleVar[469] > 0.0 && 100 + this.game.HandyFunctionsObj.GetAverageDefensiveMod_SupplyOnly(unr1) < DrawMod.RandyNumber.Next(0, 100))
            flag = true;
          if (this.game.Data.UnitObj[unr1].SFCount > -1)
          {
            let mut sfCount: i32 = this.game.Data.UnitObj[unr1].SFCount;
            for (let mut index: i32 = 0; index <= sfCount; index += 1)
            {
              let mut sf: i32 = this.game.Data.UnitObj[unr1].SFList[index];
              let mut ap: i32 = this.game.Data.SFObj[sf].Ap;
              let mut mor: i32 = this.game.Data.SFObj[sf].Mor;
              let mut type: i32 = this.game.Data.SFObj[sf].Type;
              let mut num1: i32 =  Math.Round( this.game.Data.PeopleObj[this.game.Data.SFObj[sf].People].BaseMorale[peopleGroup] * ( this.game.Data.RegimeObj[regnr].BaseMorale / 100.0));
              if (!flag)
              {
                if (mor < num1)
                {
                  let mut num2: i32 =  Math.Round( (this.game.Data.RuleVar[65] *  num1));
                  let mut unr2: i32 = !this.game.Data.UnitObj[unr1].IsHQ ? this.game.Data.UnitObj[unr1].HQ : unr1;
                  if (unr2 > -1)
                  {
                    let mut num3: i32 = this.game.HandyFunctionsObj.GetStaffPercent(unr2, true);
                    if (num3 > 100)
                      num3 = 100;
                    let mut num4: i32 = this.game.HandyFunctionsObj.GetStaffPercent(unr2);
                    if (num4 > 100)
                      num4 = 100;
                    num2 =  Math.Round( num2 * (1.0 +  this.game.Data.RuleVar[141] * ( num4 / 100.0) * ( this.game.HandyFunctionsObj.Gethqpow(unr1) / 100.0) +  this.game.HandyFunctionsObj.GetStaffMoraleMod(unr2) * ( num3 / 100.0) * ( this.game.HandyFunctionsObj.Gethqpow(unr1) / 100.0)));
                  }
                  let mut num5: i32 =  Math.Round(Conversion.Int( num2 * ( this.game.Data.SFObj[sf].Rdn / 100.0)));
                  this.game.Data.SFObj[sf].Mor += num5;
                  if (this.game.Data.SFObj[sf].Mor > num1)
                    this.game.Data.SFObj[sf].Mor = num1;
                }
              }
              else
              {
                let mut num6: i32 =  Math.Round( num1 / 5.0);
                if (mor > num6)
                {
                  let mut num7: i32 =  Math.Round( this.game.Data.RuleVar[469]);
                  let mut num8: i32 = num7 -  Math.Round( (num7 * this.game.Data.SFObj[sf].Xp) / 100.0);
                  if (num8 > 0)
                  {
                    let mut num9: i32 = mor - num8;
                    if (num9 < num6)
                      num9 = num6;
                    this.game.Data.SFObj[sf].Mor = num9;
                  }
                }
              }
            }
          }
        }
      }
    }

    pub fn DoTraining(regnr: i32)
    {
      let mut peopleGroup: i32 = this.game.Data.PeopleObj[this.game.Data.RegimeObj[regnr].People].PeopleGroup;
      if (this.game.Data.UnitCounter < 0)
        return;
      let mut unitCounter1: i32 = this.game.Data.UnitCounter;
      for (let mut unr1: i32 = 0; unr1 <= unitCounter1; unr1 += 1)
      {
        if (this.game.Data.UnitObj[unr1].Regime == regnr & this.game.Data.UnitObj[unr1].PreDef <= -1 && this.game.Data.UnitObj[unr1].SFCount > -1)
        {
          let mut sfCount: i32 = this.game.Data.UnitObj[unr1].SFCount;
          for (let mut index1: i32 = 0; index1 <= sfCount; index1 += 1)
          {
            let mut sf: i32 = this.game.Data.UnitObj[unr1].SFList[index1];
            let mut ap: i32 = this.game.Data.SFObj[sf].Ap;
            let mut mor: i32 = this.game.Data.SFObj[sf].Mor;
            let mut type: i32 = this.game.Data.SFObj[sf].Type;
            let mut people: i32 = this.game.Data.SFObj[sf].People;
            let mut xp: i32 = this.game.Data.SFObj[sf].Xp;
            let mut unitGroup: i32 = this.game.Data.SFTypeObj[type].UnitGroup;
            let mut num1: i32 = 0;
            let mut unr2: i32 = !this.game.Data.UnitObj[unr1].IsHQ ? this.game.Data.UnitObj[unr1].HQ : unr1;
            if ( xp <  this.game.Data.RuleVar[63] * 0.5)
              num1 =  Math.Round( ( num1 + Conversion.Int(this.game.Data.RuleVar[64])));
            else if ( xp <  this.game.Data.RuleVar[63] * 0.75)
            {
              num1 =  Math.Round( ( num1 + Conversion.Int(this.game.Data.RuleVar[64] / 2f)));
              if (num1 == 0 &&  VBMath.Rnd() <  this.game.Data.RuleVar[64] / 2.0)
                num1 = 1;
            }
            else if ( xp <  this.game.Data.RuleVar[63])
            {
              num1 =  Math.Round( ( num1 + Conversion.Int(this.game.Data.RuleVar[64] / 4f)));
              if (num1 == 0 &&  VBMath.Rnd() <  this.game.Data.RuleVar[64] / 4.0)
                num1 = 1;
            }
            if (unr2 > -1)
            {
              let mut num2: i32 = this.game.HandyFunctionsObj.GetStaffPercent(unr2, true);
              if (num2 > 100)
                num2 = 100;
              num1 =  Math.Round( num1 * (1.0 +  num2 / 100.0 * ( this.game.HandyFunctionsObj.Gethqpow(unr1) / 100.0)));
            }
            SFClass[] sfObj = this.game.Data.SFObj;
            SFClass[] sfClassArray = sfObj;
            let mut index2: i32 = sf;
            let mut index3: i32 = index2;
            sfClassArray[index3].Xp = sfObj[index2].Xp + num1;
            if ( this.game.Data.SFObj[sf].Xp >  this.game.Data.RuleVar[81])
              this.game.Data.SFObj[sf].Xp =  Math.Round( this.game.Data.RuleVar[81]);
          }
        }
      }
      let mut unitCounter2: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter2; index += 1)
      {
        if (this.game.Data.Turn == this.game.Data.UnitObj[index].Regime & this.game.Data.UnitObj[index].PreDef == -1 && this.game.Data.UnitObj[index].UnitIsGiven)
          this.game.Data.UnitObj[index].UnitIsGiven = false;
      }
    }

    pub fn ClearTempUnitVariables()
    {
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
      {
        if (this.game.HandyFunctionsObj.GetRegime(this.game.Data.UnitObj[index].Regime) == this.game.Data.Turn)
        {
          this.game.Data.UnitObj[index].DidAttack = false;
          this.game.Data.UnitObj[index].DidHQ = false;
          this.game.Data.UnitObj[index].DidMove = false;
          this.game.Data.UnitObj[index].defensiveCombat = 0;
          this.game.Data.UnitObj[index].offensiveCombat = 0;
          this.game.Data.UnitObj[index].MoveAPSpent = 0;
        }
      }
    }

    pub fn GainApReserveRules(regNr: i32)
    {
      if ( this.game.Data.RuleVar[472] < 1.0 || this.game.Data.Product < 6)
        return;
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
      {
        if (this.game.HandyFunctionsObj.GetRegime(this.game.Data.UnitObj[unr].Regime) == regNr)
        {
          if (this.game.Data.UnitObj[unr].DidMove)
          {
            this.game.Data.UnitObj[unr].apReserve = 0;
          }
          else
          {
            UnitClass[] unitObj = this.game.Data.UnitObj;
            UnitClass[] unitClassArray = unitObj;
            let mut index1: i32 = unr;
            let mut index2: i32 = index1;
            unitClassArray[index2].apReserve = unitObj[index1].apReserve +  Math.Round( this.game.Data.RuleVar[472]);
            if ( Math.Round( this.game.Data.RuleVar[473]) < this.game.Data.UnitObj[unr].apReserve)
              this.game.Data.UnitObj[unr].apReserve =  Math.Round( this.game.Data.RuleVar[473]);
            let mut lowestAp: i32 = this.game.HandyFunctionsObj.GetLowestAp(unr);
            if (lowestAp <= 0)
              this.game.Data.UnitObj[unr].apReserve = 0;
            else if (lowestAp <= 50)
              this.game.Data.UnitObj[unr].apReserve =  Math.Round( (this.game.Data.UnitObj[unr].apReserve * lowestAp) / 50.0);
          }
        }
      }
    }

    pub fn ApToSf_SimplifiedSupplyRules()
    {
      if (this.game.Data.UnitCounter > -1)
      {
        let mut unitCounter: i32 = this.game.Data.UnitCounter;
        for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
        {
          if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unr].PreDef <= -1)
          {
            if ( this.game.Data.RuleVar[333] == 0.0 & !this.game.Data.UnitObj[unr].UnitIsGiven)
            {
              this.game.Data.UnitObj[unr].LastAP = -1;
              let mut num1: i32 = 0;
              let mut num2: i32 = 0;
              float num3;
              if (this.game.Data.UnitObj[unr].SFCount > -1)
              {
                let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
                for (let mut index: i32 = 0; index <= sfCount; index += 1)
                {
                  let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
                  if (0 > this.game.Data.SFObj[sf].Rdn)
                    this.game.Data.SFObj[sf].Rdn = 0;
                  if (0 > this.game.Data.SFObj[sf].Ap)
                    this.game.Data.SFObj[sf].Ap = 0;
                  let mut type: i32 = this.game.Data.SFObj[sf].Type;
                  let mut qty: i32 = this.game.Data.SFObj[sf].Qty;
                  num2 += this.game.Data.SFTypeObj[type].BasicSupplyNeed * qty;
                  if (this.game.Data.SFTypeObj[type].BasicSupplyNeed > 0 & num2 == 0)
                    num2 = 1;
                  let mut num4: i32 = this.game.HandyFunctionsObj.SFSupplyUse(sf);
                  num1 += num4;
                }
                float num5;
                if (this.game.Data.UnitObj[unr].OnBoard == -1)
                {
                  num3 = num1 <= this.game.Data.UnitObj[unr].Supply ? 1f :  this.game.Data.UnitObj[unr].Supply /  num1;
                  if (num1 > num2)
                  {
                    if (num2 < this.game.Data.UnitObj[unr].Supply)
                    {
                      if ( ( (this.game.Data.UnitObj[unr].Supply - num2) /  (num1 - num2)) > 1.0)
                        num5 = 1f;
                    }
                    else
                      num5 = 0.0f;
                  }
                  else
                    num5 = 1f;
                }
                else
                {
                  num3 = num1 <= this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply ? 1f :  this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply /  num1;
                  if (num1 > num2)
                  {
                    if (num2 < this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply)
                    {
                      if ( ( (this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply - num2) /  (num1 - num2)) > 1.0)
                        num5 = 1f;
                    }
                    else
                      num5 = 0.0f;
                  }
                  else
                    num5 = 1f;
                }
              }
              let mut num6: i32 = 0;
              let mut num7: i32 = 0;
              float num8 = 0.0f;
              if (this.game.Data.UnitObj[unr].SFCount > -1)
              {
                let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
                for (let mut index1: i32 = 0; index1 <= sfCount; index1 += 1)
                {
                  let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index1];
                  let mut type: i32 = this.game.Data.SFObj[sf].Type;
                  let mut qty: i32 = this.game.Data.SFObj[sf].Qty;
                  let mut num9: i32 = this.game.HandyFunctionsObj.SFSupplyUse(sf);
                  let mut num10: i32 = this.game.Data.SFTypeObj[type].BasicSupplyNeed * qty;
                  if (this.game.Data.SFTypeObj[type].BasicSupplyNeed > 0 & num10 == 0)
                    num10 = 1;
                  float num11 = num3;
                  let mut num12: i32 =  Math.Round( Conversion.Int( num9 * num11));
                  float num13 = num12 < num10 ?  num12 /  num10 : 1f;
                  num6 += qty;
                  num7 += num12;
                  num8 +=  qty * num13;
                  float rdn =  this.game.Data.SFObj[sf].Rdn;
                  if ( num13 > 0.5)
                  {
                    let mut num14: i32 =  Math.Round(Math.Floor( this.game.Data.RuleVar[59] * 2.0 * ( num13 - 0.5)));
                    SFClass[] sfObj = this.game.Data.SFObj;
                    SFClass[] sfClassArray = sfObj;
                    let mut index2: i32 = sf;
                    let mut index3: i32 = index2;
                    sfClassArray[index3].Rdn = sfObj[index2].Rdn + num14;
                  }
                  else if ( num13 < 0.5)
                  {
                    let mut num15: i32 =  Math.Round(Math.Floor( this.game.Data.RuleVar[61] * 2.0 * (0.5 -  num13)));
                    SFClass[] sfObj = this.game.Data.SFObj;
                    SFClass[] sfClassArray = sfObj;
                    let mut index4: i32 = sf;
                    let mut index5: i32 = index4;
                    sfClassArray[index5].Rdn = sfObj[index4].Rdn - num15;
                  }
                  if ( this.game.Data.SFObj[sf].Rdn <  this.game.Data.RuleVar[60])
                    this.game.Data.SFObj[sf].Rdn =  Math.Round( this.game.Data.RuleVar[60]);
                  if ( this.game.Data.RuleVar[399] > 0.0)
                  {
                    if (this.game.Data.UnitObj[unr].DidAttack & this.game.Data.UnitObj[unr].offensiveCombat > 0 | this.game.Data.UnitObj[unr].DidMove)
                    {
                      let mut num16: i32 =  Math.Round( this.game.Data.RuleVar[398]);
                      SFClass[] sfObj = this.game.Data.SFObj;
                      SFClass[] sfClassArray = sfObj;
                      let mut index6: i32 = sf;
                      let mut index7: i32 = index6;
                      sfClassArray[index7].Vigor = sfObj[index6].Vigor - num16;
                      if (this.game.Data.SFObj[sf].Vigor < 1)
                        this.game.Data.SFObj[sf].Vigor = 1;
                    }
                    else
                    {
                      let mut num17: i32 = 0;
                      let mut num18: i32 = 100;
                      if (this.game.Data.UnitObj[unr].X > -1)
                      {
                        let mut regimeCounter: i32 = this.game.Data.RegimeCounter;
                        for (let mut Index: i32 = 0; Index <= regimeCounter; Index += 1)
                        {
                          if (Index != this.game.Data.Turn && this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].get_ZocPts(Index) > 0)
                            num17 = 1;
                        }
                      }
                      if (this.game.Data.SFObj[sf].Vigor < 60)
                        unr = unr;
                      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                        num17 = 0;
                      if (num17 == 0)
                        num18 =  Math.Round( num18 * 1.5);
                      if ( Math.Round(Conversion.Val( this.game.Data.RuleVar[393])) == 1)
                        num18 =  Math.Round( num18 * 1.5);
                      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                        num18 =  Math.Round( num18 * 1.5);
                      if (this.game.Data.UnitObj[unr].defensiveCombat > 0 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                      {
                        num18 =  Math.Round(  Math.Round( num18 * (Math.Min(50.0,  Math.Min(100, this.game.Data.SFObj[sf].Rdn) / 2.0) / 100.0)) * ( Math.Min(100, this.game.Data.SFObj[sf].Rdn) / 100.0));
                        if (num18 < 0)
                          num18 = 0;
                      }
                      let mut num19: i32 =  Math.Round( num18 *  this.game.Data.RuleVar[399] / 100.0);
                      if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                        num19 =  Math.Round(Math.Ceiling( (num19 * Math.Max(20, this.game.Data.SFObj[sf].Vigor)) / 100.0));
                      SFClass[] sfObj = this.game.Data.SFObj;
                      SFClass[] sfClassArray = sfObj;
                      let mut index8: i32 = sf;
                      let mut index9: i32 = index8;
                      sfClassArray[index9].Vigor = sfObj[index8].Vigor + num19;
                      if (this.game.Data.SFObj[sf].Vigor > 100)
                        this.game.Data.SFObj[sf].Vigor = 100;
                    }
                  }
                  let mut num20: i32 = 100;
                  if ( this.game.Data.RuleVar[399] > 0.0)
                  {
                    num20 =  Math.Round(Math.Ceiling(Math.Sqrt( this.game.Data.SFObj[sf].Vigor) * 10.0));
                    if (num20 > 100)
                      num20 = 100;
                  }
                  if (this.game.Data.SFObj[sf].Rdn > num20)
                    this.game.Data.SFObj[sf].Rdn = num20;
                  if ( this.game.Data.RuleVar[814] < 1.0)
                  {
                    if (this.game.Data.UnitObj[unr].OnBoard == -1)
                      this.game.Data.UnitObj[unr].Supply -= num12;
                    else
                      this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply -= num12;
                  }
                }
                if (num6 > 0)
                {
                  float num21 = num8 /  num6;
                  if ( num21 < 1.0)
                  {
                    let mut num22: i32 =  Math.Round( (this.game.Data.RuleVar[437] * (1f - num21)));
                    UnitClass[] unitObj = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray = unitObj;
                    let mut index10: i32 = unr;
                    let mut index11: i32 = index10;
                    unitClassArray[index11].SupplyConsume = unitObj[index10].SupplyConsume - num22;
                    if (0 > this.game.Data.UnitObj[unr].SupplyConsume)
                      this.game.Data.UnitObj[unr].SupplyConsume = 0;
                  }
                  else
                  {
                    let mut num23: i32 =  Math.Round( (this.game.Data.RuleVar[437] * 2f));
                    UnitClass[] unitObj = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray = unitObj;
                    let mut index12: i32 = unr;
                    let mut index13: i32 = index12;
                    unitClassArray[index13].SupplyConsume = unitObj[index12].SupplyConsume + num23;
                    if (100 < this.game.Data.UnitObj[unr].SupplyConsume)
                      this.game.Data.UnitObj[unr].SupplyConsume = 100;
                  }
                }
                else
                  this.game.Data.UnitObj[unr].SupplyConsume = 0;
                this.MaxReadinessRule(unr);
              }
            }
            else
              this.game.Data.UnitObj[unr].SupplyConsume = 100;
          }
        }
      }
      if (this.game.Data.UnitCounter > -1)
      {
        let mut unitCounter: i32 = this.game.Data.UnitCounter;
        for (let mut index14: i32 = 0; index14 <= unitCounter; index14 += 1)
        {
          if (this.game.Data.UnitObj[index14].Regime == this.game.Data.Turn && !this.game.Data.UnitObj[index14].UnitIsGiven && this.game.Data.UnitObj[index14].SFCount > -1)
          {
            let mut sfCount: i32 = this.game.Data.UnitObj[index14].SFCount;
            for (let mut index15: i32 = 0; index15 <= sfCount; index15 += 1)
            {
              let mut sf: i32 = this.game.Data.UnitObj[index14].SFList[index15];
              this.game.Data.SFObj[sf].Ap = 50;
              SFClass[] sfObj1 = this.game.Data.SFObj;
              SFClass[] sfClassArray1 = sfObj1;
              let mut index16: i32 = sf;
              let mut index17: i32 = index16;
              sfClassArray1[index17].Ap = sfObj1[index16].Ap +  Math.Round(Math.Floor( this.game.Data.SFObj[sf].Rdn * 0.5));
              this.game.Data.SFObj[sf].Ap =  Math.Round(Math.Floor( (this.game.Data.SFObj[sf].Ap * this.game.Data.UnitObj[index14].SupplyConsume) / 100.0));
              if (this.game.Data.SFObj[sf].Ap > 100)
                this.game.Data.SFObj[sf].Ap = 100;
              this.game.Data.SFObj[sf].Ap =  Math.Round( ( this.game.Data.SFObj[sf].Ap * this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ApMod));
              let mut sfEpGrowth: i32 = this.game.HandyFunctionsObj.GetSfEPGrowth(sf);
              SFClass[] sfObj2 = this.game.Data.SFObj;
              SFClass[] sfClassArray2 = sfObj2;
              let mut index18: i32 = sf;
              let mut index19: i32 = index18;
              sfClassArray2[index19].EP =  Math.Round( sfObj2[index18].EP +  sfEpGrowth * ( this.game.Data.SFObj[sf].Ap / 100.0));
              if ( sfEpGrowth *  this.game.Data.RuleVar[42] <  this.game.Data.SFObj[sf].EP)
                this.game.Data.SFObj[sf].EP =  Math.Round( ( sfEpGrowth * this.game.Data.RuleVar[42]));
              if (this.game.Data.UnitObj[index14].X >= 0 && this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater == 1 | this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.Data.UnitObj[index14].Map].HexObj[this.game.Data.UnitObj[index14].X, this.game.Data.UnitObj[index14].Y].LandscapeType].IsSea &&  this.game.Data.RuleVar[44] >  this.game.Data.SFObj[sf].Ap)
                this.game.Data.SFObj[sf].Ap =  Math.Round( this.game.Data.RuleVar[44]);
              if (this.game.Data.UnitObj[index14].SetAPToZero == 1)
                this.game.Data.SFObj[sf].Ap = 0;
            }
          }
          this.game.Data.UnitObj[index14].SetAPToZero = 0;
        }
      }
      this.MaximumHQSuppliesRule();
    }

    pub fn ApToSf()
    {
      if ( this.game.Data.RuleVar[434] > 0.0)
      {
        this.ApToSf_SimplifiedSupplyRules();
      }
      else
      {
        if (this.game.Data.UnitCounter > -1)
        {
          let mut unitCounter: i32 = this.game.Data.UnitCounter;
          for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
          {
            if (unr == 137)
              unr = unr;
            if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unr].PreDef <= -1)
            {
              if ( this.game.Data.RuleVar[333] == 0.0 & !this.game.Data.UnitObj[unr].UnitIsGiven)
              {
                this.game.Data.UnitObj[unr].LastAP = -1;
                let mut num1: i32 = 0;
                let mut num2: i32 = 0;
                if (unr == 154)
                  unr = unr;
                float num3;
                float num4;
                if (this.game.Data.UnitObj[unr].SFCount > -1)
                {
                  let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
                  for (let mut index: i32 = 0; index <= sfCount; index += 1)
                  {
                    let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
                    if (0 > this.game.Data.SFObj[sf].Rdn)
                      this.game.Data.SFObj[sf].Rdn = 0;
                    if (0 > this.game.Data.SFObj[sf].Ap)
                      this.game.Data.SFObj[sf].Ap = 0;
                    let mut type: i32 = this.game.Data.SFObj[sf].Type;
                    let mut qty: i32 = this.game.Data.SFObj[sf].Qty;
                    num2 =  Math.Round( num2 +  this.game.Data.SFTypeObj[type].BasicSupplyNeed / 2.0 *  qty);
                    if (this.game.Data.SFTypeObj[type].BasicSupplyNeed > 0 & num2 == 0)
                      num2 = 1;
                    let mut num5: i32 = this.game.HandyFunctionsObj.SFSupplyUse(sf);
                    num1 += num5;
                  }
                  if (this.game.Data.UnitObj[unr].OnBoard == -1)
                  {
                    num3 = num1 <= this.game.Data.UnitObj[unr].Supply ? 1f :  this.game.Data.UnitObj[unr].Supply /  num1;
                    if (num1 > num2)
                    {
                      if (num2 < this.game.Data.UnitObj[unr].Supply)
                      {
                        num4 =  (this.game.Data.UnitObj[unr].Supply - num2) /  (num1 - num2);
                        if ( num4 > 1.0)
                          num4 = 1f;
                      }
                      else
                        num4 = 0.0f;
                    }
                    else
                      num4 = 1f;
                  }
                  else
                  {
                    num3 = num1 <= this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply ? 1f :  this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply /  num1;
                    if (num1 > num2)
                    {
                      if (num2 < this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply)
                      {
                        num4 =  (this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply - num2) /  (num1 - num2);
                        if ( num4 > 1.0)
                          num4 = 1f;
                      }
                      else
                        num4 = 0.0f;
                    }
                    else
                      num4 = 1f;
                  }
                }
                if (unr == 65)
                  ;
                let mut num6: i32 = 0;
                let mut num7: i32 = 0;
                float num8 = 0.0f;
                if (this.game.Data.UnitObj[unr].SFCount > -1)
                {
                  let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
                  for (let mut index: i32 = 0; index <= sfCount; index += 1)
                  {
                    let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
                    let mut type: i32 = this.game.Data.SFObj[sf].Type;
                    let mut qty: i32 = this.game.Data.SFObj[sf].Qty;
                    let mut num9: i32 = this.game.HandyFunctionsObj.SFSupplyUse(sf);
                    let mut num10: i32 =  Math.Round( this.game.Data.SFTypeObj[type].BasicSupplyNeed / 2.0 *  qty);
                    if (this.game.Data.SFTypeObj[type].BasicSupplyNeed > 0 & num10 == 0)
                      num10 = 1;
                    float num11 = num3;
                    let mut num12: i32 =  Math.Round( Conversion.Int( num9 * num11));
                    float num13 = num12 < num10 ?  (1.0 -  num12 /  num10) : 0.0f;
                    num6 += qty;
                    num7 += num12;
                    num8 +=  qty * num13;
                    float rdn =  this.game.Data.SFObj[sf].Rdn;
                    let mut num14: i32 =  Math.Round( (  Math.Round( this.game.Data.RuleVar[61]) * num13));
                    this.game.Data.SFObj[sf].Rdn =  Math.Round(Conversion.Int( this.game.Data.SFObj[sf].Rdn * ( (100 - num14) / 100.0)));
                    if ( this.game.Data.SFObj[sf].Rdn <  this.game.Data.RuleVar[60])
                      this.game.Data.SFObj[sf].Rdn =  Math.Round( this.game.Data.RuleVar[60]);
                    if ( num13 == 0.0)
                    {
                      let mut num15: i32 =  Math.Round( (this.game.Data.RuleVar[59] * num4));
                      num16: i32;
                      if (this.game.Data.UnitObj[unr].X > -1)
                      {
                        if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.Data.UnitObj[unr].Map].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].LandscapeType].IsSea)
                          num16 =  Math.Round(Conversion.Int(Math.Sqrt( this.game.Data.MapObj[this.game.Data.UnitObj[unr].Map].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].DammageToInfra)));
                      }
                      else
                        num16 = 0;
                      if (num16 > 90)
                        num16 = 90;
                      if (this.game.Data.UnitObj[unr].X > -1 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.Data.UnitObj[unr].Map].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].LandscapeType].IsSea)
                        num15 =  Math.Round( num15 * (1.0 -  num16 / 100.0));
                      this.game.Data.SFObj[sf].Rdn += num15;
                      if (100 < this.game.Data.SFObj[sf].Rdn)
                        this.game.Data.SFObj[sf].Rdn = 100;
                    }
                    if ( this.game.Data.RuleVar[814] < 1.0)
                    {
                      if (this.game.Data.UnitObj[unr].OnBoard == -1)
                        this.game.Data.UnitObj[unr].Supply -= num12;
                      else
                        this.game.Data.UnitObj[this.game.Data.UnitObj[unr].OnBoard].Supply -= num12;
                    }
                  }
                  if (num6 > 0)
                  {
                    float num17 = num8 /  num6;
                    this.game.Data.UnitObj[unr].SupplyConsume =  Math.Round(100.0 * (1.0 -  num17));
                  }
                  else
                    this.game.Data.UnitObj[unr].SupplyConsume = 0;
                  this.MaxReadinessRule(unr);
                }
              }
              else
                this.game.Data.UnitObj[unr].SupplyConsume = 100;
            }
          }
        }
        if (this.game.Data.UnitCounter > -1)
        {
          let mut unitCounter: i32 = this.game.Data.UnitCounter;
          for (let mut index1: i32 = 0; index1 <= unitCounter; index1 += 1)
          {
            if (this.game.Data.UnitObj[index1].Regime == this.game.Data.Turn && !this.game.Data.UnitObj[index1].UnitIsGiven && this.game.Data.UnitObj[index1].SFCount > -1)
            {
              let mut sfCount: i32 = this.game.Data.UnitObj[index1].SFCount;
              for (let mut index2: i32 = 0; index2 <= sfCount; index2 += 1)
              {
                let mut sf: i32 = this.game.Data.UnitObj[index1].SFList[index2];
                this.game.Data.SFObj[sf].Ap =  Math.Round(Conversion.Int( this.game.Data.SFObj[sf].Rdn * 0.5 +  this.game.Data.UnitObj[index1].SupplyConsume * 0.5));
                if (this.game.Data.SFObj[sf].Ap > 100)
                  this.game.Data.SFObj[sf].Ap = 100;
                this.game.Data.SFObj[sf].Ap =  Math.Round( ( this.game.Data.SFObj[sf].Ap * this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ApMod));
                let mut sfEpGrowth: i32 = this.game.HandyFunctionsObj.GetSfEPGrowth(sf);
                SFClass[] sfObj = this.game.Data.SFObj;
                SFClass[] sfClassArray = sfObj;
                let mut index3: i32 = sf;
                let mut index4: i32 = index3;
                sfClassArray[index4].EP =  Math.Round( sfObj[index3].EP +  sfEpGrowth * ( this.game.Data.SFObj[sf].Ap / 100.0));
                if ( sfEpGrowth *  this.game.Data.RuleVar[42] <  this.game.Data.SFObj[sf].EP)
                  this.game.Data.SFObj[sf].EP =  Math.Round( ( sfEpGrowth * this.game.Data.RuleVar[42]));
                if (this.game.Data.UnitObj[index1].X >= 0 && this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater == 1 | this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.Data.UnitObj[index1].Map].HexObj[this.game.Data.UnitObj[index1].X, this.game.Data.UnitObj[index1].Y].LandscapeType].IsSea &&  this.game.Data.RuleVar[44] >  this.game.Data.SFObj[sf].Ap)
                  this.game.Data.SFObj[sf].Ap =  Math.Round( this.game.Data.RuleVar[44]);
                if (this.game.Data.UnitObj[index1].SetAPToZero == 1)
                  this.game.Data.SFObj[sf].Ap = 0;
              }
            }
            this.game.Data.UnitObj[index1].SetAPToZero = 0;
          }
        }
        this.MaximumHQSuppliesRule();
      }
    }

    pub fn MaximumHQSuppliesRule()
    {
      if ( this.game.Data.RuleVar[336] == 1.0)
      {
        let mut unitCounter: i32 = this.game.Data.UnitCounter;
        for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
        {
          if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn && this.game.Data.UnitObj[unr].Supply > this.game.HandyFunctionsObj.UnitSupplyStore(unr))
            this.game.Data.UnitObj[unr].Supply = this.game.HandyFunctionsObj.UnitSupplyStore(unr);
        }
      }
      else
      {
        if ( this.game.Data.RuleVar[336] <= 100.0)
          return;
        let mut unitCounter: i32 = this.game.Data.UnitCounter;
        for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
        {
          if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn && this.game.Data.UnitObj[unr].Supply > this.game.HandyFunctionsObj.UnitSupplyStore(unr))
          {
            bool flag = false;
            if (this.game.Data.UnitObj[unr].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].Type == 8)
              flag = true;
            if (this.game.Data.UnitObj[unr].HQ == -1 | flag)
            {
              let mut num1: i32 = this.game.Data.UnitObj[unr].Supply - this.game.HandyFunctionsObj.UnitSupplyStore(unr);
              if ( num1 >  this.game.Data.RuleVar[336])
              {
                let mut num2: i32 =  Math.Round( ( num1 - this.game.Data.RuleVar[336]));
                this.game.Data.UnitObj[unr].Supply -= num2;
              }
            }
            else if (this.game.Data.UnitObj[unr].Supply > this.game.HandyFunctionsObj.UnitSupplyStore(unr))
              this.game.Data.UnitObj[unr].Supply = this.game.HandyFunctionsObj.UnitSupplyStore(unr);
          }
        }
      }
    }

    pub fn AutoReinforce_UnitWillGive(unr: i32, typ: i32, movtyp: i32, ppl: i32) -> i32
    {
      let mut sfCount1: i32 = this.game.Data.UnitObj[unr].SFCount;
      for (let mut index: i32 = 0; index <= sfCount1; index += 1)
      {
        let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
        let mut reinforcementType: i32 = this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ReinforcementType;
        let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
        let mut people: i32 = this.game.Data.SFObj[sf].People;
        if (typ == reinforcementType & movtyp == moveType & ppl == people)
          return sf;
      }
      let mut sfCount2: i32 = this.game.Data.UnitObj[unr].SFCount;
      for (let mut index: i32 = 0; index <= sfCount2; index += 1)
      {
        let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
        let mut reinforcementType2: i32 = this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ReinforcementType2;
        let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
        let mut people: i32 = this.game.Data.SFObj[sf].People;
        if (typ == reinforcementType2 & movtyp == moveType & ppl == people)
          return sf;
      }
      return -1;
    }

    pub AutoReinforce_HQwillGive: i32(
      hq: i32,
      unr: i32,
      SimpleList ideallist,
      typ: i32,
      movtyp: i32,
      ppl: i32,
      secondaryPhase: i32)
    {
      if (ideallist.FindNr(typ, movtyp, ppl) == -1)
        return -1;
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      SimpleList simpleList3 = SimpleList::new();
      let mut val1: i32 = 999;
      if (this.game.Data.Product == 6)
        val1 = 1;
      let mut nr: i32 = ideallist.FindNr(typ, movtyp, ppl);
      if (nr == -1)
        return -1;
      let mut num1: i32 = ideallist.Data3[nr];
      if (this.game.Data.UnitObj[unr].IsHQ)
        num1 = 99;
      let mut sfCount1: i32 = this.game.Data.UnitObj[unr].SFCount;
      num2: i32;
      for (let mut index: i32 = 0; index <= sfCount1; index += 1)
      {
        let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 = this.game.Data.SFObj[sf].Type;
        let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
        let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
        let mut people: i32 = this.game.Data.SFObj[sf].People;
        if (reinforcementType == typ & moveType == movtyp & people == ppl)
        {
          num2 += 1;
          simpleList1.Add(type, this.game.Data.SFObj[sf].Qty * Math.Min(val1, this.game.Data.SFTypeObj[type].PowerPts));
        }
      }
      simpleList1.Sort();
      let mut sfCount2: i32 = this.game.Data.UnitObj[hq].SFCount;
      for (let mut index: i32 = 0; index <= sfCount2; index += 1)
      {
        let mut sf: i32 = this.game.Data.UnitObj[hq].SFList[index];
        let mut type: i32 = this.game.Data.SFObj[sf].Type;
        let mut num3: i32 = secondaryPhase != 0 ? this.game.Data.SFTypeObj[type].ReinforcementType2 : this.game.Data.SFTypeObj[type].ReinforcementType;
        let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
        let mut people: i32 = this.game.Data.SFObj[sf].People;
        if (num3 == typ & moveType == movtyp & people == ppl)
          simpleList3.Add(type, this.game.Data.SFObj[sf].Qty * Math.Min(val1, this.game.Data.SFTypeObj[type].PowerPts), sf);
      }
      if (Strings.InStr(this.game.Data.UnitObj[unr].Name, "***") > 0)
        unr = unr;
      if ( this.game.Data.RuleVar[910] > 0.0 |  this.game.Data.RuleVar[911] > 0.0 |  this.game.Data.RuleVar[912] > 0.0 && !this.game.HandyFunctionsObj.HasUnitAirSF(unr) & !this.game.HandyFunctionsObj.HasUnitNavySF(unr) & !this.game.Data.UnitObj[unr].IsHQ)
      {
        let mut num4: i32 = 0;
        let mut sfCount3: i32 = this.game.Data.UnitObj[unr].SFCount;
        for (let mut index: i32 = 0; index <= sfCount3; index += 1)
        {
          let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
          let mut type: i32 = this.game.Data.SFObj[sf].Type;
          let mut num5: i32 = 1;
          if ( this.game.Data.RuleVar[910] ==  this.game.Data.SFTypeObj[type].MoveType)
            num5 = 0;
          if ( this.game.Data.RuleVar[911] ==  this.game.Data.SFTypeObj[type].MoveType)
            num5 = 0;
          if ( this.game.Data.RuleVar[912] ==  this.game.Data.SFTypeObj[type].MoveType)
            num5 = 0;
          if (this.game.Data.SFTypeObj[type].CarryCap > 0)
            num5 = 0;
          if (num5 == 1)
            num4 += this.game.Data.SFTypeObj[type].Weight * this.game.Data.SFObj[sf].Qty;
        }
        this.game.HandyFunctionsObj.GetMoveType(unr);
        let mut num6: i32 = 0;
        num7: i32;
        if (this.game.Data.UnitObj[unr].Historical > -1 && this.game.Data.UnitObj[unr].HistoricalSubPart > -1)
        {
          num7 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].SubParts[this.game.Data.UnitObj[unr].HistoricalSubPart];
          if (num7 > -1)
          {
            num7 = this.game.HandyFunctionsObj.GetPreDef(num7);
            num6 = this.game.HandyFunctionsObj.GetCarryCapPts(num7, 0);
          }
        }
        let mut carryCapPts: i32 = this.game.HandyFunctionsObj.GetCarryCapPts(unr, 0);
        if (simpleList3.Counter > -1)
        {
          for (let mut counter: i32 = simpleList3.Counter; counter <= 0; counter += 1)
          {
            let mut index: i32 = simpleList3.Id[counter];
            if (num4 + this.game.Data.SFTypeObj[index].Weight > carryCapPts & num6 > carryCapPts)
            {
              let mut num8: i32 = 1;
              if (num7 > -1 && this.game.HandyFunctionsObj.GetMoveType(num7) == this.game.Data.SFTypeObj[index].MoveType)
                num8 = 0;
              if ( this.game.Data.RuleVar[910] ==  this.game.Data.SFTypeObj[index].MoveType)
                num8 = 0;
              if ( this.game.Data.RuleVar[911] ==  this.game.Data.SFTypeObj[index].MoveType)
                num8 = 0;
              if ( this.game.Data.RuleVar[912] ==  this.game.Data.SFTypeObj[index].MoveType)
                num8 = 0;
              if (num8 == 1)
                simpleList3.RemoveNr(counter);
            }
          }
        }
      }
      simpleList3.Sort();
      let mut num9: i32 = 1;
      if (num1 > num2)
      {
        for (let mut counter: i32 = simpleList3.Counter; counter >= 0; counter += -1)
        {
          if (simpleList1.FindNr(simpleList3.Id[counter]) == -1)
            return simpleList3.Data1[counter];
        }
      }
      if (num9 == 1)
      {
        let mut counter1: i32 = simpleList1.Counter;
        for (let mut index: i32 = 0; index <= counter1; index += 1)
        {
          for (let mut counter2: i32 = simpleList3.Counter; counter2 >= 0; counter2 += -1)
          {
            if (simpleList3.Id[counter2] == simpleList1.Id[index])
              return simpleList3.Data1[counter2];
          }
        }
      }
      for (let mut counter: i32 = simpleList3.Counter; counter >= 0; counter += -1)
      {
        if (simpleList1.FindNr(simpleList3.Id[counter]) == -1)
          return simpleList3.Data1[counter];
      }
      return this.game.Data.UnitObj[unr].SFCount >= 7 | !this.game.Data.UnitObj[unr].IsHQ ? -2 : -1;
    }

    pub fn WritePLog(s: String)
    {
      if ( this.game.Data.RuleVar[948] < 1.0 & !this.game.EventRelatedObj.Helper_IsDebug())
        return;
      StreamWriter text = File.CreateText(this.game.AppPath + "logs/" + s + ".txt");
      let mut plogcounter: i32 = this.plogcounter;
      for (let mut index: i32 = 0; index <= plogcounter; index += 1)
        text.WriteLine(this.plog[index]);
      text.Close();
    }

    pub fn AddPLog(s: String)
    {
      if ( this.game.Data.RuleVar[948] < 1.0 & !this.game.EventRelatedObj.Helper_IsDebug())
        return;
      this += 1.plogcounter;
      this.plog = (string[]) Utils.CopyArray((Array) this.plog, (Array) new string[this.plogcounter + 1]);
      this.plog[this.plogcounter] = s;
    }

    pub fn DoAutoReinforce()
    {
      Coordinate[] coordinateArray = new Coordinate[251];
      int[] numArray1 = new int[this.game.Data.UnitCounter + 1];
      int[] numArray2 = new int[this.game.Data.UnitCounter + 1];
      int[] numArray3 = new int[this.game.Data.UnitCounter + 1];
      SimpleList[] simpleListArray1 = new SimpleList[this.game.Data.UnitCounter + 1];
      SimpleList[] simpleListArray2 = new SimpleList[this.game.Data.UnitCounter + 1];
      SimpleList[] simpleListArray3 = new SimpleList[this.game.Data.UnitCounter + 1];
      SimpleList[] simpleListArray4 = new SimpleList[this.game.Data.UnitCounter + 1];
      SimpleList[] simpleListArray5 = new SimpleList[this.game.Data.UnitCounter + 1];
      if (this.game.Data.UnitCounter < 0)
        return;
      int[] numArray4 = new int[this.game.Data.UnitCounter + 1];
      int[] numArray5 = new int[this.game.Data.UnitCounter + 1];
      MapMatrix2[] mapMatrix2Array1 = new MapMatrix2[this.game.Data.MapCounter + 1];
      MapMatrix2[] mapMatrix2Array2 = new MapMatrix2[this.game.Data.MapCounter + 1];
      MapMatrix2[] mapMatrix2Array3 = new MapMatrix2[this.game.Data.MapCounter + 1];
      let mut val1_1: i32 = 999;
      if (this.game.Data.Product == 6)
        val1_1 = 1;
      bool useTrafficRules = false;
      if ( this.game.Data.RuleVar[459] > 0.0 & this.game.Data.Product >= 6)
        useTrafficRules = true;
      let mut num1: i32 = -1;
      let mut unitCounter1: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
        numArray4[index] = -1;
      index1: i32;
      if ( this.game.Data.RuleVar[499] > 0.0)
      {
        let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[499]));
        if (stringListById > -1)
        {
          bool[] flaggy = new bool[this.game.Data.StringListObj[stringListById].Length + 1];
          for (let mut length: i32 = this.game.Data.StringListObj[stringListById].Length; length >= 0; length += -1)
          {
            index1 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[length, 0]));
            if (index1 == this.game.Data.Turn)
              flaggy[length] = true;
          }
          this.game.Data.StringListObj[stringListById].RemoveMultipleRow(flaggy);
        }
      }
      this.plogcounter = -1;
      this.AddPLog("AUTO-REINFORCE");
      this.AddPLog("");
      for (let mut unitCounter2: i32 = this.game.Data.UnitCounter; unitCounter2 >= 0; unitCounter2 += -1)
      {
        if ( this.game.Data.RuleVar[977] > 0.0 && this.game.Data.UnitObj[unitCounter2].SOReplacementPercent == 0)
          this.game.Data.UnitObj[unitCounter2].SOReplacementPercent = 100;
        let mut num2: i32 =  this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[unitCounter2].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(unitCounter2));
        if (this.game.Data.UnitObj[unitCounter2].SFCount == -1 & num2 == 0 & this.game.Data.UnitObj[unitCounter2].X > -1 & this.game.Data.UnitObj[unitCounter2].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unitCounter2].PreDef == -1)
        {
          let mut Number: i32 = 0;
          if (this.game.Data.UnitObj[unitCounter2].Historical > -1)
          {
            let mut index2: i32 = this.game.Data.UnitObj[unitCounter2].Historical;
            if (this.game.Data.HistoricalUnitObj[index2].ModelMaster > -1)
              index2 = this.game.Data.HistoricalUnitObj[index2].ModelMaster;
            if (this.game.Data.HistoricalUnitObj[index2].PP > 0)
            {
              let mut num3: i32 = 0;
              let mut index3: i32 = 0;
              do
              {
                if (this.game.Data.HistoricalUnitObj[index2].SubParts[index3] > -1)
                  num3 += 1;
                index3 += 1;
              }
              while (index3 <= 9);
              Number =  Math.Round( this.game.Data.HistoricalUnitObj[index2].PP /  num3);
            }
          }
          this.AddPLog("(RETURN) DISBAND EMPTY UNIT = " + this.game.Data.UnitObj[unitCounter2].Name + " earns " + Conversion.Str( Number) + " pp poas: i32 refund.");
          data: DataClass = this.game.Data;
          let mut nr: i32 = unitCounter2;
          let mut gameClass: GameClass = (GameClass) null;
           let mut local: GameClass =  gameClass;
          data.RemoveUnit(nr,  local);
          if (!(this.game.Data.RegimeObj[this.game.Data.Turn].AI &  this.game.Data.RuleVar[914] == 1.0))
          {
            RegimeClass[] regimeObj = this.game.Data.RegimeObj;
            RegimeClass[] regimeClassArray = regimeObj;
            let mut turn: i32 = this.game.Data.Turn;
            let mut index4: i32 = turn;
            regimeClassArray[index4].ResPts = regimeObj[turn].ResPts + Number;
          }
        }
      }
      num4: i32;
      do
      {
        num4 = 0;
        num1 += 1;
        let mut unitCounter3: i32 = this.game.Data.UnitCounter;
        for (let mut index5: i32 = 0; index5 <= unitCounter3; index5 += 1)
        {
          if (this.game.Data.UnitObj[index5].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index5].IsHQ & this.game.Data.UnitObj[index5].PreDef <= -1)
          {
            if (num1 == 0)
            {
              if (this.game.Data.UnitObj[index5].HQ == -1)
              {
                numArray4[index5] = 0;
                num4 = 1;
              }
            }
            else if (this.game.Data.UnitObj[index5].HQ > -1 & numArray4[index5] == -1 & this.game.Data.UnitObj[index5].PreDef <= -1)
            {
              let mut hq: i32 = this.game.Data.UnitObj[index5].HQ;
              if (numArray4[hq] == num1 - 1)
              {
                num4 = 1;
                numArray4[index5] = num1;
              }
            }
          }
        }
      }
      while (num4 > 0);
      let mut num5: i32 = num1 - 1;
      if ( this.game.Data.RuleVar[887] == 1.0)
        num5 = 0;
      for (let mut index6: i32 = num5; index6 >= 0; index6 += -1)
      {
        let mut unitCounter4: i32 = this.game.Data.UnitCounter;
        for (let mut index7: i32 = 0; index7 <= unitCounter4; index7 += 1)
        {
          if (this.game.Data.UnitObj[index7].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index7].IsHQ & this.game.Data.UnitObj[index7].PreDef <= -1 && numArray4[index7] == index6)
          {
            this.AddPLog("");
            this.AddPLog("----------------------");
            this.AddPLog("----------------------");
            this.AddPLog("AUTO-REINFORCE CHECKING FOR " + this.game.Data.UnitObj[index7].Name);
            this.AddPLog("----------------------");
            this.AddPLog("----------------------");
            this.AddPLog("");
            CoordList coordList1;
            if ( this.game.Data.RuleVar[340] > -1.0)
            {
              coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn,  Math.Round( this.game.Data.RuleVar[2]), 0,  Math.Round( this.game.Data.RuleVar[339]), this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map, allowshoredrop: true, SeaBlock: true);
              let mut mapCounter1: i32 = this.game.Data.MapCounter;
              for (let mut index8: i32 = 0; index8 <= mapCounter1; index8 += 1)
                mapMatrix2Array2[index8] = this.game.EditObj.TempValue[index8].Clone();
              coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn,  Math.Round( this.game.Data.RuleVar[1]), 1,  Math.Round( this.game.Data.RuleVar[339]), this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map, allowshoredrop: true, SeaBlock: true);
              let mut mapCounter2: i32 = this.game.Data.MapCounter;
              for (let mut index9: i32 = 0; index9 <= mapCounter2; index9 += 1)
                mapMatrix2Array1[index9] = this.game.EditObj.TempValue[index9].Clone();
              coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn,  Math.Round( this.game.Data.RuleVar[0]), 0,  Math.Round( this.game.Data.RuleVar[339]), this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map, allowshoredrop: true, SeaBlock: true);
              let mut mapCounter3: i32 = this.game.Data.MapCounter;
              for (let mut index10: i32 = 0; index10 <= mapCounter3; index10 += 1)
                mapMatrix2Array3[index10] = this.game.EditObj.TempValue[index10].Clone();
            }
            let mut num6: i32 = 1;
            do
            {
              let mut secondaryPhase: i32 = 0;
              do
              {
                Application.DoEvents();
                let mut unitCounter5: i32 = this.game.Data.UnitCounter;
                for (let mut index11: i32 = 0; index11 <= unitCounter5; index11 += 1)
                  simpleListArray2[index11] = SimpleList::new();
                index12: i32;
                if ( this.game.Data.RuleVar[455] > 0.0)
                {
                  this.game.HandyFunctionsObj.MakeFuzzyOwner(true, false, this.game.Data.Turn);
                  this.game.HandyFunctionsObj.RedimTempValue3(0);
                  let mut mapWidth: i32 = this.game.Data.MapObj[index12].MapWidth;
                  for (let mut index13: i32 = 0; index13 <= mapWidth; index13 += 1)
                  {
                    let mut mapHeight: i32 = this.game.Data.MapObj[index12].MapHeight;
                    for (let mut index14: i32 = 0; index14 <= mapHeight; index14 += 1)
                      this.game.EditObj.TempValue3[index12].Value[index13, index14] = this.game.Data.MapObj[0].HexObj[index13, index14].Regime;
                  }
                  this.game.HandyFunctionsObj.MakeFuzzyOwner(false, false, this.game.Data.Turn);
                }
                coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn,  Math.Round( this.game.Data.RuleVar[99]), 99,  Math.Round( this.game.Data.RuleVar[339]), this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                this.AddPLog("");
                this.AddPLog("*********************");
                this.AddPLog("FOR HQ: " + this.game.Data.UnitObj[index7].Name + " , PRIORITY CYCLE: " + num6.ToString() + " , REINFORCEMENTTYPE1/2 = " + (secondaryPhase + 1).ToString());
                this.AddPLog("*********************");
                this.AddPLog("");
                SimpleList simpleList1 = SimpleList::new();
                let mut index15: i32 = index7;
                let mut sfCount1: i32 = this.game.Data.UnitObj[index15].SFCount;
                for (let mut index16: i32 = 0; index16 <= sfCount1; index16 += 1)
                {
                  let mut sf: i32 = this.game.Data.UnitObj[index15].SFList[index16];
                  let mut type: i32 = this.game.Data.SFObj[sf].Type;
                  index1 = secondaryPhase != 0 ? this.game.Data.SFTypeObj[type].ReinforcementType2 : this.game.Data.SFTypeObj[type].ReinforcementType;
                  let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                  let mut people: i32 = this.game.Data.SFObj[sf].People;
                  let mut nr: i32 = simpleList1.FindNr(index1, moveType, people);
                  if (nr > -1)
                  {
                    int[] weight = simpleList1.Weight;
                    int[] numArray6 = weight;
                    let mut index17: i32 = nr;
                    let mut index18: i32 = index17;
                    let mut num7: i32 = weight[index17] + this.game.Data.SFObj[sf].Qty * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                    numArray6[index18] = num7;
                    int[] data3 = simpleList1.Data3;
                    int[] numArray7 = data3;
                    let mut index19: i32 = nr;
                    let mut index20: i32 = index19;
                    let mut num8: i32 = data3[index19] + 1;
                    numArray7[index20] = num8;
                  }
                  else
                    simpleList1.Add(index1, this.game.Data.SFObj[sf].Qty * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts), moveType, people, 1, CheckExistence: false);
                }
                let mut historical1: i32 = this.game.Data.UnitObj[index7].Historical;
                predefnr1: i32;
                if (historical1 > -1)
                {
                  if (this.game.Data.HistoricalUnitObj[historical1].ModelMaster > -1)
                  {
                    let mut historicalSubPart: i32 = this.game.Data.UnitObj[index7].HistoricalSubPart;
                    predefnr1 = historicalSubPart <= -1 ? -1 : this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[historical1].ModelMaster].SubParts[historicalSubPart];
                  }
                  else
                  {
                    let mut historicalSubPart: i32 = this.game.Data.UnitObj[index7].HistoricalSubPart;
                    predefnr1 = historicalSubPart <= -1 ? this.game.Data.HistoricalUnitObj[historical1].SubParts[0] : this.game.Data.HistoricalUnitObj[historical1].SubParts[historicalSubPart];
                  }
                }
                else
                  predefnr1 = -1;
                let mut index21: i32 = this.game.HandyFunctionsObj.GetPreDef(predefnr1);
                if (index21 > -1)
                {
                  let mut sfCount2: i32 = this.game.Data.UnitObj[index21].SFCount;
                  for (let mut index22: i32 = 0; index22 <= sfCount2; index22 += 1)
                  {
                    let mut sf: i32 = this.game.Data.UnitObj[index21].SFList[index22];
                    let mut type: i32 = this.game.Data.SFObj[sf].Type;
                    index1 = this.game.Data.SFTypeObj[type].ReinforcementType;
                    let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                    let mut people: i32 = this.game.Data.SFObj[sf].People;
                    let mut nr: i32 = simpleList1.FindNr(index1, moveType, people);
                    let mut num9: i32 =  Math.Round(100.0 * ( this.game.HandyFunctionsObj.GetStaffPoints(index21) /  this.game.HandyFunctionsObj.GetStaffNeeded(index7)));
                    let mut num10: i32 = this.game.Data.SFObj[sf].Qty;
                    if (this.game.Data.UnitObj[index21].IsHQ && this.game.Data.SFTypeObj[type].StaffPts > 0 && this.game.HandyFunctionsObj.GetStaffPercent(index7) < 100)
                      num10 =  Math.Round( num10 * (100.0 /  num9));
                    if (nr > -1)
                    {
                      int[] weight = simpleList1.Weight;
                      int[] numArray8 = weight;
                      let mut index23: i32 = nr;
                      let mut index24: i32 = index23;
                      let mut num11: i32 = weight[index23] - num10 * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                      numArray8[index24] = num11;
                      if (simpleList1.Weight[nr] < 1)
                        simpleList1.RemoveNr(nr);
                    }
                  }
                }
                SimpleList simpleList2 = SimpleList::new();
                let mut unitCounter6: i32 = this.game.Data.UnitCounter;
                num12: i32;
                for (let mut index25: i32 = 0; index25 <= unitCounter6; index25 += 1)
                {
                  if (this.game.Data.UnitObj[index25].PreDef == -1 & this.game.Data.UnitObj[index25].Regime == this.game.Data.Turn & (!this.game.HandyFunctionsObj.CheckIsBattlegroup(index25) | this.game.Data.Product == 6) && this.game.Data.UnitObj[index25].HQ == index7 |  this.game.Data.RuleVar[887] == 1.0 & this.game.HandyFunctionsObj.IsUnitInHQChain(index25, index7))
                  {
                    let mut num13: i32 = 0;
                    if (this.game.Data.UnitObj[index25].Historical > -1 & this.game.Data.UnitObj[index25].HistoricalSubPart > -1 | this.game.Data.UnitObj[index25].IsHQ && this.game.Data.UnitObj[index25].X > -1)
                    {
                      if ( this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[index25].X, this.game.Data.UnitObj[index25].Y] <=  this.game.Data.RuleVar[339])
                        num13 = 1;
                      if (this.game.Data.UnitObj[index25].Historical > -1)
                      {
                        if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index25].Historical].ModelMaster == -1 & !this.game.Data.UnitObj[index25].IsHQ)
                        {
                          num13 = 0;
                          if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index25].Historical].SubParts[this.game.Data.UnitObj[index25].HistoricalSubPart] > -1)
                            num13 = 1;
                          if (this.game.Data.Product == 6 & this.game.HandyFunctionsObj.CheckIsBattlegroup(index25))
                            num13 = 1;
                        }
                        if (this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(index25) < 999 & num6 == 1)
                          num13 = 0;
                        if ( this.game.Data.RuleVar[455] > 0.0 && num13 == 1)
                        {
                          bool flag = true;
                          let mut num14: i32 = 0;
                          while (flag)
                          {
                            num14 += 1;
                            flag = false;
                            CoordList coordList2 = CoordList::new();
                            Coordinate coordinate;
                            coordinate.x = this.game.Data.UnitObj[index25].X;
                            coordinate.y = this.game.Data.UnitObj[index25].Y;
                            if (coordinate.x == 41 & coordinate.y == 27)
                              index1 = index1;
                            coordinate.onmap = true;
                            while (coordinate.onmap)
                            {
                              coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y];
                              if (coordinate.onmap)
                                coordList2.AddCoord(coordinate.x, coordinate.y, 0);
                            }
                            if (coordList2.counter > -1)
                            {
                              flag = false;
                              for (let mut counter: i32 = coordList2.counter; counter >= 0; counter += -1)
                              {
                                if (this.game.EditObj.TempValue3[index12].Value[coordList2.coord[counter].x, coordList2.coord[counter].y] != this.game.Data.Turn)
                                {
                                  this.game.Data.MapObj[0].HexObj[coordList2.coord[counter].x, coordList2.coord[counter].y].FuzzyBlock = 1;
                                  this.game.HandyFunctionsObj.MakeFuzzyOwner(false, false, this.game.Data.Turn);
                                  coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn,  Math.Round( this.game.Data.RuleVar[99]), 99,  Math.Round( this.game.Data.RuleVar[339]), this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                                  flag = true;
                                  break;
                                }
                              }
                            }
                            if (!flag &&  this.game.EditObj.TempValue[this.game.Data.UnitObj[index25].Map].Value[this.game.Data.UnitObj[index25].X, this.game.Data.UnitObj[index25].Y] >  this.game.Data.RuleVar[339])
                              num13 = 0;
                            if (num14 > 999)
                            {
                              flag = false;
                              num13 = 0;
                            }
                          }
                        }
                        if (num13 == 1)
                        {
                          this.AddPLog("Under command: -" + this.game.Data.UnitObj[index25].Name);
                          let mut breakPercent: i32 = this.game.HandyFunctionsObj.GetBreakPercent(index25);
                          let mut powerPtsAbsolute: i32 = this.game.HandyFunctionsObj.GetPowerPtsAbsolute(index25);
                          num12 =  Math.Round( this.game.Data.RuleVar[307]);
                          let mut startPower: i32 = this.game.HandyFunctionsObj.GetStartPower(index25);
                          index1 =  Math.Round( startPower * ( breakPercent / 100.0));
                          let mut tweight: i32 = startPower != 0 ? Math.Min(100,  Math.Round( powerPtsAbsolute /  startPower * 100.0)) : 100;
                          simpleList2.Add(index25, tweight, CheckExistence: false);
                        }
                      }
                    }
                  }
                }
                let mut num15: i32 = 1;
                let mut num16: i32 = 0;
                num17: i32;
                while (num15 == 1)
                {
                  num16 += 1;
                  num15 = 0;
                  let mut num18: i32 = 0;
                  simpleList2.Sort();
                  let mut counter1: i32 = simpleList2.Counter;
                  for (let mut index26: i32 = 0; index26 <= counter1; index26 += 1)
                  {
                    let mut index27: i32 = simpleList2.Id[index26];
                    if (index27 == 106)
                      index27 = index27;
                    if (index27 == 993)
                      index27 = index27;
                    if (Information.IsNothing( simpleListArray1[index27]))
                      simpleListArray1[index27] = SimpleList::new();
                    num17 = simpleList2.Weight[index26];
                    let mut num19: i32 =  this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[index27].SOReplacementPercent) : Math.Min(this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(index27), 100);
                    if (Operators.CompareString(this.game.Data.UnitObj[index27].Name, "Finnish Army HQ", false) == 0)
                      index27 = index27;
                    if (Operators.CompareString(this.game.Data.UnitObj[index27].Name, "11th Army", false) == 0)
                      index27 = index27;
                    let mut historical2: i32 = this.game.Data.UnitObj[index27].Historical;
                    if (historical2 > -1)
                    {
                      if (this.game.Data.HistoricalUnitObj[historical2].ModelMaster > -1)
                      {
                        let mut historicalSubPart: i32 = this.game.Data.UnitObj[index27].HistoricalSubPart;
                        index21 = historicalSubPart <= -1 ? -1 : this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[historical2].ModelMaster].SubParts[historicalSubPart];
                      }
                      else
                      {
                        let mut historicalSubPart: i32 = this.game.Data.UnitObj[index27].HistoricalSubPart;
                        index21 = historicalSubPart <= -1 ? this.game.Data.HistoricalUnitObj[historical2].SubParts[0] : this.game.Data.HistoricalUnitObj[historical2].SubParts[historicalSubPart];
                      }
                    }
                    else
                      index21 = -1;
                    SimpleList ideallist = SimpleList::new();
                    SimpleList simpleList3 = SimpleList::new();
                    SimpleList simpleList4 = SimpleList::new();
                    if (index21 > -1)
                    {
                      index21 = this.game.HandyFunctionsObj.GetPreDef(index21);
                      if (secondaryPhase == 0 & Strings.InStr(this.game.Data.UnitObj[index21].Name, "SS") > 0 & num15 == 0)
                        index27 = index27;
                      let mut sfCount3: i32 = this.game.Data.UnitObj[index21].SFCount;
                      for (let mut index28: i32 = 0; index28 <= sfCount3; index28 += 1)
                      {
                        let mut sf: i32 = this.game.Data.UnitObj[index21].SFList[index28];
                        let mut type: i32 = this.game.Data.SFObj[sf].Type;
                        let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
                        let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                        let mut people: i32 = this.game.Data.SFObj[sf].People;
                        let mut nr: i32 = ideallist.FindNr(reinforcementType, moveType, people);
                        let mut tdata5: i32 = 0;
                        let mut num20: i32 =  Math.Round(100.0 * ( this.game.HandyFunctionsObj.GetStaffPoints(index21) /  Math.Max(1, this.game.HandyFunctionsObj.GetStaffNeeded(index27))));
                        let mut num21: i32 = this.game.Data.SFObj[sf].Qty;
                        if (this.game.Data.UnitObj[index21].IsHQ)
                        {
                          if (this.game.Data.SFTypeObj[type].StaffPts > 0)
                          {
                            tdata5 = 1;
                            if (this.game.HandyFunctionsObj.GetStaffPercent(index27) < 100)
                            {
                              num21 =  Math.Round( num21 * (100.0 /  num20));
                              if (num21 < this.game.Data.SFObj[sf].Qty)
                                num21 = this.game.Data.SFObj[sf].Qty;
                            }
                          }
                        }
                        else if (!this.game.Data.UnitObj[index21].IsHQ | this.game.Data.SFTypeObj[type].StaffPts <= 0)
                        {
                          let mut num22: i32 =  this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[index27].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(index27));
                          num21 =  Math.Round( num21 * ( num22 / 100.0));
                        }
                        if (num21 > 0)
                        {
                          if (nr > -1)
                          {
                            int[] weight = ideallist.Weight;
                            int[] numArray9 = weight;
                            let mut index29: i32 = nr;
                            let mut index30: i32 = index29;
                            let mut num23: i32 = weight[index29] + num21 * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                            numArray9[index30] = num23;
                            int[] data3 = ideallist.Data3;
                            int[] numArray10 = data3;
                            let mut index31: i32 = nr;
                            let mut index32: i32 = index31;
                            let mut num24: i32 = data3[index31] + 1;
                            numArray10[index32] = num24;
                          }
                          else
                            ideallist.Add(reinforcementType, num21 * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts), moveType, people, 1, tdata5: tdata5, CheckExistence: false);
                        }
                      }
                      simpleList4 = SimpleList::new();
                      let mut counter2: i32 = ideallist.Counter;
                      for (let mut index33: i32 = 0; index33 <= counter2; index33 += 1)
                        simpleList4.Add(ideallist.Id[index33], ideallist.Weight[index33], ideallist.Data1[index33], ideallist.Data2[index33], tdata5: ideallist.Data5[index33], CheckExistence: false);
                      let mut sfCount4: i32 = this.game.Data.UnitObj[index27].SFCount;
                      for (let mut index34: i32 = 0; index34 <= sfCount4; index34 += 1)
                      {
                        let mut sf: i32 = this.game.Data.UnitObj[index27].SFList[index34];
                        let mut type: i32 = this.game.Data.SFObj[sf].Type;
                        let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
                        let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                        let mut people: i32 = this.game.Data.SFObj[sf].People;
                        let mut tdata5: i32 = 0;
                        if (this.game.Data.SFTypeObj[type].StaffPts > 0)
                          tdata5 = 1;
                        if (ideallist.FindNr(reinforcementType, moveType, people) > -1)
                        {
                          let mut nr: i32 = simpleList3.FindNr(reinforcementType, moveType, people);
                          let mut tweight: i32 = this.game.Data.SFObj[sf].Qty * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                          if (nr > -1)
                          {
                            int[] weight = simpleList3.Weight;
                            int[] numArray11 = weight;
                            let mut index35: i32 = nr;
                            let mut index36: i32 = index35;
                            let mut num25: i32 = weight[index35] + tweight;
                            numArray11[index36] = num25;
                            int[] data3 = simpleList3.Data3;
                            int[] numArray12 = data3;
                            let mut index37: i32 = nr;
                            let mut index38: i32 = index37;
                            let mut num26: i32 = data3[index37] + 1;
                            numArray12[index38] = num26;
                          }
                          else
                          {
                            simpleList3.Add(reinforcementType, tweight, moveType, people, 1, tdata5: tdata5, CheckExistence: false);
                            num12 = simpleList3.Counter;
                          }
                        }
                      }
                      let mut sfCount5: i32 = this.game.Data.UnitObj[index27].SFCount;
                      for (let mut index39: i32 = 0; index39 <= sfCount5; index39 += 1)
                      {
                        let mut sf: i32 = this.game.Data.UnitObj[index27].SFList[index39];
                        let mut type: i32 = this.game.Data.SFObj[sf].Type;
                        let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
                        let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                        let mut people: i32 = this.game.Data.SFObj[sf].People;
                        let mut tdata5: i32 = 0;
                        if (this.game.Data.SFTypeObj[type].StaffPts > 0)
                          tdata5 = 1;
                        if (this.game.Data.SFTypeObj[type].ReinforcementType2 > -1)
                        {
                          let mut nr1: i32 = ideallist.FindNr(reinforcementType, moveType, people);
                          let mut num27: i32 = nr1 <= -1 ? 0 : ideallist.Weight[nr1];
                          let mut nr2: i32 = simpleList3.FindNr(reinforcementType, moveType, people);
                          if (nr2 > -1)
                          {
                            let mut num28: i32 = simpleList3.Weight[nr2];
                            if (num28 > num27)
                            {
                              let mut tweight: i32 = Math.Min(this.game.Data.SFObj[sf].Qty * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts), num28 - num27);
                              let mut index40: i32 = nr2;
                              let mut reinforcementType2: i32 = this.game.Data.SFTypeObj[type].ReinforcementType2;
                              let mut num29: i32 = tweight;
                              if (this.game.Data.SFTypeObj[type].ReinforcementType > -1 & this.game.Data.SFTypeObj[type].ReinforcementType2 > -1 && this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2] > 0 & this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] > 0 && this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] != this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2])
                                tweight =  Math.Round(Conversion.Int( tweight * ( this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] /  this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2])));
                              let mut nr3: i32 = simpleList3.FindNr(reinforcementType2, moveType, people);
                              if (nr3 > -1)
                              {
                                int[] weight1 = simpleList3.Weight;
                                int[] numArray13 = weight1;
                                let mut index41: i32 = nr3;
                                let mut index42: i32 = index41;
                                let mut num30: i32 = weight1[index41] + tweight;
                                numArray13[index42] = num30;
                                int[] data3 = simpleList3.Data3;
                                int[] numArray14 = data3;
                                let mut index43: i32 = nr3;
                                let mut index44: i32 = index43;
                                let mut num31: i32 = data3[index43] + 1;
                                numArray14[index44] = num31;
                                int[] weight2 = simpleList3.Weight;
                                int[] numArray15 = weight2;
                                let mut index45: i32 = index40;
                                let mut index46: i32 = index45;
                                let mut num32: i32 = weight2[index45] - num29;
                                numArray15[index46] = num32;
                                if (simpleList3.Weight[index40] <= 0)
                                  simpleList3.RemoveNr(nr3);
                              }
                              else
                              {
                                simpleList3.Add(reinforcementType2, tweight, moveType, people, 1, tdata5: tdata5, CheckExistence: false);
                                let mut counter3: i32 = simpleList3.Counter;
                                int[] weight = simpleList3.Weight;
                                int[] numArray16 = weight;
                                let mut index47: i32 = index40;
                                let mut index48: i32 = index47;
                                let mut num33: i32 = weight[index47] - num29;
                                numArray16[index48] = num33;
                                if (simpleList3.Weight[index40] <= 0)
                                  simpleList3.RemoveNr(counter3);
                              }
                            }
                          }
                        }
                      }
                      for (let mut counter4: i32 = ideallist.Counter; counter4 >= 0; counter4 += -1)
                      {
                        if (simpleList3.FindNr(ideallist.Id[counter4], ideallist.Data1[counter4], ideallist.Data2[counter4]) == -1)
                          simpleList3.Add(ideallist.Id[counter4], 0, ideallist.Data1[counter4], ideallist.Data2[counter4], tdata5: ideallist.Data5[counter4], CheckExistence: false);
                      }
                      for (let mut counter5: i32 = simpleList3.Counter; counter5 >= 0; counter5 += -1)
                      {
                        let mut nr: i32 = ideallist.FindNr(simpleList3.Id[counter5], simpleList3.Data1[counter5], simpleList3.Data2[counter5]);
                        if (nr > -1)
                        {
                          let mut num34: i32 = ideallist.Weight[nr];
                          let mut num35: i32 = simpleList3.Weight[counter5];
                          if (num35 >=  Math.Round(Conversion.Int( num34 * ( num19 / 100.0))))
                            simpleList3.RemoveNr(counter5);
                          else
                            simpleList3.Weight[counter5] =  Math.Round(Conversion.Int( num34 * ( num19 / 100.0))) - num35;
                        }
                        else
                          simpleList3.RemoveNr(counter5);
                      }
                    }
                    SimpleList simpleList5 = SimpleList::new();
                    let mut counter6: i32 = simpleList3.Counter;
                    for (let mut index49: i32 = 0; index49 <= counter6; index49 += 1)
                      simpleList5.Add(simpleList3.Id[index49], simpleList3.Weight[index49], simpleList3.Data1[index49], simpleList3.Data2[index49], CheckExistence: false);
                    if ( this.game.Data.RuleVar[883] > 0.0)
                    {
                      let mut counter7: i32 = simpleList4.Counter;
                      for (let mut index50: i32 = 0; index50 <= counter7; index50 += 1)
                      {
                        let mut num36: i32 = simpleList4.Weight[index50];
                        if (index27 == 58 & simpleList4.Id[index50] == 0)
                          index27 = index27;
                        let mut num37: i32 = Math.Max(1,  Math.Round( ( num36 * (this.game.Data.RuleVar[883] / 100f))));
                        let mut logCounter: i32 = this.game.Data.UnitObj[index27].LogCounter;
                        for (let mut index51: i32 = 0; index51 <= logCounter; index51 += 1)
                        {
                          if (this.game.Data.UnitObj[index27].LogType[index51] == 2 && this.game.Data.UnitObj[index27].LogData1[index51] == simpleList4.Id[index50] && this.game.Data.UnitObj[index27].LogData2[index51] == simpleList4.Data2[index50])
                          {
                            num37 -= this.game.Data.UnitObj[index27].LogData3[index51];
                            if (0 > num37)
                              num37 = 0;
                          }
                        }
                        if (num37 == 3)
                          num37 = num37;
                        let mut counter8: i32 = simpleList3.Counter;
                        for (let mut index52: i32 = 0; index52 <= counter8; index52 += 1)
                        {
                          if (simpleList3.Id[index52] == simpleList4.Id[index50] & simpleList3.Data1[index52] == simpleList4.Data1[index50] & simpleList3.Data2[index52] == simpleList4.Data2[index50] && num37 < simpleList3.Weight[index52])
                            simpleList3.Weight[index52] = num37;
                        }
                      }
                    }
                    if (this.game.Data.UnitObj[index27].IsHQ &  this.game.Data.RuleVar[887] < 1.0 && !Information.IsNothing( simpleListArray3[index27]))
                    {
                      let mut counter9: i32 = simpleListArray3[index27].Counter;
                      for (let mut index53: i32 = 0; index53 <= counter9; index53 += 1)
                      {
                        let mut nr4: i32 = simpleList3.FindNr(simpleListArray3[index27].Id[index53], simpleListArray3[index27].Data1[index53], simpleListArray3[index27].Data2[index53]);
                        if (nr4 == -1)
                        {
                          simpleList3.Add(simpleListArray3[index27].Id[index53],  Math.Round(Conversion.Int( simpleListArray3[index27].Weight[index53] * ( num19 / 100.0))), simpleListArray3[index27].Data1[index53], simpleListArray3[index27].Data2[index53], CheckExistence: false);
                        }
                        else
                        {
                          int[] weight = simpleList3.Weight;
                          int[] numArray17 = weight;
                          let mut index54: i32 = nr4;
                          let mut index55: i32 = index54;
                          let mut num38: i32 =  Math.Round( weight[index54] + Conversion.Int( simpleListArray3[index27].Weight[index53] * ( num19 / 100.0)));
                          numArray17[index55] = num38;
                        }
                        let mut nr5: i32 = ideallist.FindNr(simpleListArray3[index27].Id[index53], simpleListArray3[index27].Data1[index53], simpleListArray3[index27].Data2[index53]);
                        if (nr5 == -1)
                        {
                          ideallist.Add(simpleListArray3[index27].Id[index53],  Math.Round(Conversion.Int( simpleListArray3[index27].Weight[index53] * ( num19 / 100.0))), simpleListArray3[index27].Data1[index53], simpleListArray3[index27].Data2[index53], CheckExistence: false);
                        }
                        else
                        {
                          int[] weight = ideallist.Weight;
                          int[] numArray18 = weight;
                          let mut index56: i32 = nr5;
                          let mut index57: i32 = index56;
                          let mut num39: i32 =  Math.Round( weight[index56] + Conversion.Int( simpleListArray3[index27].Weight[index53] * ( num19 / 100.0)));
                          numArray18[index57] = num39;
                        }
                        let mut nr6: i32 = simpleList5.FindNr(simpleListArray3[index27].Id[index53], simpleListArray3[index27].Data1[index53], simpleListArray3[index27].Data2[index53]);
                        if (nr6 == -1)
                        {
                          simpleList5.Add(simpleListArray3[index27].Id[index53],  Math.Round(Conversion.Int( simpleListArray3[index27].Weight[index53] * ( num19 / 100.0))), simpleListArray3[index27].Data1[index53], simpleListArray3[index27].Data2[index53], CheckExistence: false);
                        }
                        else
                        {
                          int[] weight = simpleList5.Weight;
                          int[] numArray19 = weight;
                          let mut index58: i32 = nr6;
                          let mut index59: i32 = index58;
                          let mut num40: i32 =  Math.Round( weight[index58] + Conversion.Int( simpleListArray3[index27].Weight[index53] * ( num19 / 100.0)));
                          numArray19[index59] = num40;
                        }
                      }
                    }
                    if (index7 == 54 & index27 == 993)
                      index7 = index7;
                    if ( this.game.Data.RuleVar[887] < 1.0)
                    {
                      for (let mut counter10: i32 = simpleListArray1[index27].Counter; counter10 >= 0; counter10 += -1)
                      {
                        let mut nr7: i32 = simpleList3.FindNr(simpleListArray1[index27].Id[counter10], simpleListArray1[index27].Data1[counter10], simpleListArray1[index27].Data2[counter10]);
                        if (nr7 > -1)
                        {
                          int[] weight = simpleList3.Weight;
                          int[] numArray20 = weight;
                          let mut index60: i32 = nr7;
                          let mut index61: i32 = index60;
                          let mut num41: i32 = weight[index60] - simpleListArray1[index27].Weight[counter10];
                          numArray20[index61] = num41;
                          if (1 > simpleList3.Weight[nr7])
                            simpleList3.RemoveNr(nr7);
                        }
                        let mut nr8: i32 = simpleList5.FindNr(simpleListArray1[index27].Id[counter10], simpleListArray1[index27].Data1[counter10], simpleListArray1[index27].Data2[counter10]);
                        if (nr8 > -1)
                        {
                          int[] weight = simpleList5.Weight;
                          int[] numArray21 = weight;
                          let mut index62: i32 = nr8;
                          let mut index63: i32 = index62;
                          let mut num42: i32 = weight[index62] - simpleListArray1[index27].Weight[counter10];
                          numArray21[index63] = num42;
                          if (1 > simpleList5.Weight[nr8])
                            simpleList5.RemoveNr(nr8);
                        }
                      }
                    }
                    simpleListArray2[index27] = SimpleList::new();
                    let mut counter11: i32 = simpleList5.Counter;
                    for (let mut index64: i32 = 0; index64 <= counter11; index64 += 1)
                      simpleListArray2[index27].Add(simpleList5.Id[index64], simpleList5.Weight[index64], simpleList5.Data1[index64], simpleList5.Data2[index64], CheckExistence: false);
                    if (index27 == 256)
                      index27 = index27;
                    if (num16 == 1 & secondaryPhase == 0 & (num6 == 1 | num6 == 2 & this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(index27) < 999))
                    {
                      let mut counter12: i32 = simpleList3.Counter;
                      for (let mut index65: i32 = 0; index65 <= counter12; index65 += 1)
                      {
                        if (index27 == 224)
                          index65 = index65;
                        this.game.Data.UnitObj[index27].AddLog(1, simpleList3.Id[index65], simpleList3.Data2[index65], simpleList3.Weight[index65]);
                        this.game.Data.UnitObj[index7].AddLog(5, simpleList3.Id[index65], simpleList3.Data2[index65], simpleList3.Weight[index65]);
                      }
                      let mut counter13: i32 = simpleList5.Counter;
                      for (let mut index66: i32 = 0; index66 <= counter13; index66 += 1)
                        this.game.Data.UnitObj[index27].AddLog(9, simpleList5.Id[index66], simpleList5.Data2[index66], simpleList5.Weight[index66]);
                    }
                    for (let mut counter14: i32 = simpleList3.Counter; counter14 >= 0; counter14 += -1)
                    {
                      let mut tid: i32 = simpleList3.Id[counter14];
                      let mut tdata1: i32 = simpleList3.Data1[counter14];
                      let mut tdata2: i32 = simpleList3.Data2[counter14];
                      if (simpleList1.FindNr(tid, tdata1, tdata2) == -1)
                        simpleList3.RemoveNr(counter14);
                    }
                    for (index1 = simpleList3.Counter; index1 >= 0; index1 += -1)
                    {
                      if (simpleList3.Weight[index1] < 1)
                        simpleList3.RemoveNr(index1);
                    }
                    simpleList3.Sort();
                    if (num15 == 0)
                    {
                      for (let mut counter15: i32 = simpleList3.Counter; counter15 >= 0; counter15 += -1)
                      {
                        if (index27 == 859)
                          index27 = index27;
                        index1 = this.AutoReinforce_HQwillGive(index7, index27, ideallist, simpleList3.Id[counter15], simpleList3.Data1[counter15], simpleList3.Data2[counter15], secondaryPhase);
                        if (index27 == 993)
                          index27 = index27;
                        if (index1 > -1)
                        {
                          let mut theater: i32 = -2;
                          if ( this.game.Data.RuleVar[340] == -1.0)
                            theater = -1;
                          if (index1 > -1 &&  this.game.Data.RuleVar[340] > -1.0)
                          {
                            if (( this.game.Data.RuleVar[340] == 2.0 |  this.game.Data.RuleVar[340] == 3.0) & this.game.Data.UnitObj[index7].AirCap >= mapMatrix2Array2[this.game.Data.UnitObj[index27].Map].Value[this.game.Data.UnitObj[index27].X, this.game.Data.UnitObj[index27].Y] * this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Weight)
                            {
                              theater = 2;
                              this.game.EditObj.TempValue = mapMatrix2Array2;
                            }
                            else if (( this.game.Data.RuleVar[340] == 1.0 |  this.game.Data.RuleVar[340] == 3.0) & this.game.Data.UnitObj[index7].NavyCap >= mapMatrix2Array1[this.game.Data.UnitObj[index27].Map].Value[this.game.Data.UnitObj[index27].X, this.game.Data.UnitObj[index27].Y] * this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Weight)
                            {
                              theater = 1;
                              this.game.EditObj.TempValue = mapMatrix2Array1;
                            }
                            else if (( this.game.Data.RuleVar[340] == 0.0 |  this.game.Data.RuleVar[340] == 3.0) & this.game.Data.UnitObj[index7].LandCap >= mapMatrix2Array3[this.game.Data.UnitObj[index27].Map].Value[this.game.Data.UnitObj[index27].X, this.game.Data.UnitObj[index27].Y] * this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Weight)
                            {
                              theater = 0;
                              this.game.EditObj.TempValue = mapMatrix2Array3;
                            }
                            this.game.EditObj.TempValue2 = (MapMatrix2[]) this.game.EditObj.TempValue.Clone();
                            this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].MoveType, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Theater, this.game.Data.SFObj[index1].Ap, this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map);
                            if (this.game.EditObj.TempValue[this.game.Data.UnitObj[index27].Map].Value[this.game.Data.UnitObj[index27].X, this.game.Data.UnitObj[index27].Y] <= this.game.Data.SFObj[index1].Ap)
                              theater = -1;
                            this.game.EditObj.TempValue = this.game.EditObj.TempValue2;
                          }
                          num43: i32;
                          if (theater == -2 | index1 == -2)
                          {
                            theater = -2;
                            let mut nr9: i32 = simpleList1.FindNr(simpleList3.Id[counter15], simpleList3.Data1[counter15], simpleList3.Data2[counter15]);
                            if (nr9 > -1)
                            {
                              num43 = simpleList3.Weight[counter15];
                              if (simpleList1.Weight[nr9] < num43)
                                num43 = simpleList1.Weight[nr9];
                              int[] weight3 = simpleList1.Weight;
                              int[] numArray22 = weight3;
                              let mut index67: i32 = nr9;
                              let mut index68: i32 = index67;
                              let mut num44: i32 = weight3[index67] - simpleList3.Weight[counter15];
                              numArray22[index68] = num44;
                              if (1 > simpleList1.Weight[nr9])
                                simpleList1.RemoveNr(nr9);
                              let mut nr10: i32 = simpleListArray2[index27].FindNr(simpleList3.Id[counter15], simpleList3.Data1[counter15], simpleList3.Data2[counter15]);
                              if (nr10 > -1)
                              {
                                int[] weight4 = simpleListArray2[index27].Weight;
                                int[] numArray23 = weight4;
                                let mut index69: i32 = nr10;
                                let mut index70: i32 = index69;
                                let mut num45: i32 = weight4[index69] - num43;
                                numArray23[index70] = num45;
                                if (1 > simpleListArray2[index27].Weight[nr10])
                                  simpleListArray2[index27].RemoveNr(nr10);
                                this.AddPLog(this.game.Data.UnitObj[index27].Name + " 's HQ has " + Conversion.Str( num43) + " power pts of " + this.game.Data.ReinfName[simpleList3.Id[counter15]] + ", but not the CAP to send! / or target slots full");
                                simpleListArray1[index27].Add(simpleList3.Id[counter15], num43, simpleList3.Data1[counter15], simpleList3.Data2[counter15], CheckExistence: false);
                              }
                              num15 = 1;
                            }
                          }
                          if (theater > -2)
                          {
                            if (index27 == 54)
                              index27 = index27;
                            let mut nr11: i32 = secondaryPhase != 0 ? simpleListArray2[index27].FindNr(this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].ReinforcementType2, this.game.Data.SFObj[index1].MoveType, this.game.Data.SFObj[index1].People) : simpleListArray2[index27].FindNr(this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].ReinforcementType, this.game.Data.SFObj[index1].MoveType, this.game.Data.SFObj[index1].People);
                            if (nr11 > -1)
                            {
                              int[] weight = simpleListArray2[index27].Weight;
                              int[] numArray24 = weight;
                              let mut index71: i32 = nr11;
                              let mut index72: i32 = index71;
                              let mut num46: i32 = weight[index71] - Math.Min(val1_1, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].PowerPts);
                              numArray24[index72] = num46;
                              if (simpleListArray2[index27].Weight[nr11] <= 0)
                                simpleListArray2[index27].RemoveNr(nr11);
                            }
                            if (this.game.Data.UnitObj[index27].IsHQ && !Information.IsNothing( simpleListArray3[index27]))
                            {
                              let mut nr12: i32 = secondaryPhase != 0 ? simpleListArray3[index27].FindNr(this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].ReinforcementType2, this.game.Data.SFObj[index1].MoveType, this.game.Data.SFObj[index1].People) : simpleListArray3[index27].FindNr(this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].ReinforcementType, this.game.Data.SFObj[index1].MoveType, this.game.Data.SFObj[index1].People);
                              if (nr12 > -1)
                              {
                                int[] weight = simpleListArray3[index27].Weight;
                                int[] numArray25 = weight;
                                let mut index73: i32 = nr12;
                                let mut index74: i32 = index73;
                                let mut num47: i32 = weight[index73] - Math.Min(val1_1, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].PowerPts);
                                numArray25[index74] = num47;
                                if (simpleListArray3[index27].Weight[nr12] <= 0)
                                  simpleListArray3[index27].RemoveNr(nr12);
                              }
                            }
                            this.AddPLog("transfer 1 " + this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Name + " to => " + this.game.Data.UnitObj[index27].Name + ", landcap=" + Conversion.Str( this.game.Data.UnitObj[index7].LandCap) + ", railcap=" + Conversion.Str( this.game.Data.UnitObj[index7].AirCap) + ", seacap=" + Conversion.Str( this.game.Data.UnitObj[index7].NavyCap));
                            if (!Information.IsNothing( simpleListArray3[index7]))
                            {
                              let mut counter16: i32 = simpleListArray3[index7].Counter;
                              for (let mut index75: i32 = 0; index75 <= counter16; index75 += 1)
                                this.AddPLog("HQRemListHQ(" + this.game.Data.UnitObj[index7].Name + ") " + this.game.Data.ReinfName[simpleListArray3[index7].Id[index75]] + " (mvtyp=" + simpleListArray3[index7].Data1[index75].ToString() + ",ppl=" + simpleListArray3[index7].Data2[index75].ToString() + ") = " + simpleListArray3[index7].Weight[index75].ToString());
                            }
                            let mut type: i32 = this.game.Data.SFObj[index1].Type;
                            if (theater == -1)
                              this.DoTransfer(index7, index27, 0, index1, 1, true, false, MoveMatrixDone: true);
                            else
                              this.DoTransfer(index7, index27, theater, index1, 1, AddtoHistory: false, MoveMatrixDone: true);
                            if (index27 == 58 & type == 0)
                              index27 = index27;
                            let mut nr13: i32 = simpleList1.FindNr(simpleList3.Id[counter15], simpleList3.Data1[counter15], simpleList3.Data2[counter15]);
                            if (nr13 > -1)
                            {
                              if (secondaryPhase == 0)
                              {
                                this.game.Data.UnitObj[index7].AddLog(6, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                                if (this.game.EditObj.TransferLostQty < 1)
                                  this.game.Data.UnitObj[index27].AddLog(2, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                                else
                                  this.game.Data.UnitObj[index7].AddLog(7, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                              }
                              else if (this.game.Data.Product == 6)
                              {
                                this.game.Data.UnitObj[index7].AddLog(6, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                                if (this.game.EditObj.TransferLostQty < 1)
                                  this.game.Data.UnitObj[index27].AddLog(2, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                                else
                                  this.game.Data.UnitObj[index7].AddLog(7, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                              }
                              else
                              {
                                this.game.Data.UnitObj[index7].AddLog(6, this.game.Data.SFTypeObj[type].ReinforcementType2, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                                if (this.game.EditObj.TransferLostQty < 1)
                                  this.game.Data.UnitObj[index27].AddLog(2, this.game.Data.SFTypeObj[type].ReinforcementType2, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                                else
                                  this.game.Data.UnitObj[index7].AddLog(7, this.game.Data.SFTypeObj[type].ReinforcementType2, simpleList3.Data2[counter15], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                              }
                              int[] weight = simpleList1.Weight;
                              int[] numArray26 = weight;
                              let mut index76: i32 = nr13;
                              let mut index77: i32 = index76;
                              let mut num48: i32 = weight[index76] - Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                              numArray26[index77] = num48;
                              if (simpleList1.Weight[nr13] < 1)
                                simpleList1.RemoveNr(nr13);
                            }
                            num15 = 1;
                            let mut breakPercent: i32 = this.game.HandyFunctionsObj.GetBreakPercent(index27);
                            let mut powerPtsAbsolute: i32 = this.game.HandyFunctionsObj.GetPowerPtsAbsolute(index27);
                            let mut num49: i32 =  Math.Round( this.game.Data.RuleVar[307]);
                            let mut startPower: i32 = this.game.HandyFunctionsObj.GetStartPower(index27);
                            index1 =  Math.Round( startPower * ( breakPercent / 100.0));
                            let mut num50: i32 = startPower != 0 ? Math.Min(100,  Math.Round( powerPtsAbsolute /  startPower * 100.0)) : 100;
                            simpleList2.Weight[index26] = num50;
                            if (useTrafficRules)
                            {
                              Coordinate coordinate1;
                              coordinate1.x = this.game.Data.UnitObj[index27].X;
                              coordinate1.y = this.game.Data.UnitObj[index27].Y;
                              bool flag = false;
                              let mut index78: i32 = 0;
                              while (this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y].onmap)
                              {
                                let mut num51: i32 = index78 + 1;
                                Coordinate coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                                let mut index79: i32 = this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
                                let mut index80: i32 = index79 + 3;
                                if (index80 > 5)
                                  index80 -= 6;
                                let mut liSpoint1: i32 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index79];
                                let mut liSpoint2: i32 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index80];
                                int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints;
                                int[] numArray27 = liSpoints1;
                                let mut index81: i32 = index79;
                                let mut index82: i32 = index81;
                                let mut num52: i32 = liSpoints1[index81] +  Math.Round(Math.Ceiling( (this.game.Data.SFTypeObj[type].Weight * 1) /  this.game.Data.RuleVar[33]));
                                numArray27[index82] = num52;
                                int[] liSpoints2 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints;
                                int[] numArray28 = liSpoints2;
                                let mut index83: i32 = index80;
                                let mut index84: i32 = index83;
                                let mut num53: i32 = liSpoints2[index83] +  Math.Round(Math.Ceiling( (this.game.Data.SFTypeObj[type].Weight * 1) /  this.game.Data.RuleVar[33]));
                                numArray28[index84] = num53;
                                let mut index85: i32 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RoadType[index79];
                                if (index85 > -1 && this.game.Data.RoadTypeObj[index85].trafficPoints > 0)
                                {
                                  let mut num54: i32 =  Math.Round(Math.Floor( (liSpoint1 * 1) /  this.game.Data.RoadTypeObj[index85].trafficPoints));
                                  if ( Math.Round(Math.Floor( (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index79] * 1) /  this.game.Data.RoadTypeObj[index85].trafficPoints)) > num54)
                                    flag = true;
                                }
                                index78 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].RoadType[index80];
                                if (index78 > -1 && this.game.Data.RoadTypeObj[index78].trafficPoints > 0)
                                {
                                  let mut num55: i32 =  Math.Round(Math.Floor( (num49 * 1) /  this.game.Data.RoadTypeObj[index78].trafficPoints));
                                  if ( Math.Round(Math.Floor( (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index80] * 1) /  this.game.Data.RoadTypeObj[index78].trafficPoints)) > num55)
                                    flag = true;
                                }
                                coordinate1 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                                if (index78 > 999)
                                  break;
                              }
                              if (flag)
                              {
                                coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, num43, 99,  Math.Round( this.game.Data.RuleVar[3]), this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                                break;
                              }
                              break;
                            }
                            break;
                          }
                        }
                        else
                          index1 = index1;
                      }
                    }
                    if (num15 == 1 & num18 == 1)
                      break;
                  }
                  if (num15 == 0 & secondaryPhase == 1 & num6 == 2)
                  {
                    if (index7 == 99)
                      index21 = index21;
                    simpleListArray3[index7] = SimpleList::new();
                    let mut counter17: i32 = simpleList2.Counter;
                    for (let mut index86: i32 = 0; index86 <= counter17; index86 += 1)
                    {
                      let mut index87: i32 = simpleList2.Id[index86];
                      if (index7 == 54 & index87 == 993)
                        index7 = index7;
                      if (index87 == 993)
                        index87 = index87;
                      if (this.game.Data.UnitObj[index87].HQ == index7)
                      {
                        for (index1 = simpleListArray2[index87].Counter; index1 >= 0; index1 += -1)
                        {
                          let mut num56: i32 =  this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[index7].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(index7));
                          simpleListArray2[index87].Weight[index1] =  Math.Round(Conversion.Int( simpleListArray2[index87].Weight[index1] * ( num56 / 100.0)));
                          if (simpleListArray2[index87].Weight[index1] > 0 & simpleListArray2[index87].Id[index1] > -1)
                          {
                            this.AddPLog(this.game.Data.UnitObj[index87].Name + " could not get " + Conversion.Str( simpleListArray2[index87].Weight[index1]) + " power pts of " + this.game.Data.ReinfName[simpleListArray2[index87].Id[index1]] + " (" + Conversion.Str( simpleListArray2[index87].Data1[index1]) + "," + Conversion.Str( simpleListArray2[index87].Data2[index1]) + ") passed on to " + this.game.Data.UnitObj[index7].Name);
                            let mut nr: i32 = simpleListArray3[index7].FindNr(simpleListArray2[index87].Id[index1], simpleListArray2[index87].Data1[index1], simpleListArray2[index87].Data2[index1]);
                            if (simpleListArray2[index87].Id[index1] == 0)
                              index1 = index1;
                            if (nr == -1)
                            {
                              simpleListArray3[index7].Add(simpleListArray2[index87].Id[index1], simpleListArray2[index87].Weight[index1], simpleListArray2[index87].Data1[index1], simpleListArray2[index87].Data2[index1], CheckExistence: false);
                            }
                            else
                            {
                              int[] weight = simpleListArray3[index7].Weight;
                              int[] numArray29 = weight;
                              let mut index88: i32 = nr;
                              let mut index89: i32 = index88;
                              let mut num57: i32 = weight[index88] + simpleListArray2[index87].Weight[index1];
                              numArray29[index89] = num57;
                            }
                            if (!Information.IsNothing( simpleListArray3[index7]))
                            {
                              let mut counter18: i32 = simpleListArray3[index7].Counter;
                              for (let mut index90: i32 = 0; index90 <= counter18; index90 += 1)
                                this.AddPLog("HQRemListHQ(" + this.game.Data.UnitObj[index7].Name + ") " + this.game.Data.ReinfName[simpleListArray3[index7].Id[index90]] + " (mvtyp=" + simpleListArray3[index7].Data1[index90].ToString() + ",ppl=" + simpleListArray3[index7].Data2[index90].ToString() + ") = " + simpleListArray3[index7].Weight[index90].ToString());
                            }
                          }
                          else
                            simpleListArray2[index87].RemoveNr(index1);
                        }
                      }
                    }
                    simpleListArray4[index7] = SimpleList::new();
                    if (!Information.IsNothing( simpleListArray3[index7]))
                    {
                      if (index7 == 54)
                        index7 = index7;
                      let mut counter19: i32 = simpleListArray3[index7].Counter;
                      for (index1 = 0; index1 <= counter19; index1 += 1)
                        simpleListArray4[index7].Add(simpleListArray3[index7].Id[index1], simpleListArray3[index7].Weight[index1], simpleListArray3[index7].Data1[index1], simpleListArray3[index7].Data2[index1], simpleListArray3[index7].Data3[index1], CheckExistence: false);
                    }
                    let mut counter20: i32 = simpleList1.Counter;
                    for (let mut index91: i32 = 0; index91 <= counter20; index91 += 1)
                    {
                      if (simpleList1.Id[index91] > -1)
                      {
                        index1 = simpleListArray3[index7].FindNr(simpleList1.Id[index91], simpleList1.Data1[index91], simpleList1.Data2[index91]);
                        if (index1 != -1)
                        {
                          int[] weight = simpleListArray3[index7].Weight;
                          int[] numArray30 = weight;
                          let mut index92: i32 = index1;
                          let mut index93: i32 = index92;
                          let mut num58: i32 = weight[index92] - simpleList1.Weight[index91];
                          numArray30[index93] = num58;
                          if (simpleListArray3[index7].Weight[index1] < 1)
                            simpleListArray3[index7].RemoveNr(index1);
                        }
                      }
                    }
                    if (index7 == 54)
                      index7 = index7;
                  }
                }
                if (secondaryPhase == 1 & num6 == 2)
                {
                  this.AddPLog("RETURN CHECKING FOR " + this.game.Data.UnitObj[index7].Name);
                  let mut num59: i32 = 1;
                  let mut num60: i32 = 0;
                  while (num59 == 1)
                  {
                    num59 = 0;
                    num60 += 1;
                    let mut counter21: i32 = simpleList2.Counter;
                    for (let mut index94: i32 = 0; index94 <= counter21; index94 += 1)
                    {
                      let mut unr: i32 = simpleList2.Id[index94];
                      let mut replacementPercent: i32 = this.game.Data.UnitObj[unr].SOReplacementPercent;
                      num17 = simpleList2.Weight[index94];
                      let mut num61: i32 = simpleList2.Data1[index94];
                      if (num60 == 1)
                        simpleListArray5[unr] = SimpleList::new();
                      bool flag1 = false;
                      if (this.game.HandyFunctionsObj.CheckIsBattlegroup(unr) & this.game.Data.Product == 6)
                        flag1 = true;
                      let mut historical3: i32 = this.game.Data.UnitObj[unr].Historical;
                      predefnr2: i32;
                      if (historical3 > -1)
                      {
                        if (this.game.Data.HistoricalUnitObj[historical3].ModelMaster > -1)
                        {
                          let mut historicalSubPart: i32 = this.game.Data.UnitObj[unr].HistoricalSubPart;
                          predefnr2 = historicalSubPart <= -1 ? -1 : this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[historical3].ModelMaster].SubParts[historicalSubPart];
                        }
                        else
                        {
                          let mut historicalSubPart: i32 = this.game.Data.UnitObj[unr].HistoricalSubPart;
                          predefnr2 = this.game.Data.HistoricalUnitObj[historical3].SubParts[historicalSubPart];
                        }
                      }
                      else
                        predefnr2 = -1;
                      SimpleList simpleList6 = SimpleList::new();
                      SimpleList simpleList7 = SimpleList::new();
                      SimpleList simpleList8 = SimpleList::new();
                      if (Operators.CompareString(this.game.Data.UnitObj[unr].Name, "Finnish Army HQ", false) == 0)
                        unr = unr;
                      if (Operators.CompareString(this.game.Data.UnitObj[unr].Name, "KG Hoffman", false) == 0)
                        unr = unr;
                      bool flag2 = true;
                      if (predefnr2 > -1)
                      {
                        let mut preDef: i32 = this.game.HandyFunctionsObj.GetPreDef(predefnr2);
                        let mut sfCount6: i32 = this.game.Data.UnitObj[preDef].SFCount;
                        for (let mut index95: i32 = 0; index95 <= sfCount6; index95 += 1)
                        {
                          let mut sf: i32 = this.game.Data.UnitObj[preDef].SFList[index95];
                          let mut type: i32 = this.game.Data.SFObj[sf].Type;
                          index1 = this.game.Data.SFTypeObj[type].ReinforcementType;
                          let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                          let mut people: i32 = this.game.Data.SFObj[sf].People;
                          let mut nr14: i32 = simpleList6.FindNr(index1, moveType, people);
                          let mut nr15: i32 = simpleList8.FindNr(index1, moveType, people);
                          let mut num62: i32 = this.game.Data.SFObj[sf].Qty;
                          let mut num63: i32 = this.game.Data.SFObj[sf].Qty;
                          if (this.game.Data.UnitObj[unr].IsHQ)
                          {
                            if (this.game.Data.SFTypeObj[type].StaffPts > 0)
                            {
                              let mut num64: i32 = this.game.HandyFunctionsObj.GetStaffNeeded(unr) > 0 ?  Math.Round(100.0 * ( this.game.HandyFunctionsObj.GetStaffPoints(preDef) /  this.game.HandyFunctionsObj.GetStaffNeeded(unr))) : 100;
                              if (num64 <= 100)
                              {
                                num62 =  Math.Round( num62 * (100.0 /  num64)) + 1;
                                num63 =  Math.Round( num63 * (100.0 /  num64)) + 1;
                                if (( this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[unr].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(unr))) == 0)
                                  num62 = 0;
                              }
                              else if (( this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[unr].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(unr))) == 0)
                                num62 = 0;
                            }
                            else if (( this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[unr].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(unr))) == 0)
                              num62 = 0;
                          }
                          else if (!this.game.Data.UnitObj[unr].IsHQ | this.game.Data.SFTypeObj[type].StaffPts <= 0 && ( this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[unr].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(unr))) == 0)
                            num62 = 0;
                          if (num62 > 0)
                          {
                            if (this.game.Data.SFTypeObj[type].StaffPts > 0)
                              num62 += Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                            if (nr14 > -1)
                            {
                              int[] weight = simpleList6.Weight;
                              int[] numArray31 = weight;
                              let mut index96: i32 = nr14;
                              let mut index97: i32 = index96;
                              let mut num65: i32 = weight[index96] + num62 * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                              numArray31[index97] = num65;
                              int[] data3 = simpleList6.Data3;
                              int[] numArray32 = data3;
                              let mut index98: i32 = nr14;
                              let mut index99: i32 = index98;
                              let mut num66: i32 = data3[index98] + 1;
                              numArray32[index99] = num66;
                            }
                            else
                              simpleList6.Add(index1, num62 * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts), moveType, people, 1, CheckExistence: false);
                          }
                          if (num63 > 0)
                          {
                            if (this.game.Data.SFTypeObj[type].StaffPts > 0)
                              num63 += Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                            if (nr15 > -1)
                            {
                              int[] weight = simpleList8.Weight;
                              int[] numArray33 = weight;
                              let mut index100: i32 = nr15;
                              let mut index101: i32 = index100;
                              let mut num67: i32 = weight[index100] + num63 * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                              numArray33[index101] = num67;
                              int[] data3 = simpleList8.Data3;
                              int[] numArray34 = data3;
                              let mut index102: i32 = nr15;
                              let mut index103: i32 = index102;
                              let mut num68: i32 = data3[index102] + 1;
                              numArray34[index103] = num68;
                            }
                            else
                              simpleList8.Add(index1, num63 * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts), moveType, people, 1, CheckExistence: false);
                          }
                        }
                      }
                      else
                      {
                        let mut num69: i32 = 1;
                        if (( this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[unr].SOReplacementPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedReplacementRequest(unr))) == 0)
                          num69 = 0;
                        if (num69 > 0)
                          flag2 = false;
                      }
                      if (flag2)
                      {
                        let mut sfCount7: i32 = this.game.Data.UnitObj[unr].SFCount;
                        for (let mut index104: i32 = 0; index104 <= sfCount7; index104 += 1)
                        {
                          let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index104];
                          let mut type: i32 = this.game.Data.SFObj[sf].Type;
                          index1 = this.game.Data.SFTypeObj[type].ReinforcementType;
                          let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                          let mut people: i32 = this.game.Data.SFObj[sf].People;
                          let mut nr: i32 = simpleList7.FindNr(index1, moveType, people);
                          if (!(this.game.Data.SFTypeObj[type].DontReturnFromHQ & this.game.Data.UnitObj[unr].IsHQ & this.game.Data.UnitObj[unr].SOReplacementPercent > 0) && this.game.Data.SFTypeObj[type].Theater != 1)
                          {
                            if (nr > -1)
                            {
                              int[] weight = simpleList7.Weight;
                              int[] numArray35 = weight;
                              let mut index105: i32 = nr;
                              let mut index106: i32 = index105;
                              let mut num70: i32 = weight[index105] + this.game.Data.SFObj[sf].Qty * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts);
                              numArray35[index106] = num70;
                            }
                            else
                              simpleList7.Add(index1, this.game.Data.SFObj[sf].Qty * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts), moveType, people, 1, CheckExistence: false);
                          }
                        }
                        let mut sfCount8: i32 = this.game.Data.UnitObj[unr].SFCount;
                        for (let mut index107: i32 = 0; index107 <= sfCount8; index107 += 1)
                        {
                          let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index107];
                          let mut type: i32 = this.game.Data.SFObj[sf].Type;
                          index1 = this.game.Data.SFTypeObj[type].ReinforcementType;
                          let mut moveType: i32 = this.game.Data.SFObj[sf].MoveType;
                          let mut people: i32 = this.game.Data.SFObj[sf].People;
                          if (this.game.Data.SFTypeObj[type].ReinforcementType2 > -1)
                          {
                            let mut nr16: i32 = simpleList6.FindNr(index1, moveType, people);
                            let mut num71: i32 = nr16 <= -1 ? 0 : simpleList6.Weight[nr16];
                            let mut nr17: i32 = simpleList7.FindNr(index1, moveType, people);
                            if (nr17 > -1)
                            {
                              let mut num72: i32 = simpleList7.Weight[nr17];
                              if (num72 > num71)
                              {
                                let mut val1_2: i32 = Math.Min(this.game.Data.SFObj[sf].Qty * Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts), num72 - num71);
                                let mut nr18: i32 = nr17;
                                index1 = this.game.Data.SFTypeObj[type].ReinforcementType2;
                                let mut nr19: i32 = simpleList7.FindNr(index1, moveType, people);
                                let mut nr20: i32 = simpleList6.FindNr(index1, moveType, people);
                                if (nr20 > -1)
                                {
                                  let mut num73: i32 = simpleList6.Weight[nr20];
                                  if (nr19 > -1)
                                  {
                                    let mut num74: i32 = simpleList7.Weight[nr19];
                                    let mut val1_3: i32 = Math.Min(val1_2, num73 - num74);
                                    let mut num75: i32 = val1_3;
                                    if (this.game.Data.SFTypeObj[type].ReinforcementType > -1 & this.game.Data.SFTypeObj[type].ReinforcementType2 > -1 && this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] > 0 & this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2] > 0 && this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] != this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2])
                                    {
                                      val1_3 =  Math.Round(Conversion.Int( val1_3 * ( this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] /  this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2])));
                                      if (val1_3 > num73 - num74)
                                      {
                                        val1_3 = Math.Min(val1_3, num73 - num74);
                                        num75 =  Math.Round(Conversion.Int( val1_3 * ( this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2] /  this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType])));
                                        if ( num75 <  val1_3 * ( this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2] /  this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType]))
                                          num75 += 1;
                                      }
                                    }
                                    if (val1_3 > 0)
                                    {
                                      int[] weight5 = simpleList7.Weight;
                                      int[] numArray36 = weight5;
                                      let mut index108: i32 = nr19;
                                      let mut index109: i32 = index108;
                                      let mut num76: i32 = weight5[index108] + val1_3;
                                      numArray36[index109] = num76;
                                      int[] data3 = simpleList7.Data3;
                                      int[] numArray37 = data3;
                                      let mut index110: i32 = nr19;
                                      let mut index111: i32 = index110;
                                      let mut num77: i32 = data3[index110] + 1;
                                      numArray37[index111] = num77;
                                      int[] weight6 = simpleList7.Weight;
                                      int[] numArray38 = weight6;
                                      let mut index112: i32 = nr18;
                                      let mut index113: i32 = index112;
                                      let mut num78: i32 = weight6[index112] - num75;
                                      numArray38[index113] = num78;
                                      if (simpleList7.Weight[nr18] <= 0)
                                        simpleList7.RemoveNr(nr18);
                                    }
                                  }
                                  else
                                  {
                                    let mut num79: i32 = 0;
                                    let mut num80: i32 = Math.Min(val1_2, num73 - num79);
                                    let mut num81: i32 = num80;
                                    if (this.game.Data.SFTypeObj[type].ReinforcementType > -1 & this.game.Data.SFTypeObj[type].ReinforcementType2 > -1 && this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] > 0 & this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2] > 0 && this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] != this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2])
                                    {
                                      num80 =  Math.Round(Conversion.Int( num80 * ( this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType] /  this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2])));
                                      if (num80 > num73 - num79)
                                      {
                                        num80 = Math.Min(num80, num73 - num79);
                                        num81 =  Math.Round(Conversion.Int( num80 * ( this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2] /  this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType])));
                                        if ( num81 <  num80 * ( this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType2] /  this.game.Data.ReinfRatio[this.game.Data.SFTypeObj[type].ReinforcementType]))
                                          num81 += 1;
                                      }
                                    }
                                    if (num80 > 0)
                                    {
                                      simpleList7.Add(index1, num80, moveType, people, 1, CheckExistence: false);
                                      num12 = simpleList7.Counter;
                                      int[] weight = simpleList7.Weight;
                                      int[] numArray39 = weight;
                                      let mut index114: i32 = nr18;
                                      let mut index115: i32 = index114;
                                      let mut num82: i32 = weight[index114] - num81;
                                      numArray39[index115] = num82;
                                      if (simpleList7.Weight[nr18] <= 0)
                                        simpleList7.RemoveNr(nr18);
                                    }
                                  }
                                }
                              }
                            }
                          }
                        }
                        let mut counter22: i32 = simpleList6.Counter;
                        for (let mut index116: i32 = 0; index116 <= counter22; index116 += 1)
                        {
                          index1 = simpleList7.FindNr(simpleList6.Id[index116], simpleList6.Data1[index116], simpleList6.Data2[index116]);
                          if (index1 > -1)
                          {
                            int[] weight = simpleList7.Weight;
                            int[] numArray40 = weight;
                            let mut index117: i32 = index1;
                            let mut index118: i32 = index117;
                            let mut num83: i32 = weight[index117] - simpleList6.Weight[index116];
                            numArray40[index118] = num83;
                            if (1 > simpleList7.Weight[index1])
                              simpleList7.RemoveNr(index1);
                          }
                        }
                        SimpleList simpleList9 = SimpleList::new();
                        let mut counter23: i32 = simpleList7.Counter;
                        for (let mut index119: i32 = 0; index119 <= counter23; index119 += 1)
                        {
                          simpleList7.Data5[index119] = 0;
                          simpleList9.Add(simpleList7.Id[index119], simpleList7.Weight[index119], simpleList7.Data1[index119], simpleList7.Data2[index119], CheckExistence: false);
                        }
                        if (unr == 3)
                          unr = unr;
                        if ( this.game.Data.RuleVar[883] > 0.0)
                        {
                          if (flag1)
                            simpleList8 = simpleList7.Clone();
                          let mut counter24: i32 = simpleList8.Counter;
                          for (let mut index120: i32 = 0; index120 <= counter24; index120 += 1)
                          {
                            let mut num84: i32 = simpleList8.Weight[index120];
                            index1 = !(flag1 & this.game.Data.Product == 6) ? Math.Max(1,  Math.Round( ( num84 * (this.game.Data.RuleVar[883] / 100f)))) : Math.Max(1,  Math.Round(Math.Ceiling( num84 * ( this.game.Data.RuleVar[883] / 100.0))));
                            let mut counter25: i32 = simpleList7.Counter;
                            for (let mut index121: i32 = 0; index121 <= counter25; index121 += 1)
                            {
                              if (simpleList7.Id[index121] == simpleList8.Id[index120] & simpleList7.Data1[index121] == simpleList8.Data1[index120] & simpleList7.Data2[index121] == simpleList8.Data2[index120])
                              {
                                simpleList7.Data5[index121] = 1;
                                if (index1 < simpleList7.Weight[index121])
                                  simpleList7.Weight[index121] = index1;
                              }
                            }
                          }
                          let mut counter26: i32 = simpleList7.Counter;
                          for (let mut index122: i32 = 0; index122 <= counter26; index122 += 1)
                          {
                            if (simpleList7.Data5[index122] == 0 && simpleList7.Weight[index122] > 1)
                              simpleList7.Weight[index122] = Math.Max(1,  Math.Round( ( simpleList7.Weight[index122] * (this.game.Data.RuleVar[883] / 100f))));
                          }
                          let mut counter27: i32 = simpleListArray5[unr].Counter;
                          for (let mut index123: i32 = 0; index123 <= counter27; index123 += 1)
                          {
                            for (let mut counter28: i32 = simpleList7.Counter; counter28 >= 0; counter28 += -1)
                            {
                              if (simpleList7.Id[counter28] == simpleListArray5[unr].Id[index123] & simpleList7.Data1[counter28] == simpleListArray5[unr].Data1[index123] & simpleList7.Data2[counter28] == simpleListArray5[unr].Data2[index123])
                              {
                                int[] weight = simpleList7.Weight;
                                int[] numArray41 = weight;
                                let mut index124: i32 = counter28;
                                let mut index125: i32 = index124;
                                let mut num85: i32 = weight[index124] - simpleListArray5[unr].Weight[index123];
                                numArray41[index125] = num85;
                                if (simpleList7.Weight[counter28] < 1)
                                  simpleList7.RemoveNr(counter28);
                              }
                            }
                          }
                        }
                        if (this.game.Data.UnitObj[unr].IsHQ)
                        {
                          let mut unitCounter7: i32 = this.game.Data.UnitCounter;
                          for (index1 = 0; index1 <= unitCounter7; index1 += 1)
                          {
                            if (this.game.Data.UnitObj[index1].HQ == unr && !Information.IsNothing( simpleListArray1[index1]))
                            {
                              let mut counter29: i32 = simpleListArray1[index1].Counter;
                              for (let mut index126: i32 = 0; index126 <= counter29; index126 += 1)
                              {
                                let mut nr21: i32 = simpleList7.FindNr(simpleListArray1[index1].Id[index126], simpleListArray1[index1].Data1[index126], simpleListArray1[index1].Data2[index126]);
                                if (nr21 > -1)
                                {
                                  int[] weight = simpleList7.Weight;
                                  int[] numArray42 = weight;
                                  let mut index127: i32 = nr21;
                                  let mut index128: i32 = index127;
                                  let mut num86: i32 = weight[index127] - simpleListArray1[index1].Weight[index126];
                                  numArray42[index128] = num86;
                                  if (1 > simpleList7.Weight[nr21])
                                    simpleList7.RemoveNr(nr21);
                                }
                                let mut nr22: i32 = simpleList9.FindNr(simpleListArray1[index1].Id[index126], simpleListArray1[index1].Data1[index126], simpleListArray1[index1].Data2[index126]);
                                if (nr22 > -1)
                                {
                                  int[] weight = simpleList9.Weight;
                                  int[] numArray43 = weight;
                                  let mut index129: i32 = nr22;
                                  let mut index130: i32 = index129;
                                  let mut num87: i32 = weight[index129] - simpleListArray1[index1].Weight[index126];
                                  numArray43[index130] = num87;
                                  if (1 > simpleList9.Weight[nr22])
                                    simpleList9.RemoveNr(nr22);
                                }
                              }
                            }
                          }
                        }
                        if (Operators.CompareString(this.game.Data.UnitObj[unr].Name, "11th Army", false) == 0)
                          unr = unr;
                        if (this.game.Data.UnitObj[unr].IsHQ && !Information.IsNothing( simpleListArray4[unr]))
                        {
                          let mut counter30: i32 = simpleListArray4[unr].Counter;
                          for (let mut index131: i32 = 0; index131 <= counter30; index131 += 1)
                          {
                            let mut nr23: i32 = simpleList7.FindNr(simpleListArray4[unr].Id[index131], simpleListArray4[unr].Data1[index131], simpleListArray4[unr].Data2[index131]);
                            if (nr23 > -1)
                            {
                              int[] weight = simpleList7.Weight;
                              int[] numArray44 = weight;
                              let mut index132: i32 = nr23;
                              let mut index133: i32 = index132;
                              let mut num88: i32 = weight[index132] - simpleListArray4[unr].Weight[index131];
                              numArray44[index133] = num88;
                              if (1 > simpleList7.Weight[nr23])
                                simpleList7.RemoveNr(nr23);
                            }
                            let mut nr24: i32 = simpleList9.FindNr(simpleListArray4[unr].Id[index131], simpleListArray4[unr].Data1[index131], simpleListArray4[unr].Data2[index131]);
                            if (nr24 > -1)
                            {
                              int[] weight = simpleList9.Weight;
                              int[] numArray45 = weight;
                              let mut index134: i32 = nr24;
                              let mut index135: i32 = index134;
                              let mut num89: i32 = weight[index134] - simpleListArray4[unr].Weight[index131];
                              numArray45[index135] = num89;
                              if (1 > simpleList9.Weight[nr24])
                                simpleList9.RemoveNr(nr24);
                            }
                          }
                        }
                        if (num60 == 1)
                        {
                          for (let mut counter31: i32 = simpleList9.Counter; counter31 >= 0; counter31 += -1)
                            this.game.Data.UnitObj[unr].AddLog(10, simpleList9.Id[counter31], simpleList9.Data2[counter31], simpleList9.Weight[counter31]);
                        }
                        for (let mut counter32: i32 = simpleList7.Counter; counter32 >= 0; counter32 += -1)
                        {
                          index1 = this.AutoReinforce_UnitWillGive(unr, simpleList7.Id[counter32], simpleList7.Data1[counter32], simpleList7.Data2[counter32]);
                          if (index1 > -1)
                          {
                            let mut theater: i32 = -2;
                            if ( this.game.Data.RuleVar[340] == -1.0)
                              theater = -1;
                            if ( this.game.Data.RuleVar[340] > -1.0)
                            {
                              if (( this.game.Data.RuleVar[340] == 2.0 |  this.game.Data.RuleVar[340] == 3.0) & this.game.Data.UnitObj[index7].AirCap >= mapMatrix2Array2[this.game.Data.UnitObj[unr].Map].Value[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y] * this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Weight)
                              {
                                theater = 2;
                                this.game.EditObj.TempValue = mapMatrix2Array2;
                              }
                              else if (( this.game.Data.RuleVar[340] == 1.0 |  this.game.Data.RuleVar[340] == 3.0) & this.game.Data.UnitObj[index7].NavyCap >= mapMatrix2Array1[this.game.Data.UnitObj[unr].Map].Value[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y] * this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Weight)
                              {
                                theater = 1;
                                this.game.EditObj.TempValue = mapMatrix2Array1;
                              }
                              else if (( this.game.Data.RuleVar[340] == 0.0 |  this.game.Data.RuleVar[340] == 3.0) & this.game.Data.UnitObj[index7].LandCap >= mapMatrix2Array3[this.game.Data.UnitObj[unr].Map].Value[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y] * this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Weight)
                              {
                                theater = 0;
                                this.game.EditObj.TempValue = mapMatrix2Array3;
                              }
                              this.game.EditObj.TempValue2 = (MapMatrix2[]) this.game.EditObj.TempValue.Clone();
                              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].MoveType, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Theater, this.game.Data.SFObj[index1].Ap, this.game.Data.UnitObj[index7].X, this.game.Data.UnitObj[index7].Y, this.game.Data.UnitObj[index7].Map);
                              if (this.game.EditObj.TempValue[this.game.Data.UnitObj[unr].Map].Value[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y] <= this.game.Data.SFObj[index1].Ap)
                                theater = -1;
                              this.game.EditObj.TempValue = this.game.EditObj.TempValue2;
                            }
                            if (theater > -2)
                            {
                              this.AddPLog("(RETURN) transfer 1 " + this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].Name + " FROM " + this.game.Data.UnitObj[unr].Name + ", landcap=" + Conversion.Str( this.game.Data.UnitObj[index7].LandCap) + ", railcap=" + Conversion.Str( this.game.Data.UnitObj[index7].AirCap) + ", seacap=" + Conversion.Str( this.game.Data.UnitObj[index7].NavyCap));
                              let mut nr: i32 = simpleListArray5[unr].FindNr(simpleList7.Id[counter32], simpleList7.Data1[counter32], simpleList7.Data2[counter32]);
                              if (nr > -1)
                              {
                                int[] weight = simpleListArray5[unr].Weight;
                                int[] numArray46 = weight;
                                let mut index136: i32 = nr;
                                let mut index137: i32 = index136;
                                let mut num90: i32 = weight[index136] + Math.Min(val1_1, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].PowerPts);
                                numArray46[index137] = num90;
                              }
                              else
                                simpleListArray5[unr].Add(simpleList7.Id[counter32], Math.Min(val1_1, this.game.Data.SFTypeObj[this.game.Data.SFObj[index1].Type].PowerPts), simpleList7.Data1[counter32], simpleList7.Data2[counter32]);
                              let mut type: i32 = this.game.Data.SFObj[index1].Type;
                              if (theater == -1)
                                this.DoTransfer(unr, index7, 0, index1, 1, true, false, MoveMatrixDone: true, IsDisbandTransfer: true);
                              else
                                this.DoTransfer(unr, index7, theater, index1, 1, AddtoHistory: false, byHQ: index7, MoveMatrixDone: true, IsDisbandTransfer: true);
                              if (unr == 3)
                                unr = unr;
                              if (this.game.EditObj.TransferLostQty < 1)
                              {
                                this.game.Data.UnitObj[unr].AddLog(3, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList7.Data2[counter32], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                                this.game.Data.UnitObj[index7].AddLog(8, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList7.Data2[counter32], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                              }
                              else
                                this.game.Data.UnitObj[unr].AddLog(4, this.game.Data.SFTypeObj[type].ReinforcementType, simpleList7.Data2[counter32], Math.Min(val1_1, this.game.Data.SFTypeObj[type].PowerPts));
                              num59 = 1;
                              break;
                            }
                          }
                        }
                      }
                    }
                  }
                }
                secondaryPhase += 1;
              }
              while (secondaryPhase <= 1);
              num6 += 1;
            }
            while (num6 <= 2);
          }
        }
      }
      this.game.HandyFunctionsObj.RedimTempValue(9999);
      this.game.HandyFunctionsObj.RedimTempValue2(9999);
      this.WritePLog("autoreinforcelog");
    }

    pub fn SetCapForUnitS()
    {
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
      {
        if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unr].PreDef == -1)
          this.game.HandyFunctionsObj.SetCapForUnit(unr);
      }
    }

    pub fn DoSupplySystem()
    {
      Coordinate[] coordinateArray = new Coordinate[251];
      int[] numArray1 = new int[this.game.Data.UnitCounter + 1];
      int[] numArray2 = new int[this.game.Data.UnitCounter + 1];
      int[] numArray3 = new int[this.game.Data.UnitCounter + 1];
      if (this.game.Data.UnitCounter < 0)
        return;
      int[] numArray4 = new int[this.game.Data.UnitCounter + 1];
      int[] numArray5 = new int[this.game.Data.UnitCounter + 1];
      bool useTrafficRules = false;
      if ( this.game.Data.RuleVar[459] > 0.0 & this.game.Data.Product >= 6)
        useTrafficRules = true;
      let mut num1: i32 = -1;
      let mut unitCounter1: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
      {
        numArray4[index] = -1;
        numArray3[index] = 0;
        numArray5[index] = 0;
        if ( this.game.Data.RuleVar[322] == 1.0)
          this.game.Data.UnitObj[index].Reserve = 0;
        if (this.game.Data.UnitObj[index].Regime == this.game.Data.Turn)
        {
          this.game.Data.UnitObj[index].SupplyInReq = 0;
          this.game.Data.UnitObj[index].FuelRequested = 0;
        }
        if (this.game.Data.UnitObj[index].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index].PreDef == -1)
        {
          if (!this.game.Data.UnitObj[index].UnitIsGiven)
          {
            if ( this.game.Data.RuleVar[434] < 1.0)
              this.game.Data.UnitObj[index].SupplyConsume = 0;
          }
          else
          {
            this.game.Data.UnitObj[index].SupplyIn = 0;
            this.game.Data.UnitObj[index].FuelRequested = 0;
          }
        }
      }
      num2: i32;
      do
      {
        num2 = 0;
        num1 += 1;
        let mut unitCounter2: i32 = this.game.Data.UnitCounter;
        for (let mut index: i32 = 0; index <= unitCounter2; index += 1)
        {
          if (!this.game.Data.UnitObj[index].UnitIsGiven & this.game.Data.UnitObj[index].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index].IsHQ & this.game.Data.UnitObj[index].PreDef <= -1)
          {
            if (num1 == 0)
            {
              if (this.game.Data.UnitObj[index].HQ == -1)
              {
                numArray4[index] = 0;
                num2 = 1;
              }
            }
            else if (!this.game.Data.UnitObj[index].UnitIsGiven & this.game.Data.UnitObj[index].HQ > -1 & numArray4[index] == -1 & this.game.Data.UnitObj[index].PreDef <= -1)
            {
              let mut hq: i32 = this.game.Data.UnitObj[index].HQ;
              if (numArray4[hq] == num1 - 1)
              {
                num2 = 1;
                numArray4[index] = num1;
              }
            }
          }
        }
      }
      while (num2 > 0);
      object obj =  (num1 - 1);
      if ( this.game.Data.RuleVar[887] == 1.0)
        obj =  0;
      for (let mut integer: i32 = Conversions.ToInteger(obj); integer >= 0; integer += -1)
      {
        let mut unitCounter3: i32 = this.game.Data.UnitCounter;
        for (let mut index: i32 = 0; index <= unitCounter3; index += 1)
        {
          if (!this.game.Data.UnitObj[index].UnitIsGiven & this.game.Data.UnitObj[index].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index].IsHQ & this.game.Data.UnitObj[index].PreDef <= -1 && numArray4[index] == integer)
          {
            Application.DoEvents();
            let mut num3: i32 = this.game.Data.UnitObj[index].Supply - this.game.Data.UnitObj[index].Reserve;
            let mut num4: i32 = 0;
            if (num3 < 0)
              num3 = 0;
            let mut unitCounter4: i32 = this.game.Data.UnitCounter;
            for (let mut unr: i32 = 0; unr <= unitCounter4; unr += 1)
            {
              if (!this.game.Data.UnitObj[unr].UnitIsGiven & this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & (this.game.Data.UnitObj[unr].HQ == index |  this.game.Data.RuleVar[887] == 1.0 & this.game.HandyFunctionsObj.IsUnitInHQChain(unr, index)) & this.game.Data.UnitObj[unr].PreDef <= -1)
              {
                if (this.game.Data.UnitObj[unr].IsHQ &  this.game.Data.RuleVar[887] < 1.0)
                {
                  num4 += numArray5[unr];
                  numArray1[unr] = numArray5[unr];
                }
                else
                {
                  let mut num5: i32 = this.game.HandyFunctionsObj.UnitSupplyNeed(unr, true);
                  if (unr == 137)
                    unr = unr;
                  num4 += num5;
                  numArray1[unr] = num5;
                  this.game.Data.UnitObj[unr].SupplyInReq = num5;
                }
              }
            }
            let mut num6: i32 =  this.game.Data.RuleVar[887] != 1.0 ? this.game.Data.UnitObj[index].SOSupReqPercent : this.game.HandyFunctionsObj.GetAggregatedSupplyRequest(index);
            if (this.game.Data.UnitObj[index].HQ != -1)
              num4 =  Math.Round( num4 * ( num6 / 100.0));
            let mut num7: i32 = num4 + this.game.HandyFunctionsObj.UnitSupplyNeed(index, false);
            let mut num8: i32 = num3 - num7;
            numArray5[index] = num8 >= 0 ? 0 : -num8;
            this.game.Data.UnitObj[index].SupplyInReq = numArray5[index];
          }
        }
      }
      let mut integer1: i32 = Conversions.ToInteger(obj);
      for (let mut index1: i32 = 0; index1 <= integer1; index1 += 1)
      {
        let mut unitCounter5: i32 = this.game.Data.UnitCounter;
        for (let mut index2: i32 = 0; index2 <= unitCounter5; index2 += 1)
        {
          if (this.game.Data.UnitObj[index2].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index2].IsHQ && index1 == numArray4[index2])
          {
            let mut num9: i32 = this.game.Data.UnitObj[index2].Supply;
            let mut num10: i32 = this.game.HandyFunctionsObj.UnitSupplyNeed(index2, false);
            if (num9 < 0)
              num9 = 0;
            SimpleList simpleList = SimpleList::new();
            let mut unitCounter6: i32 = this.game.Data.UnitCounter;
            for (let mut unr: i32 = 0; unr <= unitCounter6; unr += 1)
            {
              if (!this.game.Data.UnitObj[unr].UnitIsGiven & this.game.Data.UnitObj[unr].X > -1 & this.game.Data.UnitObj[unr].PreDef <= -1 && this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & (this.game.Data.UnitObj[unr].HQ == index2 |  this.game.Data.RuleVar[887] == 1.0 & this.game.HandyFunctionsObj.IsUnitInHQChain(unr, index2)))
              {
                if (this.game.Data.UnitObj[unr].IsHQ &  this.game.Data.RuleVar[887] < 1.0)
                {
                  num10 += numArray5[unr];
                }
                else
                {
                  let mut num11: i32 = this.game.HandyFunctionsObj.UnitSupplyNeed(unr, true);
                  num10 += num11;
                }
              }
            }
            this.game.Data.UnitObj[index2].SupplyReq = num10 - this.game.HandyFunctionsObj.UnitSupplyNeed(index2, false);
            float num12 = num10 <= num9 ? 1f :  num9 /  num10;
            let mut unitCounter7: i32 = this.game.Data.UnitCounter;
            unr1: i32;
            for (let mut index3: i32 = 0; index3 <= unitCounter7; index3 += 1)
            {
              if (!this.game.Data.UnitObj[index3].UnitIsGiven & this.game.Data.UnitObj[index3].X > -1 & this.game.Data.UnitObj[index3].PreDef <= -1 && this.game.Data.UnitObj[index3].Regime == this.game.Data.Turn & (this.game.Data.UnitObj[index3].HQ == index2 |  this.game.Data.RuleVar[887] == 1.0 & this.game.HandyFunctionsObj.IsUnitInHQChain(index3, index2)))
              {
                let mut num13: i32 = 0;
                if (Strings.InStr(this.game.Data.UnitObj[index3].Name, "11th Panzer") > 0)
                  unr1 = unr1;
                if (index3 == 154)
                  unr1 = unr1;
                if (!this.game.Data.UnitObj[index3].IsHQ |  this.game.Data.RuleVar[887] == 1.0)
                  num13 = this.game.HandyFunctionsObj.UnitSupplyNeed(index3, true);
                else if (this.game.Data.UnitObj[index3].IsHQ)
                  num13 += numArray5[index3];
                let mut tdata1: i32 =  Math.Round( Conversion.Int( num13 * num12));
                simpleList.Add(index3, 0, tdata1);
              }
            }
            if (simpleList.Counter > -1 && this.game.Data.UnitObj[index2].X > -1)
            {
              let mut movetype: i32 =  Math.Round( this.game.Data.RuleVar[99]);
              index4: i32;
              if ( this.game.Data.RuleVar[455] > 0.0)
              {
                this.game.HandyFunctionsObj.MakeFuzzyOwner(true, false, this.game.Data.Turn);
                let mut mapWidth: i32 = this.game.Data.MapObj[index4].MapWidth;
                for (let mut index5: i32 = 0; index5 <= mapWidth; index5 += 1)
                {
                  let mut mapHeight: i32 = this.game.Data.MapObj[index4].MapHeight;
                  for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
                    this.game.EditObj.TempValue2[index4].Value[index5, index6] = this.game.Data.MapObj[0].HexObj[index5, index6].Regime;
                }
                this.game.HandyFunctionsObj.MakeFuzzyOwner(false, false, this.game.Data.Turn);
              }
              CoordList coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99,  Math.Round( this.game.Data.RuleVar[3]), this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
              if (numArray3[index2] > 0)
              {
                let mut mapCounter: i32 = this.game.Data.MapCounter;
                for (index4 = 0; index4 <= mapCounter; index4 += 1)
                {
                  let mut mapWidth: i32 = this.game.Data.MapObj[index4].MapWidth;
                  for (let mut index7: i32 = 0; index7 <= mapWidth; index7 += 1)
                  {
                    let mut mapHeight: i32 = this.game.Data.MapObj[index4].MapHeight;
                    for (let mut index8: i32 = 0; index8 <= mapHeight; index8 += 1)
                    {
                      numArray6: Vec<i32> = this.game.EditObj.TempValue[index4].Value;
                      numArray7: Vec<i32> = numArray6;
                      let mut index9: i32 = index7;
                      let mut index10: i32 = index9;
                      let mut index11: i32 = index8;
                      let mut index12: i32 = index11;
                      let mut num14: i32 = numArray6[index9, index11] + numArray3[index2];
                      numArray7[index10, index12] = num14;
                    }
                  }
                }
              }
              let mut counter1: i32 = simpleList.Counter;
              for (let mut index13: i32 = 0; index13 <= counter1; index13 += 1)
                simpleList.Data2[index13] =  this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList.Id[index13]].Map].Value[this.game.Data.UnitObj[simpleList.Id[index13]].X, this.game.Data.UnitObj[simpleList.Id[index13]].Y] <=  this.game.Data.RuleVar[3] ? 0 : 1;
              let mut counter2: i32 = simpleList.Counter;
              for (let mut index14: i32 = 0; index14 <= counter2; index14 += 1)
              {
                index15: i32;
                if ( this.game.Data.RuleVar[455] > 0.0 && simpleList.Data2[index14] == 0)
                {
                  bool flag = true;
                  let mut num15: i32 = 0;
                  while (flag)
                  {
                    num15 += 1;
                    flag = false;
                    CoordList coordList2 = CoordList::new();
                    Coordinate coordinate;
                    coordinate.x = this.game.Data.UnitObj[simpleList.Id[index14]].X;
                    coordinate.y = this.game.Data.UnitObj[simpleList.Id[index14]].Y;
                    if (coordinate.x == 41 & coordinate.y == 27)
                      index15 = index15;
                    coordinate.onmap = true;
                    while (coordinate.onmap)
                    {
                      coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y];
                      if (coordinate.onmap)
                        coordList2.AddCoord(coordinate.x, coordinate.y, 0);
                    }
                    if (coordList2.counter > -1)
                    {
                      flag = false;
                      for (let mut index16: i32 = coordList2.counter; index16 >= 0; index16 += -1)
                      {
                        if (coordList2.coord[index14].x == 47 & coordList2.coord[index14].y == 23)
                          index16 = index16;
                        if (this.game.EditObj.TempValue2[index4].Value[coordList2.coord[index16].x, coordList2.coord[index16].y] != this.game.Data.Turn)
                        {
                          this.game.Data.MapObj[0].HexObj[coordList2.coord[index16].x, coordList2.coord[index16].y].FuzzyBlock = 1;
                          this.game.HandyFunctionsObj.MakeFuzzyOwner(false, false, this.game.Data.Turn);
                          coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99,  Math.Round( this.game.Data.RuleVar[3]), this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                          flag = true;
                          break;
                        }
                      }
                    }
                    if (!flag &&  this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList.Id[index14]].Map].Value[this.game.Data.UnitObj[simpleList.Id[index14]].X, this.game.Data.UnitObj[simpleList.Id[index14]].Y] >  this.game.Data.RuleVar[3])
                      simpleList.Data2[index14] = 1;
                    if (num15 > 999)
                    {
                      flag = false;
                      simpleList.Data2[index14] = 1;
                    }
                  }
                }
                if (simpleList.Data2[index14] == 0)
                {
                  unr1 = simpleList.Id[index14];
                  let mut num16: i32 = simpleList.Data1[index14];
                  let mut num17: i32 = 100;
                  if ( this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList.Id[index14]].Map].Value[this.game.Data.UnitObj[simpleList.Id[index14]].X, this.game.Data.UnitObj[simpleList.Id[index14]].Y] >  this.game.Data.RuleVar[51])
                    num17 = 75;
                  if ( this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList.Id[index14]].Map].Value[this.game.Data.UnitObj[simpleList.Id[index14]].X, this.game.Data.UnitObj[simpleList.Id[index14]].Y] >  this.game.Data.RuleVar[52])
                    num17 = 50;
                  if ( this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList.Id[index14]].Map].Value[this.game.Data.UnitObj[simpleList.Id[index14]].X, this.game.Data.UnitObj[simpleList.Id[index14]].Y] >  this.game.Data.RuleVar[53])
                    num17 = 25;
                  if ( this.game.Data.RuleVar[434] > 0.0)
                    num17 = this.game.Data.UnitObj[unr1].SOSupReqPercent > num17 ?  Math.Round(Math.Floor( (100 * num17) /  this.game.Data.UnitObj[unr1].SOSupReqPercent)) : 100;
                  let mut num18: i32 =  Math.Round(Conversion.Int(Math.Floor( (num16 * num17) / 100.0)));
                  let mut num19: i32 = num18;
                  let mut antiSupply: i32 = this.game.HandyFunctionsObj.GetAntiSupply(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, this.game.Data.UnitObj[unr1].X, this.game.Data.UnitObj[unr1].Y, this.game.Data.UnitObj[unr1].Map);
                  let mut suppts: i32 =  Math.Round(Conversion.Int( num18 * ( antiSupply / 100.0)));
                  let mut num20: i32 = num18 - suppts;
                  RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                  RegimeClass[] regimeClassArray = regimeObj;
                  let mut turn: i32 = this.game.Data.Turn;
                  let mut index17: i32 = turn;
                  regimeClassArray[index17].SASSupplyLost = regimeObj[turn].SASSupplyLost + suppts;
                  this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, this.game.Data.UnitObj[unr1].X, this.game.Data.UnitObj[unr1].Y, this.game.Data.UnitObj[unr1].Map, suppts, 0, 0);
                  if ( this.game.Data.RuleVar[324] == 1.0 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                    numArray3[unr1] = this.game.EditObj.TempValue[this.game.Data.UnitObj[unr1].Map].Value[this.game.Data.UnitObj[unr1].X, this.game.Data.UnitObj[unr1].Y];
                  if (this.game.Data.UnitObj[unr1].Supply < 0)
                    this.game.Data.UnitObj[unr1].Supply = 0;
                  let mut num21: i32 = Math.Max(num20 - (this.game.HandyFunctionsObj.UnitSupplyNeed(unr1, true, false) + numArray5[unr1]), 0);
                  let mut num22: i32 = num20 - num21;
                  this.game.Data.UnitObj[unr1].Supply += num22;
                  UnitClass[] unitObj1 = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray1 = unitObj1;
                  let mut index18: i32 = unr1;
                  let mut index19: i32 = index18;
                  unitClassArray1[index19].StockpileCurrent = unitObj1[index18].StockpileCurrent + num21;
                  if (this.game.Data.UnitObj[unr1].StockpileCurrent > this.game.HandyFunctionsObj.GetMaxStockpile(unr1))
                    this.game.Data.UnitObj[unr1].StockpileCurrent = this.game.HandyFunctionsObj.GetMaxStockpile(unr1);
                  if (num21 > 0)
                    ;
                  if (unr1 == 815)
                    unr1 = unr1;
                  int[] numArray8 = numArray2;
                  int[] numArray9 = numArray8;
                  let mut index20: i32 = unr1;
                  let mut index21: i32 = index20;
                  let mut num23: i32 = numArray8[index20] + num20;
                  numArray9[index21] = num23;
                  UnitClass[] unitObj2 = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray2 = unitObj2;
                  let mut index22: i32 = unr1;
                  let mut index23: i32 = index22;
                  unitClassArray2[index23].SupplyLost = unitObj2[index22].SupplyLost + suppts;
                  UnitClass[] unitObj3 = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray3 = unitObj3;
                  let mut index24: i32 = unr1;
                  let mut index25: i32 = index24;
                  unitClassArray3[index25].SupplyIn = unitObj3[index24].SupplyIn + num20;
                  UnitClass[] unitObj4 = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray4 = unitObj4;
                  let mut index26: i32 = index2;
                  let mut index27: i32 = index26;
                  unitClassArray4[index27].SupplyOut = unitObj4[index26].SupplyOut + num19;
                  simpleList.Data1[index14] = simpleList.Data1[index14] - num20;
                  this.game.Data.UnitObj[index2].Supply -= num19;
                  if (useTrafficRules)
                  {
                    Coordinate coordinate1;
                    coordinate1.x = this.game.Data.UnitObj[unr1].X;
                    coordinate1.y = this.game.Data.UnitObj[unr1].Y;
                    bool flag = false;
                    let mut index28: i32 = 0;
                    while (this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y].onmap)
                    {
                      let mut num24: i32 = index28 + 1;
                      Coordinate coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                      index15 = this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
                      let mut index29: i32 = index15 + 3;
                      if (index29 > 5)
                        index29 -= 6;
                      let mut liSpoint1: i32 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index15];
                      let mut liSpoint2: i32 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index29];
                      int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints;
                      int[] numArray10 = liSpoints1;
                      let mut index30: i32 = index15;
                      let mut index31: i32 = index30;
                      let mut num25: i32 = liSpoints1[index30] + num19;
                      numArray10[index31] = num25;
                      int[] liSpoints2 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints;
                      int[] numArray11 = liSpoints2;
                      let mut index32: i32 = index29;
                      let mut index33: i32 = index32;
                      let mut num26: i32 = liSpoints2[index32] + num19;
                      numArray11[index33] = num26;
                      let mut index34: i32 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RoadType[index15];
                      if (index34 > -1 && this.game.Data.RoadTypeObj[index34].trafficPoints > 0)
                      {
                        let mut num27: i32 =  Math.Round(Math.Floor( (liSpoint1 * 1) /  this.game.Data.RoadTypeObj[index34].trafficPoints));
                        if ( Math.Round(Math.Floor( (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index15] * 1) /  this.game.Data.RoadTypeObj[index34].trafficPoints)) > num27)
                          flag = true;
                      }
                      index28 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].RoadType[index29];
                      if (index28 > -1 && this.game.Data.RoadTypeObj[index28].trafficPoints > 0)
                      {
                        let mut num28: i32 =  Math.Round(Math.Floor( (liSpoint2 * 1) /  this.game.Data.RoadTypeObj[index28].trafficPoints));
                        if ( Math.Round(Math.Floor( (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index29] * 1) /  this.game.Data.RoadTypeObj[index28].trafficPoints)) > num28)
                          flag = true;
                      }
                      coordinate1 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                      if (index28 > 999)
                        break;
                    }
                    if (flag)
                      coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99,  Math.Round( this.game.Data.RuleVar[3]), this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                  }
                }
              }
            }
          }
        }
      }
      let mut unitCounter8: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter8; index += 1)
      {
        if (this.game.Data.UnitObj[index].Regime == this.game.Data.Turn)
          this.game.Data.UnitObj[index].LastSupplyPercent = numArray1[index] <= 0 ? -1 :  Math.Round(Conversion.Int( numArray2[index] /  numArray1[index] * 100.0));
      }
      if ( this.game.Data.RuleVar[435] <= 0.0)
        return;
      let mut num29: i32 = 0;
      let mut unitCounter9: i32 = this.game.Data.UnitCounter;
      for (let mut unr: i32 = 0; unr <= unitCounter9; unr += 1)
      {
        if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unr].PreDef == -1)
        {
          let mut num30: i32 = this.game.HandyFunctionsObj.UnitMaximumFuelReserve(unr);
          if (num30 > 0)
          {
            let mut num31: i32 = num30 - this.game.Data.UnitObj[unr].Fuel;
            if (num31 > 0)
              num29 += num31;
          }
        }
      }
      if (num29 <= 0)
        return;
      float num32 = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[ Math.Round( this.game.Data.RuleVar[435])] <= 0 ? 0.0f :  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[ Math.Round( this.game.Data.RuleVar[435])] /  num29;
      if ( num32 > 1.0)
        num32 = 1f;
      let mut unitCounter10: i32 = this.game.Data.UnitCounter;
      for (let mut unr: i32 = 0; unr <= unitCounter10; unr += 1)
      {
        if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unr].PreDef == -1)
        {
          this.game.Data.UnitObj[unr].FuelRequested = 0;
          this.game.Data.UnitObj[unr].FuelReceived = 0;
          let mut num33: i32 = Math.Max(0, this.game.HandyFunctionsObj.UnitMaximumFuelReserve(unr) - this.game.Data.UnitObj[unr].Fuel);
          this.game.Data.UnitObj[unr].FuelRequested = num33;
          if (num33 > 0 & this.game.Data.UnitObj[unr].LastSupplyPercent > 0)
          {
            let mut num34: i32 =  Math.Round(Math.Floor( num33 *  num32 *  this.game.Data.UnitObj[unr].LastSupplyPercent / 100.0));
            this.game.Data.UnitObj[unr].FuelReceived = num34;
            UnitClass[] unitObj = this.game.Data.UnitObj;
            UnitClass[] unitClassArray = unitObj;
            let mut index35: i32 = unr;
            let mut index36: i32 = index35;
            unitClassArray[index36].Fuel = unitObj[index35].Fuel + num34;
            int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
            int[] numArray12 = regimeSlot;
            let mut index37: i32 =  Math.Round( this.game.Data.RuleVar[435]);
            let mut index38: i32 = index37;
            let mut num35: i32 = regimeSlot[index37] - num34;
            numArray12[index38] = num35;
          }
        }
      }
    }

    pub void AddSupplyLog(
      fromUnitId: i32,
      fromLocId: i32,
      tooUnitId: i32,
      tooLocId: i32,
      type: i32,
      phase: i32,
      request: i32,
      delivered: i32,
      ap: i32,
      texty: String)
    {
      if ( this.game.Data.RuleVar[499] < 1.0)
        return;
      let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[499]));
      if (stringListById == -1)
        return;
      let mut turn: i32 = this.game.Data.Turn;
      this.game.Data.StringListObj[stringListById].AddRowWithData(turn.ToString(), fromUnitId.ToString(), fromLocId.ToString(), tooUnitId.ToString(), tooLocId.ToString(), type.ToString(), phase.ToString(), request.ToString(), delivered.ToString(), s10: ap.ToString(), s11: texty);
    }

    pub void DoSupplyBaseSystem(
      bool depleteMode,
      bool buildUpMode,
      bool evacuateMode,
      bool finalMode,
      bool supplySourceMode)
    {
      Coordinate[] coordinateArray = new Coordinate[251];
      let mut num1: i32 = Math.Max(this.game.Data.UnitCounter, this.game.Data.LocCounter);
      int[] numArray1 = new int[num1 + 1 + 1];
      int[] numArray2 = new int[num1 + 1 + 1];
      int[] numArray3 = new int[num1 + 1 + 1];
      int[] numArray4 = new int[num1 + 1 + 1];
      int[] numArray5 = new int[num1 + 1 + 1];
      int[] numArray6 = new int[num1 + 1 + 1];
      int[] numArray7 = new int[num1 + 1 + 1];
      int[] numArray8 = new int[num1 + 1 + 1];
      int[] numArray9 = new int[num1 + 1 + 1];
      int[] numArray10 = new int[num1 + 1 + 1];
      int[] numArray11 = new int[num1 + 1 + 1];
      int[] numArray12 = new int[num1 + 1 + 1];
      int[] numArray13 = new int[num1 + 1 + 1];
      index1: i32;
      if (depleteMode)
      {
        let mut unitCounter: i32 = this.game.Data.UnitCounter;
        for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
        {
          if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn && this.game.Data.UnitObj[unr].PreDef == -1 & this.game.Data.UnitObj[unr].X > -1)
          {
            let mut num2: i32 = this.game.HandyFunctionsObj.UnitMaximumFuelReserve(unr);
            if (this.game.Data.UnitObj[unr].Fuel > num2)
              this.game.Data.UnitObj[unr].Fuel = num2;
            index1 = this.game.HandyFunctionsObj.UnitSupplyStore(unr) + this.game.HandyFunctionsObj.UnitSupplyUse(unr);
            if (this.game.Data.UnitObj[unr].Supply > index1)
              this.game.Data.UnitObj[unr].Supply = index1;
          }
        }
      }
      if (finalMode)
      {
        let mut mapWidth: i32 = this.game.Data.MapObj[0].MapWidth;
        for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
          for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
          {
            if (this.game.Data.MapObj[0].HexObj[index2, index3].Location2 > -1 & this.game.Data.MapObj[0].HexObj[index2, index3].Regime == this.game.Data.Turn)
            {
              bool flag = false;
              location2: i32;
              if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index2, index3].Location2].Type].isSupplySource)
              {
                location2 = this.game.Data.MapObj[0].HexObj[index2, index3].Location2;
                if (this.game.Data.LocObj[location2].supplyIn <= 0 & this.game.Data.LocObj[location2].fuelIn <= 0 && this.game.Data.LocObj[location2].supply <= 0 & this.game.Data.LocObj[location2].fuel <= 0)
                  flag = true;
              }
              else if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index2, index3].Location2].Type].isSupplyBase)
              {
                let mut supplyBaseMode: i32 = this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index2, index3].Location2].supplyBaseMode;
                let mut num3: i32 = supplyBaseMode;
                location2 = this.game.Data.MapObj[0].HexObj[index2, index3].Location2;
                let mut type: i32 = this.game.Data.LocObj[location2].Type;
                if (supplyBaseMode == 5)
                {
                  let mut num4: i32 = this.game.Data.LocObj[location2].supply;
                  let mut num5: i32 =  Math.Round( (this.game.Data.LocTypeObj[type].maxSupply * this.game.Data.LocTypeObj[type].maxDestroy) / 100.0);
                  if (num5 < num4)
                    num4 = num5;
                  LocationClass[] locObj1 = this.game.Data.LocObj;
                  LocationClass[] locationClassArray1 = locObj1;
                  let mut index4: i32 = location2;
                  let mut index5: i32 = index4;
                  locationClassArray1[index5].supplyDestroyed = locObj1[index4].supplyDestroyed + num4;
                  LocationClass[] locObj2 = this.game.Data.LocObj;
                  LocationClass[] locationClassArray2 = locObj2;
                  let mut index6: i32 = location2;
                  let mut index7: i32 = index6;
                  locationClassArray2[index7].supply = locObj2[index6].supply - num4;
                  let mut num6: i32 = this.game.Data.LocObj[location2].fuel;
                  let mut num7: i32 =  Math.Round( (this.game.Data.LocTypeObj[type].maxFuel * this.game.Data.LocTypeObj[type].maxDestroy) / 100.0);
                  if (num7 < num6)
                    num6 = num7;
                  LocationClass[] locObj3 = this.game.Data.LocObj;
                  LocationClass[] locationClassArray3 = locObj3;
                  let mut index8: i32 = location2;
                  let mut index9: i32 = index8;
                  locationClassArray3[index9].fuelDestroyed = locObj3[index8].fuelDestroyed + num6;
                  LocationClass[] locObj4 = this.game.Data.LocObj;
                  LocationClass[] locationClassArray4 = locObj4;
                  let mut index10: i32 = location2;
                  let mut index11: i32 = index10;
                  locationClassArray4[index11].fuel = locObj4[index10].fuel - num6;
                }
                if (supplyBaseMode == 2 && this.game.Data.LocObj[location2].supply >= this.game.Data.LocTypeObj[type].maxSupply && this.game.Data.LocObj[location2].fuel >= this.game.Data.LocTypeObj[type].maxFuel && this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index2, index3].Location2].supplyBaseFixed < 1)
                  this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index2, index3].Location2].supplyBaseMode = 1;
                if ( Math.Round(Math.Floor( (this.game.Data.LocTypeObj[type].Logistical * this.game.Data.LocObj[location2].supply) /  this.game.Data.LocTypeObj[type].maxSupply)) <= 0 &  Math.Round(Math.Floor( (this.game.Data.LocTypeObj[type].Logistical * this.game.Data.LocObj[location2].fuel) /  this.game.Data.LocTypeObj[type].maxFuel)) <= 0 & (num3 == 4 | num3 == 5))
                  flag = true;
              }
              if (flag)
                this.game.Data.RemoveLoc(location2);
            }
          }
        }
      }
      else
      {
        if (this.game.Data.UnitCounter < 0)
          return;
        bool useTrafficRules = false;
        if ( this.game.Data.RuleVar[459] > 0.0 & this.game.Data.Product >= 6)
          useTrafficRules = true;
        if (depleteMode)
        {
          let mut unitCounter: i32 = this.game.Data.UnitCounter;
          for (let mut index12: i32 = 0; index12 <= unitCounter; index12 += 1)
          {
            this.game.Data.UnitObj[index12].supplyBaseSupplyIn = 0;
            this.game.Data.UnitObj[index12].supplyBaseFuelIn = 0;
          }
          let mut locCounter: i32 = this.game.Data.LocCounter;
          for (let mut index13: i32 = 0; index13 <= locCounter; index13 += 1)
          {
            if (this.game.Data.MapObj[0].HexObj[this.game.Data.LocObj[index13].X, this.game.Data.LocObj[index13].Y].Regime == this.game.Data.Turn)
            {
              this.game.Data.LocObj[index13].supplyEvacuated = 0;
              this.game.Data.LocObj[index13].supplyDestroyed = 0;
              this.game.Data.LocObj[index13].supplyOutUnits = 0;
              this.game.Data.LocObj[index13].fuelOutUnits = 0;
              if (!this.game.Data.LocTypeObj[this.game.Data.LocObj[index13].Type].isSupplySource)
              {
                this.game.Data.LocObj[index13].supplyIn = 0;
                this.game.Data.LocObj[index13].fuelIn = 0;
              }
              this.game.Data.LocObj[index13].supplyReq = 0;
              this.game.Data.LocObj[index13].fuelReq = 0;
              this.game.Data.LocObj[index13].fuelEvacuated = 0;
              this.game.Data.LocObj[index13].fuelDestroyed = 0;
              this.game.Data.LocObj[index13].fuelOutUnits = 0;
            }
          }
        }
        let mut unitCounter1: i32 = this.game.Data.UnitCounter;
        for (let mut index14: i32 = 0; index14 <= unitCounter1; index14 += 1)
        {
          numArray1[index14] = 0;
          numArray2[index14] = 0;
        }
        let mut locCounter1: i32 = this.game.Data.LocCounter;
        for (let mut index15: i32 = 0; index15 <= locCounter1; index15 += 1)
        {
          numArray1[index15] = 0;
          numArray2[index15] = 0;
          numArray4[index15] = 0;
          numArray3[index15] = 0;
        }
        if (supplySourceMode)
        {
          let mut locCounter2: i32 = this.game.Data.LocCounter;
          for (let mut index16: i32 = 0; index16 <= locCounter2; index16 += 1)
          {
            numArray3[index16] = 0;
            if (this.game.Data.MapObj[0].HexObj[this.game.Data.LocObj[index16].X, this.game.Data.LocObj[index16].Y].Regime == this.game.Data.Turn && this.game.Data.LocTypeObj[this.game.Data.LocObj[index16].Type].isSupplySource)
            {
              this.game.Data.LocObj[index16].supplyEvacuated = 0;
              this.game.Data.LocObj[index16].supplyDestroyed = 0;
              this.game.Data.LocObj[index16].supplyOutUnits = 0;
              this.game.Data.LocObj[index16].fuelOutUnits = 0;
              this.game.Data.LocObj[index16].supplyReq = 0;
              this.game.Data.LocObj[index16].fuelReq = 0;
              this.game.Data.LocObj[index16].fuelEvacuated = 0;
              this.game.Data.LocObj[index16].fuelDestroyed = 0;
              this.game.Data.LocObj[index16].fuelOutUnits = 0;
              this.game.Data.LocObj[index16].supplyOutBases = 0;
              this.game.Data.LocObj[index16].fuelOutBases = 0;
              this.game.Data.LocObj[index16].fuelExcess = 0;
              this.game.Data.LocObj[index16].supplyExcess = 0;
            }
          }
          let mut unitCounter2: i32 = this.game.Data.UnitCounter;
          for (let mut index17: i32 = 0; index17 <= unitCounter2; index17 += 1)
          {
            if ( this.game.Data.RuleVar[322] == 1.0)
              this.game.Data.UnitObj[index17].Reserve = 0;
            if (this.game.Data.UnitObj[index17].Regime == this.game.Data.Turn)
            {
              this.game.Data.UnitObj[index17].SupplyInReq = 0;
              this.game.Data.UnitObj[index17].FuelRequested = 0;
              this.game.Data.UnitObj[index17].supplyX = -1;
              this.game.Data.UnitObj[index17].supplyY = -1;
            }
            if (this.game.Data.UnitObj[index17].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index17].PreDef == -1)
            {
              if (!this.game.Data.UnitObj[index17].UnitIsGiven)
              {
                if ( this.game.Data.RuleVar[434] < 1.0)
                  this.game.Data.UnitObj[index17].SupplyConsume = 0;
              }
              else
              {
                this.game.Data.UnitObj[index17].SupplyIn = 0;
                this.game.Data.UnitObj[index17].FuelReceived = 0;
              }
            }
          }
        }
        let mut num8: i32 = 10;
        SimpleList simpleList1 = SimpleList::new();
        let mut mapWidth1: i32 = this.game.Data.MapObj[0].MapWidth;
        for (let mut tdata1: i32 = 0; tdata1 <= mapWidth1; tdata1 += 1)
        {
          let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
          for (let mut tdata2: i32 = 0; tdata2 <= mapHeight; tdata2 += 1)
          {
            if (this.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location2 > -1 & this.game.Data.MapObj[0].HexObj[tdata1, tdata2].Regime == this.game.Data.Turn && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location2].Type].isSupplyBase | this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location2].Type].isSupplySource)
              simpleList1.Add(tdata1 * 1000 + tdata2, 0, tdata1, tdata2);
          }
        }
        let mut num9: i32 = num8;
        num10: i32;
        num11: i32;
        num12: i32;
        num13: i32;
        for (let mut phase: i32 = 1; phase <= num9; phase += 1)
        {
          simpleList1.ReverseSort();
          index1 = index1;
          bool flag1 = false;
          let mut counter1: i32 = simpleList1.Counter;
          for (let mut index18: i32 = 0; index18 <= counter1; index18 += 1)
          {
            let mut index19: i32 = simpleList1.Data1[index18];
            let mut index20: i32 = simpleList1.Data2[index18];
            bool flag2 = false;
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index19, index20].Location2].Type].isSupplyBase)
            {
              index1 = this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index19, index20].Location2].supplyBaseMode;
              if (index1 == 3 & depleteMode)
                flag2 = true;
              if (index1 == 2 & buildUpMode)
                flag2 = true;
              if (index1 == 4 & evacuateMode)
                flag2 = true;
            }
            if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index19, index20].Location2].Type].isSupplySource && supplySourceMode)
              flag2 = true;
            if (flag2)
            {
              if (index18 != 0)
              {
                int[] weight = simpleList1.Weight;
                int[] numArray14 = weight;
                let mut index21: i32 = index18;
                let mut index22: i32 = index21;
                let mut num14: i32 = weight[index21] + 1;
                numArray14[index22] = num14;
              }
              let mut location2: i32 = this.game.Data.MapObj[0].HexObj[index19, index20].Location2;
              let mut type: i32 = this.game.Data.LocObj[location2].Type;
              SimpleList simpleList2;
              delivered1: i32;
              delivered2: i32;
              if (phase == 1)
              {
                simpleList2 = SimpleList::new();
                let mut unitCounter3: i32 = this.game.Data.UnitCounter;
                for (let mut index23: i32 = 0; index23 <= unitCounter3; index23 += 1)
                {
                  if (!this.game.Data.UnitObj[index23].UnitIsGiven & this.game.Data.UnitObj[index23].X > -1 & this.game.Data.UnitObj[index23].PreDef <= -1 && this.game.Data.UnitObj[index23].Regime == this.game.Data.Turn)
                  {
                    if (Strings.InStr(this.game.Data.UnitObj[index23].Name, "1818 H") > 0)
                      index23 = index23;
                    let mut num15: i32 =  this.game.Data.RuleVar[887] != 1.0 ? Math.Min(100, this.game.Data.UnitObj[index23].SOSupReqPercent) : Math.Min(100, this.game.HandyFunctionsObj.GetAggregatedSupplyRequest(index23));
                    if (depleteMode)
                    {
                      simpleList2.Add(index23, 1);
                      delivered1 =  Math.Round( (this.game.HandyFunctionsObj.UnitSupplyNeed(index23, true) * num15) / 100.0);
                      if (delivered1 > numArray2[index23])
                        numArray2[index23] = delivered1;
                      if (!flag1)
                        num10 += numArray2[index23];
                      delivered2 =  Math.Round( (Math.Max(0, this.game.HandyFunctionsObj.UnitMaximumFuelReserve(index23) - this.game.Data.UnitObj[index23].Fuel) * num15) / 100.0);
                      if (delivered2 > numArray6[index23])
                        numArray6[index23] = delivered2;
                      if (!flag1)
                        num11 += numArray6[index23];
                    }
                    else if (supplySourceMode)
                    {
                      simpleList2.Add(index23, 1);
                      delivered1 =  Math.Round( (this.game.HandyFunctionsObj.UnitSupplyNeed(index23, true) * num15) / 100.0);
                      if (delivered1 > numArray2[index23])
                        numArray2[index23] = delivered1;
                      this.game.Data.UnitObj[index23].SupplyInReq = numArray2[index23];
                      if (!flag1)
                        num10 += numArray2[index23];
                      delivered2 =  Math.Round( (Math.Max(0, this.game.HandyFunctionsObj.UnitMaximumFuelReserve(index23) - this.game.Data.UnitObj[index23].Fuel) * num15) / 100.0);
                      if (delivered2 > numArray6[index23])
                        numArray6[index23] = delivered2;
                      this.game.Data.UnitObj[index23].FuelRequested = numArray6[index23];
                      if (!flag1)
                        num11 += numArray6[index23];
                    }
                  }
                }
                let mut locCounter3: i32 = this.game.Data.LocCounter;
                for (let mut tid: i32 = 0; tid <= locCounter3; tid += 1)
                {
                  if (this.game.Data.MapObj[0].HexObj[this.game.Data.LocObj[tid].X, this.game.Data.LocObj[tid].Y].Regime == this.game.Data.Turn && this.game.Data.LocTypeObj[this.game.Data.LocObj[tid].Type].isSupplySource)
                  {
                    if (buildUpMode)
                    {
                      simpleList2.Add(tid, 1);
                      let mut num16: i32 = this.game.Data.LocTypeObj[type].maxSupply - this.game.Data.LocObj[location2].supply;
                      let mut num17: i32 =  Math.Round( (this.game.Data.LocTypeObj[type].maxSupply * this.game.Data.LocTypeObj[type].maxEvacuate) / 100.0);
                      if (num17 < num16)
                        num16 = num17;
                      if (num16 > numArray3[location2])
                        numArray3[location2] = num16;
                      this.game.Data.LocObj[location2].supplyReq = numArray3[location2];
                      if (!flag1)
                        num10 += numArray3[location2];
                      delivered1 = this.game.Data.LocTypeObj[type].maxFuel - this.game.Data.LocObj[location2].fuel;
                      let mut num18: i32 =  Math.Round( (this.game.Data.LocTypeObj[type].maxFuel * this.game.Data.LocTypeObj[type].maxEvacuate) / 100.0);
                      if (num18 < delivered1)
                        delivered1 = num18;
                      if (delivered1 > numArray7[location2])
                        numArray7[location2] = delivered1;
                      this.game.Data.LocObj[location2].fuelReq = numArray7[location2];
                      if (!flag1)
                        num11 += numArray7[location2];
                    }
                    else if (evacuateMode)
                    {
                      simpleList2.Add(tid, 1);
                      let mut val2: i32 = this.game.Data.LocTypeObj[type].maxSupply - this.game.Data.LocObj[location2].supply;
                      let mut num19: i32 =  Math.Round( (this.game.Data.LocTypeObj[type].maxSupply * this.game.Data.LocTypeObj[type].maxEvacuate) / 100.0);
                      if (num19 < val2)
                        val2 = num19;
                      if (val2 > numArray3[location2])
                        numArray3[location2] = val2;
                      if (!flag1)
                        num10 += Math.Max(0, val2);
                      delivered1 = this.game.Data.LocTypeObj[type].maxFuel - this.game.Data.LocObj[location2].fuel;
                      let mut num20: i32 =  Math.Round( (this.game.Data.LocTypeObj[type].maxFuel * this.game.Data.LocTypeObj[type].maxEvacuate) / 100.0);
                      if (num20 < delivered1)
                        delivered1 = num20;
                      if (delivered1 > numArray7[location2])
                        numArray7[location2] = delivered1;
                      if (!flag1)
                        num11 += numArray7[location2];
                    }
                  }
                }
              }
              if (simpleList2.Counter > -1)
              {
                let mut movetype: i32 =  Math.Round( this.game.Data.RuleVar[99]);
                index24: i32;
                if ( this.game.Data.RuleVar[455] > 0.0)
                {
                  this.game.HandyFunctionsObj.MakeFuzzyOwner(true, false, this.game.Data.Turn);
                  let mut mapWidth2: i32 = this.game.Data.MapObj[index24].MapWidth;
                  for (let mut index25: i32 = 0; index25 <= mapWidth2; index25 += 1)
                  {
                    let mut mapHeight: i32 = this.game.Data.MapObj[index24].MapHeight;
                    for (let mut index26: i32 = 0; index26 <= mapHeight; index26 += 1)
                      this.game.EditObj.TempValue2[index24].Value[index25, index26] = this.game.Data.MapObj[0].HexObj[index25, index26].Regime;
                  }
                  this.game.HandyFunctionsObj.MakeFuzzyOwner(false, false, this.game.Data.Turn);
                }
                let mut ap: i32 =  Math.Round( this.game.Data.RuleVar[3]);
                if (depleteMode)
                  ap = this.game.Data.LocTypeObj[type].supplyRange;
                CoordList coordList1 = !(supplySourceMode | buildUpMode | evacuateMode) ? this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.LocObj[location2].X, this.game.Data.LocObj[location2].Y, this.game.Data.LocObj[location2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules, blockLogisticalBonus: true) : this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.LocObj[location2].X, this.game.Data.LocObj[location2].Y, this.game.Data.LocObj[location2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                let mut counter2: i32 = simpleList2.Counter;
                for (let mut index27: i32 = 0; index27 <= counter2; index27 += 1)
                  simpleList2.Data2[index27] = !(buildUpMode | evacuateMode) ? (this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index27]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index27]].X, this.game.Data.UnitObj[simpleList2.Id[index27]].Y] <= ap ? 0 : 1) : (this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index27]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index27]].X, this.game.Data.LocObj[simpleList2.Id[index27]].Y] <= ap ? 0 : 1);
                let mut counter3: i32 = simpleList2.Counter;
                for (let mut index28: i32 = 0; index28 <= counter3; index28 += 1)
                {
                  str1: String = "Phase " + phase.ToString() + ". ";
                  str2: String = "Phase " + phase.ToString() + ". ";
                  if ( this.game.Data.RuleVar[455] > 0.0)
                  {
                    if (simpleList2.Data2[index28] == 0)
                    {
                      bool flag3 = true;
                      let mut num21: i32 = 0;
                      while (flag3)
                      {
                        num21 += 1;
                        flag3 = false;
                        CoordList coordList2 = CoordList::new();
                        Coordinate coordinate;
                        if (buildUpMode | evacuateMode)
                        {
                          coordinate.x = this.game.Data.LocObj[simpleList2.Id[index28]].X;
                          coordinate.y = this.game.Data.LocObj[simpleList2.Id[index28]].Y;
                        }
                        else
                        {
                          coordinate.x = this.game.Data.UnitObj[simpleList2.Id[index28]].X;
                          coordinate.y = this.game.Data.UnitObj[simpleList2.Id[index28]].Y;
                        }
                        if (coordinate.x == 41 & coordinate.y == 27)
                          index1 = index1;
                        coordinate.onmap = true;
                        while (coordinate.onmap)
                        {
                          coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y];
                          if (coordinate.onmap)
                            coordList2.AddCoord(coordinate.x, coordinate.y, 0);
                        }
                        if (coordList2.counter > -1)
                        {
                          flag3 = false;
                          for (let mut counter4: i32 = coordList2.counter; counter4 >= 0; counter4 += -1)
                          {
                            if (this.game.EditObj.TempValue2[index24].Value[coordList2.coord[counter4].x, coordList2.coord[counter4].y] != this.game.Data.Turn)
                            {
                              this.game.Data.MapObj[0].HexObj[coordList2.coord[counter4].x, coordList2.coord[counter4].y].FuzzyBlock = 1;
                              this.game.HandyFunctionsObj.MakeFuzzyOwner(false, false, this.game.Data.Turn);
                              coordList1 = !(supplySourceMode | buildUpMode | evacuateMode) ? this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.LocObj[location2].X, this.game.Data.LocObj[location2].Y, this.game.Data.LocObj[location2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules, blockLogisticalBonus: true) : this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.LocObj[location2].X, this.game.Data.LocObj[location2].Y, this.game.Data.LocObj[location2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                              flag3 = true;
                              break;
                            }
                          }
                        }
                        if (!flag3)
                        {
                          if (buildUpMode | evacuateMode)
                          {
                            if (this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index28]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y] > ap)
                            {
                              simpleList2.Data2[index28] = 1;
                              str1 += "Enemy forces blocking supply path. ";
                              str2 += "Enemy forces blocking supply path. ";
                            }
                          }
                          else if (this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index28]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y] > ap)
                          {
                            simpleList2.Data2[index28] = 1;
                            str1 += "Enemy forces blocking supply path. ";
                            str2 += "Enemy forces blocking supply path. ";
                          }
                        }
                        if (num21 > 999)
                        {
                          flag3 = false;
                          simpleList2.Data2[index28] = 1;
                        }
                      }
                    }
                    else
                    {
                      str1 += "No supply path possible. ";
                      str2 += "No supply path possible. ";
                    }
                  }
                  index29: i32;
                  if (simpleList2.Id[index28] == 1564)
                    index29 = index29;
                  request1: i32;
                  request2: i32;
                  if (simpleList2.Data2[index28] == 0)
                  {
                    index29 = simpleList2.Id[index28];
                    if (depleteMode)
                      delivered1 = numArray2[index29];
                    else if (supplySourceMode)
                      delivered1 = numArray2[index29];
                    else if (buildUpMode)
                      delivered1 = numArray3[location2];
                    else if (evacuateMode)
                    {
                      delivered1 =  Math.Round( (this.game.Data.LocTypeObj[type].maxSupply * this.game.Data.LocTypeObj[type].maxEvacuate) / 100.0);
                      if (this.game.Data.LocObj[location2].supply < delivered1)
                        delivered1 = this.game.Data.LocObj[location2].supply;
                    }
                    if (index29 == 124)
                      index29 = index29;
                    if (depleteMode)
                      delivered2 = numArray6[index29];
                    else if (supplySourceMode)
                      delivered2 = numArray6[index29];
                    else if (buildUpMode)
                      delivered2 = numArray7[location2];
                    else if (evacuateMode)
                    {
                      delivered2 =  Math.Round( (this.game.Data.LocTypeObj[type].maxFuel * this.game.Data.LocTypeObj[type].maxEvacuate) / 100.0);
                      if (this.game.Data.LocObj[location2].fuel < delivered2)
                        delivered2 = this.game.Data.LocObj[location2].fuel;
                    }
                    request1 = delivered1;
                    request2 = delivered2;
                    let mut num22: i32 = 100;
                    str3: String;
                    str4: String;
                    if (buildUpMode | evacuateMode)
                    {
                      if ( this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index28]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y] >  ap * ( this.game.Data.RuleVar[51] /  this.game.Data.RuleVar[3]))
                        num22 = 75;
                      if ( this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index28]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y] >  ap * ( this.game.Data.RuleVar[52] /  this.game.Data.RuleVar[3]))
                        num22 = 50;
                      if ( this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index28]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y] >  ap * ( this.game.Data.RuleVar[53] /  this.game.Data.RuleVar[3]))
                        num22 = 25;
                      if (this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index28]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y] > ap)
                        num22 = 0;
                      str3 = str1 + "Logistics percentage based on " + this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index28]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y].ToString() + " AP distance is " + num22.ToString() + "%. ";
                      str4 = str2 + "Logistics percentage based on " + this.game.EditObj.TempValue[this.game.Data.LocObj[simpleList2.Id[index28]].Map].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y].ToString() + " AP distance is " + num22.ToString() + "%. ";
                    }
                    else
                    {
                      if ( this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index28]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y] >  ap * ( this.game.Data.RuleVar[51] /  this.game.Data.RuleVar[3]))
                        num22 = 75;
                      if ( this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index28]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y] >  ap * ( this.game.Data.RuleVar[52] /  this.game.Data.RuleVar[3]))
                        num22 = 50;
                      if ( this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index28]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y] >  ap * ( this.game.Data.RuleVar[53] /  this.game.Data.RuleVar[3]))
                        num22 = 25;
                      if (this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index28]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y] > ap)
                        num22 = 0;
                      str3 = str1 + "Logistics percentage based on " + this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index28]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y].ToString() + " AP distance is " + num22.ToString() + "%. ";
                      str4 = str2 + "Logistics percentage based on " + this.game.EditObj.TempValue[this.game.Data.UnitObj[simpleList2.Id[index28]].Map].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y].ToString() + " AP distance is " + num22.ToString() + "%. ";
                    }
                    if ( this.game.Data.RuleVar[434] > 0.0 & (depleteMode | supplySourceMode))
                      num22 = this.game.Data.UnitObj[index29].SOSupReqPercent > num22 ?  Math.Round(Math.Floor( (100 * num22) /  this.game.Data.UnitObj[index29].SOSupReqPercent)) : 100;
                    str5: String = str3 + "Logistics percentage after Standing Order modifier is " + num22.ToString() + "%. ";
                    str6: String = str4 + "Logistics percentage after Standing Order modifier is " + num22.ToString() + "%. ";
                    delivered1 =  Math.Round(Conversion.Int(Math.Floor( (delivered1 * num22) / 100.0)));
                    if ( this.game.Data.RuleVar[489] > 0.0 & supplySourceMode & delivered2 > 0)
                    {
                      if (phase == 1)
                      {
                        if (num22 >= 100 | num22 < 1)
                          numArray9[index29] = -1;
                        else if (new Random(index29 *  Math.Round( this.game.Data.GameID / 10000.0) * this.game.Data.Round).Next(0, 100) < num22)
                        {
                          numArray9[index29] = 100;
                          str6 += "Binary fuel delivery rule roll succesfull. ";
                        }
                        else
                        {
                          str6 += "Binary fuel delivery rule roll failed. ";
                          numArray9[index29] = 0;
                        }
                      }
                      else if (numArray9[index29] == 0)
                        index24 = index24;
                      delivered2 = numArray9[index29] != -1 ?  Math.Round(Conversion.Int(Math.Floor( (delivered2 * numArray9[index29]) / 100.0))) :  Math.Round(Conversion.Int(Math.Floor( (delivered2 * num22) / 100.0)));
                    }
                    else
                      delivered2 =  Math.Round(Conversion.Int(Math.Floor( (delivered2 * num22) / 100.0)));
                    if (depleteMode)
                      delivered1 =  Math.Round(Math.Ceiling( (delivered1 * phase) /  num8)) - numArray1[index29];
                    else if (supplySourceMode)
                      delivered1 =  Math.Round(Math.Ceiling( (delivered1 * phase) /  num8)) - numArray1[index29];
                    else if (buildUpMode)
                      delivered1 =  Math.Round(Math.Ceiling( (delivered1 * phase) /  num8)) - numArray4[location2];
                    else if (evacuateMode)
                      delivered1 =  Math.Round(Math.Ceiling( (delivered1 * phase) /  num8)) - numArray4[location2];
                    if (depleteMode)
                      delivered2 =  Math.Round(Math.Ceiling( (delivered2 * phase) /  num8)) - numArray5[index29];
                    else if (supplySourceMode)
                      delivered2 =  Math.Round(Math.Ceiling( (delivered2 * phase) /  num8)) - numArray5[index29];
                    else if (buildUpMode)
                      delivered2 =  Math.Round(Math.Ceiling( (delivered2 * phase) /  num8)) - numArray8[location2];
                    else if (evacuateMode)
                      delivered2 =  Math.Round(Math.Ceiling( (delivered2 * phase) /  num8)) - numArray8[location2];
                    if (depleteMode)
                      request1 =  Math.Round(Math.Ceiling( (request1 * phase) /  num8)) - numArray1[index29];
                    else if (supplySourceMode)
                      request1 =  Math.Round(Math.Ceiling( (request1 * phase) /  num8)) - numArray1[index29];
                    else if (buildUpMode)
                      request1 =  Math.Round(Math.Ceiling( (request1 * phase) /  num8)) - numArray4[location2];
                    else if (evacuateMode)
                      request1 =  Math.Round(Math.Ceiling( (request1 * phase) /  num8)) - numArray4[location2];
                    if (depleteMode)
                      request2 =  Math.Round(Math.Ceiling( (request2 * phase) /  num8)) - numArray5[index29];
                    else if (supplySourceMode)
                      request2 =  Math.Round(Math.Ceiling( (request2 * phase) /  num8)) - numArray5[index29];
                    else if (buildUpMode)
                      request2 =  Math.Round(Math.Ceiling( (request2 * phase) /  num8)) - numArray8[location2];
                    else if (evacuateMode)
                      request2 =  Math.Round(Math.Ceiling( (request2 * phase) /  num8)) - numArray8[location2];
                    if (request1 > 0)
                      str5 = str5 + "Request is " + request1.ToString() + " supply. ";
                    if (request2 > 0)
                      str6 = str6 + "Request is " + request2.ToString() + " fuel. ";
                    if (depleteMode | supplySourceMode)
                    {
                      if (this.game.Data.UnitObj[index29].Supply < 0)
                        this.game.Data.UnitObj[index29].Supply = 0;
                      if (this.game.Data.UnitObj[index29].Fuel < 0)
                        this.game.Data.UnitObj[index29].Fuel = 0;
                    }
                    if (depleteMode | supplySourceMode)
                    {
                      let mut num23: i32 =  Math.Round(  Math.Round(Math.Ceiling( ((this.game.Data.LocObj[location2].supply + this.game.Data.LocObj[location2].supplyOutUnits) * phase) /  num8)) -  (this.game.Data.LocObj[location2].supplyOutUnits * phase) /  num8);
                      if (num23 > this.game.Data.LocObj[location2].supply)
                        num23 = this.game.Data.LocObj[location2].supply;
                      if (num23 < 0)
                        num23 = 0;
                      if (num23 < delivered1)
                        delivered1 = num23;
                      index1 =  Math.Round(  Math.Round(Math.Ceiling( ((this.game.Data.LocObj[location2].fuel + this.game.Data.LocObj[location2].fuelOutUnits) * phase) /  num8)) -  (this.game.Data.LocObj[location2].fuelOutUnits * phase) /  num8);
                      if (index1 > this.game.Data.LocObj[location2].fuel)
                        index1 = this.game.Data.LocObj[location2].fuel;
                      if (index1 < 0)
                        index1 = 0;
                      if (index1 < delivered2)
                        delivered2 = index1;
                    }
                    if (buildUpMode)
                    {
                      let mut supply: i32 = this.game.Data.LocObj[index29].supply;
                      if (supply < delivered1)
                        delivered1 = supply;
                      let mut fuel: i32 = this.game.Data.LocObj[index29].fuel;
                      if (fuel < delivered2)
                        delivered2 = fuel;
                    }
                    if (0 > delivered1)
                      delivered1 = 0;
                    if (0 > delivered2)
                      delivered2 = 0;
                    if (delivered1 > 0 | delivered2 > 0)
                    {
                      index1 = delivered1;
                      let mut num24: i32 =  Math.Round(Math.Floor( index1 *  this.game.Data.RuleVar[33]));
                      if ( this.game.Data.RuleVar[33] < 1.0 && index1 >  Math.Round( ( num24 / this.game.Data.RuleVar[33])))
                      {
                        let mut num25: i32 = index1 -  Math.Round( ( num24 / this.game.Data.RuleVar[33]));
                        let mut num26: i32 =  Math.Round(Math.Ceiling(1.0 /  this.game.Data.RuleVar[33]));
                        if (DrawMod.RandyNumber.Next(1, num26 + 1) <= num25)
                          num24 += 1;
                      }
                      let mut num27: i32 = num24;
                      let mut num28: i32 = 0;
                      if (delivered2 > 0)
                      {
                        if ( this.game.Data.RuleVar[488] > 0.0)
                        {
                          index1 = delivered2;
                          num28 =  Math.Round(Math.Floor( index1 *  this.game.Data.RuleVar[488]));
                          if ( this.game.Data.RuleVar[488] < 1.0 && index1 >  Math.Round( ( num28 / this.game.Data.RuleVar[488])))
                          {
                            let mut num29: i32 = index1 -  Math.Round( ( num28 / this.game.Data.RuleVar[488]));
                            let mut num30: i32 =  Math.Round(Math.Ceiling(1.0 /  this.game.Data.RuleVar[488]));
                            if (DrawMod.RandyNumber.Next(1, num30 + 1) <= num29)
                              num28 += 1;
                          }
                        }
                        else
                        {
                          index1 = delivered2;
                          num28 =  Math.Round(Math.Floor( index1 *  this.game.Data.RuleVar[33]));
                          if ( this.game.Data.RuleVar[33] < 1.0 && index1 >  Math.Round( ( num28 / this.game.Data.RuleVar[33])))
                          {
                            let mut num31: i32 = index1 -  Math.Round( ( num28 / this.game.Data.RuleVar[33]));
                            let mut num32: i32 =  Math.Round(Math.Ceiling(1.0 /  this.game.Data.RuleVar[33]));
                            if (DrawMod.RandyNumber.Next(1, num32 + 1) <= num31)
                              num28 += 1;
                          }
                        }
                      }
                      let mut num33: i32 = num27 + num28;
                      if (delivered1 > request1)
                        delivered1 = request1;
                      if (delivered2 > request2)
                        delivered2 = request2;
                      if (delivered1 > 0)
                      {
                        str7: String = str5 + delivered1.ToString() + " supply has been given ";
                        if (depleteMode)
                        {
                          texty: String = str7 + "from supply base " + this.game.Data.LocObj[location2].X.ToString() + "," + this.game.Data.LocObj[location2].Y.ToString() + ". ";
                          int[] numArray15 = numArray1;
                          int[] numArray16 = numArray15;
                          let mut index30: i32 = index29;
                          let mut index31: i32 = index30;
                          let mut num34: i32 = numArray15[index30] + delivered1;
                          numArray16[index31] = num34;
                          this.game.Data.UnitObj[index29].Supply += delivered1;
                          UnitClass[] unitObj = this.game.Data.UnitObj;
                          UnitClass[] unitClassArray = unitObj;
                          let mut index32: i32 = index29;
                          let mut index33: i32 = index32;
                          unitClassArray[index33].supplyBaseSupplyIn = unitObj[index32].supplyBaseSupplyIn + delivered1;
                          LocationClass[] locObj = this.game.Data.LocObj;
                          LocationClass[] locationClassArray = locObj;
                          let mut index34: i32 = location2;
                          let mut index35: i32 = index34;
                          locationClassArray[index35].supplyOutUnits = locObj[index34].supplyOutUnits + delivered1;
                          this.game.Data.LocObj[location2].supply -= delivered1;
                          num12 += delivered1;
                          int[] numArray17 = numArray10;
                          int[] numArray18 = numArray17;
                          let mut index36: i32 = location2;
                          let mut index37: i32 = index36;
                          let mut num35: i32 = numArray17[index36] + delivered1;
                          numArray18[index37] = num35;
                          if (phase > 2 && this.game.Data.UnitObj[index29].supplyX == -1 & this.game.EditObj.TempCameFrom[0].Value[this.game.Data.UnitObj[index29].X, this.game.Data.UnitObj[index29].Y].onmap)
                          {
                            this.game.Data.UnitObj[index29].supplyX = this.game.EditObj.TempCameFrom[0].Value[this.game.Data.UnitObj[index29].X, this.game.Data.UnitObj[index29].Y].x;
                            this.game.Data.UnitObj[index29].supplyY = this.game.EditObj.TempCameFrom[0].Value[this.game.Data.UnitObj[index29].X, this.game.Data.UnitObj[index29].Y].y;
                          }
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical * 10 + this.game.Data.UnitObj[index29].HistoricalSubPart, 0, 7, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty);
                        }
                        else if (supplySourceMode)
                        {
                          texty: String = str7 + "from supply source " + this.game.Data.LocObj[location2].X.ToString() + "," + this.game.Data.LocObj[location2].Y.ToString() + ". ";
                          if (this.game.Data.UnitObj[index29].supplyX == -1 & this.game.EditObj.TempCameFrom[0].Value[this.game.Data.UnitObj[index29].X, this.game.Data.UnitObj[index29].Y].onmap)
                          {
                            this.game.Data.UnitObj[index29].supplyX = this.game.EditObj.TempCameFrom[0].Value[this.game.Data.UnitObj[index29].X, this.game.Data.UnitObj[index29].Y].x;
                            this.game.Data.UnitObj[index29].supplyY = this.game.EditObj.TempCameFrom[0].Value[this.game.Data.UnitObj[index29].X, this.game.Data.UnitObj[index29].Y].y;
                          }
                          int[] numArray19 = numArray1;
                          int[] numArray20 = numArray19;
                          let mut index38: i32 = index29;
                          let mut index39: i32 = index38;
                          let mut num36: i32 = numArray19[index38] + delivered1;
                          numArray20[index39] = num36;
                          num12 += delivered1;
                          int[] numArray21 = numArray10;
                          int[] numArray22 = numArray21;
                          let mut index40: i32 = location2;
                          let mut index41: i32 = index40;
                          let mut num37: i32 = numArray21[index40] + delivered1;
                          numArray22[index41] = num37;
                          this.game.Data.UnitObj[index29].Supply += delivered1;
                          UnitClass[] unitObj = this.game.Data.UnitObj;
                          UnitClass[] unitClassArray = unitObj;
                          let mut index42: i32 = index29;
                          let mut index43: i32 = index42;
                          unitClassArray[index43].SupplyIn = unitObj[index42].SupplyIn + delivered1;
                          LocationClass[] locObj = this.game.Data.LocObj;
                          LocationClass[] locationClassArray = locObj;
                          let mut index44: i32 = location2;
                          let mut index45: i32 = index44;
                          locationClassArray[index45].supplyOutUnits = locObj[index44].supplyOutUnits + delivered1;
                          this.game.Data.LocObj[location2].supply -= delivered1;
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical * 10 + this.game.Data.UnitObj[index29].HistoricalSubPart, 0, 1, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty);
                        }
                        else if (evacuateMode)
                        {
                          texty: String = str7 + "from evacuation of supply base " + this.game.Data.LocObj[location2].X.ToString() + "," + this.game.Data.LocObj[location2].Y.ToString() + ". ";
                          int[] numArray23 = numArray4;
                          int[] numArray24 = numArray23;
                          let mut index46: i32 = location2;
                          let mut index47: i32 = index46;
                          let mut num38: i32 = numArray23[index46] + delivered1;
                          numArray24[index47] = num38;
                          this.game.Data.LocObj[index29].supply += delivered1;
                          LocationClass[] locObj5 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray5 = locObj5;
                          let mut index48: i32 = index29;
                          let mut index49: i32 = index48;
                          locationClassArray5[index49].supplyEvacuated = locObj5[index48].supplyEvacuated + delivered1;
                          LocationClass[] locObj6 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray6 = locObj6;
                          let mut index50: i32 = location2;
                          let mut index51: i32 = index50;
                          locationClassArray6[index51].supplyEvacuated = locObj6[index50].supplyEvacuated + delivered1;
                          this.game.Data.LocObj[location2].supply -= delivered1;
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 5, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty);
                          num12 += delivered1;
                          int[] numArray25 = numArray10;
                          int[] numArray26 = numArray25;
                          let mut index52: i32 = location2;
                          let mut index53: i32 = index52;
                          let mut num39: i32 = numArray25[index52] + delivered1;
                          numArray26[index53] = num39;
                          int[] numArray27 = numArray12;
                          int[] numArray28 = numArray27;
                          let mut index54: i32 = index29;
                          let mut index55: i32 = index54;
                          let mut num40: i32 = numArray27[index54] + delivered1;
                          numArray28[index55] = num40;
                        }
                        else if (buildUpMode)
                        {
                          texty: String = str7 + "from supply source " + this.game.Data.LocObj[index29].X.ToString() + "," + this.game.Data.LocObj[index29].Y.ToString() + ". ";
                          int[] numArray29 = numArray4;
                          int[] numArray30 = numArray29;
                          let mut index56: i32 = location2;
                          let mut index57: i32 = index56;
                          let mut num41: i32 = numArray29[index56] + delivered1;
                          numArray30[index57] = num41;
                          this.game.Data.LocObj[index29].supply -= delivered1;
                          LocationClass[] locObj7 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray7 = locObj7;
                          let mut index58: i32 = index29;
                          let mut index59: i32 = index58;
                          locationClassArray7[index59].supplyOutBases = locObj7[index58].supplyOutBases + delivered1;
                          LocationClass[] locObj8 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray8 = locObj8;
                          let mut index60: i32 = location2;
                          let mut index61: i32 = index60;
                          locationClassArray8[index61].supplyIn = locObj8[index60].supplyIn + delivered1;
                          this.game.Data.LocObj[location2].supply += delivered1;
                          num12 += delivered1;
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 11, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty);
                          int[] numArray31 = numArray10;
                          int[] numArray32 = numArray31;
                          let mut index62: i32 = index29;
                          let mut index63: i32 = index62;
                          let mut num42: i32 = numArray31[index62] + delivered1;
                          numArray32[index63] = num42;
                          int[] numArray33 = numArray12;
                          int[] numArray34 = numArray33;
                          let mut index64: i32 = location2;
                          let mut index65: i32 = index64;
                          let mut num43: i32 = numArray33[index64] + delivered1;
                          numArray34[index65] = num43;
                        }
                      }
                      if (delivered2 > 0)
                      {
                        str8: String = str6 + delivered2.ToString() + " fuel has been given ";
                        if (depleteMode)
                        {
                          texty: String = str8 + "from supply base " + this.game.Data.LocObj[location2].X.ToString() + "," + this.game.Data.LocObj[location2].Y.ToString() + ". ";
                          int[] numArray35 = numArray5;
                          int[] numArray36 = numArray35;
                          let mut index66: i32 = index29;
                          let mut index67: i32 = index66;
                          let mut num44: i32 = numArray35[index66] + delivered2;
                          numArray36[index67] = num44;
                          this.game.Data.UnitObj[index29].Fuel += delivered2;
                          UnitClass[] unitObj = this.game.Data.UnitObj;
                          UnitClass[] unitClassArray = unitObj;
                          let mut index68: i32 = index29;
                          let mut index69: i32 = index68;
                          unitClassArray[index69].supplyBaseFuelIn = unitObj[index68].supplyBaseFuelIn + delivered2;
                          LocationClass[] locObj = this.game.Data.LocObj;
                          LocationClass[] locationClassArray = locObj;
                          let mut index70: i32 = location2;
                          let mut index71: i32 = index70;
                          locationClassArray[index71].fuelOutUnits = locObj[index70].fuelOutUnits + delivered2;
                          num13 += delivered2;
                          this.game.Data.LocObj[location2].fuel -= delivered2;
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical * 10 + this.game.Data.UnitObj[index29].HistoricalSubPart, 0, 8, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty);
                          int[] numArray37 = numArray11;
                          int[] numArray38 = numArray37;
                          let mut index72: i32 = location2;
                          let mut index73: i32 = index72;
                          let mut num45: i32 = numArray37[index72] + delivered2;
                          numArray38[index73] = num45;
                        }
                        else if (supplySourceMode)
                        {
                          texty: String = str8 + "from supply source " + this.game.Data.LocObj[location2].X.ToString() + "," + this.game.Data.LocObj[location2].Y.ToString() + ". ";
                          int[] numArray39 = numArray5;
                          int[] numArray40 = numArray39;
                          let mut index74: i32 = index29;
                          let mut index75: i32 = index74;
                          let mut num46: i32 = numArray39[index74] + delivered2;
                          numArray40[index75] = num46;
                          this.game.Data.UnitObj[index29].Fuel += delivered2;
                          UnitClass[] unitObj = this.game.Data.UnitObj;
                          UnitClass[] unitClassArray = unitObj;
                          let mut index76: i32 = index29;
                          let mut index77: i32 = index76;
                          unitClassArray[index77].FuelReceived = unitObj[index76].FuelReceived + delivered2;
                          LocationClass[] locObj = this.game.Data.LocObj;
                          LocationClass[] locationClassArray = locObj;
                          let mut index78: i32 = location2;
                          let mut index79: i32 = index78;
                          locationClassArray[index79].fuelOutUnits = locObj[index78].fuelOutUnits + delivered2;
                          num13 += delivered2;
                          this.game.Data.LocObj[location2].fuel -= delivered2;
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical * 10 + this.game.Data.UnitObj[index29].HistoricalSubPart, 0, 2, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty);
                          int[] numArray41 = numArray11;
                          int[] numArray42 = numArray41;
                          let mut index80: i32 = location2;
                          let mut index81: i32 = index80;
                          let mut num47: i32 = numArray41[index80] + delivered2;
                          numArray42[index81] = num47;
                        }
                        else if (evacuateMode)
                        {
                          texty: String = str8 + "from evacuation of supply base " + this.game.Data.LocObj[location2].X.ToString() + "," + this.game.Data.LocObj[location2].Y.ToString() + ". ";
                          int[] numArray43 = numArray8;
                          int[] numArray44 = numArray43;
                          let mut index82: i32 = location2;
                          let mut index83: i32 = index82;
                          let mut num48: i32 = numArray43[index82] + delivered2;
                          numArray44[index83] = num48;
                          this.game.Data.LocObj[index29].fuel += delivered2;
                          LocationClass[] locObj9 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray9 = locObj9;
                          let mut index84: i32 = index29;
                          let mut index85: i32 = index84;
                          locationClassArray9[index85].fuelEvacuated = locObj9[index84].fuelEvacuated + delivered2;
                          LocationClass[] locObj10 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray10 = locObj10;
                          let mut index86: i32 = location2;
                          let mut index87: i32 = index86;
                          locationClassArray10[index87].fuelEvacuated = locObj10[index86].fuelEvacuated + delivered2;
                          num13 += delivered2;
                          this.game.Data.LocObj[location2].fuel -= delivered2;
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 6, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty);
                          int[] numArray45 = numArray11;
                          int[] numArray46 = numArray45;
                          let mut index88: i32 = location2;
                          let mut index89: i32 = index88;
                          let mut num49: i32 = numArray45[index88] + delivered2;
                          numArray46[index89] = num49;
                          int[] numArray47 = numArray13;
                          int[] numArray48 = numArray47;
                          let mut index90: i32 = index29;
                          let mut index91: i32 = index90;
                          let mut num50: i32 = numArray47[index90] + delivered2;
                          numArray48[index91] = num50;
                        }
                        else if (buildUpMode)
                        {
                          texty: String = str8 + "from supply source " + this.game.Data.LocObj[index29].X.ToString() + "," + this.game.Data.LocObj[index29].Y.ToString() + ". ";
                          int[] numArray49 = numArray8;
                          int[] numArray50 = numArray49;
                          let mut index92: i32 = location2;
                          let mut index93: i32 = index92;
                          let mut num51: i32 = numArray49[index92] + delivered2;
                          numArray50[index93] = num51;
                          this.game.Data.LocObj[index29].fuel -= delivered2;
                          LocationClass[] locObj11 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray11 = locObj11;
                          let mut index94: i32 = index29;
                          let mut index95: i32 = index94;
                          locationClassArray11[index95].fuelOutBases = locObj11[index94].fuelOutBases + delivered2;
                          LocationClass[] locObj12 = this.game.Data.LocObj;
                          LocationClass[] locationClassArray12 = locObj12;
                          let mut index96: i32 = location2;
                          let mut index97: i32 = index96;
                          locationClassArray12[index97].fuelIn = locObj12[index96].fuelIn + delivered2;
                          num13 += delivered2;
                          this.game.Data.LocObj[location2].fuel += delivered2;
                          this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 12, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty);
                          int[] numArray51 = numArray11;
                          int[] numArray52 = numArray51;
                          let mut index98: i32 = index29;
                          let mut index99: i32 = index98;
                          let mut num52: i32 = numArray51[index98] + delivered2;
                          numArray52[index99] = num52;
                          int[] numArray53 = numArray13;
                          int[] numArray54 = numArray53;
                          let mut index100: i32 = location2;
                          let mut index101: i32 = index100;
                          let mut num53: i32 = numArray53[index100] + delivered2;
                          numArray54[index101] = num53;
                        }
                      }
                      if (useTrafficRules & num33 > 0)
                      {
                        Coordinate coordinate1;
                        if (evacuateMode | buildUpMode)
                        {
                          coordinate1.x = this.game.Data.LocObj[index29].X;
                          coordinate1.y = this.game.Data.LocObj[index29].Y;
                        }
                        else
                        {
                          coordinate1.x = this.game.Data.UnitObj[index29].X;
                          coordinate1.y = this.game.Data.UnitObj[index29].Y;
                        }
                        bool flag4 = false;
                        let mut index102: i32 = 0;
                        while (this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y].onmap)
                        {
                          let mut num54: i32 = index102 + 1;
                          Coordinate coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                          index1 = this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
                          let mut index103: i32 = index1 + 3;
                          if (index103 > 5)
                            index103 -= 6;
                          let mut liSpoint1: i32 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index1];
                          let mut liSpoint2: i32 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index103];
                          int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints;
                          int[] numArray55 = liSpoints1;
                          let mut index104: i32 = index1;
                          let mut index105: i32 = index104;
                          let mut num55: i32 = liSpoints1[index104] + num33;
                          numArray55[index105] = num55;
                          int[] liSpoints2 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints;
                          int[] numArray56 = liSpoints2;
                          let mut index106: i32 = index103;
                          let mut index107: i32 = index106;
                          let mut num56: i32 = liSpoints2[index106] + num33;
                          numArray56[index107] = num56;
                          let mut index108: i32 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RoadType[index1];
                          if (index108 > -1 && this.game.Data.RoadTypeObj[index108].trafficPoints > 0)
                          {
                            let mut num57: i32 =  Math.Round(Math.Floor( (liSpoint1 * 1) /  this.game.Data.RoadTypeObj[index108].trafficPoints));
                            if ( Math.Round(Math.Floor( (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index1] * 1) /  this.game.Data.RoadTypeObj[index108].trafficPoints)) > num57)
                              flag4 = true;
                          }
                          index102 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].RoadType[index103];
                          if (index102 > -1 && this.game.Data.RoadTypeObj[index102].trafficPoints > 0)
                          {
                            let mut num58: i32 =  Math.Round(Math.Floor( (liSpoint2 * 1) /  this.game.Data.RoadTypeObj[index102].trafficPoints));
                            if ( Math.Round(Math.Floor( (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index103] * 1) /  this.game.Data.RoadTypeObj[index102].trafficPoints)) > num58)
                              flag4 = true;
                          }
                          coordinate1 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                          if (index102 > 999)
                            break;
                        }
                        if (flag4)
                          coordList1 = !(supplySourceMode | buildUpMode | evacuateMode) ? this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.LocObj[location2].X, this.game.Data.LocObj[location2].Y, this.game.Data.LocObj[location2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules, blockLogisticalBonus: true) : this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.LocObj[location2].X, this.game.Data.LocObj[location2].Y, this.game.Data.LocObj[location2].Map, allowshoredrop: true, SeaBlock: true, useTrafficRules: useTrafficRules);
                      }
                    }
                  }
                  else
                  {
                    if (depleteMode)
                      delivered1 = numArray2[index29];
                    else if (supplySourceMode)
                      delivered1 = numArray2[index29];
                    else if (buildUpMode)
                      delivered1 = numArray3[location2];
                    else if (evacuateMode)
                      delivered1 = numArray3[location2];
                    if (index29 == 124)
                      index29 = index29;
                    if (depleteMode)
                      delivered2 = numArray6[index29];
                    else if (supplySourceMode)
                      delivered2 = numArray6[index29];
                    else if (buildUpMode)
                      delivered2 = numArray7[location2];
                    else if (evacuateMode)
                      delivered2 = numArray7[location2];
                    request1 = delivered1;
                    request2 = delivered2;
                    texty1: String = str1 + "Nothing delivered. ";
                    texty2: String = str2 + "Nothing delivered. ";
                    if (request2 > 0)
                    {
                      if (depleteMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical, 0, 8, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty2);
                      else if (supplySourceMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical, 0, 2, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty2);
                      else if (evacuateMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 6, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty2);
                      else if (buildUpMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 12, phase, request2, delivered2, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty2);
                    }
                    if (request1 > 0)
                    {
                      if (depleteMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical, 0, 8, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty1);
                      else if (supplySourceMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.UnitObj[index29].Historical, 0, 2, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.UnitObj[simpleList2.Id[index28]].X, this.game.Data.UnitObj[simpleList2.Id[index28]].Y], texty1);
                      else if (evacuateMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 6, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty1);
                      else if (buildUpMode)
                        this.AddSupplyLog(0, this.game.Data.LocObj[location2].ID, this.game.Data.LocObj[index29].ID, 0, 12, phase, request1, delivered1, this.game.EditObj.TempValue[0].Value[this.game.Data.LocObj[simpleList2.Id[index28]].X, this.game.Data.LocObj[simpleList2.Id[index28]].Y], texty1);
                    }
                  }
                }
              }
              flag1 = true;
            }
          }
        }
        if (!( this.game.Data.RuleVar[957] > 0.0 &  this.game.Data.RuleVar[499] > 0.0))
          return;
        let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[957]));
        let mut num59: i32 = 0;
        let mut num60: i32 = 0;
        let mut num61: i32 = 0;
        let mut num62: i32 = 0;
        let mut locCounter4: i32 = this.game.Data.LocCounter;
        for (let mut index109: i32 = 0; index109 <= locCounter4; index109 += 1)
        {
          if (numArray10[index109] > 0)
            num59 += numArray10[index109];
          if (numArray11[index109] > 0)
            num60 += numArray11[index109];
          if (numArray12[index109] > 0)
            num61 += numArray12[index109];
          if (numArray13[index109] > 0)
            num62 += numArray13[index109];
          if (this.game.Data.MapObj[0].HexObj[this.game.Data.LocObj[index109].X, this.game.Data.LocObj[index109].Y].Regime == this.game.Data.Turn && this.game.Data.LocTypeObj[this.game.Data.LocObj[index109].Type].isSupplySource)
          {
            if (supplySourceMode)
              this.game.Data.StringListObj[stringListById].AddRowWithData("Supply source " + this.game.Data.LocObj[index109].X.ToString() + "," + this.game.Data.LocObj[index109].Y.ToString() + " : Supply", this.game.Data.Turn.ToString(), "Sent to units", this.game.Data.Round.ToString(), numArray10[index109].ToString());
            if (buildUpMode)
              this.game.Data.StringListObj[stringListById].AddRowWithData("Supply source " + this.game.Data.LocObj[index109].X.ToString() + "," + this.game.Data.LocObj[index109].Y.ToString() + " : Supply", this.game.Data.Turn.ToString(), "Sent to supply base", this.game.Data.Round.ToString(), numArray10[index109].ToString());
            if (supplySourceMode)
              this.game.Data.StringListObj[stringListById].AddRowWithData("Supply source " + this.game.Data.LocObj[index109].X.ToString() + "," + this.game.Data.LocObj[index109].Y.ToString() + " : Fuel", this.game.Data.Turn.ToString(), "Sent to units", this.game.Data.Round.ToString(), numArray11[index109].ToString());
            if (buildUpMode)
              this.game.Data.StringListObj[stringListById].AddRowWithData("Supply source " + this.game.Data.LocObj[index109].X.ToString() + "," + this.game.Data.LocObj[index109].Y.ToString() + " : Fuel", this.game.Data.Turn.ToString(), "Sent to supply base", this.game.Data.Round.ToString(), numArray11[index109].ToString());
          }
        }
        if (depleteMode)
        {
          this.game.Data.StringListObj[stringListById].AddRowWithData("Unit supply", this.game.Data.Turn.ToString(), "Received from supply base", this.game.Data.Round.ToString(), num12.ToString());
          this.game.Data.StringListObj[stringListById].AddRowWithData("Unit fuel", this.game.Data.Turn.ToString(), "Received from supply base", this.game.Data.Round.ToString(), num13.ToString());
        }
        else if (supplySourceMode)
        {
          this.game.Data.StringListObj[stringListById].AddRowWithData("Unit supply", this.game.Data.Turn.ToString(), "Requested", this.game.Data.Round.ToString(), num10.ToString());
          this.game.Data.StringListObj[stringListById].AddRowWithData("Unit supply", this.game.Data.Turn.ToString(), "Received from supply source", this.game.Data.Round.ToString(), num12.ToString());
          this.game.Data.StringListObj[stringListById].AddRowWithData("Unit fuel", this.game.Data.Turn.ToString(), "Requested", this.game.Data.Round.ToString(), num11.ToString());
          this.game.Data.StringListObj[stringListById].AddRowWithData("Unit fuel", this.game.Data.Turn.ToString(), "Received from supply source", this.game.Data.Round.ToString(), num13.ToString());
          this.game.Data.StringListObj[stringListById].AddRowWithData("All supply source : Supply", this.game.Data.Turn.ToString(), "Sent to units", this.game.Data.Round.ToString(), num59.ToString());
          this.game.Data.StringListObj[stringListById].AddRowWithData("All supply source : Fuel", this.game.Data.Turn.ToString(), "Sent to units", this.game.Data.Round.ToString(), num60.ToString());
        }
        else
        {
          if (evacuateMode || !buildUpMode)
            return;
          this.game.Data.StringListObj[stringListById].AddRowWithData("All supply source : Supply", this.game.Data.Turn.ToString(), "Sent to supply base", this.game.Data.Round.ToString(), num59.ToString());
          this.game.Data.StringListObj[stringListById].AddRowWithData("All supply source : Fuel", this.game.Data.Turn.ToString(), "Sent to supply base", this.game.Data.Round.ToString(), num60.ToString());
        }
      }
    }

    pub fn LIS_LocationSupply() => this.LIS_UniversalSupplyAndReturn(2, false);

    pub fn LIS_UnitSupply(bool freeOfCost)
    {
      let mut num1: i32 = 20;
      let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[404]));
      this.plogcounter = -1;
      this.AddPLog(nameof (LIS_UnitSupply));
      this.AddPLog("");
      SimpleList simpleList = SimpleList::new();
      let mut unitCounter1: i32 = this.game.Data.UnitCounter;
      index1: i32;
      for (index1 = 0; index1 <= unitCounter1; index1 += 1)
      {
        this.game.Data.UnitObj[index1].tempCoords = (CoordList) null;
        this.game.Data.UnitObj[index1].tempComplexCoords = (ComplexCoordList) null;
      }
      num2: i32;
      if (this.game.Data.Turn == 4)
        num2 = index1;
      let mut unitCounter2: i32 = this.game.Data.UnitCounter;
      index2: i32;
      dat1: i32;
      for (index2 = 0; index2 <= unitCounter2; index2 += 1)
      {
        if (this.game.Data.UnitObj[index2].IsHQ & this.game.Data.UnitObj[index2].Regime == this.game.Data.Turn)
        {
          let mut historical1: i32 = this.game.Data.UnitObj[index2].Historical;
          if (historical1 > -1 && this.game.Data.HistoricalUnitObj[historical1].Type == 8)
          {
            bool flag1 = false;
            let mut unitCounter3: i32 = this.game.Data.UnitCounter;
            for (let mut unr: i32 = 0; unr <= unitCounter3; unr += 1)
            {
              let mut historical2: i32 = this.game.Data.UnitObj[unr].Historical;
              if (this.game.HandyFunctionsObj.IsUnitInHQChain(unr, index2) & unr != index2 & historical2 > -1)
              {
                bool flag2 = true;
                let mut num3: i32 = this.game.Data.HistoricalUnitObj[historical2].GiveHisVarValue(71);
                let mut movetype: i32 = this.game.Data.HistoricalUnitObj[historical2].GiveHisVarValue(72);
                this.game.Data.UnitObj[unr].tempCoords = CoordList::new();
                this.game.Data.UnitObj[unr].tempComplexCoords = ComplexCoordList::new();
                this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 0, num3 * 2, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, false, SeaBlock: true, BlockAllSea: true);
                this.game.Data.UnitObj[unr].tempCoords.AddCoord(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0, 0, 100);
                if (this.game.Data.Turn == 2)
                  index2 = index2;
                let mut mapWidth: i32 = this.game.Data.MapObj[0].MapWidth;
                for (let mut x: i32 = 0; x <= mapWidth; x += 1)
                {
                  let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
                  for (let mut y: i32 = 0; y <= mapHeight; y += 1)
                  {
                    if (this.game.EditObj.TempValue[0].Value[x, y] <= num3 * 2)
                    {
                      let mut num4: i32 = this.game.EditObj.TempValue[0].Value[x, y];
                      let mut dat2_1: i32 = 0;
                      if (num4 <= num3)
                        dat2_1 = 100;
                      else if (num4 < num3 * 2)
                        dat2_1 = 100 -  Math.Round( (100 * (num4 - num3)) /  num3);
                      CoordList tCoordList = new CoordList(true);
                      Coordinate coordinate;
                      coordinate.onmap = true;
                      coordinate.x = x;
                      coordinate.y = y;
                      coordinate = this.game.EditObj.TempCameFrom[0].Value[x, y];
                      if (this.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] >= 9999)
                        coordinate.onmap = false;
                      while (coordinate.onmap)
                      {
                        dat1 = this.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y];
                        let mut dat2_2: i32 = 0;
                        if (dat1 <= num3)
                          dat2_2 = 100;
                        else if (dat1 < num3 * 2)
                          dat2_2 = 100 -  Math.Round( (100 * (dat1 - num3)) /  num3);
                        tCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map, dat1, dat2_2);
                        coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y];
                        if (this.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y] >= 9999)
                          coordinate.onmap = false;
                      }
                      this.game.Data.UnitObj[unr].tempComplexCoords.AddCoord(x, y, 0, this.game.EditObj.TempValue[0].Value[x, y], dat2_1, tCoordList);
                      this.game.Data.UnitObj[unr].tempCoords.AddCoord(x, y, 0, this.game.EditObj.TempValue[0].Value[x, y], dat2_1);
                    }
                  }
                }
                flag1 = flag2;
              }
            }
            if (flag1)
            {
              if (Information.IsNothing( this.game.Data.UnitObj[index2].items))
                this.game.Data.UnitObj[index2].items = ItemList::new();
              this.game.Data.UnitObj[index2].tempLisStartHistory = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y].LIShistory[6];
              simpleList.Add(index2, 1);
            }
          }
        }
      }
      if (this.game.Data.Turn == 2)
        num2 = index2;
      let mut unitCounter4: i32 = this.game.Data.UnitCounter;
      for (let mut index3: i32 = 0; index3 <= unitCounter4; index3 += 1)
      {
        this.game.Data.UnitObj[index3].tempHandledItems = SimpleList::new();
        if (this.game.Data.UnitObj[index3].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index3].PreDef == -1 && !Information.IsNothing( this.game.Data.UnitObj[index3].tempRequestItems))
        {
          let mut counter: i32 = this.game.Data.UnitObj[index3].tempRequestItems.Counter;
          for (let mut index4: i32 = 0; index4 <= counter; index4 += 1)
          {
            UnitClass[] unitObj = this.game.Data.UnitObj;
            UnitClass[] unitClassArray = unitObj;
            let mut index5: i32 = index3;
            let mut index6: i32 = index5;
            unitClassArray[index6].SupplyInReq = unitObj[index5].SupplyInReq + this.game.Data.UnitObj[index3].tempRequestItems.Weight[index4];
          }
        }
      }
      bool[] flagArray1 = new bool[this.game.Data.UnitCounter + 1];
      bool[] flagArray2 = new bool[this.game.Data.UnitCounter + 1];
      int[] numArray = new int[this.game.Data.UnitCounter + 1];
      bool[] flagArray3 = new bool[this.game.Data.UnitCounter + 1];
      bool[] flagArray4 = new bool[this.game.Data.UnitCounter + 1];
      bool[] flagArray5 = new bool[this.game.Data.UnitCounter + 1];
      bool flag3 = true;
      let mut num5: i32 = 0;
      let mut num6: i32 = 0;
      while (flag3)
      {
        num5 += 1;
        num6 += num1;
        flag3 = false;
        bool flag4 = true;
        this.AddPLog(num5.ToString() + ".Superloop. CurMaxPercent=" + num6.ToString());
        let mut num7: i32 = 0;
        while (flag4)
        {
          flag4 = false;
          num7 += 1;
          this.AddPLog("ShqLoop");
          let mut counter1: i32 = simpleList.Counter;
          bool flag5;
          for (let mut index7: i32 = 0; index7 <= counter1; index7 += 1)
          {
            let mut hq: i32 = simpleList.Id[index7];
            let mut num8: i32 = 0;
            num8 = this.game.Data.UnitObj[hq].lisInstructions.FindWeight(2) + this.game.Data.UnitObj[hq].lisInstructions.FindWeight(3);
            if (num8 > 100)
              num8 = 100;
            let mut weight1: i32 = this.game.Data.UnitObj[hq].lisInstructions.FindWeight(13);
            let mut weight2: i32 = this.game.Data.UnitObj[hq].lisInstructions.FindWeight(12);
            this.AddPLog("SHQ Max Percentage Allowed " + num8.ToString() + "%");
            let mut data3_1: i32 =  Math.Round( ((this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y].LISpoints[6] + this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y].LIShistory[6]) * num8) / 100.0) - (this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y].LIShistory[6] - this.game.Data.UnitObj[hq].tempLisStartHistory);
            if (data3_1 < 1)
              data3_1 = 0;
            if (num7 == 1 & num5 == 1)
              this.game.Data.UnitObj[hq].AddLog(603, 1, 0, data3_1);
            if (data3_1 < 1)
              data3_1 = 0;
            let mut num9: i32 =  Math.Round( (data3_1 * num6) / 100.0);
            if (!freeOfCost)
            {
              if (this.game.EventRelatedObj.Helper_AirEnabled() & weight1 == 1)
                this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map, useAirBridge: true, maxDam: weight2);
              else
                this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map);
            }
            this.AddPLog(num5.ToString() + "." + num7.ToString() + ".Shq: " + this.game.Data.UnitObj[hq].Name + ", CurrentLisAvailable: " + num9.ToString());
            flag5 = true;
            let mut num10: i32 = 0;
            let mut num11: i32 = 0;
            while (flag5)
            {
              flag5 = false;
              num10 += 1;
              num11 += 10;
              if (num11 > 100)
                num11 = 100;
              if (num11 < 100)
                flag5 = true;
              let mut unitCounter5: i32 = this.game.Data.UnitCounter;
              for (let mut unr: i32 = 0; unr <= unitCounter5; unr += 1)
              {
                if (this.game.HandyFunctionsObj.IsUnitInHQChain(unr, hq) & this.game.Data.UnitObj[unr].Historical > -1)
                {
                  bool flag6 = false;
                  if (this.game.Data.UnitObj[hq].X == this.game.Data.UnitObj[unr].X & this.game.Data.UnitObj[hq].Y == this.game.Data.UnitObj[unr].Y)
                    flag6 = true;
                  this.AddPLog(num5.ToString() + "." + num7.ToString() + "." + num10.ToString() + ".Unit: " + this.game.Data.UnitObj[unr].Name);
                  if (unr == 202)
                    unr = unr;
                  if (Information.IsNothing( this.game.Data.UnitObj[unr].tempRequestItems))
                    this.game.Data.UnitObj[unr].tempRequestItems = SimpleList::new();
                  let mut counter2: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Counter;
                  for (let mut index8: i32 = 0; index8 <= counter2; index8 += 1)
                  {
                    if (!freeOfCost & num10 == 1 & num7 == 1 & num5 == 1)
                    {
                      let mut data1: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Id[index8];
                      let mut data3_2: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Weight[index8];
                      this.game.Data.UnitObj[unr].AddLog(202, data1, 0, data3_2);
                      this.game.Data.UnitObj[hq].AddLog(202, data1, 0, data3_2);
                    }
                    if (this.game.Data.UnitObj[unr].X == 14 & this.game.Data.UnitObj[unr].Y == 3)
                      unr = unr;
                    if (num9 > 0 | freeOfCost | flag6)
                    {
                      let mut num12: i32 = this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[unr].Historical].GiveHisVarValue(71);
                      let mut num13: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Id[index8];
                      let mut num14: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Weight[index8];
                      let mut integer: i32 = Conversions.ToInteger(this.game.Data.StringListObj[stringListById].GetData(0, num13, 3));
                      let mut nr1: i32 = this.game.Data.UnitObj[unr].tempHandledItems.FindNr(num13);
                      let mut num15: i32 = 0;
                      if (nr1 > -1)
                        num15 = this.game.Data.UnitObj[unr].tempHandledItems.Weight[nr1];
                      let mut num16: i32 =  Math.Round(Math.Ceiling( num14 * ( num6 / 100.0) * ( num11 / 100.0)));
                      if (num14 > 0 & num16 == 0)
                        num16 = 1;
                      let mut num17: i32 = num16 - num15;
                      let mut pts: i32 = num17 * integer;
                      if (flag6)
                        pts = 0;
                      if (!freeOfCost && pts > num9)
                      {
                        num17 =  Math.Round(Math.Floor( (num17 * num9) /  pts));
                        pts = num9;
                        if (!flagArray5[unr])
                        {
                          flagArray5[unr] = true;
                          this.game.Data.UnitObj[unr].AddLog(704, 0, 0, 0);
                        }
                      }
                      let mut nr2: i32 = this.game.Data.UnitObj[hq].items.list.FindNr(num13);
                      if (nr2 > -1)
                      {
                        let mut num18: i32 = this.game.Data.UnitObj[hq].items.list.Weight[nr2];
                        if (num18 < 0)
                          num18 = 0;
                        if (num17 > num18)
                        {
                          num17 = num18;
                          if (!flagArray4[unr])
                          {
                            flagArray4[unr] = true;
                            this.game.Data.UnitObj[unr].AddLog(703, 0, 0, 0);
                          }
                        }
                      }
                      else
                      {
                        num17 = 0;
                        if (!flagArray4[unr])
                        {
                          flagArray4[unr] = true;
                          this.game.Data.UnitObj[unr].AddLog(703, 0, 0, 0);
                        }
                      }
                      Coordinate unitBestCoordinate;
                      if (!freeOfCost & num17 > 0)
                      {
                        if (flag6)
                        {
                          unitBestCoordinate.x = this.game.Data.UnitObj[hq].X;
                          unitBestCoordinate.y = this.game.Data.UnitObj[hq].Y;
                          unitBestCoordinate.data2 = 100;
                          if (!flagArray1[unr] | unitBestCoordinate.data2 < numArray[unr])
                          {
                            flagArray1[unr] = true;
                            numArray[unr] = unitBestCoordinate.data2;
                            this.game.Data.UnitObj[unr].AddLog(701, unitBestCoordinate.x, unitBestCoordinate.y, unitBestCoordinate.data2);
                          }
                        }
                        else
                        {
                          if (unr == 978)
                            unr = unr;
                          unitBestCoordinate = this.LIS_GetUnitBestCoordinate(this.game.Data.UnitObj[unr].tempCoords, num17, true, this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y);
                          if (this.game.Data.UnitObj[unr].X == 14)
                            unr = unr;
                          if (unitBestCoordinate.onmap)
                          {
                            if (this.game.EventRelatedObj.Helper_AirEnabled() & !flag6 && integer < 1 && this.LIS_HasAirBridgeOnTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y))
                            {
                              let mut num19: i32 = 1;
                              if (this.IsItemMaximizer(num13))
                                num17 = 0;
                              pts = num17 * num19;
                              if (!freeOfCost && pts > num9)
                              {
                                num17 =  Math.Round(Math.Floor( (num17 * num9) /  pts));
                                pts = num9;
                                if (!flagArray5[unr])
                                {
                                  flagArray5[unr] = true;
                                  this.game.Data.UnitObj[unr].AddLog(704, 0, 0, 0);
                                }
                              }
                            }
                            bool flag7 = false;
                            this.AddPLog("Best pickup adress: " + unitBestCoordinate.x.ToString() + "," + unitBestCoordinate.y.ToString() + ", organic ap need for pickup= " + unitBestCoordinate.data2.ToString());
                            if (unitBestCoordinate.data2 < 2 * num12)
                            {
                              if (this.game.Data.UnitObj[unr].tempRequestItems.Id[index8] == 7)
                              {
                                dat1 = Convert.ToInt32(Math.Floor(new Decimal(this.game.Data.UnitObj[unr].tempRequestItems.Weight[index8])));
                                flag7 = true;
                              }
                              else
                                dat1 =  Math.Round(Math.Floor( (this.game.Data.UnitObj[unr].tempRequestItems.Weight[index8] * unitBestCoordinate.data2) / 100.0));
                              if (num17 + num15 > dat1)
                              {
                                num17 = dat1 - num15;
                                if (0 > num17)
                                  num17 = 0;
                              }
                            }
                            else
                              dat1 = dat1;
                            if (!flagArray1[unr] | unitBestCoordinate.data2 < numArray[unr] && !flag7)
                            {
                              flagArray1[unr] = true;
                              numArray[unr] = unitBestCoordinate.data2;
                              this.game.Data.UnitObj[unr].AddLog(701, unitBestCoordinate.x, unitBestCoordinate.y, unitBestCoordinate.data2);
                            }
                            if (num17 > 0)
                            {
                              let mut pointsOnTrajectory: i32 = this.LIS_GetLowestPointsOnTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, true);
                              if (pointsOnTrajectory < num14 && !flagArray3[unr])
                              {
                                flagArray3[unr] = true;
                                this.game.Data.UnitObj[unr].AddLog(702, 0, 0, 0);
                              }
                              if (pointsOnTrajectory < 0)
                              {
                                num17 = 0;
                                pts = 0;
                                if (!flagArray3[unr])
                                {
                                  flagArray3[unr] = true;
                                  this.game.Data.UnitObj[unr].AddLog(702, 0, 0, 0);
                                }
                              }
                              else if (pointsOnTrajectory < pts)
                              {
                                num17 =  Math.Round(Math.Floor( (num17 * pointsOnTrajectory) /  pts));
                                pts = pointsOnTrajectory;
                                if (!flagArray3[unr])
                                {
                                  flagArray3[unr] = true;
                                  this.game.Data.UnitObj[unr].AddLog(702, 0, 0, 0);
                                }
                              }
                            }
                          }
                          else
                          {
                            this.AddPLog("No pickup hex found for unit.");
                            if (!flagArray2[unr])
                            {
                              flagArray2[unr] = true;
                              this.game.Data.UnitObj[unr].AddLog(705, 0, 0, 0);
                            }
                            num17 = 0;
                          }
                        }
                      }
                      if (num17 > 0)
                      {
                        this.AddPLog("Item: ID" + num13.ToString() + ", Qty=" + num17.ToString() + ", Already transferred: " + num15.ToString() + "/" + num14.ToString() + ", LogPtsNeed: " + pts.ToString());
                        flag5 = true;
                        let mut num20: i32 = num17;
                        bool ok;
                        if (!freeOfCost & !flag6)
                        {
                          if (this.game.EventRelatedObj.Helper_AirEnabled())
                          {
                            OrderResult orderResult = this.LIS_RemovePointsFromTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, pts);
                            ok = orderResult.OK;
                            if (orderResult.Data > 0)
                            {
                              let mut randomizedRoundingDam: i32 = this.game.HandyFunctionsObj.Air_GetRandomizedRoundingDam(num20, orderResult.Data);
                              num20 -= randomizedRoundingDam;
                              this.game.Data.UnitObj[unr].AddLog(107, num13, 0, randomizedRoundingDam);
                              this.game.Data.UnitObj[hq].AddLog(107, num13, 0, randomizedRoundingDam);
                            }
                          }
                          else
                            ok = this.LIS_RemovePointsFromTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, pts).OK;
                        }
                        this.game.Data.UnitObj[hq].items.list.RemoveWeight(num13, num17);
                        this.game.Data.UnitObj[unr].items.list.AddWeight(num13, num20);
                        if (!freeOfCost)
                          this.game.Data.UnitObj[hq].AddLog(103, num13, 0, num17);
                        if (!freeOfCost)
                          this.game.Data.UnitObj[unr].AddLog(105, num13, 0, num20);
                        UnitClass[] unitObj = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray = unitObj;
                        let mut index9: i32 = unr;
                        let mut index10: i32 = index9;
                        unitClassArray[index10].SupplyIn = unitObj[index9].SupplyIn + num20;
                        this.game.Data.UnitObj[unr].tempHandledItems.AddWeight(num13, num17);
                        if (!freeOfCost & !flag6)
                        {
                          num9 -= pts;
                          this.game.Data.UnitObj[unr].AddLog(503, 1, 0, num17);
                          this.LIS_AddOrganicPointsToTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, num17, unitBestCoordinate.data2, unr);
                          this.AddPLog("--Removed " + pts.ToString() + " from trajectory " + unitBestCoordinate.x.ToString() + "," + unitBestCoordinate.y.ToString() + " <> " + this.game.Data.UnitObj[hq].Name);
                          if (ok & pts > 0)
                          {
                            this.AddPLog("--0 reached--");
                            if (this.game.EventRelatedObj.Helper_AirEnabled() & weight1 == 1)
                              this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map, useAirBridge: true, maxDam: weight2);
                            else
                              this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map);
                          }
                        }
                      }
                    }
                    else if (!flagArray5[unr])
                    {
                      flagArray5[unr] = true;
                      this.game.Data.UnitObj[unr].AddLog(704, 0, 0, 0);
                    }
                  }
                }
              }
            }
          }
          if (flag5)
            flag4 = true;
        }
        if (num6 < 100)
          flag3 = true;
      }
      let mut counter3: i32 = simpleList.Counter;
      for (let mut index11: i32 = 0; index11 <= counter3; index11 += 1)
      {
        let mut index12: i32 = simpleList.Id[index11];
        let mut num21: i32 = 0;
        num21 = this.game.Data.UnitObj[index12].lisInstructions.FindWeight(2) + this.game.Data.UnitObj[index12].lisInstructions.FindWeight(3);
        let mut data3: i32 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index12].X, this.game.Data.UnitObj[index12].Y].LIShistory[6] - this.game.Data.UnitObj[index12].tempLisStartHistory;
        this.AddPLog("SHQ " + this.game.Data.UnitObj[index12].Name + " total Logistical Points use is " + data3.ToString() + ".");
        this.game.Data.UnitObj[index12].AddLog(503, 1, 0, data3);
      }
      if (!freeOfCost)
        this.WritePLog("LIS_UnitSupply_Log");
      if (freeOfCost)
        this.WritePLog("LIS_UnitSupply_Free_Log");
      if (!freeOfCost)
        this.LIS_SetNetwork(true);
      let mut unitCounter6: i32 = this.game.Data.UnitCounter;
      for (let mut index13: i32 = 0; index13 <= unitCounter6; index13 += 1)
        this.game.Data.UnitObj[index13].tempComplexCoords = (ComplexCoordList) null;
      if (freeOfCost || !this.game.EventRelatedObj.Helper_AirEnabled())
        return;
      this.game.HandyFunctionsObj.Air_LogUpdateForAll(this.game.Data.Turn, "", " Air Points left after Unit Supply");
    }

    pub fn LIS_UnitAutoReinforceSetTempLists(let mut underUnr: i32 = -1, bool makeLogs = true)
    {
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut unr: i32 = 0; unr <= unitCounter; unr += 1)
      {
        this.game.Data.UnitObj[unr].tempRequestItems = SimpleList::new();
        this.game.Data.UnitObj[unr].tempMaxItems = SimpleList::new();
        if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unr].PreDef == -1 & this.game.HandyFunctionsObj.LIS_AutoReinforceRulesValid(unr) & !this.game.HandyFunctionsObj.CheckIsBattlegroup(unr))
        {
          bool flag = true;
          if (underUnr > -1)
          {
            if (underUnr == unr)
              flag = true;
            else if (!this.game.HandyFunctionsObj.IsUnitInHQChain(unr, underUnr))
              flag = false;
          }
          if (flag)
          {
            let mut historical: i32 = this.game.Data.UnitObj[unr].Historical;
            if (historical > -1)
            {
              let mut preDef: i32 = this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[historical].SubParts[0]);
              let mut index1: i32 = -1;
              if (this.game.Data.UnitObj[unr].HQ > -1)
                index1 = this.game.Data.UnitObj[unr].HQ;
              if (index1 > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8 && this.game.Data.UnitObj[index1].HQ > -1)
              {
                index1 = this.game.Data.UnitObj[index1].HQ;
                if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].Type < 8)
                  index1 = -1;
              }
              SimpleList simpleList1 = SimpleList::new();
              SimpleList simpleList2 = SimpleList::new();
              if (preDef > -1)
              {
                let mut sfCount: i32 = this.game.Data.UnitObj[preDef].SFCount;
                for (let mut index2: i32 = 0; index2 <= sfCount; index2 += 1)
                {
                  let mut sf: i32 = this.game.Data.UnitObj[preDef].SFList[index2];
                  let mut type: i32 = this.game.Data.SFObj[sf].Type;
                  let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
                  let mut reinforcementType2: i32 = this.game.Data.SFTypeObj[type].ReinforcementType2;
                  let mut tweight: i32 =  Math.Round( (this.game.Data.SFObj[sf].Qty * this.game.Data.UnitObj[unr].SOReplacementPercent) / 100.0);
                  if (reinforcementType > -1 & tweight > 0)
                    simpleList1.AddWeight(reinforcementType, tweight);
                  if (reinforcementType > -1 & tweight > 0)
                    simpleList2.AddWeight(reinforcementType, tweight);
                  if (reinforcementType2 > -1 & tweight > 0)
                    simpleList2.AddWeight(reinforcementType2, tweight);
                }
              }
              SimpleList SL = SimpleList::new();
              if (preDef > -1)
              {
                let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
                for (let mut index3: i32 = 0; index3 <= sfCount; index3 += 1)
                {
                  let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index3];
                  let mut reinforcementType: i32 = this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].ReinforcementType;
                  let mut qty: i32 = this.game.Data.SFObj[sf].Qty;
                  if (reinforcementType > -1 & qty > 0)
                    SL.AddWeight(reinforcementType, qty);
                }
              }
              simpleList1.RemoveWeight( SL);
              simpleList2.RemoveWeight( SL);
              simpleList2.removeWeight0orLower();
              simpleList1.removeWeight0orLower();
              if (simpleList1.Counter > -1)
              {
                this.game.Data.UnitObj[unr].tempRequestItems = simpleList1;
                let mut counter: i32 = simpleList1.Counter;
                for (let mut index4: i32 = 0; index4 <= counter; index4 += 1)
                {
                  if (makeLogs)
                  {
                    this.game.Data.UnitObj[unr].AddLog(1, simpleList1.Id[index4], 0, simpleList1.Weight[index4]);
                    if (index1 > -1)
                      this.game.Data.UnitObj[index1].AddLog(11, simpleList1.Id[index4], 0, simpleList1.Weight[index4]);
                  }
                }
              }
              if (simpleList2.Counter > 1)
                this.game.Data.UnitObj[unr].tempMaxItems = simpleList2;
            }
          }
        }
      }
    }

    pub fn LIS_UnitAutoReinforce()
    {
      let mut num1: i32 = 20;
      this.plogcounter = -1;
      this.AddPLog("LIS_AutoReinforce");
      this.AddPLog("");
      this.LIS_UnitAutoReinforceSetTempLists();
      if (this.game.Data.Turn == 2)
      {
        let mut num2: i32 = num2;
      }
      let mut unitCounter1: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter1; index += 1)
        this.game.Data.UnitObj[index].tempCoords = (CoordList) null;
      SimpleList simpleList = SimpleList::new();
      let mut unitCounter2: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter2; index += 1)
      {
        if (this.game.Data.UnitObj[index].IsHQ & this.game.Data.UnitObj[index].Regime == this.game.Data.Turn)
        {
          let mut historical1: i32 = this.game.Data.UnitObj[index].Historical;
          if (historical1 > -1 && this.game.Data.HistoricalUnitObj[historical1].Type == 8)
          {
            bool flag = false;
            let mut unitCounter3: i32 = this.game.Data.UnitCounter;
            for (let mut unr: i32 = 0; unr <= unitCounter3; unr += 1)
            {
              if (this.game.HandyFunctionsObj.IsUnitInHQChain(unr, index) & this.game.Data.UnitObj[unr].Historical > -1 & unr != index)
              {
                flag = true;
                if (unr == 966)
                  unr = unr;
                let mut historical2: i32 = this.game.Data.UnitObj[unr].Historical;
                let mut num3: i32 = this.game.Data.HistoricalUnitObj[historical2].GiveHisVarValue(71);
                let mut movetype: i32 = this.game.Data.HistoricalUnitObj[historical2].GiveHisVarValue(72);
                this.game.Data.UnitObj[unr].tempCoords = CoordList::new();
                this.game.Data.UnitObj[unr].tempComplexCoords = ComplexCoordList::new();
                this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 0, num3 * 2, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, false, SeaBlock: true, BlockAllSea: true);
                let mut mapWidth: i32 = this.game.Data.MapObj[0].MapWidth;
                for (let mut x: i32 = 0; x <= mapWidth; x += 1)
                {
                  let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
                  for (let mut y: i32 = 0; y <= mapHeight; y += 1)
                  {
                    if (this.game.EditObj.TempValue[0].Value[x, y] <= 2 * num3)
                    {
                      let mut num4: i32 = this.game.EditObj.TempValue[0].Value[x, y];
                      let mut dat2_1: i32 = 0;
                      if (num4 <= num3)
                        dat2_1 = 100;
                      else if (num4 < num3 * 2)
                        dat2_1 = 100 -  Math.Round( (100 * (num4 - num3)) /  num3);
                      CoordList tCoordList = new CoordList(true);
                      Coordinate coordinate;
                      coordinate.onmap = true;
                      coordinate.x = x;
                      coordinate.y = y;
                      coordinate = this.game.EditObj.TempCameFrom[0].Value[x, y];
                      while (coordinate.onmap)
                      {
                        let mut dat1: i32 = this.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y];
                        let mut dat2_2: i32 = 0;
                        if (dat1 <= num3)
                          dat2_2 = 100;
                        else if (dat1 < num3 * 2)
                          dat2_2 = 100 -  Math.Round( (100 * (dat1 - num3)) /  num3);
                        tCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map, dat1, dat2_2);
                        coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y];
                        if (tCoordList.counter > 200)
                        {
                          tCoordList = new CoordList(true);
                          break;
                        }
                      }
                      this.game.Data.UnitObj[unr].tempComplexCoords.AddCoord(x, y, 0, this.game.EditObj.TempValue[0].Value[x, y], dat2_1, tCoordList);
                      this.game.Data.UnitObj[unr].tempCoords.AddCoord(x, y, 0, this.game.EditObj.TempValue[0].Value[x, y], dat2_1);
                    }
                    else
                    {
                      let mut num5: i32 = this.game.EditObj.TempValue[0].Value[x, y];
                    }
                  }
                }
                index = index;
              }
            }
            if (flag)
            {
              if (Information.IsNothing( this.game.Data.UnitObj[index].items))
                this.game.Data.UnitObj[index].items = ItemList::new();
              this.game.Data.UnitObj[index].tempLisStartHistory = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index].X, this.game.Data.UnitObj[index].Y].LIShistory[6];
              simpleList.Add(index, 1);
            }
          }
        }
      }
      let mut unitCounter4: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter4; index += 1)
        this.game.Data.UnitObj[index].tempHandledItems = SimpleList::new();
      bool flag1 = true;
      let mut num6: i32 = 0;
      let mut num7: i32 = 0;
      while (flag1)
      {
        num6 += 1;
        num7 += num1;
        flag1 = false;
        bool flag2 = true;
        this.AddPLog(num6.ToString() + ".Superloop. CurMaxPercent=" + num7.ToString());
        let mut num8: i32 = 0;
        while (flag2)
        {
          flag2 = false;
          num8 += 1;
          this.AddPLog("ShqLoop");
          let mut counter1: i32 = simpleList.Counter;
          bool flag3;
          for (let mut index1: i32 = 0; index1 <= counter1; index1 += 1)
          {
            let mut index2: i32 = simpleList.Id[index1];
            let mut num9: i32 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y].LISpoints[6] + this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y].LIShistory[6];
            let mut num10: i32 = this.game.Data.UnitObj[index2].lisInstructions.FindWeight(2) + this.game.Data.UnitObj[index2].lisInstructions.FindWeight(3) + this.game.Data.UnitObj[index2].lisInstructions.FindWeight(5) + this.game.Data.UnitObj[index2].lisInstructions.FindWeight(6);
            this.AddPLog("SHQ Max Percentage Allowed " + num10.ToString() + "%");
            if (num10 > 100)
              num10 = 100;
            let mut data3: i32 =  Math.Round( (num9 * num10) / 100.0) - (this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y].LIShistory[6] - this.game.Data.UnitObj[index2].tempLisStartHistory);
            if (data3 < 1)
              data3 = 0;
            if (num8 == 1 & num6 == 1)
              this.game.Data.UnitObj[index2].AddLog(606, 1, 0, data3);
            if (0 > data3)
              data3 = 0;
            let mut num11: i32 =  Math.Round( (data3 * num7) / 100.0);
            this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map);
            this.AddPLog(num6.ToString() + "." + num8.ToString() + ".Shq: " + this.game.Data.UnitObj[index2].Name + ", CurrentLisAvailable: " + num11.ToString());
            flag3 = true;
            let mut num12: i32 = 0;
            let mut num13: i32 = 0;
            while (flag3)
            {
              flag3 = false;
              num12 += 1;
              num13 += 10;
              if (num13 > 100)
                num13 = 100;
              if (num13 < 100)
                flag3 = true;
              let mut unitCounter5: i32 = this.game.Data.UnitCounter;
              for (let mut unr: i32 = 0; unr <= unitCounter5; unr += 1)
              {
                if (this.game.HandyFunctionsObj.IsUnitInHQChain(unr, index2) & !this.game.HandyFunctionsObj.CheckIsBattlegroup(unr))
                {
                  bool flag4 = false;
                  if (this.game.Data.UnitObj[index2].X == this.game.Data.UnitObj[unr].X & this.game.Data.UnitObj[index2].Y == this.game.Data.UnitObj[unr].Y)
                    flag4 = true;
                  this.AddPLog(num6.ToString() + "." + num8.ToString() + "." + num12.ToString() + ".Unit: " + this.game.Data.UnitObj[unr].Name);
                  let mut counter2: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Counter;
                  for (let mut index3: i32 = 0; index3 <= counter2; index3 += 1)
                  {
                    if (num12 == 1 & num8 == 1 & num6 == 1)
                    {
                      let mut num14: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Id[index3];
                      let mut num15: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Weight[index3];
                    }
                    if (num11 > 0 | flag4)
                    {
                      let mut num16: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Id[index3];
                      let mut reinfType: i32 = -1;
                      let mut num17: i32 = 0;
                      let mut index4: i32 = -1;
                      let mut historical: i32 = this.game.Data.UnitObj[unr].Historical;
                      index5: i32;
                      if (historical > -1 && this.game.Data.HistoricalUnitObj[historical].SubParts[0] > 0)
                      {
                        let mut preDef: i32 = this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[historical].SubParts[0]);
                        if (preDef > -1)
                        {
                          let mut sfCount: i32 = this.game.Data.UnitObj[preDef].SFCount;
                          for (let mut index6: i32 = 0; index6 <= sfCount; index6 += 1)
                          {
                            index5 = this.game.Data.UnitObj[preDef].SFList[index6];
                            let mut type: i32 = this.game.Data.SFObj[index5].Type;
                            if (this.game.Data.SFTypeObj[type].ReinforcementType == num16)
                            {
                              reinfType = this.game.Data.SFTypeObj[type].ReinforcementType2;
                              num17 = this.game.Data.SFTypeObj[type].Weight;
                              index4 = this.game.Data.SFTypeObj[type].MoveType;
                            }
                          }
                        }
                      }
                      let mut num18: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Weight[index3];
                      let mut nr: i32 = this.game.Data.UnitObj[unr].tempHandledItems.FindNr(num16);
                      let mut num19: i32 = 0;
                      if (nr > -1)
                        num19 = this.game.Data.UnitObj[unr].tempHandledItems.Weight[nr];
                      let mut num20: i32 =  Math.Round(Math.Ceiling( num18 * ( num7 / 100.0) * ( num13 / 100.0)));
                      if (num18 > 0 & num20 == 0)
                        num20 = 1;
                      let mut num21: i32 = num20 - num19;
                      let mut num22: i32 = num21;
                      if (flag4)
                        num22 = 0;
                      let mut num23: i32 = 1 * num17;
                      if (num23 > num11)
                      {
                        num21 =  Math.Round(Math.Floor( (num21 * num11) /  num23));
                        num23 = num11;
                      }
                      if (index4 > -1)
                      {
                        let mut num24: i32 = 9999;
                        bool flag5 = false;
                        let mut x: i32 = this.game.Data.UnitObj[unr].X;
                        let mut y: i32 = this.game.Data.UnitObj[unr].Y;
                        let mut index7: i32 = 0;
                        do
                        {
                          let mut index8: i32 = this.game.Data.MapObj[0].HexObj[x, y].RoadType[index7];
                          if (index8 > -1)
                          {
                            if (this.game.Data.RoadTypeObj[index8].MoveCostOverrule[index4] >= 999)
                            {
                              flag5 = true;
                            }
                            else
                            {
                              let mut num25: i32 = this.game.Data.RoadTypeObj[index8].MoveCostOverrule[index4];
                              if (num25 < num24)
                                num24 = num25;
                            }
                          }
                          index7 += 1;
                        }
                        while (index7 <= 5);
                        let mut num26: i32 = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[x, y].LandscapeType].MoveCost[index4];
                        if (num26 < num24)
                          num24 = num26;
                        if (num24 > 100 & !flag5)
                          num21 = 0;
                      }
                      let mut index9: i32 = -1;
                      if (num21 > 0)
                      {
                        bool quality2 = false;
                        bool quality3 = false;
                        bool quality4 = false;
                        bool quality5 = false;
                        if (num21 > 1)
                          quality2 = quality2;
                        if (historical > -1)
                        {
                          if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(42) <= 0)
                            quality2 = true;
                          if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(43) <= 0)
                            quality3 = true;
                          if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(44) <= 0)
                            quality4 = true;
                          if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(45) <= 0)
                            quality5 = true;
                          if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(11) > 0)
                          {
                            quality2 = true;
                            quality3 = true;
                            quality4 = true;
                            quality5 = true;
                          }
                        }
                        let mut people: i32 = this.game.Data.HistoricalUnitObj[historical].People;
                        let mut index10: i32 = Conversions.ToInteger(this.LIS_GetSHQSFObjNr(index2, quality2, quality3, quality4, quality5, num16, people));
                        if (index10 == -1)
                        {
                          index10 = Conversions.ToInteger(this.LIS_GetSHQSFObjNr(index2, quality2, quality3, quality4, quality5, reinfType, people));
                          if (index10 > -1)
                          {
                            let mut reinforcementType: i32 = this.game.Data.SFTypeObj[this.game.Data.SFObj[index10].Type].ReinforcementType;
                            if (this.game.Data.UnitObj[unr].tempMaxItems.FindWeight(reinforcementType) < 1)
                              index10 = -1;
                          }
                        }
                        index5 = index10;
                        if (index5 > -1)
                          index9 = this.game.Data.SFObj[index5].Type;
                        if (index10 > -1)
                        {
                          num21 = 1;
                          if (num18 < 1)
                            num21 = 0;
                        }
                        else
                          num21 = 0;
                      }
                      Coordinate unitBestCoordinate;
                      if (num21 > 0)
                      {
                        unitBestCoordinate = this.LIS_GetUnitBestCoordinate(this.game.Data.UnitObj[unr].tempCoords, num21, true, this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y);
                        if (unitBestCoordinate.onmap)
                        {
                          this.AddPLog("Pickup adress: " + unitBestCoordinate.x.ToString() + "," + unitBestCoordinate.y.ToString());
                          if (this.LIS_GetLowestPointsOnTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y) < num23)
                            num21 = 0;
                        }
                        else
                        {
                          this.AddPLog("No pickup hex found for unit.");
                          num21 = 0;
                        }
                      }
                      if (num21 > 0)
                      {
                        this.AddPLog("ReinfType: ID" + num16.ToString() + ", Qty=" + num21.ToString() + ", Already transferred: " + num19.ToString() + "/" + num18.ToString() + ", LogPtsNeed: " + num23.ToString());
                        flag3 = true;
                        this.game.Data.UnitObj[unr].AddLog(2, num16, 0, 1);
                        this.game.Data.UnitObj[index2].AddLog(12, num16, 0, 1);
                        this.game.HandyFunctionsObj.AddTroops3(unr, index9, this.game.Data.SFObj[index5].People, 1, this.game.Data.SFObj[index5].Xp, this.game.Data.SFObj[index5].Rdn, 0, this.game.Data.SFObj[index5].Mor, vigor: this.game.Data.SFObj[index5].Vigor);
                        this.game.HandyFunctionsObj.RemoveTroops(index2, index9, this.game.Data.SFObj[index5].People, 1, -1);
                        SimpleList SL = SimpleList::new();
                        let mut tid1: i32 = this.game.Data.SFTypeObj[index9].SFTypeVar[45];
                        let mut tweight1: i32 = this.game.Data.SFTypeObj[index9].SFTypeVar[52];
                        if (tweight1 > 0)
                          SL.Add(tid1, tweight1);
                        let mut tid2: i32 = this.game.Data.SFTypeObj[index9].SFTypeVar[47];
                        let mut tweight2: i32 = this.game.Data.SFTypeObj[index9].SFTypeVar[53];
                        if (tweight2 > 0)
                          SL.Add(tid2, tweight2);
                        let mut tid3: i32 = this.game.Data.SFTypeObj[index9].SFTypeVar[50];
                        let mut tweight3: i32 = this.game.Data.SFTypeObj[index9].SFTypeVar[54];
                        if (tweight3 > 0)
                          SL.Add(tid3, tweight3);
                        if (this.game.Data.UnitObj[index2].items.list.CanRemoveWeight( SL))
                        {
                          this.game.Data.UnitObj[index2].items.list.RemoveWeight( SL);
                          this.game.Data.UnitObj[unr].items.list.AddWeight( SL);
                        }
                        this.game.Data.UnitObj[unr].tempHandledItems.AddWeight(num16, num21);
                        this.game.Data.UnitObj[unr].tempMaxItems.RemoveWeight(this.game.Data.SFTypeObj[index9].ReinforcementType, 1);
                        num11 -= num23;
                        this.game.Data.UnitObj[unr].AddLog(506, 1, 0, num23);
                        this.game.Data.UnitObj[index2].AddLog(506, 1, 0, num23);
                        bool ok = this.LIS_RemovePointsFromTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, num23).OK;
                        this.LIS_AddOrganicPointsToTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, num23, unitBestCoordinate.data2, unr);
                        this.AddPLog("--Removed " + num23.ToString() + " from trajectory " + unitBestCoordinate.x.ToString() + "," + unitBestCoordinate.y.ToString() + " <> " + this.game.Data.UnitObj[index2].Name);
                        if (ok & num23 > 0)
                        {
                          this.AddPLog("--0 reached--");
                          this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map);
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          if (flag3)
            flag2 = true;
        }
        if (num7 < 100)
          flag1 = true;
      }
      let mut counter: i32 = simpleList.Counter;
      for (let mut index11: i32 = 0; index11 <= counter; index11 += 1)
      {
        let mut index12: i32 = simpleList.Id[index11];
        let mut data3: i32 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index12].X, this.game.Data.UnitObj[index12].Y].LIShistory[6] - this.game.Data.UnitObj[index12].tempLisStartHistory;
        this.AddPLog("SHQ " + this.game.Data.UnitObj[index12].Name + " total LIS use " + data3.ToString() + " pts.");
        this.game.Data.UnitObj[index12].AddLog(506, 1, 0, data3);
      }
      this.WritePLog("LIS_AutoReinforce_Log");
      this.LIS_SetNetwork(true);
      let mut unitCounter6: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter6; index += 1)
        this.game.Data.UnitObj[index].tempComplexCoords = (ComplexCoordList) null;
    }

    pub fn LIS_UnitAutoReinforceReturns(bool normal, bool swappy)
    {
      let mut num1: i32 = 20;
      let mut id: i32 = this.game.Data.RegimeObj[this.game.Data.Turn].id;
      let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[412]));
      this.plogcounter = -1;
      this.AddPLog("LIS_AutoReinforceRETURNS");
      this.AddPLog("");
      let mut num2: i32 = 1;
      do
      {
        bool flag1 = false;
        if (normal)
          flag1 = true;
        if (swappy & num2 == 1)
          flag1 = true;
        if (flag1)
        {
          if (normal)
            this.AddPLog("NORMAL PHASE " + num2.ToString());
          if (swappy)
            this.AddPLog("SWAPPY PHASE " + num2.ToString());
          this.AddPLog("");
          let mut unitCounter1: i32 = this.game.Data.UnitCounter;
          index1: i32;
          for (let mut unr: i32 = 0; unr <= unitCounter1; unr += 1)
          {
            this.game.Data.UnitObj[unr].tempRequestItems = SimpleList::new();
            this.game.Data.UnitObj[unr].tempMaxItems = SimpleList::new();
            if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn & this.game.Data.UnitObj[unr].PreDef == -1 & this.game.HandyFunctionsObj.LIS_AutoReinforceRulesValid(unr) & (!this.game.HandyFunctionsObj.CheckIsBattlegroup(unr) | this.game.Data.UnitObj[unr].SOReplacementPercent == 0))
            {
              let mut index2: i32 = this.game.Data.UnitObj[unr].Historical;
              if (index2 > -1 && this.game.Data.UnitObj[unr].IsHQ & this.game.Data.HistoricalUnitObj[index2].TempVar1 == 1 && normal)
                index2 = -1;
              if (index2 > -1 && this.game.Data.HistoricalUnitObj[index2].GiveHisVarValue(11) < 1)
              {
                let mut preDef: i32 = this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index2].SubParts[0]);
                let mut index3: i32 = -1;
                if (this.game.Data.UnitObj[unr].HQ > -1)
                  index3 = this.game.Data.UnitObj[unr].HQ;
                if (index3 > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index3].Historical].Type < 8 && this.game.Data.UnitObj[index3].HQ > -1)
                {
                  index3 = this.game.Data.UnitObj[index3].HQ;
                  if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index3].Historical].Type < 8)
                    index3 = -1;
                }
                SimpleList SL = SimpleList::new();
                SimpleList simpleList1 = SimpleList::new();
                SimpleList simpleList2 = SimpleList::new();
                let mut historical: i32 = this.game.Data.UnitObj[unr].Historical;
                bool flag2 = false;
                bool flag3 = false;
                bool flag4 = false;
                bool flag5 = false;
                if (historical > -1)
                {
                  if (swappy)
                  {
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(42) != 1)
                      flag2 = true;
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(43) != 1)
                      flag3 = true;
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(44) != 1)
                      flag4 = true;
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(45) != 1)
                      flag5 = true;
                  }
                  else if (normal)
                  {
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(42) <= 1)
                      flag2 = true;
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(43) <= 1)
                      flag3 = true;
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(44) <= 1)
                      flag4 = true;
                    if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(45) <= 1)
                      flag5 = true;
                  }
                }
                if (preDef > -1)
                {
                  let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
                  for (let mut index4: i32 = 0; index4 <= sfCount; index4 += 1)
                  {
                    index1 = this.game.Data.UnitObj[unr].SFList[index4];
                    let mut type: i32 = this.game.Data.SFObj[index1].Type;
                    let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
                    let mut reinforcementType2: i32 = this.game.Data.SFTypeObj[type].ReinforcementType2;
                    let mut qty: i32 = this.game.Data.SFObj[index1].Qty;
                    let mut integer: i32 = Conversions.ToInteger(this.game.Data.StringListObj[stringListById].GetData2(1, id, 0, this.game.Data.SFTypeObj[type].Id, 2));
                    bool flag6 = false;
                    if (num2 == 2)
                      flag6 = true;
                    if (num2 == 1 & integer == 1)
                      flag6 = true;
                    if (num2 == 1 & integer == 2 & !flag2)
                      flag6 = true;
                    if (num2 == 1 & integer == 3 & !flag3)
                      flag6 = true;
                    if (num2 == 1 & integer == 4 & !flag4)
                      flag6 = true;
                    if (num2 == 1 & integer == 5 & !flag5)
                      flag6 = true;
                    if (flag6)
                    {
                      if (reinforcementType > -1 & qty > 0)
                        simpleList2.AddWeight(reinforcementType, qty, this.game.Data.SFTypeObj[type].Weight);
                      if (reinforcementType > -1 & qty > 0)
                        simpleList1.AddWeight(reinforcementType, qty);
                      if (reinforcementType2 > -1 & reinforcementType != reinforcementType2 & qty > 0)
                        simpleList1.AddWeight(reinforcementType2, qty);
                    }
                  }
                }
                if (preDef > -1 & num2 == 2)
                {
                  let mut sfCount: i32 = this.game.Data.UnitObj[preDef].SFCount;
                  for (let mut index5: i32 = 0; index5 <= sfCount; index5 += 1)
                  {
                    index1 = this.game.Data.UnitObj[preDef].SFList[index5];
                    let mut type: i32 = this.game.Data.SFObj[index1].Type;
                    let mut reinforcementType1: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
                    let mut reinforcementType2: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
                    let mut tweight: i32 = this.game.Data.SFObj[index1].Qty;
                    if (this.game.Data.UnitObj[unr].SOReplacementPercent == 0)
                      tweight = 0;
                    if (reinforcementType1 > -1 & tweight > 0)
                      SL.AddWeight(reinforcementType1, tweight);
                    if (reinforcementType1 > -1 & tweight > 0)
                      simpleList1.RemoveWeight(reinforcementType1, tweight);
                    if (reinforcementType2 > -1 & reinforcementType1 != reinforcementType2 & tweight > 0)
                      simpleList1.RemoveWeight(reinforcementType2, tweight);
                  }
                }
                simpleList2.RemoveWeight( SL);
                simpleList2.removeWeight0orLower();
                simpleList1.removeWeight0orLower();
                SL = simpleList2.Clone();
                if (SL.Counter > -1)
                {
                  this.game.Data.UnitObj[unr].tempRequestItems = SL;
                  let mut counter: i32 = SL.Counter;
                  for (let mut index6: i32 = 0; index6 <= counter; index6 += 1)
                  {
                    if (normal)
                    {
                      this.game.Data.UnitObj[unr].AddLog(10, SL.Id[index6], 0, SL.Weight[index6]);
                      if (index3 > -1)
                        this.game.Data.UnitObj[index3].AddLog(18, SL.Id[index6], 0, SL.Weight[index6]);
                    }
                    else
                    {
                      let mut num3: i32 = swappy ? 1 : 0;
                    }
                  }
                }
                if (simpleList1.Counter > -1)
                  this.game.Data.UnitObj[unr].tempMaxItems = simpleList1;
              }
            }
          }
          let mut unitCounter2: i32 = this.game.Data.UnitCounter;
          for (let mut index7: i32 = 0; index7 <= unitCounter2; index7 += 1)
          {
            this.game.Data.UnitObj[index7].tempCoords = (CoordList) null;
            this.game.Data.UnitObj[index7].tempComplexCoords = (ComplexCoordList) null;
          }
          SimpleList simpleList = SimpleList::new();
          let mut unitCounter3: i32 = this.game.Data.UnitCounter;
          data3: i32;
          for (let mut index8: i32 = 0; index8 <= unitCounter3; index8 += 1)
          {
            if (this.game.Data.UnitObj[index8].IsHQ & this.game.Data.UnitObj[index8].Regime == this.game.Data.Turn)
            {
              let mut historical1: i32 = this.game.Data.UnitObj[index8].Historical;
              if (historical1 > -1 && this.game.Data.HistoricalUnitObj[historical1].Type == 8)
              {
                bool flag7 = false;
                let mut unitCounter4: i32 = this.game.Data.UnitCounter;
                for (let mut unr: i32 = 0; unr <= unitCounter4; unr += 1)
                {
                  if (this.game.HandyFunctionsObj.IsUnitInHQChain(unr, index8) & this.game.Data.UnitObj[unr].Historical > -1 & unr != index8)
                  {
                    flag7 = true;
                    let mut historical2: i32 = this.game.Data.UnitObj[unr].Historical;
                    let mut num4: i32 = this.game.Data.HistoricalUnitObj[historical2].GiveHisVarValue(71);
                    let mut movetype: i32 = this.game.Data.HistoricalUnitObj[historical2].GiveHisVarValue(72);
                    this.game.Data.UnitObj[unr].tempCoords = CoordList::new();
                    this.game.Data.UnitObj[unr].tempComplexCoords = ComplexCoordList::new();
                    this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 0, num4 * 2, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, false, SeaBlock: true, BlockAllSea: true);
                    let mut mapWidth: i32 = this.game.Data.MapObj[0].MapWidth;
                    for (let mut x: i32 = 0; x <= mapWidth; x += 1)
                    {
                      let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
                      for (let mut y: i32 = 0; y <= mapHeight; y += 1)
                      {
                        if (this.game.EditObj.TempValue[0].Value[x, y] <= 2 * num4)
                        {
                          data3 = this.game.EditObj.TempValue[0].Value[x, y];
                          let mut dat2_1: i32 = 0;
                          if (data3 <= num4)
                            dat2_1 = 100;
                          else if (data3 < num4 * 2)
                            dat2_1 = 100 -  Math.Round( (100 * (data3 - num4)) /  num4);
                          CoordList tCoordList = new CoordList(true);
                          Coordinate coordinate;
                          coordinate.onmap = true;
                          coordinate.x = x;
                          coordinate.y = y;
                          coordinate = this.game.EditObj.TempCameFrom[0].Value[x, y];
                          while (coordinate.onmap)
                          {
                            let mut dat1: i32 = this.game.EditObj.TempValue[0].Value[coordinate.x, coordinate.y];
                            let mut dat2_2: i32 = 0;
                            if (dat1 <= num4)
                              dat2_2 = 100;
                            else if (dat1 < num4 * 2)
                              dat2_2 = 100 -  Math.Round( (100 * (dat1 - num4)) /  num4);
                            if (tCoordList.FindSlot(coordinate.x, coordinate.y, coordinate.map) == -1)
                            {
                              tCoordList.AddCoord(coordinate.x, coordinate.y, coordinate.map, dat1, dat2_2);
                              coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y];
                            }
                            else
                              coordinate.onmap = false;
                          }
                          this.game.Data.UnitObj[unr].tempComplexCoords.AddCoord(x, y, 0, this.game.EditObj.TempValue[0].Value[x, y], dat2_1, tCoordList);
                          this.game.Data.UnitObj[unr].tempCoords.AddCoord(x, y, 0, this.game.EditObj.TempValue[0].Value[x, y], dat2_1);
                        }
                      }
                    }
                  }
                }
                if (flag7)
                {
                  if (Information.IsNothing( this.game.Data.UnitObj[index8].items))
                    this.game.Data.UnitObj[index8].items = ItemList::new();
                  this.game.Data.UnitObj[index8].tempLisStartHistory = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index8].X, this.game.Data.UnitObj[index8].Y].LIShistory[6];
                  simpleList.Add(index8, 1);
                }
              }
            }
          }
          let mut unitCounter5: i32 = this.game.Data.UnitCounter;
          for (let mut index9: i32 = 0; index9 <= unitCounter5; index9 += 1)
            this.game.Data.UnitObj[index9].tempHandledItems = SimpleList::new();
          bool flag8 = true;
          let mut num5: i32 = 0;
          let mut num6: i32 = 0;
          while (flag8)
          {
            num5 += 1;
            num6 += num1;
            flag8 = false;
            bool flag9 = true;
            this.AddPLog(num5.ToString() + ".Superloop. CurMaxPercent=" + num6.ToString());
            let mut num7: i32 = 0;
            while (flag9)
            {
              flag9 = false;
              num7 += 1;
              this.AddPLog("ShqLoop");
              let mut counter1: i32 = simpleList.Counter;
              bool flag10;
              for (let mut index10: i32 = 0; index10 <= counter1; index10 += 1)
              {
                let mut index11: i32 = simpleList.Id[index10];
                let mut liSpoint: i32 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index11].X, this.game.Data.UnitObj[index11].Y].LISpoints[6];
                let mut num8: i32 = this.game.Data.UnitObj[index11].lisInstructions.FindWeight(2) + this.game.Data.UnitObj[index11].lisInstructions.FindWeight(3) + this.game.Data.UnitObj[index11].lisInstructions.FindWeight(5) + this.game.Data.UnitObj[index11].lisInstructions.FindWeight(6);
                this.AddPLog("SHQ Max Percentage Allowed " + num8.ToString() + "%");
                if (num8 > 100)
                  num8 = 100;
                let mut num9: i32 =  Math.Round( (liSpoint * num8) / 100.0);
                data3 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index11].X, this.game.Data.UnitObj[index11].Y].LIShistory[6] - this.game.Data.UnitObj[index11].tempLisStartHistory;
                let mut num10: i32 = num9 - data3;
                if (num10 < 1)
                  num10 = 0;
                if (0 > num10)
                  num10 = 0;
                let mut num11: i32 =  Math.Round( (num10 * num6) / 100.0);
                this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index11].X, this.game.Data.UnitObj[index11].Y, this.game.Data.UnitObj[index11].Map);
                if (this.game.Data.Turn == 2)
                  index10 = index10;
                this.AddPLog(num5.ToString() + "." + num7.ToString() + ".Shq: " + this.game.Data.UnitObj[index11].Name + ", CurrentLisAvailable: " + num11.ToString());
                flag10 = true;
                let mut num12: i32 = 0;
                let mut num13: i32 = 0;
                while (flag10)
                {
                  flag10 = false;
                  num12 += 1;
                  num13 += 10;
                  if (num13 > 100)
                    num13 = 100;
                  if (num13 < 100)
                    flag10 = true;
                  let mut unitCounter6: i32 = this.game.Data.UnitCounter;
                  for (let mut unr: i32 = 0; unr <= unitCounter6; unr += 1)
                  {
                    if (this.game.HandyFunctionsObj.IsUnitInHQChain(unr, index11) & (!this.game.HandyFunctionsObj.CheckIsBattlegroup(unr) | this.game.Data.UnitObj[unr].SOReplacementPercent == 0))
                    {
                      bool flag11 = false;
                      if (this.game.Data.UnitObj[index11].X == this.game.Data.UnitObj[unr].X & this.game.Data.UnitObj[index11].Y == this.game.Data.UnitObj[unr].Y)
                        flag11 = true;
                      this.AddPLog(num5.ToString() + "." + num7.ToString() + "." + num12.ToString() + ".Unit: " + this.game.Data.UnitObj[unr].Name);
                      let mut counter2: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Counter;
                      for (let mut index12: i32 = 0; index12 <= counter2; index12 += 1)
                      {
                        num14: i32;
                        if (num12 == 1 & num7 == 1 & num5 == 1)
                        {
                          let mut num15: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Id[index12];
                          num14 = this.game.Data.UnitObj[unr].tempRequestItems.Weight[index12];
                        }
                        if (num11 > 0 | flag11)
                        {
                          let mut num16: i32 = this.game.Data.UnitObj[unr].tempRequestItems.Id[index12];
                          num14 = this.game.Data.UnitObj[unr].tempRequestItems.Weight[index12];
                          let mut num17: i32 = Math.Max(1, this.game.Data.UnitObj[unr].tempRequestItems.Data1[index12]);
                          data3 = this.game.Data.UnitObj[unr].tempHandledItems.FindNr(num16);
                          let mut num18: i32 = 0;
                          if (data3 > -1)
                            num18 = this.game.Data.UnitObj[unr].tempHandledItems.Weight[data3];
                          let mut num19: i32 =  Math.Round(Math.Ceiling( num14 * ( num6 / 100.0) * ( num13 / 100.0)));
                          if (num14 > 0 & num19 == 0)
                            num19 = 1;
                          let mut num20: i32 = num19 - num18;
                          let mut num21: i32 = num17;
                          if (flag11)
                            num21 = 0;
                          if (num21 > num11)
                          {
                            num20 =  Math.Round(Math.Floor( (num20 * num11) /  num21));
                            num21 = num11;
                          }
                          let mut index13: i32 = -1;
                          index14: i32;
                          type: i32;
                          if (num20 > 0)
                          {
                            bool quality2_1 = false;
                            bool quality3_1 = false;
                            bool quality4_1 = false;
                            bool quality5_1 = false;
                            bool quality2_2 = false;
                            bool quality3_2 = false;
                            bool quality4_2 = false;
                            bool quality5_2 = false;
                            let mut historical: i32 = this.game.Data.UnitObj[unr].Historical;
                            if (this.game.Data.Turn == 2)
                              quality2_1 = quality2_1;
                            if (historical > -1)
                            {
                              if (swappy)
                              {
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(42) != 1)
                                  quality2_1 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(43) != 1)
                                  quality3_1 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(44) != 1)
                                  quality4_1 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(45) != 1)
                                  quality5_1 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(42) == 0)
                                  quality2_2 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(43) == 0)
                                  quality3_2 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(44) == 0)
                                  quality4_2 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(45) == 0)
                                  quality5_2 = true;
                              }
                              else if (normal)
                              {
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(42) <= 1)
                                  quality2_1 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(43) <= 1)
                                  quality3_1 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(44) <= 1)
                                  quality4_1 = true;
                                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(45) <= 1)
                                  quality5_1 = true;
                              }
                            }
                            data3 = -1;
                            let mut num22: i32 = -1;
                            switch (num2)
                            {
                              case 1:
                                data3 = Conversions.ToInteger(this.LIS_GetUnitSFObjNrForReturn(unr, quality2_1, quality3_1, quality4_1, quality5_1, num16, true));
                                break;
                              case 2:
                                data3 = Conversions.ToInteger(this.LIS_GetUnitSFObjNrForReturn(unr, quality2_1, quality3_1, quality4_1, quality5_1, num16, false));
                                break;
                            }
                            if (swappy)
                            {
                              num22 = data3 <= -1 ? -1 : Conversions.ToInteger(this.LIS_GetSHQSFObjNr(index11, quality2_2, quality3_2, quality4_2, quality5_2, num16, this.game.Data.SFObj[data3].People));
                              if (num22 == -1)
                                data3 = -1;
                            }
                            if (data3 > -1 && this.game.Data.UnitObj[unr].tempMaxItems.FindWeight(this.game.Data.SFTypeObj[this.game.Data.SFObj[data3].Type].ReinforcementType) < 1)
                              data3 = -1;
                            index1 = data3;
                            index14 = num22;
                            if (index1 > -1)
                              index13 = this.game.Data.SFObj[index1].Type;
                            if (index14 > -1)
                              type = this.game.Data.SFObj[index14].Type;
                            if (swappy)
                            {
                              if (type > -1)
                                num21 = this.game.Data.SFTypeObj[type].Weight * 1;
                            }
                            else if (normal && index13 > -1)
                              num21 = this.game.Data.SFTypeObj[index13].Weight * 1;
                            if (data3 > -1)
                            {
                              num20 = 1;
                              if (num14 < 1)
                                num20 = 0;
                            }
                            else
                              num20 = 0;
                          }
                          Coordinate unitBestCoordinate;
                          if (num20 > 0)
                          {
                            unitBestCoordinate = this.LIS_GetUnitBestCoordinate(this.game.Data.UnitObj[unr].tempCoords, num20, true, this.game.Data.UnitObj[index11].X, this.game.Data.UnitObj[index11].Y);
                            if (unitBestCoordinate.onmap)
                            {
                              this.AddPLog("Pickup adress: " + unitBestCoordinate.x.ToString() + "," + unitBestCoordinate.y.ToString());
                              data3 = this.LIS_GetLowestPointsOnTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, this.game.Data.UnitObj[index11].X, this.game.Data.UnitObj[index11].Y);
                              if (data3 < num21)
                                num20 = 0;
                            }
                            else
                            {
                              this.AddPLog("No pickup hex found for unit.");
                              num20 = 0;
                            }
                          }
                          if (num20 > 0)
                          {
                            this.AddPLog("ReinfType: ID" + num16.ToString() + ", Qty=" + num20.ToString() + ", Already transferred: " + num18.ToString() + "/" + num14.ToString() + ", LogPtsNeed: " + num21.ToString());
                            flag10 = true;
                            this.game.Data.UnitObj[index11].AddLog(8, num16, 0, 1);
                            this.game.Data.UnitObj[unr].AddLog(3, num16, 0, 1);
                            if (swappy)
                            {
                              this.game.Data.UnitObj[index11].AddLog(18, num16, 0, 1);
                              this.game.Data.UnitObj[unr].AddLog(10, num16, 0, 1);
                              this.game.Data.UnitObj[unr].AddLog(1, num16, 0, 1);
                              this.game.Data.UnitObj[unr].AddLog(2, num16, 0, 1);
                              this.game.Data.UnitObj[index11].AddLog(11, num16, 0, 1);
                              this.game.Data.UnitObj[index11].AddLog(12, num16, 0, 1);
                            }
                            if (!swappy)
                            {
                              this.game.HandyFunctionsObj.AddTroops3(index11, index13, this.game.Data.SFObj[index1].People, 1, this.game.Data.SFObj[index1].Xp, this.game.Data.SFObj[index1].Rdn, 0, this.game.Data.SFObj[index1].Mor, vigor: this.game.Data.SFObj[index1].Vigor);
                              this.game.HandyFunctionsObj.RemoveTroops(unr, index13, this.game.Data.SFObj[index1].People, 1, -1);
                            }
                            else if (swappy)
                            {
                              this.game.HandyFunctionsObj.AddTroops3(index11, index13, this.game.Data.SFObj[index14].People, 1, this.game.Data.SFObj[index14].Xp, this.game.Data.SFObj[index14].Rdn, 0, this.game.Data.SFObj[index14].Mor, vigor: this.game.Data.SFObj[index14].Vigor);
                              this.game.HandyFunctionsObj.RemoveTroops(unr, index13, this.game.Data.SFObj[index1].People, 1, -1);
                              this.game.HandyFunctionsObj.AddTroops3(unr, type, this.game.Data.SFObj[index1].People, 1, this.game.Data.SFObj[index1].Xp, this.game.Data.SFObj[index1].Rdn, 0, this.game.Data.SFObj[index1].Mor, vigor: this.game.Data.SFObj[index1].Vigor);
                              this.game.HandyFunctionsObj.RemoveTroops(index11, type, this.game.Data.SFObj[index14].People, 1, -1);
                            }
                            SimpleList SL = SimpleList::new();
                            let mut tid1: i32 = this.game.Data.SFTypeObj[index13].SFTypeVar[45];
                            let mut tweight1: i32 = this.game.Data.SFTypeObj[index13].SFTypeVar[52];
                            if (tweight1 > 0)
                              SL.Add(tid1, tweight1);
                            let mut tid2: i32 = this.game.Data.SFTypeObj[index13].SFTypeVar[47];
                            let mut tweight2: i32 = this.game.Data.SFTypeObj[index13].SFTypeVar[53];
                            if (tweight2 > 0)
                              SL.Add(tid2, tweight2);
                            let mut tid3: i32 = this.game.Data.SFTypeObj[index13].SFTypeVar[50];
                            let mut tweight3: i32 = this.game.Data.SFTypeObj[index13].SFTypeVar[54];
                            if (tweight3 > 0)
                              SL.Add(tid3, tweight3);
                            if (this.game.Data.UnitObj[unr].items.list.CanRemoveWeight( SL))
                            {
                              this.game.Data.UnitObj[unr].items.list.RemoveWeight( SL);
                              this.game.Data.UnitObj[index11].items.list.AddWeight( SL);
                            }
                            if (swappy)
                            {
                              let mut tid4: i32 = this.game.Data.SFTypeObj[type].SFTypeVar[45];
                              let mut tweight4: i32 = this.game.Data.SFTypeObj[type].SFTypeVar[52];
                              if (tweight4 > 0)
                                SL.Add(tid4, tweight4);
                              let mut tid5: i32 = this.game.Data.SFTypeObj[type].SFTypeVar[47];
                              let mut tweight5: i32 = this.game.Data.SFTypeObj[type].SFTypeVar[53];
                              if (tweight5 > 0)
                                SL.Add(tid5, tweight5);
                              let mut tid6: i32 = this.game.Data.SFTypeObj[type].SFTypeVar[50];
                              let mut tweight6: i32 = this.game.Data.SFTypeObj[type].SFTypeVar[54];
                              if (tweight6 > 0)
                                SL.Add(tid6, tweight6);
                              if (this.game.Data.UnitObj[index11].items.list.CanRemoveWeight( SL))
                              {
                                this.game.Data.UnitObj[index11].items.list.RemoveWeight( SL);
                                this.game.Data.UnitObj[unr].items.list.AddWeight( SL);
                              }
                            }
                            this.game.Data.UnitObj[unr].tempHandledItems.AddWeight(num16, num20);
                            num11 -= num21;
                            this.game.Data.UnitObj[unr].AddLog(506, 1, 0, num21);
                            bool ok = this.LIS_RemovePointsFromTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, num21).OK;
                            this.LIS_AddOrganicPointsToTrajectory(unitBestCoordinate.x, unitBestCoordinate.y, num21, unitBestCoordinate.data2, unr);
                            this.AddPLog("--Removed " + num21.ToString() + " from trajectory " + unitBestCoordinate.x.ToString() + "," + unitBestCoordinate.y.ToString() + " <> " + this.game.Data.UnitObj[index11].Name);
                            if (ok & num21 > 0)
                            {
                              this.AddPLog("--0 reached--");
                              this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index11].X, this.game.Data.UnitObj[index11].Y, this.game.Data.UnitObj[index11].Map);
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
              if (flag10)
                flag9 = true;
            }
            if (num6 < 100)
              flag8 = true;
          }
          let mut counter3: i32 = simpleList.Counter;
          for (let mut index15: i32 = 0; index15 <= counter3; index15 += 1)
          {
            let mut index16: i32 = simpleList.Id[index15];
            data3 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index16].X, this.game.Data.UnitObj[index16].Y].LIShistory[6] - this.game.Data.UnitObj[index16].tempLisStartHistory;
            this.AddPLog("SHQ " + this.game.Data.UnitObj[index16].Name + " total LIS use " + data3.ToString() + " pts.");
            this.game.Data.UnitObj[index16].AddLog(506, 1, 0, data3);
          }
        }
        num2 += 1;
      }
      while (num2 <= 2);
      this.WritePLog("LIS_AutoReinforceRETURNS_Log");
      this.LIS_SetNetwork(true);
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut index: i32 = 0; index <= unitCounter; index += 1)
        this.game.Data.UnitObj[index].tempComplexCoords = (ComplexCoordList) null;
    }

    pub object LIS_GetSHQSFObjNr(
      shqNr: i32,
      bool quality2,
      bool quality3,
      bool quality4,
      bool quality5,
      reinfType: i32,
      ppl: i32)
    {
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[412]));
      let mut id: i32 = this.game.Data.RegimeObj[this.game.Data.UnitObj[shqNr].Regime].id;
      let mut sfCount: i32 = this.game.Data.UnitObj[shqNr].SFCount;
      for (let mut index: i32 = 0; index <= sfCount; index += 1)
      {
        let mut sf: i32 = this.game.Data.UnitObj[shqNr].SFList[index];
        let mut type: i32 = this.game.Data.SFObj[sf].Type;
        let mut qty: i32 = this.game.Data.SFObj[sf].Qty;
        let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
        let mut people: i32 = this.game.Data.SFObj[sf].People;
        bool flag1 = false;
        if (this.game.Data.PeopleObj[people].tv0 == this.game.Data.PeopleObj[ppl].tv0 && this.game.Data.PeopleObj[people].tv1 > 0 & this.game.Data.PeopleObj[ppl].tv1 > 0 & this.game.Data.PeopleObj[people].tv1 < 9 & this.game.Data.PeopleObj[ppl].tv1 < 9 | this.game.Data.PeopleObj[people].tv1 > 10 & this.game.Data.PeopleObj[ppl].tv1 > 10 & this.game.Data.PeopleObj[people].tv1 < 19 & this.game.Data.PeopleObj[ppl].tv1 < 19)
          flag1 = true;
        if (Strings.InStr(this.game.Data.PeopleObj[people].Name.ToLower(), "robotic") > 0 && this.game.Data.PeopleObj[people].tv1 > 0 & this.game.Data.PeopleObj[ppl].tv1 > 0 & this.game.Data.PeopleObj[people].tv1 < 9 & this.game.Data.PeopleObj[ppl].tv1 < 9 | this.game.Data.PeopleObj[people].tv1 > 10 & this.game.Data.PeopleObj[ppl].tv1 > 10 & this.game.Data.PeopleObj[people].tv1 < 19 & this.game.Data.PeopleObj[ppl].tv1 < 19)
          flag1 = true;
        if (flag1 && reinforcementType == reinfType)
        {
          bool flag2 = false;
          let mut integer: i32 = Conversions.ToInteger(this.game.Data.StringListObj[stringListById].GetData2(0, this.game.Data.SFTypeObj[type].Id, 1, id, 2));
          if (integer == 0)
            flag2 = true;
          if (integer == 2 & quality2)
            flag2 = true;
          if (integer == 3 & quality3)
            flag2 = true;
          if (integer == 4 & quality4)
            flag2 = true;
          if (integer == 5 & quality5)
            flag2 = true;
          if (this.game.Data.PeopleObj[people].tv1 > 10 & this.game.Data.PeopleObj[ppl].tv1 > 10 & this.game.Data.PeopleObj[people].tv1 < 19 & this.game.Data.PeopleObj[ppl].tv1 < 19)
            flag2 = true;
          if (flag2)
          {
            let mut num: i32 = this.game.Data.ReinfId[reinforcementType];
            let mut weight: i32 = this.game.Data.SFTypeObj[type].Weight;
            simpleList1.AddWeight(sf, DrawMod.RandyNumber.Next(0, qty + 1));
          }
        }
      }
      simpleList1.ReverseSortHighSpeed();
      return simpleList1.Counter == -1 ?  -1 :  simpleList1.Id[0];
    }

    pub object LIS_GetUnitSFObjNrForReturn(
      unr: i32,
      bool quality2,
      bool quality3,
      bool quality4,
      bool quality5,
      reinfType: i32,
      bool checkQuality)
    {
      SimpleList simpleList1 = SimpleList::new();
      SimpleList simpleList2 = SimpleList::new();
      let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[412]));
      let mut id: i32 = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].id;
      let mut sfCount: i32 = this.game.Data.UnitObj[unr].SFCount;
      for (let mut index: i32 = 0; index <= sfCount; index += 1)
      {
        let mut sf: i32 = this.game.Data.UnitObj[unr].SFList[index];
        let mut type: i32 = this.game.Data.SFObj[sf].Type;
        let mut qty: i32 = this.game.Data.SFObj[sf].Qty;
        let mut reinforcementType: i32 = this.game.Data.SFTypeObj[type].ReinforcementType;
        if (reinforcementType == reinfType)
        {
          bool flag = true;
          let mut integer: i32 = Conversions.ToInteger(this.game.Data.StringListObj[stringListById].GetData2(0, this.game.Data.SFTypeObj[type].Id, 1, id, 2));
          if (integer == 0)
            flag = false;
          if (integer == 2 & quality2)
            flag = false;
          if (integer == 3 & quality3)
            flag = false;
          if (integer == 4 & quality4)
            flag = false;
          if (integer == 5 & quality5)
            flag = false;
          if (!checkQuality)
            flag = true;
          if (flag)
          {
            let mut num: i32 = this.game.Data.ReinfId[reinforcementType];
            let mut weight: i32 = this.game.Data.SFTypeObj[type].Weight;
            simpleList1.AddWeight(sf, DrawMod.RandyNumber.Next(0, qty + 1));
          }
        }
      }
      simpleList1.ReverseSortHighSpeed();
      return simpleList1.Counter == -1 ?  -1 :  simpleList1.Id[0];
    }

    pub fn LIS_LocationReturns(bool freeOfCost) => this.LIS_UniversalSupplyAndReturn(1, freeOfCost);

    pub fn LIS_UniversalSupplyAndReturn(mode: i32, bool freeOfCost)
    {
      let mut num1: i32 = 20;
      bool flag1;
      if (mode == 1)
        flag1 = true;
      bool flag2;
      if (mode == 2)
        flag2 = true;
      let mut id: i32 =  Math.Round( this.game.Data.RuleVar[404]);
      let mut stringListById: i32 = this.game.HandyFunctionsObj.GetStringListByID(id);
      int[] numArray1 = new int[this.game.Data.StringListObj[stringListById].GetHighestValue(0) + 1 + 1];
      let mut length: i32 = this.game.Data.StringListObj[stringListById].Length;
      for (let mut index: i32 = 0; index <= length; index += 1)
        numArray1[ Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index, 0]))] =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index, 3]));
      this.plogcounter = -1;
      if (flag1)
        this.AddPLog("LIS_LocationReturns");
      if (flag2)
        this.AddPLog("LIS_LocationSupply");
      this.AddPLog("");
      if (this.game.Data.Turn == 2)
        ;
      SimpleList simpleList = SimpleList::new();
      let mut unitCounter: i32 = this.game.Data.UnitCounter;
      for (let mut tid: i32 = 0; tid <= unitCounter; tid += 1)
      {
        if (this.game.Data.UnitObj[tid].IsHQ & this.game.Data.UnitObj[tid].Regime == this.game.Data.Turn)
        {
          let mut historical: i32 = this.game.Data.UnitObj[tid].Historical;
          if (historical > -1 && this.game.Data.HistoricalUnitObj[historical].Type == 8)
          {
            bool flag3 = false;
            let mut locCounter: i32 = this.game.Data.LocCounter;
            for (let mut index: i32 = 0; index <= locCounter; index += 1)
            {
              if (this.game.Data.LocObj[index].HQ == tid)
              {
                flag3 = true;
                break;
              }
            }
            if (flag3)
            {
              if (Information.IsNothing( this.game.Data.UnitObj[tid].items))
                this.game.Data.UnitObj[tid].items = ItemList::new();
              this.game.Data.UnitObj[tid].tempLisStartHistory = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[tid].X, this.game.Data.UnitObj[tid].Y].LIShistory[6];
              simpleList.Add(tid, 1);
            }
          }
        }
      }
      let mut locCounter1: i32 = this.game.Data.LocCounter;
      for (let mut index: i32 = 0; index <= locCounter1; index += 1)
        this.game.Data.LocObj[index].tempHandledItems = SimpleList::new();
      bool flag4 = true;
      let mut num2: i32 = 0;
      let mut num3: i32 = 0;
      while (flag4)
      {
        num2 += 1;
        num3 += num1;
        flag4 = false;
        bool flag5 = true;
        this.AddPLog(num2.ToString() + ".Superloop. CurMaxPercent=" + num3.ToString());
        let mut num4: i32 = 0;
        while (flag5)
        {
          flag5 = false;
          num4 += 1;
          this.AddPLog("ShqLoop");
          let mut counter1: i32 = simpleList.Counter;
          bool flag6;
          for (let mut index1: i32 = 0; index1 <= counter1; index1 += 1)
          {
            let mut index2: i32 = simpleList.Id[index1];
            let mut num5: i32 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y].LISpoints[6] + this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y].LIShistory[6];
            if (this.game.Data.Turn == 2 & flag1)
              ;
            let mut num6: i32 = 0;
            if (flag2)
              num6 = this.game.Data.UnitObj[index2].lisInstructions.FindWeight(2);
            if (flag1)
              num6 = this.game.Data.UnitObj[index2].lisInstructions.FindWeight(2) + this.game.Data.UnitObj[index2].lisInstructions.FindWeight(3) + this.game.Data.UnitObj[index2].lisInstructions.FindWeight(5);
            if (num6 > 100)
              num6 = 100;
            let mut num7: i32 = 0;
            if (mode == 2)
              num7 = this.game.Data.UnitObj[index2].lisInstructions.FindWeight(14);
            if (mode == 1)
              num7 = this.game.Data.UnitObj[index2].lisInstructions.FindWeight(15);
            bool flag7 = false;
            if (this.game.EventRelatedObj.Helper_AirEnabled())
              flag7 = true;
            if (num7 < 1)
              flag7 = false;
            let mut weight: i32 = this.game.Data.UnitObj[index2].lisInstructions.FindWeight(12);
            this.AddPLog("SHQ Max Percentage Allowed " + num6.ToString() + "%");
            let mut num8: i32 =  Math.Round( (num5 * num6) / 100.0);
            id = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y].LIShistory[6] - this.game.Data.UnitObj[index2].tempLisStartHistory;
            let mut data3_1: i32 = num8 - id;
            if (data3_1 < 1)
              data3_1 = 0;
            if (num4 == 1 & num2 == 1)
            {
              if (this.game.Data.Turn == 2)
                index1 = index1;
              if (flag1)
                this.game.Data.UnitObj[index2].AddLog(605, 1, 0, data3_1);
              else
                this.game.Data.UnitObj[index2].AddLog(602, 1, 0, data3_1);
            }
            if (0 > data3_1)
              data3_1 = 0;
            let mut num9: i32 =  Math.Round( (data3_1 * num3) / 100.0);
            numArray2: Vec<i32> = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
            this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, addHistoryToCurrent: true);
            let mut mapWidth: i32 = this.game.Data.MapObj[0].MapWidth;
            for (let mut index3: i32 = 0; index3 <= mapWidth; index3 += 1)
            {
              let mut mapHeight: i32 = this.game.Data.MapObj[0].MapHeight;
              for (let mut index4: i32 = 0; index4 <= mapHeight; index4 += 1)
                numArray2[index3, index4] = this.game.EditObj.TempValue[0].Value[index3, index4];
            }
            if (flag7 & num7 == 1)
            {
              if (!freeOfCost)
                this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, useAirBridge: true, maxDam: weight);
            }
            else if (!freeOfCost)
              this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map);
            this.AddPLog(num2.ToString() + "." + num4.ToString() + ".Shq: " + this.game.Data.UnitObj[index2].Name + ", CurrentLisAvailable: " + num9.ToString());
            flag6 = true;
            let mut num10: i32 = 0;
            let mut num11: i32 = 0;
            while (flag6)
            {
              flag6 = false;
              num10 += 1;
              num11 += 10;
              if (num11 > 100)
                num11 = 100;
              if (num11 < 100)
                flag6 = true;
              let mut locCounter2: i32 = this.game.Data.LocCounter;
              for (let mut index5: i32 = 0; index5 <= locCounter2; index5 += 1)
              {
                if (this.game.Data.LocObj[index5].HQ == index2)
                {
                  bool flag8 = false;
                  if (this.game.Data.UnitObj[index2].X == this.game.Data.LocObj[index5].X & this.game.Data.UnitObj[index2].Y == this.game.Data.LocObj[index5].Y)
                    flag8 = true;
                  this.AddPLog(num2.ToString() + "." + num4.ToString() + "." + num10.ToString() + ".Loc: " + this.game.Data.LocObj[index5].Name);
                  if (!Information.IsNothing( this.game.Data.LocObj[index5].tempRequestItems))
                  {
                    if (this.game.Data.Turn == 2)
                      index1 = index1;
                    let mut counter2: i32 = this.game.Data.LocObj[index5].tempRequestItems.Counter;
                    for (let mut index6: i32 = 0; index6 <= counter2; index6 += 1)
                    {
                      num12: i32;
                      if (!freeOfCost & num10 == 1 & num4 == 1 & num2 == 1)
                      {
                        let mut data1: i32 = this.game.Data.LocObj[index5].tempRequestItems.Id[index6];
                        let mut data3_2: i32 = this.game.Data.LocObj[index5].tempRequestItems.Weight[index6];
                        num12 = numArray1[data1];
                        if (flag1)
                        {
                          this.game.Data.LocObj[index5].AddLog(202, data1, 0, data3_2);
                          this.game.Data.UnitObj[index2].AddLog(212, data1, 0, data3_2);
                        }
                        else
                        {
                          this.game.Data.LocObj[index5].AddLog(201, data1, 0, data3_2);
                          this.game.Data.UnitObj[index2].AddLog(211, data1, 0, data3_2);
                        }
                      }
                      if (num9 >= num12 | freeOfCost | flag8)
                      {
                        let mut index7: i32 = this.game.Data.LocObj[index5].tempRequestItems.Id[index6];
                        let mut num13: i32 =  this.game.Data.LocObj[index5].tempRequestItems.Weight[index6];
                        num12 = numArray1[index7];
                        id = this.game.Data.LocObj[index5].tempHandledItems.FindNr(index7);
                        let mut num14: i32 =  0;
                        if (id > -1)
                          num14 = this.game.Data.LocObj[index5].tempHandledItems.Weight[id];
                        let mut num15: i32 =   Math.Round(Math.Ceiling( num13 * ( num3 / 100.0) * ( num11 / 100.0)));
                        if (num13 > 0 & num15 == 0)
                          num15 = 1;
                        let mut num16: i32 =  num15 - num14;
                        if (flag7 & !flag8 && num12 < 1 && this.LIS_HasAirBridgeOnTrajectory(this.game.Data.LocObj[index5].X, this.game.Data.LocObj[index5].Y, this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y))
                        {
                          num12 = 1;
                          if (this.IsItemMaximizer(index7))
                            num16 = 0;
                        }
                        let mut num17: i32 =  num16;
                        if (flag8)
                          num17 = 0;
                        if (!freeOfCost && num17 > num9)
                        {
                          num16 = num12 <= 0 ? num13 - num14 :  Math.Round(Math.Floor( (num16 * num9) /  (num12 * num17)));
                          num17 = num9;
                        }
                        if (!flag1)
                        {
                          id = this.game.Data.UnitObj[index2].items.list.FindNr(index7);
                          if (id > -1)
                          {
                            let mut num18: i32 =  this.game.Data.UnitObj[index2].items.list.Weight[id];
                            if (num18 < 0)
                              num18 = 0;
                            if (num16 > num18)
                              num16 = num18;
                          }
                          else
                            num16 = 0;
                        }
                        if (!freeOfCost & num16 > 0 & !flag8)
                        {
                          id = this.LIS_GetLowestPointsOnTrajectory(this.game.Data.LocObj[index5].X, this.game.Data.LocObj[index5].Y, this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, true);
                          if (num12 < 1 | flag8 && id < 0 && numArray2[this.game.Data.LocObj[index5].X, this.game.Data.LocObj[index5].Y] > 0)
                            id = 1;
                          if (id < 0)
                          {
                            id = 0;
                            if (num12 == 0)
                              id = id;
                            num16 = 0;
                          }
                          else if (id < num17)
                          {
                            if (num12 > 0)
                            {
                              num16 =  Math.Round(Math.Floor( (num16 * id) /  (num12 * num17)));
                              if (num12 == 0)
                                id = id;
                            }
                            else
                              num16 = num16;
                            num17 = id;
                          }
                          else if (num12 > 0 & !flag8 & id < 1)
                          {
                            id = 0;
                            num16 = 0;
                          }
                        }
                        if (num16 > 0)
                        {
                          if (this.game.Data.Turn == 2 & mode == 2)
                            index1 = index1;
                          this.AddPLog("Item: ID" + index7.ToString() + ", Qty=" + num16.ToString() + ", Already transferred: " + num14.ToString() + "/" + num13.ToString() + ", LogPtsNeed: " + num17.ToString());
                          flag6 = true;
                          if (this.game.Data.Turn == 1)
                            id = id;
                          if (this.game.Data.LocObj[index5].X == 14 & this.game.Data.LocObj[index5].Y == 3)
                            index5 = index5;
                          let mut num19: i32 =  num16;
                          bool ok;
                          if (!(flag8 | num12 < 1))
                          {
                            if (flag7)
                            {
                              OrderResult orderResult = this.LIS_RemovePointsFromTrajectory(this.game.Data.LocObj[index5].X, this.game.Data.LocObj[index5].Y, num17 * num12);
                              ok = orderResult.OK;
                              if (orderResult.Data > 0)
                              {
                                let mut randomizedRoundingDam: i32 =  this.game.HandyFunctionsObj.Air_GetRandomizedRoundingDam(num19, orderResult.Data);
                                num19 -= randomizedRoundingDam;
                                if (flag2)
                                {
                                  this.game.Data.LocObj[index5].AddLog(107, index7, 0, randomizedRoundingDam);
                                  this.game.Data.UnitObj[index2].AddLog(108, index7, 0, randomizedRoundingDam);
                                }
                                else if (flag1)
                                {
                                  this.game.Data.LocObj[index5].AddLog(108, index7, 0, randomizedRoundingDam);
                                  this.game.Data.UnitObj[index2].AddLog(109, index7, 0, randomizedRoundingDam);
                                }
                              }
                            }
                            else
                              ok = this.LIS_RemovePointsFromTrajectory(this.game.Data.LocObj[index5].X, this.game.Data.LocObj[index5].Y, num17 * num12).OK;
                          }
                          if (flag1)
                          {
                            this.game.Data.UnitObj[index2].items.list.AddWeight(index7, num19);
                            this.game.Data.LocObj[index5].items.list.RemoveWeight(index7, num16);
                            if (!freeOfCost)
                              this.game.Data.UnitObj[index2].AddLog(101, index7, 0, num19);
                            if (!freeOfCost)
                              this.game.Data.LocObj[index5].AddLog(102, index7, 0, num16);
                            if (!freeOfCost)
                              this.game.Data.LocObj[index5].AddLog(505, 1, 0, num17 * num12);
                          }
                          else if (flag2)
                          {
                            this.game.Data.UnitObj[index2].items.list.RemoveWeight(index7, num16);
                            this.game.Data.LocObj[index5].items.list.AddWeight(index7, num19);
                            if (!freeOfCost)
                              this.game.Data.UnitObj[index2].AddLog(102, index7, 0, num16);
                            if (!freeOfCost)
                              this.game.Data.LocObj[index5].AddLog(101, index7, 0, num16);
                            if (!freeOfCost)
                              this.game.Data.LocObj[index5].AddLog(502, 1, 0, num17 * num12);
                          }
                          this.game.Data.LocObj[index5].tempHandledItems.AddWeight(index7, num16);
                          if (!freeOfCost)
                          {
                            num9 -= num17 * num12;
                            bool flag9 = false;
                            this.AddPLog("--Removed " + (num17 * num12).ToString() + " from trajectory " + this.game.Data.LocObj[index5].Name + " > " + this.game.Data.UnitObj[index2].Name);
                            if (flag9 & num17 > 0)
                            {
                              this.AddPLog("--0 reached--");
                              if (flag7)
                                this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map, useAirBridge: true, maxDam: weight);
                              else
                                this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.Data.UnitObj[index2].X, this.game.Data.UnitObj[index2].Y, this.game.Data.UnitObj[index2].Map);
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
          if (flag6)
            flag5 = true;
        }
        if (num3 < 100)
          flag4 = true;
      }
      let mut counter: i32 =  simpleList.Counter;
      for (let mut index8: i32 =  0; index8 <= counter; index8 += 1)
      {
        let mut index9: i32 =  simpleList.Id[index8];
        let mut data3: i32 =  this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[index9].X, this.game.Data.UnitObj[index9].Y].LIShistory[6] - this.game.Data.UnitObj[index9].tempLisStartHistory;
        this.AddPLog("SHQ " + this.game.Data.UnitObj[index9].Name + " total LIS use " + data3.ToString() + " pts.");
        if (flag2)
          this.game.Data.UnitObj[index9].AddLog(502, 1, 0, data3);
        if (flag1)
          this.game.Data.UnitObj[index9].AddLog(505, 1, 0, data3);
      }
      if (flag2)
        this.WritePLog("LIS_LocationSupply_Log");
      if (flag1 & !freeOfCost)
        this.WritePLog("LIS_LocationReturns_Log");
      if (flag1 & freeOfCost)
        this.WritePLog("LIS_LocationReturns_Free_Log");
      if (!freeOfCost)
        this.LIS_SetNetwork(true);
      if (freeOfCost || !this.game.EventRelatedObj.Helper_AirEnabled())
        return;
      if (flag2)
        this.game.HandyFunctionsObj.Air_LogUpdateForAll(this.game.Data.Turn, "", " Air Points left after Zone Supply");
      if (!flag1)
        return;
      this.game.HandyFunctionsObj.Air_LogUpdateForAll(this.game.Data.Turn, "", " Air Points left after Zone Pickups");
    }

    pub void LIS_AddOrganicPointsToTrajectory(
      ox: i32,
      oy: i32,
      pts: i32,
      percentageWholeTrajectory: i32,
      unr: i32)
    {
      bool flag = true;
      let mut slot1: i32 =  this.game.Data.UnitObj[unr].tempComplexCoords.FindSlot(ox, oy, 0);
      let mut slot2: i32 =  this.game.Data.UnitObj[unr].tempCoords.FindSlot(ox, oy, 0);
      if (slot1 == -1 || slot2 == -1)
        return;
      CoordList coordList = CoordList::new();
      for (let mut counter: i32 =  this.game.Data.UnitObj[unr].tempComplexCoords.coordList[slot1].counter; counter >= 0; counter += -1)
        coordList.AddCoord(this.game.Data.UnitObj[unr].tempComplexCoords.coordList[slot1].coord[counter].x, this.game.Data.UnitObj[unr].tempComplexCoords.coordList[slot1].coord[counter].y, this.game.Data.UnitObj[unr].tempComplexCoords.coordList[slot1].coord[counter].map, this.game.Data.UnitObj[unr].tempComplexCoords.coordList[slot1].coord[counter].data1, this.game.Data.UnitObj[unr].tempComplexCoords.coordList[slot1].coord[counter].data2);
      coordList.AddCoord(ox, oy, 0, 0, this.game.Data.UnitObj[unr].tempCoords.coord[slot2].data2);
      let mut num1: i32 =   Math.Round(Math.Ceiling( coordList.counter / 2.0));
      for (let mut index1: i32 =  0; index1 <= num1; index1 += 1)
      {
        let mut index2: i32 =  coordList.counter - index1;
        let mut data2: i32 =  coordList.coord[index1].data2;
        coordList.coord[index1].data2 = coordList.coord[index2].data2;
        coordList.coord[index2].data2 = data2;
      }
      let mut num2: i32 =  percentageWholeTrajectory;
      Coordinate coordinate1;
      coordinate1.x = coordList.coord[0].x;
      coordinate1.y = coordList.coord[0].y;
      coordinate1.map = 0;
      coordinate1.onmap = true;
      if (this.game.Data.Turn == 2)
        ;
      let mut counter1: i32 =  coordList.counter;
      for (let mut index3: i32 =  1; index3 <= counter1; index3 += 1)
      {
        Coordinate coordinate2 = coordList.coord[index3];
        let mut data2: i32 =  coordinate2.data2;
        if (!coordinate2.onmap)
          break;
        let mut index4: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
        let mut index5: i32 =  index4 + 3;
        if (index5 > 5)
          index5 -= 6;
        if (flag)
        {
          let mut num3: i32 =   Math.Round( (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganic[6] * this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganicPercentage[6] + pts * num2) /  (pts + this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganic[6]));
          this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganicPercentage[6] = num3;
          int[] liSorganic = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganic;
          int[] numArray = liSorganic;
          let mut index6: i32 =  6;
          let mut index7: i32 =  index6;
          let mut num4: i32 =  liSorganic[index6] + pts;
          numArray[index7] = num4;
        }
        let mut num5: i32 =   Math.Round( (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganic[index4] * this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganicPercentage[index4] + pts * num2) /  (pts + this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganic[index4]));
        this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganicPercentage[index4] = num5;
        int[] liSorganic1 = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISorganic;
        int[] numArray1 = liSorganic1;
        let mut index8: i32 =  index4;
        let mut index9: i32 =  index8;
        let mut num6: i32 =  liSorganic1[index8] + pts;
        numArray1[index9] = num6;
        let mut num7: i32 =   Math.Round( (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganic[index5] * this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganicPercentage[index5] + pts * num2) /  (pts + this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganic[index5]));
        this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganicPercentage[index5] = num7;
        int[] liSorganic2 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganic;
        int[] numArray2 = liSorganic2;
        let mut index10: i32 =  index5;
        let mut index11: i32 =  index10;
        let mut num8: i32 =  liSorganic2[index10] + pts;
        numArray2[index11] = num8;
        let mut num9: i32 =   Math.Round( (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganic[6] * this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganicPercentage[6] + pts * data2) /  (pts + this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganic[6]));
        this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganicPercentage[6] = num9;
        int[] liSorganic3 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISorganic;
        int[] numArray3 = liSorganic3;
        let mut index12: i32 =  6;
        let mut index13: i32 =  index12;
        let mut num10: i32 =  liSorganic3[index12] + pts;
        numArray3[index13] = num10;
        flag = false;
        coordinate1 = coordinate2;
        num2 = data2;
      }
    }

    pub OrderResult LIS_RemovePointsFromTrajectory(
      ox: i32,
      oy: i32,
      pts: i32,
      let mut unrForDam: i32 =  -1)
    {
      bool flag1 = false;
      bool flag2 = true;
      let mut num1: i32 =  0;
      OrderResult orderResult = OrderResult::new();
      orderResult.OK = false;
      orderResult.Data = 0;
      if (pts <= 0)
        return orderResult;
      Coordinate coordinate1;
      coordinate1.x = ox;
      coordinate1.y = oy;
      Coordinate coordinate2;
      for (coordinate1.onmap = true; coordinate1.onmap; coordinate1 = coordinate2)
      {
        coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
        if (coordinate2.onmap)
        {
          let mut index1: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
          bool flag3 = false;
          index2: i32;
          if (index1 >= 0 & index1 <= 5)
          {
            index2 = index1 + 3;
            if (index2 > 5)
              index2 -= 6;
          }
          else
          {
            flag3 = true;
            if (unrForDam > -1)
            {
              let mut damForAirBridge: i32 =  this.game.HandyFunctionsObj.Air_GetDamForAirBridge(coordinate2.x, coordinate2.y, coordinate1.x, coordinate1.y);
              this.game.HandyFunctionsObj.Air_applyDamToUnit(unrForDam, damForAirBridge);
              if (num1 > 0)
                num1 +=  Math.Round( ((100 - num1) * damForAirBridge) / 100.0);
              else
                num1 += damForAirBridge;
            }
            if (this.game.HandyFunctionsObj.Air_removeLisFromAirBridge(coordinate2.x, coordinate2.y, coordinate1.x, coordinate1.y, pts))
              flag1 = true;
          }
          if (flag2)
          {
            int[] liSpoints = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints;
            int[] numArray = liSpoints;
            let mut index3: i32 =  6;
            let mut index4: i32 =  index3;
            let mut num2: i32 =  liSpoints[index3] - pts;
            numArray[index4] = num2;
          }
          if (!flag3)
          {
            int[] liSpoints = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints;
            int[] numArray = liSpoints;
            let mut index5: i32 =  index1;
            let mut index6: i32 =  index5;
            let mut num3: i32 =  liSpoints[index5] - pts;
            numArray[index6] = num3;
          }
          if (!flag3)
          {
            int[] liSpoints = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints;
            int[] numArray = liSpoints;
            let mut index7: i32 =  index2;
            let mut index8: i32 =  index7;
            let mut num4: i32 =  liSpoints[index7] - pts;
            numArray[index8] = num4;
          }
          int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints;
          int[] numArray1 = liSpoints1;
          let mut index9: i32 =  6;
          let mut index10: i32 =  index9;
          let mut num5: i32 =  liSpoints1[index9] - pts;
          numArray1[index10] = num5;
          if (flag2)
          {
            int[] liShistory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory;
            int[] numArray2 = liShistory;
            let mut index11: i32 =  6;
            let mut index12: i32 =  index11;
            let mut num6: i32 =  liShistory[index11] + pts;
            numArray2[index12] = num6;
          }
          if (!flag3)
          {
            int[] liShistory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory;
            int[] numArray3 = liShistory;
            let mut index13: i32 =  index1;
            let mut index14: i32 =  index13;
            let mut num7: i32 =  liShistory[index13] + pts;
            numArray3[index14] = num7;
          }
          if (!flag3)
          {
            int[] liShistory = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory;
            int[] numArray4 = liShistory;
            let mut index15: i32 =  index2;
            let mut index16: i32 =  index15;
            let mut num8: i32 =  liShistory[index15] + pts;
            numArray4[index16] = num8;
          }
          int[] liShistory1 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory;
          int[] numArray5 = liShistory1;
          let mut index17: i32 =  6;
          let mut index18: i32 =  index17;
          let mut num9: i32 =  liShistory1[index17] + pts;
          numArray5[index18] = num9;
          if (flag2 & this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6] <= 0)
            flag1 = true;
          if (!flag3 && this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index1] <= 0)
            flag1 = true;
          if (!flag3 && this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index2] <= 0)
            flag1 = true;
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[6] <= 0)
            flag1 = true;
          flag2 = false;
        }
        else
          break;
      }
      return OrderResult::new() { OK = flag1, Data = num1 };
    }

    pub Coordinate LIS_GetUnitBestCoordinate(
      CoordList CL,
      lisPtsNeeded: i32,
      bool lowestOnTrajectCheck = false,
      let mut shqX: i32 =  -1,
      let mut shqY: i32 =  -1)
    {
      let mut num1: i32 =  9999999;
      let mut num2: i32 =  0;
      let mut num3: i32 =  0;
      Coordinate unitBestCoordinate;
      unitBestCoordinate.x = -1;
      unitBestCoordinate.y = -1;
      unitBestCoordinate.onmap = false;
      if (lisPtsNeeded < 1)
        lisPtsNeeded = 1;
      if (this.game.EventRelatedObj.Helper_AirEnabled())
        ;
      if (Information.IsNothing( CL))
        return unitBestCoordinate;
      if (lowestOnTrajectCheck && this.game.Data.RegimeObj[this.game.Data.Turn].AI)
        lowestOnTrajectCheck = false;
      if (!lowestOnTrajectCheck)
      {
        let mut counter1: i32 =  CL.counter;
        for (let mut index: i32 =  0; index <= counter1; index += 1)
        {
          if (this.game.Data.MapObj[0].HexObj[CL.coord[index].x, CL.coord[index].y].LISpoints[6] >= lisPtsNeeded && CL.coord[index].data2 > num2 | CL.coord[index].data2 == num2 & this.game.Data.MapObj[0].HexObj[CL.coord[index].x, CL.coord[index].y].LISpoints[6] > num3)
          {
            num1 = CL.coord[index].data1;
            num2 = CL.coord[index].data2;
            num3 = this.game.Data.MapObj[0].HexObj[CL.coord[index].x, CL.coord[index].y].LISpoints[6];
            unitBestCoordinate = CL.coord[index] with
            {
              onmap = true
            };
          }
        }
        if (!unitBestCoordinate.onmap & lisPtsNeeded > 1)
        {
          let mut counter2: i32 =  CL.counter;
          for (let mut index: i32 =  0; index <= counter2; index += 1)
          {
            if (this.game.Data.MapObj[0].HexObj[CL.coord[index].x, CL.coord[index].y].LISpoints[6] >= 1 && CL.coord[index].data1 < num1)
            {
              num1 = CL.coord[index].data1;
              unitBestCoordinate = CL.coord[index] with
              {
                onmap = true
              };
            }
          }
        }
      }
      else
      {
        let mut counter3: i32 =  CL.counter;
        data1: i32;
        for (let mut index: i32 =  0; index <= counter3; index += 1)
        {
          let mut pointsOnTrajectory: i32 =  this.LIS_GetLowestPointsOnTrajectory(CL.coord[index].x, CL.coord[index].y, shqX, shqY, true);
          num4: i32;
          if (pointsOnTrajectory >= lisPtsNeeded && CL.coord[index].data2 > num2 | CL.coord[index].data2 == num2 & pointsOnTrajectory > num3 | pointsOnTrajectory > 0 & CL.coord[index].data2 > 0 & num4 == 1)
          {
            data1 = CL.coord[index].data1;
            num2 = CL.coord[index].data2;
            num3 = pointsOnTrajectory;
            num4 = 0;
            if (this.game.EditObj.TempCameFrom[0].Value[CL.coord[index].x, CL.coord[index].y].data2 > 0 && this.game.EditObj.TempCameFrom[0].Value[CL.coord[index].x, CL.coord[index].y].data2 < 999)
              num4 = 1;
            unitBestCoordinate = CL.coord[index] with
            {
              onmap = true
            };
          }
        }
        if (!unitBestCoordinate.onmap & lisPtsNeeded > 1)
        {
          let mut counter4: i32 =  CL.counter;
          for (let mut index: i32 =  0; index <= counter4; index += 1)
          {
            let mut pointsOnTrajectory: i32 =  this.LIS_GetLowestPointsOnTrajectory(CL.coord[index].x, CL.coord[index].y, shqX, shqY, true);
            if (pointsOnTrajectory >= 1 && CL.coord[index].data2 > num2 | CL.coord[index].data2 == num2 & pointsOnTrajectory > num3)
            {
              data1 = CL.coord[index].data1;
              num2 = CL.coord[index].data2;
              num3 = pointsOnTrajectory;
              unitBestCoordinate = CL.coord[index] with
              {
                onmap = true
              };
            }
          }
        }
      }
      return unitBestCoordinate;
    }

    pub LIS_GetLowestPointsOnTrajectory: i32(
      ox: i32,
      oy: i32,
      tarX: i32,
      tarY: i32,
      bool verifyTargetReached = false)
    {
      bool flag1 = true;
      let mut pointsOnTrajectory: i32 =  9999999;
      bool flag2 = false;
      Coordinate coordinate1;
      coordinate1.x = ox;
      coordinate1.y = oy;
      coordinate1.onmap = true;
      while (coordinate1.onmap)
      {
        Coordinate coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
        if (coordinate2.onmap)
        {
          let mut index1: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
          bool flag3 = false;
          index2: i32;
          if (index1 >= 0 & index1 <= 5)
          {
            index2 = index1 + 3;
            if (index2 > 5)
              index2 -= 6;
          }
          else
          {
            flag3 = true;
            let mut integer: i32 =  Conversions.ToInteger(this.game.HandyFunctionsObj.Air_getLisFromAirBridge(coordinate2.x, coordinate2.y, coordinate1.x, coordinate1.y));
            if (integer < pointsOnTrajectory)
              pointsOnTrajectory = integer;
          }
          if (flag1 & this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6] < pointsOnTrajectory)
            pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6];
          if (!flag3)
          {
            if (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index1] < pointsOnTrajectory)
              pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index1];
            if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index2] < pointsOnTrajectory)
              pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index2];
          }
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[6] < pointsOnTrajectory)
            pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[6];
          flag1 = false;
          coordinate1 = coordinate2;
          if (coordinate1.x == tarX & coordinate1.y == tarY)
            flag2 = true;
        }
        else
        {
          if (coordinate1.x == tarX & coordinate1.y == tarY)
            flag2 = true;
          if (ox == coordinate1.x & oy == coordinate1.y & flag1)
          {
            pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6];
            break;
          }
          break;
        }
      }
      if (pointsOnTrajectory >= 999999)
        pointsOnTrajectory = 0;
      if (verifyTargetReached & !flag2)
        pointsOnTrajectory = -1;
      return pointsOnTrajectory;
    }

    pub LIS_HasAirBridgeOnTrajectory: bool(
      ox: i32,
      oy: i32,
      tarX: i32,
      tarY: i32,
      bool verifyTargetReached = false)
    {
      Coordinate coordinate1;
      coordinate1.x = ox;
      coordinate1.y = oy;
      Coordinate coordinate2;
      for (coordinate1.onmap = true; coordinate1.onmap; coordinate1 = coordinate2)
      {
        coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
        if (coordinate2.onmap)
        {
          let mut num: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
          if (!(num >= 0 & num <= 5))
            return true;
        }
        else
          break;
      }
      return false;
    }

    pub IsItemMaximizer: bool(itemNr: i32) => itemNr >= 16 & itemNr <= 20 || itemNr == 22;

    pub LIS_GetLowestPointsOnTrajectory_PREVIEW: i32(
      ox: i32,
      oy: i32,
      tarX: i32,
      tarY: i32,
      bool verifyTargetReached = false)
    {
      bool flag1 = true;
      let mut trajectoryPreview: i32 =  9999999;
      bool flag2 = false;
      Coordinate coordinate1;
      coordinate1.x = ox;
      coordinate1.y = oy;
      coordinate1.onmap = true;
      while (coordinate1.onmap)
      {
        Coordinate coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
        if (coordinate2.onmap)
        {
          let mut index1: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
          let mut index2: i32 =  index1 + 3;
          if (index2 > 5)
            index2 -= 6;
          if (flag1 & this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[6] < trajectoryPreview)
            trajectoryPreview = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[6];
          if (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[index1] < trajectoryPreview)
            trajectoryPreview = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[index1];
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].tempPreviewLIS[index2] < trajectoryPreview)
            trajectoryPreview = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].tempPreviewLIS[index2];
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].tempPreviewLIS[6] < trajectoryPreview)
            trajectoryPreview = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].tempPreviewLIS[6];
          flag1 = false;
          coordinate1 = coordinate2;
          if (coordinate1.x == tarX & coordinate1.y == tarY)
            flag2 = true;
        }
        else
        {
          if (coordinate1.x == tarX & coordinate1.y == tarY)
            flag2 = true;
          if (ox == coordinate1.x & oy == coordinate1.y & flag1)
          {
            trajectoryPreview = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].tempPreviewLIS[6];
            break;
          }
          break;
        }
      }
      if (trajectoryPreview >= 999999)
        trajectoryPreview = 0;
      if (verifyTargetReached & !flag2)
        trajectoryPreview = -1;
      return trajectoryPreview;
    }

    pub fn LIS_GetLowestTotalHistoryTrajectory(ox: i32, oy: i32) -> i32
    {
      bool flag = true;
      let mut historyTrajectory: i32 =  9999999;
      Coordinate coordinate1;
      coordinate1.x = ox;
      coordinate1.y = oy;
      Coordinate coordinate2;
      for (coordinate1.onmap = true; coordinate1.onmap; coordinate1 = coordinate2)
      {
        coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
        if (coordinate2.onmap)
        {
          let mut index1: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
          let mut index2: i32 =  index1 + 3;
          if (index2 > 5)
            index2 -= 6;
          if (flag & this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[6] + this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6] < historyTrajectory)
            historyTrajectory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[6] + this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6];
          if (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[index1] + this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index1] < historyTrajectory)
            historyTrajectory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[index1] + this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index1];
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[index2] + this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index2] < historyTrajectory)
            historyTrajectory = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[index2] + this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[index2];
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[6] + this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[6] < historyTrajectory)
            historyTrajectory = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[6] + this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LISpoints[6];
          flag = false;
        }
        else
          break;
      }
      if (historyTrajectory >= 999999)
        historyTrajectory = 0;
      return historyTrajectory;
    }

    pub fn LIS_GetLowestPotentialPointsOnTrajectory(ox: i32, oy: i32) -> i32
    {
      bool flag = true;
      let mut pointsOnTrajectory: i32 =  9999999;
      Coordinate coordinate1;
      coordinate1.x = ox;
      coordinate1.y = oy;
      Coordinate coordinate2;
      for (coordinate1.onmap = true; coordinate1.onmap; coordinate1 = coordinate2)
      {
        coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
        if (coordinate2.onmap)
        {
          let mut index1: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate1.x, coordinate1.y, 0, coordinate2.x, coordinate2.y, 0) - 1;
          let mut index2: i32 =  index1 + 3;
          if (index2 > 5)
            index2 -= 6;
          if (flag & this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIStotalHistory[6] - this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[6] < pointsOnTrajectory)
            pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIStotalHistory[6] - this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[6];
          if (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIStotalHistory[index1] - this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[index1] < pointsOnTrajectory)
            pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIStotalHistory[index1] - this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LIShistory[index1];
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIStotalHistory[index2] - this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[index2] < pointsOnTrajectory)
            pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIStotalHistory[index2] - this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[index2];
          if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIStotalHistory[6] - this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[6] < pointsOnTrajectory)
            pointsOnTrajectory = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIStotalHistory[6] - this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].LIShistory[6];
          flag = false;
        }
        else
          break;
      }
      return pointsOnTrajectory;
    }

    pub fn LIS_SetNetwork(bool isCalibrationCall, bool isPreview = false, let mut onlyForAssetID: i32 =  -1)
    {
      let mut index1: i32 =  -1;
      libName1: String = "SE_Data";
      let mut stringListById1: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
      let mut stringListById2: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 149, 0, 0));
      let mut stringListById3: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 148, 0, 0));
      let mut stringListById4: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 144, 0, 0));
      object[,] objArray1 = new object[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
      object[,] objArray2 = new object[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
      let mut mapWidth1: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut index2: i32 =  0; index2 <= mapWidth1; index2 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
        {
          objArray1[index2, index3] =  this.game.EditObj.TempValue[0].Value[index2, index3];
          objArray2[index2, index3] =  this.game.EditObj.TempCameFrom[0].Value[index2, index3];
          if (this.game.Data.MapObj[0].HexObj[index2, index3].Regime != this.game.Data.Turn)
            this.game.Data.RegimeObj[this.game.Data.Turn].Trafic[0].Value[index2, index3] = 0;
        }
      }
      if (isPreview & !this.game.Data.RegimeObj[this.game.Data.Turn].AI & this.game.Data.InTurn)
        this.game.EventRelatedObj.ExecSuperImposeMessage("Calculating", "Hold on while we calculate the logistics Preview...", 0, 0, "");
      if (!isCalibrationCall & onlyForAssetID < 1)
      {
        index1 = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 511, 0, 0));
        if (index1 > -1)
          this.game.Data.StringListObj[index1].ClearAllRows();
      }
      if (!isCalibrationCall)
      {
        index1 = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 512, 0, 0));
        if (index1 > -1)
          this.game.Data.StringListObj[index1].ClearAllRows();
      }
      if (!isCalibrationCall & !isPreview)
      {
        index1 = this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 505, 0, 0));
        if (index1 > -1)
          this.game.Data.StringListObj[index1].ClearAllRows();
      }
      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
        index1 = -1;
      if (this.game.Data.RegimeObj[this.game.Data.Turn].Sleep)
        index1 = -1;
      if (isCalibrationCall)
        index1 = -1;
      let mut pts1: i32 =  0;
      let mut pts2: i32 =  0;
      let mut pts3: i32 =  0;
      let mut pts4: i32 =  0;
      let mut num1: i32 =  -1;
      let mut num2: i32 =  -1;
      let mut num3: i32 =  -1;
      let mut num4: i32 =  -1;
      let mut num5: i32 =  0;
      this.game.EditObj.layerLisOnlyAssetId_isSupplyBase = false;
      tid1: i32;
      if (isPreview)
      {
        index1 = onlyForAssetID <= -1 ? this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 511, 0, 0)) : this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 512, 0, 0));
        if (onlyForAssetID > -1)
        {
          let mut stringListById5: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 361, 0, 0));
          num1 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(9, onlyForAssetID, 3)));
          num2 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData(9, onlyForAssetID, 4)));
          let mut length: i32 =  this.game.Data.StringListObj[stringListById5].Length;
          for (tid1 = 0; tid1 <= length; tid1 += 1)
          {
            if ( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[tid1, 0])) == onlyForAssetID)
            {
              if ( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[tid1, 1])) == 6 &&  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[tid1, 2])) == 14)
              {
                str: String = this.game.Data.StringListObj[stringListById5].Data[tid1, 3];
                let mut num6: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[tid1, 4]));
                if (Operators.CompareString(str.ToLower(), "truckpoints", false) == 0)
                  pts4 += num6;
                if (Operators.CompareString(str.ToLower(), "maglevpoints", false) == 0)
                  pts2 += num6;
                if (Operators.CompareString(str.ToLower(), "truckfreeap", false) == 0)
                  pts3 += num6;
                if (Operators.CompareString(str.ToLower(), "maglevfreeap", false) == 0)
                  pts1 += num6;
              }
              if ( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[tid1, 1])) == 5 &&  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[tid1, 2])) == 14)
              {
                str: String = this.game.Data.StringListObj[stringListById5].Data[tid1, 3];
                let mut num7: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById5].Data[tid1, 4]));
                if (Operators.CompareString(str.ToLower(), "logistical extension", false) == 0)
                  num5 += num7;
              }
            }
          }
          if (num5 > 0)
          {
            this.game.EditObj.layerLisOnlyAssetId_isSupplyBase = true;
            num3 = num1;
            num4 = num2;
            num1 = -1;
            num2 = -1;
          }
        }
      }
      if (index1 > -1 && this.game.Data.StringListObj[index1].Width < 7)
        index1 = -1;
      int[,,] numArray1 = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 5];
      index4: i32;
      if (!isCalibrationCall)
      {
        let mut mapWidth2: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index5: i32 =  0; index5 <= mapWidth2; index5 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
          {
            if (this.game.Data.MapObj[0].HexObj[index5, index6].Regime == this.game.Data.Turn)
            {
              let mut location: i32 =  this.game.Data.MapObj[0].HexObj[index5, index6].Location;
              if (location > -1)
              {
                if (isPreview)
                {
                  data1: DataClass = this.game.Data;
                  str1: String = "truckPoints";
                   local1: String =  str1;
                  libName2: String = libName1;
                  let mut libVar1: i32 =  data1.FindLibVar( local1, libName2);
                  index4 = this.game.Data.MapObj[0].HexObj[index5, index6].GetHexLibVarValue(libVar1);
                  data2: DataClass = this.game.Data;
                  str2: String = "truckFreeAp";
                   local2: String =  str2;
                  libName3: String = libName1;
                  let mut libVar2: i32 =  data2.FindLibVar( local2, libName3);
                  let mut hexLibVarValue1: i32 =  this.game.Data.MapObj[0].HexObj[index5, index6].GetHexLibVarValue(libVar2);
                  data3: DataClass = this.game.Data;
                  str3: String = "maglevPoints";
                   local3: String =  str3;
                  libName4: String = libName1;
                  let mut libVar3: i32 =  data3.FindLibVar( local3, libName4);
                  let mut hexLibVarValue2: i32 =  this.game.Data.MapObj[0].HexObj[index5, index6].GetHexLibVarValue(libVar3);
                  data4: DataClass = this.game.Data;
                  str4: String = "maglevFreeAp";
                   local4: String =  str4;
                  libName5: String = libName1;
                  let mut libVar4: i32 =  data4.FindLibVar( local4, libName5);
                  let mut hexLibVarValue3: i32 =  this.game.Data.MapObj[0].HexObj[index5, index6].GetHexLibVarValue(libVar4);
                  if (index4 > 0 & hexLibVarValue1 > 0)
                    numArray1[index5, index6, 1] = index4;
                  if (hexLibVarValue2 > 0 & hexLibVarValue3 > 0)
                    numArray1[index5, index6, 2] = hexLibVarValue2;
                }
                else if (!Information.IsNothing( this.game.Data.LocObj[location].tempLIS) && !Information.IsNothing( this.game.Data.LocObj[location].tempLISfreeAP) && this.game.Data.LocObj[location].tempLIS.Counter > -1)
                {
                  tid1 = 1;
                  do
                  {
                    index4 = this.game.Data.LocObj[location].tempLIS.FindWeight(tid1);
                    let mut weight: i32 =  this.game.Data.LocObj[location].tempLISfreeAP.FindWeight(tid1);
                    if (index4 > 0 & weight > 0)
                      numArray1[index5, index6, tid1] = index4;
                    tid1 += 1;
                  }
                  while (tid1 <= 2);
                }
              }
            }
          }
        }
      }
      if (isPreview & !isCalibrationCall)
      {
        this.tempLISwithoutLogExt = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 9];
        this.cacheLIShistory = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 9];
        this.cacheLISpoints = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 9];
        this.cacheLIStotalHistory = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 9];
        this.cacheLISorganic = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 9];
        this.cacheLISorganicPercentage = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 9];
        this.cacheLISpull = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 9];
        let mut mapWidth3: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index7: i32 =  0; index7 <= mapWidth3; index7 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index8: i32 =  0; index8 <= mapHeight; index8 += 1)
          {
            let mut index9: i32 =  0;
            do
            {
              this.tempLISwithoutLogExt[index7, index8, index9] = 0;
              this.cacheLIShistory[index7, index8, index9] = this.game.Data.MapObj[0].HexObj[index7, index8].LIShistory[index9];
              this.cacheLISpoints[index7, index8, index9] = this.game.Data.MapObj[0].HexObj[index7, index8].LISpoints[index9];
              this.cacheLIStotalHistory[index7, index8, index9] = this.game.Data.MapObj[0].HexObj[index7, index8].LIStotalHistory[index9];
              this.cacheLISorganic[index7, index8, index9] = this.game.Data.MapObj[0].HexObj[index7, index8].LISorganic[index9];
              this.cacheLISorganicPercentage[index7, index8, index9] = this.game.Data.MapObj[0].HexObj[index7, index8].LISorganicPercentage[index9];
              this.cacheLISpull[index7, index8, index9] = this.game.Data.MapObj[0].HexObj[index7, index8].LISpull[index9];
              index9 += 1;
            }
            while (index9 <= 8);
          }
        }
      }
      int[,,,] numArray2 = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 6, 4];
      int[,,] numArray3 = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 6];
      if (this.game.Data.Turn == 2)
        index4 = index4;
      if (Information.IsNothing( this.game.Data.RegimeObj[this.game.Data.Turn].Trafic[0]))
        this.game.Data.RegimeObj[this.game.Data.Turn].Trafic[0] = new MapMatrix2(this.game.Data.MapObj[0].MapWidth, this.game.Data.MapObj[0].MapHeight);
      if (Information.IsNothing( this.game.Data.RegimeObj[this.game.Data.Turn].Trafic2[0]))
        this.game.Data.RegimeObj[this.game.Data.Turn].Trafic2[0] = new MapMatrix2(this.game.Data.MapObj[0].MapWidth, this.game.Data.MapObj[0].MapHeight);
      if (this.game.Data.RegimeObj[this.game.Data.Turn].Trafic[0].Value.GetUpperBound(0) >= this.game.Data.MapObj[0].MapWidth)
      {
        let mut mapWidth4: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut x: i32 =  0; x <= mapWidth4; x += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut y: i32 =  0; y <= mapHeight; y += 1)
          {
            if (!isCalibrationCall)
            {
              let mut index10: i32 =  0;
              do
              {
                this.game.Data.MapObj[0].HexObj[x, y].LISpull[index10] = 0;
                index10 += 1;
              }
              while (index10 <= 8);
            }
            NeighboursExtra lisTraffic = this.game.HandyFunctionsObj.GetLisTraffic(x, y);
            tid1 = 0;
            do
            {
              if (lisTraffic.truck)
                numArray2[x, y, tid1, 0] = lisTraffic.data[tid1];
              if (lisTraffic.rail)
                numArray2[x, y, tid1, 1] = lisTraffic.data[tid1];
              if (lisTraffic.pull)
                numArray2[x, y, tid1, 2] = lisTraffic.data[tid1];
              tid1 += 1;
            }
            while (tid1 <= 5);
          }
        }
      }
      let mut stringListById6: i32 =  this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[405]));
      this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[404]));
      if (isCalibrationCall)
        return;
      if (isCalibrationCall)
      {
        let mut mapWidth5: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index11: i32 =  0; index11 <= mapWidth5; index11 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index12: i32 =  0; index12 <= mapHeight; index12 += 1)
          {
            let mut index13: i32 =  0;
            do
            {
              this.game.Data.MapObj[0].HexObj[index11, index12].tempOldLISpoints[index13] = this.game.Data.MapObj[0].HexObj[index11, index12].LISpoints[index13];
              this.game.Data.MapObj[0].HexObj[index11, index12].LISpoints[index13] = 0;
              index13 += 1;
            }
            while (index13 <= 8);
          }
        }
      }
      else
      {
        let mut mapWidth6: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index14: i32 =  0; index14 <= mapWidth6; index14 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index15: i32 =  0; index15 <= mapHeight; index15 += 1)
          {
            let mut index16: i32 =  0;
            do
            {
              this.game.Data.MapObj[0].HexObj[index14, index15].tempOldLISpoints[index16] = 0;
              this.game.Data.MapObj[0].HexObj[index14, index15].LIShistory[index16] = 0;
              this.game.Data.MapObj[0].HexObj[index14, index15].LISpoints[index16] = 0;
              this.game.Data.MapObj[0].HexObj[index14, index15].LIStotalHistory[index16] = 0;
              this.game.Data.MapObj[0].HexObj[index14, index15].LISorganic[index16] = 0;
              this.game.Data.MapObj[0].HexObj[index14, index15].LISorganicPercentage[index16] = 0;
              this.game.Data.MapObj[0].HexObj[index14, index15].LISpull[index16] = 0;
              index16 += 1;
            }
            while (index16 <= 8);
          }
        }
      }
      let mut locCounter1: i32 =  this.game.Data.LocCounter;
      for (let mut index17: i32 =  0; index17 <= locCounter1; index17 += 1)
      {
        if (Information.IsNothing( this.game.Data.LocObj[index17].tempLIS))
          this.game.Data.LocObj[index17].tempLIS = SimpleList::new();
        if (Information.IsNothing( this.game.Data.LocObj[index17].tempLISfreeAP))
          this.game.Data.LocObj[index17].tempLISfreeAP = SimpleList::new();
      }
      bool flag1 = false;
      if (this.game.Data.StringListObj[this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 144, 0, 0))].FindRow(0, 3291) >= 0)
        flag1 = true;
      this.game.EditObj.tempZoneTest = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
      index18: i32;
      y1: i32;
      index19: i32;
      movetype1: i32;
      Coordinate coordinate1;
      if (!isCalibrationCall & flag1)
      {
        bool[,] flagArray1 = new bool[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
        bool[,] flagArray2 = new bool[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
        let mut locCounter2: i32 =  this.game.Data.LocCounter;
        for (let mut index20: i32 =  0; index20 <= locCounter2; index20 += 1)
        {
          index18 = this.game.Data.LocObj[index20].X;
          y1 = this.game.Data.LocObj[index20].Y;
          bool flag2 = false;
          if (this.game.Data.MapObj[0].HexObj[index18, y1].Regime == this.game.Data.Turn && !Information.IsNothing( this.game.Data.LocObj[index20].tempLIS))
          {
            if (this.game.Data.LocObj[index20].tempLIS.Counter > -1)
              flag2 = true;
            if (isPreview & !flag2)
            {
              data: DataClass = this.game.Data;
              str: String = "maglevPoints";
               local: String =  str;
              libName6: String = libName1;
              index4 = data.FindLibVar( local, libName6);
              if (this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(index4) > 0)
                flag2 = true;
            }
          }
          if (flag2)
          {
            index19 = 2;
            movetype1 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].GetData(0, index19, 4)));
            let mut num8: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].GetData(0, index19, 2)));
            index4 = this.game.Data.LocObj[index20].tempLIS.FindNr(index19);
            let mut num9: i32 =  0;
            if (index4 > -1)
              num9 = Convert.ToInt32(Math.Floor(new Decimal(this.game.Data.LocObj[index20].tempLIS.Weight[index4])));
            if (num9 < 1)
            {
              data: DataClass = this.game.Data;
              str: String = "maglevPoints";
               local: String =  str;
              libName7: String = libName1;
              index4 = data.FindLibVar( local, libName7);
              let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(index4);
              if (hexLibVarValue > 0)
                num9 = hexLibVarValue;
            }
            if (num9 > 0)
              flagArray2[index18, y1] = true;
          }
        }
        let mut locCounter3: i32 =  this.game.Data.LocCounter;
        for (let mut index21: i32 =  0; index21 <= locCounter3; index21 += 1)
        {
          index18 = this.game.Data.LocObj[index21].X;
          y1 = this.game.Data.LocObj[index21].Y;
          bool flag3 = false;
          if (this.game.Data.MapObj[0].HexObj[index18, y1].Regime == this.game.Data.Turn)
          {
            if (!Information.IsNothing( this.game.Data.LocObj[index21].tempLIS) && this.game.Data.LocObj[index21].tempLIS.Counter > -1)
              flag3 = true;
            if (isPreview)
            {
              data: DataClass = this.game.Data;
              str: String = "maglevPoints";
               local: String =  str;
              libName8: String = libName1;
              let mut libVar: i32 =  data.FindLibVar( local, libName8);
              if (this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(libVar) > 0)
                flag3 = true;
            }
          }
          if (flag3)
          {
            index19 = 2;
            movetype1 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].GetData(0, index19, 4)));
            let mut theater: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].GetData(0, index19, 2)));
            index4 = 0;
            let mut num10: i32 =  0;
            if (!Information.IsNothing( this.game.Data.LocObj[index21].tempLIS))
            {
              index4 = this.game.Data.LocObj[index21].tempLIS.FindNr(index19);
              if (index4 > -1)
                num10 = Convert.ToInt32(Math.Floor(new Decimal(this.game.Data.LocObj[index21].tempLIS.Weight[index4])));
            }
            if (isPreview & num10 < 1)
            {
              data: DataClass = this.game.Data;
              str: String = "maglevPoints";
               local: String =  str;
              libName9: String = libName1;
              let mut libVar: i32 =  data.FindLibVar( local, libName9);
              let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(libVar);
              if (hexLibVarValue > 0)
                num10 = hexLibVarValue;
            }
            if (num10 > 0)
            {
              index4 = -1;
              if (!Information.IsNothing( this.game.Data.LocObj[index21].tempLISfreeAP))
                index4 = this.game.Data.LocObj[index21].tempLISfreeAP.FindNr(index19);
              let mut num11: i32 =  0;
              if (index4 > -1)
                num11 = this.game.Data.LocObj[index21].tempLISfreeAP.Weight[index4];
              if (isPreview)
              {
                data: DataClass = this.game.Data;
                str: String = "maglevFreeAp";
                 local: String =  str;
                libName10: String = libName1;
                let mut libVar: i32 =  data.FindLibVar( local, libName10);
                let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(libVar);
                if (hexLibVarValue > 0)
                  num11 = hexLibVarValue;
              }
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype1, theater, num11 * 2, index18, y1, 0, NoAPPenalties: isPreview, roadsOnly: true, alwaysUseLogisticalBonus: true, lisMode: index19, specialRuleNumber: 2);
              flagArray1[index18, y1] = true;
              if (num11 > 0)
              {
                let mut mapWidth7: i32 =  this.game.Data.MapObj[0].MapWidth;
                for (let mut index22: i32 =  0; index22 <= mapWidth7; index22 += 1)
                {
                  let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                  for (let mut index23: i32 =  0; index23 <= mapHeight; index23 += 1)
                  {
                    if (flagArray2[index22, index23] && !(index22 == index18 & index23 == y1) && this.game.EditObj.TempValue[0].Value[index22, index23] < 9999)
                    {
                      coordinate1.x = index22;
                      coordinate1.y = index23;
                      for (coordinate1.onmap = true; coordinate1.onmap; coordinate1 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y])
                        flagArray1[coordinate1.x, coordinate1.y] = true;
                    }
                  }
                }
              }
            }
          }
        }
        let mut mapWidth8: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index24: i32 =  0; index24 <= mapWidth8; index24 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index25: i32 =  0; index25 <= mapHeight; index25 += 1)
            this.game.EditObj.tempZoneTest[index24, index25] = !flagArray1[index24, index25] ? 0 : 1;
        }
        this.game.HandyFunctionsObj.RedimTempValue(9999);
      }
      else
      {
        let mut mapWidth9: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index26: i32 =  0; index26 <= mapWidth9; index26 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index27: i32 =  0; index27 <= mapHeight; index27 += 1)
            this.game.EditObj.tempZoneTest[index26, index27] = 1;
        }
      }
      if (this.game.Data.Turn == 2)
        ;
      bool flag4 = false;
      let mut mapWidth10: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut index28: i32 =  0; index28 <= mapWidth10; index28 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (let mut index29: i32 =  0; index29 <= mapHeight; index29 += 1)
        {
          if (this.game.Data.RegimeObj[this.game.Data.Turn].Trafic2[0].Value[index28, index29] > 0)
            flag4 = true;
        }
      }
      let mut stringListById7: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID(libName1, 210, 0, 0));
      let mut num12: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById7].GetData2(0, this.game.Data.RegimeObj[this.game.Data.Turn].id, 1, "pullAssetsOff", 2)));
      let mut num13: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById7].GetData2(0, this.game.Data.RegimeObj[this.game.Data.Turn].id, 1, "pullUnitsOff", 2)));
      let mut num14: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById7].GetData2(0, this.game.Data.RegimeObj[this.game.Data.Turn].id, 1, "pullCitiesOff", 2)));
      this.game.EditObj.PossiblePull = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 7];
      this.game.EditObj.origPossiblePull = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 7];
      object[,,] objArray3 = new object[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 7];
      bool flag5;
      if ( Math.Round(Conversion.Val(this.game.Data.Designer)) >= 42 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI & !this.game.Data.RegimeObj[this.game.Data.Turn].minimumDataUsage && !isCalibrationCall & onlyForAssetID < 1 & (flag4 | num12 == 0 | num13 == 0 | num14 == 0))
      {
        let mut locCounter4: i32 =  this.game.Data.LocCounter;
        for (let mut index30: i32 =  0; index30 <= locCounter4; index30 += 1)
        {
          index18 = this.game.Data.LocObj[index30].X;
          y1 = this.game.Data.LocObj[index30].Y;
          if (index18 == 120 & y1 == 65)
            index18 = index18;
          flag5 = false;
          let mut num15: i32 =  0;
          let mut num16: i32 =  0;
          let mut num17: i32 =  0;
          let mut num18: i32 =  0;
          if (this.game.Data.MapObj[0].HexObj[index18, y1].Regime == this.game.Data.Turn)
          {
            if (isPreview)
            {
              data5: DataClass = this.game.Data;
              str5: String = "truckPoints";
               local5: String =  str5;
              libName11: String = libName1;
              let mut libVar5: i32 =  data5.FindLibVar( local5, libName11);
              num15 = this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(libVar5);
              data6: DataClass = this.game.Data;
              str6: String = "truckFreeAp";
               local6: String =  str6;
              libName12: String = libName1;
              let mut libVar6: i32 =  data6.FindLibVar( local6, libName12);
              num16 = this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(libVar6);
              data7: DataClass = this.game.Data;
              str7: String = "maglevPoints";
               local7: String =  str7;
              libName13: String = libName1;
              let mut libVar7: i32 =  data7.FindLibVar( local7, libName13);
              num17 = this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(libVar7);
              data8: DataClass = this.game.Data;
              str8: String = "maglevFreeAp";
               local8: String =  str8;
              libName14: String = libName1;
              let mut libVar8: i32 =  data8.FindLibVar( local8, libName14);
              num18 = this.game.Data.MapObj[0].HexObj[index18, y1].GetHexLibVarValue(libVar8);
            }
            else
            {
              if (!Information.IsNothing( this.game.Data.LocObj[index30].tempLIS))
              {
                num15 = this.game.Data.LocObj[index30].tempLIS.FindWeight(1);
                num17 = this.game.Data.LocObj[index30].tempLIS.FindWeight(2);
              }
              if (!Information.IsNothing( this.game.Data.LocObj[index30].tempLISfreeAP))
              {
                num16 = this.game.Data.LocObj[index30].tempLISfreeAP.FindWeight(1);
                num18 = this.game.Data.LocObj[index30].tempLISfreeAP.FindWeight(2);
              }
            }
            let mut num19: i32 =  1;
            do
            {
              let mut num20: i32 =  1;
              if (num19 == 2)
                num20 = 2;
              let mut num21: i32 =  num20;
              for (let mut index31: i32 =  1; index31 <= num21; index31 += 1)
              {
                let mut theater: i32 =  0;
                tdata1: i32;
                num22: i32;
                switch (num19)
                {
                  case 1:
                    tdata1 = num15;
                    num22 = num16;
                    movetype1 = 9;
                    index19 = 1;
                    break;
                  case 2:
                    tdata1 = num17;
                    num22 = num18;
                    movetype1 = 12;
                    index19 = 2;
                    break;
                }
                if (tdata1 > 0 & num22 > 0)
                {
                  if (flag1 & num19 == 2)
                  {
                    if (index31 == 1)
                    {
                      tdata1 =  Math.Round(Math.Floor( tdata1 * 0.9));
                      this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype1, theater, num22 * 2 + 200, index18, y1, 0, NoAPPenalties: isPreview, tempZoneTest: true, roadsOnly: true, lisMode: index19);
                    }
                    else
                    {
                      tdata1 =  Math.Round(Math.Floor( tdata1 * 0.1));
                      this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype1, theater, num22 * 2 + 200, index18, y1, 0, NoAPPenalties: isPreview, roadsOnly: true, lisMode: index19);
                    }
                  }
                  else
                    this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype1, theater, num22 * 2 + 200, index18, y1, 0, NoAPPenalties: isPreview, roadsOnly: true, lisMode: index19);
                  object[,,] objArray4 = new object[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 7];
                  SimpleList simpleList1 = SimpleList::new();
                  SimpleList simpleList2 = SimpleList::new();
                  simpleList1.Add(index18 * 1000 + y1, 0, 1, tdata3: index18, tdata4: y1);
                  simpleList2.Add(index18 * 1000 + y1, 0, tdata1);
                  bool flag6 = true;
                  while (flag6)
                  {
                    flag6 = false;
                    let mut counter1: i32 =  simpleList1.Counter;
                    for (let mut index32: i32 =  0; index32 <= counter1; index32 += 1)
                    {
                      if (simpleList1.Data1[index32] == 1)
                      {
                        let mut cx: i32 =  simpleList1.Data3[index32];
                        let mut cy: i32 =  simpleList1.Data4[index32];
                        let mut num23: i32 =  this.game.EditObj.TempValue[0].Value[cx, cy];
                        index4 = simpleList1.Weight[index32];
                        let mut tweight: i32 =  simpleList2.Weight[index32];
                        tdata1 = simpleList2.Data1[index32];
                        let mut tfacing: i32 =  1;
                        do
                        {
                          coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
                          if (coordinate1.onmap)
                          {
                            let mut num24: i32 =  this.game.EditObj.TempValue[0].Value[coordinate1.x, coordinate1.y];
                            if (num24 > num23 & num24 <= num22 * 2)
                            {
                              let mut num25: i32 =  this.game.HandyFunctionsObj.GetLogisticalBonus(coordinate1.x, coordinate1.y, index19);
                              if (num25 > 0)
                              {
                                if (num24 < num25)
                                  num25 = num24;
                                if (num25 > simpleList1.Data2[index32] && num24 > simpleList1.Data2[index32])
                                  simpleList1.Data2[index32] = num24;
                              }
                              let mut num26: i32 =  num24 - num23;
                              let mut num27: i32 =  0;
                              if (simpleList1.Data5[index32] < simpleList1.Data2[index32])
                              {
                                num27 = simpleList1.Data2[index32] - simpleList1.Data5[index32];
                                if (num27 > num26)
                                  num27 = num26;
                                num26 -= num27;
                              }
                              if (Conversions.ToBoolean(Operators.AndObject( (simpleList1.Weight[index32] + num26 <= num22 * 2), Operators.CompareObjectLess(objArray4[cx, cy, tfacing - 1],  1, false))))
                              {
                                simpleList1.FindWeight(-1, tdata3: coordinate1.x, tdata4: coordinate1.y);
                                simpleList1.Add(coordinate1.x * 1000 + coordinate1.y, simpleList1.Weight[index32], 1, simpleList1.Data2[index32], coordinate1.x, coordinate1.y, simpleList1.Data5[index32], false);
                                let mut counter2: i32 =  simpleList1.Counter;
                                int[] weight = simpleList1.Weight;
                                int[] numArray4 = weight;
                                let mut index33: i32 =  counter2;
                                let mut index34: i32 =  index33;
                                let mut num28: i32 =  weight[index33] + num26;
                                numArray4[index34] = num28;
                                int[] data5 = simpleList1.Data5;
                                int[] numArray5 = data5;
                                let mut index35: i32 =  counter2;
                                let mut index36: i32 =  index35;
                                let mut num29: i32 =  data5[index35] + num27;
                                numArray5[index36] = num29;
                                flag6 = true;
                                let mut num30: i32 =  tdata1;
                                if (simpleList1.Weight[counter2] > num22)
                                {
                                  num30 -=  Math.Round( (num30 * (simpleList1.Weight[counter2] - num22)) /  num22);
                                  if (num30 < 0)
                                    num30 = 0;
                                  if (num30 == 0)
                                    simpleList1.Data1[counter2] = 0;
                                }
                                if (cx == 20 & cy == 12)
                                  cx = cx;
                                objArray4[cx, cy, tfacing - 1] =  num30;
                                if (numArray1[coordinate1.x, coordinate1.y, index19] > 0)
                                {
                                  if ( Math.Round(Conversion.Val(this.game.Data.Designer)) == 49)
                                    simpleList1.Data1[counter2] = 0;
                                  else if ( Math.Round(Conversion.Val(this.game.Data.Designer)) >= 50)
                                  {
                                    tweight += 1;
                                    if (tweight <= 3)
                                    {
                                      simpleList1.Weight[index32] = 0;
                                      tdata1 -=  Math.Round( tdata1 * ( simpleList1.Weight[index32] /  (num22 * 2)));
                                      if (tweight == 1)
                                        tdata1 =  Math.Round( tdata1 * 0.75);
                                      if (tweight == 2)
                                        tdata1 =  Math.Round( tdata1 * 0.5);
                                      if (tweight == 3)
                                        tdata1 =  Math.Round( tdata1 * 0.25);
                                    }
                                    else
                                      simpleList1.Data1[counter2] = 0;
                                  }
                                }
                                simpleList2.Add(coordinate1.x * 1000 + coordinate1.y, tweight, tdata1, CheckExistence: false);
                              }
                            }
                          }
                          tfacing += 1;
                        }
                        while (tfacing <= 6);
                        simpleList1.Data1[index32] = 0;
                      }
                    }
                  }
                  let mut mapWidth11: i32 =  this.game.Data.MapObj[0].MapWidth;
                  for (let mut index37: i32 =  0; index37 <= mapWidth11; index37 += 1)
                  {
                    let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                    for (let mut index38: i32 =  0; index38 <= mapHeight; index38 += 1)
                    {
                      let mut index39: i32 =  0;
                      do
                      {
                        int[,,] possiblePull = this.game.EditObj.PossiblePull;
                        int[,,] numArray6 = possiblePull;
                        let mut index40: i32 =  index37;
                        let mut index41: i32 =  index40;
                        let mut index42: i32 =  index38;
                        let mut index43: i32 =  index42;
                        let mut index44: i32 =  index39;
                        let mut index45: i32 =  index44;
                        let mut integer: i32 =  Conversions.ToInteger(Operators.AddObject( possiblePull[index40, index42, index44], objArray4[index37, index38, index39]));
                        numArray6[index41, index43, index45] = integer;
                        index39 += 1;
                      }
                      while (index39 <= 5);
                    }
                  }
                }
              }
              num19 += 1;
            }
            while (num19 <= 2);
          }
        }
        let mut mapWidth12: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index46: i32 =  0; index46 <= mapWidth12; index46 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index47: i32 =  0; index47 <= mapHeight; index47 += 1)
          {
            let mut index48: i32 =  0;
            do
            {
              this.game.EditObj.origPossiblePull[index46, index47, index48] = this.game.EditObj.PossiblePull[index46, index47, index48];
              index48 += 1;
            }
            while (index48 <= 5);
          }
        }
      }
      index49: i32;
      index50: i32;
      num31: i32;
      s4: String;
      if (!isCalibrationCall)
      {
        if (onlyForAssetID > 0)
        {
          let mut mapWidth13: i32 =  this.game.Data.MapObj[0].MapWidth;
          for (let mut index51: i32 =  0; index51 <= mapWidth13; index51 += 1)
          {
            let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
            for (let mut index52: i32 =  0; index52 <= mapHeight; index52 += 1)
            {
              let mut index53: i32 =  0;
              do
              {
                numArray3[index51, index52, index53] = this.game.Data.MapObj[0].HexObj[index51, index52].tempPreviewRoadPull[index53];
                index53 += 1;
              }
              while (index53 <= 5);
            }
          }
        }
        else if ( Math.Round(Conversion.Val(this.game.Data.Designer)) >= 42 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI & !this.game.Data.RegimeObj[this.game.Data.Turn].minimumDataUsage)
        {
          this.game.EventRelatedObj.Helper_MakeListForUnitRequests("SE_Data", -1, false, BothMilitiaAndReg: true);
          int[,,] numArray7 = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 7];
          numArray8: Vec<i32> = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
          numArray9: Vec<i32> = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
          int[,,] numArray10 = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1, 7];
          numArray11: Vec<i32> = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
          int[] numArray12 = new int[this.game.Data.StringListObj[stringListById1].GetHighestValue(0) + 1000 + 1];
          int[] numArray13 = new int[this.game.Data.StringListObj[stringListById1].GetHighestValue(0) + 1000 + 1];
          SimpleList simpleList3 = SimpleList::new();
          SimpleList simpleList4 = SimpleList::new();
          let mut length1: i32 =  this.game.Data.StringListObj[stringListById1].Length;
          for (let mut index54: i32 =  0; index54 <= length1; index54 += 1)
          {
            let mut tid2: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[index54, 0]));
            numArray12[tid2] = -1;
            index4 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[index54, 8]));
            if (index4 == this.game.Data.RegimeObj[this.game.Data.Turn].id)
            {
              numArray12[tid2] = this.game.Data.Turn;
              simpleList4.Add(tid2, 1, index18, y1);
              let mut id: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[index54, 6]));
              if (id > 0)
              {
                let mut locationById: i32 =  this.game.HandyFunctionsObj.GetLocationByID(id);
                if (locationById > -1)
                {
                  simpleList4.Add(tid2, 1, this.game.Data.LocObj[locationById].X, this.game.Data.LocObj[locationById].Y, locationById);
                  numArray13[tid2] = this.game.Data.LocObj[locationById].HQ;
                }
              }
            }
          }
          let mut unitCounter1: i32 =  this.game.Data.UnitCounter;
          for (let mut tid3: i32 =  0; tid3 <= unitCounter1; tid3 += 1)
          {
            if (this.game.Data.UnitObj[tid3].PreDef == -1 & this.game.Data.UnitObj[tid3].Regime == this.game.Data.Turn)
            {
              index4 = this.game.Data.UnitObj[tid3].Historical;
              if (index4 > -1 && this.game.Data.HistoricalUnitObj[index4].Type == 8)
                simpleList3.Add(tid3, 1);
            }
          }
          let mut num32: i32 =  1;
          do
          {
            counter3: i32;
            if (num32 == 3)
              counter3 = simpleList4.Counter;
            if (num32 == 2 | num32 == 1)
              counter3 = simpleList3.Counter;
            if (num32 == 3)
            {
              let mut mapWidth14: i32 =  this.game.Data.MapObj[0].MapWidth;
              for (let mut index55: i32 =  0; index55 <= mapWidth14; index55 += 1)
              {
                let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                for (index49 = 0; index49 <= mapHeight; index49 += 1)
                {
                  let mut index56: i32 =  0;
                  do
                  {
                    numArray7[index55, index49, index56] = numArray3[index55, index49, index56];
                    index56 += 1;
                  }
                  while (index56 <= 5);
                }
              }
            }
            let mut num33: i32 =  counter3;
            for (let mut index57: i32 =  0; index57 <= num33; index57 += 1)
            {
              numArray14: Vec<i32> = new int[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
              num34: i32;
              if (num32 == 3 & num12 < 1)
              {
                num34 = simpleList4.Id[index57];
                index18 = simpleList4.Data1[index57];
                y1 = simpleList4.Data2[index57];
                this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, 8, 0, 1000, index18, y1, 0, NoAPPenalties: true, roadsOnly: true, lisMode: -1, specialRuleNumber: 3);
              }
              hq: i32;
              if (num32 == 2 & num13 < 1 | num32 == 1 & (flag4 | num14 < 1))
              {
                hq = simpleList3.Id[index57];
                index18 = this.game.Data.UnitObj[hq].X;
                y1 = this.game.Data.UnitObj[hq].Y;
                switch (num32)
                {
                  case 1:
                    this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, 8, 0, 1000, index18, y1, 0, NoAPPenalties: true, roadsOnly: true, lisMode: -1, specialRuleNumber: 3, specialRuleNumber2: 1);
                    break;
                  case 2:
                    this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, 11, 0, 2000, index18, y1, 0, NoAPPenalties: true, roadsOnly: true, lisMode: -1, specialRuleNumber: 3, specialRuleNumber2: 1);
                    CoordList tCoordList = CoordList::new();
                    let mut mapWidth15: i32 =  this.game.Data.MapObj[0].MapWidth;
                    for (let mut x: i32 =  0; x <= mapWidth15; x += 1)
                    {
                      let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                      for (index49 = 0; index49 <= mapHeight; index49 += 1)
                      {
                        numArray14[x, index49] = this.game.EditObj.TempValue[0].Value[x, index49];
                        if (this.game.EditObj.TempValue[0].Value[x, index49] < 9999)
                          tCoordList.AddCoord(x, index49, 0, this.game.EditObj.TempValue[0].Value[x, index49], 0);
                      }
                    }
                    this.game.HandyFunctionsObj.MakeMovePrediction2_multiStart(tCoordList, this.game.Data.Turn, 10, 0, 200, index18, y1, 0, NoAPPenalties: true, lisMode: -1);
                    index4 = index4;
                    break;
                }
              }
              let mut mapWidth16: i32 =  this.game.Data.MapObj[0].MapWidth;
              index58: i32;
              for (index58 = 0; index58 <= mapWidth16; index58 += 1)
              {
                let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                for (index49 = 0; index49 <= mapHeight; index49 += 1)
                {
                  numArray9[index58, index49] = 0;
                  numArray11[index58, index49] = 0;
                  numArray8[index58, index49] = 0;
                  let mut index59: i32 =  0;
                  do
                  {
                    numArray10[index58, index49, index59] = 0;
                    index59 += 1;
                  }
                  while (index59 <= 6);
                }
              }
              if (num32 == 2 & num13 < 1)
              {
                let mut unitCounter2: i32 =  this.game.Data.UnitCounter;
                for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
                {
                  if (this.game.Data.UnitObj[unr].Regime == this.game.Data.Turn && this.game.Data.UnitObj[unr].PreDef == -1 && !(this.game.Data.UnitObj[unr].X == this.game.Data.UnitObj[hq].X & this.game.Data.UnitObj[unr].Y == this.game.Data.UnitObj[hq].Y) && this.game.HandyFunctionsObj.IsUnitInHQChain(unr, hq))
                  {
                    let mut num35: i32 =  -1;
                    Coordinate coordinate2;
                    coordinate2.onmap = true;
                    coordinate2.x = this.game.Data.UnitObj[unr].X;
                    coordinate2.y = this.game.Data.UnitObj[unr].Y;
                    if (unr == 345)
                      unr = unr;
                    y2: i32;
                    for (; coordinate2.onmap; coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate2.x, coordinate2.y])
                    {
                      if (numArray14[coordinate2.x, coordinate2.y] < 9999 | coordinate2.x == index18 & coordinate2.y == y1)
                      {
                        num35 = coordinate2.x;
                        y2 = coordinate2.y;
                        break;
                      }
                    }
                    if (coordinate2.x == 23 & coordinate2.y == 12 & num32 == 2)
                      index58 = index58;
                    if (num35 > -1)
                    {
                      let mut num36: i32 =  0;
                      let mut counter4: i32 =  this.game.Data.UnitObj[unr].tempRequestItems.Counter;
                      for (index50 = 0; index50 <= counter4; index50 += 1)
                      {
                        let mut idValue: i32 =  this.game.Data.UnitObj[unr].tempRequestItems.Id[index50];
                        let mut num37: i32 =  this.game.Data.UnitObj[unr].tempRequestItems.Weight[index50];
                        let mut num38: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, idValue, 3)));
                        if (num38 > 0)
                          num36 += num37 * num38;
                      }
                      if (num36 > 0)
                      {
                        let mut num39: i32 =  num35;
                        let mut num40: i32 =  y2;
                        numArray15: Vec<i32> = numArray8;
                        numArray16: Vec<i32> = numArray15;
                        let mut index60: i32 =  num39;
                        let mut index61: i32 =  index60;
                        let mut index62: i32 =  num40;
                        let mut index63: i32 =  index62;
                        let mut num41: i32 =  numArray15[index60, index62] + num36;
                        numArray16[index61, index63] = num41;
                      }
                    }
                  }
                }
              }
              if (num32 == 3 & num12 < 1)
              {
                let mut length2: i32 =  this.game.Data.StringListObj[stringListById3].Length;
                for (let mut index64: i32 =  0; index64 <= length2; index64 += 1)
                {
                  index4 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 0]));
                  let mut num42: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 14]));
                  if (num42 > 0)
                    index4 = num42;
                  if (index4 == num34 && numArray12[index4] == this.game.Data.Turn)
                  {
                    let mut idValue: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 1]));
                    let mut num43: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 5]));
                    let mut num44: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 8]));
                    let mut num45: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById4].GetData(0, idValue, 2)));
                    let mut num46: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById4].GetData(0, idValue, 5)));
                    let mut num47: i32 =  0;
                    if (num43 >= 0)
                    {
                      num47 = num45 * 100;
                      if (num46 <= 0)
                      {
                        num47 =  Math.Round( num47 / 2.0);
                        if (num44 > 0)
                          num47 = 0;
                      }
                    }
                    if (num47 > 0)
                    {
                      let mut index65: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 3]));
                      let mut index66: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 4]));
                      if (!(index65 == index18 & index66 == y1))
                      {
                        let mut num48: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].Data[index64, 15]));
                        if (num48 > 0)
                          num47 =  Math.Round(Math.Ceiling( (num47 * num48) / 100.0));
                        if (num47 > numArray9[index65, index66])
                          numArray9[index65, index66] = num47;
                      }
                    }
                  }
                }
              }
              if (num32 == 1 & num14 < 1)
              {
                let mut counter5: i32 =  simpleList4.Counter;
                for (let mut index67: i32 =  0; index67 <= counter5; index67 += 1)
                {
                  if (numArray13[simpleList4.Id[index67]] == hq && !(simpleList4.Data1[index67] == this.game.Data.UnitObj[hq].X & simpleList4.Data2[index67] == this.game.Data.UnitObj[hq].Y))
                  {
                    index4 = simpleList4.Data3[index67];
                    let mut num49: i32 =  0;
                    let mut logCounter: i32 =  this.game.Data.LocObj[index4].LogCounter;
                    for (let mut index68: i32 =  0; index68 <= logCounter; index68 += 1)
                    {
                      if (isPreview & this.game.Data.LocObj[index4].LogType[index68] == 202 | !isPreview & this.game.Data.LocObj[index4].LogType[index68] == 2202)
                      {
                        let mut idValue: i32 =  this.game.Data.LocObj[index4].LogData1[index68];
                        let mut num50: i32 =  this.game.Data.LocObj[index4].LogData3[index68];
                        let mut num51: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, idValue, 3)));
                        if (num51 > 0)
                          num49 += num50 * num51;
                      }
                    }
                    this.game.EventRelatedObj.Helper_MakeListForLocationRequests("SE_Data", simpleList4.Id[index67], true, true);
                    let mut num52: i32 =  0;
                    let mut counter6: i32 =  this.game.Data.LocObj[index4].tempRequestItems.Counter;
                    for (index50 = 0; index50 <= counter6; index50 += 1)
                    {
                      let mut idValue: i32 =  this.game.Data.LocObj[index4].tempRequestItems.Id[index50];
                      let mut num53: i32 =  this.game.Data.LocObj[index4].tempRequestItems.Weight[index50];
                      let mut num54: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].GetData(0, idValue, 3)));
                      if (num54 > 0)
                        num52 += num53 * num54;
                    }
                    let mut num55: i32 =  num49 + num52;
                    if (num55 > 0)
                    {
                      let mut num56: i32 =  simpleList4.Data1[index67];
                      let mut num57: i32 =  simpleList4.Data2[index67];
                      numArray17: Vec<i32> = numArray11;
                      numArray18: Vec<i32> = numArray17;
                      let mut index69: i32 =  num56;
                      let mut index70: i32 =  index69;
                      let mut index71: i32 =  num57;
                      let mut index72: i32 =  index71;
                      let mut num58: i32 =  numArray17[index69, index71] + num55;
                      numArray18[index70, index72] = num58;
                    }
                  }
                }
              }
              let mut mapWidth17: i32 =  this.game.Data.MapObj[0].MapWidth;
              for (let mut index73: i32 =  0; index73 <= mapWidth17; index73 += 1)
              {
                let mut mapHeight1: i32 =  this.game.Data.MapObj[0].MapHeight;
                for (index49 = 0; index49 <= mapHeight1; index49 += 1)
                {
                  index4 = this.game.Data.RegimeObj[this.game.Data.Turn].Trafic2[0].Value[index73, index49];
                  num31 = 0;
                  bool flag7 = true;
                  if (index4 < 0)
                  {
                    numArray9[index73, index49] = 0;
                    numArray8[index73, index49] = 0;
                    numArray11[index73, index49] = 0;
                  }
                  int[] liSpull1 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                  int[] numArray19 = liSpull1;
                  let mut index74: i32 =  1;
                  let mut index75: i32 =  index74;
                  let mut num59: i32 =  liSpull1[index74] + numArray9[index73, index49];
                  numArray19[index75] = num59;
                  int[] liSpull2 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                  int[] numArray20 = liSpull2;
                  let mut index76: i32 =  2;
                  let mut index77: i32 =  index76;
                  let mut num60: i32 =  liSpull2[index76] + numArray8[index73, index49];
                  numArray20[index77] = num60;
                  int[] liSpull3 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                  int[] numArray21 = liSpull3;
                  let mut index78: i32 =  3;
                  let mut index79: i32 =  index78;
                  let mut num61: i32 =  liSpull3[index78] + numArray11[index73, index49];
                  numArray21[index79] = num61;
                  if (numArray9[index73, index49] > 0 & index1 > -1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                  {
                    s4 = numArray9[index73, index49].ToString() + " Asset Pull Points have been assigned to Hex";
                    this.game.Data.StringListObj[index1].AddRowWithData(index73.ToString(), index49.ToString(), "4", "0", s4, index73.ToString(), index49.ToString(), "0");
                  }
                  if (numArray11[index73, index49] > 0 & index1 > -1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                  {
                    s4 = numArray11[index73, index49].ToString() + " City Pull Points have been assigned to Hex";
                    this.game.Data.StringListObj[index1].AddRowWithData(index73.ToString(), index49.ToString(), "4", "0", s4, index73.ToString(), index49.ToString(), "0");
                  }
                  if (numArray8[index73, index49] > 0 & index1 > -1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                  {
                    s4 = numArray8[index73, index49].ToString() + " Unit Pull Points have been assigned to Hex";
                    this.game.Data.StringListObj[index1].AddRowWithData(index73.ToString(), index49.ToString(), "4", "0", s4, index73.ToString(), index49.ToString(), "0");
                  }
                  if (index4 == -1)
                  {
                    if (num32 == 1 && index4 == -1 & index1 > -1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                    {
                      s4 = "Custom Pull Points sets Hex to 0 Pull Points";
                      this.game.Data.StringListObj[index1].AddRowWithData(index73.ToString(), index49.ToString(), "4", "0", s4, index73.ToString(), index49.ToString(), "0");
                    }
                    if (index4 == -1)
                      index4 = 0;
                    this.game.Data.MapObj[0].HexObj[index73, index49].LISpull[0] = index4;
                  }
                  else
                  {
                    if (num32 != 1)
                      index4 = 0;
                    if (index4 < 0)
                    {
                      flag7 = false;
                      index4 = Math.Abs(index4);
                    }
                    if (index4 > 0 & index1 > -1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                    {
                      if (flag7)
                        s4 = index4.ToString() + " Custom Pull Points have been added on Hex";
                      if (!flag7)
                        s4 = index4.ToString() + " Custom Pull Points have replaced any Auto Pull Points";
                      this.game.Data.StringListObj[index1].AddRowWithData(index73.ToString(), index49.ToString(), "4", "0", s4, index73.ToString(), index49.ToString(), "0");
                    }
                    if (flag7)
                    {
                      if (num32 == 3)
                      {
                        index4 += numArray9[index73, index49];
                        let mut num62: i32 =  numArray9[index73, index49];
                        if (num62 > 0)
                          num62 = num62;
                        int[] liSpull4 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                        int[] numArray22 = liSpull4;
                        let mut index80: i32 =  0;
                        let mut index81: i32 =  index80;
                        let mut num63: i32 =  liSpull4[index80] + num62;
                        numArray22[index81] = num63;
                      }
                      if (num32 == 2)
                      {
                        index4 += numArray8[index73, index49];
                        let mut num64: i32 =  numArray8[index73, index49];
                        int[] liSpull5 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                        int[] numArray23 = liSpull5;
                        let mut index82: i32 =  0;
                        let mut index83: i32 =  index82;
                        let mut num65: i32 =  liSpull5[index82] + num64;
                        numArray23[index83] = num65;
                      }
                      if (num32 == 1)
                      {
                        int[] liSpull6 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                        int[] numArray24 = liSpull6;
                        let mut index84: i32 =  4;
                        let mut index85: i32 =  index84;
                        let mut num66: i32 =  liSpull6[index84] + index4;
                        numArray24[index85] = num66;
                        int[] liSpull7 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                        int[] numArray25 = liSpull7;
                        let mut index86: i32 =  0;
                        let mut index87: i32 =  index86;
                        let mut num67: i32 =  liSpull7[index86] + index4;
                        numArray25[index87] = num67;
                        index4 += numArray11[index73, index49];
                        let mut num68: i32 =  numArray11[index73, index49];
                        int[] liSpull8 = this.game.Data.MapObj[0].HexObj[index73, index49].LISpull;
                        int[] numArray26 = liSpull8;
                        let mut index88: i32 =  0;
                        let mut index89: i32 =  index88;
                        let mut num69: i32 =  liSpull8[index88] + num68;
                        numArray26[index89] = num69;
                      }
                    }
                    else if (num32 == 1)
                    {
                      this.game.Data.MapObj[0].HexObj[index73, index49].LISpull[4] = -index4;
                      this.game.Data.MapObj[0].HexObj[index73, index49].LISpull[0] = -index4;
                    }
                  }
                  if (index4 > 0)
                  {
                    bool flag8 = true;
                    let mut num70: i32 =  -1;
                    while (flag8)
                    {
                      num70 += 1;
                      if (num70 <= 0 || num70 <= 10)
                      {
                        let mut num71: i32 =  index4;
                        flag8 = false;
                        if (num70 > 0)
                        {
                          switch (num32)
                          {
                            case 1:
                              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, 8, 0, 1000, index18, y1, 0, NoAPPenalties: true, roadsOnly: true, lisMode: -1, specialRuleNumber: 3, specialRuleNumber2: index4);
                              break;
                            case 2:
                              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, 11, 0, 2000, index18, y1, 0, NoAPPenalties: true, roadsOnly: true, lisMode: -1, specialRuleNumber: 3, specialRuleNumber2: index4);
                              CoordList tCoordList = CoordList::new();
                              let mut mapWidth18: i32 =  this.game.Data.MapObj[0].MapWidth;
                              for (let mut x: i32 =  0; x <= mapWidth18; x += 1)
                              {
                                let mut mapHeight2: i32 =  this.game.Data.MapObj[0].MapHeight;
                                for (let mut y3: i32 =  0; y3 <= mapHeight2; y3 += 1)
                                {
                                  if (this.game.EditObj.TempValue[0].Value[x, y3] < 9999)
                                    tCoordList.AddCoord(x, y3, 0, this.game.EditObj.TempValue[0].Value[x, y3], 0);
                                }
                              }
                              this.game.HandyFunctionsObj.MakeMovePrediction2_multiStart(tCoordList, this.game.Data.Turn, 10, 0, 200, index18, y1, 0, NoAPPenalties: true, lisMode: -1);
                              break;
                            case 3:
                              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, 8, 0, 1000, index18, y1, 0, NoAPPenalties: true, roadsOnly: true, lisMode: -1, specialRuleNumber: 3, specialRuleNumber2: index4);
                              break;
                          }
                        }
                        Coordinate coordinate3;
                        coordinate3.onmap = true;
                        coordinate3.x = index73;
                        coordinate3.y = index49;
                        Coordinate coordinate4;
                        for (; coordinate3.onmap; coordinate3 = coordinate4)
                        {
                          coordinate4 = this.game.EditObj.TempCameFrom[0].Value[coordinate3.x, coordinate3.y];
                          if (coordinate4.onmap | coordinate4.x == index18 & coordinate4.y == y1)
                          {
                            let mut index90: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate3.x, coordinate3.y, 0, coordinate4.x, coordinate4.y, 0) - 1;
                            let mut index91: i32 =  index90 + 3;
                            if (index91 > 5)
                              index91 -= 6;
                            if (numArray2[coordinate4.x, coordinate4.y, index91, 2] < 7)
                            {
                              if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 0)
                                index50 = 100;
                              if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 1)
                                index50 = 80;
                              if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 2)
                                index50 = 60;
                              if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 3)
                                index50 = 40;
                              if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 4)
                                index50 = 20;
                              if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 5)
                                index50 = 10;
                              if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 6)
                                index50 = 5;
                              if (index50 < 100)
                                index4 =  Math.Round(Math.Floor( (index4 * index50) / 100.0));
                            }
                            else if (numArray2[coordinate4.x, coordinate4.y, index91, 2] == 7)
                              index4 = 0;
                            if (coordinate4.x == 22 & coordinate4.y == 15)
                              index4 = index4;
                            if (index4 > this.game.EditObj.PossiblePull[coordinate4.x, coordinate4.y, index91])
                              index4 = this.game.EditObj.PossiblePull[coordinate4.x, coordinate4.y, index91];
                            if (num32 == 3)
                            {
                              if ( Math.Round(Math.Floor( (numArray10[coordinate3.x, coordinate3.y, 6] * index50) / 100.0)) > index4)
                                index4 = numArray10[coordinate3.x, coordinate3.y, 6];
                              if (index4 > numArray10[coordinate4.x, coordinate4.y, index91])
                              {
                                int[,,] numArray27 = numArray10;
                                int[,,] numArray28 = numArray27;
                                let mut x: i32 =  coordinate4.x;
                                let mut index92: i32 =  x;
                                let mut y4: i32 =  coordinate4.y;
                                let mut index93: i32 =  y4;
                                let mut index94: i32 =  6;
                                let mut index95: i32 =  index94;
                                let mut num72: i32 =  numArray27[x, y4, index94] + (index4 - numArray10[coordinate4.x, coordinate4.y, index91]);
                                numArray28[index92, index93, index95] = num72;
                                numArray10[coordinate4.x, coordinate4.y, index91] = index4;
                              }
                              if (index4 > numArray3[coordinate4.x, coordinate4.y, index91])
                              {
                                if (coordinate3.x == 8 & coordinate3.y == 26)
                                  index4 = index4;
                                num31 = 0;
                                let mut num73: i32 =  0;
                                Coordinate coordinate5 = this.game.EditObj.TempCameFrom[0].Value[coordinate4.x, coordinate4.y];
                                if (coordinate5.onmap)
                                {
                                  index50 = this.game.HandyFunctionsObj.HexFacing(coordinate5.x, coordinate5.y, 0, coordinate4.x, coordinate4.y, 0) - 1;
                                  num73 = numArray7[coordinate5.x, coordinate5.y, index50];
                                }
                                let mut num74: i32 =  numArray7[coordinate4.x, coordinate4.y, index91];
                                if (num73 > num74)
                                {
                                  int[,,] numArray29 = numArray10;
                                  int[,,] numArray30 = numArray29;
                                  let mut x: i32 =  coordinate4.x;
                                  let mut index96: i32 =  x;
                                  let mut y5: i32 =  coordinate4.y;
                                  let mut index97: i32 =  y5;
                                  let mut index98: i32 =  6;
                                  let mut index99: i32 =  index98;
                                  let mut num75: i32 =  numArray29[x, y5, index98] + (num73 - num74);
                                  numArray30[index96, index97, index99] = num75;
                                }
                                int[,,] possiblePull = this.game.EditObj.PossiblePull;
                                int[,,] numArray31 = possiblePull;
                                let mut x1: i32 =  coordinate4.x;
                                let mut index100: i32 =  x1;
                                let mut y6: i32 =  coordinate4.y;
                                let mut index101: i32 =  y6;
                                let mut index102: i32 =  index91;
                                let mut index103: i32 =  index102;
                                let mut num76: i32 =  possiblePull[x1, y6, index102] - Math.Max(0, index4 - numArray10[coordinate3.x, coordinate3.y, index90] - numArray3[coordinate4.x, coordinate4.y, index91]);
                                numArray31[index100, index101, index103] = num76;
                                if (0 > this.game.EditObj.PossiblePull[coordinate4.x, coordinate4.y, index91])
                                  this.game.EditObj.PossiblePull[coordinate4.x, coordinate4.y, index91] = 0;
                                if (index4 > numArray3[coordinate4.x, coordinate4.y, index91])
                                  numArray3[coordinate4.x, coordinate4.y, index91] = index4;
                              }
                            }
                            else
                            {
                              int[,,] numArray32 = numArray3;
                              int[,,] numArray33 = numArray32;
                              let mut x2: i32 =  coordinate4.x;
                              let mut index104: i32 =  x2;
                              let mut y7: i32 =  coordinate4.y;
                              let mut index105: i32 =  y7;
                              let mut index106: i32 =  index91;
                              let mut index107: i32 =  index106;
                              let mut num77: i32 =  numArray32[x2, y7, index106] + index4;
                              numArray33[index104, index105, index107] = num77;
                              int[,,] possiblePull = this.game.EditObj.PossiblePull;
                              int[,,] numArray34 = possiblePull;
                              let mut x3: i32 =  coordinate4.x;
                              let mut index108: i32 =  x3;
                              let mut y8: i32 =  coordinate4.y;
                              let mut index109: i32 =  y8;
                              let mut index110: i32 =  index91;
                              let mut index111: i32 =  index110;
                              let mut num78: i32 =  possiblePull[x3, y8, index110] - index4;
                              numArray34[index108, index109, index111] = num78;
                            }
                            if (index4 < 1)
                              break;
                          }
                        }
                        if (num71 > index4 & num71 != index4 & num71 > 0 & index4 > 0)
                        {
                          flag8 = true;
                          index4 = num71 - index4;
                        }
                      }
                      else
                        break;
                    }
                  }
                }
              }
            }
            num32 += 1;
          }
          while (num32 <= 3);
          let mut mapWidth19: i32 =  this.game.Data.MapObj[0].MapWidth;
          for (let mut index112: i32 =  0; index112 <= mapWidth19; index112 += 1)
          {
            let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
            for (index49 = 0; index49 <= mapHeight; index49 += 1)
            {
              let mut index113: i32 =  0;
              do
              {
                this.game.Data.MapObj[0].HexObj[index112, index49].tempPreviewRoadPull[index113] = numArray3[index112, index49, index113];
                index113 += 1;
              }
              while (index113 <= 5);
            }
          }
        }
      }
      if (!isCalibrationCall & isPreview)
      {
        let mut locCounter5: i32 =  this.game.Data.LocCounter;
        for (let mut locnr: i32 =  0; locnr <= locCounter5; locnr += 1)
        {
          let mut x: i32 =  this.game.Data.LocObj[locnr].X;
          let mut y9: i32 =  this.game.Data.LocObj[locnr].Y;
          flag5 = false;
          this.game.Data.LocObj[locnr].tempLIS = SimpleList::new();
          this.game.Data.LocObj[locnr].tempLISfreeAP = SimpleList::new();
          if (this.game.Data.MapObj[0].HexObj[x, y9].Regime == this.game.Data.Turn && num1 == -1 | num1 == x & num2 == y9)
          {
            if (num1 == -1)
            {
              let mut id: i32 =  this.game.Data.LocObj[locnr].ID;
              let mut num79: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].GetData(6, id, 0)));
              data9: DataClass = this.game.Data;
              str9: String = "truckPoints";
               local9: String =  str9;
              libName15: String = libName1;
              let mut libVar9: i32 =  data9.FindLibVar( local9, libName15);
              let mut hexLibVarValue4: i32 =  this.game.Data.MapObj[0].HexObj[x, y9].GetHexLibVarValue(libVar9);
              if (this.game.Data.RegimeObj[this.game.Data.Turn].AI & num79 > 0)
                hexLibVarValue4 += 150000;
              data10: DataClass = this.game.Data;
              str10: String = "truckFreeAp";
               local10: String =  str10;
              libName16: String = libName1;
              let mut libVar10: i32 =  data10.FindLibVar( local10, libName16);
              let mut hexLibVarValue5: i32 =  this.game.Data.MapObj[0].HexObj[x, y9].GetHexLibVarValue(libVar10);
              if (this.game.Data.RegimeObj[this.game.Data.Turn].AI & num79 > 0)
                hexLibVarValue5 += 1000;
              data11: DataClass = this.game.Data;
              str11: String = "maglevPoints";
               local11: String =  str11;
              libName17: String = libName1;
              let mut libVar11: i32 =  data11.FindLibVar( local11, libName17);
              let mut hexLibVarValue6: i32 =  this.game.Data.MapObj[0].HexObj[x, y9].GetHexLibVarValue(libVar11);
              data12: DataClass = this.game.Data;
              str12: String = "maglevFreeAp";
               local12: String =  str12;
              libName18: String = libName1;
              let mut libVar12: i32 =  data12.FindLibVar( local12, libName18);
              let mut hexLibVarValue7: i32 =  this.game.Data.MapObj[0].HexObj[x, y9].GetHexLibVarValue(libVar12);
              this.game.EventRelatedObj.ExecSetLIS(locnr, 1, -1, hexLibVarValue4, "");
              this.game.EventRelatedObj.ExecSetLISfreeAP(locnr, 1, -1, hexLibVarValue5, "");
              this.game.EventRelatedObj.ExecSetLIS(locnr, 2, -1, hexLibVarValue6, "");
              this.game.EventRelatedObj.ExecSetLISfreeAP(locnr, 2, -1, hexLibVarValue7, "");
            }
            else
            {
              if (pts4 > 0)
                this.game.EventRelatedObj.ExecSetLIS(locnr, 1, -1, pts4, "");
              if (pts3 > 0)
                this.game.EventRelatedObj.ExecSetLISfreeAP(locnr, 1, -1, pts3, "");
              if (pts2 > 0)
                this.game.EventRelatedObj.ExecSetLIS(locnr, 2, -1, pts2, "");
              if (pts1 > 0)
                this.game.EventRelatedObj.ExecSetLISfreeAP(locnr, 2, -1, pts1, "");
            }
          }
        }
      }
      let mut num80: i32 =  0;
      do
      {
        let mut locCounter6: i32 =  this.game.Data.LocCounter;
        for (let mut index114: i32 =  0; index114 <= locCounter6; index114 += 1)
        {
          let mut x: i32 =  this.game.Data.LocObj[index114].X;
          let mut y10: i32 =  this.game.Data.LocObj[index114].Y;
          bool flag9 = false;
          if (this.game.Data.MapObj[0].HexObj[x, y10].Regime == this.game.Data.Turn && !Information.IsNothing( this.game.Data.LocObj[index114].tempLIS) && this.game.Data.LocObj[index114].tempLIS.Counter > -1)
            flag9 = true;
          if (flag9)
          {
            if (x == 89 & y10 == 13)
              x = x;
            let mut num81: i32 =  100;
            let mut length: i32 =  this.game.Data.StringListObj[stringListById6].Length;
            for (let mut index115: i32 =  0; index115 <= length; index115 += 1)
            {
              let mut index116: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].Data[index115, 0]));
              bool flag10 = false;
              if (num80 == 1 & index116 == 1)
                flag10 = true;
              if (num80 == 0 & index116 == 2)
                flag10 = true;
              if (flag10)
              {
                let mut movetype2: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].Data[index115, 4]));
                str13: String = this.game.Data.StringListObj[stringListById6].Data[index115, 1];
                let mut theater: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById6].Data[index115, 2]));
                let mut nr: i32 =  this.game.Data.LocObj[index114].tempLIS.FindNr(index116);
                let mut num82: i32 =  0;
                if (nr > -1)
                  num82 =  Math.Round(Math.Floor( (this.game.Data.LocObj[index114].tempLIS.Weight[nr] * num81) / 100.0));
                if (num82 > 0)
                {
                  let mut index117: i32 =  this.game.Data.LocObj[index114].tempLISfreeAP.FindNr(index116);
                  let mut num83: i32 =  0;
                  if (index117 > -1)
                    num83 = this.game.Data.LocObj[index114].tempLISfreeAP.Weight[index117];
                  if (num83 > 0)
                  {
                    let mut num84: i32 =  1;
                    if (!isCalibrationCall & index116 == 2 & flag1)
                      num84 = 2;
                    let mut num85: i32 =  num84;
                    for (let mut index118: i32 =  1; index118 <= num85; index118 += 1)
                    {
                      bool flag11 = flag1;
                      num86: i32;
                      CoordList coordList;
                      if (!isCalibrationCall & index116 == 2 & flag1)
                      {
                        if (index118 == 1)
                        {
                          index117 = this.game.Data.LocObj[index114].tempLIS.FindNr(index116);
                          num86 = 0;
                          num82 =  Math.Round(Math.Floor( (this.game.Data.LocObj[index114].tempLIS.Weight[index117] * num81) / 100.0 * 0.9));
                          coordList = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype2, theater, num83 * 2, x, y10, 0, NoAPPenalties: isPreview, tempZoneTest: true, roadsOnly: true, alwaysUseLogisticalBonus: true, lisMode: index116);
                        }
                        else
                        {
                          flag11 = false;
                          index117 = this.game.Data.LocObj[index114].tempLIS.FindNr(index116);
                          num86 = 0;
                          num82 =  Math.Round(Math.Floor( (this.game.Data.LocObj[index114].tempLIS.Weight[index117] * num81) / 100.0 * 0.1));
                          coordList = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype2, theater, num83 * 2, x, y10, 0, NoAPPenalties: isPreview, roadsOnly: true, alwaysUseLogisticalBonus: true, lisMode: index116);
                        }
                      }
                      else
                        coordList = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype2, theater, num83 * 2, x, y10, 0, NoAPPenalties: isPreview, roadsOnly: true, alwaysUseLogisticalBonus: true, lisMode: index116);
                      let mut tid4: i32 =  0;
                      let mut num87: i32 =  0;
                      LisRunner[] arySrc = new LisRunner[1];
                      arySrc[0].awaitingSplit = true;
                      arySrc[0].originRunner = SimpleList::new();
                      arySrc[0].originRunner.Add(0, 1, CheckExistence: false);
                      arySrc[0].x = x;
                      arySrc[0].y = y10;
                      arySrc[0].apUsed = 0;
                      arySrc[0].direction = -1;
                      arySrc[0].lisPoints = num82;
                      bool flag12 = true;
                      bool flag13 = true;
                      strArray1: Vec<String> = new string[2]
                      {
                        "[",
                        "]"
                      };
                      SimpleList[,] simpleListArray = new SimpleList[this.game.Data.MapObj[0].MapWidth + 1, this.game.Data.MapObj[0].MapHeight + 1];
                      simpleListArray[x, y10] = SimpleList::new();
                      simpleListArray[x, y10].AddBlind(0, 1);
                      int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[x, y10].LISpoints;
                      int[] numArray35 = liSpoints1;
                      let mut index119: i32 =  6;
                      let mut index120: i32 =  index119;
                      let mut num88: i32 =  liSpoints1[index119] + num82;
                      numArray35[index120] = num88;
                      if (index1 > -1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                      {
                        if (x == 13 & y10 == 11)
                          x = x;
                        s4 = num82.ToString() + " Logistical Pts using '" + str13 + "' movement have been generated in Hex";
                        this.game.Data.StringListObj[index1].AddRowWithData(x.ToString(), y10.ToString(), "4", "0", s4, x.ToString(), y10.ToString(), index116.ToString());
                      }
                      while (flag13)
                      {
                        flag13 = false;
                        let mut num89: i32 =  num87;
                        let mut num90: i32 =  tid4;
                        num91: i32;
                        for (let mut index121: i32 =  num89; index121 <= num90; index121 += 1)
                        {
                          if (arySrc[index121].active)
                          {
                            flag13 = true;
                            arySrc[index121].active = false;
                            arySrc[index121].awaitingSplit = false;
                            bool flag14 = true;
                            if (index121 == 194)
                              index121 = index121;
                            while (flag14)
                            {
                              flag14 = false;
                              Coordinate coordinate6 = this.game.HandyFunctionsObj.HexNeighbour(arySrc[index121].x, arySrc[index121].y, 0, arySrc[index121].direction + 1);
                              Coordinate coordinate7 = this.game.HandyFunctionsObj.MoveApCostPreview2(arySrc[index121].x, arySrc[index121].y, this.game.Data.Turn, movetype2, theater, arySrc[index121].x, arySrc[index121].y, 0, coordinate6.x, coordinate6.y, 0, NoAPPenalties: isPreview, roadsOnly: true, alwaysUseLogisticalBonus: true);
                              let mut index122: i32 =  this.game.Data.MapObj[0].HexObj[arySrc[index121].x, arySrc[index121].y].RoadType[arySrc[index121].direction];
                              if (index122 > -1)
                              {
                                if (this.game.Data.RoadTypeObj[index122].MoveCostOverrule[movetype2] >= 999)
                                  coordinate6.onmap = false;
                              }
                              else
                                coordinate6.onmap = false;
                              if (coordinate6.onmap && index116 == 2 & index118 == 1 && this.game.EditObj.tempZoneTest[coordinate6.x, coordinate6.y] < 1)
                                coordinate6.onmap = false;
                              if (coordinate6.onmap)
                              {
                                if (arySrc[index121].x == 14 & arySrc[index121].y == 12)
                                  index121 = index121;
                                if (arySrc[index121].apUsed + coordinate7.x < num83 * 2)
                                {
                                  let mut num92: i32 =  this.game.HandyFunctionsObj.GetLogisticalBonus(arySrc[index121].x, arySrc[index121].y, index116);
                                  if ( Math.Round(Conversion.Val(this.game.Data.Designer)) > 40 && arySrc[index121].apUsed < num92)
                                  {
                                    num92 = arySrc[index121].apUsed;
                                    if (0 > num92)
                                      num92 = 0;
                                  }
                                  if (isPreview & num5 > 0 && arySrc[index121].x == num3 & arySrc[index121].y == num4 & num3 > -1)
                                  {
                                    num92 -= num5;
                                    if (0 > num92)
                                      num92 = 0;
                                  }
                                  if (num92 > arySrc[index121].extensionAllowed)
                                    arySrc[index121].extensionAllowed = num92;
                                  let mut num93: i32 =  arySrc[index121].extensionAllowed - arySrc[index121].extensionUsed;
                                  let mut num94: i32 =  coordinate7.x;
                                  if (num93 > 0)
                                  {
                                    if (num94 > num93)
                                    {
                                      num94 -= num93;
                                      arySrc[index121].extensionUsed = arySrc[index121].extensionAllowed;
                                    }
                                    else
                                    {
                                      LisRunner[] lisRunnerArray1 = arySrc;
                                      LisRunner[] lisRunnerArray2 = lisRunnerArray1;
                                      index119 = index121;
                                      let mut index123: i32 =  index119;
                                      lisRunnerArray2[index123].extensionUsed = lisRunnerArray1[index119].extensionUsed + num94;
                                      num94 = 0;
                                    }
                                  }
                                  index117 = arySrc[index121].apUsed + num94 <= num83 ? arySrc[index121].lisPoints :  Math.Round( arySrc[index121].lisPoints * ( (num83 - (arySrc[index121].apUsed + num94 - num83)) /  num83));
                                  LisRunner[] lisRunnerArray3 = arySrc;
                                  LisRunner[] lisRunnerArray4 = lisRunnerArray3;
                                  index119 = index121;
                                  let mut index124: i32 =  index119;
                                  lisRunnerArray4[index124].apUsed = lisRunnerArray3[index119].apUsed + num94;
                                  let mut num95: i32 =  arySrc[index121].direction + 3;
                                  if (num95 > 5)
                                    num95 -= 6;
                                  bool flag15 = false;
                                  bool flag16 = false;
                                  bool flag17 = false;
                                  if (arySrc[index121].x == 47 & arySrc[index121].y == 17 & coordinate6.x == 47 & coordinate6.y == 18)
                                    index117 = index117;
                                  if (coordinate6.x == 42 & coordinate6.y == 21 & arySrc[index121].x == 42 & arySrc[index121].y == 20)
                                    index117 = index117;
                                  if (!Information.IsNothing( simpleListArray[coordinate6.x, coordinate6.y]))
                                  {
                                    num31 = 0;
                                    let mut num96: i32 =  0;
                                    let mut num97: i32 =  0;
                                    let mut num98: i32 =  0;
                                    let mut num99: i32 =  0;
                                    num91 = 0;
                                    let mut counter7: i32 =  simpleListArray[coordinate6.x, coordinate6.y].Counter;
                                    for (let mut index125: i32 =  0; index125 <= counter7; index125 += 1)
                                    {
                                      let mut num100: i32 =  0;
                                      num97 += 1;
                                      let mut counter8: i32 =  arySrc[index121].originRunner.Counter;
                                      for (let mut index126: i32 =  0; index126 <= counter8; index126 += 1)
                                      {
                                        if (arySrc[index121].originRunner.Id[index126] == simpleListArray[coordinate6.x, coordinate6.y].Id[index125])
                                          num100 += 1;
                                      }
                                      if (num100 > 0)
                                        num96 += 1;
                                    }
                                    let mut counter9: i32 =  arySrc[index121].originRunner.Counter;
                                    for (index50 = 0; index50 <= counter9; index50 += 1)
                                    {
                                      let mut num101: i32 =  0;
                                      num99 += 1;
                                      let mut counter10: i32 =  simpleListArray[coordinate6.x, coordinate6.y].Counter;
                                      for (let mut index127: i32 =  0; index127 <= counter10; index127 += 1)
                                      {
                                        if (arySrc[index121].originRunner.Id[index50] == simpleListArray[coordinate6.x, coordinate6.y].Id[index127])
                                          num101 += 1;
                                      }
                                      if (num101 > 0)
                                        num98 += 1;
                                    }
                                    if (num98 == num99 | num96 == num97)
                                      flag17 = true;
                                    if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                    {
                                      if (arySrc[index121].originRunner.Counter > 9)
                                      {
                                        flag15 = true;
                                        flag16 = true;
                                        flag17 = true;
                                      }
                                    }
                                    else if (arySrc[index121].originRunner.Counter > 29)
                                    {
                                      flag15 = true;
                                      flag16 = true;
                                      flag17 = true;
                                    }
                                  }
                                  if (!flag17)
                                  {
                                    if (Information.IsNothing( simpleListArray[coordinate6.x, coordinate6.y]))
                                      simpleListArray[coordinate6.x, coordinate6.y] = SimpleList::new();
                                    simpleListArray[coordinate6.x, coordinate6.y].AddWeightBlind( arySrc[index121].originRunner);
                                  }
                                  if (flag15 | flag16)
                                    index117 = 0;
                                  if (!flag15 & index117 > 0)
                                  {
                                    int[] liSpoints2 = this.game.Data.MapObj[0].HexObj[arySrc[index121].x, arySrc[index121].y].LISpoints;
                                    int[] numArray36 = liSpoints2;
                                    index119 = arySrc[index121].direction;
                                    let mut index128: i32 =  index119;
                                    let mut num102: i32 =  liSpoints2[index119] + index117;
                                    numArray36[index128] = num102;
                                    arySrc[index121].awaitingSplit = false;
                                  }
                                  if (index1 > -1 & index117 > 0 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                  {
                                    if (!flag15)
                                      s4 = "Sent " + index117.ToString() + " " + str13 + " Points (ap=" + arySrc[index121].apUsed.ToString() + "/" + num83.ToString() + " ext=" + arySrc[index121].extensionUsed.ToString() + "/" + arySrc[index121].extensionAllowed.ToString() + " br=" + Math.Round( Math.Max(0, arySrc[index121].branchCount - 100) / 100.0, 2).ToString() + ")";
                                    if (flag15)
                                      s4 = "Sent Back " + index117.ToString() + " " + str13 + " Points (ap=" + arySrc[index121].apUsed.ToString() + "/" + num83.ToString() + " ext=" + arySrc[index121].extensionUsed.ToString() + "/" + arySrc[index121].extensionAllowed.ToString() + ")";
                                    if (flag17)
                                      s4 += " [End]";
                                    s4 += " to ";
                                    if (arySrc[index121].direction == 0)
                                      s4 += "North.";
                                    if (arySrc[index121].direction == 1)
                                      s4 += "North-East.";
                                    if (arySrc[index121].direction == 2)
                                      s4 += "South-East.";
                                    if (arySrc[index121].direction == 3)
                                      s4 += "South.";
                                    if (arySrc[index121].direction == 4)
                                      s4 += "South-West.";
                                    if (arySrc[index121].direction == 5)
                                      s4 += "North-West.";
                                    this.game.Data.StringListObj[index1].AddRowWithData(arySrc[index121].x.ToString(), arySrc[index121].y.ToString(), "2", arySrc[index121].direction.ToString(), s4, x.ToString(), y10.ToString(), index116.ToString());
                                  }
                                  if (!arySrc[index121].splitJustDone && !flag15 & index117 > 0 &&  Math.Round(Conversion.Val(this.game.Data.Designer)) >= 42 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI & !this.game.Data.RegimeObj[this.game.Data.Turn].minimumDataUsage && numArray3[arySrc[index121].x, arySrc[index121].y, arySrc[index121].direction] > 0)
                                  {
                                    if (index1 > -1)
                                    {
                                      index119 = Math.Min(numArray3[arySrc[index121].x, arySrc[index121].y, arySrc[index121].direction], index117);
                                      s4 = index119.ToString() + " Pull Points have been removed (dir=";
                                      if (arySrc[index121].direction == 0)
                                        s4 += "North).";
                                      if (arySrc[index121].direction == 1)
                                        s4 += "North-East).";
                                      if (arySrc[index121].direction == 2)
                                        s4 += "South-East).";
                                      if (arySrc[index121].direction == 3)
                                        s4 += "South).";
                                      if (arySrc[index121].direction == 4)
                                        s4 += "South-West).";
                                      if (arySrc[index121].direction == 5)
                                        s4 += "North-West).";
                                      this.game.Data.StringListObj[index1].AddRowWithData(arySrc[index121].x.ToString(), arySrc[index121].y.ToString(), "3", arySrc[index121].direction.ToString(), s4, "-1", "-1", "0");
                                    }
                                    int[,,] numArray37 = numArray3;
                                    int[,,] numArray38 = numArray37;
                                    index119 = arySrc[index121].x;
                                    let mut index129: i32 =  index119;
                                    let mut y11: i32 =  arySrc[index121].y;
                                    let mut index130: i32 =  y11;
                                    let mut direction: i32 =  arySrc[index121].direction;
                                    let mut index131: i32 =  direction;
                                    let mut num103: i32 =  numArray37[index119, y11, direction] - index117;
                                    numArray38[index129, index130, index131] = num103;
                                    if (0 > numArray3[arySrc[index121].x, arySrc[index121].y, arySrc[index121].direction])
                                      numArray3[arySrc[index121].x, arySrc[index121].y, arySrc[index121].direction] = 0;
                                  }
                                  if (!flag16 & index117 > 0)
                                  {
                                    int[] liSpoints3 = this.game.Data.MapObj[0].HexObj[coordinate6.x, coordinate6.y].LISpoints;
                                    int[] numArray39 = liSpoints3;
                                    index119 = num95;
                                    let mut index132: i32 =  index119;
                                    let mut num104: i32 =  liSpoints3[index119] + index117;
                                    numArray39[index132] = num104;
                                  }
                                  if (index1 > -1 & index117 > 0 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                  {
                                    if (!flag16)
                                      s4 = "Rec. " + index117.ToString() + " " + str13 + " Points (ap=" + arySrc[index121].apUsed.ToString() + "/" + num83.ToString() + " ext=" + arySrc[index121].extensionUsed.ToString() + "/" + arySrc[index121].extensionAllowed.ToString() + " br=" + Math.Round( Math.Max(0, arySrc[index121].branchCount - 100) / 100.0, 2).ToString() + ")";
                                    if (flag16)
                                      s4 = "Rec. Sent Back " + index117.ToString() + " " + str13 + " Points (ap=" + arySrc[index121].apUsed.ToString() + "/" + num83.ToString() + " ext=" + arySrc[index121].extensionUsed.ToString() + "/" + arySrc[index121].extensionAllowed.ToString() + ")";
                                    if (flag17)
                                      s4 += " [End]";
                                    s4 += " from ";
                                    if (num95 == 0)
                                      s4 += "North.";
                                    if (num95 == 1)
                                      s4 += "North-East.";
                                    if (num95 == 2)
                                      s4 += "South-East.";
                                    if (num95 == 3)
                                      s4 += "South.";
                                    if (num95 == 4)
                                      s4 += "South-West.";
                                    if (num95 == 5)
                                      s4 += "North-West.";
                                    this.game.Data.StringListObj[index1].AddRowWithData(coordinate6.x.ToString(), coordinate6.y.ToString(), "1", num95.ToString(), s4, x.ToString(), y10.ToString(), index116.ToString());
                                  }
                                  if (!flag17 & index117 > 0)
                                  {
                                    int[] liSpoints4 = this.game.Data.MapObj[0].HexObj[coordinate6.x, coordinate6.y].LISpoints;
                                    int[] numArray40 = liSpoints4;
                                    index119 = 6;
                                    let mut index133: i32 =  index119;
                                    let mut num105: i32 =  liSpoints4[index119] + index117;
                                    numArray40[index133] = num105;
                                  }
                                  arySrc[index121].splitJustDone = false;
                                  if (numArray1[coordinate6.x, coordinate6.y, index116] > 0 & !flag17)
                                  {
                                    if ( Math.Round(Conversion.Val(this.game.Data.Designer)) == 49)
                                    {
                                      flag17 = true;
                                      if (index1 > -1 & index117 > 0 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                      {
                                        if (index116 == 1)
                                          s4 = "[Truck Points End with meeting other Truck Logistics Asset]";
                                        if (index116 == 2)
                                          s4 = "[Rail Points End with meeting other Rail Logistics Asset]";
                                        this.game.Data.StringListObj[index1].AddRowWithData(coordinate6.x.ToString(), coordinate6.y.ToString(), "1", num95.ToString(), s4, x.ToString(), y10.ToString(), index116.ToString());
                                      }
                                    }
                                    else if ( Math.Round(Conversion.Val(this.game.Data.Designer)) >= 50)
                                    {
                                      LisRunner[] lisRunnerArray5 = arySrc;
                                      LisRunner[] lisRunnerArray6 = lisRunnerArray5;
                                      index119 = index121;
                                      let mut index134: i32 =  index119;
                                      lisRunnerArray6[index134].reFocusCount = lisRunnerArray5[index119].reFocusCount + 1;
                                      if (arySrc[index121].reFocusCount <= 3)
                                      {
                                        strArray2: Vec<String> = new string[5]
                                        {
                                          "LogPts -",
                                          null,
                                          null,
                                          null,
                                          null
                                        };
                                        strArray3: Vec<String> = strArray2;
                                        index119 =  Math.Round( (arySrc[index121].apUsed * 100) /  (num83 * 2));
                                        str14: String = index119.ToString();
                                        strArray3[1] = str14;
                                        strArray2[2] = "% for ";
                                        strArray2[3] = arySrc[index121].apUsed.ToString();
                                        strArray2[4] = "AP";
                                        str15: String = string.Concat(strArray2);
                                        let mut num106: i32 =  arySrc[index121].lisPoints -  Math.Round( arySrc[index121].lisPoints * ( arySrc[index121].apUsed /  (num83 * 2)));
                                        arySrc[index121].apUsed = 0;
                                        if (arySrc[index121].reFocusCount == 1)
                                        {
                                          num106 =  Math.Round( num106 * 0.75);
                                          str15 = "1st refocus. " + str15 + " & -25%.";
                                        }
                                        if (arySrc[index121].reFocusCount == 2)
                                        {
                                          num106 =  Math.Round( num106 * 0.5);
                                          str15 = "2nd refocus. " + str15 + " & -50%.";
                                        }
                                        if (arySrc[index121].reFocusCount == 3)
                                        {
                                          num106 =  Math.Round( num106 * 0.25);
                                          str15 = "3rd refocus. " + str15 + " & -75%.";
                                        }
                                        arySrc[index121].lisPoints = num106;
                                        str16: String = str15 + " " + num106.ToString() + " LP refocussed.";
                                        if (index1 > -1 & index117 > 0 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                        {
                                          if (index116 == 1)
                                            s4 = "[Trucks " + str16 + "]";
                                          if (index116 == 2)
                                            s4 = "[Rail " + str16 + "]";
                                          this.game.Data.StringListObj[index1].AddRowWithData(coordinate6.x.ToString(), coordinate6.y.ToString(), "1", num95.ToString(), s4, x.ToString(), y10.ToString(), index116.ToString());
                                        }
                                      }
                                      else
                                      {
                                        flag17 = true;
                                        if (index1 > -1 & index117 > 0 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                        {
                                          if (index116 == 1)
                                            s4 = "[Truck Points end with 4th time meeting other Truck Asset]";
                                          if (index116 == 2)
                                            s4 = "[Rail Points end with 4th time meeting other Rail Asset]";
                                          this.game.Data.StringListObj[index1].AddRowWithData(coordinate6.x.ToString(), coordinate6.y.ToString(), "1", num95.ToString(), s4, x.ToString(), y10.ToString(), index116.ToString());
                                        }
                                      }
                                    }
                                  }
                                  if (flag17)
                                  {
                                    arySrc[index121].awaitingSplit = false;
                                  }
                                  else
                                  {
                                    arySrc[index121].x = coordinate6.x;
                                    arySrc[index121].y = coordinate6.y;
                                    let mut num107: i32 =  -1;
                                    let mut num108: i32 =  0;
                                    let mut num109: i32 =  0;
                                    if (coordinate6.x == 13 & coordinate6.y == 11)
                                      index121 = index121;
                                    let mut num110: i32 =  index116 - 1;
                                    if (arySrc[index121].isPull)
                                      num110 = 3;
                                    let mut index135: i32 =  0;
                                    do
                                    {
                                      if (index135 != num95)
                                      {
                                        Coordinate coordinate8 = this.game.HandyFunctionsObj.HexNeighbour(coordinate6.x, coordinate6.y, 0, index135 + 1);
                                        if (coordinate8.onmap)
                                        {
                                          Coordinate coordinate9 = this.game.HandyFunctionsObj.MoveApCostPreview2(coordinate6.x, coordinate6.y, this.game.Data.Turn, movetype2, theater, coordinate6.x, coordinate6.y, 0, coordinate8.x, coordinate8.y, 0, roadsOnly: true, alwaysUseLogisticalBonus: true);
                                          let mut index136: i32 =  this.game.Data.MapObj[0].HexObj[coordinate6.x, coordinate6.y].RoadType[index135];
                                          if (index136 > -1)
                                          {
                                            if (this.game.Data.RoadTypeObj[index136].MoveCostOverrule[movetype2] >= 999)
                                              coordinate9.x = 9999;
                                          }
                                          else
                                            coordinate9.x = 9999;
                                          if (coordinate9.x < 999)
                                          {
                                            num108 += 1;
                                            num107 = index135;
                                          }
                                          else
                                            index121 = index121;
                                        }
                                      }
                                      else if (index135 != num95)
                                        num109 += 1;
                                      index135 += 1;
                                    }
                                    while (index135 <= 5);
                                    if (index117 > 0)
                                    {
                                      if (num108 == 1 & num109 == 0)
                                      {
                                        arySrc[index121].direction = num107;
                                        flag14 = true;
                                      }
                                      else if (num108 > 0 | num109 > 0)
                                      {
                                        if (arySrc[index121].x == 26 & arySrc[index121].y == 3)
                                          index121 = index121;
                                        arySrc[index121].direction = num95;
                                        flag14 = false;
                                        arySrc[index121].awaitingSplit = true;
                                        if (num108 == 0)
                                          arySrc[index121].direction = -1;
                                      }
                                    }
                                    else
                                    {
                                      index117 = index117;
                                      arySrc[index121].awaitingSplit = false;
                                    }
                                  }
                                }
                              }
                              else
                                arySrc[index121].awaitingSplit = false;
                            }
                          }
                        }
                        let mut num111: i32 =  num87;
                        let mut num112: i32 =  tid4;
                        for (let mut index137: i32 =  num111; index137 <= num112; index137 += 1)
                        {
                          if (arySrc[index137].awaitingSplit)
                          {
                            flag13 = true;
                            arySrc[index137].awaitingSplit = false;
                            arySrc[index137].active = false;
                            SimpleList simpleList = SimpleList::new();
                            let mut tid5: i32 =  0;
                            do
                            {
                              if (tid5 != arySrc[index137].direction)
                              {
                                Coordinate coordinate10 = this.game.HandyFunctionsObj.HexNeighbour(arySrc[index137].x, arySrc[index137].y, 0, tid5 + 1);
                                if (coordinate10.onmap && index116 == 2 & index118 == 1 && this.game.EditObj.tempZoneTest[coordinate10.x, coordinate10.y] < 1)
                                  coordinate10.onmap = false;
                                if (coordinate10.onmap)
                                {
                                  Coordinate coordinate11 = this.game.HandyFunctionsObj.MoveApCostPreview2(arySrc[index137].x, arySrc[index137].y, this.game.Data.Turn, movetype2, theater, arySrc[index137].x, arySrc[index137].y, 0, coordinate10.x, coordinate10.y, 0, NoAPPenalties: isPreview, roadsOnly: true, alwaysUseLogisticalBonus: true);
                                  let mut index138: i32 =  this.game.Data.MapObj[0].HexObj[arySrc[index137].x, arySrc[index137].y].RoadType[tid5];
                                  if (index138 > -1)
                                  {
                                    if (this.game.Data.RoadTypeObj[index138].MoveCostOverrule[movetype2] >= 999)
                                      coordinate11.x = 9999;
                                  }
                                  else
                                    coordinate11.x = 9999;
                                  if (arySrc[index137].apUsed + coordinate11.x < num83 * 2)
                                  {
                                    num31 = 0;
                                    let mut num113: i32 =  0;
                                    let mut num114: i32 =  0;
                                    let mut num115: i32 =  0;
                                    let mut num116: i32 =  0;
                                    num91 = 0;
                                    if (arySrc[index137].x == 20 & arySrc[index137].y == 12)
                                      index117 = index117;
                                    bool flag18 = true;
                                    if (!Information.IsNothing( simpleListArray[coordinate10.x, coordinate10.y]))
                                    {
                                      let mut counter11: i32 =  simpleListArray[coordinate10.x, coordinate10.y].Counter;
                                      for (let mut index139: i32 =  0; index139 <= counter11; index139 += 1)
                                      {
                                        let mut num117: i32 =  0;
                                        num114 += 1;
                                        let mut counter12: i32 =  arySrc[index137].originRunner.Counter;
                                        for (let mut index140: i32 =  0; index140 <= counter12; index140 += 1)
                                        {
                                          if (arySrc[index137].originRunner.Id[index140] == simpleListArray[coordinate10.x, coordinate10.y].Id[index139])
                                            num117 += 1;
                                        }
                                        if (num117 > 0)
                                          num113 += 1;
                                      }
                                      let mut counter13: i32 =  arySrc[index137].originRunner.Counter;
                                      for (index50 = 0; index50 <= counter13; index50 += 1)
                                      {
                                        let mut num118: i32 =  0;
                                        num116 += 1;
                                        let mut counter14: i32 =  simpleListArray[coordinate10.x, coordinate10.y].Counter;
                                        for (let mut index141: i32 =  0; index141 <= counter14; index141 += 1)
                                        {
                                          if (arySrc[index137].originRunner.Id[index50] == simpleListArray[coordinate10.x, coordinate10.y].Id[index141])
                                            num118 += 1;
                                        }
                                        if (num118 > 0)
                                          num115 += 1;
                                      }
                                      if (num113 == arySrc[index137].originRunner.Counter + 1)
                                        flag18 = false;
                                      if (num115 == num116)
                                        flag18 = false;
                                      if (num113 == num114)
                                        flag18 = false;
                                      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                      {
                                        if (arySrc[index137].originRunner.Counter > 11)
                                          flag18 = false;
                                      }
                                      else if (arySrc[index137].originRunner.Counter > 11)
                                        flag18 = false;
                                    }
                                    if (flag18)
                                      simpleList.Add(tid5, 10000, tdata3: coordinate11.x);
                                  }
                                }
                              }
                              tid5 += 1;
                            }
                            while (tid5 <= 5);
                            simpleList.ReverseSort();
                            bool flag19 = false;
                            let mut num119: i32 =  0;
                            if ( Math.Round(Conversion.Val(this.game.Data.Designer)) >= 42 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI & !this.game.Data.RegimeObj[this.game.Data.Turn].minimumDataUsage)
                            {
                              flag19 = true;
                              num119 = 1;
                            }
                            let mut num120: i32 =  0;
                            let mut num121: i32 =  0;
                            let mut num122: i32 =  0;
                            let mut num123: i32 =  0;
                            if (flag19)
                            {
                              let mut counter: i32 =  simpleList.Counter;
                              for (let mut index142: i32 =  0; index142 <= counter; index142 += 1)
                              {
                                if (simpleList.Weight[0] == simpleList.Weight[index142])
                                {
                                  num123 += 1;
                                  if (numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index142]] > 0)
                                  {
                                    num122 += 1;
                                    num120 += numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index142]];
                                    simpleList.Data4[index142] = numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index142]];
                                  }
                                }
                              }
                            }
                            if (num120 < 1 | num123 < 2)
                              num119 = 0;
                            for (let mut index143: i32 =  num119; index143 >= 0; index143 += -1)
                            {
                              let mut index144: i32 =  index116 - 1;
                              if (index143 == 1)
                                index144 = 3;
                              let mut num124: i32 =  0;
                              let mut num125: i32 =  0;
                              for (let mut counter: i32 =  simpleList.Counter; counter >= 0; counter += -1)
                              {
                                if (simpleList.Weight[0] == simpleList.Weight[counter])
                                {
                                  if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] < 7)
                                  {
                                    num124 += 1;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 0)
                                      index50 = 100;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 1)
                                      index50 = 80;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 2)
                                      index50 = 60;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 3)
                                      index50 = 40;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 4)
                                      index50 = 20;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 5)
                                      index50 = 10;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 6)
                                      index50 = 5;
                                    if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[counter], index144] == 7)
                                      index50 = 0;
                                    num125 += index50;
                                    simpleList.Data5[counter] = index50;
                                  }
                                  else
                                    simpleList.RemoveNr(counter);
                                }
                              }
                              if (index50 < 100 & num124 == 1 & simpleList.Counter > 0 && simpleList.Weight[1] > 0)
                              {
                                simpleList.Weight[1] = simpleList.Weight[0];
                                let mut index145: i32 =  1;
                                num124 += 1;
                                if (numArray2[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index145], index144] < 7)
                                {
                                  num125 += index50;
                                  index50 = 100 - index50;
                                  simpleList.Data5[index145] = index50;
                                }
                              }
                              if (num124 > 0)
                              {
                                simpleList.SortOnData5();
                                let mut num126: i32 =  simpleList.Counter <= 0 ? simpleList.Data5[simpleList.Counter] : simpleList.Data5[simpleList.Counter - 1];
                                if (index143 == 0)
                                  index137 = index137;
                                let mut num127: i32 =  num124 - 1;
                                for (let mut index146: i32 =  0; index146 <= num127; index146 += 1)
                                {
                                  tid4 += 1;
                                  if (tid4 > arySrc.GetUpperBound(0) - 10)
                                    arySrc = (LisRunner[]) Utils.CopyArray((Array) arySrc, (Array) new LisRunner[tid4 + 5000 + 1]);
                                  arySrc[tid4].splitJustDone = false;
                                  if (arySrc[index137].x == 153 & arySrc[index137].y == 41 & index116 == 1)
                                    index50 = index50;
                                  let mut num128: i32 =  0;
                                  let mut num129: i32 =  simpleList.Data4[index146];
                                  switch (index143)
                                  {
                                    case 0:
                                      index117 =  Math.Round(Math.Floor( Math.Max(0, arySrc[index137].lisPoints - num121) * ( simpleList.Data5[index146] /  num125)));
                                      break;
                                    case 1:
                                      index117 =  Math.Round(Math.Floor( arySrc[index137].lisPoints * ( simpleList.Data4[index146] /  num120)));
                                      if (index117 > 0)
                                      {
                                        let mut num130: i32 =  index117;
                                        let mut num131: i32 =  index117;
                                        float num132 = 1f;
                                        num133: i32;
                                        if (arySrc[index137].apUsed + simpleList.Data3[index146] > num83)
                                        {
                                          num132 =  (num83 - (arySrc[index137].apUsed + simpleList.Data3[index146] - num83)) /  num83;
                                          num131 =  Math.Round(Math.Ceiling( num130 *  num132));
                                          num133 =  Math.Round(Math.Floor( num130 *  num132));
                                        }
                                        else
                                          num133 = num130;
                                        if (num133 > numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index146]])
                                          num133 = numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index146]];
                                        if (num131 > numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index146]])
                                          num131 = numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index146]];
                                        num134: i32;
                                        if ( num132 > 0.0 &  num132 < 1.0)
                                        {
                                          let mut num135: i32 =  Math.Min(index117,  Math.Round(Math.Ceiling( (num131 * 1) /  num132)));
                                          num121 += num135;
                                          num134 = num135;
                                        }
                                        else
                                        {
                                          num121 += num133;
                                          num134 = num133;
                                        }
                                        num128 = num133;
                                        arySrc[tid4].splitJustDone = true;
                                        int[,,] numArray41 = numArray3;
                                        int[,,] numArray42 = numArray41;
                                        index119 = arySrc[index137].x;
                                        let mut index147: i32 =  index119;
                                        let mut y12: i32 =  arySrc[index137].y;
                                        let mut index148: i32 =  y12;
                                        int[] id = simpleList.Id;
                                        int[] numArray43 = id;
                                        let mut index149: i32 =  index146;
                                        let mut index150: i32 =  index149;
                                        let mut index151: i32 =  numArray43[index150];
                                        let mut num136: i32 =  numArray41[index119, y12, id[index149]] - num133;
                                        numArray42[index147, index148, index151] = num136;
                                        if (0 > numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index146]])
                                          numArray3[arySrc[index137].x, arySrc[index137].y, simpleList.Id[index146]] = 0;
                                        if (index117 > num134)
                                        {
                                          index117 = num134;
                                          break;
                                        }
                                        break;
                                      }
                                      break;
                                  }
                                  if (index143 == 1 & num122 <= 1 & num126 > 0)
                                    num126 = 0;
                                  else if (index143 == 0 & num124 <= 1 & num126 > 0)
                                    num126 = 0;
                                  else if (index143 == 0 & num126 > 0)
                                    index137 = index137;
                                  if (flag12)
                                  {
                                    flag12 = false;
                                    num126 = 100;
                                  }
                                  arySrc[tid4].lisPoints = index117;
                                  arySrc[tid4].x = arySrc[index137].x;
                                  arySrc[tid4].y = arySrc[index137].y;
                                  arySrc[tid4].apUsed = arySrc[index137].apUsed;
                                  arySrc[tid4].reFocusCount = arySrc[index137].reFocusCount;
                                  arySrc[tid4].branchCount = arySrc[index137].branchCount + num126;
                                  arySrc[tid4].extensionAllowed = arySrc[index137].extensionAllowed;
                                  arySrc[tid4].extensionUsed = arySrc[index137].extensionUsed;
                                  arySrc[tid4].active = true;
                                  arySrc[tid4].isPull = index143 == 1;
                                  arySrc[tid4].awaitingSplit = false;
                                  arySrc[tid4].direction = simpleList.Id[index146];
                                  arySrc[tid4].originRunner = SimpleList::new();
                                  arySrc[tid4].originRunner.AddWeightBlind( arySrc[index137].originRunner);
                                  arySrc[tid4].originRunner.AddBlind(tid4, 1);
                                  if (arySrc[tid4].branchCount > 500 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                  {
                                    let mut num137: i32 =   Math.Round(Math.Ceiling( (arySrc[tid4].branchCount - 500) / 100.0));
                                    let mut num138: i32 =  num137 * num137 * 10;
                                    if (num138 > 999)
                                      num138 = 999;
                                    let mut num139: i32 =  arySrc[tid4].extensionAllowed - arySrc[tid4].extensionUsed;
                                    if (num139 >= num138)
                                    {
                                      LisRunner[] lisRunnerArray7 = arySrc;
                                      LisRunner[] lisRunnerArray8 = lisRunnerArray7;
                                      index119 = tid4;
                                      let mut index152: i32 =  index119;
                                      lisRunnerArray8[index152].extensionUsed = lisRunnerArray7[index119].extensionUsed + num138;
                                    }
                                    else
                                    {
                                      num138 -= num139;
                                      arySrc[tid4].extensionUsed = arySrc[tid4].extensionAllowed;
                                      LisRunner[] lisRunnerArray9 = arySrc;
                                      LisRunner[] lisRunnerArray10 = lisRunnerArray9;
                                      index119 = tid4;
                                      let mut index153: i32 =  index119;
                                      lisRunnerArray10[index153].apUsed = lisRunnerArray9[index119].apUsed + num138;
                                    }
                                    if (index146 == 0 && index1 > -1 & num124 > 1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                                    {
                                      s4 = Math.Round( Math.Max(0, arySrc[tid4].branchCount - 100) / 100.0, 2).ToString() + "th branching causes +" + num138.ToString() + " AP cost.";
                                      this.game.Data.StringListObj[index1].AddRowWithData(arySrc[tid4].x.ToString(), arySrc[tid4].y.ToString(), "3", "6", s4, x.ToString(), y10.ToString(), index116.ToString());
                                    }
                                  }
                                  if ((0 | (index143 == 1 ? 1 : 0)) != 0 && index1 > -1 & num124 > 1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI && arySrc[tid4].lisPoints > 0)
                                  {
                                    index117 = num128;
                                    s4 = "Pull (" + num129.ToString() + ") Prioritized " + index117.ToString() + " " + str13 + " points" + " to ";
                                    if (simpleList.Id[index146] == 0)
                                      s4 += "North.";
                                    if (simpleList.Id[index146] == 1)
                                      s4 += "North-East.";
                                    if (simpleList.Id[index146] == 2)
                                      s4 += "South-East.";
                                    if (simpleList.Id[index146] == 3)
                                      s4 += "South.";
                                    if (simpleList.Id[index146] == 4)
                                      s4 += "South-West.";
                                    if (simpleList.Id[index146] == 5)
                                      s4 += "North-West.";
                                    this.game.Data.StringListObj[index1].AddRowWithData(arySrc[index137].x.ToString(), arySrc[index137].y.ToString(), "3", simpleList.Id[index146].ToString(), s4, "-1", "-1", "0");
                                  }
                                  index117 = index117;
                                }
                              }
                            }
                          }
                        }
                        index117 = 9999999;
                        let mut num140: i32 =  num87;
                        let mut num141: i32 =  tid4;
                        for (let mut index154: i32 =  num140; index154 <= num141; index154 += 1)
                        {
                          if ((arySrc[index154].active | arySrc[index154].awaitingSplit) & index117 > index154)
                            index117 = index154;
                        }
                        num87 = index117;
                      }
                    }
                  }
                }
              }
            }
          }
        }
        num80 += 1;
      }
      while (num80 <= 1);
      let mut mapWidth20: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut index155: i32 =  0; index155 <= mapWidth20; index155 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (index49 = 0; index49 <= mapHeight; index49 += 1)
        {
          if (this.game.Data.MapObj[0].HexObj[index155, index49].LISpoints[6] > 0)
          {
            let mut num142: i32 =  0;
            do
            {
              coordinate1 = this.game.HandyFunctionsObj.HexNeighbour(index155, index49, 0, num142 + 1);
              if (coordinate1.onmap && this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[6] > 0)
              {
                let mut index156: i32 =  this.game.HandyFunctionsObj.HexFacing(index155, index49, 0, coordinate1.x, coordinate1.y, 0) - 1;
                let mut index157: i32 =  index156 + 3;
                if (index157 > 5)
                  index157 -= 6;
                if (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].LISpoints[index157] == 0 & this.game.Data.MapObj[0].HexObj[index155, index49].LISpoints[index156] == 0)
                {
                  let mut num143: i32 =  this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RoadType[index157] > -1 & this.game.Data.MapObj[0].HexObj[index155, index49].RoadType[index156] > -1 ? 1 : 0;
                }
              }
              num142 += 1;
            }
            while (num142 <= 5);
          }
        }
      }
      if (this.game.EventRelatedObj.Helper_AirEnabled() && !isPreview & !isCalibrationCall)
        this.LIS_AddAirBridges(isPreview);
      if (!isCalibrationCall)
      {
        let mut mapWidth21: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index158: i32 =  0; index158 <= mapWidth21; index158 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (index49 = 0; index49 <= mapHeight; index49 += 1)
          {
            let mut index159: i32 =  0;
            do
            {
              this.game.Data.MapObj[0].HexObj[index158, index49].LIStotalHistory[index159] = this.game.Data.MapObj[0].HexObj[index158, index49].LISpoints[index159];
              index159 += 1;
            }
            while (index159 <= 8);
          }
        }
      }
      if (!isCalibrationCall && isPreview)
      {
        let mut mapWidth22: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index160: i32 =  0; index160 <= mapWidth22; index160 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (index49 = 0; index49 <= mapHeight; index49 += 1)
          {
            let mut index161: i32 =  0;
            do
            {
              if (onlyForAssetID > 0)
              {
                if (index160 == 6 & index49 == 39 & index161 == 5)
                  index161 = index161;
                if (num5 > 0)
                {
                  this.game.Data.MapObj[0].HexObj[index160, index49].tempPreviewAssetLIS[index161] = this.game.Data.MapObj[0].HexObj[index160, index49].tempPreviewLIS[index161] - this.game.Data.MapObj[0].HexObj[index160, index49].LISpoints[index161];
                  if (this.game.Data.MapObj[0].HexObj[index160, index49].tempPreviewAssetLIS[index161] < 0)
                    this.game.Data.MapObj[0].HexObj[index160, index49].tempPreviewAssetLIS[index161] = 0;
                }
                else
                  this.game.Data.MapObj[0].HexObj[index160, index49].tempPreviewAssetLIS[index161] = this.game.Data.MapObj[0].HexObj[index160, index49].LISpoints[index161];
              }
              else
              {
                this.game.Data.MapObj[0].HexObj[index160, index49].tempPreviewLIS[index161] = this.game.Data.MapObj[0].HexObj[index160, index49].LISpoints[index161];
                this.game.Data.MapObj[0].HexObj[index160, index49].tempPreviewPull[index161] = this.game.Data.MapObj[0].HexObj[index160, index49].LISpull[index161];
              }
              this.game.Data.MapObj[0].HexObj[index160, index49].LIShistory[index161] = this.cacheLIShistory[index160, index49, index161];
              this.game.Data.MapObj[0].HexObj[index160, index49].LISpoints[index161] = this.cacheLISpoints[index160, index49, index161];
              this.game.Data.MapObj[0].HexObj[index160, index49].LIStotalHistory[index161] = this.cacheLIStotalHistory[index160, index49, index161];
              this.game.Data.MapObj[0].HexObj[index160, index49].LISorganic[index161] = this.cacheLISorganic[index160, index49, index161];
              this.game.Data.MapObj[0].HexObj[index160, index49].LISorganicPercentage[index161] = this.cacheLISorganicPercentage[index160, index49, index161];
              this.game.Data.MapObj[0].HexObj[index160, index49].LISpull[index161] = this.cacheLISpull[index160, index49, index161];
              index161 += 1;
            }
            while (index161 <= 8);
          }
        }
        this.tempLISwithoutLogExt = new int[2, 2, 2];
        this.cacheLIShistory = new int[2, 2, 2];
        this.cacheLISpoints = new int[2, 2, 2];
        this.cacheLIStotalHistory = new int[2, 2, 2];
        this.cacheLISorganic = new int[2, 2, 2];
        this.cacheLISorganicPercentage = new int[2, 2, 2];
        this.cacheLISpull = new int[2, 2, 2];
      }
      if (isPreview & !this.game.Data.RegimeObj[this.game.Data.Turn].AI & this.game.Data.InTurn)
        this.game.EventRelatedObj.ExecSuperImposeMessage("", "", 0, 0, "");
      let mut mapWidth23: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut index162: i32 =  0; index162 <= mapWidth23; index162 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (index49 = 0; index49 <= mapHeight; index49 += 1)
        {
          this.game.EditObj.TempValue[0].Value[index162, index49] = Conversions.ToInteger(objArray1[index162, index49]);
          Coordinate[,] coordinateArray = this.game.EditObj.TempCameFrom[0].Value;
          let mut index163: i32 =  index162;
          let mut index164: i32 =  index49;
          object obj = objArray2[index162, index49];
          Coordinate coordinate12;
          Coordinate coordinate13 = obj != null ? (Coordinate) obj : coordinate12;
          coordinateArray[index163, index164] = coordinate13;
        }
      }
    }

    pub fn LIS_AddAirBridges(bool preview)
    {
      let mut stringListById1: i32 =  this.game.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 534, 0, 0));
      let mut stringListById2: i32 =  this.game.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 535, 0, 0));
      let mut stringListById3: i32 =  this.game.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 363, 0, 0));
      if (stringListById1 < 1)
        return;
      let mut id: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].id;
      for (let mut length: i32 =  this.game.Data.StringListObj[stringListById2].Length; length >= 0; length += -1)
      {
        let mut regimeById: i32 =  this.game.HandyFunctionsObj.GetRegimeByID( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById2].Data[length, 0])));
        if (regimeById == -1)
          this.game.Data.StringListObj[stringListById2].RemoveRow(length);
        else if (this.game.Data.RegimeObj[regimeById].AI | regimeById == this.game.Data.Turn)
          this.game.Data.StringListObj[stringListById2].RemoveRow(length);
      }
      SimpleList simpleList1 = SimpleList::new();
      let mut unitCounter1: i32 =  this.game.Data.UnitCounter;
      for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
      {
        if (this.game.Data.UnitObj[index].Regime == this.game.Data.Turn)
        {
          let mut historical: i32 =  this.game.Data.UnitObj[index].Historical;
          if (historical > -1 && this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(55) > 0)
          {
            let mut x: i32 =  this.game.Data.UnitObj[index].X;
            let mut y: i32 =  this.game.Data.UnitObj[index].Y;
            let mut tdata3: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(56);
            let mut tdata4: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(57);
            simpleList1.Add(this.game.Data.HistoricalUnitObj[historical].ID, 1, x, y, tdata3, tdata4);
          }
        }
      }
      for (let mut length: i32 =  this.game.Data.StringListObj[stringListById1].Length; length >= 0; length += -1)
      {
        if ( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 0])) == id)
        {
          let mut tdata1: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 1]));
          let mut tdata2: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 2]));
          let mut tdata3: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 3]));
          let mut tdata4: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 4]));
          let mut num1: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 9]));
          if (simpleList1.FindNr(-1, tdata1, tdata2, tdata3, tdata4) <= -1)
            num1 = -1;
          if (num1 == -1)
          {
            this.game.Data.StringListObj[stringListById1].RemoveRow(length);
            let mut unitCounter2: i32 =  this.game.Data.UnitCounter;
            for (let mut index: i32 =  0; index <= unitCounter2; index += 1)
            {
              if (this.game.Data.UnitObj[index].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index].PreDef == -1)
              {
                let mut historical: i32 =  this.game.Data.UnitObj[index].Historical;
                if (this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(55) > 0)
                {
                  let mut x: i32 =  this.game.Data.UnitObj[index].X;
                  let mut y: i32 =  this.game.Data.UnitObj[index].Y;
                  let mut num2: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(56);
                  let mut num3: i32 =  this.game.Data.HistoricalUnitObj[historical].GiveHisVarValue(57);
                  if (x == tdata1 & y == tdata2 & num2 == tdata3 & num3 == tdata4)
                  {
                    this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(55, 0);
                    this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(56, 0);
                    this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(57, 0);
                    this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(58, 0);
                    this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(59, 0);
                  }
                }
              }
            }
          }
        }
        else if (this.game.HandyFunctionsObj.GetRegimeByID( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 0]))) == -1)
          this.game.Data.StringListObj[stringListById1].RemoveRow(length);
      }
      for (let mut length: i32 =  this.game.Data.StringListObj[stringListById1].Length; length >= 0; length += -1)
      {
        if ( Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 0])) == id)
        {
          let mut x1: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 1]));
          let mut y1: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 2]));
          let mut x2: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 3]));
          let mut y2: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 4]));
          let mut num4: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 8]));
          let mut num5: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById1].Data[length, 9]));
          this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), "Attempting to execute Air Bridge");
          SimpleList simpleList2 = SimpleList::new();
          let mut counter1: i32 =  simpleList1.Counter;
          for (let mut index: i32 =  0; index <= counter1; index += 1)
          {
            if (simpleList1.Data1[index] == x1 & simpleList1.Data2[index] == y1 & simpleList1.Data3[index] == x2 & simpleList1.Data4[index] == y2)
            {
              let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(simpleList1.Id[index]);
              if (historicalUnitById > -1)
              {
                let mut unitByHistorical: i32 =  this.game.HandyFunctionsObj.GetUnitByHistorical(historicalUnitById);
                let mut lowestAirRdn: i32 =  this.game.HandyFunctionsObj.GetLowestAirRdn(unitByHistorical);
                let mut lowestAirApRule: i32 =  this.game.HandyFunctionsObj.GetLowestAirApRule(unitByHistorical);
                let mut tdata1: i32 =  this.game.HandyFunctionsObj.GetLowestAirAp(unitByHistorical);
                let mut minimumAirfieldLevel: i32 =  this.game.HandyFunctionsObj.SE1_GetUnitMinimumAirfieldLevel(unitByHistorical);
                let mut num6: i32 =  0;
                let mut num7: i32 =  0;
                if (this.game.Data.MapObj[0].HexObj[x1, y1].Location > -1)
                  num6 = this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[x1, y1].Location].tempAirfieldLevel;
                if (this.game.Data.MapObj[0].HexObj[x2, y2].Location > -1)
                  num7 = this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[x2, y2].Location].tempAirfieldLevel;
                if (tdata1 > lowestAirRdn)
                  tdata1 = lowestAirRdn;
                let mut num8: i32 =   Math.Round(Math.Floor( tdata1 /  lowestAirApRule));
                let mut num9: i32 =  this.game.HandyFunctionsObj.Distance(x1, y1, 0, x2, y2, 0);
                if (minimumAirfieldLevel > num6)
                  this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), this.game.Data.UnitObj[unitByHistorical].Name + " source Hex Airbase Level was not high enough.", "Current Level is " + num6.ToString() + " instead of " + minimumAirfieldLevel.ToString() + ".");
                else if (minimumAirfieldLevel > num7)
                  this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), this.game.Data.UnitObj[unitByHistorical].Name + " target Hex Airbase Level was not high enough.", "Current Level is " + num7.ToString() + " instead of " + minimumAirfieldLevel.ToString() + ".");
                else if (num8 >= num9)
                {
                  tdata1 = this.game.HandyFunctionsObj.GetLowestAirAp(unitByHistorical);
                  this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), this.game.Data.UnitObj[unitByHistorical].Name + " is ready (ap=" + tdata1.ToString() + ",rdn=" + lowestAirRdn.ToString() + ") and joins mission.");
                  simpleList2.Add(this.game.Data.HistoricalUnitObj[historicalUnitById].ID, 1, tdata1, unitByHistorical);
                }
                else
                {
                  tdata1 = this.game.HandyFunctionsObj.GetLowestAirAp(unitByHistorical);
                  if (lowestAirRdn < tdata1)
                    this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), this.game.Data.UnitObj[unitByHistorical].Name + " did not have enough Readiness (" + lowestAirRdn.ToString() + ") to join mission.", "Current maximum range is " + num8.ToString() + " instead of " + num9.ToString() + ".");
                  else
                    this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), this.game.Data.UnitObj[unitByHistorical].Name + " did not have enough Action Points (" + tdata1.ToString() + ") to join mission.", "Current maximum range is " + num8.ToString() + " instead of " + num9.ToString() + ".");
                }
              }
            }
          }
          if (simpleList2.Counter > -1)
          {
            this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), "Executing Air Bridge");
            this.game.TempCombat = new CombatClass(this.game);
            Coordinate Target = Coordinate::new();
            Target.x = x2;
            Target.y = y2;
            this.game.EditObj.TempUnitList = UnitList::new();
            let mut num10: i32 =  0;
            let mut counter2: i32 =  simpleList2.Counter;
            for (let mut index: i32 =  0; index <= counter2; index += 1)
            {
              let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(simpleList2.Id[index]);
              if (historicalUnitById > -1)
                this.game.EditObj.TempUnitList.add(this.game.HandyFunctionsObj.GetUnitByHistorical(historicalUnitById));
            }
            this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, 55, true);
            this.game.TempCombat.DoBattle();
            let mut se1carryPointsDelivered: i32 =  this.game.TempCombat.se1carryPointsDelivered;
            let mut se1damagePercentage: i32 =  this.game.TempCombat.se1damagePercentage;
            SimpleList simpleList3 = SimpleList::new();
            SimpleList simpleList4 = SimpleList::new();
            SimpleList simpleList5 = SimpleList::new();
            SimpleList simpleList6 = SimpleList::new();
            let mut num11: i32 =  0;
            let mut num12: i32 =  0;
            let mut num13: i32 =  0;
            let mut num14: i32 =  0;
            let mut counter3: i32 =  simpleList2.Counter;
            for (let mut index1: i32 =  0; index1 <= counter3; index1 += 1)
            {
              let mut icounter: i32 =  this.game.TempCombat.ICounter;
              for (let mut index2: i32 =  0; index2 <= icounter; index2 += 1)
              {
                if (this.game.TempCombat.IList[index2].IUnr == simpleList2.Data2[index1])
                {
                  num10 += this.game.Data.SFTypeObj[this.game.TempCombat.IList[index2].ISFType].CarryCap;
                  if (this.game.TempCombat.IList[index2].IKilled > 0)
                  {
                    num11 += this.game.Data.SFTypeObj[this.game.TempCombat.IList[index2].ISFType].Ratio;
                    simpleList4.AddWeight(this.game.TempCombat.IList[index2].ISFType, this.game.Data.SFTypeObj[this.game.TempCombat.IList[index2].ISFType].Ratio);
                  }
                  else if (this.game.TempCombat.IList[index2].IKilled == 0 & (this.game.TempCombat.IList[index2].IRetreatMode == 2 | this.game.TempCombat.IList[index2].IRetreatMode == 0))
                  {
                    num12 += this.game.Data.SFTypeObj[this.game.TempCombat.IList[index2].ISFType].Ratio;
                    simpleList6.AddWeight(this.game.TempCombat.IList[index2].ISFType, this.game.Data.SFTypeObj[this.game.TempCombat.IList[index2].ISFType].Ratio);
                  }
                  else
                  {
                    num13 += this.game.Data.SFTypeObj[this.game.TempCombat.IList[index2].ISFType].Ratio;
                    simpleList5.AddWeight(this.game.TempCombat.IList[index2].ISFType, this.game.Data.SFTypeObj[this.game.TempCombat.IList[index2].ISFType].Ratio);
                  }
                }
              }
            }
            let mut icounter1: i32 =  this.game.TempCombat.ICounter;
            for (let mut index: i32 =  0; index <= icounter1; index += 1)
            {
              if (this.game.TempCombat.IList[index].IAttacker == 0 && this.game.Data.SFTypeObj[this.game.TempCombat.IList[index].ISFType].Theater == 2)
              {
                num14 += this.game.Data.SFTypeObj[this.game.TempCombat.IList[index].ISFType].Ratio;
                simpleList3.AddWeight(this.game.TempCombat.IList[index].ISFType, this.game.Data.SFTypeObj[this.game.TempCombat.IList[index].ISFType].Ratio);
              }
            }
            if (num14 > 0)
            {
              s2: String = "Our mission was intercepted by " + num14.ToString() + " enemy aircraft.";
              s3: String = "";
              let mut counter4: i32 =  simpleList3.Counter;
              for (let mut index: i32 =  0; index <= counter4; index += 1)
              {
                if (simpleList3.Weight[index] > 0)
                {
                  if (s3.Length > 0)
                    s3 += "\r\n";
                  s3 = s3 + simpleList3.Weight[index].ToString() + "x " + this.game.Data.SFTypeObj[simpleList3.Id[index]].Name + " were intercepting.";
                }
              }
              this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), s2, s3);
            }
            s2_1: String;
            if (num12 > 0)
              s2_1 = num12.ToString() + " aircraft established Air Bridge. " + num13.ToString() + " air retreated. " + num11.ToString() + " air lost.";
            else
              s2_1 = "Failure to establish Air Bridge. " + num13.ToString() + " air retreated. " + num11.ToString() + " air lost.";
            s3_1: String = "";
            let mut counter5: i32 =  simpleList4.Counter;
            for (let mut index: i32 =  0; index <= counter5; index += 1)
            {
              if (simpleList4.Weight[index] > 0)
              {
                if (s3_1.Length > 0)
                  s3_1 += "\r\n";
                s3_1 = s3_1 + simpleList4.Weight[index].ToString() + "x " + this.game.Data.SFTypeObj[simpleList4.Id[index]].Name + " were killed in action.";
              }
            }
            this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), s2_1, s3_1);
            this.game.TempCombat.EndBattle();
            let mut counter6: i32 =  simpleList2.Counter;
            for (let mut index: i32 =  0; index <= counter6; index += 1)
            {
              let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(simpleList2.Id[index]);
              if (historicalUnitById > -1 && this.game.HandyFunctionsObj.GetUnitByHistorical(historicalUnitById) > -1)
                this.game.EventRelatedObj.Helper_MakeListForUnitRequests("SE_Data", this.game.Data.HistoricalUnitObj[historicalUnitById].ID, false);
            }
            let mut num15: i32 =  0;
            let mut num16: i32 =  0;
            num17: i32;
            if ( Math.Round(Conversion.Val(this.game.Data.Designer)) >= 82)
            {
              let mut counter7: i32 =  simpleList2.Counter;
              for (let mut index3: i32 =  0; index3 <= counter7; index3 += 1)
              {
                let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(simpleList2.Id[index3]);
                if (historicalUnitById > -1)
                {
                  let mut unitByHistorical: i32 =  this.game.HandyFunctionsObj.GetUnitByHistorical(historicalUnitById);
                  if (unitByHistorical > -1)
                  {
                    let mut sfCount: i32 =  this.game.Data.UnitObj[unitByHistorical].SFCount;
                    for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
                    {
                      let mut sf: i32 =  this.game.Data.UnitObj[unitByHistorical].SFList[index4];
                      let mut idValue: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].SFTypeVar[81];
                      if (idValue > 0)
                      {
                        let mut num18: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById3].GetData2(0, idValue, 1, 503, 2)));
                        num16 += this.game.Data.SFObj[sf].Qty;
                        num15 += this.game.Data.SFObj[sf].Qty * num18;
                      }
                    }
                  }
                }
              }
              num17 =  Math.Round(Math.Ceiling(Math.Sqrt(Math.Ceiling(  Math.Round(Math.Floor( num15 /  num16)) / 1000.0))));
              if (num17 < 1)
                num17 = 1;
            }
            else
            {
              let mut counter8: i32 =  simpleList2.Counter;
              for (let mut index5: i32 =  0; index5 <= counter8; index5 += 1)
              {
                let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(simpleList2.Id[index5]);
                if (historicalUnitById > -1)
                {
                  let mut unitByHistorical: i32 =  this.game.HandyFunctionsObj.GetUnitByHistorical(historicalUnitById);
                  if (unitByHistorical > -1)
                  {
                    let mut sfCount: i32 =  this.game.Data.UnitObj[unitByHistorical].SFCount;
                    for (let mut index6: i32 =  0; index6 <= sfCount; index6 += 1)
                    {
                      let mut sf: i32 =  this.game.Data.UnitObj[unitByHistorical].SFList[index6];
                      let mut num19: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].SFTypeVar[22];
                      num16 += this.game.Data.SFObj[sf].Qty;
                      num15 += this.game.Data.SFObj[sf].Qty * num19;
                    }
                  }
                }
              }
              num17 =  Math.Round(Math.Floor( num15 /  num16));
              if (num17 < 1)
                num17 = 1;
            }
            this.game.Data.StringListObj[stringListById1].Data[length, 5] = se1carryPointsDelivered.ToString();
            this.game.Data.StringListObj[stringListById1].Data[length, 6] = se1carryPointsDelivered.ToString();
            this.game.Data.StringListObj[stringListById1].Data[length, 7] = se1damagePercentage.ToString();
            this.game.Data.StringListObj[stringListById1].Data[length, 9] = "1";
            this.game.Data.StringListObj[stringListById1].Data[length, 10] = num17.ToString();
            this.game.Data.StringListObj[stringListById1].Data[length, 11] = num10.ToString();
            int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[x1, y1].LISpoints;
            int[] numArray1 = liSpoints1;
            let mut index7: i32 =  6;
            let mut index8: i32 =  index7;
            let mut num20: i32 =  liSpoints1[index7] + se1carryPointsDelivered;
            numArray1[index8] = num20;
            int[] liSpoints2 = this.game.Data.MapObj[0].HexObj[x2, y2].LISpoints;
            int[] numArray2 = liSpoints2;
            let mut index9: i32 =  6;
            let mut index10: i32 =  index9;
            let mut num21: i32 =  liSpoints2[index9] + se1carryPointsDelivered;
            numArray2[index10] = num21;
            if (se1carryPointsDelivered > 0)
            {
              s2_2: String = "Air Bridge established with " + se1carryPointsDelivered.ToString() + " Air Points.";
              s3_2: String = "Maximum Size for Air Bridge usage is " + num17.ToString() + ".\r\nDamage that will be taken by usage is " + se1damagePercentage.ToString() + "%.";
              this.game.Data.StringListObj[stringListById2].AddRowWithData(id.ToString(), num4.ToString(), s2_2, s3_2);
            }
          }
        }
      }
    }

    pub fn LIS_SetNetwork_BACKUP(bool isCalibrationCall)
    {
      let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[405]));
      this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[404]));
      if (isCalibrationCall)
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            let mut index3: i32 =  0;
            do
            {
              this.game.Data.MapObj[0].HexObj[index1, index2].tempOldLISpoints[index3] = this.game.Data.MapObj[0].HexObj[index1, index2].LISpoints[index3];
              this.game.Data.MapObj[0].HexObj[index1, index2].LISpoints[index3] = 0;
              index3 += 1;
            }
            while (index3 <= 8);
          }
        }
      }
      else
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
        for (let mut index4: i32 =  0; index4 <= mapWidth; index4 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
          for (let mut index5: i32 =  0; index5 <= mapHeight; index5 += 1)
          {
            let mut index6: i32 =  0;
            do
            {
              this.game.Data.MapObj[0].HexObj[index4, index5].tempOldLISpoints[index6] = 0;
              this.game.Data.MapObj[0].HexObj[index4, index5].LIShistory[index6] = 0;
              this.game.Data.MapObj[0].HexObj[index4, index5].LISpoints[index6] = 0;
              this.game.Data.MapObj[0].HexObj[index4, index5].LIStotalHistory[index6] = 0;
              this.game.Data.MapObj[0].HexObj[index4, index5].LISorganic[index6] = 0;
              this.game.Data.MapObj[0].HexObj[index4, index5].LISorganicPercentage[index6] = 0;
              index6 += 1;
            }
            while (index6 <= 8);
          }
        }
      }
      let mut locCounter: i32 =  this.game.Data.LocCounter;
      for (let mut index7: i32 =  0; index7 <= locCounter; index7 += 1)
      {
        let mut x1: i32 =  this.game.Data.LocObj[index7].X;
        let mut y1: i32 =  this.game.Data.LocObj[index7].Y;
        bool flag = false;
        if (this.game.Data.MapObj[0].HexObj[x1, y1].Regime == this.game.Data.Turn && !Information.IsNothing( this.game.Data.LocObj[index7].tempLIS) && this.game.Data.LocObj[index7].tempLIS.Counter > -1)
          flag = true;
        if (flag)
        {
          let mut num1: i32 =  100;
          if (isCalibrationCall)
          {
            let mut num2: i32 =  0;
            let mut length: i32 =  this.game.Data.StringListObj[stringListById].Length;
            for (let mut index8: i32 =  0; index8 <= length; index8 += 1)
            {
              let mut tid: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index8, 0]));
              let mut num3: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index8, 4]));
              let mut num4: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index8, 2]));
              let mut nr1: i32 =  this.game.Data.LocObj[index7].tempLIS.FindNr(tid);
              let mut num5: i32 =  0;
              if (nr1 > -1)
                num5 = this.game.Data.LocObj[index7].tempLIS.Weight[nr1];
              if (num5 > 0)
              {
                let mut nr2: i32 =  this.game.Data.LocObj[index7].tempLISfreeAP.FindNr(tid);
                let mut num6: i32 =  0;
                if (nr2 > -1)
                  num6 = this.game.Data.LocObj[index7].tempLISfreeAP.Weight[nr2];
                if (num6 > 0)
                  num2 += num5;
              }
            }
            if (num2 > 0)
              num1 =  Math.Round(Math.Floor( (this.game.Data.MapObj[0].HexObj[this.game.Data.LocObj[index7].X, this.game.Data.LocObj[index7].Y].tempOldLISpoints[6] * 100) /  num2));
          }
          let mut length1: i32 =  this.game.Data.StringListObj[stringListById].Length;
          for (let mut index9: i32 =  0; index9 <= length1; index9 += 1)
          {
            let mut tid: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index9, 0]));
            let mut movetype: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index9, 4]));
            let mut theater: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].Data[index9, 2]));
            let mut nr3: i32 =  this.game.Data.LocObj[index7].tempLIS.FindNr(tid);
            let mut num7: i32 =  0;
            if (nr3 > -1)
              num7 =  Math.Round(Math.Floor( (this.game.Data.LocObj[index7].tempLIS.Weight[nr3] * num1) / 100.0));
            if (num7 > 0)
            {
              let mut nr4: i32 =  this.game.Data.LocObj[index7].tempLISfreeAP.FindNr(tid);
              let mut num8: i32 =  0;
              if (nr4 > -1)
                num8 = this.game.Data.LocObj[index7].tempLISfreeAP.Weight[nr4];
              if (num8 > 0)
              {
                int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[x1, y1].LISpoints;
                int[] numArray1 = liSpoints1;
                let mut index10: i32 =  6;
                let mut index11: i32 =  index10;
                let mut num9: i32 =  liSpoints1[index10] + num7;
                numArray1[index11] = num9;
                CoordList coordList1 = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, theater, num8 * 2, x1, y1, 0, roadsOnly: true, alwaysUseLogisticalBonus: true);
                let mut counter1: i32 =  coordList1.counter;
                for (let mut index12: i32 =  0; index12 <= counter1; index12 += 1)
                {
                  if (coordList1.coord[index12].onmap)
                  {
                    let mut x2: i32 =  coordList1.coord[index12].x;
                    let mut y2: i32 =  coordList1.coord[index12].y;
                    if (this.game.EditObj.TempCameFrom[0].Value[x2, y2].onmap & this.game.EditObj.TempValue[0].Value[x2, y2] < 9999)
                    {
                      let mut x3: i32 =  this.game.EditObj.TempCameFrom[0].Value[x2, y2].x;
                      let mut y3: i32 =  this.game.EditObj.TempCameFrom[0].Value[x2, y2].y;
                      let mut num10: i32 =  this.game.HandyFunctionsObj.HexFacing(x3, y3, 0, x2, y2, 0) - 1;
                      let mut num11: i32 =  num10 + 3;
                      if (num11 > 5)
                        num11 -= 6;
                      let mut num12: i32 =  this.game.EditObj.TempValue[0].Value[x2, y2];
                      let mut num13: i32 =  num12 <= num8 ? num7 :  Math.Round( num7 * ( (num8 - (num12 - num8)) /  num8));
                      if (isCalibrationCall)
                      {
                        let mut pointsOnTrajectory: i32 =  this.LIS_GetLowestPotentialPointsOnTrajectory(x2, y2);
                        if (num13 > pointsOnTrajectory)
                          num13 = pointsOnTrajectory;
                      }
                      if (((!isCalibrationCall ? 1 : 0) & 0) != 0)
                      {
                        let mut num14: i32 =  0;
                        let mut num15: i32 =  0;
                        if (this.game.Data.Turn == 2)
                          num14 = num14;
                        Coordinate coordinate1;
                        coordinate1.onmap = true;
                        coordinate1.x = x3;
                        coordinate1.y = y3;
                        while (coordinate1.onmap)
                        {
                          let mut num16: i32 =  0;
                          let mut tfacing1: i32 =  1;
                          do
                          {
                            if (this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RoadType[tfacing1 - 1] > -1)
                            {
                              Coordinate coordinate2 = this.game.HandyFunctionsObj.HexNeighbour(coordinate1.x, coordinate1.y, 0, tfacing1);
                              if (coordinate2.onmap && this.game.EditObj.TempValue[0].Value[coordinate2.x, coordinate2.y] < 9999)
                              {
                                let mut index13: i32 =  tfacing1 - 1 + 3;
                                if (index13 > 5)
                                  index13 -= 6;
                                if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].RoadType[index13] > -1)
                                  num16 += 1;
                              }
                            }
                            tfacing1 += 1;
                          }
                          while (tfacing1 <= 6);
                          if (((!isCalibrationCall ? 1 : 0) & 0) != 0)
                          {
                            let mut num17: i32 =  0;
                            Coordinate coordinate3 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                            CoordList coordList2 = CoordList::new();
                            if (coordinate3.onmap)
                            {
                              let mut num18: i32 =  1;
                              Coordinate coordinate4;
                              do
                              {
                                coordinate4 = this.game.HandyFunctionsObj.HexNeighbour(coordinate1.x, coordinate1.y, 0, num18);
                                if (coordinate4.onmap && !(coordinate4.x == coordinate1.x & coordinate4.y == coordinate1.y) & !(coordinate4.x == coordinate3.x & coordinate4.y == coordinate3.y) && this.game.EditObj.TempCameFrom[0].Value[coordinate4.x, coordinate4.y].onmap & !(this.game.EditObj.TempCameFrom[0].Value[coordinate4.x, coordinate4.y].x == coordinate1.x & this.game.EditObj.TempCameFrom[0].Value[coordinate4.x, coordinate4.y].y == coordinate1.y))
                                {
                                  let mut num19: i32 =  num18 + 3;
                                  if (num19 > 6)
                                    num19 -= 6;
                                  let mut dat2: i32 =  0;
                                  if (this.game.Data.MapObj[0].HexObj[coordinate4.x, coordinate4.y].RoadType[num19 - 1] > -1 && this.game.Data.MapObj[0].HexObj[coordinate1.x, coordinate1.y].RoadType[num18 - 1] > -1)
                                  {
                                    num17 += 1;
                                    let mut tfacing2: i32 =  1;
                                    do
                                    {
                                      Coordinate coordinate5 = this.game.HandyFunctionsObj.HexNeighbour(coordinate4.x, coordinate4.y, 0, tfacing2);
                                      if (coordinate5.onmap)
                                      {
                                        let mut num20: i32 =  tfacing2 + 3;
                                        if (num20 > 6)
                                          num20 -= 6;
                                        if (this.game.Data.MapObj[0].HexObj[coordinate5.x, coordinate5.y].RoadType[num20 - 1] > -1 && this.game.Data.MapObj[0].HexObj[coordinate4.x, coordinate4.y].RoadType[tfacing2 - 1] > -1)
                                          dat2 += 1;
                                      }
                                      tfacing2 += 1;
                                    }
                                    while (tfacing2 <= 6);
                                    coordList2.AddCoord(coordinate4.x, coordinate4.y, 0, num18, dat2);
                                  }
                                }
                                num18 += 1;
                              }
                              while (num18 <= 6);
                              if (num17 > 0)
                              {
                                let mut counter2: i32 =  coordList2.counter;
                                for (let mut index14: i32 =  0; index14 <= counter2; index14 += 1)
                                {
                                  coordinate4.x = coordList2.coord[index14].x;
                                  coordinate4.y = coordList2.coord[index14].y;
                                  coordinate4.onmap = true;
                                  let mut data1: i32 =  coordList2.coord[index14].data1;
                                  let mut data2: i32 =  coordList2.coord[index14].data2;
                                  let mut num21: i32 =  data1 + 3;
                                  num22: i32;
                                  liSpoint: i32;
                                  if (num21 > 6)
                                  {
                                    let mut num23: i32 =  num21 - 6;
                                    num22 = this.game.EditObj.TempValue[0].Value[coordinate4.x, coordinate4.y];
                                    liSpoint = this.game.Data.MapObj[0].HexObj[coordinate4.x, coordinate4.y].LISpoints[6];
                                  }
                                  Coordinate coordinate6 = this.game.HandyFunctionsObj.MoveApCostPreview2(coordinate4.x, coordinate4.y, this.game.Data.Turn, movetype, theater, coordinate4.x, coordinate4.y, 0, coordinate1.x, coordinate1.y, 0, roadsOnly: true, alwaysUseLogisticalBonus: true);
                                  if (num22 + coordinate6.x <= num8 * 2)
                                  {
                                    let mut num24: i32 =   Math.Round(Math.Floor((num22 + coordinate6.x <= num8 ?  liSpoint :   Math.Round( liSpoint * ( (num8 - (num22 + coordinate6.x - num8)) /  num8))) /  (coordList2.counter + 1)));
                                    if (num24 > 0)
                                      num15 += num24;
                                  }
                                }
                              }
                            }
                          }
                          coordinate1 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                          if (coordinate1.onmap)
                          {
                            if (num16 > 2)
                              num14 += num16 - 2;
                          }
                          else if (num16 > 1)
                            num14 += num16 - 1;
                        }
                        if (num14 > 0)
                        {
                          let mut num25: i32 =  num14;
                          for (let mut index15: i32 =  1; index15 <= num25; index15 += 1)
                            num13 =  Math.Round(Math.Floor( num13 / 2.0));
                        }
                        num13 += num15;
                      }
                      int[] liSpoints2 = this.game.Data.MapObj[0].HexObj[x3, y3].LISpoints;
                      int[] numArray2 = liSpoints2;
                      let mut index16: i32 =  num10;
                      let mut index17: i32 =  index16;
                      let mut num26: i32 =  liSpoints2[index16] + num13;
                      numArray2[index17] = num26;
                      int[] liSpoints3 = this.game.Data.MapObj[0].HexObj[x2, y2].LISpoints;
                      int[] numArray3 = liSpoints3;
                      let mut index18: i32 =  num11;
                      let mut index19: i32 =  index18;
                      let mut num27: i32 =  liSpoints3[index18] + num13;
                      numArray3[index19] = num27;
                      int[] liSpoints4 = this.game.Data.MapObj[0].HexObj[x2, y2].LISpoints;
                      int[] numArray4 = liSpoints4;
                      let mut index20: i32 =  6;
                      let mut index21: i32 =  index20;
                      let mut num28: i32 =  liSpoints4[index20] + num13;
                      numArray4[index21] = num28;
                    }
                  }
                }
              }
            }
          }
        }
      }
      let mut mapWidth1: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut index22: i32 =  0; index22 <= mapWidth1; index22 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (let mut index23: i32 =  0; index23 <= mapHeight; index23 += 1)
        {
          if (this.game.Data.MapObj[0].HexObj[index22, index23].LISpoints[6] > 0)
          {
            let mut num29: i32 =  0;
            do
            {
              Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(index22, index23, 0, num29 + 1);
              if (coordinate.onmap && this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LISpoints[6] > 0)
              {
                let mut index24: i32 =  this.game.HandyFunctionsObj.HexFacing(index22, index23, 0, coordinate.x, coordinate.y, 0) - 1;
                let mut index25: i32 =  index24 + 3;
                if (index25 > 5)
                  index25 -= 6;
                if (this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LISpoints[index25] == 0 & this.game.Data.MapObj[0].HexObj[index22, index23].LISpoints[index24] == 0 && this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].RoadType[index25] > -1 & this.game.Data.MapObj[0].HexObj[index22, index23].RoadType[index24] > -1)
                {
                  let mut num30: i32 =  Math.Min(this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LISpoints[6], this.game.Data.MapObj[0].HexObj[index22, index23].LISpoints[6]);
                  this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].LISpoints[index25] = num30;
                  this.game.Data.MapObj[0].HexObj[index22, index23].LISpoints[index24] = num30;
                }
              }
              num29 += 1;
            }
            while (num29 <= 5);
          }
        }
      }
      if (isCalibrationCall)
        return;
      let mut mapWidth2: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut index26: i32 =  0; index26 <= mapWidth2; index26 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (let mut index27: i32 =  0; index27 <= mapHeight; index27 += 1)
        {
          let mut index28: i32 =  0;
          do
          {
            this.game.Data.MapObj[0].HexObj[index26, index27].LIStotalHistory[index28] = this.game.Data.MapObj[0].HexObj[index26, index27].LISpoints[index28];
            index28 += 1;
          }
          while (index28 <= 8);
        }
      }
    }

    pub fn LocationProduction()
    {
      SimpleList simpleList1 = SimpleList::new();
      let mut movetype: i32 =   Math.Round( this.game.Data.RuleVar[99]);
      let mut ap: i32 =   Math.Round( this.game.Data.RuleVar[3]);
      SimpleList simpleList2 = SimpleList::new();
      if (this.game.Data.LocCounter == -1)
        return;
      let mut locCounter: i32 =  this.game.Data.LocCounter;
      regime: i32;
      for (let mut tid: i32 =  0; tid <= locCounter; tid += 1)
      {
        regime = this.game.Data.MapObj[this.game.Data.LocObj[tid].Map].HexObj[this.game.Data.LocObj[tid].X, this.game.Data.LocObj[tid].Y].Regime;
        let mut type: i32 =  this.game.Data.LocObj[tid].Type;
        if (regime == this.game.Data.Turn & (this.game.Data.LocObj[tid].HQ > -1 |  this.game.Data.RuleVar[332] == 1.0))
          simpleList2.Add(tid, this.game.Data.LocTypeObj[type].ZOrder);
        else if (regime == this.game.Data.Turn & (this.game.Data.LocTypeObj[this.game.Data.LocObj[tid].Type].NoHQ |  this.game.Data.RuleVar[332] == 1.0))
          simpleList2.Add(tid, this.game.Data.LocTypeObj[type].ZOrder);
      }
      if (simpleList2.Counter == -1)
        return;
      simpleList2.Sort();
      let mut counter1: i32 =  simpleList2.Counter;
      CoordList coordList;
      for (let mut index1: i32 =  0; index1 <= counter1; index1 += 1)
      {
        let mut index2: i32 =  simpleList2.Id[index1];
        let mut x: i32 =  this.game.Data.LocObj[index2].X;
        let mut y: i32 =  this.game.Data.LocObj[index2].Y;
        let mut map: i32 =  this.game.Data.LocObj[index2].Map;
        let mut hq: i32 =  this.game.Data.LocObj[index2].HQ;
        regime = this.game.Data.MapObj[this.game.Data.LocObj[index2].Map].HexObj[this.game.Data.LocObj[index2].X, this.game.Data.LocObj[index2].Y].Regime;
        if (hq > -1 & !this.game.Data.LocTypeObj[this.game.Data.LocObj[index2].Type].NoHQ)
        {
          if (this.game.Data.UnitObj[hq].X == -1)
          {
            simpleList2.Data1[index1] = 1;
          }
          else
          {
            movetype =  Math.Round( this.game.Data.RuleVar[99]);
            ap =  Math.Round( this.game.Data.RuleVar[3]);
            if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI &  this.game.Data.RuleVar[312] == 1.0)
              ap = 0;
            coordList = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map);
            if (this.game.EditObj.TempValue[map].Value[x, y] < 9999)
            {
              simpleList2.Data2[index1] = this.game.EditObj.TempValue[map].Value[x, y];
              let mut antiSupply: i32 =  this.game.HandyFunctionsObj.GetAntiSupply(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map, x, y, map);
              simpleList2.Data3[index1] = antiSupply;
            }
            else
            {
              simpleList2.Data1[index1] = 1;
              simpleList2.Data2[index1] = 9999;
            }
          }
        }
        else if (this.game.Data.LocTypeObj[this.game.Data.LocObj[index2].Type].NoHQ)
        {
          simpleList2.Data2[index1] = 0;
          simpleList2.Data3[index1] = 0;
        }
        else if ( this.game.Data.RuleVar[332] != 1.0)
          simpleList2.Data1[index1] = 1;
      }
      let mut counter2: i32 =  simpleList2.Counter;
      for (let mut index3: i32 =  0; index3 <= counter2; index3 += 1)
      {
        let mut locnr: i32 =  simpleList2.Id[index3];
        let mut x: i32 =  this.game.Data.LocObj[locnr].X;
        let mut y: i32 =  this.game.Data.LocObj[locnr].Y;
        let mut map: i32 =  this.game.Data.LocObj[locnr].Map;
        let mut hq: i32 =  this.game.Data.LocObj[locnr].HQ;
        if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI &  this.game.Data.RuleVar[312] == 1.0)
          ap = 0;
        let mut type: i32 =  this.game.Data.LocObj[locnr].Type;
        if ( this.game.Data.RuleVar[332] == 0.0 & !this.game.Data.LocTypeObj[type].NoHQ)
          coordList = this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.Turn, movetype, 99, ap, this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map);
        let mut index4: i32 =  0;
        do
        {
          let mut itemtypenr: i32 =  this.game.Data.LocObj[locnr].Production[index4];
          if (itemtypenr > -1)
          {
            if (Operators.CompareString(this.game.Data.LocObj[locnr].Name, "Sydney", false) == 0)
              ;
            num1: i32;
            num2: i32;
            if ( simpleList2.Data2[index3] >  this.game.Data.RuleVar[51])
            {
              num1 =  Math.Round(Conversion.Int(this.game.HandyFunctionsObj.GetEstimatedProduction(index4, locnr, false, false)));
              num2 =  Math.Round(Conversion.Int(this.game.HandyFunctionsObj.GetEstimatedProduction(index4, locnr, false, false, true)));
            }
            else
            {
              num1 =  Math.Round(Conversion.Int(this.game.HandyFunctionsObj.GetEstimatedProduction(index4, locnr, true, false)));
              num2 =  Math.Round(Conversion.Int(this.game.HandyFunctionsObj.GetEstimatedProduction(index4, locnr, true, false, true)));
            }
            bool flag = false;
            if (itemtypenr > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, itemtypenr).result)
            {
              num1 = 0;
              flag = true;
            }
            float num3 = 1f;
            if ( this.game.Data.RuleVar[332] == 0.0)
            {
              if ( simpleList2.Data2[index3] >  this.game.Data.RuleVar[53])
              {
                num1 =  Math.Round( num1 * 0.25);
                num3 = 0.25f;
              }
              else if ( simpleList2.Data2[index3] >  this.game.Data.RuleVar[52])
              {
                num1 =  Math.Round( num1 * 0.5);
                num3 = 0.5f;
              }
              else if ( simpleList2.Data2[index3] >  this.game.Data.RuleVar[51])
              {
                num1 =  Math.Round( num1 * 0.75);
                num3 = 0.75f;
              }
            }
            if (simpleList2.Data1[index3] == 1)
              num1 = 0;
            let mut Qty: i32 =  num1;
            if (Qty > 0)
              Qty *= this.game.Data.ItemTypeObj[itemtypenr].Multiplier;
            let mut num4: i32 =  0;
            let mut index5: i32 =  0;
            do
            {
              if (this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCost[index5] > -1 && this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCost[index5]] < num2 * this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCostQty[index5])
                flag = true;
              index5 += 1;
            }
            while (index5 <= 4);
            num5: i32;
            if (simpleList2.Data3[index3] > 0 & Qty != 0)
            {
              if (this.game.Data.ItemTypeObj[itemtypenr].IsSFType > -1)
              {
                if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[itemtypenr].IsSFType].Theater != 1)
                {
                  let mut num6: i32 =  Qty;
                  Qty =  Math.Round(Conversion.Int( Qty * (1.0 -  simpleList2.Data3[index3] / 100.0)));
                  if (num6 == Qty &&  VBMath.Rnd() <  simpleList2.Data3[index3] / 100.0)
                  {
                    --Qty;
                    num4 = 1;
                  }
                  if (num6 > Qty)
                  {
                    num4 = num6 - Qty;
                    int[] sasProdLost = this.game.Data.RegimeObj[this.game.Data.Turn].SASProdLost;
                    int[] numArray = sasProdLost;
                    let mut index6: i32 =  itemtypenr;
                    let mut index7: i32 =  index6;
                    let mut num7: i32 =  sasProdLost[index6] + (num6 - Qty);
                    numArray[index7] = num7;
                    if (this.game.Data.ItemTypeObj[itemtypenr].IsSupply)
                      this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map, x, y, map, num6 - Qty, 0, 0);
                    if (this.game.Data.ItemTypeObj[itemtypenr].IsSFType > -1)
                      this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map, x, y, map, 0, this.game.Data.ItemTypeObj[itemtypenr].IsSFType, num6 - Qty);
                    num5 = num6;
                  }
                }
              }
              else if (this.game.Data.ItemTypeObj[itemtypenr].IsSupply)
              {
                let mut num8: i32 =  Qty;
                Qty =  Math.Round(Conversion.Int( Qty * (1.0 -  simpleList2.Data3[index3] / 100.0)));
                if (num8 == Qty &&  VBMath.Rnd() <  simpleList2.Data3[index3] / 100.0)
                {
                  --Qty;
                  num4 = 1;
                }
                if (num8 > Qty)
                {
                  num4 = num8 - Qty;
                  int[] sasProdLost = this.game.Data.RegimeObj[this.game.Data.Turn].SASProdLost;
                  int[] numArray = sasProdLost;
                  let mut index8: i32 =  itemtypenr;
                  let mut index9: i32 =  index8;
                  let mut num9: i32 =  sasProdLost[index8] + (num8 - Qty);
                  numArray[index9] = num9;
                  if (this.game.Data.ItemTypeObj[itemtypenr].IsSupply)
                    this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map, x, y, map, num8 - Qty, 0, 0);
                  if (this.game.Data.ItemTypeObj[itemtypenr].IsSFType > -1)
                    this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[hq].X, this.game.Data.UnitObj[hq].Y, this.game.Data.UnitObj[hq].Map, x, y, map, 0, this.game.Data.ItemTypeObj[itemtypenr].IsSFType, num8 - Qty);
                  num5 = num8;
                }
              }
            }
            if (Qty > 0)
            {
              let mut num10: i32 =  0;
              let mut Xp: i32 =  0 + this.game.Data.ItemTypeObj[itemtypenr].XpMod;
              if (Xp < 0)
                Xp = 0;
              if ( Xp >  this.game.Data.RuleVar[81])
                Xp =  Math.Round( this.game.Data.RuleVar[81]);
              let mut Peopletype: i32 =  this.game.Data.LocObj[locnr].People;
              if (this.game.Data.ItemTypeObj[itemtypenr].PeopleMod > -1)
                Peopletype = this.game.Data.ItemTypeObj[itemtypenr].PeopleMod;
              if (this.game.Data.ItemTypeObj[itemtypenr].PeopleMod == -2)
                Peopletype = this.game.Data.RegimeObj[this.game.Data.Turn].People;
              let mut Mor: i32 =  this.game.Data.PeopleObj[this.game.Data.LocObj[locnr].People].BaseMorale[this.game.Data.PeopleObj[this.game.Data.RegimeObj[this.game.Data.Turn].People].PeopleGroup];
              if (this.game.Data.ItemTypeObj[itemtypenr].MorMod > 0)
              {
                Mor += this.game.Data.ItemTypeObj[itemtypenr].MorMod;
                if (Mor > 100)
                  Mor = 100;
              }
              let mut moveTypeMod: i32 =  this.game.Data.ItemTypeObj[itemtypenr].MoveTypeMod;
              if (this.game.Data.ItemTypeObj[itemtypenr].IsRegimeSlot > -1)
              {
                int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
                int[] numArray1 = regimeSlot;
                let mut isRegimeSlot: i32 =  this.game.Data.ItemTypeObj[itemtypenr].IsRegimeSlot;
                let mut index10: i32 =  isRegimeSlot;
                let mut num11: i32 =  regimeSlot[isRegimeSlot] + Qty;
                numArray1[index10] = num11;
                if (num10 == 0)
                {
                  sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                  numArray2: Vec<i32> = sprod;
                  let mut index11: i32 =  itemtypenr;
                  let mut index12: i32 =  index11;
                  let mut round: i32 =  this.game.Data.Round;
                  let mut index13: i32 =  round;
                  let mut num12: i32 =  sprod[index11, round] + Qty;
                  numArray2[index12, index13] = num12;
                  num10 = 1;
                }
              }
              if (this.game.Data.LocTypeObj[type].NoHQ & this.game.Data.ItemTypeObj[itemtypenr].IsResPt)
              {
                RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                RegimeClass[] regimeClassArray = regimeObj;
                let mut turn: i32 =  this.game.Data.Turn;
                let mut index14: i32 =  turn;
                regimeClassArray[index14].ResPts = regimeObj[turn].ResPts + Qty;
                if (num10 == 0)
                {
                  sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                  numArray: Vec<i32> = sprod;
                  let mut index15: i32 =  itemtypenr;
                  let mut index16: i32 =  index15;
                  let mut round: i32 =  this.game.Data.Round;
                  let mut index17: i32 =  round;
                  let mut num13: i32 =  sprod[index15, round] + Qty;
                  numArray[index16, index17] = num13;
                  num10 = 1;
                }
              }
              index5 = 0;
              do
              {
                if (this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCost[index5] > -1)
                {
                  int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
                  int[] numArray3 = regimeSlot;
                  int[] regimeSlotsCost = this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCost;
                  int[] numArray4 = regimeSlotsCost;
                  let mut index18: i32 =  index5;
                  let mut index19: i32 =  index18;
                  let mut index20: i32 =  numArray4[index19];
                  let mut num14: i32 =  regimeSlot[regimeSlotsCost[index18]] - Qty * this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCostQty[index5];
                  numArray3[index20] = num14;
                }
                index5 += 1;
              }
              while (index5 <= 4);
              num15: i32;
              if (!this.game.Data.LocTypeObj[type].NoHQ)
              {
                if (this.game.Data.ItemTypeObj[itemtypenr].IsSupply)
                {
                  this.game.Data.UnitObj[hq].Supply += Qty;
                  UnitClass[] unitObj = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray = unitObj;
                  let mut index21: i32 =  hq;
                  let mut index22: i32 =  index21;
                  unitClassArray[index22].SupplyIn = unitObj[index21].SupplyIn + Qty;
                  if (num10 == 0)
                  {
                    sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                    numArray: Vec<i32> = sprod;
                    let mut index23: i32 =  itemtypenr;
                    let mut index24: i32 =  index23;
                    let mut round: i32 =  this.game.Data.Round;
                    let mut index25: i32 =  round;
                    let mut num16: i32 =  sprod[index23, round] + Qty;
                    numArray[index24, index25] = num16;
                    num15 = 1;
                  }
                }
                else if (this.game.Data.ItemTypeObj[itemtypenr].IsResPt)
                {
                  RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                  RegimeClass[] regimeClassArray = regimeObj;
                  let mut turn: i32 =  this.game.Data.Turn;
                  let mut index26: i32 =  turn;
                  regimeClassArray[index26].ResPts = regimeObj[turn].ResPts + Qty;
                  if (num10 == 0)
                  {
                    sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                    numArray: Vec<i32> = sprod;
                    let mut index27: i32 =  itemtypenr;
                    let mut index28: i32 =  index27;
                    let mut round: i32 =  this.game.Data.Round;
                    let mut index29: i32 =  round;
                    let mut num17: i32 =  sprod[index27, round] + Qty;
                    numArray[index28, index29] = num17;
                    num15 = 1;
                  }
                }
                else if (this.game.Data.ItemTypeObj[itemtypenr].IsSFType > -1)
                {
                  if ( this.game.Data.RuleVar[332] == 0.0)
                  {
                    this.game.HandyFunctionsObj.GetPowerPtsAbsolute(hq, true);
                    isSfType: i32;
                    if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[itemtypenr].IsSFType].Theater == 0 && this.game.Data.SFTypeObj[isSfType].StaffPts < 1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                    {
                      isSfType = this.game.Data.ItemTypeObj[itemtypenr].IsSFType;
                      let mut staffPoints: i32 =  this.game.HandyFunctionsObj.GetStaffPoints(hq);
                      let mut groupPowerPoints: i32 =  this.game.HandyFunctionsObj.GetGroupPowerPoints(hq);
                      let mut num18: i32 =  this.game.Data.SFTypeObj[isSfType].Theater != 0 ? groupPowerPoints : groupPowerPoints + this.game.Data.SFTypeObj[isSfType].PowerPts * Qty;
                      let mut num19: i32 =  num18 - groupPowerPoints;
                      if (num18 > staffPoints)
                        num19 -= num18 - staffPoints;
                      float num20;
                      if (num19 > 0 & staffPoints > 0)
                      {
                        num20 =  num19 /  staffPoints * this.game.Data.RuleVar[36];
                        if ( this.game.Data.RuleVar[36] <  num20)
                          num20 = this.game.Data.RuleVar[36];
                      }
                      else
                        num20 = 0.0f;
                      let mut sfCount: i32 =  this.game.Data.UnitObj[hq].SFCount;
                      for (let mut index30: i32 =  0; index30 <= sfCount; index30 += 1)
                      {
                        let mut sf: i32 =  this.game.Data.UnitObj[hq].SFList[index30];
                        if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].StaffPts > 0)
                        {
                          let mut xp: i32 =  this.game.Data.SFObj[sf].Xp;
                          let mut num21: i32 =  xp;
                          let mut num22: i32 =   Math.Round(Math.Ceiling( xp * (1.0 -  num20)));
                          float num23 =  ( num21 * (1.0 -  num20) % 1.0);
                          if ( num23 > 0.0 &&  DrawMod.RandyNumber.Next(0, 100) / 100.0 >  num23)
                            --num22;
                          if (0 > num22)
                            num22 = 0;
                          this.game.Data.SFObj[sf].Xp = num22;
                        }
                      }
                    }
                    if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[itemtypenr].IsSFType].Theater == 1 & !(this.game.Data.LocObj[locnr].X == this.game.Data.UnitObj[hq].X & this.game.Data.LocObj[locnr].Y == this.game.Data.UnitObj[hq].Y))
                    {
                      let mut unitForProduction: i32 =  this.game.HandyFunctionsObj.GetNavyUnitForProduction(this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y, this.game.Data.LocObj[locnr].Map, this.game.Data.ItemTypeObj[itemtypenr].IsSFType, this.game.Data.LocObj[locnr].People, moveTypeMod);
                      if (unitForProduction > -1)
                      {
                        this.game.HandyFunctionsObj.AddTroops3(unitForProduction, this.game.Data.ItemTypeObj[itemtypenr].IsSFType, Peopletype, Qty, Xp, 100, 100, Mor, MoveType: moveTypeMod);
                        if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                        {
                          UnitClass[] unitObj = this.game.Data.UnitObj;
                          UnitClass[] unitClassArray = unitObj;
                          let mut index31: i32 =  unitForProduction;
                          let mut index32: i32 =  index31;
                          unitClassArray[index32].Supply = unitObj[index31].Supply + this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[itemtypenr].IsSFType].SupplyCarry * Qty;
                        }
                        if (num10 == 0)
                        {
                          sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                          numArray: Vec<i32> = sprod;
                          let mut index33: i32 =  itemtypenr;
                          let mut index34: i32 =  index33;
                          let mut round: i32 =  this.game.Data.Round;
                          let mut index35: i32 =  round;
                          let mut num24: i32 =  sprod[index33, round] + Qty;
                          numArray[index34, index35] = num24;
                          num15 = 1;
                        }
                      }
                      else
                      {
                        num1 = 0;
                        Qty = 0;
                        if (num10 == 0)
                        {
                          sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                          numArray: Vec<i32> = sprod;
                          let mut index36: i32 =  itemtypenr;
                          let mut index37: i32 =  index36;
                          let mut round: i32 =  this.game.Data.Round;
                          let mut index38: i32 =  round;
                          let mut num25: i32 =  sprod[index36, round] + Qty;
                          numArray[index37, index38] = num25;
                          num15 = 1;
                        }
                      }
                    }
                    else
                    {
                      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                      {
                        UnitClass[] unitObj = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray = unitObj;
                        let mut index39: i32 =  hq;
                        let mut index40: i32 =  index39;
                        unitClassArray[index40].Supply = unitObj[index39].Supply + this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[itemtypenr].IsSFType].SupplyCarry * Qty;
                      }
                      if (this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[itemtypenr].IsSFType].Theater == 2)
                        this.game.Data.UnitObj[hq].SOInterceptRdnStop = 100;
                      this.game.HandyFunctionsObj.AddTroops3(hq, this.game.Data.ItemTypeObj[itemtypenr].IsSFType, Peopletype, Qty, Xp,  Math.Round( (100f * num3)), 0, Mor, MoveType: moveTypeMod);
                      if (num10 == 0)
                      {
                        sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                        numArray: Vec<i32> = sprod;
                        let mut index41: i32 =  itemtypenr;
                        let mut index42: i32 =  index41;
                        let mut round: i32 =  this.game.Data.Round;
                        let mut index43: i32 =  round;
                        let mut num26: i32 =  sprod[index41, round] + Qty;
                        numArray[index42, index43] = num26;
                        num15 = 1;
                      }
                    }
                  }
                  else
                  {
                    let mut unitForProduction: i32 =  this.game.HandyFunctionsObj.GetAnyUnitForProduction(this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y, this.game.Data.LocObj[locnr].Map, this.game.Data.ItemTypeObj[itemtypenr].IsSFType, this.game.Data.LocObj[locnr].People, moveTypeMod);
                    if (unitForProduction > -1)
                    {
                      this.game.HandyFunctionsObj.AddTroops3(unitForProduction, this.game.Data.ItemTypeObj[itemtypenr].IsSFType, Peopletype, Qty, Xp, 100, 0, Mor, MoveType: moveTypeMod);
                      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
                      {
                        UnitClass[] unitObj = this.game.Data.UnitObj;
                        UnitClass[] unitClassArray = unitObj;
                        let mut index44: i32 =  unitForProduction;
                        let mut index45: i32 =  index44;
                        unitClassArray[index45].Supply = unitObj[index44].Supply + this.game.Data.SFTypeObj[this.game.Data.ItemTypeObj[itemtypenr].IsSFType].SupplyCarry * Qty;
                      }
                      if (num10 == 0)
                      {
                        sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                        numArray: Vec<i32> = sprod;
                        let mut index46: i32 =  itemtypenr;
                        let mut index47: i32 =  index46;
                        let mut round: i32 =  this.game.Data.Round;
                        let mut index48: i32 =  round;
                        let mut num27: i32 =  sprod[index46, round] + Qty;
                        numArray[index47, index48] = num27;
                        num15 = 1;
                      }
                    }
                    else
                    {
                      num1 = 0;
                      Qty = 0;
                      if (num10 == 0)
                      {
                        sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                        numArray: Vec<i32> = sprod;
                        let mut index49: i32 =  itemtypenr;
                        let mut index50: i32 =  index49;
                        let mut round: i32 =  this.game.Data.Round;
                        let mut index51: i32 =  round;
                        let mut num28: i32 =  sprod[index49, round] + Qty;
                        numArray[index50, index51] = num28;
                        num15 = 1;
                      }
                    }
                  }
                }
                else if (num10 == 0)
                {
                  sprod: Vec<i32> = this.game.Data.RegimeObj[regime].SProd;
                  numArray: Vec<i32> = sprod;
                  let mut index52: i32 =  itemtypenr;
                  let mut index53: i32 =  index52;
                  let mut round: i32 =  this.game.Data.Round;
                  let mut index54: i32 =  round;
                  let mut num29: i32 =  sprod[index52, round] + Qty;
                  numArray[index53, index54] = num29;
                  num15 = 1;
                }
              }
            }
            let mut num30: i32 =  Qty;
            if (flag & num2 > num1)
              Qty += num2 - num1;
            if (!flag)
            {
              if (num1 == 0 |  simpleList2.Data2[index3] >  this.game.Data.RuleVar[51])
              {
                int[] prodPointRemainder = this.game.Data.LocObj[locnr].ProdPointRemainder;
                int[] numArray = prodPointRemainder;
                let mut index55: i32 =  index4;
                let mut index56: i32 =  index55;
                let mut num31: i32 =  prodPointRemainder[index55] + (this.game.HandyFunctionsObj.GetProdPtsForLoc(locnr, index4) - (Qty + num4) * this.game.Data.ItemTypeObj[itemtypenr].ProdWeight);
                numArray[index56] = num31;
              }
              else
                this.game.Data.LocObj[locnr].ProdPointRemainder[index4] = this.game.HandyFunctionsObj.GetProdPtsForLoc(locnr, index4, true) - (Qty + num4) * this.game.Data.ItemTypeObj[itemtypenr].ProdWeight;
            }
            else
              this.game.Data.LocObj[locnr].ProdPointRemainder[index4] = this.game.HandyFunctionsObj.GetProdPtsForLoc(locnr, index4, true) - (num30 + num4) * this.game.Data.ItemTypeObj[itemtypenr].ProdWeight;
            if (0 > this.game.Data.LocObj[locnr].ProdPointRemainder[index4])
              this.game.Data.LocObj[locnr].ProdPointRemainder[index4] = 0;
          }
          index4 += 1;
        }
        while (index4 <= 3);
      }
      if ( this.game.Data.RuleVar[843] <= 0.0)
        return;
      this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
    }

    pub fn LocationProductionPrognosis()
    {
      SimpleList simpleList1 = SimpleList::new();
      let mut num1: i32 =   Math.Round( this.game.Data.RuleVar[0]);
      let mut num2: i32 =   Math.Round( this.game.Data.RuleVar[3]);
      SimpleList simpleList2 = SimpleList::new();
      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
      for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        this.game.Data.RegimeObj[index].TempPPIncrease = 0;
      if (this.game.Data.LocCounter == -1)
        return;
      let mut locCounter: i32 =  this.game.Data.LocCounter;
      for (let mut tid: i32 =  0; tid <= locCounter; tid += 1)
      {
        let mut regime: i32 =  this.game.Data.MapObj[this.game.Data.LocObj[tid].Map].HexObj[this.game.Data.LocObj[tid].X, this.game.Data.LocObj[tid].Y].Regime;
        let mut type: i32 =  this.game.Data.LocObj[tid].Type;
        if (regime == this.game.Data.Turn & (this.game.Data.LocObj[tid].HQ > -1 |  this.game.Data.RuleVar[332] == 1.0))
        {
          if (regime != -1)
            simpleList2.Add(tid, this.game.Data.LocTypeObj[type].ZOrder);
        }
        else if (regime == this.game.Data.Turn & (this.game.Data.LocTypeObj[this.game.Data.LocObj[tid].Type].NoHQ |  this.game.Data.RuleVar[332] == 1.0) && regime != -1)
          simpleList2.Add(tid, this.game.Data.LocTypeObj[type].ZOrder);
      }
      if (simpleList2.Counter == -1)
        return;
      simpleList2.Sort();
      int[] numArray1 = new int[500];
      int[] numArray2 = new int[500];
      if (this.game.Data.Round > 0 & this.game.Data.Turn > -1)
      {
        let mut index: i32 =  0;
        do
        {
          numArray2[index] = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[index];
          index += 1;
        }
        while (index <= 499);
      }
      let mut counter1: i32 =  simpleList2.Counter;
      x: i32;
      y: i32;
      map: i32;
      hq: i32;
      for (let mut index1: i32 =  0; index1 <= counter1; index1 += 1)
      {
        let mut index2: i32 =  simpleList2.Id[index1];
        x = this.game.Data.LocObj[index2].X;
        y = this.game.Data.LocObj[index2].Y;
        map = this.game.Data.LocObj[index2].Map;
        hq = this.game.Data.LocObj[index2].HQ;
        let mut regime: i32 =  this.game.Data.MapObj[this.game.Data.LocObj[index2].Map].HexObj[this.game.Data.LocObj[index2].X, this.game.Data.LocObj[index2].Y].Regime;
        simpleList2.Data2[index1] = 0;
        simpleList2.Data3[index1] = 0;
      }
      let mut counter2: i32 =  simpleList2.Counter;
      for (let mut index3: i32 =  0; index3 <= counter2; index3 += 1)
      {
        let mut locnr: i32 =  simpleList2.Id[index3];
        let mut regime: i32 =  this.game.Data.MapObj[this.game.Data.LocObj[locnr].Map].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime;
        x = this.game.Data.LocObj[locnr].X;
        y = this.game.Data.LocObj[locnr].Y;
        map = this.game.Data.LocObj[locnr].Map;
        hq = this.game.Data.LocObj[locnr].HQ;
        let mut type: i32 =  this.game.Data.LocObj[locnr].Type;
        let mut prodslot: i32 =  0;
        do
        {
          this.game.Data.LocObj[locnr].TempProdPredict[prodslot] = 0;
          let mut itemtypenr: i32 =  this.game.Data.LocObj[locnr].Production[prodslot];
          if (itemtypenr > -1)
          {
            let mut num3: i32 =   Math.Round(Conversion.Int(this.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, locnr, true, false)));
            let mut num4: i32 =   Math.Round(Conversion.Int(this.game.HandyFunctionsObj.GetEstimatedProduction(prodslot, locnr, true, false, true)));
            if (itemtypenr > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, regime, itemtypenr).result)
              num3 = 0;
            if (simpleList2.Data1[index3] == 1)
              num3 = 0;
            let mut num5: i32 =  num3;
            if (num5 > 0)
              num5 *= this.game.Data.ItemTypeObj[itemtypenr].Multiplier;
            if (num5 > 0)
            {
              this.game.Data.LocObj[locnr].TempProdPredict[prodslot] = num5;
              if (this.game.Data.ItemTypeObj[itemtypenr].IsRegimeSlot > -1)
              {
                int[] numArray3 = numArray1;
                int[] numArray4 = numArray3;
                let mut isRegimeSlot: i32 =  this.game.Data.ItemTypeObj[itemtypenr].IsRegimeSlot;
                let mut index4: i32 =  isRegimeSlot;
                let mut num6: i32 =  numArray3[isRegimeSlot] + this.game.Data.ItemTypeObj[itemtypenr].Multiplier * num5;
                numArray4[index4] = num6;
              }
              if (this.game.Data.ItemTypeObj[itemtypenr].IsResPt)
              {
                RegimeClass[] regimeObj = this.game.Data.RegimeObj;
                RegimeClass[] regimeClassArray = regimeObj;
                let mut index5: i32 =  regime;
                let mut index6: i32 =  index5;
                regimeClassArray[index6].TempPPIncrease = regimeObj[index5].TempPPIncrease + this.game.Data.ItemTypeObj[itemtypenr].Multiplier * num5;
              }
            }
            if (regime == this.game.Data.Turn && num4 > 0)
            {
              let mut index7: i32 =  0;
              do
              {
                if (this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCost[index7] > -1)
                {
                  int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
                  int[] numArray5 = regimeSlot;
                  int[] regimeSlotsCost = this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCost;
                  int[] numArray6 = regimeSlotsCost;
                  let mut index8: i32 =  index7;
                  let mut index9: i32 =  index8;
                  let mut index10: i32 =  numArray6[index9];
                  let mut num7: i32 =  regimeSlot[regimeSlotsCost[index8]] - num4 * this.game.Data.ItemTypeObj[itemtypenr].RegimeSlotsCostQty[index7];
                  numArray5[index10] = num7;
                }
                index7 += 1;
              }
              while (index7 <= 4);
            }
          }
          prodslot += 1;
        }
        while (prodslot <= 3);
      }
      if (!(this.game.Data.Round > 0 & this.game.Data.Turn > -1))
        return;
      let mut index11: i32 =  0;
      do
      {
        this.game.Data.RegimeObj[this.game.Data.Turn].TempRegimeSlotPredict[index11] = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[index11];
        this.game.Data.RegimeObj[this.game.Data.Turn].TempRegimeSlotIncrease[index11] = numArray1[index11];
        this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[index11] = numArray2[index11];
        this.game.Data.RegimeObj[this.game.Data.Turn].LastTempRegimeSlotPredict[index11] = this.game.Data.RegimeObj[this.game.Data.Turn].TempRegimeSlotPredict[index11];
        index11 += 1;
      }
      while (index11 <= 499);
    }

    pub fn GoToMarchMode(unr: i32)
    {
      let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index1: i32 =  0; index1 <= sfCount; index1 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index1];
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        let mut index2: i32 =  sf;
        let mut index3: i32 =  index2;
        sfClassArray[index3].Ap = sfObj[index2].Ap - 25;
        if (0 > this.game.Data.SFObj[sf].Ap)
          this.game.Data.SFObj[sf].Ap = 0;
        this.game.Data.SFObj[sf].Rdn =  Math.Round(0.75 *  this.game.Data.SFObj[sf].Rdn);
      }
      this.game.Data.UnitObj[unr].moveMode = 1;
    }

    pub OrderResult ExecuteMovement(
      unr: i32,
      x1: i32,
      y1: i32,
      map1: i32,
      x2: i32,
      y2: i32,
      map2: i32,
      bool immediateBattleExecute = false,
      bool allowHistoryForOwnRegime = false)
    {
      Coordinate[] coordinateArray = new Coordinate[251];
      OrderResult orderResult1 = OrderResult::new();
      MapMatrix2[] mapMatrix2Array = new MapMatrix2[this.game.Data.MapCounter + 1];
      orderResult1.OK = false;
      if (x1 == x2 & y1 == y2)
        return orderResult1;
      orderResult1.OK = true;
      SimpleList simpleList1 = SimpleList::new();
      bool isSea = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].LandscapeType].IsSea;
      bool flag;
      if (this.game.HandyFunctionsObj.HasUnitAirSF(unr) && !this.game.HandyFunctionsObj.HasUnitlandSF(unr) && !this.game.HandyFunctionsObj.HasUnitNavySF(unr))
        flag = true;
      let mut mapCounter1: i32 =  this.game.Data.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter1; index1 += 1)
      {
        mapMatrix2Array[index1] = new MapMatrix2(this.game.Data.MapObj[index1].MapWidth, this.game.Data.MapObj[index1].MapHeight);
        let mut mapWidth: i32 =  this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
            mapMatrix2Array[index1].Value[index2, index3] = 9999;
        }
      }
      let mut index4: i32 =  x2;
      let mut index5: i32 =  y2;
      let mut index6: i32 =  map2;
      coordinateArray[0].x = x2;
      coordinateArray[0].y = y2;
      coordinateArray[0].map = map2;
      let mut index7: i32 =  0;
      let mut regime1: i32 =  this.game.Data.UnitObj[unr].Regime;
      for (; !(index4 == x1 & index5 == y1 & index6 == map1); index6 = coordinateArray[index7].map)
      {
        index7 += 1;
        if (index7 > 249)
        {
          orderResult1.OK = false;
          return orderResult1;
        }
        coordinateArray[index7] = this.game.EditObj.TempCameFrom[index6].Value[index4, index5];
        index4 = coordinateArray[index7].x;
        index5 = coordinateArray[index7].y;
      }
      let mut num1: i32 =  index7 - 1;
      let mut x3: i32 =  x1;
      let mut y3: i32 =  y1;
      let mut index8: i32 =  map1;
      if ( this.game.Data.RuleVar[814] == 1.0)
        num1 = 0;
      let mut ox: i32 =  x3;
      let mut oy: i32 =  y3;
      if ((this.game.Data.Product == 6 | this.game.Data.Product == 7) & this.game.Data.RegimeObj[this.game.Data.Turn].AI)
      {
        if (Information.IsNothing( this.game.Data.UnitObj[unr].tempCoords))
          this.game.Data.UnitObj[unr].tempCoords = CoordList::new();
        this.game.Data.UnitObj[unr].tempCoords.AddCoord(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0);
      }
      regime2: i32;
      num2: i32;
      for (let mut index9: i32 =  num1; index9 >= 0; index9 += -1)
      {
        let mut hisdata: i32 =  0;
        if ( this.game.Data.RuleVar[459] > 0.0 & this.game.Data.Product >= 6 && !this.game.HandyFunctionsObj.HasUnitAirSF(unr) && !this.game.HandyFunctionsObj.HasUnitNavySF(unr))
        {
          let mut num3: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, 0, coordinateArray[index9].x, coordinateArray[index9].y, 0) - 1;
          let mut num4: i32 =  num3 + 3;
          if (num4 > 5)
            num4 -= 6;
          let mut num5: i32 =   Math.Round(Math.Ceiling( (this.game.HandyFunctionsObj.GetUnitWeight(unr, true) * 1) /  this.game.Data.RuleVar[33]));
          if (this.game.Data.UnitObj[unr].moveMode == 1)
            num5 *= 2;
          int[] liSpoints1 = this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].LISpoints;
          int[] numArray1 = liSpoints1;
          let mut index10: i32 =  num3;
          let mut index11: i32 =  index10;
          let mut num6: i32 =  liSpoints1[index10] + num5;
          numArray1[index11] = num6;
          int[] liSpoints2 = this.game.Data.MapObj[0].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].LISpoints;
          int[] numArray2 = liSpoints2;
          let mut index12: i32 =  num4;
          let mut index13: i32 =  index12;
          let mut num7: i32 =  liSpoints2[index12] + num5;
          numArray2[index13] = num7;
        }
        if (!this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(coordinateArray[index9].x, coordinateArray[index9].y, coordinateArray[index9].map, regime1, true, true) | this.game.HandyFunctionsObj.HasUnitAirSF(unr) & !this.game.HandyFunctionsObj.HasUnitNavySF(unr))
        {
          regime2 = this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Regime;
          if (!this.game.HandyFunctionsObj.HasUnitAirSF(unr) && !this.game.HandyFunctionsObj.HasUnitNavySF(unr) && this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Regime != this.game.Data.Turn)
          {
            num2 = 0;
            if (this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Regime == -1)
              num2 = 1;
            else if (this.game.Data.RegimeObj[regime1].RegimeRel[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Regime] == 0)
              num2 = 1;
            if (num2 == 1)
            {
              if ( this.game.Data.RuleVar[840] == 1.0 & this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].OrigOwner > -1 & this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[unr].Regime, this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].OrigOwner))
              {
                if (unr > -1)
                  this.game.HandyFunctionsObj.UnitCausesHexOwnershipChange(this.game.Data.UnitObj[unr].Regime, coordinateArray[index9].x, coordinateArray[index9].y, ox, oy);
                this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].set_LastReg(this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Regime, this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].OrigOwner);
                this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Regime = this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].OrigOwner;
              }
              else
              {
                if (unr > -1)
                  this.game.HandyFunctionsObj.UnitCausesHexOwnershipChange(this.game.Data.UnitObj[unr].Regime, coordinateArray[index9].x, coordinateArray[index9].y, ox, oy);
                this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Regime = this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime <= -1 ? this.game.Data.Turn : this.game.Data.RegimeObj[this.game.Data.Turn].UberRegime;
                if (this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].FuzzyBlock == 1)
                  this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].FuzzyBlock = 2;
              }
              if (this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location > -1)
              {
                this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].HQ = -1;
                let mut index14: i32 =  0;
                do
                {
                  this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].Production[index14] = -1;
                  this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].ProdPercent[index14] = 0;
                  this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].ProdPointRemainder[index14] = 0;
                  index14 += 1;
                }
                while (index14 <= 3);
                if ( this.game.Data.RuleVar[898] > 0.0)
                  this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].StructuralPts = 0;
                if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].Type].AutoProd > -1)
                {
                  this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].Production[0] = this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].Type].AutoProd;
                  this.game.Data.LocObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location].ProdPercent[0] = 100;
                  this.game.HandyFunctionsObj.UpgradeProduction(this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].Location);
                }
                this.LocationProductionPrognosis();
              }
            }
            hisdata = 1;
          }
          let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
          for (let mut index15: i32 =  0; index15 <= sfCount1; index15 += 1)
          {
            let mut type: i32 =  this.game.Data.SFObj[this.game.Data.UnitObj[unr].SFList[index15]].Type;
            let mut tweight: i32 =  this.game.HandyFunctionsObj.MoveApCostPreview2(x1, y1, this.game.Data.Turn, this.game.Data.SFTypeObj[type].MoveType, this.game.Data.SFTypeObj[type].Theater, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, coordinateArray[index9].x, coordinateArray[index9].y, coordinateArray[index9].map, false, redux: this.game.Data.SFTypeObj[type].MoveRedux).x;
            if (this.game.Data.SFTypeObj[type].Theater == 2)
              tweight =  Math.Round( tweight / 2.0);
            if (tweight > 4000)
              tweight = 0;
            simpleList1.AddWeight(this.game.Data.UnitObj[unr].SFList[index15], tweight);
          }
          let mut transportCounter1: i32 =  this.game.Data.UnitObj[unr].TransportCounter;
          for (let mut index16: i32 =  0; index16 <= transportCounter1; index16 += 1)
          {
            let mut transport: i32 =  this.game.Data.UnitObj[unr].TransportList[index16];
            let mut sfCount2: i32 =  this.game.Data.UnitObj[transport].SFCount;
            for (let mut index17: i32 =  0; index17 <= sfCount2; index17 += 1)
            {
              let mut type: i32 =  this.game.Data.SFObj[this.game.Data.UnitObj[transport].SFList[index17]].Type;
              let mut tweight: i32 =  this.game.HandyFunctionsObj.MoveApCostPreview2(x1, y1, this.game.Data.Turn, this.game.Data.SFTypeObj[type].MoveType, this.game.Data.SFTypeObj[type].Theater, this.game.Data.UnitObj[transport].X, this.game.Data.UnitObj[transport].Y, this.game.Data.UnitObj[transport].Map, coordinateArray[index9].x, coordinateArray[index9].y, coordinateArray[index9].map, false, redux: this.game.Data.SFTypeObj[type].MoveRedux).x;
              if (this.game.Data.SFTypeObj[type].Theater == 2)
                tweight =  Math.Round( tweight / 2.0);
              if (tweight > 4000)
                tweight = 0;
              simpleList1.AddWeight(this.game.Data.UnitObj[transport].SFList[index17], tweight);
            }
          }
          this.game.Data.MapObj[index8].HexObj[x3, y3].RemoveUnitFromList(unr);
          let mut transportCounter2: i32 =  this.game.Data.UnitObj[unr].TransportCounter;
          for (let mut index18: i32 =  0; index18 <= transportCounter2; index18 += 1)
            this.game.Data.MapObj[index8].HexObj[x3, y3].RemoveUnitFromList(this.game.Data.UnitObj[unr].TransportList[index18]);
          this += 1.game.Data.StepNr;
          infostring1: String = this.game.Data.UnitObj[unr].Name + " moves...";
          if (!flag | x3 == x1 & y3 == y1 & index8 == map1)
            this.game.HandyFunctionsObj.HistoryAddHex(x3, y3, index8, regime1, 2, 0, unr, infostring: infostring1, allowAddedForCurrentTurn: allowHistoryForOwnRegime, infostringunknown: "Unknown unit moves...", infostringnotseen: "Unseen unit moves...");
          if (this.game.Data.UnitObj[unr].PassengerCounter > -1 &&  this.game.Data.RuleVar[881] > 0.0 && this.game.HandyFunctionsObj.IsHexPort(x3, y3, 0) & !isSea)
          {
            let mut passengerCounter: i32 =  this.game.Data.UnitObj[unr].PassengerCounter;
            for (let mut index19: i32 =  0; index19 <= passengerCounter; index19 += 1)
            {
              let mut passenger: i32 =  this.game.Data.UnitObj[unr].PassengerList[index19];
              let mut sfCount3: i32 =  this.game.Data.UnitObj[passenger].SFCount;
              for (let mut index20: i32 =  0; index20 <= sfCount3; index20 += 1)
                this.game.Data.SFObj[this.game.Data.UnitObj[passenger].SFList[index20]].Ap = 0;
            }
          }
          let mut transportCounter3: i32 =  this.game.Data.UnitObj[unr].TransportCounter;
          for (let mut index21: i32 =  0; index21 <= transportCounter3; index21 += 1)
            this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].AddUnitToList(this.game.Data.UnitObj[unr].TransportList[index21]);
          this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].AddUnitToList(unr);
          if ((this.game.Data.Product == 6 | this.game.Data.Product == 7) & this.game.Data.RegimeObj[this.game.Data.Turn].AI)
            this.game.Data.UnitObj[unr].tempCoords.AddCoord(coordinateArray[index9].x, coordinateArray[index9].y, 0);
          let mut x4: i32 =  this.game.Data.UnitObj[unr].X;
          let mut y4: i32 =  this.game.Data.UnitObj[unr].Y;
          this.game.Data.UnitObj[unr].X = coordinateArray[index9].x;
          this.game.Data.UnitObj[unr].Y = coordinateArray[index9].y;
          this.game.Data.UnitObj[unr].Map = coordinateArray[index9].map;
          this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(x4, y4, index8, coordinateArray[index9].x, coordinateArray[index9].y, coordinateArray[index9].map, this.game.Data.UnitObj[unr].Regime, unr, dontcountair: true);
          if (this.game.EventRelatedObj.Helper_AirEnabled())
          {
            let mut historical: i32 =  this.game.Data.UnitObj[unr].Historical;
            if (historical > -1)
            {
              this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(55, 0);
              this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(56, 0);
              this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(57, 0);
              this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(58, 0);
              this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(59, 0);
            }
          }
          let mut transportCounter4: i32 =  this.game.Data.UnitObj[unr].TransportCounter;
          for (let mut index22: i32 =  0; index22 <= transportCounter4; index22 += 1)
          {
            let mut transport: i32 =  this.game.Data.UnitObj[unr].TransportList[index22];
            let mut x5: i32 =  this.game.Data.UnitObj[transport].X;
            let mut y5: i32 =  this.game.Data.UnitObj[transport].Y;
            this.game.Data.UnitObj[transport].X = coordinateArray[index9].x;
            this.game.Data.UnitObj[transport].Y = coordinateArray[index9].y;
            this.game.Data.UnitObj[transport].Map = coordinateArray[index9].map;
            this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(x5, y5, index8, coordinateArray[index9].x, coordinateArray[index9].y, coordinateArray[index9].map, this.game.Data.UnitObj[transport].Regime, transport, dontcountair: true);
          }
          if (!flag | coordinateArray[index9].x == x2 & coordinateArray[index9].y == y2 & coordinateArray[index9].map == map2)
          {
            infostring2: String = this.game.Data.UnitObj[unr].Name + " moves...";
            this.game.HandyFunctionsObj.HistoryAddHex(coordinateArray[index9].x, coordinateArray[index9].y, coordinateArray[index9].map, regime1, 2, hisdata, unr, infostring: infostring2, allowAddedForCurrentTurn: allowHistoryForOwnRegime, infostringunknown: "Unknown unit moves...", infostringnotseen: "Unseen unit moves...");
          }
          x3 = coordinateArray[index9].x;
          y3 = coordinateArray[index9].y;
          index8 = coordinateArray[index9].map;
          mapMatrix2Array[index8].Value[x3, y3] = 0;
          orderResult1.CList.AddCoord(x3, y3, index8);
          if ( this.game.Data.RuleVar[428] > 0.0 && this.game.HandyFunctionsObj.CheckIfIntercepted(unr, coordinateArray[index9].x, coordinateArray[index9].y))
          {
            orderResult1.BattleUnit = unr;
            orderResult1.BattleX = coordinateArray[index9].x;
            orderResult1.BattleY = coordinateArray[index9].y;
            orderResult1.BattleMap = coordinateArray[index9].map;
            orderResult1.BattleIntercept = true;
            break;
          }
          if (this.game.Data.MapObj[index8].HexObj[x3, y3].CardUponConquest > -1 & (this.game.HandyFunctionsObj.HasUnitlandSF(unr) | this.game.HandyFunctionsObj.HasUnitNavySF(unr)))
          {
            this.game.EditObj.AreaX = x3;
            this.game.EditObj.AreaY = y3;
            this.game.EditObj.AreaMap = index8;
            if (regime2 > -1 && !this.game.Data.RegimeObj[regime2].AI)
              this.game.EditObj.DoCardSlot = this.game.Data.MapObj[index8].HexObj[x3, y3].CardUponConquest;
            x2 = x3;
            y2 = y3;
            map2 = index8;
            break;
          }
          ox = coordinateArray[index9].x;
          oy = coordinateArray[index9].y;
        }
        else
        {
          if (index9 == num1)
          {
            x2 = x1;
            y2 = y1;
            map2 = map1;
          }
          else
          {
            x2 = coordinateArray[index9 + 1].x;
            y2 = coordinateArray[index9 + 1].y;
            map2 = coordinateArray[index9 + 1].map;
          }
          if (!this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(coordinateArray[index9].x, coordinateArray[index9].y, coordinateArray[index9].map, regime1, alwaysshowotherregime: true))
          {
            if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].LandscapeType].IsSea &  this.game.Data.RuleVar[326] == 0.0)
            {
              numArray3: Vec<i32> = this.game.EditObj.TempValue[map2].Value;
              numArray4: Vec<i32> = numArray3;
              let mut index23: i32 =  x2;
              let mut index24: i32 =  index23;
              let mut index25: i32 =  y2;
              let mut index26: i32 =  index25;
              let mut num8: i32 =  numArray3[index23, index25] + 25;
              numArray4[index24, index26] = num8;
            }
            else
            {
              orderResult1.BattleUnit = unr;
              orderResult1.BattleX = coordinateArray[index9].x;
              orderResult1.BattleY = coordinateArray[index9].y;
              orderResult1.BattleMap = coordinateArray[index9].map;
            }
            if ( this.game.Data.RuleVar[431] < 1.0)
              this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].MaxRecon = 999;
            let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
            for (let mut index27: i32 =  0; index27 <= regimeCounter; index27 += 1)
            {
              if (index27 != this.game.Data.Turn && this.game.HandyFunctionsObj.IsAlliedOrSelf(index27, this.game.Data.Turn, true) && this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].MaxRecon > this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].get_ReconPts(index27))
                this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].set_ReconPts(index27, this.game.Data.MapObj[coordinateArray[index9].map].HexObj[coordinateArray[index9].x, coordinateArray[index9].y].MaxRecon);
            }
            break;
          }
          break;
        }
      }
      let mut index28: i32 =  unr;
      let mut transportCounter: i32 =  this.game.Data.UnitObj[index28].TransportCounter;
      for (let mut index29: i32 =  -1; index29 <= transportCounter; index29 += 1)
      {
        if (index29 == -1)
          unr = index28;
        if (index29 > -1)
          unr = this.game.Data.UnitObj[index28].TransportList[index29];
        this.game.Data.UnitObj[unr].DidMove = true;
        this.game.Data.UnitObj[unr].apReserve = 0;
        this.game.Data.UnitObj[unr].Spotted = false;
        this.game.Data.UnitObj[unr].Identified = false;
        if (this.game.Data.UnitObj[unr].SFCount > -1)
        {
          UnitClass[] unitObj1 = this.game.Data.UnitObj;
          UnitClass[] unitClassArray1 = unitObj1;
          let mut index30: i32 =  unr;
          let mut index31: i32 =  index30;
          unitClassArray1[index31].MoveAPSpent = unitObj1[index30].MoveAPSpent + this.game.EditObj.TempValue[map2].Value[x2, y2];
          SimpleList simpleList2 = SimpleList::new();
          sf1: i32;
          if (this.game.Data.Product >= 5)
          {
            let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
            for (let mut index32: i32 =  0; index32 <= sfCount; index32 += 1)
            {
              let mut sf2: i32 =  this.game.Data.UnitObj[unr].SFList[index32];
              if (sf2 != sf1)
              {
                let mut type: i32 =  this.game.Data.SFObj[sf2].Type;
                if (this.game.Data.SFTypeObj[type].CarryCap > 0)
                {
                  let mut readinessLoss: i32 =  this.game.Data.SFTypeObj[type].ReadinessLoss;
                  if (this.game.Data.UnitObj[unr].moveMode == 1 && readinessLoss * 2 > 30)
                    ;
                  simpleList2.Add(sf2, this.game.Data.SFTypeObj[type].CarryCap * this.game.Data.SFObj[sf2].Qty, this.game.Data.SFTypeObj[type].ReadinessLoss, this.game.Data.SFTypeObj[type].CarryCap, type, tdata5: this.game.Data.SFTypeObj[type].ReadinessLoss, CheckExistence: false);
                }
              }
            }
            simpleList2.removeWeight0orLower();
            simpleList2.SortOnData5();
          }
          let mut sfCount4: i32 =  this.game.Data.UnitObj[unr].SFCount;
          for (let mut index33: i32 =  0; index33 <= sfCount4; index33 += 1)
          {
            sf1 = this.game.Data.UnitObj[unr].SFList[index33];
            let mut type1: i32 =  this.game.Data.SFObj[sf1].Type;
            num2 = 1;
            if (this.game.Data.SFTypeObj[type1].Theater == 2 && this.game.HandyFunctionsObj.HasUnitNavySF(unr))
              num2 = 0;
            if (num2 == 1)
            {
              if ( this.game.Data.RuleVar[321] == 1.0 & !this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].AI)
                this.game.Data.SFObj[sf1].EP = 0;
              this.game.Data.SFObj[sf1].Ap -= this.game.EditObj.TempValue[map2].Value[x2, y2];
              if ( this.game.Data.RuleVar[407] > 0.0)
              {
                let mut num9: i32 =  0;
                Coordinate coordinate1;
                coordinate1.x = x2;
                coordinate1.y = y2;
                Coordinate coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate1.x, coordinate1.y];
                while (coordinate2.onmap)
                {
                  num9 += 1;
                  coordinate2 = this.game.EditObj.TempCameFrom[0].Value[coordinate2.x, coordinate2.y];
                  if (num9 > 99)
                  {
                    num9 = 100;
                    break;
                  }
                }
                if (num9 > 0)
                {
                  let mut tid: i32 =  this.game.Data.SFTypeObj[type1].SFTypeVar[ Math.Round( (this.game.Data.RuleVar[407] + 0.0f))];
                  let mut num10: i32 =  this.game.Data.SFTypeObj[type1].SFTypeVar[ Math.Round( (this.game.Data.RuleVar[407] + 1f))];
                  if (tid > 0 & num10 > 0)
                  {
                    tweight1: i32;
                    float d;
                    if (this.game.Data.SFTypeObj[type1].Theater == 2)
                    {
                      tweight1 =  Math.Round(Math.Ceiling( (this.game.Data.SFObj[sf1].Qty * num10 * num9) / 20.0));
                      d =  (this.game.Data.SFObj[sf1].Qty * num10 * num9) / 20f;
                    }
                    else
                    {
                      tweight1 =  Math.Round(Math.Ceiling( (this.game.Data.SFObj[sf1].Qty * num10 * num9) / 10.0));
                      d =  (this.game.Data.SFObj[sf1].Qty * num10 * num9) / 10f;
                    }
                    float num11 =  tweight1 - d;
                    if ( VBMath.Rnd() >  num11)
                    {
                      if (tweight1 > 0)
                        this.game.Data.UnitObj[unr].items.list.RemoveWeight(tid, tweight1);
                    }
                    else
                    {
                      let mut tweight2: i32 =   Math.Round(Math.Floor( d));
                      if (tweight2 > 0)
                        this.game.Data.UnitObj[unr].items.list.RemoveWeight(tid, tweight2);
                    }
                  }
                }
                if (Information.IsNothing( this.game.Data.UnitObj[unr].items))
                  this.game.Data.UnitObj[unr].items = ItemList::new();
                this.game.Data.UnitObj[unr].items.list.removeWeight0orLower();
              }
              let mut num12: i32 =  simpleList1.FindWeight(sf1);
              if (this.game.Data.UnitObj[unr].FreeCombatX == x2 & this.game.Data.UnitObj[unr].FreeCombatY == y2)
                num12 = 0;
              if (this.game.Data.SFTypeObj[type1].FuelForMove > 0 & this.game.Data.SFTypeObj[type1].FuelRegimeVar > -1)
              {
                if ( this.game.Data.RuleVar[435] > 0.0)
                {
                  let mut fuelForMove: i32 =  this.game.Data.SFTypeObj[type1].FuelForMove;
                  let mut fuelRegimeVar: i32 =  this.game.Data.SFTypeObj[type1].FuelRegimeVar;
                  if (this.game.Data.SFTypeObj[type1].FuelForMove > fuelForMove & num12 > 0)
                    fuelForMove = this.game.Data.SFTypeObj[type1].FuelForMove;
                  let mut num13: i32 =  fuelForMove * this.game.Data.SFObj[sf1].Qty;
                  if (this.game.Data.UnitObj[unr].moveMode == 1)
                    num13 =  Math.Round(Math.Ceiling( num13 / 2.0));
                  UnitClass[] unitObj2 = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray2 = unitObj2;
                  let mut index34: i32 =  unr;
                  let mut index35: i32 =  index34;
                  unitClassArray2[index35].Fuel = unitObj2[index34].Fuel - num13;
                  if (0 > this.game.Data.UnitObj[unr].Fuel)
                    this.game.Data.UnitObj[unr].Fuel = 0;
                  UnitClass[] unitObj3 = this.game.Data.UnitObj;
                  UnitClass[] unitClassArray3 = unitObj3;
                  let mut index36: i32 =  unr;
                  let mut index37: i32 =  index36;
                  unitClassArray3[index37].FuelUsedMove = unitObj3[index36].FuelUsedMove + num13;
                }
                else if (num12 > 0)
                {
                  let mut num14: i32 =   Math.Round( this.game.Data.SFTypeObj[type1].FuelForMove * ( num12 / 10.0));
                  let mut currentSlot: i32 =  this.game.Data.SFTypeObj[type1].FuelRegimeVar;
                  if ( this.game.Data.RuleVar[949] > 0.0)
                    currentSlot = this.game.HandyFunctionsObj.GetFuelSlot949(currentSlot, x1, y1);
                  if (this.game.Data.SFTypeObj[type1].FuelForMove > num14)
                    num14 = this.game.Data.SFTypeObj[type1].FuelForMove;
                  let mut num15: i32 =  num14 * this.game.Data.SFObj[sf1].Qty;
                  if (this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot[currentSlot] >= num15)
                  {
                    int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot;
                    int[] numArray5 = regimeSlot;
                    let mut index38: i32 =  currentSlot;
                    let mut index39: i32 =  index38;
                    let mut num16: i32 =  regimeSlot[index38] - num15;
                    numArray5[index39] = num16;
                    int[] regimeSlotPredict = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].TempRegimeSlotPredict;
                    int[] numArray6 = regimeSlotPredict;
                    let mut index40: i32 =  currentSlot;
                    let mut index41: i32 =  index40;
                    let mut num17: i32 =  regimeSlotPredict[index40] - num15;
                    numArray6[index41] = num17;
                    UnitClass[] unitObj4 = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray4 = unitObj4;
                    let mut index42: i32 =  unr;
                    let mut index43: i32 =  index42;
                    unitClassArray4[index43].FuelUsedMove = unitObj4[index42].FuelUsedMove + num15;
                  }
                  else
                  {
                    let mut num18: i32 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot[currentSlot];
                    int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot;
                    int[] numArray7 = regimeSlot;
                    let mut index44: i32 =  currentSlot;
                    let mut index45: i32 =  index44;
                    let mut num19: i32 =  regimeSlot[index44] - num18;
                    numArray7[index45] = num19;
                    int[] regimeSlotPredict = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].TempRegimeSlotPredict;
                    int[] numArray8 = regimeSlotPredict;
                    let mut index46: i32 =  currentSlot;
                    let mut index47: i32 =  index46;
                    let mut num20: i32 =  regimeSlotPredict[index46] - num18;
                    numArray8[index47] = num20;
                    UnitClass[] unitObj5 = this.game.Data.UnitObj;
                    UnitClass[] unitClassArray5 = unitObj5;
                    let mut index48: i32 =  unr;
                    let mut index49: i32 =  index48;
                    unitClassArray5[index49].FuelUsedMove = unitObj5[index48].FuelUsedMove + num18;
                  }
                }
              }
              float num21 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].ReadinessLoss;
              if (this.game.Data.UnitObj[unr].moveMode == 1)
              {
                num21 *= 2f;
                if ( num21 > 30.0)
                  num21 = 30f;
              }
              float Number = num21 * ( this.game.EditObj.TempValue[map2].Value[x2, y2] / 100f);
              let mut num22: i32 =   Math.Round( Conversion.Int(Number));
              if (this.game.Data.Product >= 5)
              {
                let mut num23: i32 =  0;
                let mut num24: i32 =  0;
                let mut num25: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].Weight * this.game.Data.SFObj[sf1].Qty;
                let mut num26: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].Weight * this.game.Data.SFObj[sf1].Qty;
                if (this.game.Data.UnitObj[unr].attachedTo > -1 & this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].CarryCap < 1)
                {
                  let mut attachedTo: i32 =  this.game.Data.UnitObj[unr].attachedTo;
                  let mut num27: i32 =  0;
                  let mut num28: i32 =  0;
                  let mut sfCount5: i32 =  this.game.Data.UnitObj[attachedTo].SFCount;
                  for (let mut index50: i32 =  0; index50 <= sfCount5; index50 += 1)
                  {
                    let mut type2: i32 =  this.game.Data.SFObj[this.game.Data.UnitObj[attachedTo].SFList[index50]].Type;
                    let mut qty: i32 =  this.game.Data.SFObj[this.game.Data.UnitObj[attachedTo].SFList[index50]].Qty;
                    if (this.game.Data.SFTypeObj[type2].CarryCap > 0)
                    {
                      if (this.game.Data.UnitObj[attachedTo].moveMode == 1)
                        num27 += this.game.Data.SFTypeObj[type2].ReadinessLoss * qty;
                      else
                        num27 += this.game.Data.SFTypeObj[type2].ReadinessLoss * qty;
                      num28 += qty;
                    }
                  }
                  if (num28 > 0)
                  {
                    Number =   Math.Round( num27 /  num28) * ( this.game.EditObj.TempValue[map2].Value[x2, y2] / 100f);
                    num22 =  Math.Round( Conversion.Int(Number));
                  }
                }
                else if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].CarryCap < 1)
                {
                  let mut counter: i32 =  simpleList2.Counter;
                  for (let mut index51: i32 =  0; index51 <= counter; index51 += 1)
                  {
                    if (num26 > 0 & simpleList2.Weight[index51] > 0 & this.game.Data.SFTypeObj[simpleList2.Data3[index51]].CarryCap >= this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].Weight)
                    {
                      let mut num29: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].ReadinessLoss;
                      if (this.game.Data.UnitObj[unr].moveMode == 1)
                      {
                        num29 *= 2;
                        if (num29 > 30)
                          num29 = 30;
                      }
                      if (num29 > simpleList2.Data1[index51])
                      {
                        if (simpleList2.Weight[index51] < num26)
                        {
                          num24 += simpleList2.Weight[index51] * simpleList2.Data1[index51];
                          num23 += simpleList2.Weight[index51];
                          num26 -= simpleList2.Weight[index51];
                          simpleList2.Weight[index51] = 0;
                        }
                        else
                        {
                          num24 += num26 * simpleList2.Data1[index51];
                          num23 += num26;
                          int[] weight = simpleList2.Weight;
                          int[] numArray = weight;
                          let mut index52: i32 =  index51;
                          let mut index53: i32 =  index52;
                          let mut num30: i32 =  weight[index52] - num26;
                          numArray[index53] = num30;
                          num26 = 0;
                        }
                      }
                    }
                  }
                  if (num24 > 0)
                  {
                    let mut num31: i32 =   Math.Round( num24 /  num23);
                    let mut num32: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sf1].Type].ReadinessLoss;
                    if (this.game.Data.UnitObj[unr].moveMode == 1)
                    {
                      num32 *= 2;
                      if (num32 > 30)
                        num32 = 30;
                    }
                    if (num31 < num32)
                    {
                      Number =  num31 * ( this.game.EditObj.TempValue[map2].Value[x2, y2] / 100f);
                      num22 =  Math.Round( Conversion.Int(Number));
                    }
                  }
                }
              }
              float num33 = Number -  num22;
              if ( num33 > 0.0 &&  VBMath.Rnd() <  num33)
                num22 += 1;
              if (!this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].AI |  this.game.Data.RuleVar[995] == 0.0)
                this.game.Data.SFObj[sf1].Rdn -= num22;
              else if (this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].AI |  this.game.Data.RuleVar[995] == 2.0)
                this.game.Data.SFObj[sf1].Rdn =  Math.Round( this.game.Data.SFObj[sf1].Rdn -  num22 / 2.0);
              if ( this.game.Data.SFObj[sf1].Rdn <  this.game.Data.RuleVar[60])
                this.game.Data.SFObj[sf1].Rdn =  Math.Round( this.game.Data.RuleVar[60]);
              if (0 > this.game.Data.SFObj[sf1].Ap)
                this.game.Data.SFObj[sf1].Ap = 0;
            }
            if (this.game.EditObj.TempValue[map2].Value[x2, y2] >= 0 & !(x2 == x1 & y2 == y1 & map2 == map1))
            {
              let mut num34: i32 =   Math.Round( this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[map2].HexObj[x2, y2].LandscapeType].DefBonus[this.game.Data.SFTypeObj[type1].UnitGroup]);
              if (this.game.Data.MapObj[map2].HexObj[x2, y2].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[map2].HexObj[x2, y2].Location].Type].PictureLT > -1)
                num34 =  Math.Round( ( num34 + this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[map2].HexObj[x2, y2].Location].Type].PictureLT].DefBonus[this.game.Data.SFTypeObj[type1].UnitGroup]));
              this.game.Data.SFObj[sf1].CurrentEntrench = num34;
              this.game.Data.SFObj[sf1].initialEntrench = this.game.Data.SFObj[sf1].CurrentEntrench;
            }
          }
        }
        this.game.Data.UnitObj[unr].FreeCombatX = -1;
        this.game.Data.UnitObj[unr].FreeCombatY = -1;
        this.game.Data.UnitObj[unr].FreeCombatMap = -1;
      }
      unr = index28;
      if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI & !immediateBattleExecute)
      {
        let mut num35: i32 =  3;
        if ( this.game.Data.RuleVar[419] > 0.0 &&  this.game.Data.RuleVar[422] > 0.0)
          num35 =  Math.Round(Conversion.Val( this.game.Data.RuleVar[422]));
        let mut num36: i32 =  9999;
        let mut num37: i32 =  0;
        let mut num38: i32 =  9999;
        let mut num39: i32 =  0;
        let mut counter1: i32 =  orderResult1.CList.counter;
        for (let mut index54: i32 =  0; index54 <= counter1; index54 += 1)
        {
          if (orderResult1.CList.coord[index54].x < num36)
            num36 = orderResult1.CList.coord[index54].x;
          if (orderResult1.CList.coord[index54].x > num37)
            num37 = orderResult1.CList.coord[index54].x;
          if (orderResult1.CList.coord[index54].y < num38)
            num38 = orderResult1.CList.coord[index54].y;
          if (orderResult1.CList.coord[index54].y > num39)
            num39 = orderResult1.CList.coord[index54].y;
        }
        let mut num40: i32 =  num36 - (num35 + 1);
        let mut num41: i32 =  num37 + (num35 + 1);
        let mut num42: i32 =  num38 - (num35 + 1);
        let mut num43: i32 =  num39 + (num35 + 1);
        if (num40 < 0)
          num40 = 0;
        if (num41 > this.game.Data.MapObj[0].MapWidth)
          num41 = this.game.Data.MapObj[0].MapWidth;
        if (num42 < 0)
          num42 = 0;
        if (num43 > this.game.Data.MapObj[0].MapHeight)
          num43 = this.game.Data.MapObj[0].MapHeight;
        let mut num44: i32 =  0;
        do
        {
          let mut mapCounter2: i32 =  this.game.Data.MapCounter;
          for (let mut index55: i32 =  0; index55 <= mapCounter2; index55 += 1)
          {
            let mut num45: i32 =  num40;
            let mut num46: i32 =  num41;
            for (let mut index56: i32 =  num45; index56 <= num46; index56 += 1)
            {
              let mut num47: i32 =  num42;
              let mut num48: i32 =  num43;
              for (let mut index57: i32 =  num47; index57 <= num48; index57 += 1)
              {
                if (num44 == 0 & num35 > 0)
                {
                  let mut num49: i32 =  999;
                  let mut counter2: i32 =  orderResult1.CList.counter;
                  for (let mut index58: i32 =  0; index58 <= counter2; index58 += 1)
                  {
                    let mut num50: i32 =  this.game.HandyFunctionsObj.Distance(index56, index57, 0, orderResult1.CList.coord[index58].x, orderResult1.CList.coord[index58].y, 0, num35 + 1);
                    if (num50 < num49)
                      num49 = num50;
                  }
                  if (num49 <= num35)
                    orderResult1.CList.AddCoord(index56, index57, 0);
                }
                if (num44 < 4)
                {
                  if (num44 == mapMatrix2Array[index55].Value[index56, index57])
                  {
                    let mut tfacing: i32 =  1;
                    do
                    {
                      Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(index56, index57, index55, tfacing);
                      if (coordinate.onmap && num44 + 1 < mapMatrix2Array[coordinate.map].Value[coordinate.x, coordinate.y])
                      {
                        mapMatrix2Array[coordinate.map].Value[coordinate.x, coordinate.y] = num44 + 1;
                        orderResult1.CList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                      }
                      tfacing += 1;
                    }
                    while (tfacing <= 6);
                  }
                }
                else if (mapMatrix2Array[index55].Value[index56, index57] == 9999)
                {
                  num2 = 0;
                  if (this.game.EditObj.TempValue[index55].Value[index56, index57] < 9999)
                    num2 = 1;
                  let mut index59: i32 =  0;
                  do
                  {
                    if (this.game.EditObj.TempAttack[index55].Value[index56, index57, index59])
                      num2 = 1;
                    index59 += 1;
                  }
                  while (index59 <= 5);
                  if (num2 == 1)
                    orderResult1.CList.AddCoord(index56, index57, index55);
                }
              }
            }
          }
          num2 = num2;
          num44 += 1;
        }
        while (num44 <= 4);
      }
      if (orderResult1.BattleUnit > -1 & (this.game.Data.RegimeObj[this.game.Data.Turn].AI | immediateBattleExecute))
      {
        this.game.TempCombat = new CombatClass(this.game);
        Coordinate Target = Coordinate::new();
        Target.x = orderResult1.BattleX;
        Target.y = orderResult1.BattleY;
        Target.map = orderResult1.BattleMap;
        OrderResult orderResult2;
        if (immediateBattleExecute)
          orderResult2 = orderResult1;
        if (orderResult1.BattleIntercept)
        {
          this.game.EditObj.TempUnitList = this.game.HandyFunctionsObj.GetInterceptFireUnits(orderResult1.BattleUnit, orderResult1.BattleX, orderResult1.BattleY);
          orderResult1 = this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, 99, tallowHistoryOwnRegime: allowHistoryForOwnRegime);
        }
        else
        {
          this.game.EditObj.TempUnitList = UnitList::new();
          this.game.EditObj.TempUnitList.add(orderResult1.BattleUnit);
          orderResult1 = this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, 1, tallowHistoryOwnRegime: allowHistoryForOwnRegime);
        }
        this.game.TempCombat.DoBattle();
        this.game.TempCombat.EndBattle();
        this.game.TempCombat = (CombatClass) null;
        if (immediateBattleExecute)
          orderResult1 = orderResult2;
      }
      if (unr > this.game.Data.UnitCounter)
        unr = -1;
      if (unr > -1)
      {
        this.game.ProcessingObj.MaxReadinessRule(unr);
        if (this.game.Data.MapObj[index8].HexObj[x3, y3].CardUponConquest > -1 & (this.game.HandyFunctionsObj.HasUnitlandSF(unr) | this.game.HandyFunctionsObj.HasUnitNavySF(unr)) && regime2 > -1 && this.game.Data.RegimeObj[regime2].AI)
          this.PlayCard(this.game.Data.MapObj[index8].HexObj[x3, y3].CardUponConquest);
      }
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      return orderResult1;
    }

    pub object DoDeckCards()
    {
      if ( this.game.Data.RuleVar[817] != 1.0)
      {
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
        {
          if (this.game.Data.UnitObj[index1].Historical > -1 && this.game.Data.UnitObj[index1].Regime == this.game.Data.Turn && this.game.Data.UnitObj[index1].PreDef == -1)
          {
            let mut historical: i32 =  this.game.Data.UnitObj[index1].Historical;
            if (this.game.Data.HistoricalUnitObj[historical].DeckCardCounter > -1)
            {
              let mut deckCardCounter: i32 =  this.game.Data.HistoricalUnitObj[historical].DeckCardCounter;
              for (let mut index2: i32 =  0; index2 <= deckCardCounter; index2 += 1)
              {
                let mut num1: i32 =  this.game.Data.HistoricalUnitObj[historical].DeckCard[index2];
                let mut num2: i32 =  this.game.Data.HistoricalUnitObj[historical].DeckChance[index2];
                if ( Conversion.Int(VBMath.Rnd() * 100f) + 1.0 <=  num2)
                {
                  let mut num3: i32 =  1;
                  let mut handCardCounter: i32 =  this.game.Data.HistoricalUnitObj[historical].HandCardCounter;
                  for (let mut index3: i32 =  0; index3 <= handCardCounter; index3 += 1)
                  {
                    if (this.game.Data.HistoricalUnitObj[historical].HandCard[index3] == num1)
                      num3 = 0;
                  }
                  if (num3 == 1)
                  {
                    HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
                    HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
                    let mut index4: i32 =  historical;
                    let mut index5: i32 =  index4;
                    historicalUnitClassArray[index5].HandCardCounter = historicalUnitObj[index4].HandCardCounter + 1;
                    this.game.Data.HistoricalUnitObj[historical].HandCard = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[historical].HandCard, (Array) new int[this.game.Data.HistoricalUnitObj[historical].HandCardCounter + 1]);
                    this.game.Data.HistoricalUnitObj[historical].HandCard[this.game.Data.HistoricalUnitObj[historical].HandCardCounter] = num1;
                  }
                }
              }
            }
          }
        }
      }
      object obj;
      return obj;
    }

    pub object DoAutoEvents()
    {
      if ( this.game.Data.RuleVar[817] != 1.0)
      {
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        for (let mut tv2: i32 =  0; tv2 <= unitCounter; tv2 += 1)
        {
          if (this.game.Data.UnitObj[tv2].Historical > -1 && this.game.Data.UnitObj[tv2].Regime == this.game.Data.Turn && this.game.Data.UnitObj[tv2].PreDef == -1)
          {
            let mut historical: i32 =  this.game.Data.UnitObj[tv2].Historical;
            if (this.game.Data.HistoricalUnitObj[historical].AutoEventCounter > -1)
            {
              let mut autoEventCounter: i32 =  this.game.Data.HistoricalUnitObj[historical].AutoEventCounter;
              for (let mut index: i32 =  0; index <= autoEventCounter; index += 1)
              {
                let mut enr: i32 =  this.game.Data.HistoricalUnitObj[historical].AutoEvent[index];
                let mut num: i32 =  this.game.Data.HistoricalUnitObj[historical].AutoChance[index];
                if ( Conversion.Int(VBMath.Rnd() * 100f) + 1.0 <=  num)
                  this.game.EventRelatedObj.DoCheckSpecificEvent(enr, -1, -1, tv2, -1);
              }
            }
          }
        }
      }
      object obj;
      return obj;
    }

    pub fn EditorMovement(unr: i32, x1: i32, y1: i32, map1: i32, x2: i32, y2: i32, map2: i32) -> i32
    {
      if (!(x1 == x2 & y1 == y2) && !(x1 == -1 | x2 == -1))
      {
        this.game.Data.MapObj[map1].HexObj[x1, y1].RemoveUnitFromList(unr);
        this.game.Data.MapObj[map2].HexObj[x2, y2].AddUnitToList(unr);
        this.game.Data.UnitObj[unr].X = x2;
        this.game.Data.UnitObj[unr].Y = y2;
        this.game.Data.UnitObj[unr].Map = map2;
      }
      num: i32;
      return num;
    }

    pub OrderResult LoadUnit(Unr: i32, OnUnr: i32)
    {
      OrderResult orderResult = OrderResult::new();
      if (this.game.Data.Turn > -1)
        this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(this.game.Data.UnitObj[Unr].X, this.game.Data.UnitObj[Unr].Y, this.game.Data.UnitObj[Unr].Map, -1, -1, -1, this.game.Data.Turn, Unr);
      this.game.Data.MapObj[this.game.Data.UnitObj[Unr].Map].HexObj[this.game.Data.UnitObj[Unr].X, this.game.Data.UnitObj[Unr].Y].RemoveUnitFromList(Unr);
      this.game.Data.UnitObj[Unr].X = -1;
      this.game.Data.UnitObj[Unr].Y = -1;
      this.game.Data.UnitObj[OnUnr].AddPassenger(Unr);
      this.game.Data.UnitObj[Unr].OnBoard = OnUnr;
      bool flag;
      if (this.game.Data.MapObj[this.game.Data.UnitObj[OnUnr].Map].HexObj[this.game.Data.UnitObj[OnUnr].X, this.game.Data.UnitObj[OnUnr].Y].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.Data.UnitObj[OnUnr].Map].HexObj[this.game.Data.UnitObj[OnUnr].X, this.game.Data.UnitObj[OnUnr].Y].Location].Type].IsPort)
        flag = true;
      orderResult.OK = true;
      if (this.game.Data.UnitObj[Unr].SFCount > -1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[Unr].SFCount;
        for (let mut index: i32 =  0; index <= sfCount; index += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[Unr].SFList[index];
          if ( this.game.Data.RuleVar[880] < 1.0 | !flag)
            this.game.Data.SFObj[sf].Ap = 0;
          this.game.Data.SFObj[sf].CurrentEntrench = 0;
          this.game.Data.SFObj[sf].initialEntrench = 0;
          this.game.Data.SFObj[sf].EP = 0;
        }
      }
      if (!flag &  this.game.Data.RuleVar[882] > 0.0 && this.game.Data.UnitObj[OnUnr].SFCount > -1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[OnUnr].SFCount;
        for (let mut index: i32 =  0; index <= sfCount; index += 1)
          this.game.Data.SFObj[this.game.Data.UnitObj[OnUnr].SFList[index]].Ap = 0;
      }
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      return orderResult;
    }

    pub OrderResult unLoadUnit(Unr: i32, OnUnr: i32, x: i32, y: i32, map: i32)
    {
      OrderResult orderResult = OrderResult::new();
      this.game.EditObj.SFSelected = -1;
      let mut num: i32 =  0;
      if (this.game.Data.MapObj[map].HexObj[x, y].Regime != this.game.Data.UnitObj[Unr].Regime && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[Unr].Regime, this.game.Data.MapObj[map].HexObj[x, y].Regime))
      {
        if (this.game.Data.MapObj[map].HexObj[x, y].UnitCounter < 0)
        {
          if ( this.game.Data.RuleVar[840] == 1.0 & this.game.Data.MapObj[map].HexObj[x, y].OrigOwner > -1 & this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[Unr].Regime, this.game.Data.MapObj[map].HexObj[x, y].OrigOwner))
          {
            this.game.Data.MapObj[map].HexObj[x, y].set_LastReg(this.game.Data.MapObj[map].HexObj[x, y].Regime, this.game.Data.MapObj[map].HexObj[x, y].OrigOwner);
            this.game.Data.MapObj[map].HexObj[x, y].Regime = this.game.Data.MapObj[map].HexObj[x, y].OrigOwner;
          }
          else
            this.game.Data.MapObj[map].HexObj[x, y].Regime = this.game.Data.RegimeObj[this.game.Data.UnitObj[Unr].Regime].UberRegime <= -1 ? this.game.Data.UnitObj[Unr].Regime : this.game.Data.RegimeObj[this.game.Data.UnitObj[Unr].Regime].UberRegime;
          if (this.game.Data.MapObj[map].HexObj[x, y].Location > -1)
          {
            this.game.Data.LocObj[this.game.Data.MapObj[map].HexObj[x, y].Location].HQ = -1;
            if ( this.game.Data.RuleVar[898] > 0.0)
              this.game.Data.LocObj[this.game.Data.MapObj[map].HexObj[x, y].Location].StructuralPts = 0;
            let mut index: i32 =  0;
            do
            {
              this.game.Data.LocObj[this.game.Data.MapObj[map].HexObj[x, y].Location].Production[index] = -1;
              this.game.Data.LocObj[this.game.Data.MapObj[map].HexObj[x, y].Location].ProdPercent[index] = 0;
              this.game.Data.LocObj[this.game.Data.MapObj[map].HexObj[x, y].Location].ProdPointRemainder[index] = 0;
              index += 1;
            }
            while (index <= 3);
          }
        }
        else
          num = 1;
      }
      this.game.Data.MapObj[map].HexObj[x, y].AddUnitToList(Unr);
      this.game.Data.UnitObj[OnUnr].RemovePassenger(Unr);
      this.game.Data.UnitObj[Unr].X = x;
      this.game.Data.UnitObj[Unr].Y = y;
      this.game.Data.UnitObj[Unr].Map = map;
      this.game.Data.UnitObj[Unr].OnBoard = -1;
      let mut sfCount1: i32 =  this.game.Data.UnitObj[Unr].SFCount;
      for (let mut index: i32 =  0; index <= sfCount1; index += 1)
      {
        if (this.game.Data.Round > 0)
        {
          this.game.Data.SFObj[this.game.Data.UnitObj[Unr].SFList[index]].Ap = 0;
          this.game.Data.SFObj[this.game.Data.UnitObj[Unr].SFList[index]].EP = 0;
        }
      }
      if (!this.game.HandyFunctionsObj.IsHexPort(this.game.Data.UnitObj[OnUnr].X, this.game.Data.UnitObj[OnUnr].Y, 0) &&  this.game.Data.RuleVar[882] > 0.0 && this.game.Data.UnitObj[OnUnr].SFCount > -1)
      {
        let mut sfCount2: i32 =  this.game.Data.UnitObj[OnUnr].SFCount;
        for (let mut index: i32 =  0; index <= sfCount2; index += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[OnUnr].SFList[index];
          if (this.game.Data.Round > 0)
            this.game.Data.SFObj[sf].Ap = 0;
        }
      }
      if (num == 0)
      {
        this += 1.game.Data.StepNr;
        infostring: String = this.game.Data.UnitObj[Unr].Name + " disembarks...";
        let mut sfCount3: i32 =  this.game.Data.UnitObj[Unr].SFCount;
        for (let mut index: i32 =  0; index <= sfCount3; index += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[Unr].SFList[index];
          this.game.Data.SFObj[sf].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(x, y, map, this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup);
          this.game.Data.SFObj[sf].initialEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
        }
        this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, this.game.Data.UnitObj[Unr].Regime, infostring: infostring);
        if (this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest > -1)
        {
          this.game.EditObj.AreaX = x;
          this.game.EditObj.AreaY = y;
          this.game.EditObj.AreaMap = map;
          if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
            this.PlayCard(this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest);
          else
            this.game.EditObj.DoCardSlot = this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest;
        }
      }
      this.game.HandyFunctionsObj.SetOnlyReconAround(x, y, map, this.game.Data.UnitObj[Unr].Regime, Unr);
      this.game.HandyFunctionsObj.SetOnlyZOCAround(x, y, map, this.game.Data.UnitObj[Unr].Regime, Unr);
      if (this.game.Data.Turn > -1 && num == 0)
      {
        let mut mapCounter: i32 =  this.game.Data.MapCounter;
        for (let mut map1: i32 =  0; map1 <= mapCounter; map1 += 1)
        {
          let mut mapWidth: i32 =  this.game.Data.MapObj[map1].MapWidth;
          for (let mut x1: i32 =  0; x1 <= mapWidth; x1 += 1)
          {
            let mut mapHeight: i32 =  this.game.Data.MapObj[map1].MapHeight;
            for (let mut y1: i32 =  0; y1 <= mapHeight; y1 += 1)
              this.game.HandyFunctionsObj.DoZOCConquest(x1, y1, map1, this.game.Data.Turn);
          }
        }
      }
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      orderResult.OK = true;
      return orderResult;
    }

    pub OrderResult paradropUnit(Unr: i32, OnUnr: i32, x: i32, y: i32, map: i32)
    {
      OrderResult orderResult = OrderResult::new();
      bool flag = false;
      if (this.game.Data.MapObj[map].HexObj[x, y].Regime == this.game.Data.Turn & this.game.Data.MapObj[this.game.EditObj.OrderMap].HexObj[x, y].Location > -1)
      {
        if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.OrderMap].HexObj[x, y].Location].Type].IsAirfield)
          flag = true;
        if (this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.OrderMap].HexObj[x, y].Location].isAirfield)
          flag = true;
      }
      if (flag)
        this.game.HandyFunctionsObj.MakeMovePrediction(OnUnr, this.game.Data.UnitObj[OnUnr].X, this.game.Data.UnitObj[OnUnr].Y, this.game.Data.UnitObj[OnUnr].Map, false, PredictAirOnly: true, ClearSea: true, ismove: true);
      else
        this.game.HandyFunctionsObj.MakeMovePrediction(OnUnr, this.game.Data.UnitObj[OnUnr].X, this.game.Data.UnitObj[OnUnr].Y, this.game.Data.UnitObj[OnUnr].Map, false, PredictAirOnly: true, ClearSea: true);
      if (this.game.EditObj.TempValue[map].Value[x, y] == 9999)
      {
        orderResult.OK = false;
        orderResult.ErrorString = "Not enough AP";
        return orderResult;
      }
      if (this.game.Data.UnitObj[OnUnr].SFCount > -1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[OnUnr].SFCount;
        for (let mut index1: i32 =  0; index1 <= sfCount; index1 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[OnUnr].SFList[index1];
          if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater == 2)
          {
            SFClass[] sfObj = this.game.Data.SFObj;
            SFClass[] sfClassArray = sfObj;
            let mut index2: i32 =  sf;
            let mut index3: i32 =  index2;
            sfClassArray[index3].Ap = sfObj[index2].Ap - this.game.EditObj.TempValue[map].Value[x, y];
          }
        }
      }
      this += 1.game.Data.StepNr;
      this.game.Data.MapObj[map].HexObj[x, y].AddUnitToList(Unr);
      this.game.Data.MapObj[this.game.Data.UnitObj[Unr].Map].HexObj[this.game.Data.UnitObj[Unr].X, this.game.Data.UnitObj[Unr].Y].RemoveUnitFromList(Unr);
      this.game.HandyFunctionsObj.HistoryAddHex(this.game.Data.UnitObj[Unr].X, this.game.Data.UnitObj[Unr].Y, this.game.Data.UnitObj[Unr].Map, this.game.Data.UnitObj[Unr].Regime);
      this.game.Data.UnitObj[Unr].X = x;
      this.game.Data.UnitObj[Unr].Y = y;
      this.game.Data.UnitObj[Unr].Map = map;
      let mut sfCount1: i32 =  this.game.Data.UnitObj[Unr].SFCount;
      for (let mut index: i32 =  0; index <= sfCount1; index += 1)
      {
        this.game.Data.SFObj[this.game.Data.UnitObj[Unr].SFList[index]].Ap = 0;
        this.game.Data.SFObj[this.game.Data.UnitObj[Unr].SFList[index]].EP = 0;
        this.game.Data.SFObj[this.game.Data.UnitObj[Unr].SFList[index]].CurrentEntrench = 0;
        this.game.Data.SFObj[this.game.Data.UnitObj[Unr].SFList[index]].initialEntrench = 0;
      }
      infostring: String = this.game.Data.UnitObj[Unr].Name + " paradrops without opposition....";
      this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, this.game.Data.UnitObj[Unr].Regime, infostring: infostring);
      this.game.HandyFunctionsObj.SetHexReconAndZOCAround(x, y, map, this.game.Data.UnitObj[Unr].Regime);
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      orderResult.OK = true;
      return orderResult;
    }

    pub OrderResult SetUnitHq(Unr: i32, Hqnr: i32, bool recurse = false)
    {
      OrderResult orderResult = OrderResult::new();
      if (Unr == Hqnr)
        return orderResult;
      let mut num1: i32 =  1;
      num2: i32;
      num3: i32;
      if (this.game.Data.Round > 0 &  this.game.Data.RuleVar[814] != 1.0)
      {
        num2 = this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(Unr, true);
        num3 = this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(Unr, false);
      }
      howMuchPercentage1: i32;
      howMuchPercentage2: i32;
      if (this.game.Data.Round > 0 &  this.game.Data.RuleVar[907] > 0.0)
      {
        howMuchPercentage1 = this.game.HandyFunctionsObj.AddUnitToHQIsHowMuchPercentage(Hqnr, Unr, true);
        howMuchPercentage2 = this.game.HandyFunctionsObj.AddUnitToHQIsHowMuchPercentage(Hqnr, Unr, false);
      }
      num4: i32;
      val1: i32;
      if (!recurse)
      {
        let mut num5: i32 =  0;
        if (this.game.Data.Turn > -1)
        {
          if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI & this.game.Data.UnitObj[Unr].Historical > -1)
          {
            let mut index: i32 =  0;
            do
            {
              if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[Unr].Historical].SubParts[index] > -1)
                num5 += 1;
              index += 1;
            }
            while (index <= 9);
            if (this.game.Data.RegimeObj[this.game.Data.Turn].ResPts >= this.game.HandyFunctionsObj.ExtraHQSwitchPPCost(Unr, true) && num5 > 1 & !this.game.EditObj.TutMode && Interaction.MsgBox( "Set for all subunits of unit?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              num4 = 1;
            if (num4 == 1)
            {
              let mut unitCounter: i32 =  this.game.Data.UnitCounter;
              for (let mut Unr1: i32 =  0; Unr1 <= unitCounter; Unr1 += 1)
              {
                if (this.game.Data.UnitObj[Unr1].Historical == this.game.Data.UnitObj[Unr].Historical)
                {
                  val1 += 1;
                  this.SetUnitHq(Unr1, Hqnr, true);
                }
              }
            }
          }
        }
        else if (this.game.Data.Round == 0 & this.game.Data.UnitObj[Unr].Historical > -1)
        {
          let mut index: i32 =  0;
          do
          {
            if (this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[Unr].Historical].SubParts[index] > -1)
              num5 += 1;
            index += 1;
          }
          while (index <= 9);
          if (num5 > 1 & !this.game.EditObj.TutMode)
          {
            if (Interaction.MsgBox( "Set for all subunits of unit?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              num4 = 1;
          }
          else
            num4 = 1;
          if (num4 == 1)
          {
            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
            for (let mut Unr2: i32 =  0; Unr2 <= unitCounter; Unr2 += 1)
            {
              if (this.game.Data.UnitObj[Unr2].Historical == this.game.Data.UnitObj[Unr].Historical)
              {
                val1 += 1;
                this.SetUnitHq(Unr2, Hqnr, true);
              }
            }
          }
        }
      }
      let mut index1: i32 =  Hqnr;
      if ( this.game.Data.RuleVar[897] == 0.0)
      {
        while (index1 > -1)
        {
          if (this.game.Data.UnitObj[index1].Historical > -1)
          {
            this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].HandCardCounter = -1;
            this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[index1].Historical].HandCard = new int[1];
            index1 = this.game.Data.UnitObj[index1].HQ;
          }
          else
            index1 = -1;
        }
      }
      if (this.game.Data.Turn > -1 && !(this.game.Data.RegimeObj[this.game.Data.Turn].AI &  this.game.Data.RuleVar[914] == 1.0) && this.game.Data.Round > 0 &  this.game.Data.RuleVar[814] != 1.0)
      {
        if (num4 == 1)
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut turn: i32 =  this.game.Data.Turn;
          let mut index2: i32 =  turn;
          regimeClassArray[index2].ResPts = regimeObj[turn].ResPts - num2;
        }
        else if (!recurse)
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut turn: i32 =  this.game.Data.Turn;
          let mut index3: i32 =  turn;
          regimeClassArray[index3].ResPts = regimeObj[turn].ResPts - num3;
        }
      }
      if (this.game.Data.Round > 0 &  this.game.Data.RuleVar[907] > 0.0 && Hqnr > -1)
      {
        let mut historical: i32 =  this.game.Data.UnitObj[Hqnr].Historical;
        if (historical > -1)
        {
          let mut hisVarCount: i32 =  this.game.Data.HistoricalUnitObj[historical].HisVarCount;
          for (let mut index4: i32 =  0; index4 <= hisVarCount; index4 += 1)
          {
            if ( this.game.Data.HistoricalUnitObj[historical].HisVarType[index4] ==  this.game.Data.RuleVar[907])
            {
              if (num4 == 1)
              {
                int[] hisVarValue = this.game.Data.HistoricalUnitObj[historical].HisVarValue;
                int[] numArray = hisVarValue;
                let mut index5: i32 =  index4;
                let mut index6: i32 =  index5;
                let mut num6: i32 =   Math.Round( hisVarValue[index5] - Math.Max( val1, Conversion.Int( howMuchPercentage1 / 100.0 *  this.game.Data.HistoricalUnitObj[historical].HisVarValue[index4])));
                numArray[index6] = num6;
              }
              else if (!recurse)
              {
                int[] hisVarValue = this.game.Data.HistoricalUnitObj[historical].HisVarValue;
                int[] numArray = hisVarValue;
                let mut index7: i32 =  index4;
                let mut index8: i32 =  index7;
                let mut num7: i32 =   Math.Round( hisVarValue[index7] - Math.Max(1.0, Conversion.Int( howMuchPercentage2 / 100.0 *  this.game.Data.HistoricalUnitObj[historical].HisVarValue[index4])));
                numArray[index8] = num7;
              }
              if (0 > this.game.Data.HistoricalUnitObj[historical].HisVarValue[index4])
                this.game.Data.HistoricalUnitObj[historical].HisVarValue[index4] = 0;
            }
          }
        }
      }
      if (num1 == 1 &  this.game.Data.RuleVar[814] != 1.0 && !this.game.Data.UnitObj[Unr].IsHQ)
      {
        if (Hqnr > -1)
        {
          if (this.game.Data.UnitObj[Unr].SFCount > -1 && this.game.Data.UnitObj[Unr].HQ != -1 && this.game.Data.UnitObj[Unr].HQ != Hqnr)
          {
            let mut sfCount: i32 =  this.game.Data.UnitObj[Unr].SFCount;
            for (let mut index9: i32 =  0; index9 <= sfCount; index9 += 1)
            {
              let mut sf: i32 =  this.game.Data.UnitObj[Unr].SFList[index9];
              if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater < 1)
                this.game.Data.SFObj[sf].Rdn =  Math.Round( ( this.game.Data.SFObj[sf].Rdn * this.game.Data.RuleVar[48]));
            }
          }
          if (this.game.Data.Turn > -1 && !this.game.Data.RegimeObj[this.game.Data.Turn].AI && !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
          {
            let mut staffPoints: i32 =  this.game.HandyFunctionsObj.GetStaffPoints(Hqnr);
            let mut groupPowerPoints: i32 =  this.game.HandyFunctionsObj.GetGroupPowerPoints(Hqnr);
            let mut num8: i32 =  groupPowerPoints + this.game.HandyFunctionsObj.GetPowerPtsAbsolute(Unr, true, false);
            let mut num9: i32 =  num8 - groupPowerPoints;
            if (num8 > staffPoints)
              num9 -= num8 - staffPoints;
            float num10;
            if (num9 > 0 & staffPoints > 0)
            {
              num10 =  num9 /  staffPoints * this.game.Data.RuleVar[36];
              if ( this.game.Data.RuleVar[36] <  num10)
                num10 = this.game.Data.RuleVar[36];
            }
            else
              num10 = 0.0f;
            let mut sfCount: i32 =  this.game.Data.UnitObj[Hqnr].SFCount;
            for (let mut index10: i32 =  0; index10 <= sfCount; index10 += 1)
            {
              let mut sf: i32 =  this.game.Data.UnitObj[Hqnr].SFList[index10];
              if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].StaffPts > 0)
              {
                let mut xp: i32 =  this.game.Data.SFObj[sf].Xp;
                let mut num11: i32 =  xp;
                let mut num12: i32 =   Math.Round(Math.Ceiling( xp * (1.0 -  num10)));
                float num13 =  ( num11 * (1.0 -  num10) % 1.0);
                if ( num13 > 0.0 &&  DrawMod.RandyNumber.Next() / 100.0 >  num13)
                  --num12;
                if (0 > num12)
                  num12 = 0;
                this.game.Data.SFObj[sf].Xp = num12;
              }
            }
          }
        }
        else if (this.game.Data.UnitObj[Unr].SFCount > -1 && this.game.Data.UnitObj[Unr].HQ != -1 && this.game.Data.UnitObj[Unr].HQ != Hqnr)
        {
          let mut sfCount: i32 =  this.game.Data.UnitObj[Unr].SFCount;
          for (let mut index11: i32 =  0; index11 <= sfCount; index11 += 1)
          {
            let mut sf: i32 =  this.game.Data.UnitObj[Unr].SFList[index11];
            if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater < 1)
              this.game.Data.SFObj[sf].Rdn =  Math.Round( ( this.game.Data.SFObj[sf].Rdn * this.game.Data.RuleVar[48]));
          }
        }
      }
      this.game.Data.UnitObj[Unr].HQ = Hqnr;
      this.game.Data.UnitObj[Unr].DidHQ = true;
      orderResult.OK = true;
      return orderResult;
    }

    pub OrderResult SetProdHq(locnr: i32, Hqnr: i32)
    {
      OrderResult orderResult = OrderResult::new();
      this.game.Data.LocObj[locnr].HQ = Hqnr;
      orderResult.OK = true;
      return orderResult;
    }

    pub OrderResult NewUnit(
      x: i32,
      y: i32,
      map: i32,
      bool hq,
      regime: i32,
      bool WithoutOfficerPlease = false)
    {
      OrderResult orderResult = OrderResult::new();
      if (this.game.Data.Round == 0)
      {
        if (this.game.Data.MapObj[map].HexObj[x, y].Regime > -1)
          regime = this.game.Data.MapObj[map].HexObj[x, y].Regime;
        else if (regime == -1)
        {
          orderResult.OK = false;
          return orderResult;
        }
      }
      let mut index1: i32 =  this.game.Data.AddUnit(x, y, map);
      this.game.Data.UnitObj[index1].Regime = regime;
      if (this.game.Data.Round != 0 && !(this.game.Data.RegimeObj[regime].AI &  this.game.Data.RuleVar[863] == 1.0) && !(this.game.Data.RegimeObj[regime].AI &  this.game.Data.RuleVar[914] == 1.0))
      {
        if (!hq)
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut index2: i32 =  regime;
          let mut index3: i32 =  index2;
          regimeClassArray[index3].ResPts =  Math.Round( ( regimeObj[index2].ResPts - this.game.Data.RuleVar[46]));
        }
        if (hq)
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut index4: i32 =  regime;
          let mut index5: i32 =  index4;
          regimeClassArray[index5].ResPts =  Math.Round( ( regimeObj[index4].ResPts - this.game.Data.RuleVar[47]));
        }
      }
      this.game.Data.UnitObj[index1].IsHQ = hq;
      this.game.Data.UnitObj[index1].LastSupplyPercent = -1;
      this.game.Data.UnitObj[index1].SODefendPercent = 50;
      this.game.Data.UnitObj[index1].SOSupReqPercent = 100;
      this.game.Data.UnitObj[index1].SOReplacementPercent = 100;
      this.game.Data.UnitObj[index1].SOInterceptRdnStop = 100;
      this.game.Data.UnitObj[index1].SupplyConsume =  this.game.Data.RuleVar[434] <= 0.0 ? 0 : 100;
      if (!hq)
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index6: i32 =  regime;
        let mut index7: i32 =  index6;
        regimeClassArray[index7].UnitNumber = regimeObj[index6].UnitNumber + 1;
        str1: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[regime].UnitNumber));
        str2: String = (!((this.game.Data.RegimeObj[regime].UnitNumber + 10) % 10 == 1 & (this.game.Data.RegimeObj[regime].UnitNumber + 100) % 100 != 11) ? (!((this.game.Data.RegimeObj[regime].UnitNumber + 10) % 10 == 2 & (this.game.Data.RegimeObj[regime].UnitNumber + 100) % 100 != 12) ? (!((this.game.Data.RegimeObj[regime].UnitNumber + 10) % 10 == 3 & (this.game.Data.RegimeObj[regime].UnitNumber + 100) % 100 != 13) ? str1 + "th" : str1 + "rd") : str1 + "nd") : str1 + "st") + " " + this.game.Data.RegimeObj[regime].UnitName;
        this.game.Data.UnitObj[index1].Name = str2;
      }
      else
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index8: i32 =  regime;
        let mut index9: i32 =  index8;
        regimeClassArray[index9].HQNumber = regimeObj[index8].HQNumber + 1;
        str3: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[regime].HQNumber));
        str4: String = (!((this.game.Data.RegimeObj[regime].HQNumber + 10) % 10 == 1 & (this.game.Data.RegimeObj[regime].HQNumber + 100) % 100 != 11) ? (!((this.game.Data.RegimeObj[regime].HQNumber + 10) % 10 == 2 & (this.game.Data.RegimeObj[regime].HQNumber + 100) % 100 != 12) ? (!((this.game.Data.RegimeObj[regime].HQNumber + 10) % 10 == 3 & (this.game.Data.RegimeObj[regime].HQNumber + 100) % 100 != 13) ? str3 + "th" : str3 + "rd") : str3 + "nd") : str3 + "st") + " " + this.game.Data.RegimeObj[regime].HQName;
        this.game.Data.UnitObj[index1].Name = str4;
        if ( this.game.Data.RuleVar[343] == 1.0 & !WithoutOfficerPlease & this.game.Data.RegimeObj[regime].OfficerPool > -1)
        {
          this.game.Data.AddHistoricalUnit();
          this.game.Data.UnitObj[index1].Historical = this.game.Data.HistoricalUnitCounter;
          this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitCounter].TempRegime = this.game.Data.Turn;
          this.game.ProcessingObj.RecruitOfficer(this.game.Data.Turn, false, this.game.Data.HistoricalUnitCounter);
        }
      }
      orderResult.OK = true;
      orderResult.Data = index1;
      return orderResult;
    }

    pub object DoTransfer(
      unr: i32,
      unrT: i32,
      theater: i32,
      SfNr: i32,
      Qty: i32,
      bool OwnPower = false,
      bool AddtoHistory = true,
      let mut byHQ: i32 =  -1,
      bool MoveMatrixDone = false,
      bool IsDisbandTransfer = false)
    {
      OrderResult orderResult = OrderResult::new();
      Coordinate[] coordinateArray = new Coordinate[251];
      this.game.EditObj.SFSelected = -1;
      let mut index1: i32 =  -1;
      this.game.EditObj.TransferLostQty = 0;
      this.game.EditObj.TransferLostType = -1;
      this.game.EditObj.TransferLostTransports = 0;
      if (!OwnPower)
      {
        if (this.game.Data.UnitObj[unr].IsHQ & !this.game.Data.UnitObj[unrT].IsHQ)
          index1 = unr;
        if (!this.game.Data.UnitObj[unr].IsHQ & this.game.Data.UnitObj[unrT].IsHQ)
          index1 = unrT;
        if (this.game.Data.UnitObj[unr].IsHQ & this.game.Data.UnitObj[unrT].IsHQ)
          index1 = unr;
      }
      if (byHQ > -1)
        index1 = byHQ;
      let mut index2: i32 =  SfNr != -2 ? this.game.Data.SFObj[SfNr].Type : -2;
      this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, -1, -1, -1, this.game.Data.UnitObj[unr].Regime, unr, false);
      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI & index2 > -1 && !OwnPower)
      {
        num: i32;
        if (this.game.Data.SFTypeObj[this.game.Data.SFObj[SfNr].Type].Theater == 2)
        {
          if (!MoveMatrixDone)
            this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime, this.game.Data.SFTypeObj[index2].MoveType, 2, this.game.Data.SFObj[SfNr].Ap, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, false, muststartonairfield: false, istransfer: true);
          num = this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
        }
        else if (this.game.Data.SFTypeObj[this.game.Data.SFObj[SfNr].Type].Theater == 1)
        {
          this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime, this.game.Data.SFTypeObj[index2].MoveType, 1, this.game.Data.SFObj[SfNr].Ap, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, false, muststartonairfield: false, istransfer: true);
          num = this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
        }
        else
        {
          if (!MoveMatrixDone)
            this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime, this.game.Data.SFTypeObj[index2].MoveType, this.game.Data.SFTypeObj[index2].Theater, this.game.Data.SFObj[SfNr].Ap, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map);
          num = this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
        }
        if (num <= this.game.Data.SFObj[SfNr].Ap)
        {
          OwnPower = true;
          theater = this.game.Data.SFTypeObj[index2].Theater;
        }
      }
      num1: i32;
      if (!OwnPower)
      {
        switch (theater)
        {
          case 1:
            if (!MoveMatrixDone)
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime,  Math.Round( this.game.Data.RuleVar[1]), 1,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, false);
            num1 = byHQ != unrT ? this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y] : this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
            break;
          case 2:
            if (!MoveMatrixDone)
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime,  Math.Round( this.game.Data.RuleVar[2]), 0,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map);
            num1 = byHQ != unrT ? this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y] : this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
            break;
          default:
            if (!MoveMatrixDone)
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime,  Math.Round( this.game.Data.RuleVar[0]), 0,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map);
            num1 = this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
            break;
        }
      }
      else
      {
        theater = this.game.Data.SFTypeObj[index2].Theater;
        if (theater == 2)
        {
          this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime, this.game.Data.SFTypeObj[index2].MoveType, 2, this.game.Data.SFObj[SfNr].Ap, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, false, muststartonairfield: false, istransfer: true, redux: this.game.Data.SFTypeObj[index2].MoveRedux, SFTypeX: index2, SFTypeQty: Qty);
          num1 = this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
        }
        else
        {
          this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unr].Regime, this.game.Data.SFTypeObj[index2].MoveType, this.game.Data.SFTypeObj[index2].Theater, this.game.Data.SFObj[SfNr].Ap, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, redux: this.game.Data.SFTypeObj[index2].MoveRedux, SFTypeX: index2, SFTypeQty: Qty);
          num1 = this.game.EditObj.TempValue[this.game.Data.UnitObj[unrT].Map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y];
        }
      }
      let mut antiSupply: i32 =  this.game.HandyFunctionsObj.GetAntiSupply(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y, this.game.Data.UnitObj[unrT].Map);
      Qty1: i32;
      num2: i32;
      if (antiSupply > 0 & !OwnPower)
      {
        if (SfNr == -2 | theater == 1 | theater == 0 &  this.game.Data.RuleVar[309] == 1.0)
        {
          let mut num3: i32 =   Math.Round(Conversion.Int( Qty * ( antiSupply / 100.0)));
          if ( Qty * ( antiSupply / 100.0) % 1.0 > 0.0 &&  VBMath.Rnd() <  antiSupply / 100.0)
            num3 += 1;
          Qty1 = Qty - num3;
          if (SfNr == -2)
            this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y, this.game.Data.UnitObj[unrT].Map, num3, 0, 0);
          else
            this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y, this.game.Data.UnitObj[unrT].Map, 0, this.game.Data.SFObj[SfNr].Type, num3);
          this.game.EditObj.TransferLostQty = num3;
          this.game.EditObj.TransferLostType = SfNr != -2 ? this.game.Data.SFObj[SfNr].Type : -2;
        }
        else
        {
          num2 = 0;
          Qty1 = Qty;
        }
      }
      else if (SfNr > -1 & antiSupply > 0 & theater == 1 & OwnPower)
      {
        let mut qty: i32 =   Math.Round(Conversion.Int( Qty * ( antiSupply / 100.0)));
        if ( Qty * ( antiSupply / 100.0) % 1.0 > 0.0 &&  VBMath.Rnd() <  antiSupply / 100.0)
          qty += 1;
        Qty1 = Qty - qty;
        this.game.HandyFunctionsObj.SetAntiSupplyKills(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y, this.game.Data.UnitObj[unrT].Map, 0, this.game.Data.SFObj[SfNr].Type, qty);
        this.game.EditObj.TransferLostQty = qty;
        this.game.EditObj.TransferLostType = this.game.Data.SFObj[SfNr].Type;
      }
      else
      {
        num2 = 0;
        Qty1 = Qty;
      }
      num4: i32;
      num5: i32;
      people: i32;
      Xp: i32;
      Rdn: i32;
      mor: i32;
      Ep: i32;
      Ap: i32;
      offMod: i32;
      defMod: i32;
      moveType: i32;
      entr: i32;
      theater1: i32;
      supplyConsume: i32;
      if (SfNr == -2)
      {
        num4 =  Math.Round( Conversion.Int( (num1 * Qty) * this.game.Data.RuleVar[33]));
        num5 = num4;
      }
      else
      {
        people = this.game.Data.SFObj[SfNr].People;
        Xp = this.game.Data.SFObj[SfNr].Xp;
        if (IsDisbandTransfer)
          Xp =  Math.Round( ( Xp * this.game.Data.RuleVar[926]));
        Rdn = this.game.Data.SFObj[SfNr].Rdn;
        mor = this.game.Data.SFObj[SfNr].Mor;
        Ep =  Math.Round(Conversion.Int( this.game.Data.SFObj[SfNr].EP * ( Qty /  this.game.Data.SFObj[SfNr].Qty)));
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        let mut index3: i32 =  SfNr;
        let mut index4: i32 =  index3;
        sfClassArray[index4].EP = sfObj[index3].EP - Ep;
        Ap = this.game.Data.SFObj[SfNr].Ap;
        offMod = this.game.Data.SFObj[SfNr].OffMod;
        defMod = this.game.Data.SFObj[SfNr].DefMod;
        moveType = this.game.Data.SFObj[SfNr].MoveType;
        if (this.game.Data.UnitObj[unr].X == this.game.Data.UnitObj[unrT].X & this.game.Data.UnitObj[unr].Y == this.game.Data.UnitObj[unrT].Y)
        {
          entr = this.game.Data.SFObj[SfNr].CurrentEntrench;
        }
        else
        {
          entr = this.game.HandyFunctionsObj.GetMinimumEntrench(this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y, this.game.Data.UnitObj[unrT].Map, this.game.Data.SFTypeObj[this.game.Data.SFObj[SfNr].Type].UnitGroup);
          if (OwnPower)
          {
            Ap -= num1;
            if (0 > Ap)
              Ap = 0;
          }
          else
          {
            Ep = 0;
            Ap = 0;
          }
        }
        let mut weight: i32 =  this.game.Data.SFTypeObj[index2].Weight;
        theater1 = this.game.Data.SFTypeObj[index2].Theater;
        num4 = Conversion.Int(num1 * Qty * weight);
        supplyConsume = this.game.Data.UnitObj[unr].SupplyConsume;
        if (theater == 1 & this.game.Data.SFTypeObj[this.game.Data.SFObj[SfNr].Type].Theater == 1 && !this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.Data.UnitObj[unrT].Map].HexObj[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y].LandscapeType].IsSea)
          num4 = 0;
        num5 = num4;
      }
      if (SfNr == -2)
      {
        UnitClass[] unitObj1 = this.game.Data.UnitObj;
        UnitClass[] unitClassArray1 = unitObj1;
        let mut index5: i32 =  unrT;
        let mut index6: i32 =  index5;
        unitClassArray1[index6].Supply = unitObj1[index5].Supply + Qty1;
        UnitClass[] unitObj2 = this.game.Data.UnitObj;
        UnitClass[] unitClassArray2 = unitObj2;
        let mut index7: i32 =  unr;
        let mut index8: i32 =  index7;
        unitClassArray2[index8].Supply = unitObj2[index7].Supply - Qty;
      }
      else
      {
        let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
        num6: i32;
        for (let mut index9: i32 =  0; index9 <= sfCount1; index9 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index9];
          num6 += Conversion.Int(this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].SupplyCarry * this.game.Data.SFObj[sf].Qty);
        }
        let mut num7: i32 =  this.game.Data.SFTypeObj[index2].SupplyCarry * Qty;
        num8: i32;
        if (num6 > 0)
          num8 =  Math.Round(Conversion.Int( this.game.Data.UnitObj[unr].Supply * ( num7 /  num6)));
        if (num8 > num7)
          num8 = num7;
        this.game.HandyFunctionsObj.RemoveTroops(unr, index2, people, Qty, moveType);
        UnitClass[] unitObj3 = this.game.Data.UnitObj;
        UnitClass[] unitClassArray3 = unitObj3;
        let mut index10: i32 =  unr;
        let mut index11: i32 =  index10;
        unitClassArray3[index11].Supply = unitObj3[index10].Supply - num8;
        if (Qty1 > 0)
        {
          bool flag = false;
          if (this.game.Data.UnitObj[unrT].HQ == this.game.Data.UnitObj[unr].HQ | this.game.Data.UnitObj[unr].HQ == unrT | this.game.Data.UnitObj[unrT].HQ == unr)
            flag = true;
          if (this.game.Data.SFTypeObj[index2].StaffPts > 0 & this.game.Data.UnitObj[unrT].IsHQ)
            flag = false;
          if (this.game.Data.UnitObj[unr].IsHQ & this.game.Data.UnitObj[unrT].IsHQ)
            flag = false;
          if (Strings.InStr(this.game.Data.UnitObj[unrT].Name, "326.Pi") > 0)
            unr = unr;
          if (flag)
          {
            if (theater1 == 0)
              Rdn =  Math.Round( ( Rdn * this.game.Data.RuleVar[49]));
            this.game.HandyFunctionsObj.AddTroops3(unrT, index2, people, Qty1, Xp, Rdn, Ap, mor, supplyConsume, entr, offMod, defMod, moveType, Ep);
          }
          else
          {
            if (theater1 == 0)
              Rdn =  Math.Round( ( Rdn * this.game.Data.RuleVar[50]));
            let mut unr1: i32 =  this.game.Data.UnitObj[unrT].HQ;
            if (this.game.Data.UnitObj[unrT].IsHQ)
              unr1 = unrT;
            staffPercent1: i32;
            if (unr1 > -1)
              staffPercent1 = this.game.HandyFunctionsObj.GetStaffPercent(unr1);
            if (unr1 > -1 & !this.game.Data.RegimeObj[this.game.Data.Turn].AI)
            {
              if (this.game.Data.SFTypeObj[index2].StaffPts > 0 & this.game.Data.UnitObj[unrT].IsHQ)
              {
                let mut staffPercent2: i32 =  this.game.HandyFunctionsObj.GetStaffPercent(unr1, additionalstaffpts: (this.game.Data.SFTypeObj[index2].StaffPts * Qty1));
                float num9;
                if (staffPercent2 <= 100)
                  num9 = this.game.Data.RuleVar[36];
                else if (staffPercent1 > 99)
                {
                  num9 = 0.0f;
                }
                else
                {
                  let mut num10: i32 =  staffPercent2 - staffPercent1;
                  num9 = this.game.Data.RuleVar[36] *  (1.0 -  (staffPercent2 - 100) /  num10);
                }
                let mut num11: i32 =  Xp;
                Xp =  Math.Round(Math.Ceiling( Xp * (1.0 -  num9)));
                float num12 =  ( num11 * (1.0 -  num9) % 1.0);
                if ( num12 > 0.0)
                {
                  if ( DrawMod.RandyNumber.Next(0, 100) / 100.0 >  num12)
                    --Xp;
                  if (0 > Xp)
                    Xp = 0;
                }
              }
              else if (this.game.Data.SFTypeObj[index2].StaffPts < 1)
              {
                let mut staffPoints: i32 =  this.game.HandyFunctionsObj.GetStaffPoints(unr1);
                let mut groupPowerPoints: i32 =  this.game.HandyFunctionsObj.GetGroupPowerPoints(unr1);
                let mut num13: i32 =  this.game.Data.SFTypeObj[index2].Theater != 0 ? groupPowerPoints : groupPowerPoints + this.game.Data.SFTypeObj[index2].PowerPts * Qty1;
                let mut num14: i32 =  num13 - groupPowerPoints;
                if (num13 > staffPoints)
                  num14 -= num13 - staffPoints;
                float num15;
                if (num14 > 0 & staffPoints > 0)
                {
                  num15 =  num14 /  staffPoints * this.game.Data.RuleVar[36];
                  if ( this.game.Data.RuleVar[36] <  num15)
                    num15 = this.game.Data.RuleVar[36];
                }
                else
                  num15 = 0.0f;
                let mut sfCount2: i32 =  this.game.Data.UnitObj[unr1].SFCount;
                for (let mut index12: i32 =  0; index12 <= sfCount2; index12 += 1)
                {
                  let mut sf: i32 =  this.game.Data.UnitObj[unr1].SFList[index12];
                  if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].StaffPts > 0)
                  {
                    let mut xp: i32 =  this.game.Data.SFObj[sf].Xp;
                    let mut num16: i32 =  xp;
                    let mut num17: i32 =   Math.Round(Math.Ceiling( xp * (1.0 -  num15)));
                    float num18 =  ( num16 * (1.0 -  num15) % 1.0);
                    if ( num18 > 0.0 &&  DrawMod.RandyNumber.Next(0, 100) / 100.0 >  num18)
                      --num17;
                    if (0 > num17)
                      num17 = 0;
                    this.game.Data.SFObj[sf].Xp = num17;
                  }
                }
              }
            }
            this.game.HandyFunctionsObj.AddTroops3(unrT, index2, people, Qty1, Xp, Rdn, Ap, mor, supplyConsume, entr, offMod, defMod, moveType, Ep);
          }
          this.game.Data.UnitObj[unr].LastAP = -1;
          this.game.Data.UnitObj[unrT].LastAP = -1;
          this.game.Data.UnitObj[unrT].FreeCombatX = -1;
          this.game.Data.UnitObj[unrT].FreeCombatY = -1;
          UnitClass[] unitObj4 = this.game.Data.UnitObj;
          UnitClass[] unitClassArray4 = unitObj4;
          let mut index13: i32 =  unrT;
          let mut index14: i32 =  index13;
          unitClassArray4[index14].Supply =  Math.Round( unitObj4[index13].Supply +  num8 * ( Qty1 /  Qty));
        }
        if (AddtoHistory)
        {
          this += 1.game.Data.StepNr;
          infostring: String;
          if (this.game.Data.FOWOn)
            infostring = "Transfer";
          else
            infostring = "Transferred " + Conversion.Str( Qty) + "x " + this.game.Data.SFTypeObj[index2].Name + " to " + this.game.Data.UnitObj[unrT].Name + ".";
          this.game.HandyFunctionsObj.HistoryAddHex(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unr].Regime, 2, 0, unr, infostring: infostring);
          this.game.HandyFunctionsObj.HistoryAddHex(this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y, this.game.Data.UnitObj[unrT].Map, this.game.Data.UnitObj[unr].Regime, 2, 0, unrT, infostring: infostring);
        }
      }
      if (!OwnPower && antiSupply > 0 & num4 > 0)
        this.game.EditObj.TransferLostTransports = this.game.HandyFunctionsObj.DoDestroyTransporters(unr, theater,  Math.Round( num4 * ( antiSupply / 100.0)), this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y, this.game.Data.UnitObj[unrT].Map);
      if (index1 > -1 & !OwnPower)
      {
        switch (theater)
        {
          case 0:
            this.game.Data.UnitObj[index1].LandCap -= num4;
            break;
          case 1:
            this.game.Data.UnitObj[index1].NavyCap -= num4;
            break;
          case 2:
            this.game.Data.UnitObj[index1].AirCap -= num4;
            break;
        }
      }
      let mut map: i32 =  this.game.Data.UnitObj[unrT].Map;
      if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI)
      {
        if (OwnPower)
        {
          if ( this.game.Data.RuleVar[886] == 0.0 && this.game.Data.SFTypeObj[index2].FuelForMove > 0 & this.game.Data.SFTypeObj[index2].FuelRegimeVar > -1)
          {
            if ( this.game.Data.RuleVar[435] > 0.0)
            {
              let mut num19: i32 =   Math.Round( this.game.Data.SFTypeObj[index2].FuelForMove * ( this.game.EditObj.TempValue[map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y] / 10.0));
              if (this.game.Data.SFTypeObj[index2].FuelForMove > num19 & this.game.EditObj.TempValue[map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y] > 0)
                num19 = this.game.Data.SFTypeObj[index2].FuelForMove;
              let mut num20: i32 =  num19 * this.game.Data.SFObj[SfNr].Qty;
              UnitClass[] unitObj5 = this.game.Data.UnitObj;
              UnitClass[] unitClassArray5 = unitObj5;
              let mut index15: i32 =  unr;
              let mut index16: i32 =  index15;
              unitClassArray5[index16].Fuel = unitObj5[index15].Fuel - num20;
              if (0 > this.game.Data.UnitObj[unr].Fuel)
                this.game.Data.UnitObj[unr].Fuel = 0;
              UnitClass[] unitObj6 = this.game.Data.UnitObj;
              UnitClass[] unitClassArray6 = unitObj6;
              let mut index17: i32 =  unr;
              let mut index18: i32 =  index17;
              unitClassArray6[index18].FuelUsedMove = unitObj6[index17].FuelUsedMove + num20;
            }
            else
            {
              let mut currentSlot: i32 =  this.game.Data.SFTypeObj[index2].FuelRegimeVar;
              if ( this.game.Data.RuleVar[949] > 0.0)
                currentSlot = this.game.HandyFunctionsObj.GetFuelSlot949(currentSlot, this.game.Data.UnitObj[index1].RealX( this.game), this.game.Data.UnitObj[index1].RealY( this.game));
              let mut num21: i32 =   Math.Round( this.game.Data.SFTypeObj[index2].FuelForMove * ( this.game.EditObj.TempValue[map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y] / 10.0));
              if (this.game.Data.SFTypeObj[index2].FuelForMove > num21 & this.game.EditObj.TempValue[map].Value[this.game.Data.UnitObj[unrT].X, this.game.Data.UnitObj[unrT].Y] > 0)
                num21 = this.game.Data.SFTypeObj[index2].FuelForMove;
              let mut num22: i32 =  num21 * Qty;
              if (this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot[currentSlot] >= num22)
              {
                int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot;
                int[] numArray1 = regimeSlot;
                let mut index19: i32 =  currentSlot;
                let mut index20: i32 =  index19;
                let mut num23: i32 =  regimeSlot[index19] - num22;
                numArray1[index20] = num23;
                int[] regimeSlotPredict = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].TempRegimeSlotPredict;
                int[] numArray2 = regimeSlotPredict;
                let mut index21: i32 =  currentSlot;
                let mut index22: i32 =  index21;
                let mut num24: i32 =  regimeSlotPredict[index21] - num22;
                numArray2[index22] = num24;
                UnitClass[] unitObj = this.game.Data.UnitObj;
                UnitClass[] unitClassArray = unitObj;
                let mut index23: i32 =  unr;
                let mut index24: i32 =  index23;
                unitClassArray[index24].FuelUsedMove = unitObj[index23].FuelUsedMove + num22;
              }
              else
              {
                let mut num25: i32 =  this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot[currentSlot];
                int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].RegimeSlot;
                int[] numArray3 = regimeSlot;
                let mut index25: i32 =  currentSlot;
                let mut index26: i32 =  index25;
                let mut num26: i32 =  regimeSlot[index25] - num25;
                numArray3[index26] = num26;
                int[] regimeSlotPredict = this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].TempRegimeSlotPredict;
                int[] numArray4 = regimeSlotPredict;
                let mut index27: i32 =  currentSlot;
                let mut index28: i32 =  index27;
                let mut num27: i32 =  regimeSlotPredict[index27] - num25;
                numArray4[index28] = num27;
                UnitClass[] unitObj = this.game.Data.UnitObj;
                UnitClass[] unitClassArray = unitObj;
                let mut index29: i32 =  unr;
                let mut index30: i32 =  index29;
                unitClassArray[index30].FuelUsedMove = unitObj[index29].FuelUsedMove + num25;
              }
            }
          }
        }
        else
        {
          if ( this.game.Data.RuleVar[852] > 0.0 & theater == 0 & num4 > 0)
          {
            let mut num28: i32 =   Math.Round( Math.Max(1f, Conversion.Int(  Math.Round( num4 / 1000.0) * this.game.Data.RuleVar[852])));
            float fuelSlot949 = this.game.Data.RuleVar[851];
            if ( this.game.Data.RuleVar[949] > 0.0)
              fuelSlot949 =  this.game.HandyFunctionsObj.GetFuelSlot949( Math.Round( fuelSlot949), this.game.Data.UnitObj[index1].RealX( this.game), this.game.Data.UnitObj[index1].RealY( this.game));
            int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
            int[] numArray = regimeSlot;
            let mut index31: i32 =   Math.Round( fuelSlot949);
            let mut index32: i32 =  index31;
            let mut num29: i32 =  regimeSlot[index31] - num28;
            numArray[index32] = num29;
          }
          if ( this.game.Data.RuleVar[854] > 0.0 & theater == 1 & num4 > 0)
          {
            let mut num30: i32 =   Math.Round( Math.Max(1f, Conversion.Int(  Math.Round( num4 / 1000.0) * this.game.Data.RuleVar[854])));
            float fuelSlot949 = this.game.Data.RuleVar[851];
            if ( this.game.Data.RuleVar[949] > 0.0)
              fuelSlot949 =  this.game.HandyFunctionsObj.GetFuelSlot949( Math.Round( fuelSlot949), this.game.Data.UnitObj[index1].RealX( this.game), this.game.Data.UnitObj[index1].RealY( this.game));
            int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
            int[] numArray = regimeSlot;
            let mut index33: i32 =   Math.Round( fuelSlot949);
            let mut index34: i32 =  index33;
            let mut num31: i32 =  regimeSlot[index33] - num30;
            numArray[index34] = num31;
          }
          if ( this.game.Data.RuleVar[856] > 0.0 & theater == 0 & num4 > 0)
          {
            let mut num32: i32 =   Math.Round( Math.Max(1f, Conversion.Int(  Math.Round( num4 / 1000.0) * this.game.Data.RuleVar[856])));
            float fuelSlot949 = this.game.Data.RuleVar[851];
            if ( this.game.Data.RuleVar[949] > 0.0)
              fuelSlot949 =  this.game.HandyFunctionsObj.GetFuelSlot949( Math.Round( fuelSlot949), this.game.Data.UnitObj[index1].RealX( this.game), this.game.Data.UnitObj[index1].RealY( this.game));
            int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
            int[] numArray = regimeSlot;
            let mut index35: i32 =   Math.Round( this.game.Data.RuleVar[ Math.Round( fuelSlot949)]);
            let mut index36: i32 =  index35;
            let mut num33: i32 =  regimeSlot[index35] - num32;
            numArray[index36] = num33;
          }
        }
      }
      this.game.ProcessingObj.MaxReadinessRule(unrT);
      let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
      num34: i32;
      num35: i32;
      num36: i32;
      for (let mut index37: i32 =  0; index37 <= sfCount; index37 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index37];
        let mut type: i32 =  this.game.Data.SFObj[sf].Type;
        let mut qty: i32 =  this.game.Data.SFObj[sf].Qty;
        let mut theater2: i32 =  this.game.Data.SFTypeObj[type].Theater;
        if (theater2 == 0)
          num34 += this.game.Data.SFTypeObj[type].Cap * qty;
        if (theater2 == 1)
          num35 += this.game.Data.SFTypeObj[type].Cap * qty;
        num36 += this.game.Data.SFTypeObj[type].RailCap * qty;
      }
      if (this.game.Data.UnitObj[unr].LandCap > num34)
        this.game.Data.UnitObj[unr].LandCap = num34;
      if (this.game.Data.UnitObj[unr].AirCap > num36)
        this.game.Data.UnitObj[unr].AirCap = num36;
      if (this.game.Data.UnitObj[unr].NavyCap > num35)
        this.game.Data.UnitObj[unr].NavyCap = num35;
      this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(-1, -1, 0, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unr].Regime, unr, false);
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      this.LocationProductionPrognosis();
      orderResult.OK = true;
      return  orderResult;
    }

    pub object DoStrategicTransfer(
      unrH: i32,
      unrS: i32,
      theater: i32,
      tarx: i32,
      tary: i32,
      tarmap: i32)
    {
      OrderResult orderResult = OrderResult::new();
      Coordinate[] coordinateArray = new Coordinate[251];
      if (this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(tarx, tary, 0, 1, true))
      {
        let mut num1: i32 =  num1;
      }
      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
      {
        this.game.EditObj.TransferLostQty = 0;
        this.game.EditObj.TransferLostType = -1;
        this.game.EditObj.TransferLostTransports = 0;
      }
      num2: i32;
      if (!(this.game.Data.RegimeObj[this.game.Data.Turn].AI &  this.game.Data.RuleVar[253] == 1.0))
      {
        num3: i32;
        if ( this.game.Data.RuleVar[350] == 0.0)
        {
          switch (theater)
          {
            case 1:
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unrH].Regime,  Math.Round( this.game.Data.RuleVar[1]), 1,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map);
              num3 = this.game.EditObj.TempValue[tarmap].Value[tarx, tary] + this.game.EditObj.TempValue[this.game.Data.UnitObj[unrS].Map].Value[this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y];
              break;
            case 2:
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unrH].Regime,  Math.Round( this.game.Data.RuleVar[2]), 0,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map);
              num3 = this.game.EditObj.TempValue[tarmap].Value[tarx, tary] + this.game.EditObj.TempValue[this.game.Data.UnitObj[unrS].Map].Value[this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y];
              break;
            default:
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unrH].Regime,  Math.Round( this.game.Data.RuleVar[0]), 0,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map);
              num3 = this.game.EditObj.TempValue[tarmap].Value[tarx, tary] + this.game.EditObj.TempValue[this.game.Data.UnitObj[unrS].Map].Value[this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y];
              break;
          }
        }
        else
        {
          switch (theater)
          {
            case 1:
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unrH].Regime,  Math.Round( this.game.Data.RuleVar[1]), 1,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y, this.game.Data.UnitObj[unrS].Map);
              num3 = this.game.EditObj.TempValue[tarmap].Value[tarx, tary];
              break;
            case 2:
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unrH].Regime,  Math.Round( this.game.Data.RuleVar[2]), 0,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y, this.game.Data.UnitObj[unrS].Map);
              num3 = this.game.EditObj.TempValue[tarmap].Value[tarx, tary];
              break;
            default:
              this.game.HandyFunctionsObj.MakeMovePrediction2(this.game.Data.UnitObj[unrH].Regime,  Math.Round( this.game.Data.RuleVar[0]), 0,  Math.Round( this.game.Data.RuleVar[78]), this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y, this.game.Data.UnitObj[unrS].Map);
              num3 = this.game.EditObj.TempValue[tarmap].Value[tarx, tary];
              break;
          }
        }
        num2 = Conversion.Int( Math.Round( ( num3 + this.game.Data.RuleVar[351])) * this.game.HandyFunctionsObj.GetUnitWeight(unrS, true));
        num4: i32;
        num5: i32;
        if (theater == 1 | theater == 0 &  this.game.Data.RuleVar[309] == 1.0)
        {
          let mut antiSupply1: i32 =  this.game.HandyFunctionsObj.GetAntiSupply(this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map, tarx, tary, tarmap);
          num4 = antiSupply1;
          if (antiSupply1 > 0)
            this.game.EditObj.TransferLostQty = this.game.HandyFunctionsObj.DoDestroyRandom(unrS, -1, antiSupply1, this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map, tarx, tary, tarmap);
          let mut antiSupply2: i32 =  this.game.HandyFunctionsObj.GetAntiSupply(this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map, this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y, this.game.Data.UnitObj[unrS].Map);
          num5 = antiSupply2;
          if (antiSupply2 > 0)
            this.game.EditObj.TransferLostQty = this.game.HandyFunctionsObj.DoDestroyRandom(unrS, -1, antiSupply2, this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map, this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y, this.game.Data.UnitObj[unrS].Map);
        }
        switch (theater)
        {
          case 0:
            this.game.Data.UnitObj[unrH].LandCap -= num2;
            break;
          case 1:
            this.game.Data.UnitObj[unrH].NavyCap -= num2;
            break;
          case 2:
            this.game.Data.UnitObj[unrH].AirCap -= num2;
            break;
        }
        if (num4 > 0)
          this.game.EditObj.TransferLostTransports = this.game.HandyFunctionsObj.DoDestroyTransporters(unrH, theater,  Math.Round(Conversion.Int( num2 * ( num4 / 100.0))), this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map, tarx, tary, tarmap);
        if (num5 > 0)
          this.game.EditObj.TransferLostTransports += this.game.HandyFunctionsObj.DoDestroyTransporters(unrH, theater,  Math.Round(Conversion.Int( num2 * ( num5 / 100.0))), this.game.Data.UnitObj[unrH].X, this.game.Data.UnitObj[unrH].Y, this.game.Data.UnitObj[unrH].Map, this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y, this.game.Data.UnitObj[unrS].Map);
      }
      this.game.Data.MapObj[this.game.Data.UnitObj[unrS].Map].HexObj[this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y].RemoveUnitFromList(unrS);
      let mut x: i32 =  this.game.Data.UnitObj[unrS].X;
      let mut y: i32 =  this.game.Data.UnitObj[unrS].Y;
      let mut map: i32 =  this.game.Data.UnitObj[unrS].Map;
      this.game.Data.UnitObj[unrS].X = tarx;
      this.game.Data.UnitObj[unrS].Y = tary;
      this.game.Data.UnitObj[unrS].Map = tarmap;
      this.game.Data.MapObj[tarmap].HexObj[tarx, tary].AddUnitToList(unrS);
      this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(x, y, map, -1, -1, -1, this.game.Data.UnitObj[unrS].Regime, unrS, false);
      if (this.game.EventRelatedObj.Helper_AirEnabled())
      {
        let mut historical: i32 =  this.game.Data.UnitObj[unrS].Historical;
        if (historical > -1)
        {
          this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(55, 0);
          this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(56, 0);
          this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(57, 0);
          this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(58, 0);
          this.game.Data.HistoricalUnitObj[historical].SetHisVarValue(59, 0);
        }
      }
      this += 1.game.Data.StepNr;
      this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, this.game.Data.UnitObj[unrS].Regime, infostring: "Strategic Transfer...");
      this.game.HandyFunctionsObj.HistoryAddHex(tarx, tary, tarmap, this.game.Data.UnitObj[unrS].Regime, infostring: "Strategic Transfer...");
      if (this.game.Data.UnitObj[unrS].SFCount > -1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[unrS].SFCount;
        for (let mut index: i32 =  0; index <= sfCount; index += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unrS].SFList[index];
          this.game.Data.SFObj[sf].Ap = 0;
          this.game.Data.SFObj[sf].EP = 0;
          this.game.Data.SFObj[sf].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(tarx, tary, tarmap, this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup);
          this.game.Data.SFObj[sf].initialEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
          this.game.Data.SFObj[sf].Rdn =  Math.Round( ( this.game.Data.SFObj[sf].Rdn * this.game.Data.RuleVar[131]));
        }
      }
      this.game.Data.UnitObj[unrS].Spotted = true;
      this.game.Data.UnitObj[unrS].Identified = true;
      this.game.Data.UnitObj[unrS].LandCap = 0;
      this.game.Data.UnitObj[unrS].NavyCap = 0;
      this.game.Data.UnitObj[unrS].AirCap = 0;
      this.game.Data.UnitObj[unrS].FreeCombatX = -1;
      this.game.Data.UnitObj[unrS].FreeCombatY = -1;
      this.game.ProcessingObj.MaxReadinessRule(unrS);
      if (this.game.Data.MapObj[tarmap].HexObj[tarx, tary].CardUponConquest > -1)
      {
        this.game.EditObj.AreaX = tarx;
        this.game.EditObj.AreaY = tary;
        this.game.EditObj.AreaMap = tarmap;
        if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
          this.PlayCard(this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest);
        else
          this.game.EditObj.DoCardSlot = this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest;
      }
      if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI)
      {
        if ( this.game.Data.RuleVar[852] > 0.0 & theater == 0 & num2 > 0)
        {
          let mut num6: i32 =   Math.Round( Math.Max(1f, Conversion.Int(  Math.Round( num2 / 1000.0) * this.game.Data.RuleVar[852])));
          float fuelSlot949 = this.game.Data.RuleVar[851];
          if ( this.game.Data.RuleVar[949] > 0.0)
            fuelSlot949 =  this.game.HandyFunctionsObj.GetFuelSlot949( Math.Round( fuelSlot949), this.game.Data.UnitObj[unrH].RealX( this.game), this.game.Data.UnitObj[unrH].RealY( this.game));
          int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
          int[] numArray = regimeSlot;
          let mut index1: i32 =   Math.Round( fuelSlot949);
          let mut index2: i32 =  index1;
          let mut num7: i32 =  regimeSlot[index1] - num6;
          numArray[index2] = num7;
        }
        if ( this.game.Data.RuleVar[854] > 0.0 & theater == 1 & num2 > 0)
        {
          let mut num8: i32 =   Math.Round( Math.Max(1f, Conversion.Int(  Math.Round( num2 / 1000.0) * this.game.Data.RuleVar[854])));
          float fuelSlot949 = this.game.Data.RuleVar[851];
          if ( this.game.Data.RuleVar[949] > 0.0)
            fuelSlot949 =  this.game.HandyFunctionsObj.GetFuelSlot949( Math.Round( fuelSlot949), this.game.Data.UnitObj[unrH].RealX( this.game), this.game.Data.UnitObj[unrH].RealY( this.game));
          int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
          int[] numArray = regimeSlot;
          let mut index3: i32 =   Math.Round( fuelSlot949);
          let mut index4: i32 =  index3;
          let mut num9: i32 =  regimeSlot[index3] - num8;
          numArray[index4] = num9;
        }
        if ( this.game.Data.RuleVar[856] > 0.0 & theater == 0 & num2 > 0)
        {
          let mut num10: i32 =   Math.Round( Math.Max(1f, Conversion.Int(  Math.Round( num2 / 1000.0) * this.game.Data.RuleVar[856])));
          float fuelSlot949 = this.game.Data.RuleVar[851];
          if ( this.game.Data.RuleVar[949] > 0.0)
            fuelSlot949 =  this.game.HandyFunctionsObj.GetFuelSlot949( Math.Round( fuelSlot949), this.game.Data.UnitObj[unrH].RealX( this.game), this.game.Data.UnitObj[unrH].RealY( this.game));
          int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
          int[] numArray = regimeSlot;
          let mut index5: i32 =   Math.Round( fuelSlot949);
          let mut index6: i32 =  index5;
          let mut num11: i32 =  regimeSlot[index5] - num10;
          numArray[index6] = num11;
        }
      }
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      orderResult.OK = true;
      this.LocationProductionPrognosis();
      return  orderResult;
    }

    pub object LIS_DoStrategicTransfer(unrS: i32, tarx: i32, tary: i32, tarmap: i32)
    {
      OrderResult orderResult = OrderResult::new();
      Coordinate[] coordinateArray = new Coordinate[251];
      if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
      {
        this.game.EditObj.TransferLostQty = 0;
        this.game.EditObj.TransferLostType = -1;
        this.game.EditObj.TransferLostTransports = 0;
      }
      let mut unitWeight: i32 =  this.game.HandyFunctionsObj.GetUnitWeight(unrS, includeLisWeight: true);
      this.game.ProcessingObj.LIS_RemovePointsFromTrajectory(tarx, tary, unitWeight, unrS);
      this.game.Data.MapObj[this.game.Data.UnitObj[unrS].Map].HexObj[this.game.Data.UnitObj[unrS].X, this.game.Data.UnitObj[unrS].Y].RemoveUnitFromList(unrS);
      let mut x: i32 =  this.game.Data.UnitObj[unrS].X;
      let mut y: i32 =  this.game.Data.UnitObj[unrS].Y;
      let mut map: i32 =  this.game.Data.UnitObj[unrS].Map;
      this.game.Data.UnitObj[unrS].X = tarx;
      this.game.Data.UnitObj[unrS].Y = tary;
      this.game.Data.UnitObj[unrS].Map = tarmap;
      this.game.Data.MapObj[tarmap].HexObj[tarx, tary].AddUnitToList(unrS);
      this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(x, y, map, -1, -1, -1, this.game.Data.UnitObj[unrS].Regime, unrS, false);
      this += 1.game.Data.StepNr;
      this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, this.game.Data.UnitObj[unrS].Regime, infostring: "Strategic Transfer...");
      if (this.game.Data.MapObj[0].HexObj[tarx, tary].Regime == this.game.Data.UnitObj[unrS].Regime)
        this.game.HandyFunctionsObj.HistoryAddHex(tarx, tary, tarmap, this.game.Data.UnitObj[unrS].Regime, infostring: "Strategic Transfer...");
      if (this.game.Data.UnitObj[unrS].SFCount > -1)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[unrS].SFCount;
        for (let mut index: i32 =  0; index <= sfCount; index += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unrS].SFList[index];
          this.game.Data.SFObj[sf].Ap = 0;
          this.game.Data.SFObj[sf].EP = 0;
          this.game.Data.SFObj[sf].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(tarx, tary, tarmap, this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup);
          this.game.Data.SFObj[sf].initialEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
          this.game.Data.SFObj[sf].Rdn =  Math.Round( ( this.game.Data.SFObj[sf].Rdn * this.game.Data.RuleVar[131]));
        }
      }
      this.game.Data.UnitObj[unrS].LandCap = 0;
      this.game.Data.UnitObj[unrS].NavyCap = 0;
      this.game.Data.UnitObj[unrS].AirCap = 0;
      this.game.Data.UnitObj[unrS].FreeCombatX = -1;
      this.game.Data.UnitObj[unrS].FreeCombatY = -1;
      this.game.Data.UnitObj[unrS].DidMove = true;
      this.game.ProcessingObj.MaxReadinessRule(unrS);
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      orderResult.OK = true;
      this.LocationProductionPrognosis();
      return  orderResult;
    }

    pub OrderResult BuyResearch(pplnr: i32, regnr: i32, resnr: i32)
    {
      let mut index1: i32 =  regnr;
      OrderResult orderResult = OrderResult::new();
      let mut num1: i32 =   Math.Round( Conversion.Int( this.game.Data.ResearchObj[resnr].PointCost[this.game.Data.PeopleObj[pplnr].PeopleGroup] * this.game.Data.ResCostMod));
      if (this.game.Data.ResearchObj[resnr].CostType == -1)
      {
        if (num1 > this.game.Data.RegimeObj[regnr].ResPts)
        {
          orderResult.ErrorString = "Not enough Research Pts.";
          orderResult.OK = false;
          return orderResult;
        }
      }
      else if (num1 > this.game.Data.RegimeObj[regnr].RegimeSlot[this.game.Data.ResearchObj[resnr].CostType])
      {
        orderResult.ErrorString = "Not enough Research Pts.";
        orderResult.OK = false;
        return orderResult;
      }
      if (this.game.Data.ResearchObj[resnr].CostType == -1)
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index2: i32 =  regnr;
        let mut index3: i32 =  index2;
        regimeClassArray[index3].ResPts = regimeObj[index2].ResPts - num1;
      }
      else
      {
        int[] regimeSlot = this.game.Data.RegimeObj[regnr].RegimeSlot;
        int[] numArray = regimeSlot;
        let mut costType: i32 =  this.game.Data.ResearchObj[resnr].CostType;
        let mut index4: i32 =  costType;
        let mut num2: i32 =  regimeSlot[costType] - num1;
        numArray[index4] = num2;
      }
      this.game.Data.RegimeObj[index1].ResField[resnr] = true;
      orderResult.OK = true;
      let mut sfTypeCounter: i32 =  this.game.Data.SFTypeCounter;
      for (let mut index5: i32 =  0; index5 <= sfTypeCounter; index5 += 1)
      {
        if (this.game.Data.SFTypeObj[index5].ModelID >= 0 & this.game.Data.SFTypeObj[index5].ModelRegime == index1)
        {
          let mut modelItemType: i32 =  this.game.Data.SFTypeObj[index5].ModelItemType;
          let mut tv9: i32 =  index5;
          let mut tv7: i32 =  0;
          let mut tv8: i32 =  0;
          if (this.game.Data.SFTypeObj[index5].ModelAutoImprovement[resnr] && this.game.Data.SFTypeObj[index5].ModelLastState[resnr] == 0 & this.game.Data.SFTypeObj[index5].ModelImproveEvent[resnr] > 0 && this.game.Data.RegimeObj[index1].ResField[resnr])
          {
            this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.Data.SFTypeObj[index5].ModelImproveEvent[resnr], tv9: tv9, tv7: tv7, tv8: tv8, tv10: modelItemType);
            this.game.Data.SFTypeObj[index5].ModelLastState[resnr] = 1;
          }
        }
      }
      return orderResult;
    }

    pub OrderResult DeclareWar(
      regnr: i32,
      onregnr: i32,
      bool HideMessage = false,
      bool Recurse = false,
      bool conciousChoice = false)
    {
      OrderResult orderResult = OrderResult::new();
      if (!Recurse)
      {
        if ( this.game.Data.RuleVar[818] > 0.0)
        {
          RegimeClass[] regimeObj = this.game.Data.RegimeObj;
          RegimeClass[] regimeClassArray = regimeObj;
          let mut index1: i32 =  regnr;
          let mut index2: i32 =  index1;
          regimeClassArray[index2].ResPts =  Math.Round( ( regimeObj[index1].ResPts - this.game.Data.RuleVar[818]));
          if (0 > this.game.Data.RegimeObj[regnr].ResPts)
            this.game.Data.RegimeObj[regnr].ResPts = 0;
        }
        let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
        for (let mut regnr1: i32 =  0; regnr1 <= regimeCounter1; regnr1 += 1)
        {
          if (this.game.Data.RegimeObj[regnr1].UberRegime == regnr && this.game.Data.RegimeObj[regnr1].RegimeRel[onregnr] != 0)
          {
            this.DeclareWar(regnr1, onregnr, HideMessage, true);
            let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
            for (let mut onregnr1: i32 =  0; onregnr1 <= regimeCounter2; onregnr1 += 1)
            {
              if (this.game.Data.RegimeObj[onregnr1].UberRegime == onregnr && this.game.Data.RegimeObj[regnr1].RegimeRel[onregnr1] != 0)
                this.DeclareWar(regnr1, onregnr1, HideMessage, true);
            }
          }
        }
        let mut regimeCounter3: i32 =  this.game.Data.RegimeCounter;
        for (let mut onregnr2: i32 =  0; onregnr2 <= regimeCounter3; onregnr2 += 1)
        {
          if (this.game.Data.RegimeObj[onregnr2].UberRegime == onregnr && this.game.Data.RegimeObj[regnr].RegimeRel[onregnr2] != 0)
            this.DeclareWar(regnr, onregnr2, HideMessage, true);
        }
      }
      if (this.game.Data.RegimeObj[regnr].RegimeRel[onregnr] == 0)
      {
        orderResult.ErrorString = "Already at war";
        orderResult.OK = false;
        return orderResult;
      }
      this.game.Data.RegimeObj[regnr].RegimeRel[onregnr] = 0;
      this.game.Data.RegimeObj[onregnr].RegimeRel[regnr] = 0;
      this.game.Data.RegimeObj[regnr].RegimeOffer[onregnr] = 0;
      this.game.Data.RegimeObj[onregnr].RegimeOffer[regnr] = 0;
      if (!HideMessage)
      {
        if (this.game.Data.RegimeObj[regnr].RegimeRel[onregnr] == 2)
          this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[regnr].Name + " backstabs its ally " + this.game.Data.RegimeObj[onregnr].Name + " and declares war.", -1, 1);
        else
          this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[regnr].Name + " declares war on " + this.game.Data.RegimeObj[onregnr].Name, -1, 1);
      }
      let mut mapCounter1: i32 =  this.game.Data.MapCounter;
      Number: i32;
      for (let mut map: i32 =  0; map <= mapCounter1; map += 1)
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[map].MapWidth;
        for (let mut x: i32 =  0; x <= mapWidth; x += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[map].MapHeight;
          for (let mut y: i32 =  0; y <= mapHeight; y += 1)
          {
            if (this.game.Data.MapObj[map].HexObj[x, y].Regime != regnr)
            {
              let mut num: i32 =  0;
              if (this.game.Data.MapObj[map].HexObj[x, y].Regime > -1)
              {
                if (this.game.Data.RegimeObj[regnr].RegimeRel[this.game.Data.MapObj[map].HexObj[x, y].Regime] == 0)
                  num = 1;
              }
              else
                num = 1;
              if (num == 1)
              {
                let mut index3: i32 =  -1;
                let mut unitCounter1: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitCounter;
                for (let mut index4: i32 =  0; index4 <= unitCounter1; index4 += 1)
                {
                  let mut unit: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitList[index4];
                  if (regnr != this.game.Data.UnitObj[unit].Regime && this.game.Data.RegimeObj[regnr].RegimeRel[this.game.Data.UnitObj[unit].Regime] == 0)
                    index3 = unit;
                }
                let mut unitCounter2: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitCounter;
                for (let mut index5: i32 =  0; index5 <= unitCounter2; index5 += 1)
                {
                  if (this.game.Data.UnitObj[this.game.Data.MapObj[map].HexObj[x, y].UnitList[index5]].Regime == regnr)
                  {
                    if (this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(x, y, map, regnr, true) & index3 > -1)
                    {
                      this.game.TempCombat = new CombatClass(this.game);
                      Coordinate Target = Coordinate::new();
                      Target.x = x;
                      Target.y = y;
                      Target.map = map;
                      this.game.EditObj.TempUnitList = UnitList::new();
                      let mut unitCounter3: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitCounter;
                      for (let mut index6: i32 =  0; index6 <= unitCounter3; index6 += 1)
                      {
                        let mut unit: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitList[index6];
                        if (regnr == this.game.Data.UnitObj[unit].Regime && this.game.Data.RegimeObj[this.game.Data.UnitObj[unit].Regime].RegimeRel[this.game.Data.UnitObj[index3].Regime] == 0)
                          this.game.EditObj.TempUnitList.add(unit);
                      }
                      this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, 31);
                      this.game.TempCombat.DoBattle();
                      this.game.TempCombat.EndBattle();
                      Number += 1;
                      break;
                    }
                    if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[map].HexObj[x, y].LandscapeType].IsSea)
                      this.game.Data.MapObj[map].HexObj[x, y].Regime = regnr;
                    this += 1.game.Data.StepNr;
                    this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, -1, infostring: "Hex occupied after declaration of war");
                    break;
                  }
                }
              }
            }
            else
            {
              let mut unitCounter4: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitCounter;
              for (let mut index7: i32 =  0; index7 <= unitCounter4; index7 += 1)
              {
                let mut unit1: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitList[index7];
                if (this.game.Data.RegimeObj[regnr].RegimeRel[this.game.Data.UnitObj[unit1].Regime] == 0)
                {
                  if (this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(x, y, map, this.game.Data.UnitObj[unit1].Regime, true))
                  {
                    this.game.TempCombat = new CombatClass(this.game);
                    Coordinate Target = Coordinate::new();
                    Target.x = x;
                    Target.y = y;
                    Target.map = map;
                    this.game.EditObj.TempUnitList = UnitList::new();
                    let mut unitCounter5: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitCounter;
                    for (let mut index8: i32 =  0; index8 <= unitCounter5; index8 += 1)
                    {
                      let mut unit2: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitList[index8];
                      if (regnr != this.game.Data.UnitObj[unit2].Regime && this.game.Data.RegimeObj[this.game.Data.UnitObj[unit2].Regime].RegimeRel[regnr] == 0)
                        this.game.EditObj.TempUnitList.add(unit2);
                    }
                    this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, 31);
                    this.game.TempCombat.DoBattle();
                    this.game.TempCombat.EndBattle();
                    Number += 1;
                    break;
                  }
                  if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[map].HexObj[x, y].LandscapeType].IsSea)
                    this.game.Data.MapObj[map].HexObj[x, y].Regime = this.game.Data.UnitObj[unit1].Regime;
                  this += 1.game.Data.StepNr;
                  this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, -1, infostring: "Hex occupied after declaration of war");
                  break;
                }
              }
            }
          }
        }
      }
      let mut mapCounter2: i32 =  this.game.Data.MapCounter;
      for (let mut map: i32 =  0; map <= mapCounter2; map += 1)
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[map].MapWidth;
        for (let mut x: i32 =  0; x <= mapWidth; x += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[map].MapHeight;
          for (let mut y: i32 =  0; y <= mapHeight; y += 1)
          {
            let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
            for (regnr = 0; regnr <= regimeCounter; regnr += 1)
              this.game.HandyFunctionsObj.DoZOCConquest(x, y, map, regnr);
          }
        }
      }
      orderResult.ErrorString = Number <= 0 ? "" : Strings.Trim(Conversion.Str( Number));
      orderResult.OK = true;
      return orderResult;
    }

    pub OrderResult MakePeace(
      regnr: i32,
      onregnr: i32,
      bool HideMessage = false,
      bool Recurse = false)
    {
      OrderResult orderResult = OrderResult::new();
      if (!Recurse)
      {
        let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
        for (let mut regnr1: i32 =  0; regnr1 <= regimeCounter1; regnr1 += 1)
        {
          if (this.game.Data.RegimeObj[regnr1].UberRegime == regnr && this.game.Data.RegimeObj[regnr1].RegimeRel[onregnr] != 1)
          {
            this.MakePeace(regnr1, onregnr, HideMessage, true);
            let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
            for (let mut onregnr1: i32 =  0; onregnr1 <= regimeCounter2; onregnr1 += 1)
            {
              if (this.game.Data.RegimeObj[onregnr1].UberRegime == onregnr && this.game.Data.RegimeObj[regnr1].RegimeRel[onregnr1] != 1)
                this.MakePeace(regnr1, onregnr1, HideMessage, true);
            }
          }
        }
        let mut regimeCounter3: i32 =  this.game.Data.RegimeCounter;
        for (let mut onregnr2: i32 =  0; onregnr2 <= regimeCounter3; onregnr2 += 1)
        {
          if (this.game.Data.RegimeObj[onregnr2].UberRegime == onregnr && this.game.Data.RegimeObj[regnr].RegimeRel[onregnr2] != 1)
            this.MakePeace(regnr, onregnr2, HideMessage, true);
        }
      }
      if (this.game.Data.RegimeObj[regnr].RegimeRel[onregnr] == 1)
      {
        orderResult.ErrorString = "Already at peace";
        orderResult.OK = false;
        return orderResult;
      }
      if (!HideMessage)
        this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[regnr].Name + " makes peace with " + this.game.Data.RegimeObj[onregnr].Name, -1, 1);
      this.game.Data.RegimeObj[regnr].RegimeRel[onregnr] = 1;
      this.game.Data.RegimeObj[onregnr].RegimeRel[regnr] = 1;
      this.game.Data.RegimeObj[regnr].RegimeOffer[onregnr] = 0;
      this.game.Data.RegimeObj[onregnr].RegimeOffer[regnr] = 0;
      orderResult.OK = true;
      return orderResult;
    }

    pub OrderResult MakeAlliance(
      regnr: i32,
      onregnr: i32,
      bool HideMessage = false,
      bool recurse = false)
    {
      OrderResult orderResult = OrderResult::new();
      if (!recurse)
      {
        let mut regimeCounter1: i32 =  this.game.Data.RegimeCounter;
        for (let mut regnr1: i32 =  0; regnr1 <= regimeCounter1; regnr1 += 1)
        {
          if (this.game.Data.RegimeObj[regnr1].UberRegime == regnr && this.game.Data.RegimeObj[regnr1].RegimeRel[onregnr] != 2)
          {
            this.MakeAlliance(regnr1, onregnr, HideMessage, true);
            let mut regimeCounter2: i32 =  this.game.Data.RegimeCounter;
            for (let mut onregnr1: i32 =  0; onregnr1 <= regimeCounter2; onregnr1 += 1)
            {
              if (this.game.Data.RegimeObj[onregnr1].UberRegime == onregnr && this.game.Data.RegimeObj[regnr1].RegimeRel[onregnr1] != 2)
                this.MakeAlliance(regnr1, onregnr1, HideMessage, true);
            }
          }
        }
        let mut regimeCounter3: i32 =  this.game.Data.RegimeCounter;
        for (let mut onregnr2: i32 =  0; onregnr2 <= regimeCounter3; onregnr2 += 1)
        {
          if (this.game.Data.RegimeObj[onregnr2].UberRegime == onregnr && this.game.Data.RegimeObj[regnr].RegimeRel[onregnr2] != 2)
            this.MakeAlliance(regnr, onregnr2, HideMessage, true);
        }
      }
      if (this.game.Data.RegimeObj[regnr].RegimeRel[onregnr] == 2)
      {
        orderResult.ErrorString = "Already Allied";
        orderResult.OK = false;
        return orderResult;
      }
      if (!HideMessage)
        this.game.HandyFunctionsObj.AddMessageForAll(this.game.Data.RegimeObj[regnr].Name + " makes alliance with " + this.game.Data.RegimeObj[onregnr].Name, -1, 1);
      this.game.Data.RegimeObj[regnr].RegimeRel[onregnr] = 2;
      this.game.Data.RegimeObj[onregnr].RegimeRel[regnr] = 2;
      this.game.Data.RegimeObj[regnr].RegimeOffer[onregnr] = 0;
      this.game.Data.RegimeObj[onregnr].RegimeOffer[regnr] = 0;
      orderResult.OK = true;
      return orderResult;
    }

    pub OrderResult Build(unr: i32, x: i32, y: i32, map: i32, loctype: i32, let mut regnr: i32 =  -1)
    {
      OrderResult orderResult = OrderResult::new();
      orderResult.OK = true;
      if (unr > -1)
      {
        if (regnr == -1)
          regnr = this.game.Data.UnitObj[unr].Regime;
      }
      else
        regnr = this.game.Data.Turn;
      let mut index1: i32 =  -1;
      if (unr > -1)
        index1 = !this.game.Data.UnitObj[unr].IsHQ ? this.game.Data.UnitObj[unr].HQ : unr;
      let mut location: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x, y].Location;
      let mut num1: i32 =  -1;
      if (location > -1)
      {
        num1 = this.game.Data.LocObj[location].People;
        this.game.Data.RemoveLoc(location);
      }
      this.game.Data.AddLoc(x, y, map);
      let mut locCounter: i32 =  this.game.Data.LocCounter;
      this.game.Data.MapObj[map].HexObj[x, y].Location = locCounter;
      if (this.game.Data.LocTypeObj[loctype].OverdrawLTNr > -1)
      {
        this.game.Data.MapObj[map].HexObj[x, y].LandscapeType = this.game.Data.LocTypeObj[loctype].OverdrawLTNr;
        this.game.Data.MapObj[map].HexObj[x, y].SpriteNr = this.game.Data.LocTypeObj[loctype].OverdrawSpriteNr;
      }
      this.game.Data.LocObj[locCounter].Type = loctype;
      this.game.Data.LocObj[locCounter].StructuralPts = this.game.Data.LocTypeObj[loctype].StructuralPts;
      if ( this.game.Data.RuleVar[819] > 0.0)
        this.game.Data.LocObj[locCounter].StructuralPts = 0;
      this.game.Data.LocObj[locCounter].People = unr <= -1 ? this.game.Data.RegimeObj[regnr].People : this.game.Data.RegimeObj[this.game.Data.UnitObj[unr].Regime].People;
      this.game.Data.LocObj[locCounter].HQ = index1;
      if (this.game.Data.LocTypeObj[loctype].NoHQ)
        this.game.Data.LocObj[locCounter].HQ = -1;
      if (this.game.Data.RegimeObj[regnr].AI)
        this.game.Data.LocObj[locCounter].HQ = -1;
      epCost: i32;
      if (!this.game.Data.RegimeObj[regnr].AI)
      {
        if (unr > -1)
        {
          let mut num2: i32 =   Math.Round( ( this.game.Data.LocTypeObj[loctype].SupplyCost / this.game.Data.RuleVar[77]));
          while (index1 > -1 & num2 > 0)
          {
            if (this.game.Data.UnitObj[index1].Supply >= num2)
            {
              UnitClass[] unitObj = this.game.Data.UnitObj;
              UnitClass[] unitClassArray = unitObj;
              let mut index2: i32 =  index1;
              let mut index3: i32 =  index2;
              unitClassArray[index3].Supply = unitObj[index2].Supply - num2;
              num2 = 0;
            }
            else
              index1 = this.game.Data.UnitObj[index1].HQ;
          }
        }
        let mut index4: i32 =  0;
        do
        {
          if (this.game.Data.LocTypeObj[loctype].VarType[index4] > -1)
          {
            int[] regimeSlot = this.game.Data.RegimeObj[regnr].RegimeSlot;
            int[] numArray1 = regimeSlot;
            int[] varType = this.game.Data.LocTypeObj[loctype].VarType;
            int[] numArray2 = varType;
            let mut index5: i32 =  index4;
            let mut index6: i32 =  index5;
            let mut index7: i32 =  numArray2[index6];
            let mut num3: i32 =  regimeSlot[varType[index5]] - this.game.Data.LocTypeObj[loctype].VarQty[index4];
            numArray1[index7] = num3;
          }
          index4 += 1;
        }
        while (index4 <= 4);
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index8: i32 =  regnr;
        let mut index9: i32 =  index8;
        regimeClassArray[index9].ResPts = regimeObj[index8].ResPts - this.game.Data.LocTypeObj[loctype].PPCost;
        epCost = this.game.Data.LocTypeObj[loctype].EPCost;
      }
      this.game.Data.LocObj[this.game.Data.LocCounter].Name = this.game.Data.LocTypeObj[loctype].Name;
      if (this.game.Data.LocTypeObj[loctype].SetPeopleToSlotX > -1)
      {
        let mut num4: i32 =  this.game.Data.MapObj[0].HexObj[x, y].AreaCode[this.game.Data.LocTypeObj[loctype].SetPeopleToSlotX];
        if (num4 > -1 & num4 <= this.game.Data.PeopleCounter)
          this.game.Data.LocObj[this.game.Data.LocCounter].People = num4;
      }
      if (num1 > -1)
        this.game.Data.LocObj[this.game.Data.LocCounter].People = num1;
      if (unr > -1)
      {
        let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
        num5: i32;
        for (let mut index10: i32 =  0; index10 <= sfCount1; index10 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index10];
          if (this.game.Data.SFObj[sf].EP > 0)
          {
            SFClass[] sfObj = this.game.Data.SFObj;
            SFClass[] sfClassArray = sfObj;
            let mut index11: i32 =  sf;
            let mut index12: i32 =  index11;
            sfClassArray[index12].Ap = sfObj[index11].Ap - epCost;
            if (0 > this.game.Data.SFObj[sf].Ap)
              this.game.Data.SFObj[sf].Ap = 0;
            num5 += this.game.Data.SFObj[sf].EP;
          }
        }
        float num6 =  epCost /  num5;
        if ( num6 > 1.0)
          num6 = 1f;
        let mut sfCount2: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index13: i32 =  0; index13 <= sfCount2; index13 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index13];
          SFClass[] sfObj = this.game.Data.SFObj;
          SFClass[] sfClassArray = sfObj;
          let mut index14: i32 =  sf;
          let mut index15: i32 =  index14;
          sfClassArray[index15].EP =  Math.Round( ( sfObj[index14].EP -  this.game.Data.SFObj[sf].EP * num6));
        }
      }
      let mut unitCounter: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitCounter;
      for (let mut index16: i32 =  0; index16 <= unitCounter; index16 += 1)
      {
        let mut unit: i32 =  this.game.Data.MapObj[map].HexObj[x, y].UnitList[index16];
        let mut sfCount: i32 =  this.game.Data.UnitObj[unit].SFCount;
        for (let mut index17: i32 =  0; index17 <= sfCount; index17 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unit].SFList[index17];
          if (this.game.Data.SFObj[sf].CurrentEntrench < this.game.HandyFunctionsObj.GetMinimumEntrench(x, y, map, this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup))
          {
            this.game.Data.SFObj[sf].CurrentEntrench = this.game.HandyFunctionsObj.GetMinimumEntrench(x, y, map, this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].UnitGroup);
            this.game.Data.SFObj[sf].initialEntrench = this.game.Data.SFObj[sf].CurrentEntrench;
          }
        }
      }
      if (this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.LocCounter].Type].AutoProd > -1)
      {
        this.game.Data.LocObj[this.game.Data.LocCounter].Production[0] = this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.LocCounter].Type].AutoProd;
        this.game.Data.LocObj[this.game.Data.LocCounter].ProdPercent[0] = 100;
        this.game.Data.LocObj[this.game.Data.LocCounter].Production[1] = -1;
        this.game.Data.LocObj[this.game.Data.LocCounter].Production[2] = -1;
        this.game.Data.LocObj[this.game.Data.LocCounter].Production[3] = -1;
        this.game.Data.LocObj[this.game.Data.LocCounter].ProdPercent[1] = 0;
        this.game.Data.LocObj[this.game.Data.LocCounter].ProdPercent[2] = 0;
        this.game.Data.LocObj[this.game.Data.LocCounter].ProdPercent[3] = 0;
        this.game.HandyFunctionsObj.UpgradeProduction(this.game.Data.LocCounter);
      }
      this.LocationProductionPrognosis();
      return orderResult;
    }

    pub OrderResult RepairLocation(unr: i32, x: i32, y: i32, map: i32)
    {
      OrderResult orderResult = OrderResult::new();
      orderResult.OK = true;
      let mut location: i32 =  this.game.Data.MapObj[map].HexObj[x, y].Location;
      let mut type: i32 =  this.game.Data.LocObj[location].Type;
      float num1 =  (1.0 -  this.game.Data.LocObj[location].StructuralPts /  this.game.Data.LocTypeObj[type].StructuralPts);
      let mut num2: i32 =   Math.Round( ( this.game.Data.LocTypeObj[type].EPCost * num1));
      if (num2 == 0)
        num2 = 1;
      let mut num3: i32 =  this.game.HandyFunctionsObj.GetUnitEP(unr);
      if (num3 > num2)
        num3 = num2;
      float num4 =  num3 /  num2;
      let mut Number: i32 =   Math.Round( ( (this.game.Data.LocTypeObj[type].StructuralPts - this.game.Data.LocObj[location].StructuralPts) * num4));
      editObj: EditClass = this.game.EditObj;
      editObj.FeedBackString = editObj.FeedBackString + "We repaired " + Conversion.Str( Number) + " structural points of dammage.";
      LocationClass[] locObj = this.game.Data.LocObj;
      LocationClass[] locationClassArray = locObj;
      let mut index1: i32 =  location;
      let mut index2: i32 =  index1;
      locationClassArray[index2].StructuralPts = locObj[index1].StructuralPts + Number;
      if (this.game.Data.LocObj[location].StructuralPts > this.game.Data.LocTypeObj[type].StructuralPts)
        this.game.Data.LocObj[location].StructuralPts = this.game.Data.LocTypeObj[type].StructuralPts;
      let mut num5: i32 =  num3;
      let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
      num6: i32;
      for (let mut index3: i32 =  0; index3 <= sfCount1; index3 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index3];
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        let mut index4: i32 =  sf;
        let mut index5: i32 =  index4;
        sfClassArray[index5].Ap = sfObj[index4].Ap - num5;
        if (0 > this.game.Data.SFObj[sf].Ap)
          this.game.Data.SFObj[sf].Ap = 0;
        num6 += this.game.Data.SFObj[sf].EP;
      }
      float num7 = num6 != 0 ?  num5 /  num6 : 0.0f;
      if ( num7 > 1.0)
        num7 = 1f;
      let mut sfCount2: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index6: i32 =  0; index6 <= sfCount2; index6 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index6];
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        let mut index7: i32 =  sf;
        let mut index8: i32 =  index7;
        sfClassArray[index8].EP =  Math.Round( ( sfObj[index7].EP -  this.game.Data.SFObj[sf].EP * num7));
      }
      this.LocationProductionPrognosis();
      return orderResult;
    }

    pub object SetInitialPeek(regnr: i32)
    {
      let mut mapCounter: i32 =  this.game.Data.MapCounter;
      for (let mut index1: i32 =  0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 =  this.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 =  0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 =  0; index3 <= mapHeight; index3 += 1)
          {
            this.game.Data.MapObj[index1].HexObj[index2, index3].set_LastLT(regnr, this.game.Data.MapObj[index1].HexObj[index2, index3].LandscapeType);
            if (this.game.Data.Product != 7 | !this.game.Data.ShrowdOn)
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_LastReg(regnr, this.game.Data.MapObj[index1].HexObj[index2, index3].Regime);
            else
              this.game.Data.MapObj[index1].HexObj[index2, index3].set_LastReg(regnr, -1);
            this.game.Data.MapObj[index1].HexObj[index2, index3].set_LastSpr(regnr, this.game.Data.MapObj[index1].HexObj[index2, index3].SpriteNr);
          }
        }
      }
      object obj;
      return obj;
    }

    pub OrderResult BuildInfra(unr: i32, x: i32, y: i32, map: i32, facing: i32)
    {
      OrderResult orderResult = OrderResult::new();
      orderResult.OK = true;
      Coordinate coordinate1 = this.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, map, facing + 1);
      let mut landscapeType: i32 =  this.game.Data.MapObj[map].HexObj[coordinate1.x, coordinate1.y].LandscapeType;
      Number: i32;
      bool flag1;
      if (this.game.Data.MapObj[map].HexObj[x, y].RiverType[facing] > -1)
      {
        Number =  Math.Round( (this.game.Data.LandscapeTypeObj[landscapeType].RoadCostModifier *  this.game.Data.BridgeObj[0].EPCost * this.game.Data.RiverTypeObj[this.game.Data.MapObj[map].HexObj[x, y].RiverType[facing]].BridgeCostModifier));
        flag1 = true;
      }
      bool flag2;
      if (this.game.Data.MapObj[map].HexObj[x, y].RoadType[facing] == -1)
      {
        if (Number == 0)
          Number =  Math.Round( (this.game.Data.LandscapeTypeObj[landscapeType].RoadCostModifier *  this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[32])].EPCost));
        flag2 = true;
      }
      else if ( this.game.Data.RuleVar[821] ==  this.game.Data.MapObj[map].HexObj[x, y].RoadType[facing] &  this.game.Data.RuleVar[820] > -1.0)
      {
        if (Number == 0)
          Number =  Math.Round( (this.game.Data.LandscapeTypeObj[landscapeType].RoadCostModifier *  this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[820])].EPCost));
        flag2 = true;
      }
      Coordinate coordinate2 = this.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, map, facing + 1);
      Coordinate coordinate3 = this.game.HandyFunctionsObj.MoveApCostPreview(unr, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, x, y, map, coordinate2.x, coordinate2.y, coordinate2.map, OnlyEngineer: 1);
      let mut x1: i32 =  coordinate3.x;
      if (flag1)
      {
        let mut index1: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate2.x, coordinate2.y, coordinate2.map, x, y, map) - 1;
        this.game.Data.MapObj[map].HexObj[x, y].Bridge[facing] = true;
        this.game.Data.MapObj[map].HexObj[coordinate2.x, coordinate2.y].Bridge[index1] = true;
        if ( this.game.Data.RuleVar[822] > -1.0)
        {
          int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
          int[] numArray = regimeSlot;
          let mut index2: i32 =   Math.Round( this.game.Data.RuleVar[822]);
          let mut index3: i32 =  index2;
          let mut num: i32 =   Math.Round( ( regimeSlot[index2] - this.game.Data.RuleVar[825] * this.game.Data.RiverTypeObj[this.game.Data.MapObj[map].HexObj[x, y].RiverType[facing]].BridgeCostModifier));
          numArray[index3] = num;
        }
      }
      if (flag2)
      {
        let mut index4: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate2.x, coordinate2.y, coordinate2.map, x, y, map) - 1;
        if (this.game.Data.MapObj[map].HexObj[x, y].RoadType[facing] == -1 | flag1)
        {
          if (this.game.Data.MapObj[map].HexObj[x, y].RoadType[facing] == -1)
          {
            this.game.Data.MapObj[map].HexObj[x, y].RoadType[facing] =  Math.Round( this.game.Data.RuleVar[32]);
            this.game.Data.MapObj[map].HexObj[coordinate2.x, coordinate2.y].RoadType[index4] =  Math.Round( this.game.Data.RuleVar[32]);
          }
          if ( this.game.Data.RuleVar[822] > -1.0)
          {
            int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
            int[] numArray = regimeSlot;
            let mut index5: i32 =   Math.Round( this.game.Data.RuleVar[822]);
            let mut index6: i32 =  index5;
            let mut num: i32 =   Math.Round( ( regimeSlot[index5] - this.game.Data.RuleVar[823]));
            numArray[index6] = num;
          }
        }
        else
        {
          this.game.Data.MapObj[map].HexObj[x, y].RoadType[facing] =  Math.Round( this.game.Data.RuleVar[820]);
          this.game.Data.MapObj[map].HexObj[coordinate2.x, coordinate2.y].RoadType[index4] =  Math.Round( this.game.Data.RuleVar[820]);
          if ( this.game.Data.RuleVar[822] > -1.0 & !flag1)
          {
            int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
            int[] numArray = regimeSlot;
            let mut index7: i32 =   Math.Round( this.game.Data.RuleVar[822]);
            let mut index8: i32 =  index7;
            let mut num: i32 =   Math.Round( ( regimeSlot[index7] - this.game.Data.RuleVar[824]));
            numArray[index8] = num;
          }
        }
      }
      coordinate3 = this.game.HandyFunctionsObj.MoveApCostPreview(unr, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, x, y, map, coordinate2.x, coordinate2.y, coordinate2.map, OnlyEngineer: 2);
      let mut x2: i32 =  coordinate3.x;
      let mut sfCount1: i32 =  this.game.Data.UnitObj[unr].SFCount;
      num1: i32;
      for (let mut index9: i32 =  0; index9 <= sfCount1; index9 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index9];
        if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].EP > 0)
        {
          SFClass[] sfObj = this.game.Data.SFObj;
          SFClass[] sfClassArray = sfObj;
          let mut index10: i32 =  sf;
          let mut index11: i32 =  index10;
          sfClassArray[index11].Ap = sfObj[index10].Ap - x1;
        }
        else
        {
          SFClass[] sfObj = this.game.Data.SFObj;
          SFClass[] sfClassArray = sfObj;
          let mut index12: i32 =  sf;
          let mut index13: i32 =  index12;
          sfClassArray[index13].Ap = sfObj[index12].Ap - x2;
        }
        if (this.game.Data.SFObj[sf].Ap < 0)
          this.game.Data.SFObj[sf].Ap = 0;
        num1 += this.game.Data.SFObj[sf].EP;
      }
      float num2 =  Number /  num1;
      if ( num2 > 1.0)
        num2 = 1f;
      let mut sfCount2: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index14: i32 =  0; index14 <= sfCount2; index14 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index14];
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        let mut index15: i32 =  sf;
        let mut index16: i32 =  index15;
        sfClassArray[index16].EP =  Math.Round( ( sfObj[index15].EP -  this.game.Data.SFObj[sf].EP * num2));
      }
      if ( this.game.Data.RuleVar[483] > 0.0 & this.game.Data.Product >= 6)
      {
        this += 1.game.Data.StepNr;
        infostring: String = this.game.Data.UnitObj[unr].Name + " Constructs road...";
        if (flag1)
          infostring = this.game.Data.UnitObj[unr].Name + " Constructs bridge...";
        this.game.HandyFunctionsObj.HistoryAddHex(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, map, this.game.Data.Turn, 2, 0, unr, infostring: infostring);
      }
      else
      {
        this.game.Data.MapObj[map].HexObj[x, y].RemoveUnitFromList(unr);
        this += 1.game.Data.StepNr;
        infostring1: String = this.game.Data.UnitObj[unr].Name + " Constructs road...";
        if (flag1)
          infostring1 = this.game.Data.UnitObj[unr].Name + " Constructs bridge...";
        this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, this.game.Data.Turn, 2, 0, unr, infostring: infostring1);
        this.game.Data.MapObj[map].HexObj[coordinate2.x, coordinate2.y].AddUnitToList(unr);
        this.game.Data.UnitObj[unr].X = coordinate2.x;
        this.game.Data.UnitObj[unr].Y = coordinate2.y;
        infostring2: String = "";
        this.game.HandyFunctionsObj.HistoryAddHex(coordinate2.x, coordinate2.y, coordinate2.map, this.game.Data.Turn, 2, 0, unr, infostring: infostring2);
        this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(x, y, map, coordinate2.x, coordinate2.y, coordinate2.map, this.game.Data.UnitObj[unr].Regime, unr, dontcountair: true);
      }
      this.game.EditObj.FeedBackString = "";
      if (flag1)
      {
        str: String = x1.ToString();
        if (x1 >= 999)
          str = "all";
        if (this.game.Data.Product >= 6)
          this.game.EditObj.FeedBackString = "Bridge constructed! Cost was " + Strings.Trim(Conversion.Str( Number)) + " engineer points.";
        else
          this.game.EditObj.FeedBackString = "Bridge constructed! Cost was " + Strings.Trim(Conversion.Str( x1)) + " action points and " + Strings.Trim(Conversion.Str( Number)) + " engineer points.";
      }
      if (this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest > -1)
      {
        this.game.EditObj.AreaX = x;
        this.game.EditObj.AreaY = y;
        this.game.EditObj.AreaMap = map;
        if (this.game.Data.RegimeObj[this.game.Data.Turn].AI)
          this.PlayCard(this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest);
        else
          this.game.EditObj.DoCardSlot = this.game.Data.MapObj[map].HexObj[x, y].CardUponConquest;
      }
      if ( this.game.Data.RuleVar[843] > 0.0)
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round( this.game.Data.RuleVar[843]));
      return orderResult;
    }

    pub OrderResult BlowBridge(unr: i32, x: i32, y: i32, map: i32, facing: i32)
    {
      OrderResult orderResult = OrderResult::new();
      Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(x, y, map, facing + 1);
      let mut index1: i32 =  this.game.HandyFunctionsObj.HexFacing(coordinate.x, coordinate.y, coordinate.map, x, y, map) - 1;
      let mut num1: i32 =  Conversion.Int(this.game.HandyFunctionsObj.GetBlowBridgePts(unr));
      let mut num2: i32 =   Math.Round( Conversion.Int(this.game.Data.RuleVar[7] * this.game.Data.RiverTypeObj[this.game.Data.MapObj[map].HexObj[x, y].RiverType[facing]].BridgeCostModifier));
      this.game.EditObj.FeedBackString = "";
      let mut num3: i32 =  0;
      if (this.game.Data.Product >= 6 & this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime != this.game.Data.Turn & this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].Regime > -1)
      {
        let mut num4: i32 =  0;
        let mut unitCounter1: i32 =  this.game.Data.MapObj[0].HexObj[x, y].UnitCounter;
        for (let mut index2: i32 =  0; index2 <= unitCounter1; index2 += 1)
          num4 += this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.Data.MapObj[0].HexObj[x, y].UnitList[index2]);
        let mut num5: i32 =  0;
        let mut unitCounter2: i32 =  this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitCounter;
        for (let mut index3: i32 =  0; index3 <= unitCounter2; index3 += 1)
          num5 += this.game.HandyFunctionsObj.GetPowerPtsAbsolute(this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].UnitList[index3]);
        num3 = 0;
        let mut num6: i32 =  num4 <= 0 ? 1000 :  Math.Round( (100 * num5) /  num4);
        if (num6 > 1000)
          num6 = 1000;
        if (num6 > 0)
        {
          num2 +=  Math.Round( (num2 * num6) / 100.0);
          this.game.EditObj.FeedBackString = "Due to enemy troops on opposite side of the bridge def score increased with +" + num6.ToString() + "%. ";
        }
      }
      let mut Number1: i32 =  num1;
      let mut Number2: i32 =  num2;
      let mut Number3: i32 =   Math.Round( Conversion.Int( num1 * VBMath.Rnd()));
      let mut Number4: i32 =   Math.Round( Conversion.Int( num2 * VBMath.Rnd()));
      editObj: EditClass = this.game.EditObj;
      editObj.FeedBackString = editObj.FeedBackString + "Blow attempt score = " + Conversion.Str( Number3) + " (max " + Strings.Trim(Conversion.Str( Number1)) + "), Bridge def score = " + Conversion.Str( Number4) + " (max " + Strings.Trim(Conversion.Str( Number2)) + ")";
      if (Number3 >= Number4)
      {
        this.game.Data.MapObj[map].HexObj[x, y].Bridge[facing] = false;
        this.game.Data.MapObj[map].HexObj[coordinate.x, coordinate.y].Bridge[index1] = false;
        let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index4: i32 =  0; index4 <= sfCount; index4 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index4];
          SFClass[] sfObj = this.game.Data.SFObj;
          SFClass[] sfClassArray = sfObj;
          let mut index5: i32 =  sf;
          let mut index6: i32 =  index5;
          sfClassArray[index6].Ap = sfObj[index5].Ap - 50;
          this.game.Data.SFObj[sf].EP = 0;
          if (0 > this.game.Data.SFObj[sf].Ap)
            this.game.Data.SFObj[sf].Ap = 0;
        }
        this.game.EditObj.FeedBackString += ". Bridge destroyed.";
        orderResult.OK = true;
        this += 1.game.Data.StepNr;
        this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, this.game.Data.UnitObj[unr].Regime, infostring: "Bridge is blown.");
      }
      else
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index7: i32 =  0; index7 <= sfCount; index7 += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index7];
          SFClass[] sfObj = this.game.Data.SFObj;
          SFClass[] sfClassArray = sfObj;
          let mut index8: i32 =  sf;
          let mut index9: i32 =  index8;
          sfClassArray[index9].Ap = sfObj[index8].Ap - 50;
          this.game.Data.SFObj[sf].EP = 0;
          if (0 > this.game.Data.SFObj[sf].Ap)
            this.game.Data.SFObj[sf].Ap = 0;
        }
        this.game.EditObj.FeedBackString += ". Bridge survives.";
        orderResult.OK = false;
        this += 1.game.Data.StepNr;
        this.game.HandyFunctionsObj.HistoryAddHex(x, y, map, this.game.Data.UnitObj[unr].Regime, infostring: "Failed attempt to blow bridge.");
      }
      return orderResult;
    }

    pub fn MaxReadinessRule(unr: i32)
    {
      let mut x: i32 =  this.game.Data.UnitObj[unr].X;
      let mut y: i32 =  this.game.Data.UnitObj[unr].Y;
      let mut map: i32 =  this.game.Data.UnitObj[unr].Map;
      if (x == -1 | y == -1)
        return;
      let mut location: i32 =  this.game.Data.MapObj[map].HexObj[x, y].Location;
      if (location <= -1)
        return;
      let mut type1: i32 =  this.game.Data.LocObj[location].Type;
      let mut num1: i32 =  this.game.Data.LocObj[location].StructuralPts;
      let mut structuralPts: i32 =  this.game.Data.LocTypeObj[type1].StructuralPts;
      if (this.game.Data.Product >= 6 && this.game.Data.LocTypeObj[type1].Invincible)
        num1 = structuralPts;
      let mut num2: i32 =  structuralPts <= 0 ? 100 :  Math.Round(Conversion.Int(100.0 * ( num1 /  structuralPts)));
      if ( num2 <  this.game.Data.RuleVar[60])
        num2 =  Math.Round( this.game.Data.RuleVar[60]);
      if (num2 < 100)
      {
        let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
        for (let mut index: i32 =  0; index <= sfCount; index += 1)
        {
          let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index];
          let mut type2: i32 =  this.game.Data.SFObj[sf].Type;
          if (this.game.Data.LocTypeObj[type1].IsPort && this.game.Data.SFTypeObj[type2].Theater == 1 && this.game.Data.SFObj[sf].Rdn > num2)
            this.game.Data.SFObj[sf].Rdn = num2;
          if (this.game.Data.LocTypeObj[type1].IsAirfield | this.game.Data.LocObj[location].isAirfield && this.game.Data.SFTypeObj[type2].Theater == 2 && this.game.Data.SFObj[sf].Rdn > num2)
            this.game.Data.SFObj[sf].Rdn = num2;
        }
      }
      if ( num2 >=  this.game.Data.RuleVar[60])
        return;
      let mut num3: i32 =   Math.Round( this.game.Data.RuleVar[60]);
    }

    pub OrderResult BlowLocation(unr: i32, x: i32, y: i32, map: i32)
    {
      OrderResult orderResult = OrderResult::new();
      Conversion.Int(this.game.HandyFunctionsObj.GetBlowBridgePts(unr));
      let mut location: i32 =  this.game.Data.MapObj[map].HexObj[x, y].Location;
      let mut type: i32 =  this.game.Data.LocObj[location].Type;
      let mut blowBridgePts: i32 =  this.game.HandyFunctionsObj.GetBlowBridgePts(unr);
      if (!this.game.Data.LocTypeObj[type].Invincible)
      {
        let mut Number: i32 =   Math.Round( Conversion.Int( blowBridgePts * VBMath.Rnd()));
        this.game.EditObj.FeedBackString = "Loc has " + Conversion.Str( this.game.Data.LocObj[location].StructuralPts) + "pts. Dam done to loc: " + Strings.Trim(Conversion.Str( Number)) + ".";
        LocationClass[] locObj = this.game.Data.LocObj;
        LocationClass[] locationClassArray = locObj;
        let mut index1: i32 =  location;
        let mut index2: i32 =  index1;
        locationClassArray[index2].StructuralPts = locObj[index1].StructuralPts - Number;
        if (1 > this.game.Data.LocObj[location].StructuralPts)
        {
          if (this.game.Data.LocTypeObj[type].OnDestructLT > -1)
          {
            this.game.EditObj.FeedBackString += ". Loc destroyed!";
            this.game.Data.MapObj[map].HexObj[this.game.Data.LocObj[location].X, this.game.Data.LocObj[location].Y].LandscapeType = this.game.Data.LocTypeObj[type].OnDestructLT;
            this.game.Data.MapObj[map].HexObj[this.game.Data.LocObj[location].X, this.game.Data.LocObj[location].Y].SpriteNr = this.game.Data.LocTypeObj[type].OnDestructSpriteNr;
            this.game.Data.RemoveLoc(location);
          }
          else if (this.game.Data.LocTypeObj[type].OnDestructLT == -2)
          {
            this.game.EditObj.FeedBackString += ". Loc destroyed!";
            this.game.Data.RemoveLoc(location);
          }
          else
          {
            this.game.EditObj.FeedBackString += ". Loc is knocked out!";
            this.game.Data.LocObj[location].StructuralPts = 0;
          }
        }
        else
        {
          editObj: EditClass = this.game.EditObj;
          editObj.FeedBackString = editObj.FeedBackString + ". Loc down to " + Strings.Trim(Conversion.Str( this.game.Data.LocObj[location].StructuralPts)) + " pts.";
        }
      }
      let mut sfCount: i32 =  this.game.Data.UnitObj[unr].SFCount;
      for (let mut index3: i32 =  0; index3 <= sfCount; index3 += 1)
      {
        let mut sf: i32 =  this.game.Data.UnitObj[unr].SFList[index3];
        SFClass[] sfObj = this.game.Data.SFObj;
        SFClass[] sfClassArray = sfObj;
        let mut index4: i32 =  sf;
        let mut index5: i32 =  index4;
        sfClassArray[index5].Ap = sfObj[index4].Ap - 50;
        this.game.Data.SFObj[sf].EP = 0;
        if (0 > this.game.Data.SFObj[sf].Ap)
          this.game.Data.SFObj[sf].Ap = 0;
      }
      this.LocationProductionPrognosis();
      orderResult.OK = true;
      return orderResult;
    }

    pub fn CheckForWinner()
    {
      if (this.game.Data.Winner > -1 || this.game.Data.VPWin == -1 || this.game.Data.PbemDrawGame > 0)
        return;
      let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
      for (let mut regnr: i32 =  0; regnr <= regimeCounter; regnr += 1)
      {
        if (this.game.HandyFunctionsObj.GetRegimeVP(regnr) >= this.game.Data.VPWin)
        {
          this.game.Data.Winner = regnr;
          break;
        }
      }
    }

    pub fn DoAutoRecoverLocations(regnr: i32)
    {
      if (this.game.Data.LocCounter == -1)
        return;
      let mut locCounter: i32 =  this.game.Data.LocCounter;
      for (let mut index1: i32 =  0; index1 <= locCounter; index1 += 1)
      {
        if (this.game.Data.MapObj[this.game.Data.LocObj[index1].Map].HexObj[this.game.Data.LocObj[index1].X, this.game.Data.LocObj[index1].Y].Regime == regnr)
        {
          let mut type: i32 =  this.game.Data.LocObj[index1].Type;
          bool flag = true;
          if (this.game.Data.LocTypeObj[type].isSupplyBase && this.game.Data.LocObj[index1].supplyBaseMode >= 3)
            flag = false;
          if (flag && this.game.Data.LocObj[index1].StructuralPts < this.game.Data.LocTypeObj[type].StructuralPts)
          {
            LocationClass[] locObj1 = this.game.Data.LocObj;
            LocationClass[] locationClassArray1 = locObj1;
            let mut index2: i32 =  index1;
            let mut index3: i32 =  index2;
            locationClassArray1[index3].StructuralPts = locObj1[index2].StructuralPts + this.game.Data.LocTypeObj[type].AutoRecoverPts;
            if (this.game.Data.MapObj[this.game.Data.LocObj[index1].Map].HexObj[this.game.Data.LocObj[index1].X, this.game.Data.LocObj[index1].Y].Regime > -1 && this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.Data.LocObj[index1].Map].HexObj[this.game.Data.LocObj[index1].X, this.game.Data.LocObj[index1].Y].Regime].AI &&  this.game.Data.RuleVar[250] == 1.0)
            {
              LocationClass[] locObj2 = this.game.Data.LocObj;
              LocationClass[] locationClassArray2 = locObj2;
              let mut index4: i32 =  index1;
              let mut index5: i32 =  index4;
              locationClassArray2[index5].StructuralPts =  Math.Round( locObj2[index4].StructuralPts +  this.game.Data.LocTypeObj[type].StructuralPts * 0.33);
            }
            if (this.game.Data.LocObj[index1].StructuralPts > this.game.Data.LocTypeObj[type].StructuralPts)
              this.game.Data.LocObj[index1].StructuralPts = this.game.Data.LocTypeObj[type].StructuralPts;
          }
          if (this.game.Data.LocTypeObj[type].isSupplyBase)
          {
            if (this.game.Data.LocObj[index1].supplyBaseMode >= 3)
            {
              this.game.Data.LocObj[index1].StructuralPts = 0;
            }
            else
            {
              let mut num: i32 =   Math.Round(Math.Floor( this.game.Data.LocTypeObj[type].StructuralPts * Math.Max( this.game.Data.LocObj[index1].supply /  this.game.Data.LocTypeObj[type].maxSupply,  this.game.Data.LocObj[index1].fuel /  this.game.Data.LocTypeObj[type].maxFuel)));
              if (num < this.game.Data.LocObj[index1].StructuralPts)
                this.game.Data.LocObj[index1].StructuralPts = num;
            }
          }
          this.game.Data.LocObj[index1].startTurnStructuralPts = this.game.Data.LocObj[index1].StructuralPts;
        }
      }
      num1: i32;
      for (let mut index: i32 =  0; index <= -1; index += 1)
        num1 = 1;
      num1 = 0;
    }

    pub OrderResult DoDisband(unr: i32, sfnr: i32)
    {
      OrderResult orderResult1 = OrderResult::new();
      orderResult1.OK = true;
      orderResult1.ErrorString = "";
      if ( this.game.Data.RuleVar[861] > 0.0)
      {
        let mut type: i32 =  this.game.Data.SFObj[sfnr].Type;
        let mut itemTypeCounter: i32 =  this.game.Data.ItemTypeCounter;
        for (let mut index1: i32 =  0; index1 <= itemTypeCounter; index1 += 1)
        {
          if (this.game.Data.ItemTypeObj[index1].IsSFType == type)
          {
            let mut index2: i32 =  0;
            do
            {
              if (this.game.Data.ItemTypeObj[index1].RegimeSlotsCost[index2] > -1)
              {
                num1: i32;
                num1 += 1;
                let mut index3: i32 =  this.game.Data.ItemTypeObj[index1].RegimeSlotsCost[index2];
                let mut Number: i32 =   Math.Round( ( (this.game.Data.ItemTypeObj[index1].RegimeSlotsCostQty[index2] * this.game.Data.SFObj[sfnr].Qty) * this.game.Data.RuleVar[861]));
                int[] regimeSlot = this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot;
                int[] numArray = regimeSlot;
                let mut index4: i32 =  index3;
                let mut index5: i32 =  index4;
                let mut num2: i32 =  regimeSlot[index4] + Number;
                numArray[index5] = num2;
                if (num1 == 1)
                  orderResult1.ErrorString = "Disbanding provided the following recycled resources: ";
                if (num1 > 1)
                  orderResult1.ErrorString += ", ";
                OrderResult orderResult2 = orderResult1;
                orderResult2.ErrorString = orderResult2.ErrorString + Strings.Trim(Conversion.Str( Number)) + "x " + this.game.Data.RegimeSlotName[index3];
              }
              index2 += 1;
            }
            while (index2 <= 4);
            break;
          }
        }
      }
      this.LocationProductionPrognosis();
      this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, -1, -1, -1, this.game.Data.UnitObj[unr].Regime, unr, false);
      this.game.Data.RemoveSF(sfnr);
      this.game.HandyFunctionsObj.SetHexReconAndZOCUnitMoves(-1, -1, -1, this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y, this.game.Data.UnitObj[unr].Map, this.game.Data.UnitObj[unr].Regime, unr, false);
      return orderResult1;
    }

    pub fn DoDisbandUnit(unr: i32)
    {
      let mut x: i32 =  this.game.Data.UnitObj[unr].X;
      let mut y: i32 =  this.game.Data.UnitObj[unr].Y;
      if (this.game.Data.UnitObj[unr].IsHQ)
      {
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
        {
          if (this.game.Data.UnitObj[index1].SFCount > -1 && this.game.Data.UnitObj[index1].HQ == unr && !this.game.Data.UnitObj[index1].IsHQ)
          {
            let mut sfCount: i32 =  this.game.Data.UnitObj[index1].SFCount;
            for (let mut index2: i32 =  0; index2 <= sfCount; index2 += 1)
            {
              let mut sf: i32 =  this.game.Data.UnitObj[index1].SFList[index2];
              if (this.game.Data.SFTypeObj[this.game.Data.SFObj[sf].Type].Theater < 1)
                this.game.Data.SFObj[sf].Rdn =  Math.Round( ( this.game.Data.SFObj[sf].Rdn * this.game.Data.RuleVar[48]));
            }
          }
        }
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut turn: i32 =  this.game.Data.Turn;
        let mut index: i32 =  turn;
        regimeClassArray[index].ResPts =  Math.Round( ( regimeObj[turn].ResPts + this.game.Data.RuleVar[47]));
      }
      else
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut turn: i32 =  this.game.Data.Turn;
        let mut index: i32 =  turn;
        regimeClassArray[index].ResPts =  Math.Round( ( regimeObj[turn].ResPts + this.game.Data.RuleVar[46]));
      }
      this.game.Data.RemoveUnit(unr,  this.game);
    }

    pub OrderResult DoUpgrade(unr: i32, sfnr: i32, qty: i32, hq: i32)
    {
      OrderResult orderResult = OrderResult::new();
      if (qty == 0)
        return orderResult;
      orderResult.OK = true;
      let mut upgradeToo: i32 =  this.game.Data.SFTypeObj[this.game.Data.SFObj[sfnr].Type].UpgradeToo;
      this.game.HandyFunctionsObj.AddTroops3(unr, upgradeToo, this.game.Data.SFObj[sfnr].People, qty, this.game.Data.SFObj[sfnr].Xp, this.game.Data.SFObj[sfnr].Rdn, 0, this.game.Data.SFObj[sfnr].Mor, entr: this.game.Data.SFObj[sfnr].CurrentEntrench, offmod: this.game.Data.SFObj[sfnr].OffMod, defmod: this.game.Data.SFObj[sfnr].DefMod, MoveType: this.game.Data.SFObj[sfnr].MoveType);
      this.game.HandyFunctionsObj.RemoveTroops(unr, this.game.Data.SFObj[sfnr].Type, this.game.Data.SFObj[sfnr].People, qty, this.game.Data.SFObj[sfnr].MoveType);
      let mut num: i32 =  this.game.HandyFunctionsObj.CanUpgradeCost(sfnr, unr, qty);
      if (hq > -1)
      {
        UnitClass[] unitObj = this.game.Data.UnitObj;
        UnitClass[] unitClassArray = unitObj;
        let mut index1: i32 =  hq;
        let mut index2: i32 =  index1;
        unitClassArray[index2].Supply = unitObj[index1].Supply - num;
      }
      return orderResult;
    }

    pub OrderResult DoMakeHQ(unr: i32)
    {
      let mut regime: i32 =  this.game.Data.UnitObj[unr].Regime;
      OrderResult orderResult = OrderResult::new();
      orderResult.OK = true;
      this.game.Data.UnitObj[unr].IsHQ = !this.game.Data.UnitObj[unr].IsHQ;
      if (!this.game.Data.UnitObj[unr].IsHQ)
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index1: i32 =  regime;
        let mut index2: i32 =  index1;
        regimeClassArray[index2].UnitNumber = regimeObj[index1].UnitNumber + 1;
        str1: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[regime].UnitNumber));
        if (this.game.Data.RegimeObj[regime].UnitNumber == 1)
          str1 += "st";
        if (this.game.Data.RegimeObj[regime].UnitNumber == 2)
          str1 += "nd";
        if (this.game.Data.RegimeObj[regime].UnitNumber > 2)
          str1 += "th";
        str2: String = str1 + " " + this.game.Data.RegimeObj[regime].UnitName;
        this.game.Data.UnitObj[unr].Name = str2;
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
        {
          if (this.game.Data.UnitObj[unr].HQ == unr)
            this.game.Data.UnitObj[unr].HQ = -1;
        }
      }
      else
      {
        RegimeClass[] regimeObj = this.game.Data.RegimeObj;
        RegimeClass[] regimeClassArray = regimeObj;
        let mut index4: i32 =  regime;
        let mut index5: i32 =  index4;
        regimeClassArray[index5].HQNumber = regimeObj[index4].HQNumber + 1;
        str3: String = Strings.Trim(Conversion.Str( this.game.Data.RegimeObj[regime].HQNumber));
        if (this.game.Data.RegimeObj[regime].UnitNumber == 1)
          str3 += "st";
        if (this.game.Data.RegimeObj[regime].UnitNumber == 2)
          str3 += "nd";
        if (this.game.Data.RegimeObj[regime].UnitNumber > 2)
          str3 += "th";
        str4: String = str3 + " " + this.game.Data.RegimeObj[regime].HQName;
        this.game.Data.UnitObj[unr].Name = str4;
      }
      return orderResult;
    }
  }
}
