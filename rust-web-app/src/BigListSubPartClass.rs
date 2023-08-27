// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.BigListSubPartClass
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
  pub class BigListSubPartClass : SubPartClass
  {
     int ListSize;
     int ListSelect;
    pub TopItem: i32;
     ListClass ListObj;
     OwnFont: Font;
     ownfont2: Font;
     const let mut ItemSize: i32 =  32;
     int ItemFontOffset;
     int LeftTextOffset;
     int Width;
     int Height;
     game: GameClass;
     Header: String;
     bool HeaderCenter;
     bool Highlight;
     bool ShowPair;
     int ValueWidth;
     bool DoTopAndBottom;
     backbitmap: Bitmap;
     int clickscroll;
     bool MarcStyle;

    pub fn Refresh(ListClass tListObj, int tlistselect, theader: String = "")
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

    pub fn SubDispose()
    {
      if (Information.IsNothing( this.backbitmap))
        return;
      this.backbitmap.Dispose();
      this.backbitmap = (Bitmap) null;
    }

    pub BigListSubPartClass(
      ListClass tListobj,
      int tlistsize,
      int twidth,
      int tlistselect,
      tgame: GameClass,
      bool systemfont = false,
      tHeader: String = "",
      bool tHeaderCenter = true,
      bool tHighlight = true,
      let mut tTop: i32 =  0,
      bool tShowPair = false,
      let mut tValueWidth: i32 =  50,
      bool tdotopandbottom = true,
      ref tbackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      bool tMarcStyle = false)
      : base(twidth, (tlistsize + 3) * 32)
    {
      this.ItemFontOffset = 3;
      this.LeftTextOffset = 5;
      if (!tdotopandbottom)
        this.Resize(twidth, (tlistsize + 1) * 32);
      this.MarcStyle = tMarcStyle;
      if (this.MarcStyle)
      {
        this.ItemFontOffset = 3;
        this.LeftTextOffset = 10;
      }
      this.ShowPair = tShowPair;
      this.ValueWidth = tValueWidth;
      this.DoTopAndBottom = tdotopandbottom;
      if (!Information.IsNothing( tbackbitmap))
      {
        this.backbitmap = new Bitmap(this.OwnBitmap.Width, this.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) this.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height), Rectangle::new(bbx, bby, this.OwnBitmap.Width, this.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
        graphics.Dispose();
      }
      this.Width = twidth;
      this.Height = (tlistsize + 3) * 32;
      if (!this.DoTopAndBottom)
        this.Height = (tlistsize + 1) * 32;
      this.ListSize = tlistsize;
      this.ListSelect = tlistselect;
      this.ListObj = tListobj;
      this.MouseOver = true;
      this.clickscroll = 0;
      this.Highlight = tHighlight;
      this.TopItem = tTop;
      this.HeaderCenter = tHeaderCenter;
      this.game = tgame;
      if (Strings.Len(tHeader) > 0)
        this.Header = tHeader;
      if (tTop == 0)
      {
        this.TopItem = (int) Math.Round( this.ListSelect - Conversion.Int( this.ListSize / 2.0));
        if (this.TopItem < 0)
          this.TopItem = 0;
      }
      if (this.MarcStyle)
        this.OwnFont = this.game.MarcFont5;
      else if (!systemfont)
      {
        this.OwnFont = Font::new(this.game.FontCol.Families[1], 12f, FontStyle.Regular, GraphicsUnit.Pixel);
        this.ownfont2 = Font::new(this.game.FontCol.Families[1], 11f, FontStyle.Regular, GraphicsUnit.Pixel);
      }
      else
      {
        this.OwnFont = Font::new("Courier New", 12f, FontStyle.Regular, GraphicsUnit.Pixel);
        this.ownfont2 = Font::new("Courier New", 11f, FontStyle.Regular, GraphicsUnit.Pixel);
      }
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF = SizeF::new();
      Color.FromArgb(0, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb(0, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Color.FromArgb((int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.ListSize >= this.ListObj.ListCount)
        this.TopItem = 0;
      if (Information.IsNothing( this.backbitmap))
      {
        graphics.Clear(Color.Transparent);
      }
      else
      {
        graphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple(ref graphics, ref this.backbitmap, 0, 0);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      if (!this.DoTopAndBottom)
        DrawMod.DrawBlockGradient2(ref graphics, 0, 0, this.Width, this.Height, this.game.MarcCol1, this.game.MarcCol2);
      let mut num1: i32 =  20;
      if (this.ListSize >= this.ListObj.ListCount)
        num1 = 0;
      let mut num2: i32 =  2;
      let mut num3: i32 =  1;
      if (!this.DoTopAndBottom)
      {
        num2 = 0;
        num3 = 0;
      }
      let mut num4: i32 =  -1;
      let mut topItem: i32 =  this.TopItem;
      let mut num5: i32 =  this.TopItem + this.ListSize + num2;
      for (let mut index: i32 =  topItem; index <= num5; index += 1)
      {
        num4 += 1;
        if (!(num4 == this.ListSize + 2 & this.DoTopAndBottom) && this.ListSelect == index - num3 & this.ListSelect > -1 & this.Highlight)
        {
          if (this.MarcStyle)
            DrawMod.DrawBlockGradient2(ref graphics, 0, 32 * num4, this.Width - num1, 32, Color.FromArgb(175, (int) byte.MaxValue, 200, (int) byte.MaxValue), Color.FromArgb(50, (int) byte.MaxValue, 200, (int) byte.MaxValue));
          else
            DrawMod.DrawBlockGradient2(ref graphics, 0, 32 * num4, this.Width - num1, 32, Color.FromArgb(175, 0, (int) byte.MaxValue, 0), Color.FromArgb(50, 0, (int) byte.MaxValue, 0));
        }
        if (index - num3 <= this.ListObj.ListCount & index - num3 >= 0)
        {
          strArray: Vec<String> = this.ListObj.ListName[index - num3].Split('#');
          DrawMod.DrawTextColouredMarc(ref graphics, strArray[0], this.game.MarcFont7, this.LeftTextOffset, 32 * num4 + this.ItemFontOffset, Color.White);
          DrawMod.DrawTextColouredMarc(ref graphics, strArray[1], this.game.MarcFont13, this.LeftTextOffset, 32 * num4 + this.ItemFontOffset + 16, Color.White);
          DrawMod.DrawTextColouredMarc(ref graphics, strArray[2], this.game.MarcFont8, this.Width - 50, 32 * num4 + this.ItemFontOffset + 8, Color.White);
          if (index - num3 <= this.ListObj.ListCount & index > this.TopItem)
            DrawMod.drawLine(ref graphics, 0, 32 * num4, this.Width, 32 * num4, (int) this.game.MarcCol2.R, (int) this.game.MarcCol2.G, (int) this.game.MarcCol2.B, (int) this.game.MarcCol2.A);
          else
            index = index;
        }
      }
      let mut num6: i32 =  (this.ListSize + 1) * 32;
      float num7 = this.ListObj.ListCount <= 0 ? 1f :  this.ListSize /  this.ListObj.ListCount;
      if ( num7 > 1.0)
        num7 = 1f;
      let mut num8: i32 =  (int) Math.Round( Conversion.Int( num6 * num7));
      float num9 = this.ListObj.ListCount <= 0 ? 0.0f :  this.TopItem /  this.ListObj.ListCount;
      if ( num9 > 1.0)
        num9 = 1f;
      let mut num10: i32 =  (int) Math.Round( Conversion.Int( num6 * num9));
      if (this.DoTopAndBottom)
        num10 += 32;
      if (num6 < 5)
        num6 = 5;
      if (num10 + num8 > num6 + 32)
        num8 = num6 + 32 - num10;
      if (this.ListSize < this.ListObj.ListCount)
      {
        let mut x: i32 =  this.Width - 18;
        let mut y: i32 =  num10 + 3;
        let mut width: i32 =  16;
        let mut num11: i32 =  num8 - 2;
        if (!this.MarcStyle)
        {
          ref Graphics local1 = ref graphics;
          bitmap1: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
          ref local2: Bitmap = ref bitmap1;
          Rectangle rectangle1 = Rectangle::new(0, 8, 28, 12);
          let mut srcrect1: &Rectangle = &rectangle1
          Rectangle rectangle2 = Rectangle::new(x, y + 8, width, num11 - 16);
          let mut destrect1: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2(ref local1, ref local2, srcrect1, destrect1);
          ref Graphics local3 = ref graphics;
          bitmap2: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
          ref local4: Bitmap = ref bitmap2;
          rectangle2 = Rectangle::new(0, 0, 28, 8);
          let mut srcrect2: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y, width, 8);
          let mut destrect2: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local3, ref local4, srcrect2, destrect2);
          ref Graphics local5 = ref graphics;
          bitmap3: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.SLIDER1);
          ref local6: Bitmap = ref bitmap3;
          rectangle2 = Rectangle::new(0, 28, 28, 8);
          let mut srcrect3: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(x, y + num11 - 8, 28, 8);
          let mut destrect3: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2(ref local5, ref local6, srcrect3, destrect3);
        }
        else
        {
          ref Graphics local7 = ref graphics;
          bitmap4: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local8: Bitmap = ref bitmap4;
          Rectangle rectangle3 = Rectangle::new(0, 2, 20, 6);
          let mut srcrect4: &Rectangle = &rectangle3
          Rectangle rectangle4 = Rectangle::new(x, 2, 20, this.Height - 4);
          let mut destrect4: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2(ref local7, ref local8, srcrect4, destrect4);
          ref Graphics local9 = ref graphics;
          bitmap5: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local10: Bitmap = ref bitmap5;
          rectangle3 = Rectangle::new(0, 0, 20, 2);
          let mut srcrect5: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(x, 0, 20, 2);
          let mut destrect5: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2(ref local9, ref local10, srcrect5, destrect5);
          ref Graphics local11 = ref graphics;
          bitmap5 = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
          ref local12: Bitmap = ref bitmap5;
          rectangle3 = Rectangle::new(0, 8, 20, 2);
          let mut srcrect6: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(x, this.Height - 2, 20, 2);
          let mut destrect6: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2(ref local11, ref local12, srcrect6, destrect6);
          ref Graphics local13 = ref graphics;
          bitmap5 = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
          ref local14: Bitmap = ref bitmap5;
          rectangle3 = Rectangle::new(0, 2, 20, 6);
          let mut srcrect7: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(x, y + 2, width, num11 - 2);
          let mut destrect7: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2(ref local13, ref local14, srcrect7, destrect7);
          ref Graphics local15 = ref graphics;
          bitmap5 = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
          ref local16: Bitmap = ref bitmap5;
          rectangle3 = Rectangle::new(0, 0, 20, 2);
          let mut srcrect8: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(x, y, width, 2);
          let mut destrect8: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2(ref local15, ref local16, srcrect8, destrect8);
          ref Graphics local17 = ref graphics;
          bitmap5 = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
          ref local18: Bitmap = ref bitmap5;
          rectangle3 = Rectangle::new(0, 8, 20, 2);
          let mut srcrect9: &Rectangle = &rectangle3
          rectangle4 = Rectangle::new(x, y + num11 - 2, 10, 2);
          let mut destrect9: &Rectangle = &rectangle4
          DrawMod.DrawSimplePart2(ref local17, ref local18, srcrect9, destrect9);
        }
      }
      if (!this.MarcStyle)
      {
        if (this.DoTopAndBottom)
        {
          if (this.ListSize < this.ListObj.ListCount)
            DrawMod.DrawRectangle(ref graphics, 0, 32, this.Width - 21, this.Height - 64, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
          else
            DrawMod.DrawRectangle(ref graphics, 0, 32, this.Width - 1, this.Height - 64, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
        }
        else if (this.ListSize < this.ListObj.ListCount)
          DrawMod.DrawRectangle(ref graphics, 0, 0, this.Width - 21, this.Height - 1, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
        else
          DrawMod.DrawRectangle(ref graphics, 0, 0, this.Width - 1, this.Height - 1, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue, (int) byte.MaxValue);
      }
      if (this.MarcStyle)
        DrawMod.DrawFrame(ref this.OwnBitmap, ref this.backbitmap, ref graphics, 0, 0, this.Width, this.Height, -1, -1);
      return this.OwnBitmap;
    }

    pub fn ShiftDown()
    {
      this += 1.TopItem;
      this += 1.ListSelect;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (this.ListSelect > this.ListObj.ListCount)
        this.ListSelect = this.ListObj.ListCount;
      if (0 > this.TopItem)
        this.TopItem = 0;
      if (0 <= this.ListSelect)
        return;
      this.ListSelect = 0;
    }

    pub fn ShiftUp()
    {
      --this.TopItem;
      --this.ListSelect;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (this.ListSelect > this.ListObj.ListCount)
        this.ListSelect = this.ListObj.ListCount;
      if (0 > this.TopItem)
        this.TopItem = 0;
      if (0 <= this.ListSelect)
        return;
      this.ListSelect = 0;
    }

    pub int GetSelect() => this.ListObj.ListData[this.ListSelect];

    pub int Click(int x, int y, let mut b: i32 =  1)
    {
      let mut num1: i32 =  y;
      y = (int) Math.Round(Conversion.Int( y / 32.0));
      this.Scroller = true;
      let mut num2: i32 =  0;
      let mut num3: i32 =  2;
      let mut num4: i32 =  1;
      if (!this.DoTopAndBottom)
      {
        num3 = 1;
        num2 = -1;
        num4 = 0;
      }
      let mut num5: i32 =  20;
      if (this.ListSize >= this.ListObj.ListCount)
        num5 = 0;
      if (y > num2 & y < this.ListSize + num3)
      {
        if (x <= this.Width - num5)
        {
          y -= num4;
          y += this.TopItem;
          this.clickscroll = 0;
          if (y > this.ListObj.ListCount)
            return -1;
          this.ListSelect = y;
          return this.ListObj.ListData[this.ListSelect];
        }
        this.clickscroll = 1;
        let mut num6: i32 =  (this.ListSize + 1) * 32;
        if (this.DoTopAndBottom)
          num1 -= 32;
        if (num1 < 1)
          num1 = 1;
        let mut num7: i32 =  (int) Math.Round( (int) Math.Round( ( num1 /  num6 *  this.ListObj.ListCount)) -  this.ListSize / 2.0);
        if (0 > num7)
          num7 = 0;
        this.TopItem = num7;
        if (this.TopItem > this.ListObj.ListCount - this.ListSize)
          this.TopItem = this.ListObj.ListCount - this.ListSize;
        if (0 > this.TopItem)
          this.TopItem = 0;
        return -1;
      }
      if (y == 0 & this.DoTopAndBottom)
      {
        --this.TopItem;
        this.clickscroll = 0;
        if (0 > this.TopItem)
          this.TopItem = 0;
        return -1;
      }
      if (!(y == this.ListSize + 2 & this.DoTopAndBottom))
        return -1;
      this += 1.TopItem;
      this.clickscroll = 0;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (0 > this.TopItem)
        this.TopItem = 0;
      return -1;
    }

    pub int HandleMouseUp(int x, int y)
    {
      if (!(this.clickscroll == 1 | this.Scroller))
        return -1;
      this.clickscroll = 0;
      this.Scroller = false;
      return 1;
    }

    pub bool MouseMove(int x, int y)
    {
      let mut num1: i32 =  y;
      y = (int) Math.Round(Conversion.Int( y / 32.0));
      let mut num2: i32 =  0;
      let mut num3: i32 =  2;
      let mut num4: i32 =  1;
      if (!this.DoTopAndBottom)
      {
        num3 = 1;
        num2 = -1;
        num4 = 0;
      }
      let mut num5: i32 =  20;
      if (this.ListSize >= this.ListObj.ListCount)
        num5 = 0;
      if (!(y > num2 & y < this.ListSize + num3 & this.clickscroll == 1))
        return false;
      let mut num6: i32 =  (this.ListSize + 1) * 32;
      if (this.DoTopAndBottom)
        num1 -= 32;
      if (num1 < 1)
        num1 = 1;
      let mut num7: i32 =  (int) Math.Round( (int) Math.Round( ( num1 /  num6 *  this.ListObj.ListCount)) -  this.ListSize / 2.0);
      if (0 > num7)
        num7 = 0;
      this.TopItem = num7;
      if (this.TopItem > this.ListObj.ListCount - this.ListSize)
        this.TopItem = this.ListObj.ListCount - this.ListSize;
      if (0 > this.TopItem)
        this.TopItem = 0;
      return true;
    }
  }
}
