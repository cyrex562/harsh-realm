// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MatrixSubPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class MatrixSubPartClass : SubPartClass
  {
     ListSize: i32;
     ListSelect: i32;
     ColSelect: i32;
    pub TopItemX: i32;
    pub TopItemY: i32;
     StringListClass ListObj;
     OwnFont: Font;
     ownfont2: Font;
     ItemSize: i32;
     const let mut ItemFontOffset: i32 =  1;
     const let mut LeftTextOffset: i32 =  5;
     Width: i32;
     Height: i32;
     game: GameClass;
     bool Highlight;
     bx: i32;
     by: i32;
     backbitmap: Bitmap;
     clickscroll: i32;
     rowheight: i32;
     fontsize: i32;
     fontoffsety: i32;
     bool nolines;
     bool Marcy;
     colWidth: i32;
     minimumColWidth: i32;
     totalColWidth: i32;
     LibSlot: i32;
     twoColumnVariant: i32;

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub fn ShiftUp()
    {
      --this.ListSelect;
      if (this.ListSelect - this.TopItemY >= 0)
        return;
      --this.TopItemY;
    }

    pub fn ShiftDown()
    {
      this += 1.ListSelect;
      if (this.ListSelect - this.TopItemY <= this.ListSize)
        return;
      this += 1.TopItemY;
    }

    pub fn ShiftLeft()
    {
      --this.ColSelect;
      if (this.ColSelect - this.TopItemX >= 0)
        return;
      --this.TopItemX;
    }

    pub fn ShiftRight()
    {
      let mut num1: i32 =   Math.Round(Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1)));
      if (num1 < this.colWidth)
        num1 = this.colWidth;
      let mut num2: i32 =   Math.Round( (this.Width - 20) /  num1) - 2;
      if (num2 < 0)
        num2 = 0;
      this += 1.ColSelect;
      if (this.ColSelect - this.TopItemX <= num2)
        return;
      this += 1.TopItemX;
    }

    pub fn Refresh(StringListClass tListObj, tlistselect: i32, tcolselect: i32)
    {
      this.ListObj = tListObj;
      this.ListSelect = tlistselect;
      this.ColSelect = tcolselect;
      if (this.TopItemY > this.ListObj.Length - this.ListSize)
        this.TopItemY = this.ListObj.Length - this.ListSize;
      if (0 > this.TopItemY)
        this.TopItemY = 0;
      this.Clear();
    }

    pub fn DescriptInfo(x: i32, y: i32)
    {
      if (this.game.Data.Round < 1)
        return;
      if (x > 0 & y > 0 & x < this.Width & y < this.Height)
      {
        Coordinate coordinate = this.Click2(x, y, 0);
        if (coordinate.x >= 0 & coordinate.y >= 0 & coordinate.y <= this.ListObj.TempDesc.GetUpperBound(1) & coordinate.x <= this.ListObj.TempDesc.GetUpperBound(0))
          this.Descript = this.ListObj.TempDesc[coordinate.x, coordinate.y];
        else
          this.Descript = "";
      }
      else
        this.Descript = "";
    }

    pub MatrixSubPartClass(
      StringListClass tListobj,
      tlistsize: i32,
      twidth: i32,
      tlistselect: i32,
      tcolselect: i32,
      tgame: GameClass,
      bool systemfont = false,
      bool tHighlight = true,
      let mut tTop: i32 =  0,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      let mut trowheight: i32 =  16,
      let mut tfontsize: i32 =  12,
      let mut tfontoffsety: i32 =  0,
      bool tnolines = false,
      bool tMarcy = false,
      let mut tMinColValue: i32 =  -1,
      let mut tTwoColumnVariant: i32 =  0,
      let mut tLibSlot: i32 =  -1)
      : base(twidth, (tlistsize + 3) * trowheight)
    {
      this.LibSlot = -1;
      this.nolines = tnolines;
      if (!Information.IsNothing( tbackbitmap))
      {
        if (!Information.IsNothing( this.backbitmap))
          this.backbitmap.Dispose();
        this.backbitmap = new Bitmap(this.OwnBitmap.Width, this.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) this.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height), Rectangle::new(bbx, bby, this.OwnBitmap.Width, this.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
        graphics.Dispose();
      }
      this.bx = bbx;
      this.by = bby;
      this.LibSlot = tLibSlot;
      this.Marcy = tMarcy;
      this.ItemSize = trowheight;
      this.fontsize = tfontsize;
      this.fontoffsety = tfontoffsety;
      this.Width = twidth;
      this.Height = (tlistsize + 3) * this.ItemSize;
      this.ListSize = tlistsize;
      this.ListSelect = tlistselect;
      this.ListObj = tListobj;
      this.MouseOver = true;
      this.ColSelect = tcolselect;
      this.clickscroll = 0;
      this.Highlight = tHighlight;
      this.TopItemY = tTop;
      this.twoColumnVariant = tTwoColumnVariant;
      this.game = tgame;
      if (tTop == 0)
      {
        this.TopItemY =  Math.Round( this.ListSelect - Conversion.Int( this.ListSize / 2.0));
        if (this.TopItemY < 0)
          this.TopItemY = 0;
      }
      if ( Math.Round(Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1))) < this.colWidth)
      {
        let mut colWidth: i32 =  this.colWidth;
        if (this.ColSelect > 0)
        {
          this.TopItemX =  Math.Round( this.ColSelect - Conversion.Int( this.ColSelect / 2.0));
          if (this.TopItemX < 0)
            this.TopItemX = 0;
        }
      }
      else
        this.TopItemX = 0;
      if (!systemfont)
      {
        if (this.Marcy)
        {
          if (this.game.Data.Round == 0)
          {
            this.OwnFont = this.game.MarcFont5;
            this.ownfont2 = this.game.MarcFont7;
          }
          else
          {
            this.OwnFont = this.game.MarcFont4;
            this.ownfont2 = this.game.MarcFont4;
          }
        }
        else
        {
          this.OwnFont = Font::new(this.game.FontCol.Families[1],  this.fontsize, FontStyle.Regular, GraphicsUnit.Pixel);
          this.ownfont2 = Font::new(this.game.FontCol.Families[1],   Math.Round( this.fontsize * 0.9), FontStyle.Regular, GraphicsUnit.Pixel);
        }
      }
      else
      {
        this.OwnFont = Font::new("Courier New", 12f, FontStyle.Regular, GraphicsUnit.Pixel);
        this.ownfont2 = Font::new("Courier New", 11f, FontStyle.Regular, GraphicsUnit.Pixel);
      }
      this.minimumColWidth = 70;
      if (tMinColValue > 0)
        this.minimumColWidth = tMinColValue;
      this.colWidth =  Math.Round(Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1)));
      if (this.minimumColWidth > this.colWidth)
      {
        this.colWidth = this.minimumColWidth;
        --this.ListSize;
      }
      this.totalColWidth = this.colWidth * (this.ListObj.Width + 1);
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF1 = SizeF::new();
      Color.FromArgb(0,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb(0,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.ListSize >= this.ListObj.Length)
        this.TopItemY = 0;
      if (Information.IsNothing( this.backbitmap))
      {
        Expression.Clear(Color.Transparent);
      }
      else
      {
        Expression.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple( Expression,  this.backbitmap, 0, 0);
        Expression.CompositingMode = CompositingMode.SourceOver;
      }
      if (this.Marcy)
      {
        DrawMod.DrawBlockGradient2( Expression, 0, 0, this.Width - 1, this.ItemSize, this.game.MarcCol3, this.game.MarcCol2);
        DrawMod.DrawBlockGradient2( Expression, 0, this.ItemSize, this.Width, this.Height - 2 * this.ItemSize, this.game.MarcCol1, this.game.MarcCol2);
      }
      else if (!this.nolines)
        DrawMod.DrawBlock( Expression, 0, this.ItemSize, this.Width, this.Height - 2 * this.ItemSize, 0, 0, 0, 166);
      else
        DrawMod.DrawBlock( Expression, 0, this.ItemSize, this.Width, this.Height - 2 * this.ItemSize, 0, 0, 0, 100);
      let mut num1: i32 =  20;
      if (this.ListSize >= this.ListObj.Length)
        num1 = 0;
      let mut num2: i32 =  2;
      let mut num3: i32 =  1;
      let mut num4: i32 =  -1;
      let mut topItemY: i32 =  this.TopItemY;
      let mut num5: i32 =  this.TopItemY + this.ListSize + num2;
      bitmap: Bitmap;
      for (let mut index1: i32 =  topItemY; index1 <= num5; index1 += 1)
      {
        num4 += 1;
        if (num4 == 0)
        {
          if (this.ListObj.Width > -1)
          {
            let mut num6: i32 =   Math.Round(Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1)));
            if (num6 < this.colWidth)
              num6 = this.colWidth;
            if (this.ListObj.Width > this.ListObj.ColumnName.GetUpperBound(0))
              this.ListObj.ColumnName = (string[]) Utils.CopyArray((Array) this.ListObj.ColumnName, (Array) new string[this.ListObj.Width + 1]);
            let mut width: i32 =  this.ListObj.Width;
            for (let mut index2: i32 =  0; index2 <= width; index2 += 1)
            {
              if (Operators.CompareString(this.ListObj.ColumnName[index2], (string) null, false) == 0)
                this.ListObj.ColumnName[index2] = "";
              let mut x1: i32 =  5 + num6 * index2 - num6 * this.TopItemX;
              if (this.twoColumnVariant > 0 && index2 == 1)
                x1 = this.twoColumnVariant + 5;
              if (x1 > 0 & x1 <= this.Width)
              {
                if (Conversions.ToDouble(this.ListObj.TempColumBmp[index2]) > 0.0)
                {
                   let mut local1: &Graphics = &Expression;
                  bitmap = BitmapStore.GetBitmap(Conversions.ToInteger(this.ListObj.TempColumBmp[index2]));
                   let mut local2: &Bitmap = &bitmap;
                  let mut x2: i32 =  x1;
                  let mut y: i32 =  this.ItemSize * num4 + 1 + 1;
                  DrawMod.DrawSimple( local1,  local2, x2, y);
                }
                else
                {
                  str: String = this.ListObj.ColumnName[index2];
                  if (this.game.Data.Round == 0)
                  {
                    for (SizeF sizeF2 = Expression.MeasureString(str, this.OwnFont);  sizeF2.Width >  num6 & str.Length > 0; sizeF2 = Expression.MeasureString(str, this.OwnFont))
                      str = Strings.Left(str, Strings.Len(str) - 1);
                    if (this.ListObj.ColumnName[index2].Length <= str.Length & this.game.Data.Round == 0)
                    {
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.Number)
                        str += "<num>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.Text)
                        str += "<txt>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.RegimeId)
                        str += "<reg>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.YesNo)
                        str += "<yes/no>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.DateString)
                        str += "<date>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.HistoricalUnitId)
                        str += "<his>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.LandscapeId)
                        str += "<land>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.OfficerId)
                        str += "<com>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.HistoricalUnitModelId)
                        str += "<mod>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.PeopleId)
                        str += "<ppl>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.RiverId)
                        str += "<riv>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.RoadId)
                        str += "<road>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.SFTypeId)
                        str += "<sftyp>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.SmallGfxId)
                        str += "<smgfx>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.EventPicId)
                        str += "<evpic>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.ActionCardId)
                        str += "<acard>";
                      if (this.ListObj.ColValueType[index2] == NewEnums.LibVarValueType.LocationTypeId)
                        str += "<loctyp>";
                    }
                    for (SizeF sizeF3 = Expression.MeasureString(str, this.OwnFont);  sizeF3.Width >  num6 & str.Length > 0; sizeF3 = Expression.MeasureString(str, this.OwnFont))
                      str = Strings.Left(str, Strings.Len(str) - 1);
                  }
                  if (this.Marcy)
                    DrawMod.DrawTextColouredMarc( Expression, str, this.OwnFont, x1, this.ItemSize * num4 + 1 - 1 + this.fontoffsety, Color.White);
                  else
                    DrawMod.DrawText( Expression, str, this.OwnFont, x1, this.ItemSize * num4 + 1 - 1 + this.fontoffsety);
                }
              }
            }
          }
          if (this.TopItemY > 0)
          {
            if (num1 > 0)
            {
               let mut local3: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(this.game.LISTUP);
               let mut local4: &Bitmap = &bitmap;
              let mut x: i32 =  this.Width - 20;
              DrawMod.DrawSimple( local3,  local4, x, 3);
            }
          }
          else if (num1 > 0)
          {
             let mut local5: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(this.game.LISTBLOCK);
             let mut local6: &Bitmap = &bitmap;
            let mut x: i32 =  this.Width - 20;
            DrawMod.DrawSimple( local5,  local6, x, 3);
          }
        }
        else if (num4 == this.ListSize + 2)
        {
          if (this.TopItemY + this.ListSize >= this.ListObj.Length)
          {
            if (num1 > 0)
            {
               let mut local7: &Graphics = &Expression;
              bitmap = BitmapStore.GetBitmap(this.game.LISTBLOCK);
               let mut local8: &Bitmap = &bitmap;
              let mut x: i32 =  this.Width - 20;
              let mut y: i32 =  this.ItemSize * num4 + 3;
              DrawMod.DrawSimple( local7,  local8, x, y);
            }
          }
          else if (num1 > 0)
          {
             let mut local9: &Graphics = &Expression;
            bitmap = BitmapStore.GetBitmap(this.game.LISTDOWN);
             let mut local10: &Bitmap = &bitmap;
            let mut x: i32 =  this.Width - 20;
            let mut y: i32 =  this.ItemSize * num4 + 3;
            DrawMod.DrawSimple( local9,  local10, x, y);
          }
        }
        else if (index1 - num3 <= this.ListObj.Length && this.ListObj.Width > -1)
        {
          if (!this.nolines)
            DrawMod.drawLine( Expression, 0, this.ItemSize * num4 + this.ItemSize, this.Width - 20, this.ItemSize * num4 + this.ItemSize, 128, 128, 128,  byte.MaxValue);
          let mut w1: i32 =   Math.Round(Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1)));
          if (w1 < this.colWidth)
            w1 = this.colWidth;
          let mut width: i32 =  this.ListObj.Width;
          col: i32;
          for (col = 0; col <= width; col += 1)
          {
            let mut x3: i32 =  5 + w1 * col - w1 * this.TopItemX;
            if (this.twoColumnVariant > 0)
            {
              if (col == 0)
                w1 = this.twoColumnVariant;
              if (col == 1)
              {
                x3 = this.twoColumnVariant + 5;
                w1 = this.Width - (20 - this.twoColumnVariant);
              }
            }
            if (x3 > 0 & x3 <= this.Width)
            {
              if (col == 0 & !this.Marcy && this.ListSelect == index1 - num3)
                DrawMod.DrawBlockGradient2( Expression, 0, this.ItemSize * num4, this.Width - num1, this.ItemSize, Color.FromArgb(55, 0,  byte.MaxValue, 0), Color.FromArgb(55, 0,  byte.MaxValue, 0));
              if (this.ListSelect == index1 - num3 & this.ColSelect == col)
              {
                if (!this.nolines)
                {
                  if (this.Marcy)
                    DrawMod.DrawBlockGradient2( Expression, x3 - 5, this.ItemSize * num4, w1, this.ItemSize, Color.FromArgb(165,  byte.MaxValue, 200,  byte.MaxValue), Color.FromArgb(10,  byte.MaxValue, 200,  byte.MaxValue));
                  else
                    DrawMod.DrawBlockGradient2( Expression, x3 - 5, this.ItemSize * num4, w1, this.ItemSize, Color.FromArgb(175, 0,  byte.MaxValue, 0), Color.FromArgb(50, 0,  byte.MaxValue, 0));
                }
              }
              else if (this.game.Data.Product >= 5 && !this.Marcy & this.ColSelect > -1 & this.ColSelect <= this.ListObj.Width & this.ListSelect > -1)
              {
                valueText: String = this.ListObj.GetValue( this.game.Data, this.ListSelect, this.ColSelect, this.LibSlot).valueText;
                if (valueText.Length > 0 & Strings.InStr(this.ListObj.GetValue( this.game.Data, index1 - num3, col).valueText, valueText) > 0)
                  DrawMod.DrawBlockGradient2( Expression, x3 - 5, this.ItemSize * num4, w1, this.ItemSize, Color.FromArgb(135,  byte.MaxValue,  byte.MaxValue, 0), Color.FromArgb(50,  byte.MaxValue,  byte.MaxValue, 0));
              }
              if (this.ListObj.TempBmp[index1 - num3, col] > 0)
              {
                num7: i32;
                num8: i32;
                nr: i32;
                if (this.game.Data.Round == 0)
                {
                  num7 = w1 - 4;
                  num8 = this.ItemSize - 2;
                  nr = this.ListObj.TempBmp[index1 - num3, col];
                  if (BitmapStore.GetWidth(nr) < num7)
                    num7 = BitmapStore.GetWidth(nr);
                  if (BitmapStore.Getheight(nr) < num8)
                    num8 = BitmapStore.Getheight(nr);
                }
                else
                {
                  nr = this.ListObj.TempBmp[index1 - num3, col];
                  num7 = BitmapStore.GetWidth(nr);
                  num8 = BitmapStore.Getheight(nr);
                }
                 let mut local11: &Graphics = &Expression;
                bitmap = BitmapStore.GetBitmap(nr);
                 let mut local12: &Bitmap = &bitmap;
                let mut x4: i32 =  x3;
                let mut y: i32 =  this.ItemSize * num4 + 1 + 1;
                let mut w2: i32 =  num7;
                let mut h: i32 =  num8;
                DrawMod.DrawScaled( local11,  local12, x4, y, w2, h);
              }
              else
              {
                str: String = this.ListObj.GetValue( this.game.Data, index1 - num3, col, this.LibSlot).valueText;
                try
                {
                  while ( Expression.MeasureString(str, this.ownfont2).Width >  w1)
                    str =  Expression.MeasureString(str, this.ownfont2).Width <=  w1 * 2.2 ? Strings.Left(str, str.Length - 1) : Strings.Left(str,  Math.Round( str.Length / 2.0));
                  if (this.Marcy)
                    DrawMod.DrawTextColouredMarc( Expression, str, this.ownfont2, x3, this.ItemSize * num4 + 1 - 1 + this.fontoffsety, Color.White);
                  else
                    DrawMod.DrawText( Expression, str, this.ownfont2, x3, this.ItemSize * num4 + 1 - 1 + this.fontoffsety);
                }
                catch (Exception ex)
                {
                  ProjectData.SetProjectError(ex);
                  ProjectData.ClearProjectError();
                }
              }
              if (!this.nolines & col > 0)
                DrawMod.drawLine( Expression, x3 - 5, this.ItemSize * num4, x3 - 5, 1 * this.ItemSize + this.ItemSize * num4 - 1, 128, 128, 128,  byte.MaxValue);
            }
          }
          if (!this.nolines & col > 0)
            DrawMod.drawLine( Expression, w1 * col, this.ItemSize * num4, w1 * col, 1 * this.ItemSize + this.ItemSize * num4 - 1, 128, 128, 128,  byte.MaxValue);
        }
      }
      let mut num9: i32 =  (this.ListSize + 1) * this.ItemSize;
      float num10 = this.ListObj.Length <= 0 ? 1f :  this.ListSize /  this.ListObj.Length;
      if ( num10 > 1.0)
        num10 = 1f;
      let mut num11: i32 =   Math.Round( Conversion.Int( num9 * num10));
      float num12 = this.ListObj.Length <= 0 ? 0.0f :  this.TopItemY /  this.ListObj.Length;
      if ( num12 > 1.0)
        num12 = 1f;
      let mut num13: i32 =   Math.Round( Conversion.Int( num9 * num12)) + this.ItemSize;
      if (num9 < 5)
        num9 = 5;
      if (num13 + num11 > num9 + this.ItemSize)
        num11 = num9 + this.ItemSize - num13;
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (this.ListSize < this.ListObj.Length)
      {
        let mut x: i32 =  this.Width - 18;
        let mut y: i32 =  num13 + 3;
        let mut width: i32 =  16;
        let mut num14: i32 =  num11 - 2;
         let mut local13: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
         let mut local14: &Bitmap = &bitmap;
        rectangle1 = Rectangle::new(0, 8, 28, 12);
        let mut srcrect1: &Rectangle = &rectangle1
        rectangle2 = Rectangle::new(x, y + 8, width, num14 - 16);
        let mut destrect1: &Rectangle = &rectangle2
        DrawMod.DrawSimplePart2( local13,  local14, srcrect1, destrect1);
         let mut local15: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
         let mut local16: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 0, 28, 8);
        let mut srcrect2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, y, width, 8);
        let mut destrect2: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local15,  local16, srcrect2, destrect2);
         let mut local17: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
         let mut local18: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 28, 28, 8);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, y + num14 - 8, 28, 8);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local17,  local18, srcrect3, destrect3);
      }
      if (this.totalColWidth > this.Width - 20)
      {
        let mut num15: i32 =   Math.Round(Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1)));
        if (num15 < this.colWidth)
          num15 = this.colWidth;
        let mut num16: i32 =   Math.Round( (this.Width - 20) /  num15) - 2;
        if (num16 < 0)
          num16 = 0;
        let mut num17: i32 =  4;
        let mut y: i32 =  this.Height - 2 * this.ItemSize;
        let mut num18: i32 =  this.Width - 20 - 24;
        let mut num19: i32 =   Math.Round( this.Width * ( (this.Width - 20) /  this.totalColWidth));
        let mut num20: i32 =  this.Width - 20 - 24 - num19;
        double num21 =  this.TopItemX /  (this.ListObj.Width - num16) *  num20;
        DrawMod.DrawBlock( Expression, 0, y - 1, this.Width - 20, 24, 0, 0, 0, 128);
        let mut x: i32 =   Math.Round( num17 + num21);
         let mut local19: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.VSLIDER);
         let mut local20: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(8, 0, 8, 22);
        let mut srcrect4: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x + 8, y, num19 + 4, 22);
        let mut destrect4: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local19,  local20, srcrect4, destrect4);
         let mut local21: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.VSLIDER);
         let mut local22: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 0, 8, 22);
        let mut srcrect5: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x, y, 8, 22);
        let mut destrect5: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local21,  local22, srcrect5, destrect5);
         let mut local23: &Graphics = &Expression;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.VSLIDER);
         let mut local24: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(14, 0, 8, 22);
        let mut srcrect6: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x + num19 + 8, y, 8, 22);
        let mut destrect6: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local23,  local24, srcrect6, destrect6);
      }
      if (!this.nolines)
      {
        if (this.ListSize < this.ListObj.Length)
        {
          if (!this.Marcy)
            DrawMod.DrawRectangle( Expression, 0, this.ItemSize, this.Width - 21, this.Height - this.ItemSize * 2,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
        }
        else if (!this.Marcy)
          DrawMod.DrawRectangle( Expression, 0, this.ItemSize, this.Width - 1, this.Height - this.ItemSize * 2,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
      }
      if (this.Marcy)
        DrawMod.DrawFrame( this.OwnBitmap,  this.backbitmap,  Expression, 0, 0, this.Width, this.Height - 1 * this.ItemSize, -1, -1);
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub Coordinate Click2(x: i32, y: i32, let mut b: i32 =  1)
    {
      let mut num1: i32 =  y;
      let mut num2: i32 =  x;
      y =  Math.Round(Conversion.Int( y /  this.ItemSize));
      this.Scroller = true;
      let mut num3: i32 =  1;
      let mut num4: i32 =  20;
      if (this.ListSize >= this.ListObj.Length)
        num4 = 0;
      if (y > 0 & y < this.ListSize + 2)
      {
        if (x <= this.Width - num4)
        {
          y -= num3;
          y += this.TopItemY;
          float num5 =  Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1));
          if ( num5 <  this.colWidth)
            num5 =  this.colWidth;
          this.clickscroll = 0;
          if (y > this.ListObj.Length)
          {
            Coordinate coordinate;
            coordinate.x = -1;
            coordinate.y = -1;
            return coordinate;
          }
          this.ListSelect = y;
          Coordinate coordinate1;
          coordinate1.x = this.ListSelect;
          if (this.twoColumnVariant > 0)
          {
            if (x <= this.twoColumnVariant)
            {
              coordinate1.y = 1;
              this.ColSelect = 1;
            }
            else
            {
              coordinate1.y = 1;
              this.ColSelect = 1;
            }
          }
          else
          {
            coordinate1.y =  Math.Round( (Conversion.Int( x / num5) +  this.TopItemX));
            this.ColSelect =  Math.Round( (Conversion.Int( x / num5) +  this.TopItemX));
          }
          return coordinate1;
        }
        this.clickscroll = 1;
        let mut num6: i32 =  (this.ListSize + 1) * this.ItemSize;
        let mut num7: i32 =  num1 - this.ItemSize;
        if (num7 < 1)
          num7 = 1;
        let mut num8: i32 =   Math.Round(  Math.Round( ( num7 /  num6 *  this.ListObj.Length)) -  this.ListSize / 2.0);
        if (0 > num8)
          num8 = 0;
        this.TopItemY = num8;
        if (this.TopItemY > this.ListObj.Length - this.ListSize)
          this.TopItemY = this.ListObj.Length - this.ListSize;
        if (0 > this.TopItemY)
          this.TopItemY = 0;
        Coordinate coordinate2;
        coordinate2.x = -1;
        coordinate2.y = -1;
        return coordinate2;
      }
      if (y == 0)
      {
        --this.TopItemY;
        this.clickscroll = 0;
        if (0 > this.TopItemY)
          this.TopItemY = 0;
        Coordinate coordinate;
        coordinate.x = -1;
        coordinate.y = -1;
        return coordinate;
      }
      if (y == this.ListSize + 2)
      {
        if (this.totalColWidth > this.Width - 20)
        {
          this.clickscroll = 2;
          if (x > this.Width - 20 + 8)
            x = this.Width - 20 + 8;
          if (4 > x)
            x = 4;
          float num9 =  Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1));
          if ( num9 <  this.colWidth)
            num9 =  this.colWidth;
          let mut num10: i32 =   Math.Round( ( (this.Width - 20) / num9)) - 2;
          if (num10 < 0)
            num10 = 0;
          this.TopItemX =  Math.Round(( this.ListObj.Width + 0.5) * ( num2 /  (this.Width - 20)));
          this.TopItemX =  Math.Round( this.TopItemX -  num10 / 2.0);
          if (0 > this.TopItemX)
            this.TopItemX = 0;
          if (this.TopItemX > this.ListObj.Width - num10)
            this.TopItemX = this.ListObj.Width - num10;
          Coordinate coordinate;
          coordinate.x = -1;
          coordinate.y = -1;
          return coordinate;
        }
        this += 1.TopItemY;
        this.clickscroll = 0;
        if (this.TopItemY > this.ListObj.Length - this.ListSize)
          this.TopItemY = this.ListObj.Length - this.ListSize;
        if (0 > this.TopItemY)
          this.TopItemY = 0;
        Coordinate coordinate3;
        coordinate3.x = -1;
        coordinate3.y = -1;
        return coordinate3;
      }
      Coordinate coordinate4;
      coordinate4.x = -1;
      coordinate4.y = -1;
      return coordinate4;
    }

    pub fn HandleMouseUp(x: i32, y: i32) -> i32
    {
      if (this.game.EditObj.matrixSubPart_BlockMouseUpScroller1time)
      {
        this.game.EditObj.matrixSubPart_BlockMouseUpScroller1time = false;
        this.Scroller = false;
      }
      if (this.clickscroll == 1 | this.Scroller)
      {
        this.clickscroll = 0;
        this.Scroller = false;
        return 1;
      }
      if (this.clickscroll != 2)
        return -1;
      this.clickscroll = 0;
      this.Scroller = false;
      return 1;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      let mut num1: i32 =  y;
      let mut num2: i32 =  x;
      y =  Math.Round(Conversion.Int( y /  this.ItemSize));
      let mut num3: i32 =  0;
      let mut num4: i32 =  2;
      let mut num5: i32 =  20;
      if (this.ListSize >= this.ListObj.Length)
        num5 = 0;
      if (y > num3 & y < this.ListSize + num4 & this.clickscroll == 1)
      {
        let mut num6: i32 =  (this.ListSize + 1) * this.ItemSize;
        let mut num7: i32 =  num1 - this.ItemSize;
        if (num7 < 1)
          num7 = 1;
        let mut num8: i32 =   Math.Round(  Math.Round( ( num7 /  num6 *  this.ListObj.Length)) -  this.ListSize / 2.0);
        if (0 > num8)
          num8 = 0;
        this.TopItemY = num8;
        if (this.TopItemY > this.ListObj.Length - this.ListSize)
          this.TopItemY = this.ListObj.Length - this.ListSize;
        if (0 > this.TopItemY)
          this.TopItemY = 0;
        return true;
      }
      if (this.clickscroll != 2)
        return false;
      if (x > this.Width - 20 + 8)
        x = this.Width - 20 + 8;
      if (4 > x)
        x = 4;
      float num9 =  Conversion.Int( (this.Width - 20) /  (this.ListObj.Width + 1));
      if ( num9 <  this.colWidth)
        num9 =  this.colWidth;
      let mut num10: i32 =   Math.Round( ( (this.Width - 20) / num9)) - 2;
      if (num10 < 0)
        num10 = 0;
      this.TopItemX =  Math.Round(( this.ListObj.Width + 0.5) * ( num2 /  (this.Width - 20)));
      this.TopItemX =  Math.Round( this.TopItemX -  num10 / 2.0);
      if (0 > this.TopItemX)
        this.TopItemX = 0;
      if (this.TopItemX > this.ListObj.Width - num10)
        this.TopItemX = this.ListObj.Width - num10;
      return true;
    }
  }
}
