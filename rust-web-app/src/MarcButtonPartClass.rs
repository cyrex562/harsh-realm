// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MarcButtonPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class MarcButtonPartClass : SubPartClass
  {
     OwnBitmapNr: i32;
     colorized: i32;
     backbitmap: Bitmap;
     otherback: i32;

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub MarcButtonPartClass(
      tbmpnr: i32,
      let mut tcolorized: i32 =  0,
      tDescript: String = "",
       tBackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      let mut totherback: i32 =  0,
      let mut tsize: i32 =  35)
      : base(tsize, tsize)
    {
      this.OwnBitmapNr = tbmpnr;
      this.colorized = tcolorized;
      this.Descript = tDescript;
      this.otherback = totherback;
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
      Graphics objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        objGraphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( objGraphics,  this.backbitmap, 0, 0);
        objGraphics.CompositingMode = CompositingMode.SourceOver;
      }
      if (this.colorized == 0)
      {
        if (this.otherback == 0)
        {
           let mut local1: &Graphics = &objGraphics;
          bitmap1: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1);
           let mut local2: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local1,  local2, 0, 0);
           let mut local3: &Graphics = &objGraphics;
          bitmap2: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local4: &Bitmap = &bitmap2;
          DrawMod.DrawSimple( local3,  local4, 2, 2);
        }
        else if (this.otherback == 1)
        {
           let mut local5: &Graphics = &objGraphics;
          bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK1);
           let mut local6: &Bitmap = &bitmap3;
          DrawMod.DrawSimple( local5,  local6, 0, 0);
           let mut local7: &Graphics = &objGraphics;
          bitmap4: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local8: &Bitmap = &bitmap4;
          DrawMod.DrawSimple( local7,  local8, 0, 0);
        }
        else if (this.otherback == 2)
        {
           let mut local9: &Graphics = &objGraphics;
          bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK2);
           let mut local10: &Bitmap = &bitmap5;
          DrawMod.DrawSimple( local9,  local10, 0, 0);
           let mut local11: &Graphics = &objGraphics;
          bitmap6: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local12: &Bitmap = &bitmap6;
          DrawMod.DrawSimple( local11,  local12, 0, 0);
        }
        else if (this.otherback == 3)
        {
           let mut local13: &Graphics = &objGraphics;
          bitmap7: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK3);
           let mut local14: &Bitmap = &bitmap7;
          DrawMod.DrawSimple( local13,  local14, 0, 0);
           let mut local15: &Graphics = &objGraphics;
          bitmap8: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local16: &Bitmap = &bitmap8;
          DrawMod.DrawSimple( local15,  local16, 0, 0);
        }
        else if (this.otherback == 4)
        {
           let mut local17: &Graphics = &objGraphics;
          bitmap9: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK4);
           let mut local18: &Bitmap = &bitmap9;
          DrawMod.DrawSimple( local17,  local18, 0, 0);
           let mut local19: &Graphics = &objGraphics;
          bitmap10: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local20: &Bitmap = &bitmap10;
          DrawMod.DrawSimple( local19,  local20, 0, 0);
        }
      }
      else if (this.colorized == 1)
      {
        if (this.otherback == 0)
        {
           let mut local21: &Graphics = &objGraphics;
          bitmap11: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1);
           let mut local22: &Bitmap = &bitmap11;
          DrawMod.Draw( local21,  local22, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local23: &Graphics = &objGraphics;
          bitmap12: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local24: &Bitmap = &bitmap12;
          DrawMod.DrawGray( local23,  local24, 2, 2);
        }
        else if (this.otherback == 1)
        {
           let mut local25: &Graphics = &objGraphics;
          bitmap13: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK1);
           let mut local26: &Bitmap = &bitmap13;
          DrawMod.Draw( local25,  local26, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local27: &Graphics = &objGraphics;
          bitmap14: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local28: &Bitmap = &bitmap14;
          DrawMod.DrawGray( local27,  local28, 0, 0);
        }
        else if (this.otherback == 2)
        {
           let mut local29: &Graphics = &objGraphics;
          bitmap15: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK2);
           let mut local30: &Bitmap = &bitmap15;
          DrawMod.Draw( local29,  local30, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local31: &Graphics = &objGraphics;
          bitmap16: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local32: &Bitmap = &bitmap16;
          DrawMod.DrawGray( local31,  local32, 0, 0);
        }
        else if (this.otherback == 3)
        {
           let mut local33: &Graphics = &objGraphics;
          bitmap17: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK3);
           let mut local34: &Bitmap = &bitmap17;
          DrawMod.Draw( local33,  local34, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local35: &Graphics = &objGraphics;
          bitmap18: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local36: &Bitmap = &bitmap18;
          DrawMod.DrawGray( local35,  local36, 0, 0);
        }
        else if (this.otherback == 4)
        {
           let mut local37: &Graphics = &objGraphics;
          bitmap19: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK4);
           let mut local38: &Bitmap = &bitmap19;
          DrawMod.Draw( local37,  local38, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local39: &Graphics = &objGraphics;
          bitmap20: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local40: &Bitmap = &bitmap20;
          DrawMod.DrawGray( local39,  local40, 0, 0);
        }
      }
      if (!Information.IsNothing( objGraphics))
      {
        objGraphics.Dispose();
        objGraphics = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub PaintOverlay: Bitmap()
    {
      Graphics objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        objGraphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( objGraphics,  this.backbitmap, 0, 0);
        objGraphics.CompositingMode = CompositingMode.SourceOver;
      }
      if (this.colorized == 0)
      {
        if (this.otherback == 0)
        {
           let mut local1: &Graphics = &objGraphics;
          bitmap1: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1b);
           let mut local2: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local1,  local2, 0, 0);
           let mut local3: &Graphics = &objGraphics;
          bitmap2: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local4: &Bitmap = &bitmap2;
          DrawMod.DrawSimple( local3,  local4, 2, 2);
        }
        else if (this.otherback == 1)
        {
           let mut local5: &Graphics = &objGraphics;
          bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK1B);
           let mut local6: &Bitmap = &bitmap3;
          DrawMod.DrawSimple( local5,  local6, 0, 0);
           let mut local7: &Graphics = &objGraphics;
          bitmap4: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local8: &Bitmap = &bitmap4;
          DrawMod.DrawSimple( local7,  local8, 0, 0);
        }
        else if (this.otherback == 2)
        {
           let mut local9: &Graphics = &objGraphics;
          bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK2B);
           let mut local10: &Bitmap = &bitmap5;
          DrawMod.DrawSimple( local9,  local10, 0, 0);
           let mut local11: &Graphics = &objGraphics;
          bitmap6: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local12: &Bitmap = &bitmap6;
          DrawMod.DrawSimple( local11,  local12, 0, 0);
        }
        else if (this.otherback == 3)
        {
           let mut local13: &Graphics = &objGraphics;
          bitmap7: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK3B);
           let mut local14: &Bitmap = &bitmap7;
          DrawMod.DrawSimple( local13,  local14, 0, 0);
           let mut local15: &Graphics = &objGraphics;
          bitmap8: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local16: &Bitmap = &bitmap8;
          DrawMod.DrawSimple( local15,  local16, 0, 0);
        }
        else if (this.otherback == 4)
        {
           let mut local17: &Graphics = &objGraphics;
          bitmap9: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK4B);
           let mut local18: &Bitmap = &bitmap9;
          DrawMod.DrawSimple( local17,  local18, 0, 0);
           let mut local19: &Graphics = &objGraphics;
          bitmap10: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local20: &Bitmap = &bitmap10;
          DrawMod.DrawSimple( local19,  local20, 0, 0);
        }
      }
      else if (this.colorized == 1)
      {
        if (this.otherback == 0)
        {
           let mut local21: &Graphics = &objGraphics;
          bitmap11: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.BUTTONMARC1b);
           let mut local22: &Bitmap = &bitmap11;
          DrawMod.Draw( local21,  local22, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local23: &Graphics = &objGraphics;
          bitmap12: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local24: &Bitmap = &bitmap12;
          DrawMod.Draw( local23,  local24, 2, 2, 0.0f, 0.0f, 0.0f, 0.4f);
        }
        else if (this.otherback == 1)
        {
           let mut local25: &Graphics = &objGraphics;
          bitmap13: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK1B);
           let mut local26: &Bitmap = &bitmap13;
          DrawMod.Draw( local25,  local26, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local27: &Graphics = &objGraphics;
          bitmap14: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local28: &Bitmap = &bitmap14;
          DrawMod.Draw( local27,  local28, 0, 0, 0.0f, 0.0f, 0.0f, 0.4f);
        }
        else if (this.otherback == 2)
        {
           let mut local29: &Graphics = &objGraphics;
          bitmap15: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK2B);
           let mut local30: &Bitmap = &bitmap15;
          DrawMod.Draw( local29,  local30, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local31: &Graphics = &objGraphics;
          bitmap16: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local32: &Bitmap = &bitmap16;
          DrawMod.Draw( local31,  local32, 0, 0, 0.0f, 0.0f, 0.0f, 0.4f);
        }
        else if (this.otherback == 3)
        {
           let mut local33: &Graphics = &objGraphics;
          bitmap17: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK3B);
           let mut local34: &Bitmap = &bitmap17;
          DrawMod.Draw( local33,  local34, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local35: &Graphics = &objGraphics;
          bitmap18: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local36: &Bitmap = &bitmap18;
          DrawMod.Draw( local35,  local36, 0, 0, 0.0f, 0.0f, 0.0f, 0.4f);
        }
        else if (this.otherback == 4)
        {
           let mut local37: &Graphics = &objGraphics;
          bitmap19: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.MARCBACK4B);
           let mut local38: &Bitmap = &bitmap19;
          DrawMod.Draw( local37,  local38, 0, 0, 0.0f, 0.0f, 0.0f, 0.2f);
           let mut local39: &Graphics = &objGraphics;
          bitmap20: Bitmap = BitmapStore.GetBitmap(this.OwnBitmapNr);
           let mut local40: &Bitmap = &bitmap20;
          DrawMod.Draw( local39,  local40, 0, 0, 0.0f, 0.0f, 0.0f, 0.4f);
        }
      }
      if (!Information.IsNothing( objGraphics))
      {
        objGraphics.Dispose();
        objGraphics = (Graphics) null;
      }
      return this.OwnBitmap;
    }
  }
}
