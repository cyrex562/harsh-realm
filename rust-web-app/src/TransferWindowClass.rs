// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TransferWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TransferWindowClass : WindowClass
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
     x1id: i32;
     x2id: i32;
     SwitchId: i32;
     Type1Id: i32;
     Type2Id: i32;
     Type3Id: i32;
     text4id: i32;
     text5id: i32;
     text6id: i32;
     text7id: i32;
     text8id: i32;
     text9id: i32;
     Text1Id: i32;
     Text2Id: i32;
     Text3Id: i32;
     Pic1Id: i32;
     Pic2Id: i32;
     detailnr: i32;
     detailtype: i32;
     OrderTextId: i32;
     OrderText2Id: i32;
     OrderUpId: i32;
     OrderDownId: i32;
     ExtraId: i32;
     OptionsListId: i32;
     ATListClass OptionsListObj;
     OptionsList2Id: i32;
     ATListClass OptionsList2Obj;
     OrderOKId: i32;
     OrderOKTextId: i32;
     OrderALLId: i32;
     OrderALLTextId: i32;
     YesId: i32;
     sliderID: i32;
     CapTheater: i32;
     TempNew: i32;
     object LandCost;
     object NavyCost;
     object AirCost;
     RemLandCost: i32;
     RemNavyCost: i32;
     RemAirCost: i32;
     unr: i32;
     unrT: i32;
     hq: i32;
     nothq: i32;
     overrulehq: i32;
     OwnPowerTransfer: i32;
     MapMatrix2[] templand;
     MapMatrix2[] tempnavy;
     MapMatrix2[] tempair;
     seltheater: i32;
     tempSfType: i32;
     bool AirCarrier;
     bool HasAirCarrier;

    pub TransferWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 = -1, let mut sy: i32 = -1)
      : base( tGame, 1024, 200, 99, screenbitmap: screenbitmap, sx: sx, sy: sy)
    {
      self.templand = new MapMatrix2[1];
      self.tempnavy = new MapMatrix2[1];
      self.tempair = new MapMatrix2[1];
      self.detailnr = -1;
      self.detailtype = 1;
      self.fixshade = true;
      self.DoNewStuff();
      self.overrulehq = -1;
      self.dostuff();
    }

    pub fn DoNewStuff()
    {
      self.templand = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.tempnavy = new MapMatrix2[self.game.Data.MapCounter + 1];
      self.tempair = new MapMatrix2[self.game.Data.MapCounter + 1];
      let mut mapCounter1: i32 = self.game.Data.MapCounter;
      for (let mut index: i32 = 0; index <= mapCounter1; index += 1)
      {
        self.templand[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
        self.tempnavy[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
        self.tempair[index] = new MapMatrix2(self.game.Data.MapObj[index].MapWidth, self.game.Data.MapObj[index].MapHeight);
      }
      self.unr = self.game.EditObj.OrderUnit;
      self.unrT = self.game.EditObj.OrderTarget;
      if (self.game.Data.UnitObj[self.unr].SFCount > -1)
        self.detailnr = self.game.Data.UnitObj[self.unr].SFList[0];
      if (self.unrT > -1)
      {
        if (self.game.Data.UnitObj[self.unr].IsHQ & !self.game.Data.UnitObj[self.unrT].IsHQ)
          self.hq = self.unr;
        if (!self.game.Data.UnitObj[self.unr].IsHQ & self.game.Data.UnitObj[self.unrT].IsHQ)
          self.hq = self.unrT;
        if (self.game.Data.UnitObj[self.unr].IsHQ & self.game.Data.UnitObj[self.unrT].IsHQ)
          self.hq = self.unr;
      }
      else
        self.hq = -1;
      DrawMod.TGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[0]), 0,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.game.EditObj.OrderUnit].X, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Y, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Map);
      if (self.unrT > -1)
        self.LandCost =  self.game.EditObj.TempValue[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Map].Value[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y];
      let mut mapCounter2: i32 = self.game.Data.MapCounter;
      for (let mut index1: i32 = 0; index1 <= mapCounter2; index1 += 1)
      {
        let mut mapWidth: i32 = self.game.Data.MapObj[index1].MapWidth;
        for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[index1].MapHeight;
          for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
            self.templand[index1].Value[index2, index3] = self.game.EditObj.TempValue[index1].Value[index2, index3];
        }
      }
      DrawMod.TGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[1]), 1,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.game.EditObj.OrderUnit].X, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Y, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Map, false, muststartonairfield: false, SeaBlock: true);
      if (self.unrT > -1)
        self.NavyCost =  self.game.EditObj.TempValue[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Map].Value[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y];
      let mut mapCounter3: i32 = self.game.Data.MapCounter;
      for (let mut index4: i32 = 0; index4 <= mapCounter3; index4 += 1)
      {
        let mut mapWidth: i32 = self.game.Data.MapObj[index4].MapWidth;
        for (let mut index5: i32 = 0; index5 <= mapWidth; index5 += 1)
        {
          let mut mapHeight: i32 = self.game.Data.MapObj[index4].MapHeight;
          for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
            self.tempnavy[index4].Value[index5, index6] = self.game.EditObj.TempValue[index4].Value[index5, index6];
        }
      }
      if ( self.game.Data.RuleVar[509] == 0.0)
      {
        if ( self.game.Data.RuleVar[2] > -1.0)
        {
          DrawMod.TGame.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.UnitObj[self.unr].Regime,  Math.Round( self.game.Data.RuleVar[2]), 0,  Math.Round( self.game.Data.RuleVar[78]), self.game.Data.UnitObj[self.game.EditObj.OrderUnit].X, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Y, self.game.Data.UnitObj[self.game.EditObj.OrderUnit].Map);
          if (self.unrT > -1)
            self.AirCost =  self.game.EditObj.TempValue[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Map].Value[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y];
          let mut mapCounter4: i32 = self.game.Data.MapCounter;
          for (let mut index7: i32 = 0; index7 <= mapCounter4; index7 += 1)
          {
            let mut mapWidth: i32 = self.game.Data.MapObj[index7].MapWidth;
            for (let mut index8: i32 = 0; index8 <= mapWidth; index8 += 1)
            {
              let mut mapHeight: i32 = self.game.Data.MapObj[index7].MapHeight;
              for (let mut index9: i32 = 0; index9 <= mapHeight; index9 += 1)
                self.tempair[index7].Value[index8, index9] = self.game.EditObj.TempValue[index7].Value[index8, index9];
            }
          }
        }
        else
          self.AirCost =  9999;
      }
      self.CapTheater = 0;
      self.TempNew = 1;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectGreaterEqual(self.LandCost,  9999, false), Operators.CompareObjectLess(self.NavyCost,  9999, false))))
        self.CapTheater = 1;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.AndObject(Operators.AndObject(Operators.CompareObjectGreaterEqual(self.LandCost,  9999, false), Operators.CompareObjectGreaterEqual(self.NavyCost,  9999, false)), Operators.CompareObjectLess(self.AirCost,  9999, false)),  ( self.game.Data.RuleVar[509] == 0.0))))
        self.CapTheater = 2;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectLess(self.AirCost,  9999, false),  ( self.game.Data.RuleVar[509] == 0.0))))
        self.CapTheater = 2;
      if (self.hq > -1 && self.game.Data.UnitObj[self.hq].AirCap <= 1 & self.CapTheater == 2)
        self.CapTheater = self.game.Data.UnitObj[self.hq].LandCap <= self.game.Data.UnitObj[self.hq].NavyCap ? 1 : 0;
      self.SetTempValue();
      self.RemLandCost = Conversions.ToInteger(self.LandCost);
      self.RemAirCost = Conversions.ToInteger(self.AirCost);
      self.RemNavyCost = Conversions.ToInteger(self.NavyCost);
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
            if (self.CapTheater == 0)
              self.game.EditObj.TempValue[index1].Value[index2, index3] = self.templand[index1].Value[index2, index3];
            if (self.CapTheater == 1)
              self.game.EditObj.TempValue[index1].Value[index2, index3] = self.tempnavy[index1].Value[index2, index3];
            if (self.CapTheater == 2)
              self.game.EditObj.TempValue[index1].Value[index2, index3] = self.tempair[index1].Value[index2, index3];
          }
        }
      }
    }

    pub fn DoRefresh()
    {
      bool flag = false;
      self.detailtype = 1;
      self.overrulehq = -1;
      self.unr = self.game.EditObj.OrderUnit;
      self.unrT = self.game.EditObj.OrderTarget;
      if (self.unrT > -1)
      {
        if (self.game.Data.UnitObj[self.unr].IsHQ & !self.game.Data.UnitObj[self.unrT].IsHQ)
          self.hq = self.unr;
        if (!self.game.Data.UnitObj[self.unr].IsHQ & self.game.Data.UnitObj[self.unrT].IsHQ)
          self.hq = self.unrT;
        if (self.game.Data.UnitObj[self.unr].IsHQ & self.game.Data.UnitObj[self.unrT].IsHQ)
        {
          self.hq = self.unr;
          flag = true;
        }
      }
      else
        self.hq = -1;
      if (self.overrulehq > -1 & flag)
        self.hq = self.overrulehq;
      if (self.unrT > -1)
      {
        self.LandCost =  self.templand[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Map].Value[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y];
        self.NavyCost =  self.tempnavy[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Map].Value[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y];
        if ( self.game.Data.RuleVar[509] == 0.0)
          self.AirCost =  self.game.Data.RuleVar[2] <= -1.0 ?  9999 :  self.tempair[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Map].Value[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y];
      }
      if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectGreaterEqual(self.LandCost,  9999, false), Operators.CompareObjectLess(self.NavyCost,  9999, false))))
        self.CapTheater = 1;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.AndObject(Operators.AndObject(Operators.CompareObjectGreaterEqual(self.LandCost,  9999, false), Operators.CompareObjectGreaterEqual(self.NavyCost,  9999, false)), Operators.CompareObjectLess(self.AirCost,  9999, false)),  ( self.game.Data.RuleVar[509] == 0.0))))
        self.CapTheater = 2;
      if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectLess(self.AirCost,  9999, false),  ( self.game.Data.RuleVar[509] == 0.0))))
        self.CapTheater = 2;
      if (self.hq > -1 && self.game.Data.UnitObj[self.hq].AirCap <= 1 & self.CapTheater == 2)
        self.CapTheater = self.game.Data.UnitObj[self.hq].LandCap <= self.game.Data.UnitObj[self.hq].NavyCap ? 1 : 0;
      if (self.sliderID > 0)
      {
        self.RemoveSubPart(self.sliderID);
        self.sliderID = 0;
      }
      self.TempNew = 1;
      self.SetTempValue();
      self.RemLandCost = Conversions.ToInteger(self.LandCost);
      self.RemAirCost = Conversions.ToInteger(self.AirCost);
      self.RemNavyCost = Conversions.ToInteger(self.NavyCost);
      self.dostuff();
    }

     void dostuff()
    {
      if (self.x1id > 0)
        self.RemoveSubPart(self.x1id);
      if (self.x2id > 0)
        self.RemoveSubPart(self.x2id);
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
      if (self.text7id > 0)
        self.RemoveSubPart(self.text7id);
      if (self.text8id > 0)
        self.RemoveSubPart(self.text8id);
      if (self.text9id > 0)
        self.RemoveSubPart(self.text9id);
      if (self.B1Id > 0)
        self.RemoveSubPart(self.B1Id);
      if (self.Pic1Id > 0)
        self.RemoveSubPart(self.Pic1Id);
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
      if (self.OrderOKId > 0)
        self.RemoveSubPart(self.OrderOKId);
      if (self.OrderOKTextId > 0)
        self.RemoveSubPart(self.OrderOKTextId);
      if (self.OrderALLId > 0)
        self.RemoveSubPart(self.OrderALLId);
      if (self.OrderALLTextId > 0)
        self.RemoveSubPart(self.OrderALLTextId);
      if (self.Type1Id > 0)
        self.RemoveSubPart(self.Type1Id);
      if (self.Type2Id > 0)
        self.RemoveSubPart(self.Type2Id);
      if (self.Type3Id > 0)
        self.RemoveSubPart(self.Type3Id);
      if (self.YesId > 0)
        self.RemoveSubPart(self.YesId);
      if (self.SwitchId > 0)
        self.RemoveSubPart(self.SwitchId);
      self.LandCost =  self.RemLandCost;
      self.NavyCost =  self.RemNavyCost;
      self.AirCost =  self.RemAirCost;
      self.NewBackGroundAndClearAll(1024, 200, -1);
      self.seltheater = 0;
      self.OwnPowerTransfer = 0;
      self.unr = self.game.EditObj.OrderUnit;
      self.unrT = self.game.EditObj.OrderTarget;
      let mut redux: i32 = 0;
      bool flag1 = false;
      self.hq = -1;
      if (self.unrT > -1)
      {
        if (self.game.Data.UnitObj[self.unr].IsHQ & !self.game.Data.UnitObj[self.unrT].IsHQ)
          self.hq = self.unr;
        if (!self.game.Data.UnitObj[self.unr].IsHQ & self.game.Data.UnitObj[self.unrT].IsHQ)
          self.hq = self.unrT;
        if (self.game.Data.UnitObj[self.unr].IsHQ & self.game.Data.UnitObj[self.unrT].IsHQ)
        {
          self.hq = self.unr;
          flag1 = true;
        }
      }
      else
        self.hq = -1;
      if (flag1 & self.overrulehq == -1)
        self.overrulehq = self.hq;
      if (self.overrulehq > -1 & flag1)
        self.hq = self.overrulehq;
      bool flag2 = true;
      if (self.game.EditObj.TransferLostQty > 0)
      {
        if (self.OptionsListId > 0)
        {
          self.RemoveSubPart(self.OptionsListId);
          self.OptionsListId = 0;
        }
        if (self.OptionsList2Id > 0)
        {
          self.RemoveSubPart(self.OptionsList2Id);
          self.OptionsList2Id = 0;
        }
        Graphics.FromImage((Image) self.OwnBitmap);
        str: String;
        if (self.game.EditObj.TransferLostType > 0)
          str = self.game.Data.SFTypeObj[self.game.EditObj.TransferLostType].Name;
        if (self.game.EditObj.TransferLostType == -2)
          str = "Supplies";
        txt: String;
        if (self.game.EditObj.TransferLostTransports > 0)
          txt = "Lost " + Strings.Trim(Conversion.Str( self.game.EditObj.TransferLostQty)) + " " + str + " and " + Strings.Trim(Conversion.Str( self.game.EditObj.TransferLostTransports)) + " transport troops due to enemy Anti-Cap.";
        else
          txt = "Lost " + Strings.Trim(Conversion.Str( self.game.EditObj.TransferLostQty)) + " " + str + " due to enemy Air/Navy.";
        let mut tsubpart1: SubPartClass =  new ATTextPartClass(txt, self.game.VicFont1, 600, 20, true);
        self.Text1Id = self.AddSubPart( tsubpart1, 0, 70, 600, 20, 0);
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.OKBALL, tDescript: "Click to continue");
        self.YesId = self.AddSubPart( tsubpart2, 284, 130, 35, 35, 1);
        self.game.EditObj.TransferLostQty = 0;
      }
      else
      {
        Graphics.FromImage((Image) self.OwnBitmap);
        if (flag2)
        {
          self.OptionsListObj = ATListClass::new();
          let mut tlistselect1: i32 = -1;
          let mut num1: i32 = 0;
          if (self.unrT > -1)
          {
            let mut num2: i32 = 0;
            if (self.game.Data.UnitObj[self.unr].IsHQ & self.game.Data.UnitObj[self.unrT].IsHQ)
              num2 = 1;
            if (self.game.Data.UnitObj[self.unr].IsHQ & self.game.HandyFunctionsObj.IsHexHarbourOrSea(self.game.Data.UnitObj[self.unrT].X, self.game.Data.UnitObj[self.unrT].Y, self.game.Data.UnitObj[self.unrT].Map))
              num2 = 1;
            if (!self.game.Data.UnitObj[self.unrT].IsHQ & !self.game.HandyFunctionsObj.HasUnitNavySF(self.unrT))
              num2 = 0;
            if (num2 == 1 &  self.game.Data.RuleVar[322] == 0.0)
            {
              self.OptionsListObj.add(Conversion.Str( self.game.Data.UnitObj[self.unr].Supply) + "x Supply Pts", -2);
              num1 = 1;
              if (self.detailnr == -2)
                tlistselect1 = 0;
            }
          }
          if (self.game.Data.UnitObj[self.unr].SFCount > -1)
          {
            let mut sfCount: i32 = self.game.Data.UnitObj[self.unr].SFCount;
            for (let mut index: i32 = 0; index <= sfCount; index += 1)
            {
              let mut sf: i32 = self.game.Data.UnitObj[self.unr].SFList[index];
              if (sf == self.detailnr)
                tlistselect1 = index + num1;
              let mut type: i32 = self.game.Data.SFObj[sf].Type;
              if (sf == self.detailnr)
                self.tempSfType = type;
              self.OptionsListObj.add(Conversion.Str( self.game.Data.SFObj[sf].Qty) + "x " + self.game.Data.SFTypeObj[type].Name + "(" + Strings.Left(self.game.Data.PeopleObj[self.game.Data.SFObj[sf].People].Name, 3) + ")", sf);
            }
          }
          if (self.OptionsListId > 0)
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect1);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
          }
          else
          {
            let mut tsubpart: SubPartClass =  new ATListSubPartClass(self.OptionsListObj, 5, 220, tlistselect1, self.game, tHeader: self.game.Data.UnitObj[self.unr].Name, tbackbitmap: ( self.OwnBitmap), bbx: 70, bby: 30);
            self.OptionsListId = self.AddSubPart( tsubpart, 70, 30, 220, 128, 0);
          }
          if (tlistselect1 == -1)
            self.detailnr = -1;
          if (self.unrT > -1)
          {
            self.OptionsList2Obj = ATListClass::new();
            let mut tlistselect2: i32 = -1;
            if (self.game.Data.UnitObj[self.unr].IsHQ & (self.game.Data.UnitObj[self.unrT].IsHQ | self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.UnitObj[self.unrT].X, self.game.Data.UnitObj[self.unrT].Y].LandscapeType].IsSea) &&  self.game.Data.RuleVar[322] == 0.0)
              self.OptionsList2Obj.add(Conversion.Str( self.game.Data.UnitObj[self.unrT].Supply) + "x Supply Pts", -2);
            if (self.game.Data.UnitObj[self.unrT].SFCount > -1)
            {
              let mut sfCount: i32 = self.game.Data.UnitObj[self.unrT].SFCount;
              for (let mut index: i32 = 0; index <= sfCount; index += 1)
              {
                let mut sf: i32 = self.game.Data.UnitObj[self.unrT].SFList[index];
                if (sf == self.detailnr)
                  tlistselect2 = index;
                let mut type: i32 = self.game.Data.SFObj[sf].Type;
                if (sf == self.detailnr)
                  self.tempSfType = type;
                self.OptionsList2Obj.add(Conversion.Str( self.game.Data.SFObj[sf].Qty) + "x " + self.game.Data.SFTypeObj[type].Name + "(" + Strings.Left(self.game.Data.PeopleObj[self.game.Data.SFObj[sf].People].Name, 3) + ")", sf);
              }
            }
            if (self.OptionsList2Id > 0)
            {
              self.SubPartList[self.SubpartNr(self.OptionsList2Id)].Refresh(self.OptionsList2Obj, tlistselect2, self.game.Data.UnitObj[self.unrT].Name);
              self.SubPartFlag[self.SubpartNr(self.OptionsList2Id)] = true;
            }
            else
            {
              let mut tsubpart: SubPartClass =  new ATListSubPartClass(self.OptionsList2Obj, 8, 220, -1, self.game, tHeader: self.game.Data.UnitObj[self.unrT].Name, tbackbitmap: ( self.OwnBitmap), bbx: 515, bby: 30);
              self.OptionsList2Id = self.AddSubPart( tsubpart, 515, 30, 220, 176, 0);
            }
            txt1: String;
            if (self.game.HandyFunctionsObj.HasUnitNavySF(self.unrT))
            {
              let mut unitCarryCap: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unrT, 1);
              let mut Number: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unrT, 1) - self.game.HandyFunctionsObj.GetUnitCarryCap(self.unrT, 1, true);
              txt1 = "Naval Carry:" + Strings.Trim(Conversion.Str( unitCarryCap)) + " Weight: " + Strings.Trim(Conversion.Str( Number));
            }
            else if (self.game.HandyFunctionsObj.HasUnitAirSF(self.unrT))
              txt1 = "Air Carry:" + Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetUnitCarryCap(self.unrT, 2)));
            else if (self.game.HandyFunctionsObj.HasUnitlandSF(self.unrT))
            {
              let mut unitCarryCap: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unrT, 0);
              let mut Number: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unrT, 0) - self.game.HandyFunctionsObj.GetUnitCarryCap(self.unrT, 0, true);
              txt1 = "Land Carry:" + Strings.Trim(Conversion.Str( unitCarryCap)) + " Weight: " + Strings.Trim(Conversion.Str( Number));
            }
            else
              txt1 = "";
            let mut tsubpart3: SubPartClass =  new ATTextPartClass(txt1, self.game.VicFont5, 200, 20, false);
            self.text7id = self.AddSubPart( tsubpart3, 300, 170, 200, 20, 0);
            if (self.detailnr > -1)
            {
              txt2: String = "Weight: " + Strings.Trim(Conversion.Str( self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].Weight));
              if (self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].StaffPts > 0 & self.game.Data.UnitObj[self.unrT].Historical > -1)
                txt2 = txt2 + ", Max Leader Staff=" + self.game.HandyFunctionsObj.GetMaxStaffIndividuals(self.unrT, -1).ToString();
              let mut tsubpart4: SubPartClass =  new ATTextPartClass(txt2, self.game.VicFont5, 195, 20, false);
              self.text9id = self.AddSubPart( tsubpart4, 300, 185, 195, 20, 0);
            }
          }
        }
        let mut tsubpart5: SubPartClass =  new ATTextPartClass("FROM", self.game.VicFont2, 55, 20, true, tDescript: "The unit you are transferring from");
        self.Text1Id = self.AddSubPart( tsubpart5, 5, 35, 55, 20, 0);
        let mut tsubpart6: SubPartClass =  new ATTextPartClass("TO", self.game.VicFont2, 55, 20, true, tDescript: "The unit you are transferring too");
        self.Text2Id = self.AddSubPart( tsubpart6, 5, 95, 55, 20, 0);
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.game.EditObj.OrderUnit), self.game.Data.UnitObj[self.unr].Name);
        self.Pic1Id = self.AddSubPart( tsubpart7, 15, 55, 31, 31, 0);
        SubPartClass tsubpart8;
        if (self.unrT > -1)
        {
          tsubpart8 =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.game.EditObj.OrderTarget), self.game.Data.UnitObj[self.unrT].Name);
          self.Pic2Id = self.AddSubPart( tsubpart8, 15, 115, 31, 31, 0);
        }
        if (flag2)
        {
          if (flag1)
          {
            tsubpart8 =  new ATTextPartClass("CAP BY", self.game.VicFont2, 70, 20, true, tDescript: "The HQ that is providing Cap");
            self.text8id = self.AddSubPart( tsubpart8, 80, 167, 70, 20, 0);
            tsubpart8 =  ButtonPartClass::new(self.game.CustomBitmapObj.DrawUnit(self.hq), "Cap by " + self.game.Data.UnitObj[self.hq].Name + ". Click to switch HQ that providdes Capacity for transfer.");
            self.SwitchId = self.AddSubPart( tsubpart8, 160, 157, 37, 37, 0);
          }
          Number1: i32;
          if (self.CapTheater == 0)
            Number1 = Conversions.ToInteger(self.LandCost);
          if (self.CapTheater == 1)
            Number1 = Conversions.ToInteger(self.NavyCost);
          if (self.CapTheater == 2)
            Number1 = Conversions.ToInteger(self.AirCost);
          if (self.detailnr > -1 | self.detailnr == -2)
          {
            float weight;
            moveType: i32;
            ap: i32;
            if (self.detailnr == -2)
            {
              weight = self.game.Data.RuleVar[33];
              redux = 0;
            }
            else if (self.detailtype == 1)
            {
              weight =  self.game.Data.SFTypeObj[self.tempSfType].Weight;
              moveType = self.game.Data.SFTypeObj[self.tempSfType].MoveType;
              ap = self.game.Data.SFObj[self.detailnr].Ap;
              redux = self.game.Data.SFTypeObj[self.tempSfType].MoveRedux;
            }
            let mut integer1: i32 = Conversions.ToInteger(Conversion.Int(Operators.MultiplyObject(Operators.MultiplyObject(self.LandCost,  self.TempNew),  weight)));
            let mut integer2: i32 = Conversions.ToInteger(Conversion.Int(Operators.MultiplyObject(Operators.MultiplyObject(self.NavyCost,  self.TempNew),  weight)));
            let mut integer3: i32 = Conversions.ToInteger(Conversion.Int(Operators.MultiplyObject(Operators.MultiplyObject(self.AirCost,  self.TempNew),  weight)));
            if (self.detailnr != -2 && self.detailtype == 1)
              self.seltheater = self.game.Data.SFTypeObj[self.tempSfType].Theater;
            Number2: i32;
            Number3: i32;
            Number4: i32;
            bool flag3;
            bool flag4;
            bool flag5;
            if (self.hq > -1)
            {
              Number2 = self.game.Data.UnitObj[self.hq].LandCap;
              Number3 = self.game.Data.UnitObj[self.hq].NavyCap;
              Number4 = self.game.Data.UnitObj[self.hq].AirCap;
              if ( self.game.Data.RuleVar[852] > 0.0)
              {
                let mut num3: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[ Math.Round( self.game.Data.RuleVar[851])];
                let mut num4: i32 =  Math.Round( Number2 / 1000.0 *  self.game.Data.RuleVar[852]);
                if (1 > num4)
                  num4 = 1;
                if (num4 > num3)
                {
                  Number2 =  Math.Round(Conversion.Int( Number2 * ( num3 /  num4)));
                  flag3 = true;
                }
                if (Number2 < 1)
                  Number2 = 0;
              }
              if ( self.game.Data.RuleVar[854] > 0.0)
              {
                let mut num5: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[ Math.Round( self.game.Data.RuleVar[853])];
                let mut num6: i32 =  Math.Round( Number3 / 1000.0 *  self.game.Data.RuleVar[854]);
                if (1 > num6)
                  num6 = 1;
                if (num6 > num5)
                {
                  Number3 =  Math.Round(Conversion.Int( Number3 * ( num5 /  num6)));
                  flag4 = true;
                }
                if (Number3 < 1)
                  Number3 = 0;
              }
              if ( self.game.Data.RuleVar[856] > 0.0)
              {
                let mut num7: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].RegimeSlot[ Math.Round( self.game.Data.RuleVar[855])];
                let mut num8: i32 =  Math.Round( Number4 / 1000.0 *  self.game.Data.RuleVar[856]);
                if (1 > num8)
                  num8 = 1;
                if (num8 > num7)
                {
                  Number4 =  Math.Round(Conversion.Int( Number4 * ( num7 /  num8)));
                  flag5 = true;
                }
                if (Number4 < 1)
                  Number4 = 0;
              }
            }
            let mut num9: i32 = 0;
            if (self.hq > -1)
            {
              if (self.CapTheater == 0)
                num9 = Number2;
              if (self.CapTheater == 1)
                num9 = Number3;
              if (self.CapTheater == 2)
                num9 = Number4;
            }
            num10: i32;
            if (self.CapTheater == 0)
              num10 = integer1;
            if (self.CapTheater == 1)
              num10 = integer2;
            if (self.CapTheater == 2)
              num10 = integer3;
            if (self.detailtype == 1 & self.unrT > -1 && self.detailnr != -2)
            {
              self.AirCarrier = false;
              self.game.HandyFunctionsObj.MakeMovePrediction2(self.game.Data.Turn, moveType, self.seltheater, ap, self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y, self.game.Data.UnitObj[self.unr].Map, false, muststartonairfield: false, istransfer: true, redux: redux, SFTypeX: self.tempSfType, SFTypeQty: Math.Max(1, self.TempNew));
              if (self.game.EditObj.TempValue[self.game.Data.UnitObj[self.unrT].Map].Value[self.game.Data.UnitObj[self.unrT].X, self.game.Data.UnitObj[self.unrT].Y] <= ap)
              {
                let mut num11: i32 = 0;
                if (self.seltheater == 2)
                {
                  let mut location: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.UnitObj[self.unrT].X, self.game.Data.UnitObj[self.unrT].Y].Location;
                  if (location > -1 && self.game.Data.LocTypeObj[self.game.Data.LocObj[location].Type].IsAirfield)
                    num11 = 1;
                  let mut landscapeType: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.UnitObj[self.unrT].X, self.game.Data.UnitObj[self.unrT].Y].LandscapeType;
                  if (self.game.HandyFunctionsObj.GetAirCarryCapPts(self.unrT) > 0)
                  {
                    num11 = 1;
                    self.AirCarrier = true;
                  }
                }
                else
                  num11 = 1;
                if (num11 == 1)
                {
                  self.OwnPowerTransfer = 1;
                  num10 = 0;
                  Number1 = 0;
                }
              }
              else
              {
                if (!self.game.Data.UnitObj[self.unr].IsHQ && self.seltheater == 1)
                  Number1 = 9999;
                if (self.game.HandyFunctionsObj.GetAirCarryCapPts(self.unrT) > 0)
                  self.AirCarrier = true;
              }
              if (self.unrT != self.hq & self.detailnr != -2 && self.game.HandyFunctionsObj.GetUnitSFNr(self.unrT, self.game.Data.SFObj[self.detailnr].Type, self.game.Data.SFObj[self.detailnr].People) == -1)
              {
                if (self.game.Data.UnitObj[self.unrT].SFCount > 6 & !self.game.Data.UnitObj[self.unrT].IsHQ)
                  Number1 = 9999;
                if (!self.game.Data.UnitObj[self.unrT].IsHQ && self.game.Data.UnitObj[self.unrT].SFCount + self.game.Data.UnitObj[self.unrT].PassengerCounter + 1 > 6)
                  Number1 = 9999;
              }
            }
            self.OrderOKId = 0;
            self.OrderALLId = 0;
            str1: String = "";
            str2: String = "";
            if (self.seltheater == 1 & self.OwnPowerTransfer < 1 & self.detailnr > -1 && self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].Theater == 1)
            {
              Number1 = 9999;
              self.NavyCost =  9999;
              self.LandCost =  9999;
              self.AirCost =  9999;
              str1 = "navy can only be transferred under 'own power'";
            }
            if (self.unrT > -1 & self.detailnr > -1 && !self.game.HandyFunctionsObj.CanAddTroops(self.unrT, self.game.Data.SFObj[self.detailnr].Type, self.game.Data.SFObj[self.detailnr].People, self.game.Data.SFObj[self.detailnr].MoveType))
            {
              Number1 = 9999;
              self.NavyCost =  9999;
              self.LandCost =  9999;
              self.AirCost =  9999;
              str1 = "target unit is full. these troops cannot be added";
            }
            if ( self.game.Data.RuleVar[801] == 1.0)
            {
              let mut reinforcementType: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].ReinforcementType;
              if (self.unrT > -1 && reinforcementType > -1 & self.game.Data.UnitObj[self.unrT].Historical > -1 && self.game.Data.UnitObj[self.unrT].HistoricalSubPart > -1)
              {
                let mut reinforcementPoints: i32 = self.game.HandyFunctionsObj.GetPowerInReinforcementPoints(self.game.HandyFunctionsObj.GetPreDef(self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.unrT].Historical].SubParts[self.game.Data.UnitObj[self.unrT].HistoricalSubPart]), reinforcementType);
                if (self.game.HandyFunctionsObj.GetPowerInReinforcementPoints(self.unrT, reinforcementType) + self.TempNew > reinforcementPoints)
                  Number1 = 9999;
              }
            }
            if (Operators.CompareString(str1, "", false) == 0 & num9 >= num10 & self.unrT > -1)
            {
              if (self.TempNew >= 1)
              {
                if (Number1 < 9999)
                {
                  if (self.detailnr == -2 & self.unrT > -1)
                  {
                    let mut num12: i32 = self.game.Data.UnitObj[self.unr].Supply;
                    if (!self.game.Data.UnitObj[self.unrT].IsHQ)
                    {
                      let mut num13: i32 = self.game.HandyFunctionsObj.UnitSupplyStore(self.unrT) - self.game.Data.UnitObj[self.unrT].Supply;
                      if (0 > num13)
                        num13 = 0;
                      if (num12 > num13)
                        num12 = num13;
                    }
                    if (num12 > 0 & self.game.Data.UnitObj[self.unr].Supply > 0)
                    {
                      tsubpart8 =  new TextButtonPartClass("Do Transfer", 100, "Click to transfer selected amount",  self.OwnBitmap, 300, 130);
                      self.OrderOKId = self.AddSubPart( tsubpart8, 300, 130, 100, 35, 1);
                    }
                  }
                  else if (!(self.detailnr == -2 & self.game.Data.UnitObj[self.unr].Supply <= 0))
                  {
                    tsubpart8 =  new TextButtonPartClass("Do Transfer", 100, "Click to transfer selected amount",  self.OwnBitmap, 300, 130);
                    self.OrderOKId = self.AddSubPart( tsubpart8, 300, 130, 100, 35, 1);
                  }
                }
              }
              else
                str1 = "You must select at least 1 individual for transfer";
            }
            if (Operators.CompareString(str1, "", false) != 0 & Number1 >= 9999)
              str1 = "basiccost => 9999, thus not possible";
            if (self.unrT > -1 & self.unr > -1 && self.game.Data.UnitObj[self.unr].SFCount > -1 & Number1 < 9999 && self.game.Data.UnitObj[self.unrT].X == self.game.Data.UnitObj[self.unr].X & self.game.Data.UnitObj[self.unrT].Y == self.game.Data.UnitObj[self.unr].Y)
            {
              let mut num14: i32 = 1;
              if (self.game.Data.UnitObj[self.unr].PassengerCounter > -1)
              {
                num14 = 0;
                str2 = "You cannot transfer all from a unit with passengers";
              }
              if (!self.game.Data.UnitObj[self.unrT].IsHQ && self.game.Data.UnitObj[self.unr].SFCount + 1 + self.game.Data.UnitObj[self.unrT].SFCount + self.game.Data.UnitObj[self.unrT].PassengerCounter + 1 > 7)
              {
                num14 = 0;
                str2 = "Maximum 8 subformations in target unit.";
              }
              if (num14 == 1)
              {
                tsubpart8 =  new TextButtonPartClass("Transfer All", 100, "Click to transfer all contents of source unit",  self.OwnBitmap, 410, 130);
                self.OrderALLId = self.AddSubPart( tsubpart8, 410, 130, 100, 35, 1);
              }
            }
            if (Operators.CompareString(str1, "", false) == 0)
              str1 = "not possible";
            if (Operators.CompareString(str2, "", false) == 0)
              str2 = "not possible";
            if (self.OrderOKId == 0)
            {
              tsubpart8 =  new TextButtonPartClass("Do Transfer", 100, str1,  self.OwnBitmap, 400, 130, true);
              self.x1id = self.AddSubPart( tsubpart8, 300, 130, 100, 35, 1);
            }
            if (self.OrderALLId == 0)
            {
              tsubpart8 =  new TextButtonPartClass("Transfer All", 100, str2,  self.OwnBitmap, 410, 130, true);
              self.x2id = self.AddSubPart( tsubpart8, 410, 130, 100, 35, 1);
            }
            let mut num15: i32 = 1;
            let mut tsmallchange: i32 = 1;
            if (self.detailnr == -2)
              tsmallchange = 10;
            if (Number1 == 9999)
              num15 = 0;
            if (num9 < Number1 & self.OwnPowerTransfer == 0)
              num15 = 0;
            if (num15 == 1 & self.unrT > -1)
            {
              if (self.detailnr == -2)
              {
                let mut tmaxval: i32 = self.game.Data.UnitObj[self.unr].Supply;
                if (!self.game.Data.UnitObj[self.unrT].IsHQ)
                {
                  let mut num16: i32 = self.game.HandyFunctionsObj.UnitSupplyStore(self.unrT) - self.game.Data.UnitObj[self.unrT].Supply;
                  if (0 > num16)
                    num16 = 0;
                  if (tmaxval > num16)
                    tmaxval = num16;
                }
                if (tmaxval > 0 & self.sliderID <= 0)
                {
                  tsubpart8 =  new NumberSliderSubPartClass2(self.game, "", "x Supply Pts", 205, 0, tmaxval, self.TempNew, tsmallchange: tsmallchange, tbackbitmap: ( self.OwnBitmap), bbx: 300, bby: 75);
                  self.sliderID = self.AddSubPart( tsubpart8, 300, 75, 205, 40, 0);
                }
              }
              else if (self.sliderID <= 0)
              {
                tsubpart8 =  new NumberSliderSubPartClass2(self.game, "", "x " + self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].Name, 205, 0, self.game.Data.SFObj[self.detailnr].Qty, self.TempNew, tbackbitmap: ( self.OwnBitmap), bbx: 300, bby: 75);
                self.sliderID = self.AddSubPart( tsubpart8, 300, 75, 205, 40, 0);
              }
            }
            txt3: String;
            if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectLess(self.LandCost,  9999, false),  (self.seltheater != 1))))
            {
              txt3 = self.hq <= -1 ? "LandCap = " + Conversion.Str( integer1) : "LandCap = " + Conversion.Str( integer1) + " / " + Conversion.Str( Number2);
              if (flag3)
                txt3 += " (fuel!)";
            }
            else
              txt3 = "No Land Connect";
            if (self.unrT == -1)
              txt3 = "Land";
            if (self.unrT > -1)
            {
              if (self.CapTheater == 0)
              {
                tsubpart8 =  new SteveButtonPartClass(self.game.OKBALL, tDescript: "Click to use Land Cap for this transfer", tBackbitmap: ( self.OwnBitmap), bbx: 760, bby: 35);
                self.B4Id = self.AddSubPart( tsubpart8, 760, 45, 35, 35, 1);
                tsubpart8 =  new ATTextPartClass(txt3, self.game.VicFont2, 190, 20, true, tBlackBack: true);
                self.text4id = self.AddSubPart( tsubpart8, 800, 55, 190, 20, 0);
              }
              else
              {
                tsubpart8 =  new SteveButtonPartClass(self.game.CANCELBALL, tDescript: "Click to use Land Cap for this transfer", tBackbitmap: ( self.OwnBitmap), bbx: 760, bby: 35);
                self.B4Id = self.AddSubPart( tsubpart8, 760, 45, 35, 35, 1);
                tsubpart8 =  new ATTextPartClass(txt3, self.game.VicFont2, 190, 20, true, tBlackBack: true);
                self.text4id = self.AddSubPart( tsubpart8, 800, 55, 190, 20, 0);
              }
            }
            txt4: String;
            if (Operators.ConditionalCompareObjectLess(self.NavyCost,  9999, false))
            {
              if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(self.NavyCost,  0, false),  (self.seltheater == 1 & Number1 == 0))))
              {
                txt4 = "Self Transfer";
              }
              else
              {
                txt4 = self.hq <= -1 ? "NavyCap = " + Conversion.Str( integer2) : "NavyCap = " + Conversion.Str( integer2) + " / " + Conversion.Str( Number3);
                if (flag4)
                  txt4 += " (fuel!)";
              }
            }
            else
              txt4 = "No Navy Connect";
            if (self.unrT == -1)
              txt4 = "Navy";
            if (self.unrT > -1)
            {
              if (self.CapTheater == 1)
              {
                tsubpart8 =  new SteveButtonPartClass(self.game.OKBALL, tDescript: "Click to use Navy Cap for this transfer", tBackbitmap: ( self.OwnBitmap), bbx: 760, bby: 110);
                self.B5Id = self.AddSubPart( tsubpart8, 760, 90, 35, 35, 1);
                tsubpart8 =  new ATTextPartClass(txt4, self.game.VicFont2, 190, 20, true, tBlackBack: true);
                self.text5id = self.AddSubPart( tsubpart8, 800, 100, 190, 20, 0);
              }
              else
              {
                tsubpart8 =  new SteveButtonPartClass(self.game.CANCELBALL, tDescript: "Click to use Navy Cap for this transfer", tBackbitmap: ( self.OwnBitmap), bbx: 760, bby: 110);
                self.B5Id = self.AddSubPart( tsubpart8, 760, 90, 35, 35, 1);
                tsubpart8 =  new ATTextPartClass(txt4, self.game.VicFont2, 190, 20, true, tBlackBack: true);
                self.text5id = self.AddSubPart( tsubpart8, 800, 100, 190, 20, 0);
              }
            }
            if ( self.game.Data.RuleVar[509] == 0.0)
            {
              txt5: String;
              if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectLess(self.AirCost,  9999, false),  ( self.game.Data.RuleVar[509] == 0.0))))
              {
                txt5 = self.hq <= -1 ? "RailCap = " + Conversion.Str( integer3) : "RailCap = " + Conversion.Str( integer3) + " / " + Conversion.Str( Number4);
                if (flag5)
                  txt5 += " (fuel!)";
              }
              else
                txt5 = "No Rail Connect";
              if (self.unrT == -1)
                txt5 = "Rail";
              if (self.unrT > -1)
              {
                if (self.CapTheater == 2)
                {
                  tsubpart8 =  new SteveButtonPartClass(self.game.OKBALL, tDescript: "Click to use Rail Cap for this transfer", tBackbitmap: ( self.OwnBitmap), bbx: 760, bby: 135);
                  self.B6Id = self.AddSubPart( tsubpart8, 760, 135, 35, 35, 1);
                  tsubpart8 =  new ATTextPartClass(txt5, self.game.VicFont2, 190, 20, true, tBlackBack: true);
                  self.text6id = self.AddSubPart( tsubpart8, 800, 145, 190, 20, 0);
                }
                else
                {
                  tsubpart8 =  new SteveButtonPartClass(self.game.CANCELBALL, tDescript: "Click to use Rail Cap for this transfer", tBackbitmap: ( self.OwnBitmap), bbx: 760, bby: 135);
                  self.B6Id = self.AddSubPart( tsubpart8, 760, 135, 35, 35, 1);
                  tsubpart8 =  new ATTextPartClass(txt5, self.game.VicFont2, 190, 20, true, tBlackBack: true);
                  self.text6id = self.AddSubPart( tsubpart8, 800, 145, 190, 20, 0);
                }
              }
            }
            txt6: String = self.OwnPowerTransfer != 0 ? "Own Power" : (self.hq != -1 ? "Basic Cost = " + Conversion.Str( Number1) : "Not enough Ap");
            if (self.unrT > -1)
            {
              tsubpart8 =  new ATTextPartClass(txt6, self.game.VicFont2, 205, 20, true, tBlackBack: true);
              self.Text3Id = self.AddSubPart( tsubpart8, 300, 47, 205, 20, 0);
            }
          }
        }
        if (!flag2)
        {
          tsubpart8 =  new ATTextPartClass("Target of transfer must be a lower type of HQ.", self.game.VicFont2, 420, 25, false);
          self.OrderOKTextId = self.AddSubPart( tsubpart8, 305, 99, 420, 25, 0);
        }
        self.SetTempValue();
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.YesId)
            {
              self.game.EditObj.TransferLostQty = -1;
              self.game.EditObj.TransferLostType = -1;
              self.game.EditObj.TransferLostTransports = -1;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId)
            {
              let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num2 > -1 | num2 == -2)
              {
                if (self.sliderID > 0)
                {
                  self.RemoveSubPart(self.sliderID);
                  self.sliderID = -1;
                }
                self.detailnr = num2;
                self.TempNew = 1;
                self.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList2Id)
            {
              self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B4Id)
            {
              self.CapTheater = 0;
              self.SetTempValue();
              self.dostuff();
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B5Id)
            {
              self.CapTheater = 1;
              self.SetTempValue();
              self.dostuff();
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B6Id)
            {
              self.CapTheater = 2;
              self.SetTempValue();
              self.dostuff();
              self.game.EditObj.TempCoordList = CoordList::new();
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Type1Id)
            {
              self.detailtype = 1;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Type2Id)
            {
              self.detailtype = 2;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.Type3Id)
            {
              self.detailtype = 3;
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.SwitchId)
            {
              self.overrulehq = self.unr != self.overrulehq ? self.unr : self.unrT;
              self.detailnr = -1;
              self.TempNew = 1;
              self.DoNewStuff();
              self.dostuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OrderOKId)
            {
              if (self.detailtype == 1)
              {
                if (self.OwnPowerTransfer == 1)
                {
                  orderResult = (OrderResult) self.game.ProcessingObj.DoTransfer(self.unr, self.unrT, self.CapTheater, self.detailnr, self.TempNew, true);
                }
                else
                {
                  if (self.game.EditObj.SoundOn)
                    SoundMod.PlayAWave(self.game.AppPath + "sound/transfer.wav",  self.game.EditObj);
                  orderResult = (OrderResult) self.game.ProcessingObj.DoTransfer(self.unr, self.unrT, self.CapTheater, self.detailnr, self.TempNew, byHQ: self.overrulehq);
                }
              }
              windowReturnClass.AddCommand(4, 66);
              if (orderResult.OK)
              {
                self.TempNew = 1;
                if (self.sliderID > 0)
                {
                  self.RemoveSubPart(self.sliderID);
                  self.sliderID = 0;
                }
                self.dostuff();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.SetFlag(false);
              return windowReturnClass;
            }
            if (num1 == self.OrderALLId)
            {
              if (Interaction.MsgBox( "Are you sure you want to transfer all subformations?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                for (let mut sfCount: i32 = self.game.Data.UnitObj[self.unr].SFCount; sfCount >= 0; sfCount += -1)
                {
                  let mut sf: i32 = self.game.Data.UnitObj[self.unr].SFList[sfCount];
                  let mut qty: i32 = self.game.Data.SFObj[sf].Qty;
                  orderResult = (OrderResult) self.game.ProcessingObj.DoTransfer(self.unr, self.unrT, 0, sf, qty, true, false);
                }
                if (orderResult.OK)
                {
                  self.TempNew = 1;
                  if (self.sliderID > 0)
                  {
                    self.RemoveSubPart(self.sliderID);
                    self.sliderID = 0;
                  }
                  self.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.SetFlag(false);
                return windowReturnClass;
              }
            }
            else if (num1 == self.sliderID)
            {
              let mut tempNew: i32 = self.TempNew;
              self.TempNew = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index], b);
              if (self.detailnr == -2)
              {
                if (self.TempNew > self.game.Data.UnitObj[self.unr].Supply)
                  self.TempNew = self.game.Data.UnitObj[self.unr].Supply;
              }
              else if (self.TempNew > self.game.Data.SFObj[self.detailnr].Qty)
                self.TempNew = self.game.Data.SFObj[self.detailnr].Qty;
              if (self.detailnr != -2)
              {
                if ( self.game.Data.RuleVar[801] == 1.0)
                {
                  let mut reinforcementType: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].ReinforcementType;
                  if (reinforcementType > -1 & self.game.Data.UnitObj[self.unrT].Historical > -1 && self.game.Data.UnitObj[self.unrT].HistoricalSubPart > -1)
                  {
                    let mut reinforcementPoints: i32 = self.game.HandyFunctionsObj.GetPowerInReinforcementPoints(self.game.HandyFunctionsObj.GetPreDef(self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.unrT].Historical].SubParts[self.game.Data.UnitObj[self.unrT].HistoricalSubPart]), reinforcementType);
                    if (self.game.HandyFunctionsObj.GetPowerInReinforcementPoints(self.unrT, reinforcementType) + self.TempNew > reinforcementPoints)
                    {
                      self.TempNew = tempNew;
                      self.game.EditObj.FeedBackString = "Maximum " + Conversion.Str( reinforcementPoints) + " is powerpts of " + self.game.Data.ReinfName[reinforcementType];
                      windowReturnClass.AddCommand(4, 29);
                    }
                  }
                }
                if (self.AirCarrier & self.seltheater == 2)
                {
                  if (self.TempNew * self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].Weight > self.game.HandyFunctionsObj.GetAirCarryCapPts(self.game.EditObj.OrderTarget) - self.game.HandyFunctionsObj.GetAirCarryCapPtsOccupied(self.game.EditObj.OrderTarget))
                  {
                    self.TempNew = tempNew;
                    self.game.EditObj.FeedBackString = "No more place on aircarrier";
                    windowReturnClass.AddCommand(4, 29);
                  }
                }
                else if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y].LandscapeType].IsSea && self.OwnPowerTransfer == 0)
                {
                  self.TempNew = tempNew;
                  self.game.EditObj.FeedBackString = "Cannot transfer to open sea";
                  windowReturnClass.AddCommand(4, 29);
                }
              }
              self.SubPartFlag[index] = true;
              self.dostuff();
              windowReturnClass.SetFlag(true);
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

    pub HandleMouseUp: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (self.SubPartList[index].Scroller)
          {
            let mut num: i32 = self.SubPartID[index];
            if (num == self.OptionsListId)
              self.SubPartList[index].HandleMouseUp(x - self.SubPartX[index], y - self.SubPartY[index]);
            else if (num == self.OptionsList2Id)
              self.SubPartList[index].HandleMouseUp(x - self.SubPartX[index], y - self.SubPartY[index]);
            else if (num == self.sliderID)
            {
              let mut tempNew: i32 = self.TempNew;
              self.TempNew = self.SubPartList[index].HandleMouseUp(x - self.SubPartX[index], y - self.SubPartY[index]);
              if (self.detailnr == -2)
              {
                if (self.TempNew > self.game.Data.UnitObj[self.unr].Supply)
                  self.TempNew = self.game.Data.UnitObj[self.unr].Supply;
              }
              else if (self.TempNew > self.game.Data.SFObj[self.detailnr].Qty)
                self.TempNew = self.game.Data.SFObj[self.detailnr].Qty;
              if (self.detailnr != -2)
              {
                if (!self.game.HandyFunctionsObj.CanUnitReceiveTransfer(self.game.EditObj.OrderTarget, self.game.Data.SFObj[self.detailnr].Type, self.TempNew, self.game.Data.SFObj[self.detailnr].People))
                {
                  self.TempNew = tempNew;
                  windowReturnClass.AddCommand(4, 29);
                  self.dostuff();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (self.AirCarrier & self.seltheater == 2)
                {
                  if (self.TempNew * self.game.Data.SFTypeObj[self.game.Data.SFObj[self.detailnr].Type].Weight > self.game.HandyFunctionsObj.GetAirCarryCapPts(self.game.EditObj.OrderTarget) - self.game.HandyFunctionsObj.GetAirCarryCapPtsOccupied(self.game.EditObj.OrderTarget))
                  {
                    self.TempNew = tempNew;
                    self.game.EditObj.FeedBackString = "No more place on aircarrier";
                    windowReturnClass.AddCommand(4, 29);
                  }
                }
                else if (self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.UnitObj[self.game.EditObj.OrderTarget].X, self.game.Data.UnitObj[self.game.EditObj.OrderTarget].Y].LandscapeType].IsSea && self.OwnPowerTransfer == 0)
                {
                  self.TempNew = tempNew;
                  self.game.EditObj.FeedBackString = "Cannot transfer to open sea";
                  windowReturnClass.AddCommand(4, 29);
                }
              }
              self.SubPartFlag[index] = true;
              self.dostuff();
              windowReturnClass.SetFlag(true);
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
