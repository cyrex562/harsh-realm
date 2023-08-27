// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class SimpleButtonPartClass : SubPartClass
  {
     OwnBitmapNr: i32;
     backbitmap: Bitmap;

    pub fn SubDispose()
    {
      if (Information.IsNothing( self.backbitmap))
        return;
      self.backbitmap.Dispose();
      self.backbitmap = (Bitmap) null;
    }

    pub SimpleButtonPartClass(
      tbmpnr: i32,
      tDescript: String,
       tBackbitmap: Bitmap,
      bbx: i32,
      bby: i32)
      : base(BitmapStore.GetWidth(tbmpnr), BitmapStore.Getheight(tbmpnr))
    {
      self.OwnBitmapNr = tbmpnr;
      self.Descript = tDescript;
      if (Information.IsNothing( tBackbitmap))
        return;
      self.backbitmap = new Bitmap(self.OwnBitmap.Width, self.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
      self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) self.backbitmap);
      Expression.CompositingMode = CompositingMode.SourceCopy;
      Expression.DrawImage((Image) tBackbitmap, Rectangle::new(0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height), Rectangle::new(bbx, bby, self.OwnBitmap.Width, self.OwnBitmap.Height), GraphicsUnit.Pixel);
      Expression.CompositingMode = CompositingMode.SourceOver;
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub Paint: Bitmap()
    {
      Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
      if (!Information.IsNothing( self.backbitmap))
      {
        objGraphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( objGraphics,  self.backbitmap, 0, 0);
        objGraphics.CompositingMode = CompositingMode.SourceOver;
      }
       let mut local1: &Graphics = &objGraphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(self.OwnBitmapNr);
       let mut local2: &Bitmap = &bitmap;
      DrawMod.DrawSimple( local1,  local2, 0, 0);
      if (!Information.IsNothing( objGraphics))
      {
        objGraphics.Dispose();
        objGraphics = (Graphics) null;
      }
      return self.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
      if (!Information.IsNothing( self.backbitmap))
      {
        objGraphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( objGraphics,  self.backbitmap, 0, 0);
        objGraphics.CompositingMode = CompositingMode.SourceOver;
      }
       let mut local1: &Graphics = &objGraphics;
      bitmap: Bitmap = BitmapStore.GetBitmap(self.OwnBitmapNr);
       let mut local2: &Bitmap = &bitmap;
      DrawMod.Draw( local1,  local2, 0, 0, 0.3f, 0.3f, 0.3f, 1f);
      if (!Information.IsNothing( objGraphics))
      {
        objGraphics.Dispose();
        objGraphics = (Graphics) null;
      }
      return self.OwnBitmap;
    }
  }
}
