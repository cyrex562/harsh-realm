// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.OfficerPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class OfficerPartClass : SubPartClass
  {
     object OwnBitmapNr;
     unr: i32;
     game: GameClass;
     his: i32;
     bool commander;
     bool AllowMoreButton;

    pub OfficerPartClass(tunr: i32, tgame: GameClass, int @this = -1, bool tAllowMoreButton = false)
      : base(300, 200)
    {
      this.unr = tunr;
      this.game = tgame;
      this.AllowMoreButton = tAllowMoreButton;
      if (this.unr > -1)
        this.his = this.game.Data.UnitObj[this.unr].Historical;
      if (this.his > -1 && Operators.CompareString(this.game.Data.HistoricalUnitObj[this.his].CommanderName, "", false) != 0)
        this.commander = true;
      if (@this <= -1)
        return;
      this.his = @this;
      this.commander = true;
    }

    pub fn DescriptInfo(x: i32, y: i32)
    {
      if (this.game.EditObj.UnitSelected == -1)
        return;
      this.Descript = "";
      if (this.commander)
      {
        let mut num1: i32 =  -1;
        let mut num2: i32 =  Math.Min(3, this.game.Data.HistoricalUnitObj[this.his].HandCardCounter);
        for (let mut index: i32 =  0; index <= num2; index += 1)
        {
          num1 += 1;
          let mut num3: i32 =  140 + num1 * 45;
          let mut num4: i32 =  45;
          if (x > num3 & y > num4 & x < num3 + 45 & y < num4 + 60)
            this.Descript = "Officer can play: " + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.his].HandCard[index]].Title;
        }
        let mut num5: i32 =  Math.Min(3, this.game.Data.HistoricalUnitObj[this.his].DeckCardCounter);
        for (let mut index1: i32 =  0; index1 <= num5; index1 += 1)
        {
          let mut num6: i32 =  0;
          let mut handCardCounter: i32 =  this.game.Data.HistoricalUnitObj[this.his].HandCardCounter;
          for (let mut index2: i32 =  0; index2 <= handCardCounter; index2 += 1)
          {
            if (this.game.Data.HistoricalUnitObj[this.his].DeckCard[index1] == this.game.Data.HistoricalUnitObj[this.his].HandCard[index2])
              num6 = 1;
          }
          if (num6 == 0)
          {
            num1 += 1;
            if (num1 <= 3)
            {
              let mut num7: i32 =  140 + num1 * 45;
              let mut num8: i32 =  45;
              if (x > num7 & y > num8 & x < num7 + 45 & y < num8 + 60)
                this.Descript = "Officer has " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.his].DeckChance[index1]) + "% chance to receive " + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.his].DeckCard[index1]].Title + " as a handcard.";
            }
          }
        }
      }
      if (this.game.EditObj.UnitSelected > -1 & this.game.Data.FOWOn)
      {
        if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn))
        {
          if (x > 50 & y > 7 & x < 138 & y < 104)
            this.Descript = "The portrait of the commander. Click on it for full description.";
          if (x > 50 & y > 107 & x < 350 & y < 177)
            this.Descript = "The description of the commander. Click on it to read the full description.";
        }
      }
      else
      {
        if (x > 50 & y > 7 & x < 138 & y < 104)
          this.Descript = "The portrait of the commander. Click on it for full description.";
        if (x > 50 & y > 107 & x < 350 & y < 177)
          this.Descript = "The description of the commander. Click on it to read the full description.";
      }
      let mut num9: i32 =  4;
      if (x > 0 & y > num9 + 14 & x < 45 & y < num9 + 14 + 23)
        this.Descript = "Staff individuals that can be commanded effectively.";
      let mut num10: i32 =  num9 + 37;
      if (x > 0 & y > num10 + 14 & x < 45 & y < num10 + 14 + 23)
        this.Descript = "Combat modifier of commander. This modifier increases the staff combat bonus.";
      let mut num11: i32 =  num10 + 37;
      if (x > 0 & y > num11 + 14 & x < 45 & y < num11 + 14 + 23)
        this.Descript = "Morale modifier of commander. This modifier increase the staff morale recovery bonus.";
      let mut num12: i32 =  num11 + 37;
      if (!(x > 0 & y > num12 + 14 & x < 45 & y < num12 + 14 + 23))
        return;
      this.Descript = "The commanders experience points.";
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF = SizeF::new();
      Coordinate coordinate = Coordinate::new();
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.unr > -1)
      {
        if (this.game.Data.UnitObj[this.unr].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
          coordinate.x = 3;
        else
          coordinate = this.game.HandyFunctionsObj.GetReconMinusHide(this.unr, this.game.Data.Turn);
      }
      else
        coordinate.x = 3;
      if (coordinate.x > 1 && this.commander && this.game.Data.HistoricalUnitObj[this.his].CommanderSpriteID > -1)
      {
        DrawMod.DrawOfficerATG(objgraphics, this.his, 50, 7, 88, 97);
        str1: String = this.game.Data.HistoricalUnitObj[this.his].CommanderName;
        if (Strings.Len(str1) > 20)
          str1 = Strings.Left(str1, 20);
         let mut local1: &Graphics = &objgraphics;
        Rectangle rectangle1 = Rectangle::new(140, 4, 40, 14);
        let mut rect1_1: &Rectangle = &rectangle1
        Rectangle rectangle2 = Rectangle::new(140, 18, 200, 23);
        let mut rect2_1: &Rectangle = &rectangle2
        txt2_1: String = str1;
        DrawMod.MakeFullBoxVic2( local1, rect1_1, "NAME", rect2_1, txt2_1);
        let mut num1: i32 =  0;
        if (!this.game.Data.FOWOn)
          num1 = 1;
        if (this.unr > -1)
        {
          if (this.game.Data.Turn == this.game.Data.UnitObj[this.unr].Regime)
            num1 = 1;
          if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.UnitObj[this.unr].Regime))
            num1 = 1;
        }
        if (this.unr == -1)
          num1 = 1;
        if (this.game.Data.Round == 0)
          num1 = 1;
        num2: i32;
        if (num1 == 1)
        {
          let mut staffPoints: i32 =  this.game.HandyFunctionsObj.GetStaffPoints(this.unr);
          let mut num3: i32 =  this.game.HandyFunctionsObj.GetStaffNeeded(this.unr);
          if (num3 == 0)
            num3 = 1;
          if ( Math.Round(40.0 * ( staffPoints /  num3)) > 80)
            num2 = 80;
          str2: String = this.game.HandyFunctionsObj.GetMaxStaffIndividuals(this.unr, this.his).ToString();
          str3: String = this.game.Data.HistoricalUnitObj[this.his].CombatMod.ToString() + "%";
          str4: String = this.game.Data.HistoricalUnitObj[this.his].MoraleMod.ToString() + "%";
          str5: String = this.game.Data.HistoricalUnitObj[this.his].Xp.ToString();
          if (this.game.EditObj.UnitSelected > -1 & this.game.Data.FOWOn & this.game.Data.Round > 0 && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn))
          {
            str2 = "?";
            str3 = "?";
            str4 = "?";
            str5 = "?";
          }
          let mut y1: i32 =  4;
           let mut local2: &Graphics = &objgraphics;
          rectangle2 = Rectangle::new(0, y1, 45, 14);
          let mut rect1_2: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, y1 + 14, 45, 23);
          let mut rect2_2: &Rectangle = &rectangle1
          txt2_2: String = str2;
          DrawMod.MakeFullBoxVic2( local2, rect1_2, "STAFF", rect2_2, txt2_2);
          let mut y2: i32 =  y1 + 37;
           let mut local3: &Graphics = &objgraphics;
          rectangle2 = Rectangle::new(0, y2, 48, 14);
          let mut rect1_3: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, y2 + 14, 45, 23);
          let mut rect2_3: &Rectangle = &rectangle1
          txt2_3: String = "+" + str3;
          DrawMod.MakeFullBoxVic2( local3, rect1_3, "COMBAT", rect2_3, txt2_3);
          let mut y3: i32 =  y2 + 37;
           let mut local4: &Graphics = &objgraphics;
          rectangle2 = Rectangle::new(0, y3, 48, 14);
          let mut rect1_4: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, y3 + 14, 45, 23);
          let mut rect2_4: &Rectangle = &rectangle1
          txt2_4: String = "+" + str4;
          DrawMod.MakeFullBoxVic2( local4, rect1_4, "MORALE", rect2_4, txt2_4);
          let mut y4: i32 =  y3 + 37;
           let mut local5: &Graphics = &objgraphics;
          rectangle2 = Rectangle::new(0, y4, 45, 14);
          let mut rect1_5: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, y4 + 14, 45, 23);
          let mut rect2_5: &Rectangle = &rectangle1
          txt2_5: String = str5;
          DrawMod.MakeFullBoxVic2( local5, rect1_5, "EXP", rect2_5, txt2_5);
        }
        bool flag = true;
        if (this.game.EditObj.UnitSelected > -1 && !this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime, this.game.Data.Turn))
          flag = false;
        if (!this.game.Data.FOWOn | this.game.Data.Round == 0)
          flag = true;
        if (flag)
        {
          num2 = 0;
          let mut num4: i32 =  -1;
          let mut num5: i32 =  Math.Min(3, this.game.Data.HistoricalUnitObj[this.his].HandCardCounter);
          for (let mut index: i32 =  0; index <= num5; index += 1)
          {
            num4 += 1;
            let mut num6: i32 =  140 + num4 * 45;
            let mut num7: i32 =  45;
             let mut local6: &Graphics = &objgraphics;
            bitmap: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.HistoricalUnitObj[this.his].HandCard[index], small: true);
             let mut local7: &Bitmap = &bitmap;
            let mut x: i32 =  num6;
            let mut y: i32 =  num7;
            DrawMod.DrawSimple( local6,  local7, x, y);
          }
          let mut num8: i32 =  Math.Min(3, this.game.Data.HistoricalUnitObj[this.his].DeckCardCounter);
          bitmap1: Bitmap;
          for (let mut index1: i32 =  0; index1 <= num8; index1 += 1)
          {
            let mut num9: i32 =  0;
            let mut handCardCounter: i32 =  this.game.Data.HistoricalUnitObj[this.his].HandCardCounter;
            for (let mut index2: i32 =  0; index2 <= handCardCounter; index2 += 1)
            {
              if (this.game.Data.HistoricalUnitObj[this.his].DeckCard[index1] == this.game.Data.HistoricalUnitObj[this.his].HandCard[index2])
                num9 = 1;
            }
            if (num9 == 0)
            {
              num4 += 1;
              if (num4 <= 3)
              {
                let mut x1: i32 =  140 + num4 * 45;
                let mut num10: i32 =  45;
                 let mut local8: &Graphics = &objgraphics;
                bitmap1 = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.HistoricalUnitObj[this.his].DeckCard[index1], small: true);
                 let mut local9: &Bitmap = &bitmap1;
                let mut x2: i32 =  x1;
                let mut y5: i32 =  num10;
                DrawMod.DrawSimple( local8,  local9, x2, y5);
                 let mut local10: &Graphics = &objgraphics;
                bitmap1 = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.HistoricalUnitObj[this.his].DeckCard[index1], small: true);
                 let mut local11: &Bitmap = &bitmap1;
                let mut x3: i32 =  x1;
                let mut y6: i32 =  num10;
                DrawMod.Draw( local10,  local11, x3, y6, -1f, -1f, -1f, 0.5f);
                DrawMod.DrawTextColouredMarc( objgraphics, Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[this.his].DeckChance[index1])) + "%", this.game.MarcFont4, x1, num10 + 38, Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
              }
            }
          }
           let mut local12: &Graphics = &objgraphics;
          Rectangle rectangle3;
          let mut rect1_6: &Rectangle = &rectangle3
          rectangle2 = Rectangle::new(50, 107, 249, 70);
          let mut rect2_6: &Rectangle = &rectangle2
          DrawMod.MakeFullBoxVic2( local12, rect1_6, "", rect2_6, "");
          TextAreaClass textAreaClass = new TextAreaClass(this.game, 280, 3, this.game.VicFont3, "", false, this.game.Data.HistoricalUnitObj[this.his].Descript, Color.White, tbackbitmap: ( this.OwnBitmap), bbx: 50, bby: 109, tHideShade: true, tBlockScroller: true);
           let mut local13: &Graphics = &objgraphics;
          bitmap1 = textAreaClass.Paint();
           let mut local14: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local13,  local14, 50, 109);
        }
      }
      if (!Information.IsNothing( objgraphics))
      {
        objgraphics.Dispose();
        objgraphics = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
       let mut local1: &Graphics = &Expression;
      bitmap: Bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(this.OwnBitmapNr));
       let mut local2: &Bitmap = &bitmap;
      DrawMod.Draw( local1,  local2, 0, 0, 0.3f, 0.3f, 0.3f, 1f);
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return this.OwnBitmap;
    }

    pub fn GetSelect() -> i32 => this.his;
  }
}
