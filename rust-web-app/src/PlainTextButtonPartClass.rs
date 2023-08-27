// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PlainTextButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class PlainTextButtonPartClass : SubPartClass
  {
     Rectangle useRect;
     useCol: Color;
     useColHigh: Color;
     backbitmap: Bitmap;
     butText: String;

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub PlainTextButtonPartClass(
      Rectangle trect,
      tTexty: String,
      tUseCol: Color,
      tUseColHigh: Color,
      tDescript: String = "",
       tBackbitmap: Bitmap = null,
      let mut bbx: i32 = -1,
      let mut bby: i32 = -1)
      : base(trect.Width, trect.Height)
    {
      this.Descript = tDescript;
      this.useRect = trect;
      this.useCol = tUseCol;
      this.butText = tTexty;
      this.useColHigh = tUseColHigh;
      if (Information.IsNothing( tBackbitmap))
        return;
      this.backbitmap = new Bitmap(this.OwnBitmap.Width, this.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
      this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) this.backbitmap);
      Expression.CompositingMode = CompositingMode.SourceCopy;
      Expression.DrawImage((Image) tBackbitmap, Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height), Rectangle::new(bbx, bby, this.OwnBitmap.Width, this.OwnBitmap.Height), GraphicsUnit.Pixel);
      Expression.CompositingMode = CompositingMode.SourceOver;
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub Paint: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  this.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      DrawMod.DrawTextColouredMarcCenter( Expression, this.butText, DrawMod.TGame.MarcFont8c,  Math.Round( this.useRect.Width / 2.0), 2, this.useCol);
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  this.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      DrawMod.DrawTextColouredMarcCenter( Expression, this.butText, DrawMod.TGame.MarcFont8b,  Math.Round( this.useRect.Width / 2.0), 2, this.useColHigh);
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return this.OwnBitmap;
    }
  }
}
