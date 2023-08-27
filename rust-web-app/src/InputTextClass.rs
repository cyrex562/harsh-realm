// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.InputTextClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class InputTextClass : SubPartClass
  {
     OwnFont: Font;
     OwnText: String;
     maxchar: i32;
     bool inactive;
     bool noSpace;
     maxSize: i32;

    pub InputTextClass(
      txt: String,
      f: Font,
      w: i32,
      h: i32,
      bool tinactive,
      tmaxchar: i32,
      bool tnospace = false,
      let mut tmaxsize: i32 =  50)
      : base(w, h)
    {
      this.OwnFont = f;
      this.inactive = tinactive;
      this.maxchar = tmaxchar;
      this.noSpace = tnospace;
      txt = txt.Replace('\b'.ToString(), "");
      this.OwnText = txt;
      this.maxSize = tmaxsize;
      if (this.OwnText.Length <= this.maxSize)
        return;
      this.OwnText = Strings.Left(this.OwnText, this.maxSize);
    }

    pub fn Refresh(s: String)
    {
      let mut length: i32 =  s.Length;
      for (let mut Start: i32 =  1; Start <= length; Start += 1)
      {
        char ch = Conversions.ToChar(Strings.Mid(s, Start, 1));
        if (this.OwnText.Length < this.maxchar | ch == '\b')
        {
          if (ch == '\b' & this.OwnText.Length > 0)
            this.OwnText = Strings.Mid(this.OwnText, 1, Strings.Len(this.OwnText) - 1);
          else if (ch != '\b')
            this.OwnText += ch.ToString();
        }
      }
      if (this.OwnText.Length <= this.maxSize)
        return;
      this.OwnText = Strings.Left(this.OwnText, this.maxSize);
    }

    pub GetText: String()
    {
      if (this.OwnText.Length > this.maxSize)
        this.OwnText = Strings.Left(this.OwnText, this.maxSize);
      this.OwnText = this.OwnText.Replace('\b'.ToString(), "");
      return this.OwnText;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF = SizeF::new();
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.inactive)
        DrawMod.Clear( Expression, 80, 80, 80);
      else
        DrawMod.Clear( Expression, 0, 0, 0);
      DrawMod.DrawRectangle( Expression, 0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height, 200, 200, 200, 200, 2);
      x: i32;
      y: i32;
      if (Operators.CompareString(this.Descript, "select", false) == 0)
      {
        DrawMod.DrawRectangle( Expression, 2, 2, this.OwnBitmap.Width - 4, this.OwnBitmap.Height - 4,  byte.MaxValue, 0, 0, 200, 2);
        DrawMod.DrawTextColouredMarc( Expression, this.OwnText + "|", this.OwnFont, x, y, Color.White);
      }
      else if (this.inactive)
        DrawMod.DrawTextColouredMarc( Expression, this.OwnText, this.OwnFont, x, y, Color.LightGray);
      else
        DrawMod.DrawTextColouredMarc( Expression, this.OwnText, this.OwnFont, x, y, Color.White);
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return this.OwnBitmap;
    }
  }
}
