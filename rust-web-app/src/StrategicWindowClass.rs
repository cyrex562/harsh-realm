// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.StrategicWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class StrategicWindowClass : WindowClass
  {
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B5Id: i32;
     B6Id: i32;
     text4id: i32;
     text5id: i32;
     text6id: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Pic2Id: i32;
     int[] Pic1Id;
     detailnr: i32;
     OrderTextId: i32;
     OrderText2Id: i32;
     OrderUpId: i32;
     OrderDownId: i32;
     ExtraId: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OrderOKId: i32;
     OrderOKTextId: i32;
     yesid: i32;
     CapTheater: i32;
     TempNew: i32;
     LandCost: i32;
     NavyCost: i32;
     AirCost: i32;
     Land2Cost: i32;
     Navy2Cost: i32;
     Air2Cost: i32;
     unr: i32;
     unrS: i32;
     OwnPowerTransfer: i32;
     MapMatrix2[] templand;
     MapMatrix2[] tempnavy;
     MapMatrix2[] tempair;
     MapMatrix2[] temp2land;
     MapMatrix2[] temp2navy;
     MapMatrix2[] temp2air;
     SimpleList SL;

    pub StrategicWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 = -1, let mut sy: i32 = -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      self.Pic1Id = new int[1];
      self.templand = new MapMatrix2[1];
      self.tempnavy = new MapMatrix2[1];
      self.tempair = new MapMatrix2[1];
      self.temp2land = new MapMatrix2[1];
      self.temp2navy = new MapMatrix2[1];
      self.temp2air = new MapMatrix2[1];
      self.fixshade = true;
      self.SL = SimpleList::new();
      if (self.game.EditObj.OrderType == 49)
      {
        let mut unitCounter: i32 = self.game.Data.UnitCounter;
        for (let mut tid: i32 = 0; tid <= unitCounter; tid += 1)
        {
          if (self.game.Data.UnitObj[tid].Historical == self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Historical && self.game.Data.UnitObj[tid].PreDef == -1 & self.game.Data.UnitObj[tid].Regime > -1)
            self.SL.Add(tid, 1);
        }
      }
      else
        self.SL.Add(self.game.EditObj.OrderUnit, 1);
      self.detailnr = -1;
      if (self.game.EditObj.TargetX == -1)
      {
        self.game.EditObj.TargetX = self.game.Data.UnitObj[self.game.EditObj.OrderUnit].X;
        self.game.EditObj.TargetY = self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Y;
      }
      self.templand = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.tempnavy = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.tempair = new MapMatrix2[self.game.Data.MapCounter + 1];
      let mut mapCounter1: i32 = self.game.Data.MapCounter;
      for (let mut index: i32 = 0; index <= mapCounter1; index += 1)
      {
        self.templand[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
        self.tempnavy[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
        self.tempair[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
        self.temp2land[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
        self.temp2navy[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
        self.temp2air[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
      }
      self.NavyCost = 0;
      self.AirCost = 0;
      self.LandCost = 0;
      self.Navy2Cost = 0;
      self.Air2Cost = 0;
      self.Land2Cost = 0;
      let mut counter: i32 = self.SL.Counter;
      for (let mut index1: i32 = 0; index1 <= counter; index1 += 1)
      {
        self.unr = self.game.EditObj.OrderTarget;
        self.unrS = self.SL.Id[index1];
        if ( self.game.Data.RuleVar[350] == 0.0)
        {
          tGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[1]), 1,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y, self.game.Data.UnitObj[self.unr].Map, SeaBlock: true);
          if (self.game.EditObj.TargetX > -1)
          {
            self.NavyCost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.NavyCost += self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.NavyCost =  Math.Round( ( self.NavyCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true)));
            self.Navy2Cost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
            self.Navy2Cost += self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
            self.Navy2Cost =  Math.Round( ( self.Navy2Cost + self.game.Data.RuleVar[351]));
          }
          let mut mapCounter2: i32 = self.game.Data.MapCounter;
          for (let mut index2: i32 = 0; index2 <= mapCounter2; index2 += 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[index2].MapWidth;
            for (let mut index3: i32 = 0; index3 <= mapWidth; index3 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[index2].MapHeight;
              for (let mut index4: i32 = 0; index4 <= mapHeight; index4 += 1)
              {
                numArray1: Vec<i32> = self.tempnavy[index2].Value;
                numArray2: Vec<i32> = numArray1;
                let mut index5: i32 = index3;
                let mut index6: i32 = index5;
                let mut index7: i32 = index4;
                let mut index8: i32 = index7;
                let mut num1: i32 = numArray1[index5, index7] + self.game.EditObj.TempValue[index2].Value[index3, index4] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
                numArray2[index6, index8] = num1;
                numArray3: Vec<i32> = self.temp2navy[index2].Value;
                numArray4: Vec<i32> = numArray3;
                let mut index9: i32 = index3;
                let mut index10: i32 = index9;
                let mut index11: i32 = index4;
                let mut index12: i32 = index11;
                let mut num2: i32 = numArray3[index9, index11] + self.game.EditObj.TempValue[index2].Value[index3, index4];
                numArray4[index10, index12] = num2;
              }
            }
          }
          tGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[0]), 0,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y, self.game.Data.UnitObj[self.unr].Map);
          if (self.game.EditObj.TargetX > -1)
          {
            self.LandCost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.LandCost += self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.LandCost =  Math.Round( ( self.LandCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true)));
            self.Land2Cost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
            self.Land2Cost += self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
            self.Land2Cost =  Math.Round( ( self.Land2Cost + self.game.Data.RuleVar[351]));
          }
          let mut mapCounter3: i32 = self.game.Data.MapCounter;
          for (let mut index13: i32 = 0; index13 <= mapCounter3; index13 += 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth;
            for (let mut index14: i32 = 0; index14 <= mapWidth; index14 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight;
              for (let mut index15: i32 = 0; index15 <= mapHeight; index15 += 1)
              {
                numArray5: Vec<i32> = self.templand[index13].Value;
                numArray6: Vec<i32> = numArray5;
                let mut index16: i32 = index14;
                let mut index17: i32 = index16;
                let mut index18: i32 = index15;
                let mut index19: i32 = index18;
                let mut num3: i32 = numArray5[index16, index18] + self.game.EditObj.TempValue[index13].Value[index14, index15] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
                numArray6[index17, index19] = num3;
                numArray7: Vec<i32> = self.temp2land[index13].Value;
                numArray8: Vec<i32> = numArray7;
                let mut index20: i32 = index14;
                let mut index21: i32 = index20;
                let mut index22: i32 = index15;
                let mut index23: i32 = index22;
                let mut num4: i32 = numArray7[index20, index22] + self.game.EditObj.TempValue[index13].Value[index14, index15];
                numArray8[index21, index23] = num4;
              }
            }
          }
          tGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[2]), 0,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y, self.game.Data.UnitObj[self.unr].Map);
          if ( self.game.Data.RuleVar[509] == 0.0 && self.game.EditObj.TargetX > -1)
          {
            self.AirCost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.AirCost += self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.AirCost =  Math.Round( ( self.AirCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true)));
            self.Air2Cost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
            self.Air2Cost += self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
            self.Air2Cost =  Math.Round( ( self.Air2Cost + self.game.Data.RuleVar[351]));
          }
          let mut mapCounter4: i32 = self.game.Data.MapCounter;
          for (let mut index24: i32 = 0; index24 <= mapCounter4; index24 += 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth;
            for (let mut index25: i32 = 0; index25 <= mapWidth; index25 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight;
              for (let mut index26: i32 = 0; index26 <= mapHeight; index26 += 1)
              {
                numArray9: Vec<i32> = self.tempair[index24].Value;
                numArray10: Vec<i32> = numArray9;
                let mut index27: i32 = index25;
                let mut index28: i32 = index27;
                let mut index29: i32 = index26;
                let mut index30: i32 = index29;
                let mut num5: i32 = numArray9[index27, index29] + self.game.EditObj.TempValue[index24].Value[index25, index26] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
                numArray10[index28, index30] = num5;
                numArray11: Vec<i32> = self.temp2air[index24].Value;
                numArray12: Vec<i32> = numArray11;
                let mut index31: i32 = index25;
                let mut index32: i32 = index31;
                let mut index33: i32 = index26;
                let mut index34: i32 = index33;
                let mut num6: i32 = numArray11[index31, index33] + self.game.EditObj.TempValue[index24].Value[index25, index26];
                numArray12[index32, index34] = num6;
              }
            }
          }
        }
        else
        {
          tGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[1]), 1,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y, self.game.Data.UnitObj[self.unrS].Map, SeaBlock: true);
          if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unr].Map].Value[self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y] >  self.game.Data.RuleVar[78])
            self.game.HandyFunctionsObj.RedimTempValue(9999);
          if (self.game.EditObj.TargetX > -1)
          {
            self.NavyCost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.NavyCost =  Math.Round( ( self.NavyCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true)));
            self.Navy2Cost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
            self.Navy2Cost =  Math.Round( ( self.Navy2Cost + self.game.Data.RuleVar[351]));
          }
          let mut mapCounter5: i32 = self.game.Data.MapCounter;
          for (let mut index35: i32 = 0; index35 <= mapCounter5; index35 += 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[index35].MapWidth;
            for (let mut index36: i32 = 0; index36 <= mapWidth; index36 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[index35].MapHeight;
              for (let mut index37: i32 = 0; index37 <= mapHeight; index37 += 1)
              {
                numArray13: Vec<i32> = self.tempnavy[index35].Value;
                numArray14: Vec<i32> = numArray13;
                let mut index38: i32 = index36;
                let mut index39: i32 = index38;
                let mut index40: i32 = index37;
                let mut index41: i32 = index40;
                let mut num7: i32 = numArray13[index38, index40] + self.game.EditObj.TempValue[index35].Value[index36, index37] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
                numArray14[index39, index41] = num7;
                numArray15: Vec<i32> = self.temp2navy[index35].Value;
                numArray16: Vec<i32> = numArray15;
                let mut index42: i32 = index36;
                let mut index43: i32 = index42;
                let mut index44: i32 = index37;
                let mut index45: i32 = index44;
                let mut num8: i32 = numArray15[index42, index44] + self.game.EditObj.TempValue[index35].Value[index36, index37];
                numArray16[index43, index45] = num8;
              }
            }
          }
          tGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[0]), 0,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y, self.game.Data.UnitObj[self.unrS].Map);
          if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unr].Map].Value[self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y] >  self.game.Data.RuleVar[78])
            self.game.HandyFunctionsObj.RedimTempValue(9999);
          if (self.game.EditObj.TargetX > -1)
          {
            self.LandCost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.LandCost =  Math.Round( ( self.LandCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true)));
            self.Land2Cost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
            self.Land2Cost =  Math.Round( ( self.Land2Cost + self.game.Data.RuleVar[351]));
          }
          let mut mapCounter6: i32 = self.game.Data.MapCounter;
          for (let mut index46: i32 = 0; index46 <= mapCounter6; index46 += 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth;
            for (let mut index47: i32 = 0; index47 <= mapWidth; index47 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight;
              for (let mut index48: i32 = 0; index48 <= mapHeight; index48 += 1)
              {
                numArray17: Vec<i32> = self.templand[index46].Value;
                numArray18: Vec<i32> = numArray17;
                let mut index49: i32 = index47;
                let mut index50: i32 = index49;
                let mut index51: i32 = index48;
                let mut index52: i32 = index51;
                let mut num9: i32 = numArray17[index49, index51] + self.game.EditObj.TempValue[index46].Value[index47, index48] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
                numArray18[index50, index52] = num9;
                numArray19: Vec<i32> = self.temp2land[index46].Value;
                numArray20: Vec<i32> = numArray19;
                let mut index53: i32 = index47;
                let mut index54: i32 = index53;
                let mut index55: i32 = index48;
                let mut index56: i32 = index55;
                let mut num10: i32 = numArray19[index53, index55] + self.game.EditObj.TempValue[index46].Value[index47, index48];
                numArray20[index54, index56] = num10;
              }
            }
          }
          tGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[2]), 0,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y, self.game.Data.UnitObj[self.unrS].Map);
          if ( self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unr].Map].Value[self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y] >  self.game.Data.RuleVar[78])
            self.game.HandyFunctionsObj.RedimTempValue(9999);
          if ( self.game.Data.RuleVar[509] == 0.0 && self.game.EditObj.TargetX > -1)
          {
            self.AirCost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
            self.AirCost =  Math.Round( ( self.AirCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true)));
            self.Air2Cost += self.game.EditObj.TempValue[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
            self.Air2Cost =  Math.Round( ( self.Air2Cost + self.game.Data.RuleVar[351]));
          }
          let mut mapCounter7: i32 = self.game.Data.MapCounter;
          for (let mut index57: i32 = 0; index57 <= mapCounter7; index57 += 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth;
            for (let mut index58: i32 = 0; index58 <= mapWidth; index58 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight;
              for (let mut index59: i32 = 0; index59 <= mapHeight; index59 += 1)
              {
                numArray21: Vec<i32> = self.tempair[index57].Value;
                numArray22: Vec<i32> = numArray21;
                let mut index60: i32 = index58;
                let mut index61: i32 = index60;
                let mut index62: i32 = index59;
                let mut index63: i32 = index62;
                let mut num11: i32 = numArray21[index60, index62] + self.game.EditObj.TempValue[index57].Value[index58, index59] * self.game.HandyFunctionsObj.GetUnitWeight(self.unrS, true);
                numArray22[index61, index63] = num11;
                numArray23: Vec<i32> = self.temp2air[index57].Value;
                numArray24: Vec<i32> = numArray23;
                let mut index64: i32 = index58;
                let mut index65: i32 = index64;
                let mut index66: i32 = index59;
                let mut index67: i32 = index66;
                let mut num12: i32 = numArray23[index64, index66] + self.game.EditObj.TempValue[index57].Value[index58, index59];
                numArray24[index65, index67] = num12;
              }
            }
          }
        }
      }
      self.CapTheater = 0;
      if (self.Air2Cost < 9999 &  self.game.Data.RuleVar[509] == 0.0)
        self.CapTheater = 2;
      if (self.game.Data.UnitObj[self.unr].AirCap > self.game.Data.UnitObj[self.unr].LandCap)
        self.CapTheater = 2;
      if (self.unr > -1 && self.game.Data.UnitObj[self.unr].AirCap == 0 & self.CapTheater == 2)
        self.CapTheater = self.game.Data.UnitObj[self.unr].LandCap <= self.game.Data.UnitObj[self.unr].NavyCap ? 1 : 0;
      self.SetTempValue();
      self.TempNew = 0;
      self.dostuff();
    }

    pub fn SetTempValue()
    {
      let mut mapCounter: i32 = self.game.Data.MapCounter;
      for (let mut index1: i32 = 0; index1 <= mapCounter; index1 += 1)
      {
        let mut mapWidth: i32 = self.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
          {
            if ( self.game.Data.RuleVar[350] == 0.0)
            {
              if (self.CapTheater == 0)
                self.game.EditObj.TempValue[index1].Value[index2, index3] = self.templand[index1].Value[index2, index3] + self.templand[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
              if (self.CapTheater == 1)
                self.game.EditObj.TempValue[index1].Value[index2, index3] = self.tempnavy[index1].Value[index2, index3] + self.tempnavy[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
              if (self.CapTheater == 2)
                self.game.EditObj.TempValue[index1].Value[index2, index3] = self.tempair[index1].Value[index2, index3] + self.tempair[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
            }
            else
            {
              if (self.CapTheater == 0)
                self.game.EditObj.TempValue[index1].Value[index2, index3] = self.templand[index1].Value[index2, index3];
              if (self.CapTheater == 1)
                self.game.EditObj.TempValue[index1].Value[index2, index3] = self.tempnavy[index1].Value[index2, index3];
              if (self.CapTheater == 2)
                self.game.EditObj.TempValue[index1].Value[index2, index3] = self.tempair[index1].Value[index2, index3];
            }
            let mut counter: i32 = self.SL.Counter;
            for (let mut index4: i32 = 0; index4 <= counter; index4 += 1)
            {
              numArray1: Vec<i32> = self.game.EditObj.TempValue[index1].Value;
              numArray2: Vec<i32> = numArray1;
              let mut index5: i32 = index2;
              let mut index6: i32 = index5;
              let mut index7: i32 = index3;
              let mut index8: i32 = index7;
              let mut num: i32 =  Math.Round( ( numArray1[index5, index7] + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.SL.Id[index4], true)));
              numArray2[index6, index8] = num;
            }
          }
        }
      }
    }

    pub fn DoRefresh()
    {
      if (self.game.EditObj.TargetX > -1)
      {
        self.AirCost = 0;
        self.NavyCost = 0;
        self.LandCost = 0;
        if ( self.game.Data.RuleVar[350] == 0.0)
        {
          self.LandCost = self.templand[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] + self.templand[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
          self.NavyCost = self.tempnavy[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] + self.tempnavy[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
          if ( self.game.Data.RuleVar[509] == 0.0)
            self.AirCost =  self.game.Data.RuleVar[2] <= -1.0 ? 9999 : self.tempair[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY] + self.tempair[self.game.Data.UnitObj[self.unrS].Map].Value[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y];
        }
        else
        {
          self.LandCost = self.templand[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
          self.NavyCost = self.tempnavy[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
          if ( self.game.Data.RuleVar[509] == 0.0)
            self.AirCost =  self.game.Data.RuleVar[2] <= -1.0 ? 9999 : self.tempair[self.game.EditObj.TargetMap].Value[self.game.EditObj.TargetX, self.game.EditObj.TargetY];
        }
        let mut counter: i32 = self.SL.Counter;
        for (let mut index: i32 = 0; index <= counter; index += 1)
        {
          self.LandCost =  Math.Round( ( self.LandCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.SL.Id[index], true)));
          self.NavyCost =  Math.Round( ( self.NavyCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.SL.Id[index], true)));
          self.AirCost =  Math.Round( ( self.AirCost + self.game.Data.RuleVar[351] *  self.game.HandyFunctionsObj.GetUnitWeight(self.SL.Id[index], true)));
        }
      }
      self.TempNew = 0;
      self.dostuff();
    }

     void dostuff()
    {
      if (self.Text1Id > 0)
        self.RemoveSubPart(self.Text1Id);
      if (self.Text2Id > 0)
        self.RemoveSubPart(self.Text2Id);
      if (self.Text3Id > 0)
        self.RemoveSubPart(self.Text3Id);
      if (self.text4id > 0)
        self.RemoveSubPart(self.text4id);
      if (self.text5id > 0)
        self.RemoveSubPart(self.text5id);
      if (self.text6id > 0)
        self.RemoveSubPart(self.text6id);
      if (self.B1Id > 0)
        self.RemoveSubPart(self.B1Id);
      let mut upperBound: i32 = self.Pic1Id.GetUpperBound(0);
      for (let mut index: i32 = 0; index <= upperBound; index += 1)
      {
        if (self.Pic1Id[index] > 0)
          self.RemoveSubPart(self.Pic1Id[index]);
      }
      if (self.Pic2Id > 0)
        self.RemoveSubPart(self.Pic2Id);
      if (self.B1TextId > 0)
        self.RemoveSubPart(self.B1TextId);
      if (self.B2Id > 0)
        self.RemoveSubPart(self.B2Id);
      if (self.B2TextId > 0)
        self.RemoveSubPart(self.B2TextId);
      if (self.B3Id > 0)
        self.RemoveSubPart(self.B3Id);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B6Id > 0)
        self.RemoveSubPart(self.B6Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.OrderUpId > 0)
        self.RemoveSubPart(self.OrderUpId);
      if (self.OrderDownId > 0)
        self.RemoveSubPart(self.OrderDownId);
      if (self.ExtraId > 0)
        self.RemoveSubPart(self.ExtraId);
      if (self.OrderTextId > 0)
        self.RemoveSubPart(self.OrderTextId);
      if (self.OrderText2Id > 0)
        self.RemoveSubPart(self.OrderText2Id);
      if (self.OptionsListId > 0)
        self.RemoveSubPart(self.OptionsListId);
      if (self.OrderOKId > 0)
        self.RemoveSubPart(self.OrderOKId);
      if (self.OrderOKTextId > 0)
        self.RemoveSubPart(self.OrderOKTextId);
      if (self.yesid > 0)
        self.RemoveSubPart(self.yesid);
      self.NewBackGroundAndClearAll(1024, 200, -1);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      self.OptionsListObj = ListClass::new();
      self.OwnPowerTransfer = 0;
      self.unrS = self.game.EditObj.OrderUnit;
      self.unr = self.game.EditObj.OrderTarget;
      let mut x: i32 = 165;
      if (self.game.EditObj.TransferLostQty > 0)
      {
        str: String = "troops";
        txt: String;
        if (self.game.EditObj.TransferLostTransports > 0)
          txt = "Lost " + Strings.Trim(Conversion.Str( self.game.EditObj.TransferLostQty)) + " " + str + " and " + Strings.Trim(Conversion.Str( self.game.EditObj.TransferLostTransports)) + " transport troops due to enemy Anti-Cap.";
        else
          txt = "Lost " + Strings.Trim(Conversion.Str( self.game.EditObj.TransferLostQty)) + " " + str + " due to enemy Anti-Cap.";
        let mut tsubpart: SubPartClass =  new ATTextPartClass(txt, self.game.VicFont2, x + 600, 20, true);
        self.Text1Id = self.AddSubPart( tsubpart, x, 70, 600, 20, 0);
        tsubpart =  ButtonPartClass::new(self.game.OKBALL, tDescript: "Click to continue");
        self.yesid = self.AddSubPart( tsubpart, x + 284, 130, 35, 35, 1);
        self.game.EditObj.TransferLostQty = 0;
      }
      else
      {
        let mut num1: i32 = x - 20;
        let mut tsubpart1: SubPartClass =  new ATTextPartClass("UNIT", self.game.VicFont2, 55, 20, true, tDescript: "The Unit you are transferring");
        self.Text1Id = self.AddSubPart( tsubpart1, num1 + 145, 45, 55, 20, 0);
        let mut tsubpart2: SubPartClass =  new ATTextPartClass("WITH", self.game.VicFont2, 55, 20, true, tDescript: "The Unit which movement capacity you are using");
        self.Text2Id = self.AddSubPart( tsubpart2, num1 + 145, 105, 55, 20, 0);
        self.Pic1Id = new int[self.SL.Counter + 1];
        let mut counter: i32 = self.SL.Counter;
        SubPartClass tsubpart3;
        for (let mut index1: i32 = 0; index1 <= counter; index1 += 1)
        {
          int[] pic1Id = self.Pic1Id;
          let mut index2: i32 = index1;
          tsubpart3 =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.SL.Id[index1]), "The Unit you are transferring");
          let mut num2: i32 = self.AddSubPart( tsubpart3, num1 + 115 +  Math.Round(40.0 * ( (index1 + 1) /  (self.SL.Counter + 1))), 65, 31, 31, 0);
          pic1Id[index2] = num2;
        }
        tsubpart3 =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.game.EditObj.OrderTarget), "The Unit which movement capacity you are using");
        self.Pic2Id = self.AddSubPart( tsubpart3, num1 + 155, 125, 31, 31, 0);
        let mut num3: i32 = num1 + 20;
        let mut num4: i32 = 0;
        if (self.CapTheater == 0)
          num4 += self.Land2Cost;
        if (self.CapTheater == 1)
          num4 += self.Navy2Cost;
        if (self.CapTheater == 2)
          num4 += self.Air2Cost;
        let mut land2Cost: i32 = self.Land2Cost;
        let mut navy2Cost: i32 = self.Navy2Cost;
        let mut air2Cost: i32 = self.Air2Cost;
        let mut landCost: i32 = self.LandCost;
        let mut navyCost: i32 = self.NavyCost;
        let mut airCost: i32 = self.AirCost;
        let mut orderTarget: i32 = self.game.EditObj.OrderTarget;
        let mut Number1: i32 = self.game.Data.UnitObj[orderTarget].LandCap;
        let mut Number2: i32 = self.game.Data.UnitObj[orderTarget].NavyCap;
        let mut Number3: i32 = self.game.Data.UnitObj[orderTarget].AirCap;
        bool flag1;
        if ( self.game.Data.RuleVar[852] > 0.0)
        {
          let mut num5: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[ Math.Round( self.game.Data.RuleVar[851])];
          let mut num6: i32 =  Math.Round( Number1 / 1000.0 *  self.game.Data.RuleVar[852]);
          if (1 > num6)
            num6 = 1;
          if (num6 > num5)
          {
            Number1 =  Math.Round(Conversion.Int( Number1 * ( num5 /  num6)));
            flag1 = true;
          }
        }
        bool flag2;
        if ( self.game.Data.RuleVar[854] > 0.0)
        {
          let mut num7: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[ Math.Round( self.game.Data.RuleVar[853])];
          let mut num8: i32 =  Math.Round( Number2 / 1000.0 *  self.game.Data.RuleVar[854]);
          if (1 > num8)
            num8 = 1;
          if (num8 > num7)
          {
            Number2 =  Math.Round(Conversion.Int( Number2 * ( num7 /  num8)));
            flag2 = true;
          }
        }
        bool flag3;
        if ( self.game.Data.RuleVar[856] > 0.0)
        {
          let mut num9: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[ Math.Round( self.game.Data.RuleVar[855])];
          let mut num10: i32 =  Math.Round( Number3 / 1000.0 *  self.game.Data.RuleVar[856]);
          if (1 > num10)
            num10 = 1;
          if (num10 > num9)
          {
            Number3 =  Math.Round(Conversion.Int( Number3 * ( num9 /  num10)));
            flag3 = true;
          }
        }
        let mut num11: i32 = 0;
        if (self.CapTheater == 0)
          num11 = Number1;
        if (self.CapTheater == 1)
          num11 = Number2;
        if (self.CapTheater == 2)
          num11 = Number3;
        num12: i32;
        if (self.CapTheater == 0)
          num12 = landCost;
        if (self.CapTheater == 1)
          num12 = navyCost;
        if (self.CapTheater == 2)
          num12 = airCost;
        if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.EditObj.TargetX, self.game.EditObj.TargetY].UnitCounter > 14)
          num4 = 9999;
        if (self.game.Data.MapObj[self.game.EditObj.TargetMap].HexObj[self.game.EditObj.TargetX, self.game.EditObj.TargetY].get_APPenalty(self.game.Data.Turn) > 0)
          num4 = 9999;
        if (self.game.Data.MapObj[self.game.Data.UnitObj[self.unrS].Map].HexObj[self.game.Data.UnitObj[self.unrS].X, self.game.Data.UnitObj[self.unrS].Y].get_APPenalty(self.game.Data.Turn) > 0)
          num4 = 9999;
        if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.EditObj.TargetX, self.game.EditObj.TargetY].Regime == -1)
          num4 = 9999;
        if (!(self.game.Data.UnitObj[self.unrS].X == self.game.EditObj.TargetX & self.game.Data.UnitObj[self.unrS].Y == self.game.EditObj.TargetY))
        {
          if (num11 >= num12)
          {
            if (num4 < 9999)
            {
              tsubpart3 =  new TextButtonPartClass("Do Strategic Transfer", 160, "Do the Strategic Transfer!",  self.OwnBitmap, num3 + 540, 85, theight: 50);
              self.OrderOKId = self.AddSubPart( tsubpart3, num3 + 540, 85, 160, 50, 1);
            }
            else
            {
              tsubpart3 =  new TextButtonPartClass("Do Strategic Transfer", 160, "Impossible to reach hex",  self.OwnBitmap, num3 + 540, 85, true, theight: 50);
              self.OrderOKTextId = self.AddSubPart( tsubpart3, num3 + 540, 85, 160, 50, 0);
            }
          }
          else
          {
            tsubpart3 =  new TextButtonPartClass("Do Strategic Transfer", 160, "Impossible to reach hex",  self.OwnBitmap, num3 + 540, 85, true, theight: 50);
            self.OrderOKTextId = self.AddSubPart( tsubpart3, num3 + 540, 85, 160, 50, 0);
          }
        }
        else
        {
          tsubpart3 =  new TextButtonPartClass("Do Strategic Transfer", 160, "Cannot transfer unit to the hex it is already located at.",  self.OwnBitmap, num3 + 540, 85, true, theight: 50);
          self.OrderOKTextId = self.AddSubPart( tsubpart3, num3 + 540, 85, 160, 50, 0);
        }
        num13: i32;
        txt1: String;
        if (self.game.EditObj.TargetX > -1)
        {
          txt1 = !(land2Cost < 9999 & num13 != 2 & self.temp2land[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY] < 9999) ? "No Land Connect" : "LandCap = " + Conversion.Str( landCost) + " / " + Conversion.Str( Number1);
          if (flag1)
            txt1 += " (fuel!)";
        }
        else
          txt1 = "Land";
        Number4: i32;
        if (self.CapTheater == 0)
        {
          Number4 = self.temp2land[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY];
          tsubpart3 =  new SteveButtonPartClass(self.game.OKBALL, tDescript: "Transfer over Land", tBackbitmap: ( self.OwnBitmap), bbx: (num3 + 250), bby: 65);
          self.B4Id = self.AddSubPart( tsubpart3, num3 + 250, 65, 35, 35, 1);
          tsubpart3 =  new ATTextPartClass(txt1, self.game.VicFont2, 230, 20, false);
          self.text4id = self.AddSubPart( tsubpart3, num3 + 290, 73, 230, 20, 0);
        }
        else
        {
          tsubpart3 =  new SteveButtonPartClass(self.game.CANCELBALL, tDescript: "Transfer over Land", tBackbitmap: ( self.OwnBitmap), bbx: (num3 + 250), bby: 65);
          self.B4Id = self.AddSubPart( tsubpart3, num3 + 250, 65, 35, 35, 1);
          tsubpart3 =  new ATTextPartClass(txt1, self.game.VicFont2, 230, 20, false);
          self.text4id = self.AddSubPart( tsubpart3, num3 + 290, 73, 230, 20, 0);
        }
        txt2: String;
        if (self.game.EditObj.TargetX > -1)
        {
          txt2 = !(navy2Cost < 9999 & num13 != 1 & self.temp2navy[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY] < 9999) ? "No Navy Connect" : "NavyCap = " + Conversion.Str( navyCost) + " / " + Conversion.Str( Number2);
          if (flag2)
            txt2 += " (fuel!)";
        }
        else
          txt2 = "Navy";
        if (self.CapTheater == 1)
        {
          Number4 = self.temp2navy[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY];
          tsubpart3 =  new SteveButtonPartClass(self.game.OKBALL, tDescript: "Transfer over Sea", tBackbitmap: ( self.OwnBitmap), bbx: (num3 + 250), bby: 110);
          self.B5Id = self.AddSubPart( tsubpart3, num3 + 250, 110, 35, 35, 1);
          tsubpart3 =  new ATTextPartClass(txt2, self.game.VicFont2, 230, 20, false);
          self.text5id = self.AddSubPart( tsubpart3, num3 + 290, 118, 230, 20, 0);
        }
        else
        {
          tsubpart3 =  new SteveButtonPartClass(self.game.CANCELBALL, tDescript: "Transfer over Sea", tBackbitmap: ( self.OwnBitmap), bbx: (num3 + 250), bby: 110);
          self.B5Id = self.AddSubPart( tsubpart3, num3 + 250, 110, 35, 35, 1);
          tsubpart3 =  new ATTextPartClass(txt2, self.game.VicFont2, 230, 20, false);
          self.text5id = self.AddSubPart( tsubpart3, num3 + 290, 118, 230, 20, 0);
        }
        if ( self.game.Data.RuleVar[509] == 0.0)
        {
          txt3: String;
          if (self.game.EditObj.TargetX > -1)
          {
            txt3 = !(air2Cost < 9999 & num13 != 1 &  self.game.Data.RuleVar[509] == 0.0 & self.temp2air[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY] < 9999) ? "No Rail Connect" : "RailCap = " + Conversion.Str( airCost) + " / " + Conversion.Str( Number3);
            if (flag3)
              txt3 += " (fuel!)";
          }
          else
            txt3 = "Rail";
          if (self.CapTheater == 2)
          {
            Number4 = self.temp2air[self.game.EditObj.MapSelected].Value[self.game.SelectX, self.game.SelectY];
            tsubpart3 =  new SteveButtonPartClass(self.game.OKBALL, tDescript: "Transfer over Rail", tBackbitmap: ( self.OwnBitmap), bbx: (num3 + 250), bby: 155);
            self.B6Id = self.AddSubPart( tsubpart3, num3 + 250, 155, 35, 35, 1);
            tsubpart3 =  new ATTextPartClass(txt3, self.game.VicFont2, 230, 20, false);
            self.text6id = self.AddSubPart( tsubpart3, num3 + 290, 163, 230, 20, 0);
          }
          else
          {
            tsubpart3 =  new SteveButtonPartClass(self.game.CANCELBALL, tDescript: "Transfer over Rail", tBackbitmap: ( self.OwnBitmap), bbx: (num3 + 250), bby: 155);
            self.B6Id = self.AddSubPart( tsubpart3, num3 + 250, 155, 35, 35, 1);
            tsubpart3 =  new ATTextPartClass(txt3, self.game.VicFont2, 230, 20, false);
            self.text6id = self.AddSubPart( tsubpart3, num3 + 290, 163, 230, 20, 0);
          }
        }
        tsubpart3 =  new ATTextPartClass("BaseCost=" + Strings.Trim(Conversion.Str( Number4)), self.game.VicFont2, 190, 20, true, tDescript: "What it costs to transfer one unit of weight to the selected destination", tBlackBack: true);
        self.Text3Id = self.AddSubPart( tsubpart3, num3 + 250, 20, 190, 20, 0);
        if (Information.IsNothing( Expression))
          return;
        Expression.Dispose();
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num: i32 = self.SubPartID[index1];
            if (num == self.yesid)
            {
              self.game.EditObj.TransferLostQty = -1;
              self.game.EditObj.TransferLostType = -1;
              self.game.EditObj.TransferLostTransports = -1;
              self.game.EditObj.TempCoordList = CoordList::new();
              self.game.EditObj.ShowTransfer = false;
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 44);
              windowReturnClass.AddCommand(1, 5);
              windowReturnClass.AddCommand(2, 20);
              windowReturnClass.AddCommand(4, 18);
              self.game.EditObj.OrderType = 0;
              self.game.SelectX = self.game.EditObj.TargetX;
              self.game.SelectY = self.game.EditObj.TargetY;
              self.game.EditObj.TargetX = -1;
              self.game.EditObj.TargetY = -1;
              self.game.EditObj.UnitSelected = self.game.EditObj.OrderUnit;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.B4Id)
            {
              self.CapTheater = 0;
              self.SetTempValue();
              self.dostuff();
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.B5Id)
            {
              self.CapTheater = 1;
              self.SetTempValue();
              self.dostuff();
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.B6Id)
            {
              self.CapTheater = 2;
              self.SetTempValue();
              self.dostuff();
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == self.OrderOKId)
            {
              self.game.EditObj.TransferLostQty = 0;
              self.game.EditObj.TransferLostType = -1;
              self.game.EditObj.TransferLostTransports = 0;
              let mut counter: i32 = self.SL.Counter;
              for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
                orderResult = (OrderResult) self.game.ProcessingObj.DoStrategicTransfer(self.game.EditObj.OrderTarget, self.SL.Id[index2], self.CapTheater, self.game.EditObj.TargetX, self.game.EditObj.TargetY, self.game.EditObj.TargetMap);
              if (orderResult.OK)
              {
                if (self.game.EditObj.SoundOn)
                  SoundMod.PlayAWave(self.game.AppPath + "sound/transfer.wav",  self.game.EditObj);
                if (self.game.EditObj.TransferLostQty < 1)
                {
                  self.game.EditObj.TempCoordList = CoordList::new();
                  self.game.EditObj.ShowTransfer = false;
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.AddCommand(4, 44);
                  windowReturnClass.AddCommand(1, 5);
                  windowReturnClass.AddCommand(2, 20);
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.AddCommand(4, 66);
                  self.game.EditObj.OrderType = 0;
                  self.game.SelectX = self.game.EditObj.TargetX;
                  self.game.SelectY = self.game.EditObj.TargetY;
                  self.game.EditObj.TargetX = -1;
                  self.game.EditObj.TargetY = -1;
                  self.game.EditObj.UnitSelected = self.game.EditObj.OrderUnit;
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(4, 66);
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.SetFlag(false);
              return windowReturnClass;
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
