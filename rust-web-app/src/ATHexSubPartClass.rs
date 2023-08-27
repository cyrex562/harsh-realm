// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ATHexSubPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class ATHexSubPartClass : SubPartClass
  {
     object OwnBitmapNr;
     int x;
     int y;
     game: GameClass;
     int[] mzx;
     int[] mzy;
     int[] mznr;
     int mzcount;
     int prod;
     bool IgnoreAttack;

    pub ATHexSubPartClass(int tx, int ty, tgame: GameClass, bool tIgnoreAttack = false)
      : base(200, 82)
    {
      this.mzx = new int[401];
      this.mzy = new int[401];
      this.mznr = new int[401];
      this.x = tx;
      this.y = ty;
      this.game = tgame;
      this.prod = 0;
      this.IgnoreAttack = tIgnoreAttack;
    }

    pub fn DescriptInfo(int ix, int iy)
    {
      Coordinate coordinate = Coordinate::new();
      if (this.x == -1 | this.y == -1 || this.game.Data.Round == 0)
        return;
      this.Descript = "";
      if (this.mzcount <= -1)
        return;
      let mut mzcount: i32 =  this.mzcount;
      for (let mut index: i32 =  0; index <= mzcount; index += 1)
      {
        if (ix > this.mzx[index] & iy > this.mzy[index] & ix < this.mzx[index] + 31 & iy < this.mzy[index] + 31)
        {
          if (this.game.Data.UnitObj[this.mznr[index]].Regime == this.game.Data.Turn | this.game.Data.Round == 0)
            coordinate.x = 3;
          else
            coordinate = this.game.HandyFunctionsObj.GetReconMinusHide(this.mznr[index], this.game.Data.Turn);
          if (coordinate.x >= 2)
            this.Descript = this.game.Data.UnitObj[this.mznr[index]].Name;
          else
            this.Descript = "Unknown Unit";
        }
      }
    }

    pub Paint: Bitmap()
    {
      bool flag;
      if (this.game.EditObj.OrderType == 14)
        flag = true;
      if (this.game.EditObj.OrderType == 15)
        flag = true;
      if (this.game.EditObj.OrderType == 2)
        flag = true;
      if (this.game.EditObj.OrderType == 12)
        flag = true;
      if (this.game.EditObj.OrderType == 11)
        flag = true;
      if (this.game.EditObj.OrderType == 13)
        flag = true;
      if (flag & !this.IgnoreAttack)
      {
        this.x = this.game.EditObj.TargetX;
        this.y = this.game.EditObj.TargetY;
      }
      if (this.x == -1 | this.y == -1 || this.game.SelectX == -1 | this.game.SelectY == -1)
      {
        bitmap: Bitmap;
        return bitmap;
      }
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      let mut landscapeType: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].LandscapeType;
      let mut spriteNr: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].SpriteNr;
      if (this.game.Data.Round > 0 && this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].get_SeeNow(this.game.Data.Turn) < 1)
      {
        DrawMod.DrawBlock(ref graphics, 0, 0, 200, 82, 0, 0, 0, (int) byte.MaxValue);
        str: String;
        tstring: String = str + "Shrouded (" + Conversion.Str( this.x) + "," + Conversion.Str( this.y) + ")";
        DrawMod.DrawText(ref graphics, tstring, Font::new("Times New Roman", 12f, FontStyle.Bold, GraphicsUnit.Pixel), 10, 35);
        return this.OwnBitmap;
      }
      if (landscapeType > -1 & spriteNr > -1)
      {
        bitmap: Bitmap;
        if ( this.game.Data.RuleVar[869] == 0.0 |  this.game.Data.RuleVar[869] == 3.0)
        {
          let mut nr: i32 =  this.game.Data.LandscapeTypeObj[landscapeType].BasicPicID[spriteNr];
          ref Graphics local1 = ref graphics;
          bitmap = BitmapStore.GetBitmap(nr);
          ref local2: Bitmap = ref bitmap;
          DrawMod.DrawScaled(ref local1, ref local2, 0, 0, 200, 82);
        }
        else
        {
          if ( this.game.Data.RuleVar[869] == 1.0)
          {
            let mut nr: i32 =  this.game.Data.LandscapeTypeObj[landscapeType].SidewaysSPriteID1[spriteNr];
            ref Graphics local3 = ref graphics;
            bitmap = BitmapStore.GetBitmap(nr);
            ref local4: Bitmap = ref bitmap;
            DrawMod.DrawScaled(ref local3, ref local4, 0, 0, 200, 82);
          }
          let mut nr1: i32 =  this.game.Data.LandscapeTypeObj[landscapeType].SidewaysSPriteID2[spriteNr];
          ref Graphics local5 = ref graphics;
          bitmap = BitmapStore.GetBitmap(nr1);
          ref local6: Bitmap = ref bitmap;
          DrawMod.DrawScaled(ref local5, ref local6, 0, 0, 200, 82);
          let mut nr2: i32 =  this.game.Data.LandscapeTypeObj[landscapeType].SidewaysSPriteID3[spriteNr];
          ref Graphics local7 = ref graphics;
          bitmap = BitmapStore.GetBitmap(nr2);
          ref local8: Bitmap = ref bitmap;
          DrawMod.DrawScaled(ref local7, ref local8, 0, 0, 200, 82);
        }
        if (this.game.Data.MapObj[0].HexObj[this.x, this.y].Location > -1 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureLT > -1)
        {
          if ( this.game.Data.RuleVar[869] == 0.0 |  this.game.Data.RuleVar[869] == 3.0)
          {
            let mut nr: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureLT].BasicPicID[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureSprite];
            ref Graphics local9 = ref graphics;
            bitmap = BitmapStore.GetBitmap(nr);
            ref local10: Bitmap = ref bitmap;
            DrawMod.DrawScaled(ref local9, ref local10, 0, 0, 200, 82);
          }
          else
          {
            if ( this.game.Data.RuleVar[869] == 1.0)
            {
              let mut nr: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureLT].SidewaysSPriteID1[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureSprite];
              ref Graphics local11 = ref graphics;
              bitmap = BitmapStore.GetBitmap(nr);
              ref local12: Bitmap = ref bitmap;
              DrawMod.DrawScaled(ref local11, ref local12, 0, 0, 200, 82);
            }
            let mut nr3: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureLT].SidewaysSPriteID2[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureSprite];
            ref Graphics local13 = ref graphics;
            bitmap = BitmapStore.GetBitmap(nr3);
            ref local14: Bitmap = ref bitmap;
            DrawMod.DrawScaled(ref local13, ref local14, 0, 0, 200, 82);
            let mut nr4: i32 =  this.game.Data.LandscapeTypeObj[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureLT].SidewaysSPriteID3[this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.x, this.y].Location].Type].PictureSprite];
            ref Graphics local15 = ref graphics;
            bitmap = BitmapStore.GetBitmap(nr4);
            ref local16: Bitmap = ref bitmap;
            DrawMod.DrawScaled(ref local15, ref local16, 0, 0, 200, 82);
          }
        }
      }
      DrawMod.DrawRectangle(ref graphics, 0, 0, 199, 81, (int) this.game.VicColor3.R, (int) this.game.VicColor3.G, (int) this.game.VicColor3.B, (int) this.game.VicColor3.A);
      this.mzcount = -1;
      if (!flag | this.IgnoreAttack && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitCounter > -1)
      {
        let mut num1: i32 =  -1;
        let mut unitCounter1: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitCounter;
        let mut num2: i32 =  unitCounter1;
        for (let mut index: i32 =  0; index <= num2; index += 1)
        {
          if (this.game.HandyFunctionsObj.CanWeSeeUnit(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitList[index], this.game.Data.Turn) > 0)
            num1 += 1;
        }
        let mut num3: i32 =  num1;
        let mut num4: i32 =  unitCounter1 + 1;
        let mut unitCounter2: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitCounter;
        for (let mut index: i32 =  unitCounter2; index >= 0; index += -1)
        {
          let mut unit: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitList[index];
          if (this.game.HandyFunctionsObj.CanWeSeeUnit(unit, this.game.Data.Turn) > 0)
          {
            --num4;
            let mut num5: i32 =  (int) Math.Round(170.0 / (Conversion.Int( num3 / 2.0) + 1.0));
            if (num5 > 37)
              num5 = 37;
            let mut num6: i32 =  (int) Math.Round(Conversion.Int( (unitCounter2 + 1) / 2.0));
            if (num6 < 4)
              num6 = 4;
            int ty;
            int tx;
            if (index < num6)
            {
              ty = 2;
              tx = 2 + num4 * num5;
            }
            else
            {
              ty = 41;
              tx = 2 + (num4 - num6) * num5;
            }
            bool forcehighlight = false;
            if (this.game.EditObj.UnitSelected == unit)
              forcehighlight = true;
            this.game.CustomBitmapObj.DrawUnit(unit, forcehighlight, graphics, tx, ty, true);
            this += 1.mzcount;
            this.mzx[this.mzcount] = tx;
            this.mzy[this.mzcount] = ty;
            this.mznr[this.mzcount] = unit;
          }
        }
      }
      if (flag & !this.IgnoreAttack)
      {
        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitCounter > -1)
        {
          let mut unitCounter: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitCounter;
          for (let mut index: i32 =  unitCounter; index >= 0; index += -1)
          {
            let mut unit: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.x, this.y].UnitList[index];
            if (this.game.HandyFunctionsObj.CanWeSeeUnit(unit, this.game.Data.Turn) > 0)
            {
              let mut ty: i32 =  2;
              let mut num: i32 =  (int) Math.Round(170.0 /  (unitCounter + 1));
              if (num > 39)
                num = 39;
              let mut tx: i32 =  2 + index * num;
              this.game.CustomBitmapObj.DrawUnit(unit, true, graphics, tx, ty);
              this += 1.mzcount;
              this.mzx[this.mzcount] = tx;
              this.mzy[this.mzcount] = ty;
              this.mznr[this.mzcount] = unit;
            }
          }
        }
        if (this.game.EditObj.TempUnitList.counter > -1)
        {
          let mut counter: i32 =  this.game.EditObj.TempUnitList.counter;
          for (let mut index: i32 =  counter; index >= 0; index += -1)
          {
            let mut num7: i32 =  this.game.EditObj.TempUnitList.unr[index];
            if (this.game.HandyFunctionsObj.CanWeSeeUnit(num7, this.game.Data.Turn) > 0)
            {
              let mut ty: i32 =  41;
              let mut num8: i32 =  (int) Math.Round(170.0 /  (counter + 1));
              if (num8 > 37)
                num8 = 37;
              let mut tx: i32 =  2 + index * num8;
              this.game.CustomBitmapObj.DrawUnit(num7, true, graphics, tx, ty);
              this += 1.mzcount;
              this.mzx[this.mzcount] = tx;
              this.mzy[this.mzcount] = ty;
              this.mznr[this.mzcount] = num7;
            }
          }
        }
      }
      if (!Information.IsNothing( graphics))
      {
        graphics.Dispose();
        graphics = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub int Click(int x, int y, let mut b: i32 =  1)
    {
      if (this.mzcount <= -1)
        return -1;
      for (let mut mzcount: i32 =  this.mzcount; mzcount >= 0; mzcount += -1)
      {
        if (x > this.mzx[mzcount] & y > this.mzy[mzcount] & x < this.mzx[mzcount] + 37 & y < this.mzy[mzcount] + 37)
          return this.mznr[mzcount];
      }
      return -1;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      ref Graphics local1 = ref graphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(this.OwnBitmapNr));
      ref local2: Bitmap = ref bitmap;
      DrawMod.Draw(ref local1, ref local2, 0, 0, 0.3f, 0.3f, 0.3f, 1f);
      return this.OwnBitmap;
    }
  }
}
