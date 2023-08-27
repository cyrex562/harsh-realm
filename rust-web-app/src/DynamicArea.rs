// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.DynamicArea
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
  pub class DynamicArea : SubPartClass
  {
     OwnFont: Font;
     Width: i32;
     Height: i32;
     game: GameClass;
     backbitmap: Bitmap;
     bx: i32;
     by: i32;
     clickscroll: i32;
    pub texty: String;
     curY: i32;
     maxY: i32;
     lastY: i32;
     paper: Bitmap;
     bool alwaysBlockScrollBar;
     bool useActualWidth;

    pub fn SubDispose()
    {
      if (!Information.IsNothing( this.backbitmap))
      {
        this.backbitmap.Dispose();
        this.backbitmap = (Bitmap) null;
      }
      if (Information.IsNothing( this.paper))
        return;
      this.paper.Dispose();
      this.paper = (Bitmap) null;
    }

    pub bool HandleTimerWheel(x: i32, y: i32, ref WindowClass tWindow)
    {
      if (this.game.EditObj.MouseWheel > 0)
      {
        let mut mouseWheel: i32 =  this.game.EditObj.MouseWheel;
        for (let mut index: i32 =  1; index <= mouseWheel; index += 1)
          this.ShiftUp();
        this.game.EditObj.MouseWheel = 0;
        this.game.EditObj.MouseWheelWait = 4;
        return true;
      }
      if (this.game.EditObj.MouseWheel >= 0)
        return false;
      let mut mouseWheel1: i32 =  this.game.EditObj.MouseWheel;
      for (let mut index: i32 =  -1; index >= mouseWheel1; index += -1)
        this.ShiftDown();
      this.game.EditObj.MouseWheel = 0;
      this.game.EditObj.MouseWheelWait = 4;
      return true;
    }

    pub DynamicArea(
      tgame: GameClass,
      twidth: i32,
      theight: i32,
      tTexty: String,
      ref tbackbitmap: Bitmap = null,
      let mut bbx: i32 =  -1,
      let mut bby: i32 =  -1,
      bool talwaysBlockScrollBar = false,
      bool tJustCheckHeight = false,
      bool tUseActualWidth = false)
      : base(twidth, theight)
    {
      this.Width = twidth;
      this.Height = theight;
      this.game = tgame;
      this.texty = tTexty;
      this.useActualWidth = tUseActualWidth;
      this.alwaysBlockScrollBar = talwaysBlockScrollBar;
      if (!Information.IsNothing( tbackbitmap) & !tJustCheckHeight)
      {
        this.backbitmap = new Bitmap(this.OwnBitmap.Width, this.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        this.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) this.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, this.OwnBitmap.Width, this.OwnBitmap.Height), Rectangle::new(bbx, bby, this.OwnBitmap.Width, this.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      this.bx = bbx;
      this.by = bby;
      this.MouseOver = true;
      if (tJustCheckHeight)
        return;
      this.MakeBitmap();
    }

    pub fn HeightUsed() -> i32 => this.maxY;

    pub fn HandleToolTip(x: i32, y: i32)
    {
      this.game.EditObj.TipColor = 0;
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          this.game.EditObj.TipColor = this.MouseData[index];
          if (this.game.EditObj.TipText.Length != 0)
            break;
          this.game.EditObj.TipText = "...................";
          break;
        }
      }
    }

    pub fn DoJustCheckHeight() -> i32
    {
      DynamicData dynamicData = new DynamicData(this.texty);
      let mut num1: i32 =  0;
      let mut elementCounter: i32 =  dynamicData.elementCounter;
      for (let mut index1: i32 =  0; index1 <= elementCounter; index1 += 1)
      {
        bitmap: Bitmap;
        if (dynamicData.element[index1].type == DynamicType.TextField)
        {
          if (Strings.InStr(dynamicData.element[index1].fontName, "courier") > 0)
          {
            let mut game: GameClass = this.game;
            let mut w: i32 =  dynamicData.element[index1].w;
            let mut trows: i32 =   Math.Round( dynamicData.element[index1].h /  dynamicData.element[index1].lineHeight);
            marcFont4b: Font = this.game.MarcFont4b;
            texty: String = dynamicData.element[index1].texty;
            let mut lineHeight: i32 =  dynamicData.element[index1].lineHeight;
            bitmap = (Bitmap) null;
            ref local: Bitmap = ref bitmap;
            let mut r: i32 =   dynamicData.element[index1].color.R;
            let mut g: i32 =   dynamicData.element[index1].color.G;
            let mut b: i32 =   dynamicData.element[index1].color.B;
            let mut a: i32 =   dynamicData.element[index1].color.A;
            let mut num2: i32 =  new TextAreaClass2(game, w, trows, marcFont4b, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g, colb: b, cola: a, tshadow: false, tUseEncy: true).HeightUsed() + 24;
            if (num2 > num1)
              num1 = num2;
          }
          else
          {
            let mut index2: i32 =  this.game.AddDynFont(dynamicData.element[index1].fontName, dynamicData.element[index1].fontSize, dynamicData.element[index1].fontStyle);
            if (index2 > -1)
            {
              let mut game: GameClass = this.game;
              let mut w: i32 =  dynamicData.element[index1].w;
              let mut trows: i32 =   Math.Round( dynamicData.element[index1].h /  dynamicData.element[index1].lineHeight);
              tfont: Font = this.game.DynFont[index2];
              texty: String = dynamicData.element[index1].texty;
              let mut lineHeight: i32 =  dynamicData.element[index1].lineHeight;
              bitmap = (Bitmap) null;
              ref local: Bitmap = ref bitmap;
              let mut r: i32 =   dynamicData.element[index1].color.R;
              let mut g: i32 =   dynamicData.element[index1].color.G;
              let mut b: i32 =   dynamicData.element[index1].color.B;
              let mut a: i32 =   dynamicData.element[index1].color.A;
              let mut num3: i32 =  new TextAreaClass2(game, w, trows, tfont, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g, colb: b, cola: a, tshadow: false, tUseEncy: true).HeightUsed() + 24;
              if (num3 > num1)
                num1 = num3;
            }
          }
        }
        if (dynamicData.element[index1].type == DynamicType.PictureField && dynamicData.element[index1].eventPicture > -1)
        {
          let mut num4: i32 =  BitmapStore.Getheight(this.game.Data.EventPicNr[dynamicData.element[index1].eventPicture]);
          if (num4 > num1)
            num1 = num4;
        }
      }
      return num1;
    }

    pub fn MakeBitmap()
    {
      this.ClearMouse();
      DynamicData dynamicData = new DynamicData(this.texty);
      Graphics.FromImage((Image) this.OwnBitmap);
      let mut num1: i32 =  0;
      let mut elementCounter1: i32 =  dynamicData.elementCounter;
      for (let mut index1: i32 =  0; index1 <= elementCounter1; index1 += 1)
      {
        if (dynamicData.element[index1].type == DynamicType.TextField)
        {
          let mut index2: i32 =  this.game.AddDynFont(dynamicData.element[index1].fontName, dynamicData.element[index1].fontSize, dynamicData.element[index1].fontStyle);
          if (Strings.InStr(dynamicData.element[index1].fontName, "courier") > 0)
          {
            let mut game: GameClass = this.game;
            let mut w: i32 =  dynamicData.element[index1].w;
            let mut trows: i32 =   Math.Round( dynamicData.element[index1].h /  dynamicData.element[index1].lineHeight);
            marcFont4: Font = this.game.MarcFont4;
            texty: String = dynamicData.element[index1].texty;
            let mut lineHeight: i32 =  dynamicData.element[index1].lineHeight;
            bitmap: Bitmap = (Bitmap) null;
            ref local: Bitmap = ref bitmap;
            let mut r: i32 =   dynamicData.element[index1].color.R;
            let mut g: i32 =   dynamicData.element[index1].color.G;
            let mut b: i32 =   dynamicData.element[index1].color.B;
            let mut a: i32 =   dynamicData.element[index1].color.A;
            TextAreaClass2 textAreaClass2 = new TextAreaClass2(game, w, trows, marcFont4, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g, colb: b, cola: a, tshadow: false, tUseEncy: true);
            let mut num2: i32 =  textAreaClass2.HeightUsed() + dynamicData.element[index1].y;
            if (num2 > num1)
              num1 = num2 + dynamicData.element[index1].lineHeight * 1 + 48;
            dynamicData.element[index1].h = textAreaClass2.HeightUsed() + dynamicData.element[index1].lineHeight * 1 + 48;
          }
          else if (index2 > -1)
          {
            TextAreaClass2 textAreaClass2;
            if (!dynamicData.element[index1].center)
            {
              let mut game: GameClass = this.game;
              let mut w: i32 =  dynamicData.element[index1].w;
              let mut trows: i32 =   Math.Round( dynamicData.element[index1].h /  dynamicData.element[index1].lineHeight);
              tfont: Font = this.game.DynFont[index2];
              texty: String = dynamicData.element[index1].texty;
              let mut lineHeight: i32 =  dynamicData.element[index1].lineHeight;
              bitmap: Bitmap = (Bitmap) null;
              ref local: Bitmap = ref bitmap;
              let mut r: i32 =   dynamicData.element[index1].color.R;
              let mut g: i32 =   dynamicData.element[index1].color.G;
              let mut b: i32 =   dynamicData.element[index1].color.B;
              let mut a: i32 =   dynamicData.element[index1].color.A;
              textAreaClass2 = new TextAreaClass2(game, w, trows, tfont, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g, colb: b, cola: a, tshadow: false, tUseEncy: true);
            }
            else
            {
              let mut game: GameClass = this.game;
              let mut w: i32 =  dynamicData.element[index1].w;
              let mut trows: i32 =   Math.Round( dynamicData.element[index1].h /  dynamicData.element[index1].lineHeight);
              tfont: Font = this.game.DynFont[index2];
              texty: String = dynamicData.element[index1].texty;
              let mut lineHeight: i32 =  dynamicData.element[index1].lineHeight;
              bitmap: Bitmap = (Bitmap) null;
              ref local: Bitmap = ref bitmap;
              let mut r: i32 =   dynamicData.element[index1].color.R;
              let mut g: i32 =   dynamicData.element[index1].color.G;
              let mut b: i32 =   dynamicData.element[index1].color.B;
              let mut a: i32 =   dynamicData.element[index1].color.A;
              textAreaClass2 = new TextAreaClass2(game, w, trows, tfont, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, tcenterit: true, colr: r, colg: g, colb: b, cola: a, tshadow: false, tUseEncy: true);
            }
            let mut num3: i32 =  textAreaClass2.HeightUsed() + dynamicData.element[index1].y;
            if (num3 > num1)
              num1 = num3 + dynamicData.element[index1].lineHeight * 1 + 48;
            dynamicData.element[index1].h = textAreaClass2.HeightUsed() + dynamicData.element[index1].lineHeight * 1 + 48;
          }
        }
        else if (dynamicData.element[index1].type == DynamicType.PictureField && dynamicData.element[index1].y + dynamicData.element[index1].h > num1)
          num1 = dynamicData.element[index1].y + dynamicData.element[index1].h + 48;
      }
      if (this.maxY < this.Height)
        this.maxY = this.Height;
      this.maxY = this.Height;
      this.maxY -= this.maxY % 100;
      if (this.maxY < 100)
        this.maxY = 100;
      let mut elementCounter2: i32 =  dynamicData.elementCounter;
      for (let mut index: i32 =  0; index <= elementCounter2; index += 1)
      {
        if (dynamicData.element[index].y + dynamicData.element[index].h > this.maxY)
          this.maxY = dynamicData.element[index].y + dynamicData.element[index].h;
      }
      this.curY = 0;
      this.paper = this.alwaysBlockScrollBar ? new Bitmap(this.Width, this.maxY, PixelFormat.Format32bppPArgb) : (!this.useActualWidth ? new Bitmap(540, this.maxY, PixelFormat.Format32bppPArgb) : (this.alwaysBlockScrollBar ? new Bitmap(this.Width, this.maxY, PixelFormat.Format32bppPArgb) : new Bitmap(this.Width - 30, this.maxY, PixelFormat.Format32bppPArgb)));
      this.paper.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics g1 = Graphics.FromImage((Image) this.paper);
      if (!this.alwaysBlockScrollBar)
      {
        if (this.useActualWidth)
        {
          let mut width: i32 =  this.Width;
          if (!this.alwaysBlockScrollBar)
            width -= 30;
          g1.CompositingMode = CompositingMode.SourceCopy;
          g1.DrawImage((Image) BitmapStore.GetBitmap(this.game.PAPERBACK1), 0, 0, width, 100);
          for (let mut y: i32 =  100; y < this.maxY - 100; y += 100)
            g1.DrawImage((Image) BitmapStore.GetBitmap(this.game.PAPERBACK2), 0, y, width, 100);
          g1.DrawImage((Image) BitmapStore.GetBitmap(this.game.PAPERBACK3), 0, this.maxY - 100, width, 100);
          g1.CompositingMode = CompositingMode.SourceOver;
        }
        else
        {
          g1.CompositingMode = CompositingMode.SourceCopy;
          g1.DrawImage((Image) BitmapStore.GetBitmap(this.game.PAPERBACK1), 0, 0);
          for (let mut y: i32 =  100; y < this.maxY - 100; y += 100)
            g1.DrawImage((Image) BitmapStore.GetBitmap(this.game.PAPERBACK2), 0, y);
          g1.DrawImage((Image) BitmapStore.GetBitmap(this.game.PAPERBACK3), 0, this.maxY - 100);
          g1.CompositingMode = CompositingMode.SourceOver;
        }
      }
      let mut elementCounter3: i32 =  dynamicData.elementCounter;
      for (let mut index3: i32 =  0; index3 <= elementCounter3; index3 += 1)
      {
        if (dynamicData.element[index3].type == DynamicType.TextField)
        {
          let mut index4: i32 =  this.game.AddDynFont(dynamicData.element[index3].fontName, dynamicData.element[index3].fontSize, dynamicData.element[index3].fontStyle);
          Rectangle trect1;
          Rectangle trect2;
          if (Strings.InStr(dynamicData.element[index3].fontName, "courier") > 0)
          {
            let mut game: GameClass = this.game;
            let mut w: i32 =  dynamicData.element[index3].w;
            let mut trows: i32 =   Math.Round( dynamicData.element[index3].h /  dynamicData.element[index3].lineHeight);
            marcFont4b: Font = this.game.MarcFont4b;
            texty: String = dynamicData.element[index3].texty;
            let mut lineHeight: i32 =  dynamicData.element[index3].lineHeight;
            bitmap: Bitmap = (Bitmap) null;
            ref local: Bitmap = ref bitmap;
            let mut r: i32 =   dynamicData.element[index3].color.R;
            let mut g2: i32 =   dynamicData.element[index3].color.G;
            let mut b: i32 =   dynamicData.element[index3].color.B;
            let mut a: i32 =   dynamicData.element[index3].color.A;
            TextAreaClass2 textAreaClass2 = new TextAreaClass2(game, w, trows, marcFont4b, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g2, colb: b, cola: a, tshadow: false, tUseEncy: true);
            textAreaClass2.Paint();
            let mut mouseCounter: i32 =  textAreaClass2.MouseCounter;
            for (let mut index5: i32 =  0; index5 <= mouseCounter; index5 += 1)
            {
              trect1 = Rectangle::new(textAreaClass2.MouseRect[index5].X + dynamicData.element[index3].x, textAreaClass2.MouseRect[index5].Y + dynamicData.element[index3].y, textAreaClass2.MouseRect[index5].Width, textAreaClass2.MouseRect[index5].Height);
              trect2 = trect1;
              this.AddMouse(ref trect2, textAreaClass2.MouseTitle[index5], textAreaClass2.MouseText[index5], textAreaClass2.MouseData[index5]);
            }
            g1.DrawImage((Image) textAreaClass2.OwnBitmap, dynamicData.element[index3].x, dynamicData.element[index3].y);
          }
          else if (index4 > -1)
          {
            TextAreaClass2 textAreaClass2;
            if (!dynamicData.element[index3].center)
            {
              let mut game: GameClass = this.game;
              let mut w: i32 =  dynamicData.element[index3].w;
              let mut trows: i32 =   Math.Round( dynamicData.element[index3].h /  dynamicData.element[index3].lineHeight);
              tfont: Font = this.game.DynFont[index4];
              texty: String = dynamicData.element[index3].texty;
              let mut lineHeight: i32 =  dynamicData.element[index3].lineHeight;
              bitmap: Bitmap = (Bitmap) null;
              ref local: Bitmap = ref bitmap;
              let mut r: i32 =   dynamicData.element[index3].color.R;
              let mut g3: i32 =   dynamicData.element[index3].color.G;
              let mut b: i32 =   dynamicData.element[index3].color.B;
              let mut a: i32 =   dynamicData.element[index3].color.A;
              textAreaClass2 = new TextAreaClass2(game, w, trows, tfont, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g3, colb: b, cola: a, tshadow: false, tUseEncy: true);
            }
            else
            {
              let mut game: GameClass = this.game;
              let mut w: i32 =  dynamicData.element[index3].w;
              let mut trows: i32 =   Math.Round( dynamicData.element[index3].h /  dynamicData.element[index3].lineHeight);
              tfont: Font = this.game.DynFont[index4];
              texty: String = dynamicData.element[index3].texty;
              let mut lineHeight: i32 =  dynamicData.element[index3].lineHeight;
              bitmap: Bitmap = (Bitmap) null;
              ref local: Bitmap = ref bitmap;
              let mut r: i32 =   dynamicData.element[index3].color.R;
              let mut g4: i32 =   dynamicData.element[index3].color.G;
              let mut b: i32 =   dynamicData.element[index3].color.B;
              let mut a: i32 =   dynamicData.element[index3].color.A;
              textAreaClass2 = new TextAreaClass2(game, w, trows, tfont, texty, lineHeight, ref local, tWithoutScrollbars: true, tWithoutFrame: true, tcenterit: true, colr: r, colg: g4, colb: b, cola: a, tshadow: false, tUseEncy: true);
            }
            textAreaClass2.Paint();
            let mut mouseCounter: i32 =  textAreaClass2.MouseCounter;
            for (let mut index6: i32 =  0; index6 <= mouseCounter; index6 += 1)
            {
              trect2 = Rectangle::new(textAreaClass2.MouseRect[index6].X + dynamicData.element[index3].x, textAreaClass2.MouseRect[index6].Y + dynamicData.element[index3].y, textAreaClass2.MouseRect[index6].Width, textAreaClass2.MouseRect[index6].Height);
              trect1 = trect2;
              this.AddMouse(ref trect1, textAreaClass2.MouseTitle[index6], textAreaClass2.MouseText[index6], textAreaClass2.MouseData[index6]);
            }
            g1.DrawImage((Image) textAreaClass2.OwnBitmap, dynamicData.element[index3].x, dynamicData.element[index3].y);
          }
        }
        if (dynamicData.element[index3].type == DynamicType.PictureField)
        {
          if (dynamicData.element[index3].historicalUnitPortrait > 0)
          {
            let mut historicalUnitById: i32 =  this.game.HandyFunctionsObj.GetHistoricalUnitByID(dynamicData.element[index3].historicalUnitPortrait);
            if (historicalUnitById > -1)
            {
              if (dynamicData.element[index3].w > 0)
                DrawMod.DrawOfficer(g1, historicalUnitById, dynamicData.element[index3].x, dynamicData.element[index3].y, dynamicData.element[index3].w, dynamicData.element[index3].h);
              else
                DrawMod.DrawOfficer(g1, historicalUnitById, dynamicData.element[index3].x, dynamicData.element[index3].y, 95, 105);
            }
          }
          else if (dynamicData.element[index3].w > 0)
          {
            ref Graphics local1 = ref g1;
            bitmap: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[dynamicData.element[index3].eventPicture]);
            ref local2: Bitmap = ref bitmap;
            let mut x: i32 =  dynamicData.element[index3].x;
            let mut y: i32 =  dynamicData.element[index3].y;
            let mut w: i32 =  dynamicData.element[index3].w;
            let mut h: i32 =  dynamicData.element[index3].h;
            let mut width: i32 =  BitmapStore.GetBitmap(this.game.Data.EventPicNr[dynamicData.element[index3].eventPicture]).Width;
            let mut height: i32 =  BitmapStore.GetBitmap(this.game.Data.EventPicNr[dynamicData.element[index3].eventPicture]).Height;
            double r =  ( dynamicData.element[index3].color.R /  byte.MaxValue);
            double g5 =  ( dynamicData.element[index3].color.G /  byte.MaxValue);
            double b =  ( dynamicData.element[index3].color.B /  byte.MaxValue);
            double a =  ( dynamicData.element[index3].color.A /  byte.MaxValue);
            DrawMod.DrawScaledColorized(ref local1, ref local2, x, y, w, h, width, height,  r,  g5,  b,  a);
          }
          else if (dynamicData.element[index3].color.R == byte.MaxValue & dynamicData.element[index3].color.G == byte.MaxValue & dynamicData.element[index3].color.B == byte.MaxValue & dynamicData.element[index3].color.A == byte.MaxValue)
          {
            ref Graphics local3 = ref g1;
            bitmap: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[dynamicData.element[index3].eventPicture]);
            ref local4: Bitmap = ref bitmap;
            let mut x: i32 =  dynamicData.element[index3].x;
            let mut y: i32 =  dynamicData.element[index3].y;
            DrawMod.DrawSimple(ref local3, ref local4, x, y);
          }
          else
          {
            ref Graphics local5 = ref g1;
            bitmap: Bitmap = BitmapStore.GetBitmap(this.game.Data.EventPicNr[dynamicData.element[index3].eventPicture]);
            ref local6: Bitmap = ref bitmap;
            let mut x: i32 =  dynamicData.element[index3].x;
            let mut y: i32 =  dynamicData.element[index3].y;
            double r =  ( dynamicData.element[index3].color.R /  byte.MaxValue);
            double g6 =  ( dynamicData.element[index3].color.G /  byte.MaxValue);
            double b =  ( dynamicData.element[index3].color.B /  byte.MaxValue);
            double a =  ( dynamicData.element[index3].color.A /  byte.MaxValue);
            DrawMod.Draw(ref local5, ref local6, x, y,  r,  g6,  b,  a);
          }
        }
      }
      g1.Dispose();
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF = SizeF::new();
      Graphics objGraphics = Graphics.FromImage((Image) this.OwnBitmap);
      if (this.lastY != this.curY)
      {
        let mut num: i32 =  this.lastY - this.curY;
        let mut mouseCounter: i32 =  this.MouseCounter;
        for (let mut index1: i32 =  0; index1 <= mouseCounter; index1 += 1)
        {
          Rectangle[] mouseRect = this.MouseRect;
          Rectangle[] rectangleArray = mouseRect;
          let mut index2: i32 =  index1;
          let mut index3: i32 =  index2;
          rectangleArray[index3].Y = mouseRect[index2].Y + num;
        }
        this.lastY = this.curY;
      }
      if (!Information.IsNothing( this.backbitmap))
      {
        objGraphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimple(ref objGraphics, ref this.backbitmap, 0, 0);
        objGraphics.CompositingMode = CompositingMode.SourceOver;
      }
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (!this.alwaysBlockScrollBar)
      {
        if (this.useActualWidth)
        {
          if (!this.alwaysBlockScrollBar)
          {
            Graphics graphics = objGraphics;
            paper: Bitmap = this.paper;
            rectangle1 = Rectangle::new(0, 0, this.Width - 30, this.Height);
            let mut destRect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(0, this.curY, this.Width - 30, this.Height);
            let mut srcRect: &Rectangle = &rectangle2
            graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
          }
          else
          {
            Graphics graphics = objGraphics;
            paper: Bitmap = this.paper;
            rectangle2 = Rectangle::new(0, 0, this.Width, this.Height);
            let mut destRect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, this.curY, this.Width, this.Height);
            let mut srcRect: &Rectangle = &rectangle1
            graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
          }
        }
        else
        {
          Graphics graphics = objGraphics;
          paper: Bitmap = this.paper;
          rectangle2 = Rectangle::new(0, 0, 540, this.Height);
          let mut destRect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(0, this.curY, 540, this.Height);
          let mut srcRect: &Rectangle = &rectangle1
          graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
        }
      }
      else
      {
        Graphics graphics = objGraphics;
        paper: Bitmap = this.paper;
        rectangle2 = Rectangle::new(0, 0, this.Width, this.Height);
        let mut destRect: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(0, this.curY, this.Width, this.Height);
        let mut srcRect: &Rectangle = &rectangle1
        graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
      }
      if (!this.alwaysBlockScrollBar && this.maxY > this.Height)
      {
        let mut x1: i32 =  this.Width - 20;
        let mut num: i32 =   Math.Round( (this.Height - 16) * ( this.curY /  (this.maxY - this.Height)) + 8.0);
        if (num > this.Height - 16)
          num = this.Height - 16;
        ref Graphics local1 = ref objGraphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
        ref local2: Bitmap = ref bitmap;
        rectangle2 = Rectangle::new(0, 3, 20, 4);
        let mut srcrect1: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1, 3, 20, this.Height);
        let mut destrect1: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2(ref local1, ref local2, srcrect1, destrect1);
        ref Graphics local3 = ref objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
        ref local4: Bitmap = ref bitmap;
        rectangle2 = Rectangle::new(0, 0, 20, 3);
        let mut srcrect2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1, 0, 20, 3);
        let mut destrect2: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2(ref local3, ref local4, srcrect2, destrect2);
        ref Graphics local5 = ref objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
        ref local6: Bitmap = ref bitmap;
        rectangle2 = Rectangle::new(0, 7, 20, 3);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1, this.Height - 8, 20, 3);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2(ref local5, ref local6, srcrect3, destrect3);
        ref Graphics local7 = ref objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTUP);
        ref local8: Bitmap = ref bitmap;
        let mut x2: i32 =  x1;
        DrawMod.DrawSimple(ref local7, ref local8, x2, 0);
        ref Graphics local9 = ref objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTDOWN);
        ref local10: Bitmap = ref bitmap;
        let mut x3: i32 =  x1;
        let mut y1: i32 =  this.Height - 8;
        DrawMod.DrawSimple(ref local9, ref local10, x3, y1);
        ref Graphics local11 = ref objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
        ref local12: Bitmap = ref bitmap;
        let mut x4: i32 =  x1;
        let mut y2: i32 =  num;
        DrawMod.DrawSimple(ref local11, ref local12, x4, y2);
      }
      return this.OwnBitmap;
    }

    pub fn ShiftDown()
    {
      if (this.maxY <= this.Height)
        return;
      this.curY += 20;
      if (this.curY <= this.maxY - this.Height)
        return;
      this.curY = this.maxY - this.Height;
    }

    pub fn ShiftUp()
    {
      if (this.maxY <= this.Height)
        return;
      this.curY -= 20;
      if (0 <= this.curY)
        return;
      this.curY = 0;
    }

    pub fn HandleMouseUp(x: i32, y: i32) -> i32
    {
      if (!(this.clickscroll == 1 | this.Scroller))
        return -1;
      this.clickscroll = 0;
      this.Scroller = false;
      return 1;
    }

    pub fn Click(x: i32, y: i32, let mut b: i32 =  1) -> i32
    {
      if (this.alwaysBlockScrollBar || this.maxY <= this.Height || x <= this.Width - 20)
      {
        num: i32;
        return num;
      }
      if (y >= 0 & y <= 8)
      {
        this.curY -= 20;
        this.clickscroll = 0;
        if (0 > this.curY)
          this.curY = 0;
        return -1;
      }
      if (y > this.Height - 8)
      {
        this.curY += 20;
        this.clickscroll = 0;
        if (this.curY > this.maxY - this.Height)
          this.curY = this.maxY - this.Height;
        return -1;
      }
      this.clickscroll = 1;
      this.Scroller = true;
      this.curY =  Math.Round( (this.maxY - this.Height) * ( (y - 8) /  (this.Height - 16)));
      if (0 > this.curY)
        this.curY = 0;
      if (this.curY > this.maxY - this.Height)
        this.curY = this.maxY - this.Height;
      return -1;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      if (this.alwaysBlockScrollBar || this.clickscroll != 1)
        return false;
      this.clickscroll = 1;
      this.Scroller = true;
      this.clickscroll = 1;
      this.Scroller = true;
      this.curY =  Math.Round( (this.maxY - this.Height) * ( (y - 8) /  (this.Height - 16)));
      if (0 > this.curY)
        this.curY = 0;
      if (this.curY > this.maxY - this.Height)
        this.curY = this.maxY - this.Height;
      return true;
    }
  }
}
