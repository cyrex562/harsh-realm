// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TextPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TextPartClass : SubPartClass
  {
     OwnFont: Font;
     OwnText: String;
     bool CenterIt;
     OColor: i32;
     bool BlackBack;
     bool Outline;
     progress: i32;
     bool Marc;

    pub TextPartClass(
      txt: String,
      f: Font,
      w: i32,
      h: i32,
      bool tcenterit,
      let mut tcolor: i32 = -1,
      tDescript: String = "",
      bool tBlackBack = false,
      bool toutline = false,
      let mut tProgress: i32 = -1,
      bool tMarc = false)
      : base(w, h)
    {
      self.OwnFont = f;
      self.Marc = tMarc;
      self.CenterIt = tcenterit;
      self.OwnText = txt;
      self.OColor = tcolor;
      self.Descript = tDescript;
      self.BlackBack = tBlackBack;
      self.Outline = toutline;
      self.progress = tProgress;
      self.oldStyle = true;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      x: i32;
      y: i32;
      if (self.CenterIt)
      {
        SizeF sizeF2 = Expression.MeasureString(self.OwnText, self.OwnFont);
        if ( sizeF2.Width <=  self.OwnBitmap.Width)
        {
          x =  Math.Round( Conversion.Int( (( self.OwnBitmap.Width -  sizeF2.Width) / 2.0)));
          y =  Math.Round( Conversion.Int( (( self.OwnBitmap.Height -  sizeF2.Height) / 2.0)));
        }
        sizeF1 = SizeF::new();
      }
      if (self.BlackBack)
      {
        if (!self.Marc)
        {
          DrawMod.DrawBlock( Expression, 0, 0, self.OwnBitmap.Width - 1, self.OwnBitmap.Height - 1, 0, 0, 0, 175);
          DrawMod.drawLine( Expression, 2, self.OwnBitmap.Height - 1, self.OwnBitmap.Width - 1, self.OwnBitmap.Height - 1, 200, 220, 120,  byte.MaxValue);
          DrawMod.drawLine( Expression, self.OwnBitmap.Width - 1, 3, self.OwnBitmap.Width - 1, self.OwnBitmap.Height - 1, 200, 220, 120,  byte.MaxValue);
          if (self.progress > -1)
            DrawMod.DrawBlock( Expression, 0, 0,  Math.Round( self.OwnBitmap.Width * ( self.progress / 100.0)), self.OwnBitmap.Height - 1,  byte.MaxValue, 0, 0, 105);
        }
        else
        {
          if (self.progress > -1)
            DrawMod.DrawBlock( Expression, 5, 5,  Math.Round( (self.OwnBitmap.Width - 10) * ( self.progress / 100.0)), self.OwnBitmap.Height - 10 - 1,  byte.MaxValue, 0, 0, 105);
          DrawMod.DrawFrame( self.OwnBitmap,  self.OwnBitmap,  Expression, 0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height, -1, -1);
        }
      }
      if (!self.Marc)
      {
        if (!self.Outline)
        {
          if (self.OColor == -1)
            DrawMod.DrawText( Expression, self.OwnText, self.OwnFont, x, y);
          else if (self.OColor == 1)
            DrawMod.DrawTextColoured( Expression, self.OwnText, self.OwnFont, x, y, Color.FromArgb( byte.MaxValue,  byte.MaxValue, 0, 0));
          else if (self.OColor == 2)
            DrawMod.DrawTextColoured( Expression, self.OwnText, self.OwnFont, x, y, Color.FromArgb( byte.MaxValue, 0, 0, 0));
          else if (self.OColor == 0)
            DrawMod.DrawTextColoured( Expression, self.OwnText, self.OwnFont, x, y, Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
        }
        else
          DrawMod.DrawTextOutline( Expression, self.OwnText, self.OwnFont, x, y);
      }
      else
        DrawMod.DrawTextColouredMarc( Expression, self.OwnText, self.OwnFont, x, y, Color.White);
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return self.OwnBitmap;
    }
  }
}
