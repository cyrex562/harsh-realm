// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UDSPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class UDSPartClass : SubPartClass
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
    pub curY: i32;
     maxY: i32;
     lastY: i32;
     paper: Bitmap;
     bool alwaysBlockScrollBar;
     bool justCheckHeight;
     lastElementHigh: i32;
     bool allGray;
     bool alwaysShowBackground;
    pub UDSData dyn;
     scrollelementclicked: i32;
     scrollelementclicked2: i32;
     bmp: Vec<Bitmap>;
     int[] bmpLink;
     backBmp: Vec<Bitmap>;
     int[] backBmpLink;
     backBitmapCounter: i32;
     bool noBackground;

    pub bool UseSourceCopyForPaintSpecific() => true;

    pub fn SubDispose()
    {
      self.unloadAnyStuff();
      if (!Information.IsNothing( self.backbitmap))
      {
        self.backbitmap.Dispose();
        self.backbitmap = (Bitmap) null;
      }
      if (!Information.IsNothing( self.paper))
      {
        self.paper.Dispose();
        self.paper = (Bitmap) null;
      }
      self.game = (GameClass) null;
      self.OwnFont =  null;
      self.dyn = (UDSData) null;
    }

    pub bool HandleTimerWheel(x: i32, y: i32,  WindowClass tWindow)
    {
      if (self.game.EditObj.MouseWheel > 0)
      {
        let mut mouseWheel: i32 = self.game.EditObj.MouseWheel;
        for (let mut index: i32 = 1; index <= mouseWheel; index += 1)
          self.ShiftUp();
        self.game.EditObj.MouseWheel = 0;
        self.game.EditObj.MouseWheelWait = 4;
        return true;
      }
      if (self.game.EditObj.MouseWheel >= 0)
        return false;
      let mut mouseWheel1: i32 = self.game.EditObj.MouseWheel;
      for (let mut index: i32 = -1; index >= mouseWheel1; index += -1)
        self.ShiftDown();
      self.game.EditObj.MouseWheel = 0;
      self.game.EditObj.MouseWheelWait = 4;
      return true;
    }

    pub UDSPartClass(
      tgame: GameClass,
      twidth: i32,
      theight: i32,
      tTexty: String,
       tbackbitmap: Bitmap = null,
      let mut bbx: i32 = -1,
      let mut bby: i32 = -1,
      bool talwaysBlockScrollBar = false,
      bool tJustCheckHeight = false,
      bool tAllGray = false,
      bool tAlwaysShowBackground = false,
      bool tNoBackground = false,
      bool noBitmapDraw = false)
      : base(twidth, theight)
    {
      self.scrollelementclicked = -1;
      self.scrollelementclicked2 = -1;
      self.bmp = new Bitmap[600];
      self.backBmp = new Bitmap[600];
      self.backBitmapCounter = -1;
      self.Width = twidth;
      self.Height = theight;
      self.game = tgame;
      self.texty = tTexty;
      self.allGray = tAllGray;
      self.justCheckHeight = tJustCheckHeight;
      self.alwaysBlockScrollBar = talwaysBlockScrollBar;
      self.lastElementHigh = -1;
      self.alwaysShowBackground = tAlwaysShowBackground;
      self.noBackground = tNoBackground;
      self.curY = 0;
      if (!Information.IsNothing( tbackbitmap) & !self.justCheckHeight)
      {
        self.backbitmap = new Bitmap(self.OwnBitmap.Width, self.OwnBitmap.Height, PixelFormat.Format32bppPArgb);
        self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics graphics = Graphics.FromImage((Image) self.backbitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) tbackbitmap, Rectangle::new(0, 0, self.OwnBitmap.Width, self.OwnBitmap.Height), Rectangle::new(bbx, bby, self.OwnBitmap.Width, self.OwnBitmap.Height), GraphicsUnit.Pixel);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      self.bx = bbx;
      self.by = bby;
      self.MouseOver = true;
      if (!self.justCheckHeight)
      {
        self.MakeBitmap();
      }
      else
      {
        if (noBitmapDraw || !Information.IsNothing( self.dyn))
          return;
        self.dyn = new UDSData(self.texty, self.allGray);
      }
    }

    pub fn HeightUsed() -> i32 => self.maxY;

    pub AdjustSliders: bool(activeSliderNr: i32)
    {
      let mut num1: i32 = 0;
      let mut num2: i32 = 0;
      int[] numArray1 = new int[self.dyn.elementCounter + 1];
      SimpleList simpleList = SimpleList::new();
      if (self.dyn.element[activeSliderNr].group < 1)
        return false;
      let mut elementCounter: i32 = self.dyn.elementCounter;
      for (let mut tid: i32 = 0; tid <= elementCounter; tid += 1)
      {
        if (self.dyn.element[tid].type == UDSType.Slider & self.dyn.element[tid].group == self.dyn.element[activeSliderNr].group)
        {
          num1 =  Math.Round( num1 + Conversions.ToDouble(self.dyn.element[tid].value));
          numArray1[tid] = Conversions.ToInteger(self.dyn.element[tid].value);
          if (tid != activeSliderNr)
          {
            bool flag = true;
            if (self.dyn.element[tid].rotation > 0 && self.dyn.element[self.dyn.element[tid].rotation].flagged)
              flag = false;
            if (flag)
            {
              num2 += 1;
              simpleList.Add(tid, 1);
            }
          }
        }
      }
      if (num2 <= 0)
      {
        if (num1 > 100)
        {
          UDSElement[] element = self.dyn.element;
          UDSElement[] udsElementArray = element;
          let mut index1: i32 = activeSliderNr;
          let mut index2: i32 = index1;
          udsElementArray[index2].value = Conversions.ToString(Conversions.ToDouble(element[index1].value) -  (num1 - 100));
          if (Conversions.ToDouble(self.dyn.element[activeSliderNr].value) <  self.dyn.element[activeSliderNr].minvalue)
            self.dyn.element[activeSliderNr].value = Conversions.ToString(self.dyn.element[activeSliderNr].minvalue);
        }
        else if (num1 < 100)
        {
          UDSElement[] element = self.dyn.element;
          UDSElement[] udsElementArray = element;
          let mut index3: i32 = activeSliderNr;
          let mut index4: i32 = index3;
          udsElementArray[index4].value = Conversions.ToString(Conversions.ToDouble(element[index3].value) +  (100 - num1));
          if (Conversions.ToDouble(self.dyn.element[activeSliderNr].value) >  self.dyn.element[activeSliderNr].maxvalue)
            self.dyn.element[activeSliderNr].value = Conversions.ToString(self.dyn.element[activeSliderNr].maxvalue);
        }
        return false;
      }
      if (num1 > 100)
      {
        let mut num3: i32 = DrawMod.RandyNumber.Next(0, simpleList.Counter + 1) - 1;
        let mut num4: i32 = 0;
        while (num1 > 100)
        {
          if (num3 == simpleList.Counter)
            num3 = -1;
          bool flag = false;
          let mut num5: i32 = num3 + 1;
          let mut counter: i32 = simpleList.Counter;
          for (let mut index5: i32 = num5; index5 <= counter; index5 += 1)
          {
            let mut index6: i32 = simpleList.Id[index5];
            num3 = index5;
            if (numArray1[index6] > 0 & activeSliderNr != index6)
            {
              int[] numArray2 = numArray1;
              int[] numArray3 = numArray2;
              let mut index7: i32 = index6;
              let mut index8: i32 = index7;
              let mut num6: i32 = numArray2[index7] - 1;
              numArray3[index8] = num6;
              --num1;
              flag = true;
              break;
            }
          }
          if (!flag)
            num4 += 1;
          if (num4 > 19)
            break;
        }
      }
      if (num1 < 100)
      {
        let mut num7: i32 = DrawMod.RandyNumber.Next(0, simpleList.Counter + 1) - 1;
        while (num1 < 100)
        {
          bool flag = false;
          if (num7 == simpleList.Counter)
            num7 = -1;
          let mut num8: i32 = num7 + 1;
          let mut counter: i32 = simpleList.Counter;
          for (let mut index9: i32 = num8; index9 <= counter; index9 += 1)
          {
            let mut index10: i32 = simpleList.Id[index9];
            num7 = index9;
            if (numArray1[index10] < 100)
            {
              int[] numArray4 = numArray1;
              int[] numArray5 = numArray4;
              let mut index11: i32 = index10;
              let mut index12: i32 = index11;
              let mut num9: i32 = numArray4[index11] + 1;
              numArray5[index12] = num9;
              num1 += 1;
              flag = true;
              break;
            }
          }
          if (!flag)
            break;
        }
      }
      let mut counter1: i32 = simpleList.Counter;
      for (let mut index13: i32 = 0; index13 <= counter1; index13 += 1)
      {
        let mut index14: i32 = simpleList.Id[index13];
        if (index14 != activeSliderNr)
          self.dyn.element[index14].value = Conversions.ToString(numArray1[index14]);
      }
      if (num1 > 100)
      {
        UDSElement[] element = self.dyn.element;
        UDSElement[] udsElementArray = element;
        let mut index15: i32 = activeSliderNr;
        let mut index16: i32 = index15;
        udsElementArray[index16].value = Conversions.ToString(Conversions.ToDouble(element[index15].value) -  (num1 - 100));
        if (Conversions.ToDouble(self.dyn.element[activeSliderNr].value) <  self.dyn.element[activeSliderNr].minvalue)
          self.dyn.element[activeSliderNr].value = Conversions.ToString(self.dyn.element[activeSliderNr].minvalue);
      }
      else if (num1 < 100)
      {
        UDSElement[] element = self.dyn.element;
        UDSElement[] udsElementArray = element;
        let mut index17: i32 = activeSliderNr;
        let mut index18: i32 = index17;
        udsElementArray[index18].value = Conversions.ToString(Conversions.ToDouble(element[index17].value) +  (100 - num1));
        if (Conversions.ToDouble(self.dyn.element[activeSliderNr].value) >  self.dyn.element[activeSliderNr].maxvalue)
          self.dyn.element[activeSliderNr].value = Conversions.ToString(self.dyn.element[activeSliderNr].maxvalue);
      }
      return true;
    }

    pub bool HandleMouseMove(x: i32, y: i32)
    {
      let mut i1: i32 = -1;
      bool flag = false;
      let mut mouseCounter1: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter1; index += 1)
      {
        if (self.MouseType[index] == 3 & self.scrollelementclicked > -1)
        {
          Graphics g = Graphics.FromImage((Image) self.paper);
          let mut activeSliderNr: i32 = self.MouseData[index];
          if (self.scrollelementclicked == activeSliderNr)
          {
            self.dyn.element[activeSliderNr].flagged = true;
            let mut num1: i32 =  Math.Round(Math.Max(20.0,  self.MouseRect[index].Width / 10.0));
            let mut num2: i32 = x - self.MouseRect[index].X;
            let mut num3: i32 = self.MouseRect[index].Width - num1;
            num4: i32;
            if ( num2 <  num1 / 2.0)
              num4 = self.dyn.element[activeSliderNr].minvalue;
            else if ( num2 >  self.MouseRect[index].Width -  num1 / 2.0)
            {
              num4 = self.dyn.element[activeSliderNr].maxvalue;
            }
            else
            {
              let mut num5: i32 =  Math.Round( num2 -  num1 / 2.0);
              num4 =  Math.Round( (self.dyn.element[activeSliderNr].maxvalue - self.dyn.element[activeSliderNr].minvalue) * ( num5 /  num3)) + self.dyn.element[activeSliderNr].minvalue;
            }
            self.dyn.element[activeSliderNr].value = Conversions.ToString(num4);
            if (self.AdjustSliders(activeSliderNr))
            {
              let mut elementCounter: i32 = self.dyn.elementCounter;
              for (let mut i2: i32 = 0; i2 <= elementCounter; i2 += 1)
              {
                if (activeSliderNr != i2 & self.dyn.element[i2].type == UDSType.Slider & self.dyn.element[i2].group == self.dyn.element[activeSliderNr].group)
                  self.DrawElement(i2,  g, false);
              }
            }
            i1 = self.MouseData[index];
            flag = true;
          }
          else if (self.MouseType[index] != 3)
          {
            self.scrollelementclicked = -1;
            self.scrollelementclicked2 = -1;
          }
        }
      }
      let mut mouseCounter2: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter2; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseType[index] > 0 & self.MouseType[index] != 9)
          {
            i1 = self.MouseData[index];
            flag = true;
            break;
          }
          break;
        }
      }
      if (self.scrollelementclicked > -1 & i1 != self.scrollelementclicked)
        i1 = self.scrollelementclicked;
      if (i1 == self.lastElementHigh & self.scrollelementclicked == -1)
        flag = false;
      else if (i1 == -1 & self.lastElementHigh > -1)
        flag = true;
      if (flag)
      {
        if (self.game.EmpireStyle)
          SoundMod.PlayAWave(self.game.AppPath + "sound/interface/mouseover.wav",  self.game.EditObj);
        Graphics g = Graphics.FromImage((Image) self.paper);
        if (i1 > -1)
        {
          if (self.lastElementHigh != -1 && self.lastElementHigh != i1 && !(self.dyn.element[self.lastElementHigh].type == UDSType.Table | self.dyn.element[self.lastElementHigh].type == UDSType.TextField | self.dyn.element[self.lastElementHigh].type == UDSType.PictureField))
            self.DrawElement(self.lastElementHigh,  g, false);
          if (!(self.dyn.element[i1].type == UDSType.Table | self.dyn.element[i1].type == UDSType.TextField | self.dyn.element[i1].type == UDSType.PictureField))
            self.DrawElement(i1,  g, false, true);
        }
        else if (self.lastElementHigh > -1 && !(self.dyn.element[self.lastElementHigh].type == UDSType.Table | self.dyn.element[self.lastElementHigh].type == UDSType.TextField | self.dyn.element[self.lastElementHigh].type == UDSType.PictureField))
          self.DrawElement(self.lastElementHigh,  g, false);
      }
      self.lastElementHigh = i1;
      return flag;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      self.game.EditObj.TipColor = 0;
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index1: i32 = 0; index1 <= mouseCounter; index1 += 1)
      {
        if (x > self.MouseRect[index1].X & x < self.MouseRect[index1].X + self.MouseRect[index1].Width && y > self.MouseRect[index1].Y & y < self.MouseRect[index1].Y + self.MouseRect[index1].Height)
        {
          self.game.EditObj.TipButton = false;
          if (self.MouseType[index1] > 0)
            self.game.EditObj.TipButton = true;
          if (self.MouseType[index1] < 1 & self.MouseData[index1] > 0)
            self.game.EditObj.TipButton = true;
          if (self.MouseType[index1] == 25)
          {
            StringListClass stringListClass = self.game.Data.StringListObj[ Math.Round(Conversion.Val(self.dyn.element[self.MouseData[index1]].texty))];
            let mut num1: i32 = y - self.dyn.element[self.MouseData[index1]].y + self.curY;
            if (num1 <= self.dyn.element[self.MouseData[index1]].lineHeight)
            {
              index2: i32;
              if (stringListClass.ColWidth[0] > 0)
              {
                let mut num2: i32 = x - self.dyn.element[self.MouseData[index1]].x;
                let mut num3: i32 = 0;
                index2 = -1;
                let mut width: i32 = stringListClass.Width;
                for (let mut index3: i32 = 0; index3 <= width; index3 += 1)
                {
                  num3 += stringListClass.ColWidth[index3];
                  if (num2 < num3)
                  {
                    index2 = index3;
                    break;
                  }
                }
                if (index2 == -1)
                  index2 = 0;
              }
              else
              {
                let mut num4: i32 =  Math.Round( self.dyn.element[self.MouseData[index1]].w /  (stringListClass.Width + 1));
                index2 =  Math.Round(Math.Floor( (x - self.dyn.element[self.MouseData[index1]].x) /  num4));
              }
              if (stringListClass.ColValueType[index2] == NewEnums.LibVarValueType.Number)
              {
                self.game.EditObj.TipTitle = "";
                if (stringListClass.ColSort[index2] == 0)
                  self.game.EditObj.TipText = "Click to sort high to low";
                if (stringListClass.ColSort[index2] == 1)
                  self.game.EditObj.TipText = "Click to sort high to low";
                if (stringListClass.ColSort[index2] != 2)
                  break;
                self.game.EditObj.TipText = "Click to sort low to high";
                break;
              }
              if (stringListClass.ColValueType[index2] == NewEnums.LibVarValueType.Text)
              {
                self.game.EditObj.TipTitle = "";
                if (stringListClass.ColSort[index2] == 0)
                  self.game.EditObj.TipText = "Click to sort A to Z";
                if (stringListClass.ColSort[index2] == 1)
                  self.game.EditObj.TipText = "Click to sort A to Z";
                if (stringListClass.ColSort[index2] != 2)
                  break;
                self.game.EditObj.TipText = "Click to sort Z to A";
                break;
              }
              self.game.EditObj.TipButton = false;
              break;
            }
            if (num1 > self.dyn.element[self.MouseData[index1]].lineHeight * (self.dyn.element[self.MouseData[index1]].rowsPerPage + 1) - 2)
            {
              self.game.EditObj.TipButton = false;
              break;
            }
            index4: i32;
            if (stringListClass.ColWidth[0] > 0)
            {
              let mut num5: i32 = x - self.dyn.element[self.MouseData[index1]].x;
              let mut num6: i32 = 0;
              index4 = -1;
              let mut width: i32 = stringListClass.Width;
              for (let mut index5: i32 = 0; index5 <= width; index5 += 1)
              {
                if (index5 <= stringListClass.Width)
                {
                  num6 += stringListClass.ColWidth[index5];
                  if (num5 < num6)
                  {
                    index4 = index5;
                    break;
                  }
                }
              }
              if (index4 == -1)
                index4 = 0;
            }
            else
            {
              let mut num7: i32 =  Math.Round( self.dyn.element[self.MouseData[index1]].w /  (stringListClass.Width + 1));
              index4 =  Math.Round(Math.Floor( (x - self.dyn.element[self.MouseData[index1]].x) /  num7));
            }
            let mut index6: i32 =  Math.Round(Math.Floor( (num1 - self.dyn.element[self.MouseData[index1]].lineHeight) /  self.dyn.element[self.MouseData[index1]].lineHeight)) + self.dyn.element[self.MouseData[index1]].topRow;
            str1: String = "";
            if (index6 <= stringListClass.Length)
            {
              str2: String = stringListClass.TempDesc[index6, index4];
              if (!Information.IsNothing( str2))
              {
                if (Strings.InStr(str2, "#") > 0)
                  str1 = str2.Split('#')[0];
                else if (str2.Length > 1)
                  str1 = str2;
              }
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = str1;
            }
            if (str1.Length >= 2)
              break;
            self.game.EditObj.TipButton = false;
            break;
          }
          self.game.EditObj.TipTitle = self.MouseTitle[index1];
          self.game.EditObj.TipText = self.MouseText[index1];
          if (Information.IsNothing( self.MouseText[index1]))
            break;
          self.game.EditObj.TipColor = self.MouseType[index1] >= 1 ? 0 : self.MouseData[index1];
          if (self.game.EditObj.TipText.Length != 0)
            break;
          self.game.EditObj.TipText = "...................";
          break;
        }
      }
    }

    pub fn DoJustCheckHeight(bool heightIncludingY = false) -> i32
    {
      self.dyn = new UDSData(self.texty, false);
      let mut num1: i32 = 0;
      let mut elementCounter: i32 = self.dyn.elementCounter;
      for (let mut index1: i32 = 0; index1 <= elementCounter; index1 += 1)
      {
        if (self.dyn.element[index1].type == UDSType.TextField)
        {
          let mut index2: i32 = self.game.AddDynFont(self.dyn.element[index1].fontName, self.dyn.element[index1].fontSize, self.dyn.element[index1].fontStyle);
          if (index2 > -1)
          {
            let mut game: GameClass = self.game;
            let mut w: i32 = self.dyn.element[index1].w;
            let mut trows: i32 =  Math.Round( self.dyn.element[index1].h /  self.dyn.element[index1].lineHeight);
            tfont: Font = self.game.DynFont[index2];
            texty: String = self.dyn.element[index1].texty;
            let mut lineHeight: i32 = self.dyn.element[index1].lineHeight;
            bitmap: Bitmap = (Bitmap) null;
             let mut local: &Bitmap = &bitmap;
            let mut r: i32 =  self.dyn.element[index1].color.R;
            let mut g: i32 =  self.dyn.element[index1].color.G;
            let mut b: i32 =  self.dyn.element[index1].color.B;
            let mut a: i32 =  self.dyn.element[index1].color.A;
            TextAreaClass2 textAreaClass2 = new TextAreaClass2(game, w, trows, tfont, texty, lineHeight,  local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g, colb: b, cola: a, tshadow: false, tUseMin40width: false);
            let mut num2: i32 = textAreaClass2.HeightUsed();
            if (heightIncludingY)
              num2 += self.dyn.element[index1].y;
            if (num2 > num1)
              num1 = num2;
            textAreaClass2.Dispose();
          }
        }
        if (self.dyn.element[index1].type == UDSType.PictureField && self.dyn.element[index1].eventPicture > -1)
        {
          let mut num3: i32 = BitmapStore.Getheight(self.game.Data.EventPicNr[self.dyn.element[index1].eventPicture]);
          if (self.dyn.element[index1].h < num3)
            num3 = self.dyn.element[index1].h;
          if (heightIncludingY)
            num3 += self.dyn.element[index1].y;
          if (num3 > num1)
            num1 = num3;
        }
      }
      return num1;
    }

    pub fn DoJustCheckWidth() -> i32
    {
      self.dyn = new UDSData(self.texty, false);
      let mut num1: i32 = 0;
      let mut elementCounter: i32 = self.dyn.elementCounter;
      for (let mut index1: i32 = 0; index1 <= elementCounter; index1 += 1)
      {
        if (self.dyn.element[index1].type == UDSType.TextField)
        {
          let mut index2: i32 = self.game.AddDynFont(self.dyn.element[index1].fontName, self.dyn.element[index1].fontSize, self.dyn.element[index1].fontStyle);
          if (index2 > -1)
          {
            let mut game: GameClass = self.game;
            let mut w: i32 = self.dyn.element[index1].w;
            let mut trows: i32 =  Math.Round( self.dyn.element[index1].h /  self.dyn.element[index1].lineHeight);
            tfont: Font = self.game.DynFont[index2];
            texty: String = self.dyn.element[index1].texty;
            let mut lineHeight: i32 = self.dyn.element[index1].lineHeight;
            bitmap: Bitmap = (Bitmap) null;
             let mut local: &Bitmap = &bitmap;
            let mut r: i32 =  self.dyn.element[index1].color.R;
            let mut g: i32 =  self.dyn.element[index1].color.G;
            let mut b: i32 =  self.dyn.element[index1].color.B;
            let mut a: i32 =  self.dyn.element[index1].color.A;
            TextAreaClass2 textAreaClass2 = new TextAreaClass2(game, w, trows, tfont, texty, lineHeight,  local, tWithoutScrollbars: true, tWithoutFrame: true, colr: r, colg: g, colb: b, cola: a, tshadow: false, tUseMin40width: false);
            let mut num2: i32 = textAreaClass2.WidthUsed();
            if (num2 > num1)
              num1 = num2;
            textAreaClass2.Dispose();
          }
        }
        if (self.dyn.element[index1].type == UDSType.PictureField && self.dyn.element[index1].eventPicture > -1)
        {
          let mut num3: i32 = BitmapStore.GetWidth(self.game.Data.EventPicNr[self.dyn.element[index1].eventPicture]);
          if (self.dyn.element[index1].w < num3)
            num3 = self.dyn.element[index1].w;
          if (num3 > num1)
            num1 = num3;
        }
      }
      return num1;
    }

    pub fn MakeBitmap()
    {
      self.ClearMouse();
      if (Information.IsNothing( self.dyn))
        self.dyn = new UDSData(self.texty, self.allGray);
      self.loadPageStuff();
      Graphics graphics1 = Graphics.FromImage((Image) self.OwnBitmap);
      let mut num1: i32 = 0;
      let mut elementCounter1: i32 = self.dyn.elementCounter;
      for (let mut index1: i32 = 0; index1 <= elementCounter1; index1 += 1)
      {
        if (self.dyn.element[index1].type == UDSType.TextField)
        {
          let mut index2: i32 = self.game.AddDynFont(self.dyn.element[index1].fontName, self.dyn.element[index1].fontSize, self.dyn.element[index1].fontStyle);
          if (index2 > -1)
          {
            SizeF sizeF = graphics1.MeasureString(self.dyn.element[index1].texty, self.game.DynFont[index2], self.dyn.element[index1].w);
            let mut num2: i32 =  Math.Round( sizeF.Height);
            if (num2 < self.dyn.element[index1].lineHeight)
              num2 = self.dyn.element[index1].lineHeight;
            let mut num3: i32 = num2 + self.dyn.element[index1].y;
            if (num3 > num1)
              num1 = num3 + self.dyn.element[index1].lineHeight * 1 + 48;
            self.dyn.element[index1].h =  Math.Round( sizeF.Height +  (self.dyn.element[index1].lineHeight * 1) + 48.0);
          }
        }
        else if (self.dyn.element[index1].type == UDSType.PictureField)
        {
          if (self.dyn.element[index1].y + self.dyn.element[index1].h > num1)
            num1 = self.dyn.element[index1].y + self.dyn.element[index1].h + 48;
        }
        else if (self.dyn.element[index1].type == UDSType.PictureField)
        {
          if (self.dyn.element[index1].y + self.dyn.element[index1].h > num1)
            num1 = self.dyn.element[index1].y + self.dyn.element[index1].h;
        }
        else if (self.dyn.element[index1].type == UDSType.Flag)
        {
          if (self.dyn.element[index1].y + self.dyn.element[index1].h > num1)
            num1 = self.dyn.element[index1].y + self.dyn.element[index1].h;
        }
        else if (self.dyn.element[index1].type == UDSType.Table && self.dyn.element[index1].y + self.dyn.element[index1].h > num1)
          num1 = self.dyn.element[index1].y + self.dyn.element[index1].h;
      }
      if (self.maxY < self.Height)
        self.maxY = self.Height;
      self.maxY = self.Height;
      self.maxY -= self.maxY % 100;
      let mut elementCounter2: i32 = self.dyn.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter2; index += 1)
      {
        if (self.dyn.element[index].y + self.dyn.element[index].h > self.maxY && self.dyn.element[index].type != UDSType.Line)
          self.maxY = self.dyn.element[index].y + self.dyn.element[index].h;
      }
      self.maxY += 20;
      if (self.maxY < self.Height)
        self.maxY = self.Height;
      if (self.maxY > 5000)
        self.maxY = 5000;
      self.paper = new Bitmap(self.Width, self.maxY, PixelFormat.Format32bppPArgb);
      self.paper.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics2 = Graphics.FromImage((Image) self.paper);
      num4: i32;
      if (!self.noBackground)
      {
        if (!self.alwaysBlockScrollBar)
        {
          graphics2.CompositingMode = CompositingMode.SourceCopy;
          Rectangle rectangle1;
          Rectangle rectangle2;
          y: i32;
          for (; y < self.maxY - 1390; y += 1390)
          {
             let mut local1: &Graphics = &graphics2;
            bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_PAPER2);
             let mut local2: &Bitmap = &bitmap;
            rectangle1 = Rectangle::new(0, 0, Math.Min(1126, self.Width - 24), 1390);
            let mut srcrect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(0, y, self.Width - 24, 1390);
            let mut destrect: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          }
          if (y < self.maxY)
          {
             let mut local3: &Graphics = &graphics2;
            bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_PAPER2);
             let mut local4: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, Math.Min(1126, self.Width - 24), self.maxY - y);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, y, self.Width - 24, self.maxY - y);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
            num4 = y + (self.maxY - y);
          }
           let mut local5: &Graphics = &graphics2;
           local6: Bitmap =  self.backbitmap;
          rectangle2 = Rectangle::new(0, 0, self.Width, 5);
          let mut rect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart( local5,  local6, rect);
          graphics2.CompositingMode = CompositingMode.SourceOver;
          if (self.maxY >= 256)
          {
             let mut local7: &Graphics = &graphics2;
            bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_PAPER3);
             let mut local8: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, Math.Min(1126, self.Width - 24), 256);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, self.maxY - 256, self.Width - 24, 256);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local7,  local8, srcrect, destrect);
          }
          DrawMod.DrawRectangle( graphics2, 10, 15, self.Width - 45, self.maxY - 25, 0, 0, 0, 32, 10);
        }
        else if (self.alwaysShowBackground)
        {
          graphics2.CompositingMode = CompositingMode.SourceCopy;
          Rectangle rectangle3;
          Rectangle rectangle4;
          y: i32;
          for (; y < self.maxY - 1390; y += 1390)
          {
             let mut local9: &Graphics = &graphics2;
            bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_PAPER2);
             let mut local10: &Bitmap = &bitmap;
            rectangle3 = Rectangle::new(0, 0, Math.Min(1126, self.Width - 0), 1390);
            let mut srcrect: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, y, self.Width - 0, 1390);
            let mut destrect: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
          }
          if (y < self.maxY)
          {
             let mut local11: &Graphics = &graphics2;
            bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_PAPER2);
             let mut local12: &Bitmap = &bitmap;
            rectangle3 = Rectangle::new(0, 0, Math.Min(1126, self.Width - 0), self.maxY - y);
            let mut srcrect: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, y, self.Width - 0, self.maxY - y);
            let mut destrect: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
            num4 = y + (self.maxY - y);
          }
          graphics2.CompositingMode = CompositingMode.SourceOver;
          if (self.maxY >= 256)
          {
             let mut local13: &Graphics = &graphics2;
            bitmap: Bitmap = BitmapStore.GetBitmap(self.game.SE1_PAPER3);
             let mut local14: &Bitmap = &bitmap;
            rectangle3 = Rectangle::new(0, 0, Math.Min(1126, self.Width - 0), 256);
            let mut srcrect: &Rectangle = &rectangle3
            rectangle4 = Rectangle::new(0, self.maxY - 256, self.Width - 0, 256);
            let mut destrect: &Rectangle = &rectangle4
            DrawMod.DrawSimplePart2( local13,  local14, srcrect, destrect);
          }
        }
      }
      self.backBmpLink = new int[self.dyn.elementCounter + 1];
      let mut elementCounter3: i32 = self.dyn.elementCounter;
      for (let mut i: i32 = 0; i <= elementCounter3; i += 1)
        self.DrawElement(i,  graphics2, true);
      graphics2.Dispose();
      if (!(!Information.IsNothing( self.dyn) & !self.justCheckHeight))
        return;
      let mut elementCounter4: i32 = self.dyn.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter4; index += 1)
      {
        if (self.dyn.element[index].type == UDSType.Wav)
        {
          if (!self.game.EmpireStyle)
            break;
          SoundMod.PlayAWave(self.game.AppPath + "sound/" + self.dyn.element[index].texty,  self.game.EditObj);
          break;
        }
      }
    }

    pub fn DrawElement(i: i32,  Graphics g, bool firstCall, bool high = false)
    {
      Graphics Expression;
      Rectangle trect;
      Rectangle rectangle;
      if (self.dyn.element[i].type == UDSType.TextField)
      {
        let mut index1: i32 = self.game.AddDynFont(self.dyn.element[i].fontName, self.dyn.element[i].fontSize, self.dyn.element[i].fontStyle);
        if (index1 > -1)
        {
          if (self.dyn.element[i].parentElement > -1)
          {
            if (firstCall)
            {
              self += 1.backBitmapCounter;
              self.backBmp[self.backBitmapCounter] = new Bitmap(self.dyn.element[i].w, self.dyn.element[i].h, PixelFormat.Format32bppPArgb);
              self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
              Expression = Graphics.FromImage((Image) self.backBmp[self.backBitmapCounter]);
              Expression.CompositingMode = CompositingMode.SourceCopy;
              Graphics graphics = Expression;
              paper: Bitmap = self.paper;
              trect = Rectangle::new(0, 0, self.backBmp[self.backBitmapCounter].Width, self.backBmp[self.backBitmapCounter].Height);
              let mut destRect: &Rectangle = &trect
              rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
              let mut srcRect: &Rectangle = &rectangle
              graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
              Expression.CompositingMode = CompositingMode.SourceOver;
              self.backBmpLink[i] = self.backBitmapCounter;
            }
            else
              g.DrawImage((Image) self.backBmp[self.backBmpLink[i]], self.dyn.element[i].x, self.dyn.element[i].y);
          }
          let mut game: GameClass = self.game;
          let mut w: i32 = self.dyn.element[i].w;
          let mut trows: i32 =  Math.Round( self.dyn.element[i].h /  self.dyn.element[i].lineHeight);
          tfont: Font = self.game.DynFont[index1];
          texty: String = self.dyn.element[i].texty;
          let mut lineHeight: i32 = self.dyn.element[i].lineHeight;
          bitmap: Bitmap = (Bitmap) null;
           let mut local: &Bitmap = &bitmap;
          let mut num: i32 = self.dyn.element[i].center == 1 ? 1 : 0;
          let mut r: i32 =  self.dyn.element[i].color.R;
          let mut g1: i32 =  self.dyn.element[i].color.G;
          let mut b: i32 =  self.dyn.element[i].color.B;
          let mut a: i32 =  self.dyn.element[i].color.A;
          TextAreaClass2 textAreaClass2 = new TextAreaClass2(game, w, trows, tfont, texty, lineHeight,  local, tWithoutScrollbars: true, tWithoutFrame: true, tcenterit: (num != 0), colr: r, colg: g1, colb: b, cola: a, tshadow: false, tUseEncy: true, tminimalHeight: true, tUseMin40width: false);
          textAreaClass2.Paint();
          let mut mouseCounter: i32 = textAreaClass2.MouseCounter;
          for (let mut index2: i32 = 0; index2 <= mouseCounter; index2 += 1)
          {
            if (firstCall)
            {
              rectangle = Rectangle::new(textAreaClass2.MouseRect[index2].X + self.dyn.element[i].x, textAreaClass2.MouseRect[index2].Y + self.dyn.element[i].y, textAreaClass2.MouseRect[index2].Width, textAreaClass2.MouseRect[index2].Height);
              trect = rectangle;
              self.AddMouse( trect, textAreaClass2.MouseTitle[index2], textAreaClass2.MouseText[index2], textAreaClass2.MouseData[index2]);
            }
          }
          g.DrawImage((Image) textAreaClass2.OwnBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
          textAreaClass2.Dispose();
        }
      }
      if (self.dyn.element[i].type == UDSType.Table)
      {
        let mut index: i32 = self.game.AddDynFont(self.dyn.element[i].fontName, self.dyn.element[i].fontSize, self.dyn.element[i].fontStyle);
        if (index > -1)
        {
          if (firstCall)
          {
            self += 1.backBitmapCounter;
            self.backBmp[self.backBitmapCounter] = new Bitmap(self.dyn.element[i].w, self.dyn.element[i].h, PixelFormat.Format32bppPArgb);
            self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
            Expression = Graphics.FromImage((Image) self.backBmp[self.backBitmapCounter]);
            Expression.CompositingMode = CompositingMode.SourceCopy;
            Graphics graphics = Expression;
            paper: Bitmap = self.paper;
            rectangle = Rectangle::new(0, 0, self.backBmp[self.backBitmapCounter].Width, self.backBmp[self.backBitmapCounter].Height);
            let mut destRect: &Rectangle = &rectangle
            trect = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            let mut srcRect: &Rectangle = &trect
            graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
            Expression.CompositingMode = CompositingMode.SourceOver;
            self.backBmpLink[i] = self.backBitmapCounter;
          }
          else
            g.DrawImage((Image) self.backBmp[self.backBmpLink[i]], self.dyn.element[i].x, self.dyn.element[i].y);
          StringListClass stringListClass = self.game.Data.StringListObj[ Math.Round(Conversion.Val(self.dyn.element[i].texty))];
          let mut num1: i32 = stringListClass.Length + 1;
          let mut num2: i32 =  Math.Round(Math.Floor( self.dyn.element[i].h /  self.dyn.element[i].lineHeight)) - 2;
          StringListClass tListobj = stringListClass;
          let mut tlistsize: i32 = num2;
          let mut w: i32 = self.dyn.element[i].w;
          let mut tgame: GameClass = DrawMod.TGame;
          let mut topRow: i32 = self.dyn.element[i].topRow;
          bitmap: Bitmap = (Bitmap) null;
           let mut local1: &Bitmap = &bitmap;
          let mut lineHeight: i32 = self.dyn.element[i].lineHeight;
           local2: Font =  self.game.DynFont[index];
          UDSMatrixSubPartClass matrixSubPartClass = new UDSMatrixSubPartClass(tListobj, tlistsize, w, -1, -1, tgame, tHighlight: false, tTop: topRow, tbackbitmap: ( local1), trowheight: lineHeight, tfontsize: 16, tMarcy: true, customFont: ( local2));
          matrixSubPartClass.Paint();
          g.DrawImage((Image) matrixSubPartClass.OwnBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
          matrixSubPartClass.Dispose();
          if (firstCall)
          {
            rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            trect = rectangle;
            self.AddMouse( trect, "", "", i, 25);
          }
        }
      }
      bitmap1: Bitmap;
      if (self.dyn.element[i].type == UDSType.PictureField)
      {
        if (Information.IsNothing( self.dyn.element[i].mouseOver))
          self.dyn.element[i].mouseOver = "";
        if (self.dyn.element[i].historicalUnitPortrait > 0)
        {
          let mut historicalUnitById: i32 = self.game.HandyFunctionsObj.GetHistoricalUnitByID(self.dyn.element[i].historicalUnitPortrait);
          if (historicalUnitById > -1)
          {
            if (self.dyn.element[i].w > 0)
              DrawMod.DrawOfficer(g, historicalUnitById, self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            else
              DrawMod.DrawOfficer(g, historicalUnitById, self.dyn.element[i].x, self.dyn.element[i].y, 95, 105);
          }
        }
        else if (self.dyn.element[i].customBitmapFunction > 0)
        {
          if (self.dyn.element[i].color.R == (byte) 0 & self.dyn.element[i].color.G == (byte) 0 & self.dyn.element[i].color.B == (byte) 0)
          {
             let mut local3: &Graphics = &g;
            bitmap2: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(self.dyn.element[i].customBitmapFunction, self.dyn.element[i].w, self.dyn.element[i].h, relChange: -999);
             let mut local4: &Bitmap = &bitmap2;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            DrawMod.DrawSimple( local3,  local4, x, y);
          }
          else if (self.dyn.element[i].color.A != byte.MaxValue | self.dyn.element[i].color.R != byte.MaxValue | self.dyn.element[i].color.G != byte.MaxValue | self.dyn.element[i].color.B != byte.MaxValue)
          {
             let mut local5: &Graphics = &g;
            bitmap3: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(self.dyn.element[i].customBitmapFunction, self.dyn.element[i].w, self.dyn.element[i].h, true, self.dyn.element[i].subtype);
             let mut local6: &Bitmap = &bitmap3;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            double r =  ( self.dyn.element[i].color.R /  byte.MaxValue) - 1.0;
            double g2 =  ( self.dyn.element[i].color.G /  byte.MaxValue) - 1.0;
            double b =  ( self.dyn.element[i].color.B /  byte.MaxValue) - 1.0;
            double a =  ( self.dyn.element[i].color.A /  byte.MaxValue);
            DrawMod.Draw( local5,  local6, x, y,  r,  g2,  b,  a);
          }
          else
          {
             let mut local7: &Graphics = &g;
            bitmap4: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(self.dyn.element[i].customBitmapFunction, self.dyn.element[i].w, self.dyn.element[i].h, true, self.dyn.element[i].subtype);
             let mut local8: &Bitmap = &bitmap4;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            DrawMod.DrawSimple( local7,  local8, x, y);
          }
        }
        else if (self.dyn.element[i].customBitmapFunction3 > 0)
        {
          let mut stringListById1: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 311, 0, 0));
          let mut stringListById2: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 306, 0, 0));
          let mut peopleById: i32 = self.game.HandyFunctionsObj.GetPeopleByID(self.dyn.element[i].customBitmapFunction3);
          let mut tv0: i32 = self.game.Data.PeopleObj[peopleById].tv0;
          let mut tv1: i32 = self.game.Data.PeopleObj[peopleById].tv1;
          let mut idValue: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById2].GetData(0, tv0, 2)));
          let mut isUniformEventPic: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, idValue, 3, tv1, 4)));
          let mut isAllowHair: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, idValue, 3, tv1, 6)));
          let mut num: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, idValue, 3, tv1, 5)));
          let mut isPeoplePortraitGroup: i32 =  Math.Round(Conversion.Val(self.game.Data.StringListObj[stringListById1].GetData2(0, idValue, 3, tv1, 7)));
          let mut ox: i32 = self.dyn.element[i].ox;
          let mut sfNr: i32 = self.dyn.element[i].oy;
          if (sfNr < 1)
            sfNr = -1;
          if (self.dyn.element[i].color.R == (byte) 0 & self.dyn.element[i].color.G == (byte) 0 & self.dyn.element[i].color.B == (byte) 0)
          {
             let mut local9: &Graphics = &g;
            bitmap5: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(-1, self.dyn.element[i].w, self.dyn.element[i].h, relChange: -999, isPeoplePortraitGroup: isPeoplePortraitGroup, isPeopleId: peopleById, isPeopleType: tv1, isRegId: ox, isAllowHair: isAllowHair, isUniformEventPic: isUniformEventPic, sfNr: sfNr);
             let mut local10: &Bitmap = &bitmap5;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            DrawMod.DrawSimple( local9,  local10, x, y);
          }
          else if (self.dyn.element[i].color.A != byte.MaxValue | self.dyn.element[i].color.R != byte.MaxValue | self.dyn.element[i].color.G != byte.MaxValue | self.dyn.element[i].color.B != byte.MaxValue)
          {
             let mut local11: &Graphics = &g;
            bitmap6: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(-1, self.dyn.element[i].w, self.dyn.element[i].h, relChange: -999, isPeoplePortraitGroup: isPeoplePortraitGroup, isPeopleId: peopleById, isPeopleType: tv1, isRegId: ox, isAllowHair: isAllowHair, isUniformEventPic: isUniformEventPic, sfNr: sfNr);
             let mut local12: &Bitmap = &bitmap6;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            double r =  ( self.dyn.element[i].color.R /  byte.MaxValue) - 1.0;
            double g3 =  ( self.dyn.element[i].color.G /  byte.MaxValue) - 1.0;
            double b =  ( self.dyn.element[i].color.B /  byte.MaxValue) - 1.0;
            double a =  ( self.dyn.element[i].color.A /  byte.MaxValue);
            DrawMod.Draw( local11,  local12, x, y,  r,  g3,  b,  a);
          }
          else
          {
             let mut local13: &Graphics = &g;
            bitmap7: Bitmap = self.game.CustomBitmapObj.DrawLeaderPortrait(-1, self.dyn.element[i].w, self.dyn.element[i].h, relChange: -999, isPeoplePortraitGroup: isPeoplePortraitGroup, isPeopleId: peopleById, isPeopleType: tv1, isRegId: ox, isAllowHair: isAllowHair, isUniformEventPic: isUniformEventPic, sfNr: sfNr);
             let mut local14: &Bitmap = &bitmap7;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            DrawMod.DrawSimple( local13,  local14, x, y);
          }
        }
        else if (self.dyn.element[i].customBitmapFunction2 > 0)
        {
          let mut sfTypeById: i32 = self.game.HandyFunctionsObj.GetSFTypeByID(self.dyn.element[i].customBitmapFunction2);
          if (sfTypeById > -1)
          {
            objBitmap: Bitmap = self.game.CustomBitmapObj.DrawSFTypeGraphic(sfTypeById, self.dyn.element[i].ox == 1, self.dyn.element[i].oy, self.dyn.element[i].ow, -1);
            if (self.dyn.element[i].w == 76)
            {
              let mut num3: i32 = 0;
              let mut num4: i32 = 2;
              let mut w: i32 = 76;
              let mut h: i32 = 64;
              let mut width: i32 = objBitmap.Width;
              let mut height: i32 = objBitmap.Height;
              if (width > w | height > h)
              {
                if ( width /  w >  height /  h)
                {
                  float num5 =  w /  width;
                  let mut num6: i32 =  Math.Round( ( h -  height * num5));
                  num4 +=  Math.Round( num6 / 2.0);
                  h -= num6;
                }
                else
                {
                  float num7 =  h /  height;
                  let mut num8: i32 =  Math.Round( ( w -  width * num7));
                  num3 +=  Math.Round( num8 / 2.0);
                  w -= num8;
                }
                DrawMod.DrawScaled( g,  objBitmap, self.dyn.element[i].x + num3, self.dyn.element[i].y + num4, w, h);
              }
              else
                DrawMod.DrawSimple( g,  objBitmap, self.dyn.element[i].x + num3 +  Math.Round( (w - width) / 2.0), self.dyn.element[i].y + num4 +  Math.Round( (h - height) / 2.0));
            }
            else if (self.dyn.element[i].w < objBitmap.Width)
            {
              let mut num9: i32 = 0;
              let mut num10: i32 = 2;
              let mut w: i32 = self.dyn.element[i].w;
              let mut h: i32 = self.dyn.element[i].h;
              let mut width: i32 = objBitmap.Width;
              let mut height: i32 = objBitmap.Height;
              if (width > w | height > h)
              {
                if ( width /  w >  height /  h)
                {
                  float num11 =  w /  width;
                  let mut num12: i32 =  Math.Round( ( h -  height * num11));
                  num10 +=  Math.Round( num12 / 2.0);
                  h -= num12;
                }
                else
                {
                  float num13 =  h /  height;
                  let mut num14: i32 =  Math.Round( ( w -  width * num13));
                  num9 +=  Math.Round( num14 / 2.0);
                  w -= num14;
                }
                DrawMod.DrawScaled( g,  objBitmap, self.dyn.element[i].x + num9, self.dyn.element[i].y + num10, w, h);
              }
              else
                DrawMod.DrawSimple( g,  objBitmap, self.dyn.element[i].x + num9 +  Math.Round( (w - width) / 2.0), self.dyn.element[i].y + num10 +  Math.Round( (h - height) / 2.0));
            }
            else if (self.dyn.element[i].color.R == (byte) 0 & self.dyn.element[i].color.G == (byte) 0 & self.dyn.element[i].color.B == (byte) 0)
              DrawMod.DrawSimple( g,  objBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
            else if (self.dyn.element[i].color.A != byte.MaxValue | self.dyn.element[i].color.R != byte.MaxValue | self.dyn.element[i].color.G != byte.MaxValue | self.dyn.element[i].color.B != byte.MaxValue)
              DrawMod.Draw( g,  objBitmap, self.dyn.element[i].x, self.dyn.element[i].y,  self.dyn.element[i].color.R /  byte.MaxValue - 1f,  self.dyn.element[i].color.G /  byte.MaxValue - 1f,  self.dyn.element[i].color.B /  byte.MaxValue - 1f,  self.dyn.element[i].color.A /  byte.MaxValue);
            else
              DrawMod.DrawSimple( g,  objBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
          }
        }
        else if (self.dyn.element[i].bitmapSlot > 0 & self.dyn.element[i].bitmapSlot < 59999)
        {
          if (self.dyn.element[i].color.R != byte.MaxValue | self.dyn.element[i].color.G != byte.MaxValue | self.dyn.element[i].color.B != byte.MaxValue)
          {
             let mut local15: &Graphics = &g;
            bitmap8: Bitmap = BitmapStore.GetBitmap(self.dyn.element[i].bitmapSlot);
             let mut local16: &Bitmap = &bitmap8;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            let mut w: i32 = self.dyn.element[i].w;
            let mut h: i32 = self.dyn.element[i].h;
            let mut width: i32 = BitmapStore.GetWidth(self.dyn.element[i].bitmapSlot);
            let mut origh: i32 = BitmapStore.Getheight(self.dyn.element[i].bitmapSlot);
            double r =  ( self.dyn.element[i].color.R /  byte.MaxValue);
            double g4 =  ( self.dyn.element[i].color.G /  byte.MaxValue);
            double b =  ( self.dyn.element[i].color.B /  byte.MaxValue);
            double a =  ( self.dyn.element[i].color.A /  byte.MaxValue);
            DrawMod.DrawScaledColorized2( local15,  local16, x, y, w, h, width, origh,  r,  g4,  b,  a);
          }
          else
          {
             let mut local17: &Graphics = &g;
            bitmap9: Bitmap = BitmapStore.GetBitmap(self.dyn.element[i].bitmapSlot);
             let mut local18: &Bitmap = &bitmap9;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            let mut w: i32 = self.dyn.element[i].w;
            let mut h: i32 = self.dyn.element[i].h;
            DrawMod.DrawScaled( local17,  local18, x, y, w, h);
          }
        }
        else if (self.dyn.element[i].smallgfx > 0)
        {
          if (BitmapStore.GetWidth(self.game.Data.SmallPicNr[self.dyn.element[i].smallgfx]) > self.dyn.element[i].w)
          {
            if (self.dyn.element[i].h == 0)
              self.dyn.element[i].h =  Math.Round( BitmapStore.Getheight(self.game.Data.SmallPicNr[self.dyn.element[i].smallgfx]) * ( self.dyn.element[i].w /  BitmapStore.GetWidth(self.game.Data.SmallPicNr[self.dyn.element[i].smallgfx])));
             let mut local19: &Graphics = &g;
            bitmap10: Bitmap = BitmapStore.GetBitmap(self.game.Data.SmallPicNr[self.dyn.element[i].smallgfx]);
             let mut local20: &Bitmap = &bitmap10;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            let mut w: i32 = self.dyn.element[i].w;
            let mut h: i32 = self.dyn.element[i].h;
            DrawMod.DrawScaled( local19,  local20, x, y, w, h);
          }
          else if (BitmapStore.GetWidth(self.game.Data.SmallPicNr[self.dyn.element[i].smallgfx]) < self.dyn.element[i].w)
          {
             let mut local21: &Graphics = &g;
            bitmap11: Bitmap = BitmapStore.GetBitmap(self.game.Data.SmallPicNr[self.dyn.element[i].smallgfx], 1);
             let mut local22: &Bitmap = &bitmap11;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            DrawMod.DrawSimple( local21,  local22, x, y);
          }
          else
          {
             let mut local23: &Graphics = &g;
            bitmap1 = BitmapStore.GetBitmap(self.game.Data.SmallPicNr[self.dyn.element[i].smallgfx]);
             let mut local24: &Bitmap = &bitmap1;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            DrawMod.DrawSimple( local23,  local24, x, y);
          }
        }
        else if (self.dyn.element[i].tempPicture.Length > 1)
        {
          if (self.dyn.element[i].w > 0)
          {
            if (self.dyn.element[i].color.A != byte.MaxValue | self.dyn.element[i].color.R != byte.MaxValue | self.dyn.element[i].color.G != byte.MaxValue | self.dyn.element[i].color.B != byte.MaxValue)
              DrawMod.DrawScaledColorized2( g,  self.bmp[self.bmpLink[i]], self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h, self.bmp[self.bmpLink[i]].Width, self.bmp[self.bmpLink[i]].Height,  self.dyn.element[i].color.R /  byte.MaxValue,  self.dyn.element[i].color.G /  byte.MaxValue,  self.dyn.element[i].color.B /  byte.MaxValue,  self.dyn.element[i].color.A /  byte.MaxValue);
            else if (self.bmpLink[i] > -1)
              DrawMod.DrawScaled( g,  self.bmp[self.bmpLink[i]], self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
          }
        }
        else if (self.dyn.element[i].ow > 0 & self.dyn.element[i].eventPicture > 0)
        {
           let mut local25: &Graphics = &g;
          bitmap12: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]);
           let mut local26: &Bitmap = &bitmap12;
          rectangle = Rectangle::new(self.dyn.element[i].ox, self.dyn.element[i].oy, self.dyn.element[i].ow, self.dyn.element[i].oh);
          let mut srcrect: &Rectangle = &rectangle
          trect = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
          let mut destrect: &Rectangle = &trect
          DrawMod.DrawSimplePart2( local25,  local26, srcrect, destrect);
        }
        else if (self.dyn.element[i].w > 0 & self.dyn.element[i].eventPicture > 0)
        {
          if (self.dyn.element[i].color.A != byte.MaxValue)
          {
             let mut local27: &Graphics = &g;
            bitmap13: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]);
             let mut local28: &Bitmap = &bitmap13;
            rectangle = Rectangle::new(0, 0, BitmapStore.GetWidth(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]), BitmapStore.Getheight(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]));
            let mut srcrect: &Rectangle = &rectangle
            trect = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            let mut destrect: &Rectangle = &trect
            double alpha =  ( self.dyn.element[i].color.A /  byte.MaxValue);
            DrawMod.DrawSimplePartAlpha( local27,  local28, srcrect, destrect,  alpha);
          }
          else if (self.dyn.element[i].color.R != byte.MaxValue | self.dyn.element[i].color.G != byte.MaxValue | self.dyn.element[i].color.B != byte.MaxValue)
          {
             let mut local29: &Graphics = &g;
            bitmap14: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]);
             let mut local30: &Bitmap = &bitmap14;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            let mut w: i32 = self.dyn.element[i].w;
            let mut h: i32 = self.dyn.element[i].h;
            let mut width: i32 = BitmapStore.GetBitmap(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]).Width;
            let mut height: i32 = BitmapStore.GetBitmap(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]).Height;
            double r =  ( self.dyn.element[i].color.R /  byte.MaxValue);
            double g5 =  ( self.dyn.element[i].color.G /  byte.MaxValue);
            double b =  ( self.dyn.element[i].color.B /  byte.MaxValue);
            DrawMod.DrawScaledColorized2( local29,  local30, x, y, w, h, width, height,  r,  g5,  b,  byte.MaxValue);
          }
          else if (self.dyn.element[i].h > 0)
          {
             let mut local31: &Graphics = &g;
            bitmap15: Bitmap = BitmapStore.GetBitmap(self.game.Data.EventPicNr[self.dyn.element[i].eventPicture]);
             let mut local32: &Bitmap = &bitmap15;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            let mut w: i32 = self.dyn.element[i].w;
            let mut h: i32 = self.dyn.element[i].h;
            DrawMod.DrawScaled( local31,  local32, x, y, w, h);
          }
        }
        else if (Conversions.ToDouble(self.dyn.element[i].value) > 0.0 & Operators.CompareString(self.dyn.element[i].mouseOver, "", false) == 0 && Operators.CompareString(self.dyn.element[i].key, "", false) == 0)
        {
          if (Conversions.ToDouble(self.dyn.element[i].value) > 0.0 & self.dyn.element[i].x > 0 & Conversions.ToDouble(self.dyn.element[i].value) <=  self.game.Data.UnitCounter)
          {
            self.game.CustomBitmapObj.DrawUnitBig(Conversions.ToInteger(self.dyn.element[i].value), toG: g, tx: self.dyn.element[i].x, ty: self.dyn.element[i].y);
          }
          else
          {
            let mut num15: i32 = num15;
          }
        }
        if (!Information.IsNothing( self.dyn.element[i].mouseOver) && firstCall & self.dyn.element[i].mouseOver.Length > 1)
        {
          if (self.dyn.element[i].eventNr > 0)
          {
            rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            trect = rectangle;
            self.AddMouse( trect, "", self.dyn.element[i].mouseOver, i, 9);
          }
          else
          {
            rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            trect = rectangle;
            self.AddMouse( trect, "", self.dyn.element[i].mouseOver, i);
          }
        }
      }
      if (self.dyn.element[i].type == UDSType.Button)
      {
        let mut index: i32 = self.game.AddDynFont(self.dyn.element[i].fontName, self.dyn.element[i].fontSize, self.dyn.element[i].fontStyle);
        if (index > -1 & self.backBitmapCounter < 199 & !self.dyn.element[i].hidden)
        {
          if (firstCall)
          {
            self += 1.backBitmapCounter;
            self.backBmp[self.backBitmapCounter] = new Bitmap(self.dyn.element[i].w, self.dyn.element[i].h, PixelFormat.Format32bppPArgb);
            self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
            Expression = Graphics.FromImage((Image) self.backBmp[self.backBitmapCounter]);
            Expression.CompositingMode = CompositingMode.SourceCopy;
            Graphics graphics = Expression;
            paper: Bitmap = self.paper;
            rectangle = Rectangle::new(0, 0, self.backBmp[self.backBitmapCounter].Width, self.backBmp[self.backBitmapCounter].Height);
            let mut destRect: &Rectangle = &rectangle
            trect = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            let mut srcRect: &Rectangle = &trect
            graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
            Expression.CompositingMode = CompositingMode.SourceOver;
            self.backBmpLink[i] = self.backBitmapCounter;
          }
          else
            g.DrawImage((Image) self.backBmp[self.backBmpLink[i]], self.dyn.element[i].x, self.dyn.element[i].y);
          texty: String = self.dyn.element[i].texty;
          let mut w: i32 = self.dyn.element[i].w;
          mouseOver: String = self.dyn.element[i].mouseOver;
          bitmap1 = (Bitmap) null;
           let mut local: &Bitmap = &bitmap1;
          let mut num: i32 = self.dyn.element[i].grayed == 1 ? 1 : 0;
          let mut h: i32 = self.dyn.element[i].h;
          let mut fontSize: i32 = self.dyn.element[i].fontSize;
          usefont: Font = self.game.DynFont[index];
          let mut subtype: i32 = self.dyn.element[i].subtype;
          TextButtonPartClass textButtonPartClass = new TextButtonPartClass(texty, w, mouseOver,  local, tinactive: (num != 0), theight: h, tfontsize: fontSize, usefont: usefont, tudsButton: true, tudsButtonSubType: subtype);
          if (high)
            textButtonPartClass.PaintOverlay();
          else
            textButtonPartClass.Paint();
          g.DrawImage((Image) textButtonPartClass.OwnBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
          if (firstCall & self.dyn.element[i].grayed == 0)
          {
            rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
            trect = rectangle;
            self.AddMouse( trect, "", self.dyn.element[i].mouseOver, i, 1);
          }
          else if (firstCall & self.dyn.element[i].grayed == 1)
          {
            if (self.dyn.element[i].parentElement > -1)
            {
              rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
              trect = rectangle;
              self.AddMouse( trect, "", self.dyn.element[i].mouseOver, i, 1);
            }
            else
            {
              rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
              trect = rectangle;
              self.AddMouse( trect, "", self.dyn.element[i].mouseOver, ttype: -1);
            }
          }
        }
      }
      if (self.dyn.element[i].type == UDSType.Line)
        DrawMod.drawLine( g, self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h, self.dyn.element[i].color, Math.Max(1, self.dyn.element[i].lineHeight));
      if (self.dyn.element[i].type == UDSType.Slider)
      {
        let mut index: i32 = self.game.AddDynFont(self.dyn.element[i].fontName, self.dyn.element[i].fontSize, self.dyn.element[i].fontStyle);
        if (index > -1)
        {
           let mut local33: &Graphics = &g;
          bitmap1 = BitmapStore.GetBitmap(self.game.PAPERBACK2);
           let mut local34: &Bitmap = &bitmap1;
          let mut x: i32 = self.dyn.element[i].x;
          let mut y: i32 = self.dyn.element[i].y;
          let mut w1: i32 = self.dyn.element[i].w;
          let mut lineHeight1: i32 = self.dyn.element[i].lineHeight;
          DrawMod.DrawScaled( local33,  local34, x, y, w1, lineHeight1);
          DrawMod.DrawBlock( g, self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].lineHeight, 0, 0, 0, 32);
          str1: String = self.dyn.element[i].texty.Replace("<value>", self.dyn.element[i].value.ToString()).Replace(" 000", " 0").Replace(" 00", " 0");
          if (self.game.Data.Product == 7)
          {
            let mut Start: i32 = Strings.InStr(str1, "[[");
            if (Start > 0)
            {
              let mut num16: i32 = Strings.InStr(Start, str1, "]]");
              if (num16 > 0 & num16 > Start)
              {
                str2: String = Strings.Mid(str1, Start, num16 - Start + 2);
                str3: String = Strings.Mid(str2, 3, str2.Length - 4);
                let mut stringListById3: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 169, 0, 0));
                let mut stringListById4: i32 = self.game.HandyFunctionsObj.GetStringListByID(self.game.EventRelatedObj.CheckStringlistID("SE_Data", 168, 0, 0));
                eventRelatedObj: EventRelatedClass = self.game.EventRelatedObj;
                let mut id1: i32 = self.game.Data.StringListObj[stringListById4].ID;
                let mut id2: i32 = self.game.Data.StringListObj[stringListById3].ID;
                logicString: String = str3;
                Random random = (Random) null;
                 Random local35 =  random;
                let mut num17: i32 = eventRelatedObj.CheckLogicStringStart(id1, id2, logicString, 0,  local35);
                str1 = str1.Replace(str2, num17.ToString());
              }
            }
          }
          let mut game: GameClass = self.game;
          let mut w2: i32 = self.dyn.element[i].w;
          let mut trows: i32 =  Math.Round( self.dyn.element[i].h /  self.dyn.element[i].lineHeight);
          tfont: Font = self.game.DynFont[index];
          tText: String = str1;
          let mut lineHeight2: i32 = self.dyn.element[i].lineHeight;
          bitmap1 = (Bitmap) null;
           let mut local36: &Bitmap = &bitmap1;
          TextAreaClass2 textAreaClass2 = new TextAreaClass2(game, w2, trows, tfont, tText, lineHeight2,  local36, tWithoutScrollbars: true, tWithoutFrame: true, cola: ( byte.MaxValue), tshadow: false, tminimalHeight: true);
          textAreaClass2.Paint();
          g.DrawImage((Image) textAreaClass2.OwnBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
          textAreaClass2.Dispose();
          NumberSliderSubPartClassUDS sliderSubPartClassUds = new NumberSliderSubPartClassUDS(self.game, self.dyn.element[i].w, self.dyn.element[i].h - self.dyn.element[i].lineHeight, self.dyn.element[i].minvalue, self.dyn.element[i].maxvalue, Conversions.ToInteger(self.dyn.element[i].value));
          if (high)
            sliderSubPartClassUds.PaintOverlay();
          else
            sliderSubPartClassUds.Paint();
          g.DrawImage((Image) sliderSubPartClassUds.OwnBitmap, self.dyn.element[i].x, self.dyn.element[i].y + self.dyn.element[i].lineHeight);
          if (firstCall & self.dyn.element[i].grayed == 0)
          {
            rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y + self.dyn.element[i].lineHeight, self.dyn.element[i].w, self.dyn.element[i].h - self.dyn.element[i].lineHeight);
            trect = rectangle;
            self.AddMouse( trect, "", self.dyn.element[i].mouseOver, i, 3);
          }
          sliderSubPartClassUds.Dispose();
        }
      }
      if (self.dyn.element[i].type == UDSType.Flag & self.backBitmapCounter < 199)
      {
        if (firstCall)
        {
          self += 1.backBitmapCounter;
          self.backBmp[self.backBitmapCounter] = new Bitmap(self.dyn.element[i].w, self.dyn.element[i].h, PixelFormat.Format32bppPArgb);
          self.backbitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
          Expression = Graphics.FromImage((Image) self.backBmp[self.backBitmapCounter]);
          Expression.CompositingMode = CompositingMode.SourceCopy;
          Graphics graphics = Expression;
          paper: Bitmap = self.paper;
          rectangle = Rectangle::new(0, 0, self.backBmp[self.backBitmapCounter].Width, self.backBmp[self.backBitmapCounter].Height);
          let mut destRect: &Rectangle = &rectangle
          trect = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
          let mut srcRect: &Rectangle = &trect
          graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
          Expression.CompositingMode = CompositingMode.SourceOver;
          self.backBmpLink[i] = self.backBitmapCounter;
        }
        else
          g.DrawImage((Image) self.backBmp[self.backBmpLink[i]], self.dyn.element[i].x, self.dyn.element[i].y);
        if (self.dyn.element[i].h < 35)
        {
          MarcRadioPartClass2 marcRadioPartClass2;
          if (self.dyn.element[i].flagged)
          {
            mouseOver: String = self.dyn.element[i].mouseOver;
            bitmap1 = (Bitmap) null;
             let mut local: &Bitmap = &bitmap1;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            marcRadioPartClass2 = new MarcRadioPartClass2(0, true, mouseOver,  local, x, y, true);
          }
          else
          {
            mouseOver: String = self.dyn.element[i].mouseOver;
            bitmap1 = (Bitmap) null;
             let mut local: &Bitmap = &bitmap1;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            marcRadioPartClass2 = new MarcRadioPartClass2(0, false, mouseOver,  local, x, y, true);
          }
          if (high)
            marcRadioPartClass2.PaintOverlay();
          else
            marcRadioPartClass2.Paint();
          g.DrawImage((Image) marcRadioPartClass2.OwnBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
        }
        else
        {
          MarcRadioPartClass marcRadioPartClass;
          if (self.dyn.element[i].flagged)
          {
            mouseOver: String = self.dyn.element[i].mouseOver;
            bitmap1 = (Bitmap) null;
             let mut local: &Bitmap = &bitmap1;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            marcRadioPartClass = new MarcRadioPartClass(0, true, mouseOver,  local, x, y, true);
          }
          else
          {
            mouseOver: String = self.dyn.element[i].mouseOver;
            bitmap1 = (Bitmap) null;
             let mut local: &Bitmap = &bitmap1;
            let mut x: i32 = self.dyn.element[i].x;
            let mut y: i32 = self.dyn.element[i].y;
            marcRadioPartClass = new MarcRadioPartClass(0, false, mouseOver,  local, x, y, true);
          }
          if (high)
            marcRadioPartClass.PaintOverlay();
          else
            marcRadioPartClass.Paint();
          g.DrawImage((Image) marcRadioPartClass.OwnBitmap, self.dyn.element[i].x, self.dyn.element[i].y);
        }
        if (firstCall)
        {
          rectangle = Rectangle::new(self.dyn.element[i].x, self.dyn.element[i].y, self.dyn.element[i].w, self.dyn.element[i].h);
          trect = rectangle;
          self.AddMouse( trect, "", self.dyn.element[i].mouseOver, i, 2);
        }
      }
      if (!firstCall || Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub Paint: Bitmap()
    {
      SizeF sizeF = SizeF::new();
      Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
      if (self.lastY != self.curY)
      {
        let mut num: i32 = self.lastY - self.curY;
        let mut mouseCounter: i32 = self.MouseCounter;
        for (let mut index1: i32 = 0; index1 <= mouseCounter; index1 += 1)
        {
          Rectangle[] mouseRect = self.MouseRect;
          Rectangle[] rectangleArray = mouseRect;
          let mut index2: i32 = index1;
          let mut index3: i32 = index2;
          rectangleArray[index3].Y = mouseRect[index2].Y + num;
        }
        self.lastY = self.curY;
      }
      if (!Information.IsNothing( self.backbitmap))
      {
        objGraphics.CompositingMode = CompositingMode.SourceCopy;
        DrawMod.DrawSimpleFast( objGraphics,  self.backbitmap,  self.OwnBitmap, 0, 0);
        objGraphics.CompositingMode = CompositingMode.SourceOver;
      }
      Graphics graphics = objGraphics;
      paper: Bitmap = self.paper;
      Rectangle rectangle1 = Rectangle::new(0, 0, self.Width, self.Height);
      let mut destRect: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(0, self.curY, self.Width, self.Height);
      let mut srcRect: &Rectangle = &rectangle2
      graphics.DrawImage((Image) paper, destRect, srcRect, GraphicsUnit.Pixel);
      if (!self.alwaysBlockScrollBar && self.maxY > self.Height)
      {
        let mut x1: i32 = self.Width - 20;
        let mut num: i32 =  Math.Round( (self.Height - 16) * ( self.curY /  (self.maxY - self.Height)) + 8.0);
        if (num > self.Height - 16)
          num = self.Height - 16;
         let mut local1: &Graphics = &objGraphics;
        bitmap: Bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
         let mut local2: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 3, 20, 4);
        let mut srcrect1: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1, 3, 20, self.Height);
        let mut destrect1: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local1,  local2, srcrect1, destrect1);
         let mut local3: &Graphics = &objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
         let mut local4: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 0, 20, 3);
        let mut srcrect2: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1, 0, 20, 3);
        let mut destrect2: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local3,  local4, srcrect2, destrect2);
         let mut local5: &Graphics = &objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBACK);
         let mut local6: &Bitmap = &bitmap;
        rectangle2 = Rectangle::new(0, 7, 20, 3);
        let mut srcrect3: &Rectangle = &rectangle2
        rectangle1 = Rectangle::new(x1, self.Height - 8, 20, 3);
        let mut destrect3: &Rectangle = &rectangle1
        DrawMod.DrawSimplePart2( local5,  local6, srcrect3, destrect3);
         let mut local7: &Graphics = &objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTUP);
         let mut local8: &Bitmap = &bitmap;
        let mut x2: i32 = x1;
        DrawMod.DrawSimple( local7,  local8, x2, 0);
         let mut local9: &Graphics = &objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTDOWN);
         let mut local10: &Bitmap = &bitmap;
        let mut x3: i32 = x1;
        let mut y1: i32 = self.Height - 8;
        DrawMod.DrawSimple( local9,  local10, x3, y1);
         let mut local11: &Graphics = &objGraphics;
        bitmap = BitmapStore.GetBitmap(DrawMod.TGame.LISTBLOCK);
         let mut local12: &Bitmap = &bitmap;
        let mut x4: i32 = x1;
        let mut y2: i32 = num;
        DrawMod.DrawSimple( local11,  local12, x4, y2);
      }
      return self.OwnBitmap;
    }

    pub fn ShiftDown()
    {
      if (self.maxY <= self.Height)
        return;
      self.curY += 20;
      if (self.curY <= self.maxY - self.Height)
        return;
      self.curY = self.maxY - self.Height;
    }

    pub fn ShiftUp()
    {
      if (self.maxY <= self.Height)
        return;
      self.curY -= 20;
      if (0 <= self.curY)
        return;
      self.curY = 0;
    }

    pub fn HandleBLOCKEDMouseUp(x: i32, y: i32) => self.HandleMouseUp(x, y) -> i32;

    pub fn HandleMouseUp(x: i32, y: i32) -> i32
    {
      if (self.clickscroll == 1 | self.Scroller)
      {
        self.clickscroll = 0;
        self.Scroller = false;
        self.scrollelementclicked = -1;
        self.scrollelementclicked2 = -1;
        return 1;
      }
      if (self.scrollelementclicked > -1 && self.dyn.element[self.scrollelementclicked].eventNr > 0)
      {
        let mut scrollelementclicked: i32 = self.scrollelementclicked;
        if (self.dyn.element[scrollelementclicked].flagged)
          self.SetHiddenAndBaseData(scrollelementclicked, self.dyn.element[scrollelementclicked].key, self.dyn.element[scrollelementclicked].value);
        if (!self.dyn.element[scrollelementclicked].flagged)
          self.SetHiddenAndBaseData(scrollelementclicked, self.dyn.element[scrollelementclicked].key, Conversions.ToString(0));
        return self.dyn.element[scrollelementclicked].eventNr;
      }
      self.scrollelementclicked = -1;
      if (self.scrollelementclicked2 > -1 && self.dyn.element[self.scrollelementclicked2].eventNr > 0)
      {
        let mut scrollelementclicked2: i32 = self.scrollelementclicked2;
        if (self.dyn.element[scrollelementclicked2].flagged)
          self.SetHiddenAndBaseData(scrollelementclicked2, self.dyn.element[scrollelementclicked2].key, self.dyn.element[scrollelementclicked2].value);
        if (!self.dyn.element[scrollelementclicked2].flagged)
          self.SetHiddenAndBaseData(scrollelementclicked2, self.dyn.element[scrollelementclicked2].key, Conversions.ToString(0));
        return self.dyn.element[scrollelementclicked2].eventNr;
      }
      self.scrollelementclicked2 = -1;
      return -1;
    }

    pub fn SetHiddenAndBaseData(elementSlotClicked: i32, tkey: String, tval: String)
    {
      if (DrawMod.TGame.EmpireStyle)
        SoundMod.PlayAWave(DrawMod.TGame.AppPath + "sound/interface/click.wav",  DrawMod.TGame.EditObj);
      self.game.EditObj.UDSClearInput();
      self.game.EditObj.UDSAddInput(tkey, tval);
      let mut elementCounter1: i32 = self.dyn.elementCounter;
      for (let mut index1: i32 = 0; index1 <= elementCounter1; index1 += 1)
      {
        if (!Information.IsNothing( self.dyn.element[index1].key) && elementSlotClicked != index1 & self.dyn.element[index1].key.Length > 0)
        {
          if (self.dyn.element[index1].type == UDSType.Hidden)
            self.game.EditObj.UDSAddInput(self.dyn.element[index1].key, self.dyn.element[index1].value);
          else if (self.dyn.element[index1].type == UDSType.Flag)
          {
            if (self.dyn.element[index1].flagged)
            {
              self.game.EditObj.UDSAddInput(self.dyn.element[index1].key, self.dyn.element[index1].value);
            }
            else
            {
              bool flag = true;
              let mut elementCounter2: i32 = self.dyn.elementCounter;
              for (let mut index2: i32 = 0; index2 <= elementCounter2; index2 += 1)
              {
                if (Operators.CompareString(self.dyn.element[index2].key, self.dyn.element[index1].key, false) == 0 & self.dyn.element[index2].flagged)
                  flag = false;
              }
              if (flag)
                self.game.EditObj.UDSAddInput(self.dyn.element[index1].key, 0);
            }
          }
          else if (self.dyn.element[index1].type == UDSType.Slider)
            self.game.EditObj.UDSAddInput(self.dyn.element[index1].key, self.dyn.element[index1].value);
        }
      }
    }

    pub fn Click(x: i32, y: i32, let mut b: i32 = 1) -> i32
    {
      self.scrollelementclicked = -1;
      self.scrollelementclicked2 = -1;
      let mut mouseCounter1: i32 = self.MouseCounter;
      for (let mut index1: i32 = 0; index1 <= mouseCounter1; index1 += 1)
      {
        if (x > self.MouseRect[index1].X & x < self.MouseRect[index1].X + self.MouseRect[index1].Width && y > self.MouseRect[index1].Y & y < self.MouseRect[index1].Y + self.MouseRect[index1].Height)
        {
          float a1 =  self.MouseData[index1];
          if ( a1 > -1.0 && !Information.IsNothing( self.dyn.element[ Math.Round( a1)].key) & (self.MouseType[index1] == 0 | self.MouseType[index1] == 1))
          {
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_RENAMEUNIT", false) == 0 &  Math.Round(Conversion.Val(self.dyn.element[ Math.Round( a1)].value)) > -1)
            {
              self.game.EditObj.interfaceCue = 2;
              float integer =  Conversions.ToInteger(self.dyn.element[ Math.Round( a1)].value);
              str: String = Interaction.InputBox("Give new name for unit", "Rename unit", self.game.Data.UnitObj[ Math.Round( integer)].Name);
              self.game.Data.UnitObj[ Math.Round( integer)].Name = str;
              if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                return -1;
            }
            float integer1;
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_TEMPRENAME", false) == 0 &  Math.Round(Conversion.Val(self.dyn.element[ Math.Round( a1)].value)) > -1)
            {
              self.game.EditObj.interfaceCue = 2;
              integer1 =  Conversions.ToInteger(self.dyn.element[ Math.Round( a1)].value);
              self.game.EditObj.tempRenameString = Interaction.InputBox("Give new name", "Rename", self.game.EditObj.tempRenameString);
              if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                return -1;
            }
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_TEMPRENAME2", false) == 0 &  Math.Round(Conversion.Val(self.dyn.element[ Math.Round( a1)].value)) > -1)
            {
              self.game.EditObj.interfaceCue = 2;
              integer1 =  Conversions.ToInteger(self.dyn.element[ Math.Round( a1)].value);
              self.game.EditObj.tempRenameString2 = Interaction.InputBox("Give new name", "Rename", self.game.EditObj.tempRenameString2);
              if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                return -1;
            }
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_TEMPRENAME3", false) == 0 &  Math.Round(Conversion.Val(self.dyn.element[ Math.Round( a1)].value)) > -1)
            {
              self.game.EditObj.interfaceCue = 2;
              integer1 =  Conversions.ToInteger(self.dyn.element[ Math.Round( a1)].value);
              self.game.EditObj.tempRenameString3 = Interaction.InputBox("Give new name", "Rename", self.game.EditObj.tempRenameString3);
              if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                return -1;
            }
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_RENAMEKEY", false) == 0 & self.dyn.element[ Math.Round( a1)].value.Length > 1)
            {
              self.game.EditObj.interfaceCue = 2;
              DefaultResponse: String = "";
              let mut elementCounter1: i32 = self.dyn.elementCounter;
              for (let mut index2: i32 = 0; index2 <= elementCounter1; index2 += 1)
              {
                if (Operators.CompareString(Strings.LCase(self.dyn.element[index2].key), Strings.LCase(self.dyn.element[ Math.Round( a1)].value), false) == 0)
                {
                  DefaultResponse = self.dyn.element[index2].value;
                  break;
                }
              }
              str: String = Interaction.InputBox("Give new name", "Give new name", DefaultResponse);
              let mut elementCounter2: i32 = self.dyn.elementCounter;
              for (let mut elementSlotClicked: i32 = 0; elementSlotClicked <= elementCounter2; elementSlotClicked += 1)
              {
                if (Operators.CompareString(Strings.LCase(self.dyn.element[elementSlotClicked].key), Strings.LCase(self.dyn.element[ Math.Round( a1)].value), false) == 0)
                {
                  self.dyn.element[elementSlotClicked].value = str;
                  self.SetHiddenAndBaseData(elementSlotClicked, self.dyn.element[elementSlotClicked].key, self.dyn.element[elementSlotClicked].value);
                  break;
                }
              }
              if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                return -1;
            }
            color: Color;
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_RECOLORKEY", false) == 0 & self.dyn.element[ Math.Round( a1)].value.Length > 1)
            {
              self.game.EditObj.interfaceCue = 2;
              str1: String = "";
              let mut elementCounter3: i32 = self.dyn.elementCounter;
              for (let mut index3: i32 = 0; index3 <= elementCounter3; index3 += 1)
              {
                if (Operators.CompareString(Strings.LCase(self.dyn.element[index3].key), Strings.LCase(self.dyn.element[ Math.Round( a1)].value), false) == 0)
                {
                  str1 = self.dyn.element[index3].value;
                  break;
                }
              }
              strArray: Vec<String> = str1.Split('#');
              let mut red: i32 =  Math.Round(Conversion.Val(strArray[0]));
              let mut green: i32 =  Math.Round(Conversion.Val(strArray[1]));
              let mut blue: i32 =  Math.Round(Conversion.Val(strArray[2]));
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = Color.FromArgb( byte.MaxValue, red, green, blue);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                color = colorDialog.Color;
                blue =  color.B;
                color = colorDialog.Color;
                green =  color.G;
                color = colorDialog.Color;
                red =  color.R;
              }
              str2: String = red.ToString() + "#" + green.ToString() + "#" + blue.ToString();
              let mut elementCounter4: i32 = self.dyn.elementCounter;
              for (let mut index4: i32 = 0; index4 <= elementCounter4; index4 += 1)
              {
                if (Operators.CompareString(Strings.LCase(self.dyn.element[index4].key), Strings.LCase(self.dyn.element[ Math.Round( a1)].value), false) == 0)
                {
                  self.dyn.element[index4].value = str2;
                  break;
                }
              }
              if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                return -1;
            }
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_RECOLORUNIT", false) == 0 &  Math.Round(Conversion.Val(self.dyn.element[ Math.Round( a1)].value)) > -1)
            {
              self.game.EditObj.interfaceCue = 2;
              float integer2 =  Conversions.ToInteger(self.dyn.element[ Math.Round( a1)].value);
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.UnitObj[ Math.Round( integer2)].Red, self.game.Data.UnitObj[ Math.Round( integer2)].Green, self.game.Data.UnitObj[ Math.Round( integer2)].Blue);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                UnitClass unitClass1 = self.game.Data.UnitObj[ Math.Round( integer2)];
                color = colorDialog.Color;
                let mut b1: i32 =  color.B;
                unitClass1.Blue = b1;
                UnitClass unitClass2 = self.game.Data.UnitObj[ Math.Round( integer2)];
                color = colorDialog.Color;
                let mut g: i32 =  color.G;
                unitClass2.Green = g;
                UnitClass unitClass3 = self.game.Data.UnitObj[ Math.Round( integer2)];
                color = colorDialog.Color;
                let mut r: i32 =  color.R;
                unitClass3.Red = r;
              }
              if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                return -1;
            }
            if (Operators.CompareString(self.dyn.element[ Math.Round( a1)].key, "HARD_SELECTUNIT", false) == 0 &  Math.Round(Conversion.Val(self.dyn.element[ Math.Round( a1)].value)) > -1)
            {
              self.game.EditObj.interfaceCue = 2;
              float integer3 =  Conversions.ToInteger(self.dyn.element[ Math.Round( a1)].value);
              if ( integer3 > -1.0)
              {
                let mut index5: i32 =  Math.Round( integer3);
                let mut index6: i32 = 0;
                if (self.game.Data.UnitObj[index5].X > -1)
                {
                  self.game.SelectX = self.game.Data.UnitObj[index5].X;
                  self.game.SelectY = self.game.Data.UnitObj[index5].Y;
                }
                else
                {
                  self.game.SelectX = self.game.Data.UnitObj[self.game.Data.UnitObj[index5].OnBoard].X;
                  self.game.SelectY = self.game.Data.UnitObj[self.game.Data.UnitObj[index5].OnBoard].Y;
                }
                while (self.game.Data.MapObj[index6].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0] != index5)
                {
                  let mut unit: i32 = self.game.Data.MapObj[index6].HexObj[self.game.SelectX, self.game.SelectY].UnitList[self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter];
                  index6 = 0;
                  if (self.game.Data.MapObj[index6].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter > 0)
                  {
                    for (let mut unitCounter: i32 = self.game.Data.MapObj[index6].HexObj[self.game.SelectX, self.game.SelectY].UnitCounter; unitCounter >= 1; unitCounter += -1)
                      self.game.Data.MapObj[index6].HexObj[self.game.SelectX, self.game.SelectY].UnitList[unitCounter] = self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].UnitList[unitCounter - 1];
                  }
                  self.game.Data.MapObj[index6].HexObj[self.game.SelectX, self.game.SelectY].UnitList[0] = unit;
                }
                self.game.EditObj.UnitSelected = index5;
                self.game.HandyFunctionsObj.SetcornerXY2();
                self.game.EditObj.TempCoordList = CoordList::new();
                if (self.dyn.element[ Math.Round( a1)].eventNr < 0)
                  return -1;
              }
            }
          }
          if (self.MouseType[index1] == 25)
          {
            StringListClass stringListClass = self.game.Data.StringListObj[ Math.Round(Conversion.Val(self.dyn.element[self.MouseData[index1]].texty))];
            let mut num1: i32 = y - self.dyn.element[self.MouseData[index1]].y + self.curY;
            if (num1 <= self.dyn.element[self.MouseData[index1]].lineHeight)
            {
              col: i32;
              if (stringListClass.ColWidth[0] > 0)
              {
                let mut num2: i32 = x - self.dyn.element[self.MouseData[index1]].x;
                let mut num3: i32 = 0;
                col = -1;
                let mut width: i32 = stringListClass.Width;
                for (let mut index7: i32 = 0; index7 <= width; index7 += 1)
                {
                  num3 += stringListClass.ColWidth[index7];
                  if (num2 < num3)
                  {
                    col = index7;
                    break;
                  }
                }
                if (col == -1)
                  col = 0;
              }
              else
              {
                let mut num4: i32 =  Math.Round( self.dyn.element[self.MouseData[index1]].w /  (stringListClass.Width + 1));
                col =  Math.Round(Math.Floor( (x - self.dyn.element[self.MouseData[index1]].x) /  num4));
              }
              stringListClass.Sort(col);
              if (DrawMod.TGame.EmpireStyle)
                SoundMod.PlayAWave(DrawMod.TGame.AppPath + "sound/interface/click.wav",  DrawMod.TGame.EditObj);
              Graphics g = Graphics.FromImage((Image) self.paper);
              self.DrawElement(self.MouseData[index1],  g, false);
              g.Dispose();
            }
            else if (num1 <= self.dyn.element[self.MouseData[index1]].lineHeight * (self.dyn.element[self.MouseData[index1]].rowsPerPage + 1))
            {
              index8: i32;
              if (stringListClass.ColWidth[0] > 0)
              {
                let mut num5: i32 = x - self.dyn.element[self.MouseData[index1]].x;
                let mut num6: i32 = 0;
                index8 = -1;
                let mut width: i32 = stringListClass.Width;
                for (let mut index9: i32 = 0; index9 <= width; index9 += 1)
                {
                  num6 += stringListClass.ColWidth[index9];
                  if (num5 < num6)
                  {
                    index8 = index9;
                    break;
                  }
                }
                if (index8 == -1)
                  index8 = 0;
              }
              else
              {
                let mut num7: i32 =  Math.Round( self.dyn.element[self.MouseData[index1]].w /  (stringListClass.Width + 1));
                index8 =  Math.Round(Math.Floor( (x - self.dyn.element[self.MouseData[index1]].x) /  num7));
              }
              let mut index10: i32 =  Math.Round(Math.Floor( (num1 - self.dyn.element[self.MouseData[index1]].lineHeight) /  self.dyn.element[self.MouseData[index1]].lineHeight)) + self.dyn.element[self.MouseData[index1]].topRow;
              if (index10 <= stringListClass.Length)
              {
                Expression: String = stringListClass.TempDesc[index10, index8];
                if (!Information.IsNothing( Expression))
                {
                  strArray: Vec<String> = Expression.Split('#');
                  if (strArray.GetUpperBound(0) >= 1)
                  {
                    if (Operators.CompareString(strArray[1], "HARDXY", false) == 0)
                    {
                      self.game.EditObj.interfaceCue = 2;
                      self.game.SelectX =  Math.Round(Conversion.Val(strArray[2]));
                      self.game.SelectY =  Math.Round(Conversion.Val(strArray[3]));
                      if (strArray.GetUpperBound(0) >= 4)
                      {
                        self.game.EditObj.SetViewModeExtraNr =  Math.Round(Conversion.Val(strArray[4]));
                        if (self.game.EditObj.SetViewModeExtraNr == 3)
                          self.game.EditObj.se1_SelectAssetButton = -1;
                      }
                      if (strArray.GetUpperBound(0) >= 5)
                      {
                        self.game.EditObj.SetViewModeExtraNr =  Math.Round(Conversion.Val(strArray[4]));
                        if ( self.game.Data.RuleVar[701] > 0.0)
                        {
                          self.game.EditObj.UnitSelected =  Math.Round(Conversion.Val(strArray[5]));
                          ScreenClass screeny = self.game.FormRef.Screeny;
                          System.Type type = typeof (MapWindowClass2);
                           System.Type local =  type;
                          ((MapWindowClass2) screeny.GetWindow( local)).UdsClickUnit(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].X, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Y, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Map, true);
                        }
                      }
                      self.game.HandyFunctionsObj.SetcornerXY2();
                      self.game.EditObj.TempCoordList = CoordList::new();
                      return -1;
                    }
                    let mut num8: i32 =  Math.Round(Conversion.Val(strArray[1]));
                    if (strArray.GetUpperBound(0) >= 3)
                      self.SetHiddenAndBaseData(self.MouseData[index1], strArray[2], strArray[3]);
                    if (strArray.GetUpperBound(0) >= 5)
                      self.SetHiddenAndBaseData(self.MouseData[index1], strArray[4], strArray[5]);
                    if (strArray.GetUpperBound(0) >= 7)
                      self.SetHiddenAndBaseData(self.MouseData[index1], strArray[6], strArray[7]);
                    if (strArray.GetUpperBound(0) >= 9)
                      self.SetHiddenAndBaseData(self.MouseData[index1], strArray[8], strArray[9]);
                    return num8 <= 0 ? -999 : num8;
                  }
                }
              }
            }
          }
          if (self.MouseType[index1] == 1 | self.MouseType[index1] == 9)
          {
            let mut elementSlotClicked: i32 = self.MouseData[index1];
            let mut parentElement: i32 = self.dyn.element[elementSlotClicked].parentElement;
            if (parentElement > -1 & self.dyn.element[elementSlotClicked].grayed == 0)
            {
              if (self.dyn.element[parentElement].type == UDSType.Table)
              {
                if (self.dyn.element[elementSlotClicked].childType == 1)
                  self.dyn.element[parentElement].topRow = 0;
                if (self.dyn.element[elementSlotClicked].childType == 2)
                  self.dyn.element[parentElement].topRow -= self.dyn.element[elementSlotClicked].childData;
                if (self.dyn.element[elementSlotClicked].childType == 4)
                {
                  self.dyn.element[parentElement].topRow = 0;
                  while (self.dyn.element[parentElement].topRow < self.dyn.element[parentElement].totalRows - self.dyn.element[elementSlotClicked].childData)
                    self.dyn.element[parentElement].topRow += self.dyn.element[elementSlotClicked].childData;
                }
                if (self.dyn.element[elementSlotClicked].childType == 3)
                  self.dyn.element[parentElement].topRow += self.dyn.element[elementSlotClicked].childData;
                if (self.dyn.element[parentElement].topRow < 0)
                  self.dyn.element[parentElement].topRow = 0;
                if (self.dyn.element[parentElement].topRow >= self.dyn.element[parentElement].totalRows - 1)
                  self.dyn.element[parentElement].topRow = self.dyn.element[parentElement].totalRows - 1;
                if (DrawMod.TGame.EmpireStyle)
                  SoundMod.PlayAWave(DrawMod.TGame.AppPath + "sound/interface/click.wav",  DrawMod.TGame.EditObj);
                Graphics g = Graphics.FromImage((Image) self.paper);
                self.DrawElement(parentElement,  g, false);
                let mut elementCounter: i32 = self.dyn.elementCounter;
                for (let mut i: i32 = 0; i <= elementCounter; i += 1)
                {
                  if (self.dyn.element[i].parentElement == parentElement)
                  {
                    if (self.dyn.element[i].type == UDSType.Button)
                    {
                      if (self.dyn.element[i].childType == 1)
                        self.dyn.element[i].grayed = self.dyn.element[parentElement].topRow > 0 ? 0 : 1;
                      if (self.dyn.element[i].childType == 2)
                        self.dyn.element[i].grayed = self.dyn.element[parentElement].topRow > 0 ? 0 : 1;
                      if (self.dyn.element[i].childType == 3)
                        self.dyn.element[i].grayed = self.dyn.element[parentElement].topRow < self.dyn.element[parentElement].totalRows - self.dyn.element[elementSlotClicked].childData ? 0 : 1;
                      if (self.dyn.element[i].childType == 4)
                        self.dyn.element[i].grayed = self.dyn.element[parentElement].topRow < self.dyn.element[parentElement].totalRows - self.dyn.element[elementSlotClicked].childData ? 0 : 1;
                    }
                    if (self.dyn.element[i].type == UDSType.TextField)
                    {
                      float num9 =  ( Math.Round(Math.Floor( self.dyn.element[parentElement].topRow /  self.dyn.element[parentElement].rowsPerPage)) + 1);
                      float num10 =  ( Math.Round(Math.Floor( (self.dyn.element[parentElement].totalRows - 1) /  self.dyn.element[parentElement].rowsPerPage)) + 1);
                      self.dyn.element[i].texty = "Page " + num9.ToString() + "/" + num10.ToString();
                    }
                    self.DrawElement(i,  g, false, elementSlotClicked == i);
                  }
                }
              }
              return -1;
            }
            if (self.dyn.element[elementSlotClicked].grayed == 0)
            {
              self.SetHiddenAndBaseData(elementSlotClicked, self.dyn.element[elementSlotClicked].key, self.dyn.element[elementSlotClicked].value);
              return self.dyn.element[elementSlotClicked].eventNr <= 0 ? -999 : self.dyn.element[elementSlotClicked].eventNr;
            }
          }
          if (self.MouseType[index1] == 2)
          {
            float a2 =  self.MouseData[index1];
            if (self.dyn.element[ Math.Round( a2)].grayed < 1)
            {
              if (DrawMod.TGame.EmpireStyle)
                SoundMod.PlayAWave(DrawMod.TGame.AppPath + "sound/interface/click.wav",  DrawMod.TGame.EditObj);
              Graphics g = Graphics.FromImage((Image) self.paper);
              if (self.dyn.element[ Math.Round( a2)].flagged)
              {
                let mut num11: i32 = 0;
                let mut num12: i32 = 0;
                let mut mouseCounter2: i32 = self.MouseCounter;
                for (let mut index11: i32 = 0; index11 <= mouseCounter2; index11 += 1)
                {
                  if (index1 != index11 & self.MouseType[index11] == 2)
                  {
                    float a3 =  self.MouseData[index11];
                    if (self.dyn.element[ Math.Round( a2)].group == self.dyn.element[ Math.Round( a3)].group & self.dyn.element[ Math.Round( a2)].group > 0)
                    {
                      if (self.dyn.element[ Math.Round( a3)].flagged)
                      {
                        num12 += 1;
                        num11 += 1;
                      }
                      else
                        num11 += 1;
                    }
                  }
                }
                if (num12 > 0 | num11 == 0)
                  self.dyn.element[ Math.Round( a2)].flagged = false;
              }
              else
                self.dyn.element[ Math.Round( a2)].flagged = true;
              let mut mouseCounter3: i32 = self.MouseCounter;
              for (let mut index12: i32 = 0; index12 <= mouseCounter3; index12 += 1)
              {
                if (index1 != index12 & self.MouseType[index12] == 2)
                {
                  float a4 =  self.MouseData[index12];
                  if (self.dyn.element[ Math.Round( a2)].group == self.dyn.element[ Math.Round( a4)].group & self.dyn.element[ Math.Round( a2)].group > 0 && self.dyn.element[ Math.Round( a2)].flagged)
                  {
                    self.dyn.element[ Math.Round( a4)].flagged = false;
                    self.DrawElement( Math.Round( a4),  g, false);
                  }
                }
              }
              if (self.dyn.element[ Math.Round( a2)].eventNr > 0)
              {
                if (self.dyn.element[ Math.Round( a2)].flagged)
                  self.SetHiddenAndBaseData( Math.Round( a2), self.dyn.element[ Math.Round( a2)].key, self.dyn.element[ Math.Round( a2)].value);
                if (!self.dyn.element[ Math.Round( a2)].flagged)
                  self.SetHiddenAndBaseData( Math.Round( a2), self.dyn.element[ Math.Round( a2)].key, Conversions.ToString(0));
                return self.dyn.element[ Math.Round( a2)].eventNr;
              }
              self.DrawElement( Math.Round( a2),  g, false);
              return -1;
            }
          }
          if (self.MouseType[index1] == 3)
          {
            Graphics g = Graphics.FromImage((Image) self.paper);
            float a5 =  self.MouseData[index1];
            self.dyn.element[ Math.Round( a5)].flagged = !self.dyn.element[ Math.Round( a5)].flagged;
            let mut num13: i32 =  Math.Round(Math.Max(20.0,  self.MouseRect[index1].Width / 10.0));
            let mut num14: i32 = x - self.MouseRect[index1].X;
            let mut num15: i32 = self.MouseRect[index1].Width - num13;
            num16: i32;
            if (num14 < self.MouseRect[index1].Height)
            {
              num16 = Conversions.ToInteger(self.dyn.element[ Math.Round( a5)].value) - 1;
              if (num16 < self.dyn.element[ Math.Round( a5)].minvalue)
                num16 = self.dyn.element[ Math.Round( a5)].minvalue;
              self.scrollelementclicked2 =  Math.Round( a5);
            }
            else if (num14 > self.MouseRect[index1].Width - self.MouseRect[index1].Height)
            {
              num16 = Conversions.ToInteger(self.dyn.element[ Math.Round( a5)].value) + 1;
              if (num16 > self.dyn.element[ Math.Round( a5)].maxvalue)
                num16 = self.dyn.element[ Math.Round( a5)].maxvalue;
              self.scrollelementclicked2 =  Math.Round( a5);
            }
            else
            {
              if ( num14 <  num13 / 2.0)
                num16 = self.dyn.element[ Math.Round( a5)].minvalue;
              else if ( num14 >  self.MouseRect[index1].Width -  num13 / 2.0)
              {
                num16 = self.dyn.element[ Math.Round( a5)].maxvalue;
              }
              else
              {
                let mut num17: i32 =  Math.Round( num14 -  num13 / 2.0);
                num16 =  Math.Round( (self.dyn.element[ Math.Round( a5)].maxvalue - self.dyn.element[ Math.Round( a5)].minvalue) * ( num17 /  num15)) + self.dyn.element[ Math.Round( a5)].minvalue;
              }
              self.scrollelementclicked =  Math.Round( a5);
            }
            self.dyn.element[ Math.Round( a5)].value = Conversions.ToString(num16);
            if (self.AdjustSliders( Math.Round( a5)))
            {
              float elementCounter =  self.dyn.elementCounter;
              for (float a6 = 0.0f;  a6 <=  elementCounter; a6 += 1)
              {
                if ( a5 !=  a6 & self.dyn.element[ Math.Round( a6)].type == UDSType.Slider & self.dyn.element[ Math.Round( a6)].group == self.dyn.element[ Math.Round( a5)].group)
                  self.DrawElement( Math.Round( a6),  g, false, true);
              }
            }
            self.DrawElement( Math.Round( a5),  g, false, true);
            return -1;
          }
        }
      }
      if (self.alwaysBlockScrollBar || self.maxY <= self.Height || x <= self.Width - 20)
        return -1;
      if (y >= 0 & y <= 8)
      {
        self.curY -= 20;
        self.clickscroll = 0;
        if (0 > self.curY)
          self.curY = 0;
        return -1;
      }
      if (y > self.Height - 8)
      {
        self.curY += 20;
        self.clickscroll = 0;
        if (self.curY > self.maxY - self.Height)
          self.curY = self.maxY - self.Height;
        return -1;
      }
      self.clickscroll = 1;
      self.Scroller = true;
      self.curY =  Math.Round( (self.maxY - self.Height) * ( (y - 8) /  (self.Height - 16)));
      if (0 > self.curY)
        self.curY = 0;
      if (self.curY > self.maxY - self.Height)
        self.curY = self.maxY - self.Height;
      return -1;
    }

    pub bool MouseMove(x: i32, y: i32)
    {
      if (self.alwaysBlockScrollBar || self.clickscroll != 1)
        return false;
      self.clickscroll = 1;
      self.Scroller = true;
      self.clickscroll = 1;
      self.Scroller = true;
      self.curY =  Math.Round( (self.maxY - self.Height) * ( (y - 8) /  (self.Height - 16)));
      if (0 > self.curY)
        self.curY = 0;
      if (self.curY > self.maxY - self.Height)
        self.curY = self.maxY - self.Height;
      return true;
    }

    pub fn unloadAnyStuff()
    {
      let mut index1: i32 = 0;
      do
      {
        if (!Information.IsNothing( self.bmp[index1]))
        {
          self.bmp[index1].Dispose();
          self.bmp[index1] = (Bitmap) null;
        }
        index1 += 1;
      }
      while (index1 <= 29);
      let mut upperBound: i32 = self.backBmp.GetUpperBound(0);
      for (let mut index2: i32 = 0; index2 <= upperBound; index2 += 1)
      {
        if (!Information.IsNothing( self.backBmp[index2]))
        {
          self.backBmp[index2].Dispose();
          self.backBmp[index2] = (Bitmap) null;
        }
      }
    }

    pub fn loadPageStuff()
    {
      str: String = self.game.AppPath + "graphics/";
      self.unloadAnyStuff();
      let mut slot: i32 = 0;
      self.bmpLink = new int[self.dyn.elementCounter + 1];
      let mut elementCounter: i32 = self.dyn.elementCounter;
      for (let mut index: i32 = 0; index <= elementCounter; index += 1)
      {
        self.bmpLink[index] = -1;
        if (!Information.IsNothing( self.dyn.element[index].tempPicture) && self.dyn.element[index].type == UDSType.PictureField & self.dyn.element[index].tempPicture.Length > 1)
        {
          self.bmpLink[index] = -1;
          self.loadSpecificBmp(str + self.dyn.element[index].tempPicture, slot, self.dyn.element[index].rotation);
          self.bmpLink[index] = slot;
          if (self.dyn.element[index].rotation == -1)
            DrawMod.MakeFuzzyBorder( self.bmp[self.bmpLink[index]], 30, 2);
          if (self.dyn.element[index].h == -1)
          {
            self.dyn.element[index].w = self.bmp[self.bmpLink[index]].Width;
            self.dyn.element[index].h = self.bmp[self.bmpLink[index]].Height;
          }
          slot += 1;
        }
      }
    }

    pub fn loadSpecificBmp(s: String, slot: i32, rotate: i32)
    {
      if (!Information.IsNothing( self.bmp[slot]))
      {
        self.bmp[slot].Dispose();
        self.bmp[slot] = (Bitmap) null;
      }
      FileStream fileStream = new FileStream(s, FileMode.Open, FileAccess.Read);
      bitmap1: Bitmap = (Bitmap) Image.FromStream((Stream) fileStream);
      bitmap2: Bitmap = new Bitmap(bitmap1.Width, bitmap1.Height, PixelFormat.Format32bppPArgb);
      Graphics graphics = Graphics.FromImage((Image) bitmap2);
      if (rotate > 0)
      {
        graphics.TranslateTransform( bitmap1.Width / -2f,  bitmap1.Height / -2f);
        graphics.RotateTransform( rotate, MatrixOrder.Append);
        graphics.TranslateTransform( bitmap1.Width / 2f,  bitmap1.Height / 2f, MatrixOrder.Append);
        graphics.TranslateTransform( (bitmap2.Width - bitmap1.Width) / 2f,  (bitmap2.Height - bitmap1.Height) / 2f, MatrixOrder.Append);
        graphics.DrawImage((Image) bitmap1, Rectangle::new(0, 0, bitmap1.Width, bitmap1.Height));
      }
      else
        graphics.DrawImage((Image) bitmap1, Rectangle::new(0, 0, bitmap1.Width, bitmap1.Height));
      graphics.Dispose();
      bitmap2.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      fileStream.Close();
      fileStream.Dispose();
      bitmap1.Dispose();
      self.bmp[slot] = bitmap2;
    }
  }
}
