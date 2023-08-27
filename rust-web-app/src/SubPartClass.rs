// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SubPartClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Imaging;

namespace WindowsApplication1
{
  pub class SubPartClass
  {
    pub OwnBitmap: Bitmap;
    pub Descript: String;
    pub MouseOver: bool;
    pub Scroller: bool;
    pub Rectangle[] MouseRect;
    pub MouseCounter: i32;
    pub MouseText: Vec<String>;
    pub MouseTitle: Vec<String>;
    pub MouseData: Vec<i32>;
    pub MouseType: Vec<i32>;
    pub oldStyle: bool;

    pub virtual bool UseSourceCopyForPaintSpecific() => false;

    pub fn ClearMouse() => self.MouseCounter = -1;

    pub fn AddMouse( Rectangle trect, ttitle: String, ttext: String, let mut tdata: i32 = -1, let mut ttype: i32 = 0)
    {
      self += 1.MouseCounter;
      self.MouseRect = (Rectangle[]) Utils.CopyArray((Array) self.MouseRect, (Array) Rectangle::new[self.MouseCounter + 1]);
      self.MouseText = (string[]) Utils.CopyArray((Array) self.MouseText, (Array) new string[self.MouseCounter + 1]);
      self.MouseTitle = (string[]) Utils.CopyArray((Array) self.MouseTitle, (Array) new string[self.MouseCounter + 1]);
      self.MouseData = (int[]) Utils.CopyArray((Array) self.MouseData, (Array) new int[self.MouseCounter + 1]);
      self.MouseType = (int[]) Utils.CopyArray((Array) self.MouseType, (Array) new int[self.MouseCounter + 1]);
      self.MouseRect[self.MouseCounter] = trect;
      self.MouseText[self.MouseCounter] = ttext;
      self.MouseTitle[self.MouseCounter] = ttitle;
      self.MouseData[self.MouseCounter] = tdata;
      self.MouseType[self.MouseCounter] = ttype;
    }

    pub fn Dispose()
    {
      if (!Information.IsNothing( self.OwnBitmap))
        self.OwnBitmap.Dispose();
      self.OwnBitmap = (Bitmap) null;
      self.SubDispose();
    }

    pub fn SubDispose()
    {
    }

    pub fn GetMemorySize() =>  Math.Round( (64 * self.OwnBitmap.Width * self.OwnBitmap.Height) / 8000.0) -> i32;

    pub virtual GetSelect: i32() => -1;

    pub virtual GetTopItem: i32() => -1;

    pub SubPartClass(w: i32, h: i32)
    {
      self.oldStyle = false;
      self.MouseCounter = -1;
      if (w > 0 & h > 0)
      {
        self.OwnBitmap = new Bitmap(w, h, PixelFormat.Format32bppPArgb);
        self.OwnBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
        DrawMod.ClearTransparent( objgraphics);
        if (!Information.IsNothing( objgraphics))
          objgraphics.Dispose();
      }
      self.MouseOver = false;
    }

    pub fn Resize(w: i32, h: i32)
    {
      if (!Information.IsNothing( self.OwnBitmap))
        self.OwnBitmap.Dispose();
      self.OwnBitmap = new Bitmap(w, h, PixelFormat.Format32bppPArgb);
      self.OwnBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.ClearTransparent( objgraphics);
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
    }

    pub fn Refresh(ListClass tListObj, tlistselect: i32, theader: String = "")
    {
    }

    pub fn Refresh(ATListClass tListObj, tlistselect: i32, theader: String = "")
    {
    }

    pub fn Refresh(StringListClass tListObj, trow: i32, tcol: i32)
    {
    }

    pub fn Refresh(s: String)
    {
    }

    pub virtual HeightUsed: i32() => self.OwnBitmap.Height;

    pub virtual GetText: String() => "";

    pub fn Clear()
    {
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.ClearTransparent( objgraphics);
      if (Information.IsNothing( objgraphics))
        return;
      objgraphics.Dispose();
    }

    pub virtual Paint: Bitmap()
    {
      bitmap: Bitmap;
      return bitmap;
    }

    pub virtual HandleMouseUp: i32(x: i32, y: i32) => -1;

    pub virtual HandleBLOCKEDMouseUp: i32(x: i32, y: i32) => -1;

    pub fn HandleToolTip(x: i32, y: i32)
    {
    }

    pub virtual bool HandleMouseMove(x: i32, y: i32)
    {
      bool flag;
      return flag;
    }

    pub virtual bool HandleTimerWheel(x: i32, y: i32,  WindowClass tWindow) => false;

    pub virtual PaintOverlay: Bitmap() => self.Paint();

    pub virtual bool MouseMove(x: i32, y: i32) => false;

    pub virtual Click: i32(x: i32, y: i32, let mut b: i32 = 1)
    {
      num: i32;
      return num;
    }

    pub virtual Coordinate Click2(x: i32, y: i32, let mut b: i32 = 1)
    {
      Coordinate coordinate;
      return coordinate;
    }

    pub virtual Coordinate ClickMap(x: i32, y: i32)
    {
      Coordinate coordinate;
      return coordinate;
    }

    pub fn PaintCoordinate(
      Graphics g,
      x: i32,
      y: i32,
      map: i32,
      let mut counteralpha: i32 = 255,
       gBitmap: Bitmap = null)
    {
    }

    pub fn DescriptInfo(x: i32, y: i32)
    {
    }

    pub fn ShiftLeft()
    {
    }

    pub fn ShiftRight()
    {
    }

    pub fn ShiftUp()
    {
    }

    pub fn ShiftDown()
    {
    }
  }
}
