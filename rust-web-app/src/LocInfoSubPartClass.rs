// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LocInfoSubPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class LocInfoSubPartClass : SubPartClass
  {
     object OwnBitmapNr;
     object locnr;
     game: GameClass;
     int[] mzx;
     int[] mzy;
     int[] mznr;
     mzcount: i32;

    pub LocInfoSubPartClass(tlocnr: i32, tgame: GameClass)
      : base(200, 150)
    {
      self.mzx = new int[41];
      self.mzy = new int[41];
      self.mznr = new int[41];
      self.locnr =  tlocnr;
      self.game = tgame;
    }

    pub Paint: Bitmap()
    {
      if (Operators.ConditionalCompareObjectLess(self.locnr,  0, false))
        return self.OwnBitmap;
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      let mut type: i32 =  self.game.Data.LocObj[Conversions.ToInteger(self.locnr)].Type;
      let mut regime: i32 =  self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.LocObj[Conversions.ToInteger(self.locnr)].X, self.game.Data.LocObj[Conversions.ToInteger(self.locnr)].Y].Regime;
      if (self.game.Data.Round > 0)
      {
        red: i32;
        green: i32;
        blue: i32;
        num1: i32;
        num2: i32;
        num3: i32;
        if (regime == -1)
        {
          red = 128;
          green = 128;
          blue = 128;
          num1 = 0;
          num2 = 0;
          num3 = 0;
        }
        else
        {
          red = self.game.Data.RegimeObj[regime].Red;
          green = self.game.Data.RegimeObj[regime].Green;
          blue = self.game.Data.RegimeObj[regime].Blue;
          num1 = self.game.Data.RegimeObj[regime].Red2;
          num2 = self.game.Data.RegimeObj[regime].Green2;
          num3 = self.game.Data.RegimeObj[regime].Blue2;
        }
        c1: Color = Color.FromArgb( byte.MaxValue, red, green, blue);
        c2: Color = Color.FromArgb(150, red, green, blue);
        DrawMod.DrawBlockGradient( Expression, 0, 0, 200, 50, c1, c2);
      }
      else
      {
        c1: Color = Color.FromArgb( byte.MaxValue, 180, 180, 180);
        c2: Color = Color.FromArgb(150, 90, 90, 90);
        DrawMod.DrawBlockGradient( Expression, 0, 0, 200, 50, c1, c2);
      }
      if (Strings.Len(self.game.Data.LocObj[Conversions.ToInteger(self.locnr)].Name) > 1)
      {
        name: String = self.game.Data.LocObj[Conversions.ToInteger(self.locnr)].Name;
        DrawMod.DrawText( Expression, name, Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), 0, 0);
        tstring: String = "(" + self.game.Data.LocTypeObj[type].Name + ")";
        DrawMod.DrawText( Expression, tstring, Font::new("Times New Roman", 13f, FontStyle.Bold, GraphicsUnit.Pixel), 0, 18);
      }
      else
      {
        name: String = self.game.Data.LocTypeObj[type].Name;
        DrawMod.DrawText( Expression, name, Font::new("Times New Roman", 16f, FontStyle.Bold, GraphicsUnit.Pixel), 0, 0);
      }
      if (self.game.Data.LocObj[Conversions.ToInteger(self.locnr)].HQ == -1)
      {
        tstring: String = "No HQ assigned!";
        DrawMod.DrawText( Expression, tstring, Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 0, 70);
      }
      else
      {
        tstring: String = "HQ: " + self.game.Data.UnitObj[self.game.Data.LocObj[Conversions.ToInteger(self.locnr)].HQ].Name;
        DrawMod.DrawText( Expression, tstring, Font::new("Times New Roman", 13f, FontStyle.Regular, GraphicsUnit.Pixel), 0, 68);
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }

    pub fn Click(x: i32, y: i32, let mut b: i32 =  1) -> i32
    {
      if (self.mzcount <= -1)
        return -1;
      let mut mzcount: i32 =  self.mzcount;
      for (let mut index: i32 =  0; index <= mzcount; index += 1)
      {
        if (x > self.mzx[index] & y > self.mzy[index] & x < self.mzx[index] + 31 & y < self.mzy[index] + 31)
          return self.mznr[index];
      }
      return -1;
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
