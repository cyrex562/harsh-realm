// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.OfficerInfoWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class OfficerInfoWindowClass2 : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     TAid: i32;
     His: i32;
     Card: i32;

    pub OfficerInfoWindowClass2( tGame: GameClass)
      : base( tGame, 680, 680, 8)
    {
      this.His = this.game.EditObj.OfficerHisOverrule > 0 ? this.game.EditObj.OfficerHisOverrule : tGame.Data.UnitObj[tGame.EditObj.UnitSelected].Historical;
      this.game.EditObj.OfficerHisOverrule = -1;
      this.View();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      for (let mut mouseCounter: i32 =  this.MouseCounter; mouseCounter >= 0; mouseCounter += -1)
      {
        if (x > this.MouseRect[mouseCounter].X & x < this.MouseRect[mouseCounter].X + this.MouseRect[mouseCounter].Width && y > this.MouseRect[mouseCounter].Y & y < this.MouseRect[mouseCounter].Y + this.MouseRect[mouseCounter].Height)
        {
          if (this.MouseData[mouseCounter] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[mouseCounter];
          this.game.EditObj.TipText = this.MouseText[mouseCounter];
          break;
        }
      }
    }

    pub fn View()
    {
      if (this.cancelid > 0)
      {
        this.RemoveSubPart(this.cancelid);
        this.cancelid = 0;
      }
      if (this.TAid > 0)
        this.RemoveSubPart(this.TAid);
      this.ClearMouse();
      this.NewBackGroundAndClearAll(680, 680, -1);
      Graphics g = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame( this.OwnBitmap,  g, 0, 0, 680, 680);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      let mut commanderSpriteId: i32 =  this.game.Data.HistoricalUnitObj[this.His].CommanderSpriteID;
      SizeF sizeF1 = SizeF::new();
      sizeF1 = g.MeasureString(this.game.Data.HistoricalUnitObj[this.His].CommanderName, Font::new(this.game.FontCol.Families[1], 48f, FontStyle.Regular, GraphicsUnit.Pixel));
      DrawMod.DrawTextColouredMarc( g, this.game.Data.HistoricalUnitObj[this.His].CommanderName, this.game.MarcFont1, 20, 19, Color.White);
      DrawMod.DrawOfficer(g, this.His, 20, 70, 177, 194);
      Rectangle trect1;
      Rectangle trect2;
      if ( this.game.Data.RuleVar[976] < 1.0)
      {
        let mut num1: i32 =  270;
        DrawMod.DrawBlockGradient2( g, 20, num1, 79, 19, this.game.MarcCol1, this.game.MarcCol2);
        DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  g, 20, num1, 80, 20, -1, -1);
        str1: String = Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[this.His].CombatMod)) + "%";
        if (this.game.Data.HistoricalUnitObj[this.His].CombatMod > 0)
          str1 = "+" + str1;
        DrawMod.DrawTextColouredMarc( g, "COM = " + str1, this.game.MarcFont5, 25, num1 + 4, Color.White);
        if ( this.game.Data.RuleVar[927] > 0.0)
        {
          trect1 = Rectangle::new(20, num1, 80, 20);
          this.AddMouse( trect1, "COMBAT MODIFIER", "How much percent does the officer increase\r\nstaff combat bonus.\r\nMind you: Only if the officer is in command at the lowest HQ level!");
        }
        else
        {
          trect1 = Rectangle::new(20, num1, 80, 20);
          let mut trect3: &Rectangle = &trect1
          this.AddMouse( trect3, "COMBAT MODIFIER", "How much percent does the officer increase\r\nstaff combat bonus.");
        }
        let mut num2: i32 =  300;
        DrawMod.DrawBlockGradient2( g, 20, num2, 79, 19, this.game.MarcCol1, this.game.MarcCol2);
        DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  g, 20, num2, 80, 20, -1, -1);
        str2: String = Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[this.His].MoraleMod)) + "%";
        if (this.game.Data.HistoricalUnitObj[this.His].MoraleMod > 0)
          str2 = "+" + str2;
        DrawMod.DrawTextColouredMarc( g, "MOR = " + str2, this.game.MarcFont5, 25, num2 + 4, Color.White);
        if ( this.game.Data.RuleVar[927] > 0.0)
        {
          trect1 = Rectangle::new(20, num2, 80, 20);
          let mut trect4: &Rectangle = &trect1
          this.AddMouse( trect4, "MORALE MODIFIER", "How much percent does the officer increase \r\n regular morale recovery.\r\nMind you: Only if the officer is in command at the lowest HQ level!");
        }
        else
        {
          trect1 = Rectangle::new(20, num2, 80, 20);
          let mut trect5: &Rectangle = &trect1
          this.AddMouse( trect5, "MORALE MODIFIER", "How much percent does the officer increase \r\n regular morale recovery.");
        }
        let mut num3: i32 =  270;
        DrawMod.DrawBlockGradient2( g, 110, num3, 79, 19, this.game.MarcCol1, this.game.MarcCol2);
        DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  g, 110, num3, 80, 20, -1, -1);
        str3: String = Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[this.His].StaffSize));
        DrawMod.DrawTextColouredMarc( g, "STAFF = " + str3, this.game.MarcFont5, 120, num3 + 4, Color.White);
        trect1 = Rectangle::new(110, num3, 80, 20);
        let mut trect6: &Rectangle = &trect1
        this.AddMouse( trect6, "STAFF POINTS", "How much staff points can officer \r\n handle in this scenario.");
        let mut num4: i32 =  300;
        if ( this.game.Data.RuleVar[343] == 1.0)
        {
          DrawMod.DrawBlockGradient2( g, 110, num4, 79, 19, this.game.MarcCol1, this.game.MarcCol2);
          DrawMod.DrawFrame( this.OwnBitmap,  this.BackBitmap,  g, 110, num4, 80, 20, -1, -1);
          str4: String = Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[this.His].PP));
          DrawMod.DrawTextColouredMarc( g, "POL = " + str4, this.game.MarcFont5, 120, num4 + 4, Color.White);
          trect1 = Rectangle::new(110, num4, 80, 20);
          trect2 = trect1;
          this.AddMouse( trect2, "POLITICAL VALUE", "If has negative points here \r\n it costs PP to reassign units under this officer.");
        }
      }
      bool flag1 = false;
      let mut hisVarCount: i32 =  this.game.Data.HistoricalUnitObj[this.His].HisVarCount;
      for (let mut index: i32 =  0; index <= hisVarCount; index += 1)
      {
        if (this.game.Data.HistoricalUnitObj[this.His].HisVarCount >= index && this.game.Data.HistoricalUnitObj[this.His].HisVarNato[index] > 0 | this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index] > -1)
        {
          bool flag2 = true;
          if (this.game.Data.HistoricalUnitObj[this.His].HisVarType[index] <= 99 && Operators.CompareString(this.game.Data.TempString[1400 + this.game.Data.HistoricalUnitObj[this.His].HisVarType[index]], "1", false) == 0)
            flag2 = false;
          if (this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index] > -1 && Strings.InStr(this.game.Data.SmallPicName[this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index]], "trans.") > 0)
            flag2 = false;
          if (flag2)
            flag1 = true;
        }
      }
      bitmap1: Bitmap;
      if (this.game.HandyFunctionsObj.GetVisibleHisVar(this.His) > 0 & flag1)
      {
        let mut bbx: i32 =  210;
        let mut num5: i32 =  56;
        TextAreaClass2 textAreaClass2 = new TextAreaClass2(this.game, 420, 3, this.game.MarcFont13, "", 12,  this.BackBitmap, bbx, num5, true);
         let mut local1: &Graphics = &g;
        bitmap2: Bitmap = textAreaClass2.Paint();
         let mut local2: &Bitmap = &bitmap2;
        let mut x1: i32 =  bbx;
        let mut y: i32 =  num5;
        DrawMod.DrawSimple( local1,  local2, x1, y);
        let mut num6: i32 =  0;
        while (num6 < 380)
        {
          index: i32;
          if (this.game.Data.HistoricalUnitObj[this.His].HisVarCount >= index)
          {
            bool flag3 = true;
            if (this.game.Data.HistoricalUnitObj[this.His].HisVarType[index] <= 99 && Operators.CompareString(this.game.Data.TempString[1400 + this.game.Data.HistoricalUnitObj[this.His].HisVarType[index]], "1", false) == 0)
              flag3 = false;
            if (this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index] > -1 && Strings.InStr(this.game.Data.SmallPicName[this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index]], "trans.") > 0)
              flag3 = false;
            if (flag3 & (this.game.Data.HistoricalUnitObj[this.His].HisVarNato[index] > 0 | this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index] > -1))
            {
              str: String = Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[this.His].HisVarValue[index]));
              SizeF sizeF2 = g.MeasureString(str, this.game.MarcFont8b);
              let mut num7: i32 =  bbx + num6;
              let mut num8: i32 =  num5 + 10;
              DrawMod.DrawBlock( g, num7 + 36, num5 + 13, 2, 64,  this.game.MarcCol3.R,  this.game.MarcCol3.G,  this.game.MarcCol3.B, 128);
              let mut x2: i32 =   Math.Round( ( (bbx + num6 + 18) - sizeF2.Width / 2f));
              DrawMod.DrawTextColouredMarc( g, str, this.game.MarcFont8b, x2, 105, Color.White);
              if (this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index] > -1)
              {
                 let mut local3: &Graphics = &g;
                bitmap1 = BitmapStore.GetBitmap(this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.His].HisVarSmall[index]]);
                 let mut local4: &Bitmap = &bitmap1;
                let mut x3: i32 =  x2;
                DrawMod.DrawSimple( local3,  local4, x3, 81);
              }
              else
              {
                 let mut local5: &Graphics = &g;
                bitmap1 = BitmapStore.GetBitmap(this.game.NATO[this.game.Data.HistoricalUnitObj[this.His].HisVarNato[index]]);
                 let mut local6: &Bitmap = &bitmap1;
                let mut x4: i32 =  x2;
                DrawMod.DrawSimple( local5,  local6, x4, 81);
              }
              if (this.game.Data.Turn == this.game.Data.HistoricalUnitObj[this.His].TempRegime)
              {
                trect1 = Rectangle::new(x2, num5, 35, 50);
                trect2 = trect1;
                this.AddMouse( trect2, "", this.game.Data.TempString[1200 + this.game.Data.HistoricalUnitObj[this.His].HisVarType[index]]);
              }
              num6 += 35;
            }
          }
          else
            num6 = 380;
          index += 1;
        }
      }
      let mut num9: i32 =  -1;
      let mut num10: i32 =  -12;
      let mut num11: i32 =  0;
      let mut deckCardCounter1: i32 =  this.game.Data.HistoricalUnitObj[this.His].DeckCardCounter;
      x5: i32;
      y1: i32;
      num12: i32;
      for (let mut index: i32 =  0; index <= deckCardCounter1; index += 1)
      {
        let mut num13: i32 =  1;
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow > 0)
          num13 = 0;
        if (num13 == 1)
        {
          num9 += 1;
          num11 += 1;
          if (num11 == 1)
          {
            let mut num14: i32 =  num10 + 12;
            x5 = 20;
            y1 = 330 + num14;
            tstring: String = "Officer deck cards:";
            DrawMod.DrawTextColouredMarc( g, tstring, this.game.MarcFont8b, x5, y1, Color.White);
            num10 = num14 + 24;
          }
          if (num9 <= 15)
          {
            if (num9 <= 3)
            {
              x5 = num9 * 47 + 20;
              y1 = 330 + num10;
              num12 = 55;
            }
            else if (num9 <= 7)
            {
              x5 = (num9 - 4) * 47 + 20;
              y1 = 385 + num10;
              num12 = 110;
            }
            else if (num9 <= 11)
            {
              x5 = (num9 - 8) * 47 + 20;
              y1 = 440 + num10;
              num12 = 165;
            }
            else if (num9 <= 15)
            {
              x5 = (num9 - 12) * 47 + 20;
              y1 = 495 + num10;
              num12 = 220;
            }
          }
           let mut local7: &Graphics = &g;
          bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.HistoricalUnitObj[this.His].TempRegime, this.game.Data.HistoricalUnitObj[this.His].DeckCard[index], size: 3, Percent: this.game.Data.HistoricalUnitObj[this.His].DeckChance[index]);
           let mut local8: &Bitmap = &bitmap1;
          let mut x6: i32 =  x5;
          let mut y2: i32 =  y1;
          DrawMod.DrawSimple( local7,  local8, x6, y2);
          trect1 = Rectangle::new(x5, y1, 33, 46);
          trect2 = trect1;
          this.AddMouse( trect2, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Title, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Text + "\r\n\r\nEffect:\r\n" + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].MouseOver, 700 + index);
        }
      }
      let mut num15: i32 =  0;
      let mut num16: i32 =  -1;
      let mut num17: i32 =  num10 + num12;
      let mut num18: i32 =  0;
      let mut num19: i32 =  0;
      let mut deckCardCounter2: i32 =  this.game.Data.HistoricalUnitObj[this.His].DeckCardCounter;
      for (let mut index: i32 =  0; index <= deckCardCounter2; index += 1)
      {
        let mut num20: i32 =  1;
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 0)
          num20 = 0;
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 2)
          num20 = 0;
        if (this.game.Data.Product == 6 && this.game.Data.HistoricalUnitObj[this.His].Type > 5)
          num20 = 0;
        if (num20 == 1)
          num19 += 1;
      }
      let mut deckCardCounter3: i32 =  this.game.Data.HistoricalUnitObj[this.His].DeckCardCounter;
      for (let mut index: i32 =  0; index <= deckCardCounter3; index += 1)
      {
        let mut num21: i32 =  1;
        if (this.game.EditObj.OrderType != 45)
        {
          let mut num22: i32 =  this.game.Data.HistoricalUnitObj[this.His].Type > 5 & this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 1 ? 1 : 0;
        }
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 0)
          num21 = 0;
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 2)
          num21 = 0;
        if (this.game.Data.Product == 6 && this.game.Data.HistoricalUnitObj[this.His].Type > 5)
          num21 = 0;
        if (num21 == 1)
        {
          num16 += 1;
          num15 += 1;
          if (num15 == 1)
          {
            let mut num23: i32 =  num17 + 12;
            x5 = 20;
            y1 = 330 + num23;
            tstring: String = "Tactical deck cards:";
            DrawMod.DrawTextColouredMarc( g, tstring, this.game.MarcFont8b, x5, y1, Color.White);
            num17 = num23 + 24;
          }
          if (this.game.Data.Product == 6)
          {
            let mut num24: i32 =   Math.Round(133.0 /  Math.Max(1, num19 - 1));
             let mut local9: &Graphics = &g;
            bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.HistoricalUnitObj[this.His].DeckCard[index], size: 2);
             let mut local10: &Bitmap = &bitmap1;
            let mut x7: i32 =  x5;
            let mut y3: i32 =  y1 + 20;
            DrawMod.DrawSimple( local9,  local10, x7, y3);
            trect1 = Rectangle::new(x5, y1 + 20, 105, 147);
            trect2 = trect1;
            this.AddMouse( trect2, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Title, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Text + "\r\n\r\nEffect:\r\n" + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].MouseOver, 700 + index);
            y1 += num24;
          }
          else
          {
            if (num16 <= 15)
            {
              if (num16 <= 3)
              {
                x5 = num16 * 47 + 20;
                y1 = 330 + num17;
                num18 = 55;
              }
              else if (num16 <= 7)
              {
                x5 = (num16 - 4) * 47 + 20;
                y1 = 385 + num17;
                num18 = 110;
              }
              else if (num16 <= 11)
              {
                x5 = (num16 - 8) * 47 + 20;
                y1 = 440 + num17;
                num18 = 165;
              }
              else if (num16 <= 15)
              {
                x5 = (num16 - 12) * 47 + 20;
                y1 = 495 + num17;
                num18 = 220;
              }
            }
            if (this.His > -1)
            {
               let mut local11: &Graphics = &g;
              bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.Data.HistoricalUnitObj[this.His].DeckCard[index], size: 3, Percent: this.game.Data.HistoricalUnitObj[this.His].DeckChance[index]);
               let mut local12: &Bitmap = &bitmap1;
              let mut x8: i32 =  x5;
              let mut y4: i32 =  y1;
              DrawMod.DrawSimple( local11,  local12, x8, y4);
            }
            else
            {
               let mut local13: &Graphics = &g;
              bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.HistoricalUnitObj[this.His].DeckCard[index], size: 3, Percent: this.game.Data.HistoricalUnitObj[this.His].DeckChance[index]);
               let mut local14: &Bitmap = &bitmap1;
              let mut x9: i32 =  x5;
              let mut y5: i32 =  y1;
              DrawMod.DrawSimple( local13,  local14, x9, y5);
            }
            trect1 = Rectangle::new(x5, y1, 33, 46);
            trect2 = trect1;
            this.AddMouse( trect2, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Title, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Text + "\r\n\r\nEffect:\r\n" + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].MouseOver, 700 + index);
          }
        }
      }
      let mut num25: i32 =  0;
      let mut num26: i32 =  -1;
      let mut num27: i32 =  num17 + num18;
      let mut num28: i32 =  0;
      let mut deckCardCounter4: i32 =  this.game.Data.HistoricalUnitObj[this.His].DeckCardCounter;
      for (let mut index: i32 =  0; index <= deckCardCounter4; index += 1)
      {
        let mut num29: i32 =  1;
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 0)
          num29 = 0;
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 1)
          num29 = 0;
        if (this.game.Data.Product == 6 && this.game.Data.HistoricalUnitObj[this.His].Type < 6)
          num29 = 0;
        if (num29 == 1)
          num28 += 1;
      }
      let mut deckCardCounter5: i32 =  this.game.Data.HistoricalUnitObj[this.His].DeckCardCounter;
      for (let mut index: i32 =  0; index <= deckCardCounter5; index += 1)
      {
        let mut num30: i32 =  1;
        if (this.game.EditObj.OrderType != 45)
        {
          let mut num31: i32 =  this.game.Data.HistoricalUnitObj[this.His].Type < 6 & this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 2 ? 1 : 0;
        }
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 0)
          num30 = 0;
        if (this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].LimitedShow == 1)
          num30 = 0;
        if (this.game.Data.Product == 6 && this.game.Data.HistoricalUnitObj[this.His].Type < 6)
          num30 = 0;
        if (num30 == 1)
        {
          num26 += 1;
          num25 += 1;
          if (num25 == 1)
          {
            let mut num32: i32 =  num27 + 12;
            x5 = 20;
            y1 = 330 + num32;
            tstring: String = "Strategic deck cards:";
            DrawMod.DrawTextColouredMarc( g, tstring, this.game.MarcFont8b, x5, y1, Color.White);
            num27 = num32 + 24;
          }
          if (this.game.Data.Product == 6)
          {
            let mut num33: i32 =   Math.Round(133.0 /  Math.Max(1, num28 - 1));
             let mut local15: &Graphics = &g;
            bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.HistoricalUnitObj[this.His].DeckCard[index], size: 2);
             let mut local16: &Bitmap = &bitmap1;
            let mut x10: i32 =  x5;
            let mut y6: i32 =  y1 + 20;
            DrawMod.DrawSimple( local15,  local16, x10, y6);
            trect1 = Rectangle::new(x5, y1 + 20, 105, 147);
            trect2 = trect1;
            this.AddMouse( trect2, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Title, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Text + "\r\n\r\nEffect:\r\n" + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].MouseOver, 700 + index);
            y1 += num33;
          }
          else
          {
            if (num26 <= 15)
            {
              if (num26 <= 3)
              {
                x5 = num26 * 47 + 20;
                y1 = 330 + num27;
              }
              else if (num26 <= 7)
              {
                x5 = (num26 - 4) * 47 + 20;
                y1 = 385 + num27;
              }
              else if (num26 <= 11)
              {
                x5 = (num26 - 8) * 47 + 20;
                y1 = 440 + num27;
              }
              else if (num26 <= 15)
              {
                x5 = (num26 - 12) * 47 + 20;
                y1 = 495 + num27;
              }
            }
            if (this.His > -1)
            {
               let mut local17: &Graphics = &g;
              bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.Data.HistoricalUnitObj[this.His].DeckCard[index], size: 3, Percent: this.game.Data.HistoricalUnitObj[this.His].DeckChance[index]);
               let mut local18: &Bitmap = &bitmap1;
              let mut x11: i32 =  x5;
              let mut y7: i32 =  y1;
              DrawMod.DrawSimple( local17,  local18, x11, y7);
            }
            else
            {
               let mut local19: &Graphics = &g;
              bitmap1 = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.HistoricalUnitObj[this.His].DeckCard[index], size: 3, Percent: this.game.Data.HistoricalUnitObj[this.His].DeckChance[index]);
               let mut local20: &Bitmap = &bitmap1;
              let mut x12: i32 =  x5;
              let mut y8: i32 =  y1;
              DrawMod.DrawSimple( local19,  local20, x12, y8);
            }
            trect1 = Rectangle::new(x5, y1, 33, 46);
            trect2 = trect1;
            this.AddMouse( trect2, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Title, this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].Text + "\r\n\r\nEffect:\r\n" + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.His].DeckCard[index]].MouseOver, 700 + index);
          }
        }
      }
      if (this.game.HandyFunctionsObj.GetVisibleHisVar(this.His) > 0 & flag1)
      {
        let mut tsubpart: SubPartClass =  new TextAreaClass2(this.game, 420, 25, this.game.MarcFont8, this.game.Data.HistoricalUnitObj[this.His].Descript, tbackbitmap: ( this.BackBitmap), bbx: 210, bby: 150);
        this.TAid = this.AddSubPart( tsubpart, 210, 150, 420, 432, 0);
      }
      else
      {
        let mut tsubpart: SubPartClass =  new TextAreaClass2(this.game, 420, 29, this.game.MarcFont8, this.game.Data.HistoricalUnitObj[this.His].Descript, tbackbitmap: ( this.BackBitmap), bbx: 210, bby: 56);
        this.TAid = this.AddSubPart( tsubpart, 210, 56, 420, 496, 0);
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("OK", 150, "Click to return to main screen.",  this.OwnBitmap, 316, 615, theight: 40, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
      this.cancelid = this.AddSubPart( tsubpart1, 316, 615, 150, 40, 1);
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      try
      {
        if (nr == 27 | nr == 32)
        {
          windowReturnClass.AddCommand(6, 0);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 40)
        {
          this.SubPartList[this.SubpartNr(this.TAid)].ShiftDown();
          this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
          windowReturnClass.SetFlag(true);
        }
        if (nr == 38)
        {
          this.SubPartList[this.SubpartNr(this.TAid)].ShiftUp();
          this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
          windowReturnClass.SetFlag(true);
        }
        if (nr == 37)
        {
          this.SubPartList[this.SubpartNr(this.TAid)].ShiftLeft();
          this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
          windowReturnClass.SetFlag(true);
        }
        if (nr == 39)
        {
          this.SubPartList[this.SubpartNr(this.TAid)].ShiftRight();
          this.SubPartFlag[this.SubpartNr(this.TAid)] = true;
          windowReturnClass.SetFlag(true);
        }
      }
      catch (Exception ex)
      {
        ProjectData.SetProjectError(ex);
        ProjectData.ClearProjectError();
      }
      return windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num: i32 =  this.SubPartID[index];
            if (num == this.TAid)
            {
              this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num == this.cancelid)
            {
              windowReturnClass.AddCommand(6, 0);
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
