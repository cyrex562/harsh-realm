// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.PaperAreaClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;

namespace WindowsApplication1
{
  pub class PaperAreaClass : SubPartClass
  {
     ListSize: i32;
     ListSelect: i32;
    pub TopItem: i32;
     ListClass ListObj;
     OwnFont: Font;
     ItemSize: i32;
     const let mut ItemFontOffset: i32 =  1;
     const let mut LeftTextOffset: i32 =  5;
     Width: i32;
     Height: i32;
     Header: String;
     bool HeaderCenter;
     game: GameClass;
     backbitmap: Bitmap;
     fontcol: Color;
     bool centerit;
     clickscroll: i32;
     bool HideShade;
     PaginationLines: i32;
     mzx: i32;
     mzy: i32;
     mzx2: i32;
     mzy2: i32;
     bool mzover;
     bool mzover2;
     bgcolor: i32;

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub fn Refresh(ListClass tListObj, tlistselect: i32, theader: String = "")
    {
      this.ListObj = tListObj;
      this.ListSelect = tlistselect;
      if (Strings.Len(theader) > 0)
        this.Header = theader;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (0 > this.TopItem)
        this.TopItem = 0;
      this.Clear();
    }

    pub PaperAreaClass(
      tgame: GameClass,
      twidth: i32,
      trows: i32,
      tfont: Font,
      theader: String,
      bool theadercenter,
      tText: String,
      tfontcol: Color,
      let mut tTop: i32 =  0,
      let mut tItemSize: i32 =  16,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      bool tcenterit = false,
      bool tHideShade = false,
      let mut tPaginationLines: i32 =  2,
      let mut tbgcolor: i32 =  -1)
      : base(twidth, (trows + 1) * tItemSize)
    {
      this.ItemSize = tItemSize;
      this.Width = twidth;
      this.Height = (trows + 1) * this.ItemSize;
      this.game = tgame;
      this.HideShade = tHideShade;
      this.PaginationLines = tPaginationLines;
      this.centerit = tcenterit;
      this.bgcolor = tbgcolor;
      Graphics objGraphics;
      if (!Information.IsNothing( tbackbitmap))
      {
        this.backbitmap = new Bitmap(this.Width, this.Height);
        this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        objGraphics = Graphics.FromImage((Image) this.backbitmap);
        objGraphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimplePart2( objGraphics,  tbackbitmap, Rectangle::new(bbx, bby, this.Width, this.Height), Rectangle::new(0, 0, this.Width, this.Height));
        objGraphics = (Graphics) null;
      }
      this.fontcol = !Information.IsNothing( tfontcol) ? tfontcol : Color.White;
      SizeF sizeF = SizeF::new();
      this.ListObj = ListClass::new();
      objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      this.OwnFont = !Information.IsNothing( tfont) ? tfont : this.game.VicFont7;
      let mut num1: i32 =  1;
      this.clickscroll = 0;
      if (Information.IsNothing( tText))
        tText = "";
      tText = tText.Replace("\t", " ");
      while (Strings.Len(tText) > 0)
      {
        let mut num2: i32 =  1;
        str1: String = "";
        while (num2 == 1)
        {
          let mut num3: i32 =  Strings.InStr(tText, "\r\n");
          let mut num4: i32 =  Strings.InStr(tText, " ");
          if (num4 == 0)
            num4 = 9999999;
          if (num3 < num4 & num3 > 0)
          {
            let mut num5: i32 =  num3;
            num2 = 0;
            let mut num6: i32 =  0;
            if (num5 != 1)
            {
              if ( objGraphics.MeasureString(str1 + Strings.Left(tText, num5 - 1), this.OwnFont).Width <=  this.Width)
                str1 += Strings.Left(tText, num5 - 1);
              else
                num6 = 1;
            }
            if (num6 == 0)
              tText = num5 >= Strings.Len(tText) ? "" : Strings.Mid(tText, num5 + 2);
          }
          else
          {
            let mut Length: i32 =  Strings.InStr(tText, " ");
            str2: String = Length <= 0 ? tText : Strings.Left(tText, Length);
            let mut num7: i32 =  0;
            num2 = 0;
            if ( objGraphics.MeasureString(str1 + str2, this.OwnFont).Width <=  this.Width)
            {
              num1 = 1;
              num7 = 1;
            }
            else if (num1 == 1)
            {
              num1 = 0;
            }
            else
            {
              num1 = 1;
              num7 = 1;
            }
            if (num7 == 1)
            {
              str1 += str2;
              if (Length > 0)
              {
                if (Strings.Len(tText) >= Length + 1)
                {
                  tText = Strings.Mid(tText, Length + 1);
                  num2 = 1;
                }
                else
                {
                  tText = "";
                  num2 = 0;
                }
              }
              else
              {
                tText = "";
                num2 = 0;
              }
            }
          }
        }
        if (Strings.InStr(str1, "Spaniards") > 0)
          str1 = str1;
        this.ListObj.add(str1, 0);
      }
      this.ListSize = trows;
      this.ListSelect = -1;
      this.TopItem = tTop;
      this.HeaderCenter = theadercenter;
      if (Strings.Len(theader) > 0)
        this.Header = theader;
      if (tTop == 0)
      {
        this.TopItem =  Math.Round( this.ListSelect - Conversion.Int( this.ListSize / 2.0));
        if (this.TopItem < 0)
          this.TopItem = 0;
      }
      this.MouseOver = true;
      if (Information.IsNothing( objGraphics))
        return;
      objGraphics.Dispose();
      objGraphics = (Graphics) null;
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (!Information.IsNothing( this.backbitmap))
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimplePart2( Expression,  this.backbitmap, Rectangle::new(0, 0, this.Width, this.Height), Rectangle::new(0, 0, this.Width, this.Height));
      }
      else if (this.bgcolor == -1)
        Expression.Clear(Color.White);
      Expression.CompositingMode = CompositingMode.SourceOver;
      let mut num1: i32 =  -1;
      let mut topItem: i32 =  this.TopItem;
      let mut num2: i32 =  this.TopItem + this.ListSize - this.PaginationLines;
      SizeF sizeF2;
      for (let mut index: i32 =  topItem; index <= num2; index += 1)
      {
        num1 += 1;
        if (index <= this.ListObj.ListCount)
        {
          if (!this.centerit)
          {
            DrawMod.DrawTextColoured( Expression, this.ListObj.ListName[index], this.OwnFont, 5, this.ItemSize * num1 + 1, this.fontcol);
          }
          else
          {
            sizeF2 = Expression.MeasureString(this.ListObj.ListName[index], this.OwnFont);
            let mut num3: i32 =   Math.Round( this.Width / 2.0 -  sizeF2.Width / 2.0);
            if (0 > num3)
              num3 = 0;
            DrawMod.DrawTextColoured( Expression, this.ListObj.ListName[index], this.OwnFont, num3 + 5, this.ItemSize * num1 + 1, this.fontcol);
          }
        }
      }
      if (this.ListSize - this.PaginationLines < this.ListObj.ListCount)
      {
        let mut Number1: i32 =   Math.Round(Conversion.Int( this.ListObj.ListCount /  (this.ListSize - this.PaginationLines)) + 1.0);
        let mut Number2: i32 =   Math.Round(Conversion.Int( this.TopItem /  (this.ListSize - this.PaginationLines)) + 1.0);
        str: String = "Page " + Strings.Trim(Conversion.Str( Number2)) + " of " + Strings.Trim(Conversion.Str( Number1));
        sizeF2 = Expression.MeasureString(str, this.OwnFont);
        let mut num4: i32 =   Math.Round( this.Width / 2.0 -  sizeF2.Width / 2.0);
        if (0 > num4)
          num4 = 0;
        DrawMod.DrawTextColoured( Expression, str, this.game.VicFont7, 5 + num4, this.Height - this.ItemSize - 15, Color.Black);
        this.mzx = 9999;
        this.mzy = 9999;
        this.mzx2 = 9999;
        this.mzy2 = 9999;
        if (Number2 > 1)
        {
          this.mzx = 5 + num4 - 60;
          this.mzy = this.Height - this.ItemSize - 17;
          DrawMod.DrawButton( Expression, this.mzx, this.mzy, 40, 20, this.mzover, "<");
        }
        if (Number2 < Number1)
        {
          this.mzx2 =  Math.Round( ( (5 + num4 + 20) + sizeF2.Width));
          this.mzy2 = this.Height - this.ItemSize - 17;
          DrawMod.DrawButton( Expression, this.mzx2, this.mzy2, 40, 20, this.mzover2, ">");
        }
      }
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      return this.OwnBitmap;
    }

