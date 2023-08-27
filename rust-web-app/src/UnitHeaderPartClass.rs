// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UnitHeaderPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class UnitHeaderPartClass : SubPartClass
  {
     object OwnBitmapNr;
     unr: i32;
     game: GameClass;
     his: i32;

    pub UnitHeaderPartClass(tunr: i32, tgame: GameClass)
      : base(280, 200)
    {
      self.unr = tunr;
      self.game = tgame;
      self.his = self.game.Data.UnitObj[self.unr].Historical;
    }

    pub fn DescriptInfo(x: i32, y: i32)
    {
      if (self.game.EditObj.UnitSelected == -1)
        return;
      self.Descript = "";
      if (x > 105 & y > 5 & x < 140 & y < 45)
        self.Descript = "Action Points";
      if (x > 0 & y > 152 & x < 70 & y < 204)
        self.Descript = "Supply % consumed at start of this turn.";
      if (x > 140 & y > 5 & x < 175 & y < 45)
        self.Descript = "Readiness.";
      if (x > 175 & y > 5 & x < 210 & y < 45)
        self.Descript = "Experience.";
      if (x > 210 & y > 5 & x < 245 & y < 45)
        self.Descript = "Morale.";
      if (x > 245 & y > 5 & x < 280 & y < 45)
        self.Descript = "Entrenchment.";
      if (x > 125 & y > 123 & x < 277 & y < 143)
      {
        let mut hq: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ;
        if (self.game.Data.Round != 0 & self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime != self.game.Data.Turn & self.game.Data.FOWOn)
        {
          self.Descript = "HQ power and Staff points in HQ expressed as a % of needed staff points. ";
        }
        else
        {
          Number1: i32;
          Number2: i32;
          if (hq == -1 | self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
          {
            Number1 = 0;
            Number2 = 0;
          }
          else
          {
            let mut num1: i32 = self.game.HandyFunctionsObj.GetStaffPercent(hq, true);
            let mut num2: i32 = self.game.HandyFunctionsObj.GetStaffPercent(hq, true);
            let mut num3: i32 = self.game.HandyFunctionsObj.GetStaffPercent(hq);
            let mut num4: i32 = self.game.HandyFunctionsObj.GetStaffPercent(hq);
            if (num1 > 100)
              num1 = 100;
            if (num2 > 100)
              num2 = 100;
            if (num3 > 100)
              num3 = 100;
            if (num4 > 100)
              num4 = 100;
            Number1 =  Math.Round(  Math.Round( num1 *  self.game.HandyFunctionsObj.GetStaffCombatMod(hq) * ( self.game.HandyFunctionsObj.Gethqpow(self.game.EditObj.UnitSelected) / 100.0)) +  num3 *  self.game.Data.RuleVar[140] * ( self.game.HandyFunctionsObj.Gethqpow(self.game.EditObj.UnitSelected) / 100.0));
            Number2 =  Math.Round(  Math.Round( num2 *  self.game.HandyFunctionsObj.GetStaffMoraleMod(hq) * ( self.game.HandyFunctionsObj.Gethqpow(self.game.EditObj.UnitSelected) / 100.0)) +  num4 *  self.game.Data.RuleVar[141] * ( self.game.HandyFunctionsObj.Gethqpow(self.game.EditObj.UnitSelected) / 100.0));
          }
          if (self.game.HandyFunctionsObj.HasUnitlandSF(self.unr))
            self.Descript = "Staff points in HQ expressed as a % of needed staff points. Current CombatMod = +" + Conversion.Str( Number1) + "%. Current MoraleMod = +" + Conversion.Str( Number2) + "%";
          else
            self.Descript = "Staff only gives bonuses to land theater subformations. not to air and navy.";
        }
      }
      if (x > 125 & y > 70 & x < 277 & y < 88)
        self.Descript = "Click to change name of unit.";
      if (x > 125 & y > 90 & x < 277 & y < 108)
      {
        if (self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn)
          self.Descript = "Click to jump to units HQ.";
        else
          self.Descript = "";
      }
      if (x > 0 & y > 65 & x < 44 & y < 103)
        self.Descript = "For all practical purposes this unit travels with the movement speed this movement type";
      if (x > 0 & y > 105 & x < 44 & y < 132)
        self.Descript = "The regime controlling the unit. " + self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].Name;
      if (x > 44 & y > 60 & x < 122 & y < 138)
      {
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
          self.Descript = "The selected unit. Click to change of: Color hq.";
        else
          self.Descript = "The selected unit.";
      }
      if (x > 70 & y > 152 & x < 140 & y < 204)
        self.Descript = "Supply Stock. The total number of supply points with the unit.";
      if (x > 140 & y > 152 & x < 210 & y < 204)
      {
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
          self.Descript = "Supply Requested Out";
        else
          self.Descript = "Supply Requested In";
      }
      if (!(x > 210 & y > 152 & x < 280 & y < 204))
        return;
      if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
        self.Descript = "Supply Send Out";
      else
        self.Descript = "Supply Received In";
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Coordinate coordinate = Coordinate::new();
      if (self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn | self.game.Data.Round == 0)
        coordinate.x = 3;
      else
        coordinate = self.game.HandyFunctionsObj.GetReconMinusHide(self.unr, self.game.Data.Turn);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      Conversions.ToInteger(self.game.HandyFunctionsObj.GetUnitPeople(self.unr));
      let mut regime: i32 = self.game.Data.UnitObj[self.unr].Regime;
      str1: String = self.game.Data.UnitObj[self.unr].Name;
       let mut local1: &Graphics = &graphics;
      Rectangle rectangle1 = Rectangle::new(0, 35, 150, 14);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2;
      let mut rect2_1: &Rectangle = &rectangle2
      DrawMod.MakeFullBoxVic2( local1, rect1_1, "SELECTED UNIT", rect2_1, "");
      let mut num1: i32 = 45;
      DrawMod.DrawBlock( graphics, 0, 5 + num1, 280, 95,  self.game.VicColor3Shade.R,  self.game.VicColor3Shade.G,  self.game.VicColor3Shade.B,  self.game.VicColor3Shade.A);
      DrawMod.DrawRectangle( graphics, 0, 5 + num1, 279, 95,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
      self.game.CustomBitmapObj.DrawUnitBig(self.unr, true, graphics, 44, 15 + num1, true);
      if ( self.game.Data.RuleVar[344] == 1.0 &&  self.game.Data.RuleVar[337] == 1.0)
      {
        if (self.game.Data.UnitObj[self.unr].Historical > -1)
        {
          if (self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.unr].Historical].ModelMaster > -1)
          {
            str2: String = str1 + " (" + Strings.Left(self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.unr].Historical].ModelMaster].Name, 3);
            if (self.game.Data.UnitObj[self.unr].HistoricalSubPart > -1)
              str2 = str2 + ", " + Strings.Left(self.game.Data.UnitObj[self.game.HandyFunctionsObj.GetPreDef(self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.unr].Historical].SubParts[self.game.Data.UnitObj[self.unr].HistoricalSubPart])].Name, 3);
            str1 = str2 + ")";
          }
          else
            str1 += " (ad hoc)";
        }
        else
          str1 += " (remnant)";
      }
      if (coordinate.x < 2)
        str1 = "?";
      str3: String = str1;
      str4: String;
      if (self.game.Data.UnitObj[self.unr].HQ > -1)
      {
        str4 = self.game.Data.UnitObj[self.game.Data.UnitObj[self.unr].HQ].Name;
        if (coordinate.x < 2)
          str4 = "HQ: ?";
      }
      else
      {
        str4 = "(has no HQ)";
        if (coordinate.x < 2)
          str4 = "HQ: ?";
      }
      str5: String = str4;
      str6: String = "";
      SizeF sizeF2;
      if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, self.game.Data.UnitObj[self.unr].Regime))
      {
        if (!self.game.Data.UnitObj[self.unr].IsHQ)
        {
          str7: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.Gethqpow(self.unr)));
          if (coordinate.x <= 2)
            str7 = "?";
          if (coordinate.x <= 1)
            str7 = "?";
          str6 = "HQP: " + str7 + "%" + ", ";
        }
        str8: String;
        if (!self.game.Data.UnitObj[self.unr].IsHQ)
        {
          str8 = self.game.Data.UnitObj[self.unr].HQ <= -1 ? "N/A" : Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetStaffPercent(self.game.Data.UnitObj[self.unr].HQ)));
          if (coordinate.x <= 2)
            str8 = "?";
        }
        else
        {
          str8 = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetStaffPercent(self.unr)));
          if (coordinate.x <= 2)
            str8 = "?";
        }
        if (coordinate.x <= 1)
          str8 = "?";
        str9: String = str6 + "STF: " + str8 + "%";
        sizeF2 = graphics.MeasureString(str3, self.game.VicFont2);
        let mut num2: i32 =  Math.Round((152.0 -  sizeF2.Width) / 2.0);
        if ( sizeF2.Width > 145.0)
        {
          sizeF2 = graphics.MeasureString(str3, self.game.VicFont4);
          let mut num3: i32 =  Math.Round((152.0 -  sizeF2.Width) / 2.0);
          DrawMod.DrawTextVic2( graphics, str3, self.game.VicFont4, 125 + num3, 25 + num1, self.game.VicColor2, self.game.VicColor2Shade);
        }
        else
          DrawMod.DrawTextVic2( graphics, str3, self.game.VicFont2, 125 + num2, 25 + num1, self.game.VicColor2, self.game.VicColor2Shade);
        sizeF2 = graphics.MeasureString(str9, self.game.VicFont3);
        let mut num4: i32 =  Math.Round((152.0 -  sizeF2.Width) / 2.0);
        DrawMod.DrawTextVic2( graphics, str9, self.game.VicFont3, 125 + num4, 78 + num1, self.game.VicColor1, self.game.VicColor1Shade);
        sizeF2 = graphics.MeasureString(str5, self.game.VicFont3);
        let mut num5: i32 =  Math.Round((152.0 -  sizeF2.Width) / 2.0);
        if ( sizeF2.Width > 145.0)
        {
          sizeF2 = graphics.MeasureString(str5, self.game.VicFont4);
          let mut num6: i32 =  Math.Round((152.0 -  sizeF2.Width) / 2.0);
          DrawMod.DrawTextVic2( graphics, str5, self.game.VicFont4, 125 + num6, 45 + num1, self.game.VicColor1, self.game.VicColor1Shade);
        }
        else
          DrawMod.DrawTextVic2( graphics, str5, self.game.VicFont3, 125 + num5, 45 + num1, self.game.VicColor1, self.game.VicColor1Shade);
      }
      else
      {
        str10: String = "HQPW: ?, STF: ?";
        let mut num7: i32 =  Math.Round((152.0 -  graphics.MeasureString(str10, self.game.VicFont3).Width) / 2.0);
        DrawMod.DrawTextVic2( graphics, str10, self.game.VicFont3, 125 + num7, 78 + num1, self.game.VicColor1, self.game.VicColor1Shade);
        let mut num8: i32 =  Math.Round((152.0 -  graphics.MeasureString(str3, self.game.VicFont2).Width) / 2.0);
        DrawMod.DrawTextVic2( graphics, str3, self.game.VicFont2, 125 + num8, 25 + num1, self.game.VicColor2, self.game.VicColor2Shade);
        let mut num9: i32 =  Math.Round((152.0 -  graphics.MeasureString(str4, self.game.VicFont3).Width) / 2.0);
        DrawMod.DrawTextVic2( graphics, str4, self.game.VicFont3, 125 + num9, 45 + num1, self.game.VicColor1, self.game.VicColor1Shade);
      }
      let mut num10: i32 = 0;
      bitmap1: Bitmap;
      if (self.game.Data.Turn == self.game.Data.UnitObj[self.unr].Regime | !self.game.Data.FOWOn | self.game.Data.Round == 0 | coordinate.x >= 2)
      {
        num10 = 1;
        let mut nr: i32 = -1;
        let mut index1: i32 = self.game.HandyFunctionsObj.GetLowestSpeed(self.unr, -1, true);
        if (index1 > -1)
        {
          let mut sfCount: i32 = self.game.Data.UnitObj[self.unr].SFCount;
          for (nr = 0; nr <= sfCount; nr += 1)
          {
            if (self.game.Data.SFTypeObj[self.game.Data.SFObj[self.game.Data.UnitObj[self.unr].SFList[nr]].Type].SymbolGroup == self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].SymbolGroup && self.game.Data.SFTypeObj[self.game.Data.SFObj[self.game.Data.UnitObj[self.unr].SFList[nr]].Type].SymbolWeight > self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].SymbolWeight && self.game.Data.SFTypeObj[self.game.Data.SFObj[self.game.Data.UnitObj[self.unr].SFList[nr]].Type].MoveType == self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].MoveType)
              index1 = self.game.Data.UnitObj[self.unr].SFList[nr];
          }
        }
        str11: String = index1 <= -1 ? "Immobile" : (self.game.Data.SFObj[index1].MoveType != -1 ? Strings.Trim(self.game.Data.TempString[self.game.Data.SFObj[index1].MoveType]) : Strings.Trim(self.game.Data.TempString[self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].MoveType]));
        if (index1 == -2)
          nr = self.game.SUPPLIESSYMBOL;
        else if (index1 > -1)
          nr = self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].SymbolSpriteID;
        let mut integer: i32 = Conversions.ToInteger(self.game.HandyFunctionsObj.GetUnitPeople(self.unr));
        if (regime > -1 & index1 > -1)
        {
          if (self.game.Data.RegimeObj[regime].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].ExtraCounter;
            for (let mut index2: i32 = 0; index2 <= extraCounter; index2 += 1)
            {
              if (self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].ExtraCode[index2] == self.game.Data.RegimeObj[regime].ExtraGraphicUse)
                nr = self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].ExtraSymbolSpriteID[index2];
            }
          }
          else if (self.game.Data.PeopleObj[integer].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].ExtraCounter;
            for (let mut index3: i32 = 0; index3 <= extraCounter; index3 += 1)
            {
              if (self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].ExtraCode[index3] == self.game.Data.PeopleObj[integer].ExtraGraphicUse)
                nr = self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].ExtraSymbolSpriteID[index3];
            }
          }
        }
        if (coordinate.x < 3)
          nr = -1;
        if (nr > -1 & index1 > -1)
        {
           let mut local2: &Graphics = &graphics;
          bitmap1 = BitmapStore.GetBitmap(nr);
           let mut local3: &Bitmap = &bitmap1;
          let mut y: i32 = 20 + num1;
          DrawMod.DrawSimple( local2,  local3, 2, y);
          str12: String = Strings.UCase(Strings.Left(self.game.Data.SFObj[index1].MoveType <= -1 ? self.game.Data.TempString[self.game.Data.SFTypeObj[self.game.Data.SFObj[index1].Type].MoveType] : self.game.Data.TempString[self.game.Data.SFObj[index1].MoveType], 5));
          sizeF2 = graphics.MeasureString(str12, self.game.VicFont5);
          DrawMod.DrawTextVic2( graphics, str12, self.game.VicFont5,  Math.Round(22.0 -  sizeF2.Width / 2.0), 43 + num1, self.game.VicColor1, self.game.VicColor1Shade);
        }
        else if (nr > -1)
        {
           let mut local4: &Graphics = &graphics;
          bitmap2: Bitmap = BitmapStore.GetBitmap(nr);
           let mut local5: &Bitmap = &bitmap2;
          let mut y: i32 = 20 + num1;
          DrawMod.DrawSimple( local4,  local5, 2, y);
          str13: String = Strings.UCase(Strings.Left(str11, 5));
          sizeF2 = graphics.MeasureString(str13, self.game.VicFont5);
          DrawMod.DrawTextVic2( graphics, str13, self.game.VicFont5,  Math.Round(22.0 -  sizeF2.Width / 2.0), 43 + num1, self.game.VicColor1, self.game.VicColor1Shade);
        }
      }
      let mut hqSpriteNr: i32 = self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].HQSpriteNr;
       let mut local6: &Graphics = &graphics;
      bitmap1 = BitmapStore.GetBitmap(hqSpriteNr);
       let mut local7: &Bitmap = &bitmap1;
      let mut y1: i32 = 60 + num1;
      DrawMod.DrawSimple( local6,  local7, 0, y1);
      if (self.game.Data.Turn == self.game.Data.UnitObj[self.unr].Regime | !self.game.Data.FOWOn | self.game.Data.Round == 0 | coordinate.x >= 1)
      {
        let mut num11: i32 = -105;
        str14: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetLowestAp(self.unr)));
        if (coordinate.x == 2)
          str14 = "?";
        if (coordinate.x <= 1)
          str14 = "?";
         let mut local8: &Graphics = &graphics;
        rectangle1 = Rectangle::new(105, num11 + 110, 30, 14);
        let mut rect1_2: &Rectangle = &rectangle1
        Rectangle rectangle3 = Rectangle::new(105, num11 + 124, 30, 23);
        let mut rect2_2: &Rectangle = &rectangle3
        txt2_1: String = str14;
        DrawMod.MakeFullBoxVic2( local8, rect1_2, "AP", rect2_2, txt2_1);
        let mut Number1: i32 = self.game.HandyFunctionsObj.GetAverageRdn(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num12 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num13 =  ((1.0 -  num12) * 2.0);
          float num14 = VBMath.Rnd() * num13 + num12;
          Number1 =  Math.Round( Conversion.Int( Number1 * num14));
          if (Number1 < 0)
            Number1 = 0;
          if (Number1 > 100)
            Number1 = 100;
        }
        str15: String = Strings.Trim(Conversion.Str( Number1));
        if (coordinate.x <= 1)
          str15 = "?";
         let mut local9: &Graphics = &graphics;
        rectangle3 = Rectangle::new(140, num11 + 110, 30, 14);
        let mut rect1_3: &Rectangle = &rectangle3
        rectangle1 = Rectangle::new(140, num11 + 124, 30, 23);
        let mut rect2_3: &Rectangle = &rectangle1
        txt2_2: String = str15;
        DrawMod.MakeFullBoxVic2( local9, rect1_3, "RDN", rect2_3, txt2_2);
        let mut Number2: i32 = self.game.HandyFunctionsObj.GetAverageXp(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num15 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num16 =  ((1.0 -  num15) * 2.0);
          float num17 = VBMath.Rnd() * num16 + num15;
          Number2 =  Math.Round( Conversion.Int( Number2 * num17));
          if (Number2 < 0)
            Number2 = 0;
          if (Number2 > 100)
            Number2 = 100;
        }
        str16: String = Strings.Trim(Conversion.Str( Number2));
        if (coordinate.x <= 1)
          str16 = "?";
         let mut local10: &Graphics = &graphics;
        rectangle3 = Rectangle::new(175, num11 + 110, 30, 14);
        let mut rect1_4: &Rectangle = &rectangle3
        rectangle1 = Rectangle::new(175, num11 + 124, 30, 23);
        let mut rect2_4: &Rectangle = &rectangle1
        txt2_3: String = str16;
        DrawMod.MakeFullBoxVic2( local10, rect1_4, "EXP", rect2_4, txt2_3);
        let mut Number3: i32 = self.game.HandyFunctionsObj.GetAverageMor(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num18 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num19 =  ((1.0 -  num18) * 2.0);
          float num20 = VBMath.Rnd() * num19 + num18;
          Number3 =  Math.Round( Conversion.Int( Number3 * num20));
          if (Number3 < 0)
            Number3 = 0;
          if (Number3 > 999)
            Number3 = 999;
        }
        str17: String = Strings.Trim(Conversion.Str( Number3));
        if (coordinate.x <= 1)
          str17 = "?";
         let mut local11: &Graphics = &graphics;
        rectangle3 = Rectangle::new(210, num11 + 110, 30, 14);
        let mut rect1_5: &Rectangle = &rectangle3
        rectangle1 = Rectangle::new(210, num11 + 124, 30, 23);
        let mut rect2_5: &Rectangle = &rectangle1
        txt2_4: String = str17;
        DrawMod.MakeFullBoxVic2( local11, rect1_5, "MOR", rect2_5, txt2_4);
        let mut Number4: i32 = self.game.HandyFunctionsObj.GetAverageEntrench(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num21 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num22 =  ((1.0 -  num21) * 2.0);
          float num23 = VBMath.Rnd() * num22 + num21;
          Number4 =  Math.Round( Conversion.Int( Number4 * num23));
          if (Number4 < 0)
            Number4 = 0;
          if (Number4 > 999)
            Number4 = 999;
        }
        str18: String = Strings.Trim(Conversion.Str( Number4));
        if (coordinate.x <= 1)
          str18 = "?";
         let mut local12: &Graphics = &graphics;
        rectangle3 = Rectangle::new(245, num11 + 110, 30, 14);
        let mut rect1_6: &Rectangle = &rectangle3
        rectangle1 = Rectangle::new(245, num11 + 124, 30, 23);
        let mut rect2_6: &Rectangle = &rectangle1
        txt2_5: String = str18;
        DrawMod.MakeFullBoxVic2( local12, rect1_6, "ENT", rect2_6, txt2_5);
        let mut num24: i32 = 40;
        str19: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].SupplyConsume)) + "%";
        if (coordinate.x <= 2)
          str19 = "?";
        if (coordinate.x <= 1)
          str19 = "?";
         let mut local13: &Graphics = &graphics;
        rectangle3 = Rectangle::new(0, num24 + 110, 65, 14);
        let mut rect1_7: &Rectangle = &rectangle3
        rectangle1 = Rectangle::new(0, num24 + 124, 65, 23);
        let mut rect2_7: &Rectangle = &rectangle1
        txt2_6: String = str19;
        DrawMod.MakeFullBoxVic2( local13, rect1_7, "SUP.CONS", rect2_7, txt2_6);
        if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.UnitObj[self.unr].Regime, self.game.Data.Turn))
        {
          str20: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].Supply));
          if (coordinate.x <= 2)
            str20 = "?";
          if (coordinate.x <= 1)
            str20 = "?";
           let mut local14: &Graphics = &graphics;
          rectangle3 = Rectangle::new(70, num24 + 110, 65, 14);
          let mut rect1_8: &Rectangle = &rectangle3
          rectangle1 = Rectangle::new(70, num24 + 124, 65, 23);
          let mut rect2_8: &Rectangle = &rectangle1
          txt2_7: String = str20;
          DrawMod.MakeFullBoxVic2( local14, rect1_8, "SUP.STOCK", rect2_8, txt2_7);
          if (self.game.Data.UnitObj[self.unr].IsHQ)
          {
            str21: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].SupplyReq));
            if (coordinate.x == 2)
              str21 = "?";
            if (coordinate.x <= 1)
              str21 = "?";
             let mut local15: &Graphics = &graphics;
            rectangle3 = Rectangle::new(140, num24 + 110, 65, 14);
            let mut rect1_9: &Rectangle = &rectangle3
            rectangle1 = Rectangle::new(140, num24 + 124, 65, 23);
            let mut rect2_9: &Rectangle = &rectangle1
            txt2_8: String = str21;
            DrawMod.MakeFullBoxVic2( local15, rect1_9, "REQ.OUT", rect2_9, txt2_8);
            str22: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].SupplyOut));
            if (coordinate.x == 2)
              str22 = "?";
            if (coordinate.x <= 1)
              str22 = "?";
             let mut local16: &Graphics = &graphics;
            rectangle3 = Rectangle::new(210, num24 + 110, 65, 14);
            let mut rect1_10: &Rectangle = &rectangle3
            rectangle1 = Rectangle::new(210, num24 + 124, 65, 23);
            let mut rect2_10: &Rectangle = &rectangle1
            txt2_9: String = str22;
            DrawMod.MakeFullBoxVic2( local16, rect1_10, "SEND.OUT", rect2_10, txt2_9);
          }
          else
          {
            str23: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].SupplyInReq));
            if (coordinate.x == 2)
              str23 = "?";
            if (coordinate.x <= 1)
              str23 = "?";
             let mut local17: &Graphics = &graphics;
            rectangle3 = Rectangle::new(140, num24 + 110, 65, 14);
            let mut rect1_11: &Rectangle = &rectangle3
            rectangle1 = Rectangle::new(140, num24 + 124, 65, 23);
            let mut rect2_11: &Rectangle = &rectangle1
            txt2_10: String = str23;
            DrawMod.MakeFullBoxVic2( local17, rect1_11, "REQ.IN", rect2_11, txt2_10);
            str24: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].SupplyIn));
            if (coordinate.x == 2)
              str24 = "?";
            if (coordinate.x <= 1)
              str24 = "?";
             let mut local18: &Graphics = &graphics;
            rectangle3 = Rectangle::new(210, num24 + 110, 65, 14);
            let mut rect1_12: &Rectangle = &rectangle3
            rectangle1 = Rectangle::new(210, num24 + 124, 65, 23);
            let mut rect2_12: &Rectangle = &rectangle1
            txt2_11: String = str24;
            DrawMod.MakeFullBoxVic2( local18, rect1_12, "SUP.IN", rect2_12, txt2_11);
          }
        }
      }
      if (!Information.IsNothing( graphics))
      {
        graphics.Dispose();
        graphics = (Graphics) null;
      }
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
       let mut local1: &Graphics = &Expression;
      bitmap: Bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(self.OwnBitmapNr));
       let mut local2: &Bitmap = &bitmap;
      DrawMod.Draw( local1,  local2, 0, 0, 0.3f, 0.3f, 0.3f, 1f);
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }
  }
}
