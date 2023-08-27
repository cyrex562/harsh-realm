// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SEUnitBigButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SEUnitBigButtonPartClass : SubPartClass
  {
     description: String;
     bool active;
     unr: i32;

    pub SEUnitBigButtonPartClass(tUnr: i32, tDescript: String, bool tactive)
      : base(93, 97)
    {
      self.Descript = tDescript;
      self.active = tactive;
      self.unr = tUnr;
    }

    pub Paint: Bitmap()
    {
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      if (!self.active)
      {
         let mut local1: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_UNITBED);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
      }
      if (self.active)
      {
         let mut local3: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_UNITBEDHIGH);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local3,  local4, 0, 0);
      }
      DrawMod.TGame.CustomBitmapObj.DrawUnitBig(self.unr, toG: graphics, tx: 10, ty: 11);
      if (!Information.IsNothing( graphics))
        graphics.Dispose();
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      bitmap: Bitmap;
      if (!self.active)
      {
         let mut local1: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_UNITBED);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local1,  local2, 0, 0);
         let mut local3: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_UNITBEDHIGH);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.Draw( local3,  local4, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
      }
      if (self.active)
      {
         let mut local5: &Graphics = &graphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SE1_UNITBEDHIGH);
         let mut local6: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local5,  local6, 0, 0);
      }
      DrawMod.TGame.CustomBitmapObj.DrawUnitBig(self.unr, toG: graphics, tx: 10, ty: 11);
      if (!Information.IsNothing( graphics))
        graphics.Dispose();
      return self.OwnBitmap;
    }
  }
}
