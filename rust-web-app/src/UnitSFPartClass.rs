// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UnitSFPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class UnitSFPartClass : SubPartClass
  {
     object OwnBitmapNr;
     unr: i32;
     game: GameClass;

    pub UnitSFPartClass(tunr: i32, tgame: GameClass)
      : base(620, 200)
    {
      self.unr = tunr;
      self.game = tgame;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF = SizeF::new();
      Coordinate coordinate1 = Coordinate::new();
      Coordinate coordinate2 = Coordinate::new();
      bool flag1;
      if (self.game.EditObj.OrderType == 14)
        flag1 = true;
      if (self.game.EditObj.OrderType == 15)
        flag1 = true;
      if (self.game.EditObj.OrderType == 2)
        flag1 = true;
      if (self.game.EditObj.OrderType == 12)
        flag1 = true;
      bool flag2;
      if (self.game.EditObj.OrderType == 11)
      {
        flag1 = true;
        flag2 = true;
      }
      if (self.game.EditObj.OrderType == 13)
      {
        flag1 = true;
        flag2 = true;
      }
      if (self.game.Data.UnitObj[self.unr].Regime == self.game.Data.Turn | self.game.Data.Round == 0)
        coordinate1.x = 3;
      else
        coordinate1 = self.game.HandyFunctionsObj.GetReconMinusHide(self.unr, self.game.Data.Turn);
      if (self.unr < 0 | self.unr > self.game.Data.UnitCounter)
        return self.OwnBitmap;
      if (self.game.Data.UnitObj[self.unr].SFCount < 0)
        return self.OwnBitmap;
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      let mut regime: i32 = self.game.Data.UnitObj[self.unr].Regime;
      let mut red1: i32 = self.game.Data.RegimeObj[regime].Red;
      let mut green1: i32 = self.game.Data.RegimeObj[regime].Green;
      let mut blue1: i32 = self.game.Data.RegimeObj[regime].Blue;
      let mut red2: i32 = red1 + 50;
      let mut green2: i32 = green1 + 50;
      let mut blue2: i32 = blue1 + 50;
      if ( byte.MaxValue < red2)
        red2 =  byte.MaxValue;
      if ( byte.MaxValue < green2)
        green2 =  byte.MaxValue;
      if ( byte.MaxValue < blue2)
        blue2 =  byte.MaxValue;
      let mut red2_1: i32 = self.game.Data.RegimeObj[regime].Red2;
      let mut green2_1: i32 = self.game.Data.RegimeObj[regime].Green2;
      let mut blue2_1: i32 = self.game.Data.RegimeObj[regime].Blue2;
      c1_1: Color = Color.FromArgb( byte.MaxValue, red1, green1, blue1);
      c2_1: Color = Color.FromArgb(0, red1, green1, blue1);
      c1_2: Color = Color.FromArgb( byte.MaxValue, red2, green2, blue2);
      c2_2: Color = Color.FromArgb(0, red2, green2, blue2);
      color: Color = Color.FromArgb( byte.MaxValue, red2_1, green2_1, blue2_1);
      Color.FromArgb(185, 0, 0, 0);
      Color.FromArgb(200, red2_1, green2_1, blue2_1);
      let mut num1: i32 = -1;
      if (coordinate1.x > 1)
      {
        let mut sfCount: i32 = self.game.Data.UnitObj[self.unr].SFCount;
        for (let mut i: i32 = 0; i <= sfCount; i += 1)
        {
          num1 += 1;
          let mut sf: i32 = self.game.Data.UnitObj[self.unr].SFList[i];
          let mut type: i32 = self.game.Data.SFObj[sf].Type;
          let mut people: i32 = self.game.Data.SFObj[sf].People;
          let mut picSpriteId: i32 = self.game.Data.SFTypeObj[type].PicSpriteID;
          let mut sidewaysSpriteId: i32 = self.game.Data.SFTypeObj[type].SidewaysSpriteID;
          let mut baseColor: i32 = self.game.Data.SFTypeObj[type].BaseColor;
          if (self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[type].ExtraCounter;
            for (let mut index: i32 = 0; index <= extraCounter; index += 1)
            {
              if (self.game.Data.SFTypeObj[type].ExtraCode[index] == self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].ExtraGraphicUse)
              {
                picSpriteId = self.game.Data.SFTypeObj[type].ExtraPicSpriteID[index];
                sidewaysSpriteId = self.game.Data.SFTypeObj[type].ExtraSidewaysSpriteID[index];
              }
            }
          }
          else if (self.game.Data.PeopleObj[people].ExtraGraphicUse > -1)
          {
            let mut extraCounter: i32 = self.game.Data.SFTypeObj[type].ExtraCounter;
            for (let mut index: i32 = 0; index <= extraCounter; index += 1)
            {
              if (self.game.Data.SFTypeObj[type].ExtraCode[index] == self.game.Data.PeopleObj[people].ExtraGraphicUse)
              {
                picSpriteId = self.game.Data.SFTypeObj[type].ExtraPicSpriteID[index];
                sidewaysSpriteId = self.game.Data.SFTypeObj[type].ExtraSidewaysSpriteID[index];
              }
            }
          }
          if (self.game.Data.UnitObj[self.unr].SFCount < 8)
          {
            let mut num2: i32 =  Math.Round(3.0 + Conversion.Int( i / 4.0) * -620.0 +  (i * 155));
            let mut num3: i32 =  Math.Round(5.0 + Conversion.Int( i / 4.0) * 98.0);
            let mut index1: i32 = regime;
            if (self.game.Data.PeopleObj[people].RegCol > -1)
              index1 = self.game.Data.PeopleObj[people].RegCol;
            let mut red3: i32 = self.game.Data.RegimeObj[index1].Red;
            let mut green3: i32 = self.game.Data.RegimeObj[index1].Green;
            let mut blue3: i32 = self.game.Data.RegimeObj[index1].Blue;
            let mut red2_2: i32 = self.game.Data.RegimeObj[index1].Red2;
            let mut green2_2: i32 = self.game.Data.RegimeObj[index1].Green2;
            let mut blue2_2: i32 = self.game.Data.RegimeObj[index1].Blue2;
            let mut red4: i32 = red3 - 50;
            let mut green4: i32 = green3 - 50;
            let mut blue4: i32 = blue3 - 50;
            if (0 > red4)
              red4 = 0;
            if (0 > green4)
              green4 = 0;
            if (0 > blue4)
              blue4 = 0;
            c1_1 = Color.FromArgb( byte.MaxValue, red3, green3, blue3);
            c2_1 = Color.FromArgb( byte.MaxValue, red4, green4, blue4);
            color = Color.FromArgb( byte.MaxValue, red2_2, green2_2, blue2_2);
            DrawMod.DrawBlock( graphics, num2, num3 + 2, 150, 15,  self.game.VicColor3Shade.R,  self.game.VicColor3Shade.G,  self.game.VicColor3Shade.B,  self.game.VicColor3Shade.A);
            let mut Number1: i32 = self.game.Data.SFObj[sf].Qty;
            if (coordinate1.x < 3 && self.game.Data.FOWOn & self.game.Data.UnitObj[self.unr].Regime != self.game.Data.Turn)
            {
              self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, i);
              if (coordinate1.x == 2)
              {
                self.game.HandyFunctionsObj.RandomizeForUnit(self.unr, i);
                float num4 =  coordinate1.y / (self.game.Data.RuleVar[56] - self.game.Data.RuleVar[55]);
                float num5 =  ((1.0 -  num4) * 2.0);
                Number1 =  Math.Round( Conversion.Int((VBMath.Rnd() * num5 + num4) *  Number1));
                if (Number1 < 1)
                  Number1 = 1;
                VBMath.Randomize();
              }
            }
            if (self.game.Data.SFTypeObj[type].Ratio > 0)
              Number1 *= self.game.Data.SFTypeObj[type].Ratio;
            tstring1: String = Strings.Trim(Conversion.Str( Number1)) + "x " + self.game.Data.SFTypeObj[type].Name;
            if (self.game.Data.SFTypeObj[type].ModelID <= 0)
            {
              if (self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].ExtraGraphicUse > -1)
              {
                let mut extraCounter: i32 = self.game.Data.SFTypeObj[type].ExtraCounter;
                for (let mut index2: i32 = 0; index2 <= extraCounter; index2 += 1)
                {
                  if (self.game.Data.SFTypeObj[type].ExtraCode[index2] == self.game.Data.RegimeObj[self.game.Data.UnitObj[self.unr].Regime].ExtraGraphicUse)
                    tstring1 = Strings.Trim(Conversion.Str( Number1)) + "x " + self.game.Data.SFTypeObj[type].ExtraName[index2];
                }
              }
              else if (self.game.Data.PeopleObj[self.game.Data.SFObj[sf].People].ExtraGraphicUse > -1)
              {
                let mut extraCounter: i32 = self.game.Data.SFTypeObj[type].ExtraCounter;
                for (let mut index3: i32 = 0; index3 <= extraCounter; index3 += 1)
                {
                  if (self.game.Data.SFTypeObj[type].ExtraCode[index3] == self.game.Data.PeopleObj[self.game.Data.SFObj[sf].People].ExtraGraphicUse)
                    tstring1 = Strings.Trim(Conversion.Str( Number1)) + "x " + self.game.Data.SFTypeObj[type].ExtraName[index3];
                }
              }
            }
            DrawMod.DrawTextVic2( graphics, tstring1, self.game.VicFont2, num2, num3 + 2, self.game.VicColor2, self.game.VicColor2Shade);
            bitmap: Bitmap;
            Rectangle rectangle1;
            Rectangle rectangle2;
            if ( self.game.Data.RuleVar[869] >= 1.0)
            {
              if (self.game.Data.UnitObj[self.unr].X > -1)
              {
                let mut index4: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y].LandscapeType;
                let mut index5: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y].SpriteNr;
                if (self.game.Data.LandscapeTypeObj[index4].AirOverride > -1 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 2)
                {
                  index4 = self.game.Data.LandscapeTypeObj[index4].AirOverride;
                  index5 = 0;
                }
                else if ( self.game.Data.RuleVar[848] > 0.0 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 2)
                {
                  index4 =  Math.Round( self.game.Data.RuleVar[848]);
                  index5 = 0;
                }
                if (self.game.Data.LandscapeTypeObj[index4].NavyOverride > -1 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 1)
                {
                  index4 = self.game.Data.LandscapeTypeObj[index4].NavyOverride;
                  index5 = 0;
                }
                else if ( self.game.Data.RuleVar[872] > 0.0 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 1)
                {
                  index4 =  Math.Round( self.game.Data.RuleVar[872]);
                  index5 = 0;
                }
                if ( self.game.Data.RuleVar[869] == 3.0)
                {
                  let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].BasicPicID[index5];
                   let mut local1: &Graphics = &graphics;
                  bitmap = BitmapStore.GetBitmap(nr);
                   let mut local2: &Bitmap = &bitmap;
                  rectangle1 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
                  let mut srcrect: &Rectangle = &rectangle1
                  rectangle2 = Rectangle::new(num2 + 1, num3 + 19, 96, 72);
                  let mut destrect: &Rectangle = &rectangle2
                  DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
                }
                else
                {
                  if ( self.game.Data.RuleVar[869] == 1.0)
                  {
                    let mut nr: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID1[index5];
                     let mut local3: &Graphics = &graphics;
                    bitmap = BitmapStore.GetBitmap(nr);
                     let mut local4: &Bitmap = &bitmap;
                    rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
                    let mut srcrect: &Rectangle = &rectangle2
                    rectangle1 = Rectangle::new(num2 + 1, num3 + 19, 96, 72);
                    let mut destrect: &Rectangle = &rectangle1
                    DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
                  }
                  let mut nr1: i32 = self.game.Data.LandscapeTypeObj[index4].SidewaysSPriteID2[index5];
                   let mut local5: &Graphics = &graphics;
                  bitmap = BitmapStore.GetBitmap(nr1);
                   let mut local6: &Bitmap = &bitmap;
                  rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr1));
                  let mut srcrect1: &Rectangle = &rectangle2
                  rectangle1 = Rectangle::new(num2 + 1, num3 + 19, 96, 72);
                  let mut destrect1: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart2( local5,  local6, srcrect1, destrect1);
                }
              }
              else
                DrawMod.DrawBlock( graphics, num2 + 1, num3 + 19, 96, 72, 0, 0, 0,  byte.MaxValue);
            }
            let mut red5: i32 = self.game.Data.RegimeObj[index1].Red;
            let mut green5: i32 = self.game.Data.RegimeObj[index1].Green;
            let mut blue5: i32 = self.game.Data.RegimeObj[index1].Blue;
            switch (baseColor)
            {
              case 0:
                 let mut local7: &Graphics = &graphics;
                bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local8: &Bitmap = &bitmap;
                let mut x1: i32 = num2 + 1;
                let mut y1: i32 = num3 + 19;
                DrawMod.DrawScaled( local7,  local8, x1, y1, 96, 72);
                break;
              case 1:
                 let mut local9: &Graphics = &graphics;
                bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local10: &Bitmap = &bitmap;
                let mut x2: i32 = num2 + 1;
                let mut y2: i32 = num3 + 19;
                let mut width1: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh1: i32 = BitmapStore.Getheight(picSpriteId);
                double r1 =  ( red5 / 256f);
                double g1 =  ( green5 / 256f);
                double b1 =  ( blue5 / 256f);
                DrawMod.DrawScaledColorized2( local9,  local10, x2, y2, 96, 72, width1, origh1,  r1,  g1,  b1, 1f);
                break;
              case 2:
                let mut red2_3: i32 = self.game.Data.RegimeObj[index1].Red2;
                let mut green2_3: i32 = self.game.Data.RegimeObj[index1].Green2;
                let mut blue2_3: i32 = self.game.Data.RegimeObj[index1].Blue2;
                 let mut local11: &Graphics = &graphics;
                bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local12: &Bitmap = &bitmap;
                let mut x3: i32 = num2 + 1;
                let mut y3: i32 = num3 + 19;
                let mut width2: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh2: i32 = BitmapStore.Getheight(picSpriteId);
                double r2 =  ( red2_3 / 256f);
                double g2 =  ( green2_3 / 256f);
                double b2 =  ( blue2_3 / 256f);
                DrawMod.DrawScaledColorized2( local11,  local12, x3, y3, 96, 72, width2, origh2,  r2,  g2,  b2, 1f);
                break;
              case 3:
                let mut red3_1: i32 = self.game.Data.RegimeObj[index1].Red3;
                let mut green3_1: i32 = self.game.Data.RegimeObj[index1].Green3;
                let mut blue3_1: i32 = self.game.Data.RegimeObj[index1].Blue3;
                 let mut local13: &Graphics = &graphics;
                bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local14: &Bitmap = &bitmap;
                let mut x4: i32 = num2 + 1;
                let mut y4: i32 = num3 + 19;
                let mut width3: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh3: i32 = BitmapStore.Getheight(picSpriteId);
                double r3 =  ( red3_1 / 256f);
                double g3 =  ( green3_1 / 256f);
                double b3 =  ( blue3_1 / 256f);
                DrawMod.DrawScaledColorized2( local13,  local14, x4, y4, 96, 72, width3, origh3,  r3,  g3,  b3, 1f);
                break;
              case 4:
                let mut red4_1: i32 = self.game.Data.RegimeObj[index1].Red4;
                let mut green4_1: i32 = self.game.Data.RegimeObj[index1].Green4;
                let mut blue4_1: i32 = self.game.Data.RegimeObj[index1].Blue4;
                 let mut local15: &Graphics = &graphics;
                bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local16: &Bitmap = &bitmap;
                let mut x5: i32 = num2 + 1;
                let mut y5: i32 = num3 + 19;
                let mut width4: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh4: i32 = BitmapStore.Getheight(picSpriteId);
                double r4 =  ( red4_1 / 256f);
                double g4 =  ( green4_1 / 256f);
                double b4 =  ( blue4_1 / 256f);
                DrawMod.DrawScaledColorized2( local15,  local16, x5, y5, 96, 72, width4, origh4,  r4,  g4,  b4, 1f);
                break;
              case 5:
                 let mut local17: &Graphics = &graphics;
                bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local18: &Bitmap = &bitmap;
                let mut x6: i32 = num2 + 1;
                let mut y6: i32 = num3 + 19;
                let mut width5: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh5: i32 = BitmapStore.Getheight(picSpriteId);
                double r5 =  ( (red5 + 392) / 1024f);
                double g5 =  ( (green5 + 392) / 1024f);
                double b5 =  ( (blue5 + 392) / 1024f);
                DrawMod.DrawScaledColorized2( local17,  local18, x6, y6, 96, 72, width5, origh5,  r5,  g5,  b5, 1f);
                break;
              case 6:
                 let mut local19: &Graphics = &graphics;
                bitmap = BitmapStore.GetBitmap(picSpriteId);
                 let mut local20: &Bitmap = &bitmap;
                let mut x7: i32 = num2 + 1;
                let mut y7: i32 = num3 + 19;
                let mut width6: i32 = BitmapStore.GetWidth(picSpriteId);
                let mut origh6: i32 = BitmapStore.Getheight(picSpriteId);
                double r6 =  ( (red5 + 80) / 512f);
                double g6 =  ( (green5 + 200) / 512f);
                double b6 =  ( (blue5 + 80) / 512f);
                DrawMod.DrawScaledColorized2( local19,  local20, x7, y7, 96, 72, width6, origh6,  r6,  g6,  b6, 1f);
                break;
            }
            if ( self.game.Data.RuleVar[870] > 0.0 & !Information.IsNothing( BitmapStore.GetBitmap(sidewaysSpriteId)))
            {
               let mut local21: &Graphics = &graphics;
              bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
               let mut local22: &Bitmap = &bitmap;
              let mut x8: i32 = num2 + 1;
              let mut y8: i32 = num3 + 19;
              DrawMod.DrawScaled( local21,  local22, x8, y8, 96, 72);
            }
            if ( self.game.Data.RuleVar[869] >= 1.0 &  self.game.Data.RuleVar[869] < 3.0 && self.game.Data.UnitObj[self.unr].X > -1)
            {
              let mut index6: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y].LandscapeType;
              let mut index7: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y].SpriteNr;
              if (self.game.Data.LandscapeTypeObj[index6].AirOverride > -1 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 2)
              {
                index6 = self.game.Data.LandscapeTypeObj[index6].AirOverride;
                index7 = 0;
              }
              else if ( self.game.Data.RuleVar[848] > 0.0 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 2)
              {
                index6 =  Math.Round( self.game.Data.RuleVar[848]);
                index7 = 0;
              }
              if (self.game.Data.LandscapeTypeObj[index6].NavyOverride > -1 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 1)
              {
                index6 = self.game.Data.LandscapeTypeObj[index6].NavyOverride;
                index7 = 0;
              }
              else if ( self.game.Data.RuleVar[872] > 0.0 & self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 1)
              {
                index6 =  Math.Round( self.game.Data.RuleVar[872]);
                index7 = 0;
              }
              let mut nr: i32 = self.game.Data.LandscapeTypeObj[index6].SidewaysSPriteID3[index7];
               let mut local23: &Graphics = &graphics;
              bitmap = BitmapStore.GetBitmap(nr);
               let mut local24: &Bitmap = &bitmap;
              rectangle2 = Rectangle::new(0, 0, 138, BitmapStore.Getheight(nr));
              let mut srcrect: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(num2 + 1, num3 + 19, 96, 72);
              let mut destrect: &Rectangle = &rectangle1
              DrawMod.DrawSimplePart2( local23,  local24, srcrect, destrect);
            }
            DrawMod.DrawRectangle( graphics, num2, num3 + 18, 97, 73,  self.game.VicColor3.R,  self.game.VicColor3.G,  self.game.VicColor3.B,  self.game.VicColor3.A);
            if (self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].EP > 0)
            {
              str: String = Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].EP));
              if (coordinate1.x < 2)
                str = "?";
               let mut local25: &Graphics = &graphics;
              rectangle2 = Rectangle::new(num2 + 98, num3 + 18, 26, 15);
              let mut rect1: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(num2 + 124, num3 + 18, 28, 15);
              let mut rect2: &Rectangle = &rectangle1
              txt2: String = str;
              DrawMod.MakeFullBoxVic( local25, rect1, "EP", rect2, txt2, 3);
            }
            else
            {
              str: String = Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].Ap));
              if (coordinate1.x < 2)
                str = "?";
               let mut local26: &Graphics = &graphics;
              rectangle2 = Rectangle::new(num2 + 98, num3 + 18, 26, 15);
              let mut rect1: &Rectangle = &rectangle2
              rectangle1 = Rectangle::new(num2 + 124, num3 + 18, 28, 15);
              let mut rect2: &Rectangle = &rectangle1
              txt2: String = str;
              DrawMod.MakeFullBoxVic( local26, rect1, "AP", rect2, txt2, 3);
            }
            str1: String = Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].Rdn));
            if (coordinate1.x < 2)
              str1 = "?";
             let mut local27: &Graphics = &graphics;
            rectangle2 = Rectangle::new(num2 + 98, num3 + 18 + 15, 26, 15);
            let mut rect1_1: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(num2 + 124, num3 + 18 + 15, 28, 15);
            let mut rect2_1: &Rectangle = &rectangle1
            txt2_1: String = str1;
            DrawMod.MakeFullBoxVic( local27, rect1_1, "RDN", rect2_1, txt2_1, 3);
            str2: String = Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].Xp));
            if (coordinate1.x < 2)
              str2 = "?";
             let mut local28: &Graphics = &graphics;
            rectangle2 = Rectangle::new(num2 + 98, num3 + 18 + 30, 26, 15);
            let mut rect1_2: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(num2 + 124, num3 + 18 + 30, 28, 15);
            let mut rect2_2: &Rectangle = &rectangle1
            txt2_2: String = str2;
            DrawMod.MakeFullBoxVic( local28, rect1_2, "EXP", rect2_2, txt2_2, 3);
            str3: String = Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].Mor));
            if (coordinate1.x < 2)
              str3 = "?";
             let mut local29: &Graphics = &graphics;
            rectangle2 = Rectangle::new(num2 + 98, num3 + 18 + 45, 26, 15);
            let mut rect1_3: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(num2 + 124, num3 + 18 + 45, 28, 15);
            let mut rect2_3: &Rectangle = &rectangle1
            txt2_3: String = str3;
            DrawMod.MakeFullBoxVic( local29, rect1_3, "MOR", rect2_3, txt2_3, 3);
            str4: String = Strings.LCase(Strings.Left(self.game.Data.PeopleObj[people].Name, 3));
            if (coordinate1.x < 2)
              str4 = "?";
             let mut local30: &Graphics = &graphics;
            rectangle2 = Rectangle::new(num2 + 98, num3 + 18 + 60, 26, 15);
            let mut rect1_4: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(num2 + 124, num3 + 18 + 60, 28, 15);
            let mut rect2_4: &Rectangle = &rectangle1
            txt2_4: String = str4;
            DrawMod.MakeFullBoxVic( local30, rect1_4, "PE", rect2_4, txt2_4, 3);
            if (self.game.Data.SFObj[sf].OffMod > 0)
            {
              tstring2: String = "+" + Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].OffMod)) + "%";
              DrawMod.DrawBlockGradient2( graphics, num2 + 10, num3 + 25, 31, 15, Color.Red, Color.DarkRed);
              DrawMod.DrawTextVic( graphics, tstring2, self.game.VicFont4, num2 + 11, num3 + 27, self.game.VicColor1, self.game.VicColor1Shade);
            }
            else if (self.game.Data.SFObj[sf].OffMod < 0)
            {
              DrawMod.DrawBlockGradient2( graphics, num2 + 10, num3 + 25, 31, 15, Color.Red, Color.DarkRed);
              tstring3: String = Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].OffMod)) + "%";
              DrawMod.DrawTextVic( graphics, tstring3, self.game.VicFont4, num2 + 11, num3 + 27, self.game.VicColor1, self.game.VicColor1Shade);
            }
            if (self.game.Data.SFTypeObj[self.game.Data.SFObj[sf].Type].Theater == 2)
            {
              float num6 =  self.game.HandyFunctionsObj.GetAirFieldStackModifier(self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y) >= 1.0 ? 1f :  (0.33 + 0.66 *  self.game.HandyFunctionsObj.GetAirFieldStackModifier(self.game.Data.UnitObj[self.unr].X, self.game.Data.UnitObj[self.unr].Y));
              if ( num6 < 1.0)
              {
                let mut Number2: i32 = - Math.Round(100.0 - 100.0 *  num6);
                DrawMod.DrawBlockGradient2( graphics, num2 + 10, num3 + 75, 31, 15, Color.Green, Color.DarkGreen);
                tstring4: String = Strings.Trim(Conversion.Str( Number2)) + "%";
                DrawMod.DrawTextVic( graphics, tstring4, self.game.VicFont4, num2 + 11, num3 + 77, self.game.VicColor1, self.game.VicColor1Shade);
              }
            }
            if (self.game.Data.SFObj[sf].DefMod > 0)
            {
              tstring5: String = "+" + Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].DefMod)) + "%";
              DrawMod.DrawBlockGradient2( graphics, num2 + 60, num3 + 25, 33, 15, Color.Blue, Color.DarkBlue);
              DrawMod.DrawTextVic( graphics, tstring5, self.game.VicFont4, num2 + 61, num3 + 27, self.game.VicColor1, self.game.VicColor1Shade);
            }
            else if (self.game.Data.SFObj[sf].DefMod < 0)
            {
              DrawMod.DrawBlockGradient2( graphics, num2 + 60, num3 + 25, 33, 15, Color.Blue, Color.DarkBlue);
              tstring6: String = Strings.Trim(Conversion.Str( self.game.Data.SFObj[sf].DefMod)) + "%";
              DrawMod.DrawTextVic( graphics, tstring6, self.game.VicFont4, num2 + 61, num3 + 27, self.game.VicColor1, self.game.VicColor1Shade);
            }
            if (self.game.Data.SFObj[sf].MoveType > -1)
            {
              tstring7: String = Strings.UCase(self.game.Data.TempString[self.game.Data.SFObj[sf].MoveType]);
              DrawMod.DrawTextVic( graphics, tstring7, self.game.VicFont4, num2 + 4, num3 + 83, self.game.VicColor1, self.game.VicColor1Shade);
            }
          }
        }
        if (self.game.Data.UnitObj[self.unr].PassengerCounter > -1)
        {
          let mut passengerCounter: i32 = self.game.Data.UnitObj[self.unr].PassengerCounter;
          for (let mut index: i32 = 0; index <= passengerCounter; index += 1)
          {
            num1 += 1;
            let mut passenger: i32 = self.game.Data.UnitObj[self.unr].PassengerList[index];
            let mut num7: i32 =  Math.Round(3.0 + Conversion.Int( num1 / 4.0) * -620.0 +  (num1 * 155));
            let mut num8: i32 =  Math.Round(3.0 + Conversion.Int( num1 / 4.0) * 100.0);
            if (self.game.EditObj.SFSelected == num1)
              DrawMod.DrawBlockGradient( graphics, num7, num8, 150, 95, c1_2, c2_2);
            else
              DrawMod.DrawBlockGradient( graphics, num7, num8, 150, 95, c1_1, c2_1);
            self.game.CustomBitmapObj.DrawUnit(passenger, toG: graphics, tx: (num7 + 46), ty: (num8 + 30));
            DrawMod.DrawBlock( graphics, num7, num8, 150, 25, 0, 0, 0, 100);
            name: String = self.game.Data.UnitObj[passenger].Name;
            DrawMod.DrawText( graphics, name, Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), num7, num8);
            if (self.game.EditObj.SFSelected != index)
              ;
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

    pub fn DescriptInfo(ix: i32, iy: i32)
    {
      self.Descript = "";
      let mut num: i32 =  Math.Round(Conversion.Int( iy / 102.0) * 4.0 + Conversion.Int( ix / 155.0));
      if (num > self.game.Data.UnitObj[self.unr].SFCount)
      {
        if (num - (self.game.Data.UnitObj[self.unr].SFCount + 1) > self.game.Data.UnitObj[self.unr].PassengerCounter)
          return;
        self.Descript = "Click to see info on passenger.";
      }
      else
        self.Descript = "Click to see info on this SubformationType. (Ap=Action points, Rd=Readiness, Xp=Experience, Mo=Morale, Pe=People)";
    }

    pub fn Click(x: i32, y: i32, let mut b: i32 = 1) -> i32
    {
      if (self.game.Data.UnitObj[self.unr].SFCount > 7)
      {
        let mut num: i32 =  Math.Round(Conversion.Int( y / 55.0) * 4.0 + Conversion.Int( x / 155.0));
        if (num <= self.game.Data.UnitObj[self.unr].SFCount || num - (self.game.Data.UnitObj[self.unr].SFCount + 1) <= self.game.Data.UnitObj[self.unr].PassengerCounter)
          return num;
      }
      else
      {
        let mut num: i32 =  Math.Round(Conversion.Int( y / 102.0) * 4.0 + Conversion.Int( x / 155.0));
        if (num <= self.game.Data.UnitObj[self.unr].SFCount || num - (self.game.Data.UnitObj[self.unr].SFCount + 1) <= self.game.Data.UnitObj[self.unr].PassengerCounter)
          return num;
      }
      return -1;
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
