// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.QuickUnitInfoSubPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class QuickUnitInfoSubPartClass : SubPartClass
  {
     object OwnBitmapNr;
     unr: i32;
     game: GameClass;
     bool shownothing;

    pub QuickUnitInfoSubPartClass(tunr: i32, tgame: GameClass, bool tShowNothing = false)
      : base(96, 200)
    {
      this.unr = tunr;
      this.game = tgame;
      this.shownothing = tShowNothing;
    }

    pub fn DescriptInfo(x: i32, y: i32)
    {
      if (x == -1 | y == -1 || this.game.Data.Round == 0 || this.unr == -1)
        return;
      this.Descript = "";
      if (y > 5 & y < 28)
      {
        if (this.game.Data.UnitObj[this.unr].IsHQ)
          this.Descript = "Headquarter supply stock available for troops. Total supply including stock of troops in hq is " + Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].Supply)) + ". Click to ditch supply pts";
        else
          this.Descript = "Supply stocks the unit has with it. Can max be " + Conversion.Str( this.game.HandyFunctionsObj.UnitSupplyStore(this.unr));
      }
      if (y > 28 & y < 51)
        this.Descript = "Amount of supply that arrived at the unit at start of turn.";
      if (y > 51 & y < 74)
        this.Descript = "Amount of supply the unit had requested at start of turn.";
      if (y > 74 & y < 97)
        this.Descript = "Amount of supply that was sent to unit, but destroyed due to anti-supply pts";
      if (this.game.Data.UnitObj[this.unr].IsHQ)
      {
        if (y > 97 & y < 120)
          this.Descript = "Real total supply points in HQ.";
        if (y > 120 & y < 143)
          this.Descript = "The total amount of supply points subordinates requested in supply points at start of turn.";
        if (y > 143 & y < 166)
          this.Descript = "The total amount of supply points sent out to subordinates at start of turn.";
        if (!(y > 166 & y < 189 &  this.game.Data.RuleVar[322] == 0.0))
          return;
        this.Descript = "The desired supply reserve points for this HQ. Click on it to set it.";
      }
      else
      {
        if (!this.game.HandyFunctionsObj.DoesUnitUseFuel(this.unr))
          return;
        if (y > 97 & y < 120)
          this.Descript = "Fuel used in movement (and moving into attack hex)";
        if (y > 120 & y < 143)
          this.Descript = "Fuel used in offensive combat";
        if (!(y > 143 & y < 166))
          return;
        this.Descript = "Fuel used in defensive combat";
      }
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF = SizeF::new();
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.shownothing)
      {
        DrawMod.Clear( objgraphics, 101, 101, 101);
        return this.OwnBitmap;
      }
      let mut num: i32 = 40;
      let mut width: i32 = 56;
      str1: String;
      if (this.game.Data.UnitObj[this.unr].IsHQ)
      {
        str1 = Strings.Trim(Conversion.Str( this.game.HandyFunctionsObj.GetRealHQSupplyPts(this.unr)));
      }
      else
      {
        str2: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].Supply));
        if (this.game.HandyFunctionsObj.UnitSupplyUse(this.unr) > 0)
        {
          double Number = Math.Round( this.game.Data.UnitObj[this.unr].Supply /  this.game.HandyFunctionsObj.UnitSupplyUse(this.unr), 1);
          if (Number > 99.0)
            Number = 99.0;
          str1 = str2 + " (" + Strings.Trim(Conversion.Str( Number)) + ")";
        }
        else
          str1 = str2 + " (>99)";
      }
       let mut local1: &Graphics = &objgraphics;
      Rectangle rectangle1 = Rectangle::new(0, 5, num, 23);
      let mut rect1_1: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(num, 5, width, 23);
      let mut rect2_1: &Rectangle = &rectangle2
      txt2_1: String = str1;
      DrawMod.MakeFullBoxVic( local1, rect1_1, "STK", rect2_1, txt2_1, 1);
      str3: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].SupplyIn));
       let mut local2: &Graphics = &objgraphics;
      rectangle2 = Rectangle::new(0, 28, num, 23);
      let mut rect1_2: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(num, 28, width, 23);
      let mut rect2_2: &Rectangle = &rectangle1
      txt2_2: String = str3;
      DrawMod.MakeFullBoxVic( local2, rect1_2, "IN", rect2_2, txt2_2, 1);
      str4: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].SupplyInReq));
       let mut local3: &Graphics = &objgraphics;
      rectangle2 = Rectangle::new(0, 51, num, 23);
      let mut rect1_3: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(num, 51, width, 23);
      let mut rect2_3: &Rectangle = &rectangle1
      txt2_3: String = str4;
      DrawMod.MakeFullBoxVic( local3, rect1_3, "REQ", rect2_3, txt2_3, 1);
      str5: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].SupplyLost));
       let mut local4: &Graphics = &objgraphics;
      rectangle2 = Rectangle::new(0, 74, num, 23);
      let mut rect1_4: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(num, 74, width, 23);
      let mut rect2_4: &Rectangle = &rectangle1
      txt2_4: String = str5;
      DrawMod.MakeFullBoxVic( local4, rect1_4, "DES", rect2_4, txt2_4, 1);
      if (this.game.Data.UnitObj[this.unr].IsHQ)
      {
        str6: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].Supply));
         let mut local5: &Graphics = &objgraphics;
        rectangle2 = Rectangle::new(0, 97, num, 23);
        let mut rect1_5: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(num, 97, width, 23);
        let mut rect2_5: &Rectangle = &rectangle1
        txt2_5: String = str6;
        DrawMod.MakeFullBoxVic( local5, rect1_5, "RST", rect2_5, txt2_5, 1);
        str7: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].SupplyReq));
         let mut local6: &Graphics = &objgraphics;
        rectangle2 = Rectangle::new(0, 120, num, 23);
        let mut rect1_6: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(num, 120, width, 23);
        let mut rect2_6: &Rectangle = &rectangle1
        txt2_6: String = str7;
        DrawMod.MakeFullBoxVic( local6, rect1_6, "ORQ", rect2_6, txt2_6, 1);
        str8: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].SupplyOut));
         let mut local7: &Graphics = &objgraphics;
        rectangle2 = Rectangle::new(0, 143, num, 23);
        let mut rect1_7: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(num, 143, width, 23);
        let mut rect2_7: &Rectangle = &rectangle1
        txt2_7: String = str8;
        DrawMod.MakeFullBoxVic( local7, rect1_7, "OUT", rect2_7, txt2_7, 1);
        str9: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].Reserve));
        if ( this.game.Data.RuleVar[322] == 0.0)
        {
           let mut local8: &Graphics = &objgraphics;
          rectangle2 = Rectangle::new(0, 166, num, 23);
          let mut rect1_8: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(num, 166, width, 23);
          let mut rect2_8: &Rectangle = &rectangle1
          txt2_8: String = str9;
          DrawMod.MakeFullBoxVic( local8, rect1_8, "RES", rect2_8, txt2_8, 1);
        }
      }
      else if (this.game.HandyFunctionsObj.DoesUnitUseFuel(this.unr))
      {
        str10: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].FuelUsedMove));
         let mut local9: &Graphics = &objgraphics;
        rectangle2 = Rectangle::new(0, 97, num, 23);
        let mut rect1_9: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(num, 97, width, 23);
        let mut rect2_9: &Rectangle = &rectangle1
        txt2_9: String = str10;
        DrawMod.MakeFullBoxVic( local9, rect1_9, "FMV", rect2_9, txt2_9, 1);
        str11: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].FuelUsedAtt));
         let mut local10: &Graphics = &objgraphics;
        rectangle2 = Rectangle::new(0, 120, num, 23);
        let mut rect1_10: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(num, 120, width, 23);
        let mut rect2_10: &Rectangle = &rectangle1
        txt2_10: String = str11;
        DrawMod.MakeFullBoxVic( local10, rect1_10, "FAT", rect2_10, txt2_10, 1);
        str12: String = Strings.Trim(Conversion.Str( this.game.Data.UnitObj[this.unr].FuelUsedDef));
         let mut local11: &Graphics = &objgraphics;
        rectangle2 = Rectangle::new(0, 143, num, 23);
        let mut rect1_11: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(num, 143, width, 23);
        let mut rect2_11: &Rectangle = &rectangle1
        txt2_11: String = str12;
        DrawMod.MakeFullBoxVic( local11, rect1_11, "FDF", rect2_11, txt2_11, 1);
      }
      if (!Information.IsNothing( objgraphics))
        objgraphics.Dispose();
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
  }
}
