// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.WindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Imaging;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class WindowClass
  {
    pub SubPartClass[] SubPartList;
    protected SubPartCounter: i32;
    protected SubPartIDCounter: i32;
    protected int[] SubPartX;
    protected int[] SubPartY;
    protected int[] SubPartW;
    protected int[] SubPartH;
    protected int[] SubPartID;
    protected int[] SubPartOverlay;
    protected bool[] SubPartFlag;
    pub OwnBitmap: Bitmap;
    pub BackBitmap: Bitmap;
    pub HeaderString: String;
    pub BackColor: Color;
    pub BackColor2: Color;
     bool NoPartDraw;
     bool DoGradient;
     DoBorders: i32;
     LastOverlaySubPart: i32;
    pub game: GameClass;
    protected bool mapframe;
    protected bool downframe;
    protected bool mainframe;
    protected bool extrabackground;
    pub formref: Form1;
    pub fixshade: bool;
    pub shade: bool;
    pub screenbackref: Bitmap;
    pub screenx: i32;
    pub screeny: i32;
    pub screenw: i32;
    pub screenh: i32;
    pub useparentscreenbitmap: bool;
    pub backspritescaled: bool;
    pub transbacksprite: bool;
    pub WindowClass LowerWindow;
    pub Rectangle LowerRect;
    pub Rectangle ShowRect;
    pub DoShowRect: bool;
    pub Rectangle[] MouseRect;
    pub MouseCounter: i32;
    pub MouseText: Vec<String>;
    pub MouseTitle: Vec<String>;
    pub MouseData: Vec<i32>;
    pub MouseData2: Vec<i32>;
    pub BlockBlit: bool;
    pub MouseInThisWindow: bool;
    pub NewGfx: bool;
    pub Rectangle[] QuickRect;
    pub QuickDrawMode: bool;
    pub QuickRectCount: i32;
    pub QuickRectMax: i32;
    pub OrderResult form3_orderResult;
    pub form3_returnInstruction: bool;
    pub form3_data1: i32;
    pub form3_data2: i32;

    pub fn ClearMouse() => self.MouseCounter = -1;

    pub fn AddQuickRect(Rectangle trect)
    {
      if (self.QuickRectCount >= self.QuickRectMax)
      {
        self.QuickDrawMode = false;
      }
      else
      {
        self += 1.QuickRectCount;
        self.QuickRect[self.QuickRectCount] = trect;
        self.QuickDrawMode = true;
      }
    }

    pub fn ResetQuickRect()
    {
      self.QuickDrawMode = false;
      self.QuickRectCount = -1;
    }

    pub fn AddMouse( Rectangle trect, ttitle: String, ttext: String, let mut tdata: i32 = -1, let mut tdata2: i32 = -1)
    {
      self += 1.MouseCounter;
      self.MouseRect = (Rectangle[]) Utils.CopyArray((Array) self.MouseRect, (Array) Rectangle::new[self.MouseCounter + 1]);
      self.MouseText = (string[]) Utils.CopyArray((Array) self.MouseText, (Array) new string[self.MouseCounter + 1]);
      self.MouseTitle = (string[]) Utils.CopyArray((Array) self.MouseTitle, (Array) new string[self.MouseCounter + 1]);
      self.MouseData = (int[]) Utils.CopyArray((Array) self.MouseData, (Array) new int[self.MouseCounter + 1]);
      self.MouseData2 = (int[]) Utils.CopyArray((Array) self.MouseData2, (Array) new int[self.MouseCounter + 1]);
      self.MouseRect[self.MouseCounter] = trect;
      self.MouseText[self.MouseCounter] = ttext;
      self.MouseTitle[self.MouseCounter] = ttitle;
      self.MouseData[self.MouseCounter] = tdata;
      self.MouseData2[self.MouseCounter] = tdata2;
    }

    pub fn GetMemorySize() -> i32
    {
      let mut memorySize: i32 =  Math.Round( (64 * self.OwnBitmap.Width * self.OwnBitmap.Height) / 8000.0);
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        memorySize += self.SubPartList[index].GetMemorySize();
      return memorySize;
    }

    pub fn SubpartNr(id: i32) -> i32
    {
      if (self.SubPartCounter <= -1)
        return -1;
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (self.SubPartID[index] == id)
          return index;
      }
      return -1;
    }

    pub fn Dispose()
    {
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        self.SubPartList[index].Dispose();
        self.SubPartList[index] =  null;
      }
      self.OwnBitmap.Dispose();
      self.OwnBitmap = (Bitmap) null;
      self.BackBitmap.Dispose();
      self.BackBitmap = (Bitmap) null;
      self.screenbackref = (Bitmap) null;
      self.game = (GameClass) null;
      self.LowerWindow = (WindowClass) null;
      GC.Collect(GC.MaxGeneration, GCCollectionMode.Forced);
      Application.DoEvents();
    }

    pub WindowClass(
       tGame: GameClass,
      w: i32,
      h: i32,
      let mut backcolornr: i32 = -1,
      let mut BackSprite: i32 = -1,
      bool tBackSpriteScaled = false,
      let mut tDoBorders: i32 = 0,
      bool tDoGradient = true,
      tHeaderString: String = "",
      bool tShade = true,
      screenbitmap: Bitmap = null,
      let mut sx: i32 = -1,
      let mut sy: i32 = -1,
      bool tTransBacksprite = false)
    {
      self.QuickRect = Rectangle::new[20];
      self.QuickDrawMode = false;
      self.QuickRectCount = -1;
      self.QuickRectMax = 19;
      self.MouseCounter = -1;
      self.QuickRectCount = -1;
      self.game = tGame;
      self.DoBorders = tDoBorders;
      self.DoGradient = tDoGradient;
      self.HeaderString = tHeaderString;
      self.transbacksprite = tTransBacksprite;
      self.backspritescaled = tBackSpriteScaled;
      self.screenbackref = screenbitmap;
      self.screenx = sx;
      self.screeny = sy;
      self.screenw = w;
      self.screenh = h;
      self.shade = tShade;
      if (backcolornr == 0)
      {
        self.BackColor = Color.FromArgb(14, 70, 140,  byte.MaxValue);
        self.BackColor2 = Color.FromArgb(14, 70, 140,  byte.MaxValue);
      }
      if (backcolornr == 1)
      {
        self.BackColor = Color.FromArgb(100, 150, 50,  byte.MaxValue);
        self.BackColor2 = Color.FromArgb(14, 70, 140,  byte.MaxValue);
      }
      if (backcolornr == 5 | backcolornr == -1)
      {
        self.BackColor = Color.FromArgb( byte.MaxValue, 10, 10, 10);
        self.BackColor2 = Color.FromArgb( byte.MaxValue, 40, 40, 70);
      }
      if (backcolornr == 6)
      {
        self.BackColor = Color.FromArgb( byte.MaxValue, 0, 0, 0);
        self.BackColor2 = Color.FromArgb(14, 70, 140,  byte.MaxValue);
      }
      if (backcolornr == 7)
      {
        self.BackColor = Color.FromArgb( byte.MaxValue, 0, 0, 0);
        self.BackColor2 = Color.FromArgb( byte.MaxValue, 0, 0, 0);
      }
      if (backcolornr == 8)
      {
        self.BackColor = Color.FromArgb(0, 0, 0, 0);
        self.BackColor2 = Color.FromArgb(0, 0, 0, 0);
      }
      if (self.game.Data.Product == 6)
      {
        if (backcolornr == 9)
        {
          self.BackColor = Color.FromArgb( byte.MaxValue, 60, 120, 60);
          self.BackColor2 = Color.FromArgb( byte.MaxValue, 120, 180, 120);
        }
      }
      else if (backcolornr == 9)
      {
        self.BackColor = Color.FromArgb( byte.MaxValue, 40, 80, 120);
        self.BackColor2 = Color.FromArgb( byte.MaxValue, 100, 140, 180);
      }
      self.BackBitmap = new Bitmap(w, h, PixelFormat.Format32bppPArgb);
      self.BackBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics graphics = Graphics.FromImage((Image) self.BackBitmap);
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (BackSprite == -1 | self.transbacksprite)
      {
        if (backcolornr == 99 & !Information.IsNothing( self.screenbackref))
        {
           let mut local1: &Graphics = &graphics;
           local2: Bitmap =  self.screenbackref;
          rectangle1 = Rectangle::new(self.screenx, self.screeny, self.screenw, self.screenh);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(0, 0, self.screenw, self.screenh);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          self.useparentscreenbitmap = true;
        }
        else if (!self.DoGradient)
          DrawMod.DrawClear(graphics,  self.BackBitmap, Color.FromArgb( self.BackColor.A,  self.BackColor.R,  self.BackColor.G,  self.BackColor.B));
        else
          DrawMod.DrawBlockGradient3( graphics, 0, 0, self.screenw, self.screenh, self.BackColor, self.BackColor2);
        if (Strings.Len(self.HeaderString) > 0)
        {
          graphics.SmoothingMode = SmoothingMode.AntiAlias;
          DrawMod.DrawBlock( graphics, 2, 2, w - 4, 38, 0, 0, 0, 128);
          SizeF sizeF = graphics.MeasureString(self.HeaderString, self.game.MarcFont1);
          DrawMod.DrawTextColouredMarc( graphics, self.HeaderString, self.game.MarcFont1,  Math.Round( w / 2.0 -  sizeF.Width / 2.0),  Math.Round(22.0 -  sizeF.Height / 2.0), Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
        }
        if (self.DoBorders > 0)
          DrawMod.DrawRectangle( graphics, 0, 0, w - 1, h - 1, 0, 0, 0,  byte.MaxValue);
      }
      if (BackSprite > -1)
      {
        if (!self.transbacksprite)
          graphics.CompositingMode = CompositingMode.SourceCopy;
        if (!self.backspritescaled)
        {
          if (BitmapStore.GetWidth(BackSprite) > w)
          {
             let mut local3: &Graphics = &graphics;
            bitmap: Bitmap = BitmapStore.GetBitmap(BackSprite);
             let mut local4: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new( Math.Round( BitmapStore.GetWidth(BackSprite) / 2.0 -  w / 2.0), 0, w, h);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, 0, w, h);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
          }
          else
          {
             let mut local5: &Graphics = &graphics;
            bitmap: Bitmap = BitmapStore.GetBitmap(BackSprite);
             let mut local6: &Bitmap = &bitmap;
            DrawMod.DrawSimple( local5,  local6, 0, 0);
          }
        }
        else
        {
           let mut local7: &Graphics = &graphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(BackSprite);
           let mut local8: &Bitmap = &bitmap;
          let mut w1: i32 = w;
          let mut h1: i32 = h;
          DrawMod.DrawScaled( local7,  local8, 0, 0, w1, h1);
        }
        if (!self.transbacksprite)
          graphics.CompositingMode = CompositingMode.SourceOver;
      }
      self.OwnBitmap = (Bitmap) self.BackBitmap.Clone();
      self.SubPartCounter = -1;
      self.SubPartIDCounter = 0;
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
    }

    pub fn FlagAll()
    {
      if (self.SubPartCounter < 0)
        return;
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        self.SubPartFlag[index] = true;
    }

    pub Paint: Bitmap()
    {
      if (Information.IsNothing( self.OwnBitmap))
      {
        if (Information.IsNothing( self.BackBitmap))
        {
          self.OwnBitmap = new Bitmap(self.screenw, self.screenh, PixelFormat.Format32bppPArgb);
          self.OwnBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
          Graphics.FromImage((Image) self.OwnBitmap).Clear(Color.Pink);
        }
        else
          self.OwnBitmap = (Bitmap) self.BackBitmap.Clone();
      }
      Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
      let mut subPartCounter: i32 = self.SubPartCounter;
      bitmap: Bitmap;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if ((!Information.IsNothing( self.LowerWindow) | self.BlockBlit) & !self.SubPartList[index].oldStyle)
        {
          if (self.SubPartFlag[index])
          {
            if (self.LastOverlaySubPart == self.SubPartID[index])
            {
               let mut local1: &Graphics = &objGraphics;
              bitmap = self.SubPartList[index].PaintOverlay();
               let mut local2: &Bitmap = &bitmap;
               local3: Bitmap =  self.OwnBitmap;
              let mut x: i32 = self.SubPartX[index];
              let mut y: i32 = self.SubPartY[index];
              DrawMod.DrawSimpleFast( local1,  local2,  local3, x, y);
            }
            else
            {
               let mut local4: &Graphics = &objGraphics;
              bitmap = self.SubPartList[index].Paint();
               let mut local5: &Bitmap = &bitmap;
               local6: Bitmap =  self.OwnBitmap;
              let mut x: i32 = self.SubPartX[index];
              let mut y: i32 = self.SubPartY[index];
              DrawMod.DrawSimpleFast( local4,  local5,  local6, x, y);
            }
            self.SubPartFlag[index] = false;
          }
        }
        else if (self.SubPartFlag[index])
        {
          if (self.LastOverlaySubPart == self.SubPartID[index])
          {
             let mut local7: &Graphics = &objGraphics;
            bitmap = self.SubPartList[index].PaintOverlay();
             let mut local8: &Bitmap = &bitmap;
            let mut x: i32 = self.SubPartX[index];
            let mut y: i32 = self.SubPartY[index];
            DrawMod.DrawSimple( local7,  local8, x, y);
          }
          else
          {
             let mut local9: &Graphics = &objGraphics;
            bitmap = self.SubPartList[index].Paint();
             let mut local10: &Bitmap = &bitmap;
            let mut x: i32 = self.SubPartX[index];
            let mut y: i32 = self.SubPartY[index];
            DrawMod.DrawSimple( local9,  local10, x, y);
          }
          self.SubPartFlag[index] = false;
        }
      }
      if (self.mapframe)
        DrawMod.DrawRectangle( objGraphics, 0, 0, self.OwnBitmap.Width - 1, self.OwnBitmap.Height - 1, 0, 0, 0,  byte.MaxValue);
      if (!Information.IsNothing( objGraphics))
        objGraphics.Dispose();
      return self.OwnBitmap;
    }

    pub fn PaintSpecific(id: i32)
    {
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (id == self.SubPartID[index])
        {
          if (self.SubPartList[index].UseSourceCopyForPaintSpecific())
          {
            Expression.CompositingMode = CompositingMode.SourceCopy;
             let mut local1: &Graphics = &Expression;
            bitmap: Bitmap = self.SubPartList[index].Paint();
             let mut local2: &Bitmap = &bitmap;
            let mut x: i32 = self.SubPartX[index];
            let mut y: i32 = self.SubPartY[index];
            DrawMod.DrawSimple( local1,  local2, x, y);
            Expression.CompositingMode = CompositingMode.SourceOver;
          }
          else
          {
             let mut local3: &Graphics = &Expression;
            bitmap: Bitmap = self.SubPartList[index].Paint();
             let mut local4: &Bitmap = &bitmap;
            let mut x: i32 = self.SubPartX[index];
            let mut y: i32 = self.SubPartY[index];
            DrawMod.DrawSimple( local3,  local4, x, y);
          }
          self.AddQuickRect(Rectangle::new(self.SubPartX[index], self.SubPartY[index], self.SubPartW[index], self.SubPartH[index]));
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub fn Before_DoRefresh_Called_By_FlagAllIncludingRefresh()
    {
    }

    pub fn DoRefresh()
    {
    }

    pub fn PaintCurrentBitmap(id: i32)
    {
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (id == self.SubPartID[index])
        {
          if (self.SubPartW[index] == self.OwnBitmap.Width & self.SubPartH[index] == self.OwnBitmap.Height & self.GetType().Equals(typeof (MapWindowClass)))
          {
            self.OwnBitmap.Dispose();
            self.OwnBitmap = (Bitmap) self.SubPartList[index].OwnBitmap.Clone();
            GC.Collect(GC.MaxGeneration, GCCollectionMode.Forced);
            Application.DoEvents();
          }
          else
          {
             let mut local1: &Graphics = &Expression;
            bitmap: Bitmap = self.SubPartList[index].Paint();
             let mut local2: &Bitmap = &bitmap;
            let mut x: i32 = self.SubPartX[index];
            let mut y: i32 = self.SubPartY[index];
            DrawMod.DrawSimple( local1,  local2, x, y);
          }
        }
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub fn PaintCurrentBitmapScaled(id: i32)
    {
      Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (id == self.SubPartID[index])
          DrawMod.DrawScaled( objGraphics,  self.SubPartList[index].OwnBitmap, self.SubPartX[index], self.SubPartY[index], self.SubPartW[index], self.SubPartH[index]);
      }
      if (Information.IsNothing( objGraphics))
        return;
      objGraphics.Dispose();
    }

    pub PaintSpecificOverlay: Bitmap(id: i32)
    {
      let mut subPartCounter: i32 = self.SubPartCounter;
      Graphics Expression;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (id == self.SubPartID[index])
        {
          Expression = Graphics.FromImage((Image) self.OwnBitmap);
           let mut local1: &Graphics = &Expression;
          bitmap: Bitmap = self.SubPartList[index].PaintOverlay();
           let mut local2: &Bitmap = &bitmap;
          let mut x: i32 = self.SubPartX[index];
          let mut y: i32 = self.SubPartY[index];
          DrawMod.DrawSimple( local1,  local2, x, y);
          self.AddQuickRect(Rectangle::new(self.SubPartX[index], self.SubPartY[index], self.SubPartW[index], self.SubPartH[index]));
        }
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return self.OwnBitmap;
    }

    pub MouseOverSpecific: bool(id: i32)
    {
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (id == self.SubPartID[index] && (self.SubPartList[index].MouseOver | self.SubPartList[index].Scroller) & DrawMod.MouseClicked)
        {
          Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
          if (self.formref.doubleSize)
          {
            if (self.SubPartList[index].MouseMove( Math.Round( ( Cursor.Position.X * self.formref.doubleModX -  (self.SubPartX[index] + self.screenx))),  Math.Round( ( Cursor.Position.Y * self.formref.doubleModY -  (self.SubPartY[index] + self.screeny)))))
            {
               let mut local1: &Graphics = &graphics;
              bitmap: Bitmap = self.SubPartList[index].Paint();
               let mut local2: &Bitmap = &bitmap;
              let mut x: i32 = self.SubPartX[index];
              let mut y: i32 = self.SubPartY[index];
              DrawMod.DrawSimple( local1,  local2, x, y);
              return true;
            }
          }
          else if (self.SubPartList[index].MouseMove(Cursor.Position.X - (self.SubPartX[index] + self.screenx), Cursor.Position.Y - (self.SubPartY[index] + self.screeny)))
          {
             let mut local3: &Graphics = &graphics;
            bitmap: Bitmap = self.SubPartList[index].Paint();
             let mut local4: &Bitmap = &bitmap;
            let mut x: i32 = self.SubPartX[index];
            let mut y: i32 = self.SubPartY[index];
            DrawMod.DrawSimple( local3,  local4, x, y);
            return true;
          }
          return false;
        }
      }
      return false;
    }

    pub fn AddSubPart( SubPartClass tsubpart, x: i32, y: i32, w: i32, h: i32, overlay: i32) -> i32
    {
      self += 1.SubPartCounter;
      self += 1.SubPartIDCounter;
      self.SubPartList = (SubPartClass[]) Utils.CopyArray((Array) self.SubPartList, (Array) new SubPartClass[self.SubPartCounter + 1]);
      self.SubPartFlag = (bool[]) Utils.CopyArray((Array) self.SubPartFlag, (Array) new bool[self.SubPartCounter + 1]);
      self.SubPartX = (int[]) Utils.CopyArray((Array) self.SubPartX, (Array) new int[self.SubPartCounter + 1]);
      self.SubPartY = (int[]) Utils.CopyArray((Array) self.SubPartY, (Array) new int[self.SubPartCounter + 1]);
      self.SubPartW = (int[]) Utils.CopyArray((Array) self.SubPartW, (Array) new int[self.SubPartCounter + 1]);
      self.SubPartH = (int[]) Utils.CopyArray((Array) self.SubPartH, (Array) new int[self.SubPartCounter + 1]);
      self.SubPartID = (int[]) Utils.CopyArray((Array) self.SubPartID, (Array) new int[self.SubPartCounter + 1]);
      self.SubPartOverlay = (int[]) Utils.CopyArray((Array) self.SubPartOverlay, (Array) new int[self.SubPartCounter + 1]);
      self.SubPartList[self.SubPartCounter] = tsubpart;
      self.SubPartFlag[self.SubPartCounter] = true;
      self.SubPartX[self.SubPartCounter] = x;
      self.SubPartY[self.SubPartCounter] = y;
      self.SubPartW[self.SubPartCounter] = w;
      self.SubPartH[self.SubPartCounter] = h;
      self.SubPartOverlay[self.SubPartCounter] = overlay;
      self.SubPartID[self.SubPartCounter] = self.SubPartIDCounter;
      return self.SubPartIDCounter;
    }

    pub fn NewBackGroundAndClearAll(w: i32, h: i32, backsprite: i32)
    {
      if (Information.IsNothing( self.BackBitmap))
      {
        self.BackBitmap = new Bitmap(w, h, PixelFormat.Format32bppPArgb);
        self.BackBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      }
      Graphics graphics = Graphics.FromImage((Image) self.BackBitmap);
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (backsprite == -1 | self.transbacksprite)
      {
        if (self.useparentscreenbitmap)
        {
           let mut local1: &Graphics = &graphics;
           local2: Bitmap =  self.screenbackref;
          rectangle1 = Rectangle::new(self.screenx, self.screeny, self.screenw, self.screenh);
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(0, 0, self.screenw, self.screenh);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
        }
        else if (!Information.IsNothing( self.LowerWindow))
        {
          if (!Information.IsNothing( self.LowerWindow.SubPartList))
          {
            if (self.LowerWindow.GetType().Equals(typeof (MapWindowClass)) | self.LowerWindow.GetType().Equals(typeof (MapWindowClass2)))
            {
               let mut local3: &Graphics = &graphics;
               local4: Bitmap =  self.LowerWindow.SubPartList[0].OwnBitmap;
              Rectangle lowerRect = self.LowerRect;
              rectangle2 = Rectangle::new(0, 0, w, h);
              let mut destrect: &Rectangle = &rectangle2
              DrawMod.DrawSimplePart2( local3,  local4, lowerRect, destrect);
            }
            else
            {
               let mut local5: &Graphics = &graphics;
               local6: Bitmap =  self.LowerWindow.OwnBitmap;
              Rectangle lowerRect = self.LowerRect;
              rectangle2 = Rectangle::new(0, 0, w, h);
              let mut destrect: &Rectangle = &rectangle2
              DrawMod.DrawSimplePart2( local5,  local6, lowerRect, destrect);
            }
          }
          else if (!Information.IsNothing( self.LowerWindow))
          {
             let mut local7: &Graphics = &graphics;
             local8: Bitmap =  self.LowerWindow.OwnBitmap;
            Rectangle lowerRect = self.LowerRect;
            rectangle2 = Rectangle::new(0, 0, w, h);
            let mut destrect: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local7,  local8, lowerRect, destrect);
          }
        }
        else if (!self.DoGradient)
          DrawMod.DrawClear(graphics,  self.BackBitmap, Color.FromArgb( self.BackColor.A,  self.BackColor.R,  self.BackColor.G,  self.BackColor.B));
        else
          DrawMod.DrawBlockGradient3( graphics, 0, 0, self.screenw, self.screenh, self.BackColor, self.BackColor2);
        if (Strings.Len(self.HeaderString) > 0)
        {
          graphics.SmoothingMode = SmoothingMode.AntiAlias;
          DrawMod.DrawBlock( graphics, 2, 2, w - 4, 38, 0, 0, 0, 128);
          SizeF sizeF = graphics.MeasureString(self.HeaderString, self.game.MarcFont1);
          DrawMod.DrawTextColouredMarc( graphics, self.HeaderString, self.game.MarcFont1,  Math.Round( w / 2.0 -  sizeF.Width / 2.0),  Math.Round(22.0 -  sizeF.Height / 2.0), Color.FromArgb( byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue));
        }
        if (self.DoBorders > 0)
          DrawMod.DrawRectangle( graphics, 0, 0, w - 1, h - 1, 0, 0, 0,  byte.MaxValue);
      }
      if (backsprite > -1)
      {
        if (!self.transbacksprite)
          graphics.CompositingMode = CompositingMode.SourceCopy;
        if (!self.backspritescaled)
        {
          if (BitmapStore.GetWidth(backsprite) > w)
          {
             let mut local9: &Graphics = &graphics;
            bitmap: Bitmap = BitmapStore.GetBitmap(backsprite);
             let mut local10: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new( Math.Round( BitmapStore.GetWidth(backsprite) / 2.0 -  w / 2.0), 0, w, h);
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, 0, w, h);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local9,  local10, srcrect, destrect);
          }
          else if (BitmapStore.GetWidth(backsprite) < w)
          {
             let mut local11: &Graphics = &graphics;
            bitmap: Bitmap = BitmapStore.GetBitmap(backsprite);
             let mut local12: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, BitmapStore.GetWidth(backsprite), BitmapStore.Getheight(backsprite));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(0, 0, w, h);
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local11,  local12, srcrect, destrect);
          }
          else
          {
             let mut local13: &Graphics = &graphics;
            bitmap: Bitmap = BitmapStore.GetBitmap(backsprite);
             let mut local14: &Bitmap = &bitmap;
            DrawMod.DrawSimple( local13,  local14, 0, 0);
          }
        }
        else
        {
           let mut local15: &Graphics = &graphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(backsprite);
           let mut local16: &Bitmap = &bitmap;
          let mut w1: i32 = w;
          let mut h1: i32 = h;
          DrawMod.DrawScaled( local15,  local16, 0, 0, w1, h1);
        }
        if (!self.transbacksprite)
          graphics.CompositingMode = CompositingMode.SourceOver;
      }
      if (!Information.IsNothing( self.OwnBitmap))
      {
        if (!Information.IsNothing( graphics))
        {
          graphics.Dispose();
          graphics = (Graphics) null;
        }
        graphics = Graphics.FromImage((Image) self.OwnBitmap);
        graphics.CompositingMode = CompositingMode.SourceCopy;
        graphics.DrawImage((Image) self.BackBitmap, 0, 0);
        graphics.CompositingMode = CompositingMode.SourceOver;
      }
      if (Information.IsNothing( graphics))
        return;
      graphics.Dispose();
      graphics = (Graphics) null;
    }

    pub fn RemoveSubPart(id: i32)
    {
      if (self.SubPartCounter <= -1)
        return;
      let mut subPartCounter1: i32 = self.SubPartCounter;
      for (let mut index1: i32 = 0; index1 <= subPartCounter1; index1 += 1)
      {
        if (self.SubPartID[index1] == id)
        {
          if (!Information.IsNothing( self.OwnBitmap))
          {
            Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
            Rectangle rect = Rectangle::new(self.SubPartX[index1], self.SubPartY[index1], self.SubPartW[index1], self.SubPartH[index1]);
            DrawMod.DrawSimplePart( objGraphics,  self.BackBitmap, rect);
          }
          if (self.LastOverlaySubPart == id)
            self.LastOverlaySubPart = 0;
          self.SubPartList[index1].Dispose();
          self.SubPartList[index1] =  null;
          if (index1 != self.SubPartCounter)
          {
            let mut num: i32 = index1 + 1;
            let mut subPartCounter2: i32 = self.SubPartCounter;
            for (let mut index2: i32 = num; index2 <= subPartCounter2; index2 += 1)
            {
              self.SubPartList[index2 - 1] = self.SubPartList[index2];
              self.SubPartFlag[index2 - 1] = self.SubPartFlag[index2];
              self.SubPartX[index2 - 1] = self.SubPartX[index2];
              self.SubPartY[index2 - 1] = self.SubPartY[index2];
              self.SubPartW[index2 - 1] = self.SubPartW[index2];
              self.SubPartOverlay[index2 - 1] = self.SubPartOverlay[index2];
              self.SubPartH[index2 - 1] = self.SubPartH[index2];
              self.SubPartID[index2 - 1] = self.SubPartID[index2];
            }
          }
          --self.SubPartCounter;
          self.SubPartList = (SubPartClass[]) Utils.CopyArray((Array) self.SubPartList, (Array) new SubPartClass[self.SubPartCounter + 1]);
          self.SubPartFlag = (bool[]) Utils.CopyArray((Array) self.SubPartFlag, (Array) new bool[self.SubPartCounter + 1]);
          self.SubPartX = (int[]) Utils.CopyArray((Array) self.SubPartX, (Array) new int[self.SubPartCounter + 1]);
          self.SubPartOverlay = (int[]) Utils.CopyArray((Array) self.SubPartOverlay, (Array) new int[self.SubPartCounter + 1]);
          self.SubPartY = (int[]) Utils.CopyArray((Array) self.SubPartY, (Array) new int[self.SubPartCounter + 1]);
          self.SubPartW = (int[]) Utils.CopyArray((Array) self.SubPartW, (Array) new int[self.SubPartCounter + 1]);
          self.SubPartH = (int[]) Utils.CopyArray((Array) self.SubPartH, (Array) new int[self.SubPartCounter + 1]);
          self.SubPartID = (int[]) Utils.CopyArray((Array) self.SubPartID, (Array) new int[self.SubPartCounter + 1]);
          break;
        }
      }
    }

    pub virtual HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32) => WindowReturnClass::new();

    pub virtual HandleMouseClickOutsideWindow: WindowReturnClass(
      x: i32,
      y: i32,
      b: i32)
    {
      return WindowReturnClass::new();
    }

    pub virtual HandleMouseUp: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (self.SubPartList[index].HandleMouseUp(x - self.SubPartX[index], y - self.SubPartY[index]) > -1)
          {
            windowReturnClass.SetFlag(true);
            self.SubPartFlag[index] = true;
            return windowReturnClass;
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub virtual HandleBLOCKEDMouseUp: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (!Information.IsNothing( self.SubPartList[index]))
            self.SubPartList[index].HandleBLOCKEDMouseUp(x - self.SubPartX[index], y - self.SubPartY[index]);
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub virtual object allowClickOutsideWindow() =>  false;

    pub fn clearoverlay()
    {
      if (self.LastOverlaySubPart > 0)
        self.PaintSpecific(self.LastOverlaySubPart);
      self.LastOverlaySubPart = 0;
    }

    pub virtual HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false) => WindowReturnClass::new()
    {
      Flag = false
    };

    pub virtual HandleKeyup: WindowReturnClass(nr: i32) => WindowReturnClass::new()
    {
      Flag = false
    };

    pub virtual handleTimerWheel: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass.Flag = false;
      if (self.SubPartCounter > -1)
      {
        for (let mut subPartCounter: i32 = self.SubPartCounter; subPartCounter >= 0; subPartCounter += -1)
        {
          if (x > self.SubPartX[subPartCounter] & y > self.SubPartY[subPartCounter] & x < self.SubPartX[subPartCounter] + self.SubPartW[subPartCounter] & y < self.SubPartY[subPartCounter] + self.SubPartH[subPartCounter])
          {
            let mut subPart: SubPartClass = self.SubPartList[subPartCounter];
            let mut x1: i32 = x - self.SubPartX[subPartCounter];
            let mut y1: i32 = y - self.SubPartY[subPartCounter];
            WindowClass windowClass = this;
             WindowClass local =  windowClass;
            if (subPart.HandleTimerWheel(x1, y1,  local))
            {
              windowReturnClass.SetFlag(true);
              self.SubPartFlag[subPartCounter] = true;
              return windowReturnClass;
            }
          }
        }
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub virtual handleTimer: WindowReturnClass() => WindowReturnClass::new()
    {
      Flag = false
    };

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            return;
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          if (self.MouseData[index] > 0)
            self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          break;
        }
      }
    }

    pub virtual WindowDescription: String(x: i32, y: i32) => "";

    pub virtual HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        bool flag;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] & y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] | self.SubPartList[index].Scroller)
          {
            if (self.SubPartList[index].HandleMouseMove(x - self.SubPartX[index], y - self.SubPartY[index]))
            {
              self.PaintSpecific(self.SubPartID[index]);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if ( self.game.Data.RuleVar[839] == 0.0 & !self.NewGfx | self.game.Data.Round == 0)
            {
              self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
              windowReturnClass.SetFlag(false);
              if (Operators.CompareString(self.SubPartList[index].Descript, self.game.EditObj.CurrentDescript, false) == 0)
              {
                flag = true;
              }
              else
              {
                self.game.EditObj.CurrentDescript = self.SubPartList[index].Descript;
                flag = true;
                windowReturnClass.AddCommand(4, 29);
                windowReturnClass.SetFlag(true);
              }
            }
            if (self.SubPartOverlay[index] > 0)
            {
              if (self.LastOverlaySubPart == self.SubPartID[index])
              {
                windowReturnClass.SetOverlay(true);
                return windowReturnClass;
              }
              if (self.LastOverlaySubPart > 0)
                self.PaintSpecific(self.LastOverlaySubPart);
              self.PaintSpecificOverlay(self.SubPartID[index]);
              if (self.game.EmpireStyle)
                SoundMod.PlayAWave(self.game.AppPath + "sound/interface/mouseover.wav",  self.game.EditObj);
              self.LastOverlaySubPart = self.SubPartID[index];
              windowReturnClass.SetFlag(true);
              windowReturnClass.SetOverlay(true);
              return windowReturnClass;
            }
            if (self.MouseOverSpecific(self.SubPartID[index]))
            {
              windowReturnClass.SetFlag(true);
              windowReturnClass.SetOverlay(true);
              return windowReturnClass;
            }
          }
        }
        if ( self.game.Data.RuleVar[839] == 0.0 & !self.NewGfx)
        {
          if (!flag)
          {
            Left: String = self.WindowDescription(x, y);
            if (Operators.CompareString(Left, "", false) != 0)
            {
              self.game.EditObj.CurrentDescript = Left;
              flag = true;
              windowReturnClass.AddCommand(4, 29);
              windowReturnClass.SetFlag(true);
            }
          }
          if (!flag && Operators.CompareString("", self.game.EditObj.CurrentDescript, false) != 0)
          {
            self.game.EditObj.CurrentDescript = "";
            windowReturnClass.AddCommand(4, 29);
            windowReturnClass.SetFlag(true);
          }
        }
        if (self.LastOverlaySubPart > 0)
        {
          self.PaintSpecific(self.LastOverlaySubPart);
          self.LastOverlaySubPart = 0;
          windowReturnClass.SetFlag(true);
        }
        windowReturnClass.SetOverlay(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      windowReturnClass.SetOverlay(false);
      return windowReturnClass;
    }
  }
}