    pub PaintOverlay: Bitmap() => this.Paint();

    pub fn Click(x: i32, y: i32, let mut b: i32 =  1) -> i32
    {
      if (x > this.mzx & x < this.mzx + 40 & y > this.mzy & y < this.mzy + 21)
      {
        this.TopItem -= this.ListSize - this.PaginationLines;
        if (0 > this.TopItem)
          this.TopItem = 0;
      }
      if (x > this.mzx2 & x < this.mzx2 + 40 & y > this.mzy2 & y < this.mzy2 + 21)
      {
        this.TopItem += this.ListSize - this.PaginationLines;
        if (this.TopItem > this.ListObj.ListCount)
          this.TopItem = this.ListObj.ListCount;
      }
      num: i32;
      return num;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      if (x > this.mzx & x < this.mzx + 40 & y > this.mzy & y < this.mzy + 21)
      {
        this.mzover = true;
        return true;
      }
      if (this.mzover)
      {
        this.mzover = false;
        return true;
      }
      if (x > this.mzx2 & x < this.mzx2 + 40 & y > this.mzy2 & y < this.mzy2 + 21)
      {
        this.mzover2 = true;
        return true;
      }
      if (!this.mzover2)
        return false;
      this.mzover2 = false;
      return true;
    }
  }
}
