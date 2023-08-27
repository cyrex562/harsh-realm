// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UnitHeaderPartClass_Backuo
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class UnitHeaderPartClass_Backuo : SubPartClass
  {
     object OwnBitmapNr;
     unr: i32;
     game: GameClass;
     his: i32;

    pub UnitHeaderPartClass_Backuo(tunr: i32, tgame: GameClass)
      : base(225, 200)
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
      if (x > 0 & y > 42 & x < 33 & y < 100)
        self.Descript = "Action Points";
      if (x > 33 & y > 42 & x < 71 & y < 100)
        self.Descript = "Supply % consumed at start of this turn.";
      if (x > 71 & y > 42 & x < 109 & y < 100)
        self.Descript = "Readiness.";
      if (x > 109 & y > 42 & x < 147 & y < 100)
        self.Descript = "Experience.";
      if (x > 147 & y > 42 & x < 185 & y < 100)
        self.Descript = "Morale.";
      if (x > 185 & y > 42 & x < 223 & y < 100)
        self.Descript = "Entrenchment.";
      if (x > 0 & y > 170 & x < 100 & y < 190)
        self.Descript = "Weight of whole unit.";
      if (x > 0 & y > 110 & x < 100 & y < 130)
        self.Descript = "Stack Points of whole unit.";
      if (x > 0 & y > 130 & x < 100 & y < 150)
        self.Descript = "Land Transport Capacity. (for transfers)";
      if (x > 0 & y > 150 & x < 100 & y < 170)
        self.Descript = "Navy Transport Capacity. (for naval transfers)";
      if (x > 125 & y > 110 & x < 225 & y < 130)
      {
        let mut hq: i32 = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HQ;
        if (self.game.Data.Round != 0 & self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Regime != self.game.Data.Turn & self.game.Data.FOWOn)
        {
          self.Descript = "Staff points in HQ expressed as a % of needed staff points. ";
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
      if (x > 125 & y > 130 & x < 225 & y < 150)
        self.Descript = "Headquarter Power over unit (determines staff influence).";
      if (x > 125 & y > 150 & x < 225 & y < 170)
        self.Descript = "Carry Points. L=Land, N=Navy, A=Air. ";
      if (x > 125 & y > 170 & x < 225 & y < 190)
      {
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
        {
          if ( self.game.Data.RuleVar[509] == 0.0)
            self.Descript = "Rail capacity availabe";
          else
            self.Descript = "";
        }
        else
          self.Descript = "Aircraftcarrier Points / And amount of those used.";
      }
      if (x > 45 & y > 0 & x < 200 & y < 17)
        self.Descript = "Click to change name of unit.";
      if (x > 45 & y > 17 & x < 200 & y < 30)
      {
        if (self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn)
          self.Descript = "Click to jump to units HQ.";
        else
          self.Descript = "";
      }
      if (x > 190 & y < 41)
        self.Descript = "For all practical purposes this unit travels with the movement speed this movement type";
      if (!(x > 5 & y > 1 & x < 43 & y < 38) || !self.game.Data.UnitObj[self.game.EditObj.UnitSelected].IsHQ)
        return;
      self.Descript = "Click to change of: Color hq.";
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Coordinate coordinate = Coordinate::new();
      if (self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn | self.game.Data.Round == 0)
        coordinate.x = 3;
      else
        coordinate = self.game.HandyFunctionsObj.GetReconMinusHide(self.unr, self.game.Data.Turn);
      Graphics toG = Graphics.FromImage((Image) self.OwnBitmap);
      let mut integer1: i32 = Conversions.ToInteger(self.game.HandyFunctionsObj.GetUnitPeople(self.unr));
      let mut regime: i32 = self.game.Data.UnitObj[self.unr].Regime;
      let mut index1: i32 = regime;
      if (self.game.Data.PeopleObj[integer1].RegCol > -1)
        index1 = self.game.Data.PeopleObj[integer1].RegCol;
      let mut red1: i32 = self.game.Data.RegimeObj[index1].Red;
      let mut green1: i32 = self.game.Data.RegimeObj[index1].Green;
      let mut blue1: i32 = self.game.Data.RegimeObj[index1].Blue;
      let mut red2: i32 = self.game.Data.RegimeObj[index1].Red2;
      let mut green2: i32 = self.game.Data.RegimeObj[index1].Green2;
      let mut blue2: i32 = self.game.Data.RegimeObj[index1].Blue2;
      let mut red3: i32 = red1 - 50;
      let mut green3: i32 = green1 - 50;
      let mut blue3: i32 = blue1 - 50;
      let mut red4: i32 = red1 + 40;
      let mut green4: i32 = green1 + 40;
      let mut blue4: i32 = blue1 + 40;
      if (red4 >  byte.MaxValue)
        red4 =  byte.MaxValue;
      if (green4 >  byte.MaxValue)
        green4 =  byte.MaxValue;
      if (blue4 >  byte.MaxValue)
        blue4 =  byte.MaxValue;
      if (0 > red3)
        red3 = 0;
      if (0 > green3)
        green3 = 0;
      if (0 > blue3)
        blue3 = 0;
      Color.FromArgb( byte.MaxValue, red4, green4, blue4);
      Color.FromArgb( byte.MaxValue, red3, green3, blue3);
      Color.FromArgb( byte.MaxValue, red2, green2, blue2);
      self.game.CustomBitmapObj.DrawUnit(self.unr, toG: toG, tx: 5, ShowAttacker: true);
      str1: String = self.game.Data.UnitObj[self.unr].Name;
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
      SizeF sizeF2 = toG.MeasureString(str1, self.game.GameFont3);
      if ( sizeF2.Width < 150.0)
      {
        DrawMod.DrawText( toG, str1, self.game.GameFont3, 45, 0);
      }
      else
      {
        sizeF2 = toG.MeasureString(str1, self.game.GameFont1);
        if ( sizeF2.Width < 150.0)
          DrawMod.DrawText( toG, str1, self.game.GameFont1, 45, 0);
        else
          DrawMod.DrawText( toG, str1, self.game.GameFont2, 45, 0);
      }
      if (self.game.Data.UnitObj[self.unr].HQ > -1)
      {
        str3: String = self.game.Data.UnitObj[self.game.Data.UnitObj[self.unr].HQ].Name;
        if (coordinate.x < 2)
          str3 = "?";
        DrawMod.DrawTextColoured( toG, "HQ: " + str3, Font::new(self.game.FontCol.Families[1], 11f, FontStyle.Underline, GraphicsUnit.Pixel), 45, 17, Color.White);
      }
      else
      {
        tstring: String = "(has no HQ)";
        if (coordinate.x < 2)
          tstring = "HQ: ?";
        DrawMod.DrawTextColoured( toG, tstring, Font::new(self.game.FontCol.Families[1], 11f, FontStyle.Italic, GraphicsUnit.Pixel), 45, 17, Color.White);
      }
      let mut num1: i32 = 0;
      if (!self.game.Data.FOWOn)
        num1 = 1;
      if (self.unr > -1)
      {
        if (self.game.Data.Turn == self.game.Data.UnitObj[self.unr].Regime)
          num1 = 1;
        if (self.game.HandyFunctionsObj.IsAlliedOrSelf(self.game.Data.Turn, self.game.Data.UnitObj[self.unr].Regime))
          num1 = 1;
      }
      if (num1 == 1 &  self.game.Data.RuleVar[354] == 1.0 && self.game.HandyFunctionsObj.GetStartPower(self.unr) > 0 & !self.game.Data.UnitObj[self.unr].IsHQ)
      {
        DrawMod.DrawBlock( toG, 40, 39, 184, 10,  byte.MaxValue,  byte.MaxValue, 0,  byte.MaxValue);
        let mut breakPercent: i32 = self.game.HandyFunctionsObj.GetBreakPercent(self.unr);
        let mut powerPtsAbsolute: i32 = self.game.HandyFunctionsObj.GetPowerPtsAbsolute(self.unr, true);
        let mut num2: i32 =  Math.Round( self.game.Data.RuleVar[307]);
        let mut startPower: i32 = self.game.HandyFunctionsObj.GetStartPower(self.unr);
        let mut num3: i32 =  Math.Round( startPower * ( breakPercent / 100.0));
        float num4 = startPower < powerPtsAbsolute ? 184f /  powerPtsAbsolute : 184f /  startPower;
        let mut w1: i32 =  Math.Round( ( num3 * num4));
        let mut num5: i32 =  Math.Round( ( startPower * num4));
        let mut num6: i32 =  Math.Round( ( powerPtsAbsolute * num4));
        let mut w2: i32 =  Math.Round( ( num2 * num4));
        if (num5 >= num6)
        {
          DrawMod.DrawBlockGradient( toG, 40 + w1 - 5, 39, num5 - w1 + 5, 10, Color.Yellow, Color.Green);
          DrawMod.DrawBlockGradient( toG, 40, 39, w1, 10, Color.Red, Color.Yellow);
          DrawMod.DrawBlockGradient( toG, 40, 39, w2, 10, Color.Red, Color.Red);
          DrawMod.DrawBlock( toG, 225 - (184 - num6), 39, 184 - num6, 10, 0, 0, 0,  byte.MaxValue);
        }
        else
        {
          DrawMod.DrawBlockGradient( toG, 40, 39, 184, 10, Color.Yellow, Color.Green);
          DrawMod.DrawBlockGradient( toG, 40, 39, w1, 10, Color.Red, Color.Yellow);
          DrawMod.DrawBlockGradient( toG, 40, 39, w2, 10, Color.Red, Color.Red);
        }
        DrawMod.drawLine( toG, 40 + num5, 39, 40 + num5, 49,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.drawLine( toG, 40 + w1, 39, 40 + w1, 49,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.drawLine( toG, 40 + w2, 39, 40 + w2, 49,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.DrawBlockGradient( toG, 1, 39, 40, 10, self.game.GameCol5, self.game.GameCol6);
        DrawMod.DrawRectangle( toG, 0, 39, 224, 10,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.drawLine( toG, 40, 39, 40, 49,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.DrawText( toG, "Intgrty", self.game.GameFont2, 3, 37);
      }
      let mut num7: i32 = 0;
      if (self.game.Data.Turn == self.game.Data.UnitObj[self.unr].Regime | !self.game.Data.FOWOn | self.game.Data.Round == 0 | coordinate.x >= 2)
      {
        num7 = 1;
        let mut nr: i32 = -1;
        let mut index2: i32 = self.game.HandyFunctionsObj.GetLowestSpeed(self.unr, -1, true);
        if (index2 > -1)
        {
          let mut sfCount: i32 = self.game.Data.UnitObj[self.unr].SFCount;
          for (nr = 0; nr <= sfCount; nr += 1)
          {
            if (self.game.Data.SFTypeObj[self.game.Data.SFObj[self.game.Data.UnitObj[self.unr].SFList[nr]].Type].SymbolGroup == self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].SymbolGroup && self.game.Data.SFTypeObj[self.game.Data.SFObj[self.game.Data.UnitObj[self.unr].SFList[nr]].Type].SymbolWeight > self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].SymbolWeight)
              index2 = self.game.Data.UnitObj[self.unr].SFList[nr];
          }
        }
        str4: String = index2 <= -1 ? "Immobile" : (self.game.Data.SFObj[index2].MoveType != -1 ? Strings.Trim(self.game.Data.TempString[self.game.Data.SFObj[index2].MoveType]) : Strings.Trim(self.game.Data.TempString[self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].MoveType]));
        if (index2 == -2)
          nr = self.game.SUPPLIESSYMBOL;
        else if (index2 > -1)
          nr = self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].SymbolSpriteID;
        let mut integer2: i32 = Conversions.ToInteger(self.game.HandyFunctionsObj.GetUnitPeople(self.unr));
        if (regime > -1 & index2 > -1)
        {
          if (self.game.Data.RegimeObj[regime].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].ExtraCounter;
            for (let mut index3: i32 = 0; index3 <= extraCounter; index3 += 1)
            {
              if (self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].ExtraCode[index3] == self.game.Data.RegimeObj[regime].ExtraGraphicUse)
                nr = self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].ExtraSymbolSpriteID[index3];
            }
          }
          else if (self.game.Data.PeopleObj[integer2].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].ExtraCounter;
            for (let mut index4: i32 = 0; index4 <= extraCounter; index4 += 1)
            {
              if (self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].ExtraCode[index4] == self.game.Data.PeopleObj[integer2].ExtraGraphicUse)
                nr = self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].ExtraSymbolSpriteID[index4];
            }
          }
        }
        if (nr > -1 & index2 > -1)
        {
           let mut local1: &Graphics = &toG;
          bitmap: Bitmap = BitmapStore.GetBitmap(nr);
           let mut local2: &Bitmap = &bitmap;
          DrawMod.DrawSimple( local1,  local2, 188, 13);
          str5: String = self.game.Data.SFObj[index2].MoveType <= -1 ? self.game.Data.TempString[self.game.Data.SFTypeObj[self.game.Data.SFObj[index2].Type].MoveType] : self.game.Data.TempString[self.game.Data.SFObj[index2].MoveType];
          if (Strings.Len(str5) > 7)
            str5 = Strings.Left(str5, 7);
          sizeF2 = toG.MeasureString(str5, Font::new(self.game.FontCol.Families[1], 10f, FontStyle.Regular, GraphicsUnit.Pixel));
          DrawMod.DrawText( toG, str5, Font::new(self.game.FontCol.Families[1], 10f, FontStyle.Regular, GraphicsUnit.Pixel),  Math.Round(209.0 -  sizeF2.Width / 2.0), 1);
        }
        else if (nr > -1)
        {
           let mut local3: &Graphics = &toG;
          bitmap: Bitmap = BitmapStore.GetBitmap(nr);
           let mut local4: &Bitmap = &bitmap;
          DrawMod.DrawSimple( local3,  local4, 188, 13);
          if (Strings.Len(str4) > 7)
            str4 = Strings.Left(str4, 7);
          sizeF2 = toG.MeasureString(str4, Font::new(self.game.FontCol.Families[1], 10f, FontStyle.Regular, GraphicsUnit.Pixel));
          DrawMod.DrawText( toG, str4, Font::new(self.game.FontCol.Families[1], 10f, FontStyle.Regular, GraphicsUnit.Pixel),  Math.Round(209.0 -  sizeF2.Width / 2.0), 1);
        }
        str6: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetLowestAp(self.unr)));
        if (coordinate.x == 2)
          str6 = "?";
        sizeF2 = toG.MeasureString(str6, self.game.GameFont3);
        DrawMod.DrawText( toG, str6, self.game.GameFont3,  Math.Round(16.0 -  sizeF2.Width / 2.0), 84);
        str7: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].SupplyConsume));
        if (coordinate.x == 2)
          str7 = "?";
        sizeF2 = toG.MeasureString(str7, self.game.GameFont3);
        DrawMod.DrawText( toG, str7, self.game.GameFont3,  Math.Round(57.0 -  sizeF2.Width / 2.0), 84);
        let mut Number1: i32 = self.game.HandyFunctionsObj.GetAverageRdn(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num8 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num9 =  ((1.0 -  num8) * 2.0);
          float num10 = VBMath.Rnd() * num9 + num8;
          Number1 =  Math.Round( Conversion.Int( Number1 * num10));
          if (Number1 < 0)
            Number1 = 0;
          if (Number1 > 100)
            Number1 = 100;
        }
        str8: String = Strings.Trim(Conversion.Str( Number1));
        sizeF2 = toG.MeasureString(str8, self.game.GameFont3);
        DrawMod.DrawText( toG, str8, self.game.GameFont3,  Math.Round(95.0 -  sizeF2.Width / 2.0), 84);
        let mut Number2: i32 = self.game.HandyFunctionsObj.GetAverageXp(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num11 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num12 =  ((1.0 -  num11) * 2.0);
          float num13 = VBMath.Rnd() * num12 + num11;
          Number2 =  Math.Round( Conversion.Int( Number2 * num13));
          if (Number2 < 0)
            Number2 = 0;
          if (Number2 > 100)
            Number2 = 100;
        }
        str9: String = Strings.Trim(Conversion.Str( Number2));
        sizeF2 = toG.MeasureString(str9, self.game.GameFont3);
        DrawMod.DrawText( toG, str9, self.game.GameFont3,  Math.Round(133.0 -  sizeF2.Width / 2.0), 84);
        let mut Number3: i32 = self.game.HandyFunctionsObj.GetAverageMor(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num14 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num15 =  ((1.0 -  num14) * 2.0);
          float num16 = VBMath.Rnd() * num15 + num14;
          Number3 =  Math.Round( Conversion.Int( Number3 * num16));
          if (Number3 < 0)
            Number3 = 0;
          if (Number3 > 999)
            Number3 = 999;
        }
        str10: String = Strings.Trim(Conversion.Str( Number3));
        sizeF2 = toG.MeasureString(str10, self.game.GameFont3);
        DrawMod.DrawText( toG, str10, self.game.GameFont3,  Math.Round(171.0 -  sizeF2.Width / 2.0), 84);
        let mut Number4: i32 = self.game.HandyFunctionsObj.GetAverageEntrench(self.unr);
        if (coordinate.x == 2)
        {
          self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, 0);
          float num17 =  coordinate.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
          float num18 =  ((1.0 -  num17) * 2.0);
          float num19 = VBMath.Rnd() * num18 + num17;
          Number4 =  Math.Round( Conversion.Int( Number4 * num19));
          if (Number4 < 0)
            Number4 = 0;
          if (Number4 > 999)
            Number4 = 999;
        }
        str11: String = Strings.Trim(Conversion.Str( Number4));
        sizeF2 = toG.MeasureString(str11, self.game.GameFont3);
        DrawMod.DrawText( toG, str11, self.game.GameFont3,  Math.Round(209.0 -  sizeF2.Width / 2.0), 84);
      }
      let mut num20: i32 = 3;
      do
      {
        DrawMod.DrawBlockGradient( toG, 0, 50 + num20 * 20, 40, 20, self.game.GameCol5, self.game.GameCol6);
        DrawMod.DrawBlock( toG, 40, 50 + num20 * 20, 60, 20,  self.game.GameCol7.R,  self.game.GameCol7.G,  self.game.GameCol7.B,  byte.MaxValue);
        DrawMod.DrawRectangle( toG, 0, 50 + num20 * 20, 100, 20,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.drawLine( toG, 40, 50 + num20 * 20, 40, 70 + num20 * 20,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.DrawRectangle( toG, 1, 51 + num20 * 20, 38, 18, 0, 0, 0,  byte.MaxValue);
        num20 += 1;
      }
      while (num20 <= 6);
      let mut num21: i32 = 3;
      do
      {
        DrawMod.DrawBlockGradient( toG, 124, 50 + num21 * 20, 40, 20, self.game.GameCol5, self.game.GameCol6);
        DrawMod.DrawBlock( toG, 164, 50 + num21 * 20, 60, 20,  self.game.GameCol7.R,  self.game.GameCol7.G,  self.game.GameCol7.B,  byte.MaxValue);
        DrawMod.DrawRectangle( toG, 124, 50 + num21 * 20, 100, 20,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.drawLine( toG, 164, 50 + num21 * 20, 164, 70 + num21 * 20,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
        DrawMod.DrawRectangle( toG, 125, 51 + num21 * 20, 38, 18, 0, 0, 0,  byte.MaxValue);
        num21 += 1;
      }
      while (num21 <= 6);
      DrawMod.DrawText( toG, "Stk", self.game.GameFont2, 3, 115);
      if (self.game.Data.UnitObj[self.unr].IsHQ)
        DrawMod.DrawText( toG, "Lnd", self.game.GameFont2, 3, 135);
      else
        DrawMod.DrawText( toG, "Units", self.game.GameFont2, 3, 135);
      if (self.game.Data.UnitObj[self.unr].IsHQ)
        DrawMod.DrawText( toG, "Nvy", self.game.GameFont2, 3, 155);
      else
        DrawMod.DrawText( toG, "Owner", self.game.GameFont2, 3, 155);
      DrawMod.DrawText( toG, "Wgt", self.game.GameFont2, 3, 175);
      DrawMod.DrawText( toG, "Staf", self.game.GameFont2, 128, 115);
      DrawMod.DrawText( toG, "HqPw", self.game.GameFont2, 128, 135);
      DrawMod.DrawText( toG, "Car", self.game.GameFont2, 128, 155);
      tstring1: String = Strings.Trim(Conversion.Str(Operators.AddObject(self.game.HandyFunctionsObj.GetUnitNonSeaWeight(self.unr, true), self.game.HandyFunctionsObj.GetUnitExcessWeight(self.unr))));
      if (coordinate.x <= 2)
        tstring1 = "?";
      DrawMod.DrawText( toG, tstring1, self.game.gamefont1b, 41, 175);
      tstring2: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetUnitStackPts(self.unr)));
      if (coordinate.x <= 2)
        tstring2 = "?";
      DrawMod.DrawText( toG, tstring2, self.game.gamefont1b, 41, 115);
      tstring3: String;
      if (self.game.HandyFunctionsObj.HasUnitNavySF(self.unr))
      {
        let mut unitCarryCap: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unr, 1);
        let mut Number: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unr, 1) - self.game.HandyFunctionsObj.GetUnitCarryCap(self.unr, 1, true);
        tstring3 = "N:" + Strings.Trim(Conversion.Str( unitCarryCap)) + "/" + Strings.Trim(Conversion.Str( Number));
      }
      else if (self.game.HandyFunctionsObj.HasUnitAirSF(self.unr))
        tstring3 = "A:" + Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetUnitCarryCap(self.unr, 2)));
      else if (self.game.HandyFunctionsObj.HasUnitlandSF(self.unr))
      {
        let mut unitCarryCap: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unr, 0);
        let mut Number: i32 = self.game.HandyFunctionsObj.GetUnitCarryCap(self.unr, 0) - self.game.HandyFunctionsObj.GetUnitCarryCap(self.unr, 0, true);
        tstring3 = "L:" + Strings.Trim(Conversion.Str( unitCarryCap)) + "/" + Strings.Trim(Conversion.Str( Number));
      }
      else
        tstring3 = "N/A";
      if (coordinate.x <= 2)
        tstring3 = "?";
      DrawMod.DrawText( toG, tstring3, self.game.gamefont2b, 166, 155);
      if (!self.game.Data.UnitObj[self.unr].IsHQ)
      {
        DrawMod.DrawText( toG, "Acar", self.game.GameFont2, 128, 175);
        tstring4: String;
        if (self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn)
        {
          let mut airCarryCapPts: i32 = self.game.HandyFunctionsObj.GetAirCarryCapPts(self.unr);
          let mut integer3: i32 = Conversions.ToInteger(Operators.SubtractObject(self.game.HandyFunctionsObj.GetUnitNonSeaWeight(self.unr, true), self.game.HandyFunctionsObj.GetUnitNonSeaWeight(self.unr, false)));
          tstring4 = Strings.Trim(Conversion.Str( airCarryCapPts)) + "/" + Strings.Trim(Conversion.Str( integer3));
          if (coordinate.x <= 2)
            tstring4 = "?";
        }
        else
          tstring4 = "?";
        DrawMod.DrawText( toG, tstring4, self.game.gamefont1b, 166, 175);
      }
      if ((self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn | !self.game.Data.FOWOn) & self.game.Data.UnitObj[self.unr].IsHQ)
      {
        if ( self.game.Data.RuleVar[509] == 0.0)
        {
          DrawMod.DrawText( toG, "Rail", self.game.GameFont2, 128, 175);
          tstring5: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].AirCap));
          if (coordinate.x <= 2)
            tstring5 = "?";
          DrawMod.DrawText( toG, tstring5, self.game.gamefont1b, 166, 175);
        }
        tstring6: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].NavyCap));
        if (coordinate.x <= 2)
          tstring6 = "?";
        DrawMod.DrawText( toG, tstring6, self.game.gamefont1b, 44, 155);
        tstring7: String = Strings.Trim(Conversion.Str( self.game.Data.UnitObj[self.unr].LandCap));
        if (coordinate.x <= 2)
          tstring7 = "?";
        DrawMod.DrawText( toG, tstring7, self.game.gamefont1b, 44, 135);
        tstring8: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetStaffPercent(self.unr))) + "%";
        if (coordinate.x <= 2)
          tstring8 = "?";
        DrawMod.DrawText( toG, tstring8, self.game.gamefont1b, 166, 115);
        tstring9: String = "100%";
        if (coordinate.x <= 2)
          tstring9 = "?";
        DrawMod.DrawText( toG, tstring9, self.game.gamefont1b, 166, 135);
      }
      else
      {
        tstring10: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.Gethqpow(self.unr))) + "%";
        if (coordinate.x <= 2)
          tstring10 = "?";
        DrawMod.DrawText( toG, tstring10, self.game.gamefont1b, 166, 135);
        if (!self.game.Data.UnitObj[self.unr].IsHQ)
        {
          tstring11: String = self.game.Data.UnitObj[self.unr].HQ <= -1 ? "N/A" : Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetStaffPercent(self.game.Data.UnitObj[self.unr].HQ))) + "%";
          if (coordinate.x <= 2)
            tstring11 = "?";
          DrawMod.DrawText( toG, tstring11, self.game.gamefont1b, 166, 115);
        }
        else
        {
          tstring12: String = Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetStaffPoints(self.unr))) + "/" + Strings.Trim(Conversion.Str( self.game.HandyFunctionsObj.GetStaffNeeded(self.unr)));
          if (coordinate.x <= 2)
            tstring12 = "?";
          DrawMod.DrawText( toG, tstring12, self.game.gamefont2b, 166, 115);
        }
        if (self.game.Data.UnitObj[self.unr].Historical > -1)
        {
          tstring13: String = Strings.Trim(Conversion.Str( self.game.Data.HistoricalUnitObj[self.game.Data.UnitObj[self.unr].Historical].StartSize));
          if (coordinate.x <= 2)
            tstring13 = "?";
          DrawMod.DrawText( toG, tstring13, self.game.gamefont2b, 44, 135);
        }
        else
          DrawMod.DrawText( toG, "N/A", self.game.gamefont2b, 44, 135);
        tstring14: String = Strings.Left(self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].Name, 10);
        DrawMod.DrawText( toG, tstring14, self.game.gamefont2b, 44, 155);
      }
      if (((num7 == 0 ? 1 : 0) & 0) != 0)
      {
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 41, 115);
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 41, 135);
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 41, 155);
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 41, 175);
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 166, 115);
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 166, 135);
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 166, 155);
        DrawMod.DrawText( toG, "?", self.game.gamefont1b, 166, 175);
      }
      if (num7 == 0)
      {
        DrawMod.DrawText( toG, "?", self.game.GameFont3, 15, 83);
        DrawMod.DrawText( toG, "?", self.game.GameFont3, 53, 83);
        DrawMod.DrawText( toG, "?", self.game.GameFont3, 91, 83);
        DrawMod.DrawText( toG, "?", self.game.GameFont3, 129, 83);
        DrawMod.DrawText( toG, "?", self.game.GameFont3, 167, 83);
        DrawMod.DrawText( toG, "?", self.game.GameFont3, 205, 83);
      }
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
       let mut local1: &Graphics = &graphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(self.OwnBitmapNr));
       let mut local2: &Bitmap = &bitmap;
      DrawMod.Draw( local1,  local2, 0, 0, 0.3f, 0.3f, 0.3f, 1f);
      return self.OwnBitmap;
    }
  }
}
