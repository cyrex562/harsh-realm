// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ScreenClass
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
  pub class ScreenClass
  {
    protected WindowClass[] WindowList;
    protected int[] WindowX;
    protected int[] WindowY;
    protected int[] WindowW;
    protected int[] WindowH;
    protected bool[] WindowInputBlock;
    protected bool[] WindowFlag;
    protected WindowCounter: i32;
    protected WindowIDCounter: i32;
    protected int[] WindowID;
    protected OwnBitmap: Bitmap;
    pub OwnBackground: Bitmap;
    pub LastOverlayWindow: i32;
    pub game: GameClass;
    protected FormRef: Form1;
    pub doMinimize: bool;
    pub doQuit: bool;
    pub Rectangle LastToolTipRect;
    pub AllowRightMouse: bool;

    pub fn Dispose()
    {
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        self.WindowList[index].Dispose();
        self.WindowList[index] = (WindowClass) null;
      }
      self.OwnBitmap.Dispose();
      self.OwnBitmap = (Bitmap) null;
      self.OwnBackground.Dispose();
      self.OwnBackground = (Bitmap) null;
      self.Game = (GameClass) null;
      self.FormRef = (Form1) null;
      GC.Collect(GC.MaxGeneration, GCCollectionMode.Forced);
      Application.DoEvents();
    }

    pub HasOwnBitmap: bool() => !Information.IsNothing( self.OwnBitmap);

    pub fn GetMemorySize() -> i32
    {
      let mut memorySize: i32 =  Math.Round( (64 * self.OwnBitmap.Width * self.OwnBitmap.Height) / 8000.0);
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
        memorySize += self.WindowList[index].GetMemorySize();
      return memorySize;
    }

    pub fn RefreshOwnBackground(backgroundsprite: i32)
    {
      if (Information.IsNothing( self.OwnBackground))
      {
        self.OwnBackground = new Bitmap(self.Game.ScreenWidth, self.Game.ScreenHeight, PixelFormat.Format32bppPArgb);
        self.OwnBackground.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      }
      Graphics graphics = Graphics.FromImage((Image) self.OwnBackground);
      if (BitmapStore.GetWidth(backgroundsprite) <= 512)
      {
        let mut width: i32 = BitmapStore.GetWidth(backgroundsprite);
        let mut num1: i32 = BitmapStore.Getheight(backgroundsprite);
        let mut num2: i32 =  Math.Round(Conversion.Int( self.Game.ScreenWidth /  width));
        for (let mut index1: i32 = 0; index1 <= num2; index1 += 1)
        {
          let mut num3: i32 =  Math.Round(Conversion.Int( self.Game.ScreenHeight /  num1));
          for (let mut index2: i32 = 0; index2 <= num3; index2 += 1)
          {
             let mut local1: &Graphics = &graphics;
            bitmap: Bitmap = BitmapStore.GetBitmap(backgroundsprite);
             let mut local2: &Bitmap = &bitmap;
            let mut x: i32 = index1 * width;
            let mut y: i32 = index2 * num1;
            let mut w: i32 = width;
            let mut h: i32 = num1;
            DrawMod.DrawScaled( local1,  local2, x, y, w, h);
          }
        }
      }
      else
      {
        let mut width: i32 = BitmapStore.GetWidth(backgroundsprite);
        let mut num4: i32 = BitmapStore.Getheight(backgroundsprite);
        float num5 =  self.Game.ScreenWidth /  width;
        float num6 =  self.Game.ScreenHeight /  num4;
        if ( num5 !=  num6)
        {
          if ( num5 >  num6)
            num6 = num5;
          else
            num5 = num6;
          let mut num7: i32 =  Math.Round( ( width * num5));
          let mut num8: i32 =  Math.Round( ( num4 * num6));
          let mut num9: i32 =  Math.Round(0.0 +  (self.Game.ScreenWidth - num7) / 2.0);
          let mut num10: i32 =  Math.Round(0.0 +  (self.Game.ScreenHeight - num8) / 2.0);
          if (num9 > 0)
          {
            num7 += num9;
            num10 -= num9;
            num8 += num9;
            num9 = 0;
          }
          if (num10 > 0)
          {
            num8 += num10;
            num7 += num10;
            num9 -= num10;
            num10 = 0;
          }
           let mut local3: &Graphics = &graphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(backgroundsprite);
           let mut local4: &Bitmap = &bitmap;
          let mut x: i32 = num9;
          let mut y: i32 = num10;
          let mut w: i32 = num7;
          let mut h: i32 = num8;
          DrawMod.DrawScaled( local3,  local4, x, y, w, h);
        }
        else
        {
           let mut local5: &Graphics = &graphics;
          bitmap: Bitmap = BitmapStore.GetBitmap(backgroundsprite);
           let mut local6: &Bitmap = &bitmap;
          let mut screenWidth: i32 = self.Game.ScreenWidth;
          let mut screenHeight: i32 = self.Game.ScreenHeight;
          DrawMod.DrawScaled( local5,  local6, 0, 0, screenWidth, screenHeight);
        }
      }
      if (Information.IsNothing( self.OwnBitmap))
      {
        self.OwnBitmap = new Bitmap(self.Game.ScreenWidth, self.Game.ScreenHeight, PixelFormat.Format32bppPArgb);
        self.OwnBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      }
      Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
      objGraphics.CompositingMode = CompositingMode.SourceCopy;
      DrawMod.DrawSimple( objGraphics,  self.OwnBackground, 0, 0);
      objGraphics.CompositingMode = CompositingMode.SourceOver;
      self.WindowCounter = -1;
      self.WindowIDCounter = 0;
      if (Information.IsNothing( objGraphics))
        return;
      objGraphics.Dispose();
    }

    pub ScreenClass( tGame: GameClass, let mut backgroundsprite: i32 = -1, tFormRef: Form1 = null)
    {
      self.Game = tGame;
      self.FormRef = tFormRef;
      self.OwnBackground = new Bitmap(self.Game.ScreenWidth, self.Game.ScreenHeight, PixelFormat.Format32bppPArgb);
      self.OwnBackground.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBackground);
      switch (backgroundsprite)
      {
        case -4:
          if (Strings.Len(tGame.EditObj.CampaignRoomTitle) > 0 &  self.Game.Data.RuleVar[839] == 0.0)
          {
            DrawMod.DrawBlock( Expression, 0, 0, self.Game.ScreenWidth, self.Game.ScreenHeight, 0, 0, 0,  byte.MaxValue);
            break;
          }
          DrawMod.DrawSimple( Expression,  self.FormRef.StoredScreeny.OwnBitmap, 0, 0);
          DrawMod.DrawBlock( Expression, 0, 0, self.Game.ScreenWidth, self.Game.ScreenHeight, 0, 0, 0, 100);
          break;
        case -3:
          DrawMod.Clear( Expression, 0, 0, 150);
          break;
        case -2:
          DrawMod.Clear( Expression, 0, 0, 0);
          break;
        case -1:
          DrawMod.Clear( Expression, 0, 0, 0);
          break;
        default:
          if (BitmapStore.GetWidth(backgroundsprite) <= 512)
          {
            let mut width: i32 = BitmapStore.GetWidth(backgroundsprite);
            let mut num1: i32 = BitmapStore.Getheight(backgroundsprite);
            let mut num2: i32 =  Math.Round(Conversion.Int( self.Game.ScreenWidth /  width));
            for (let mut index1: i32 = 0; index1 <= num2; index1 += 1)
            {
              let mut num3: i32 =  Math.Round(Conversion.Int( self.Game.ScreenHeight /  num1));
              for (let mut index2: i32 = 0; index2 <= num3; index2 += 1)
              {
                 let mut local1: &Graphics = &Expression;
                bitmap: Bitmap = BitmapStore.GetBitmap(backgroundsprite);
                 let mut local2: &Bitmap = &bitmap;
                let mut x: i32 = index1 * width;
                let mut y: i32 = index2 * num1;
                let mut w: i32 = width;
                let mut h: i32 = num1;
                DrawMod.DrawScaled( local1,  local2, x, y, w, h);
              }
            }
            break;
          }
          let mut width1: i32 = BitmapStore.GetWidth(backgroundsprite);
          let mut num4: i32 = BitmapStore.Getheight(backgroundsprite);
          float num5 =  self.Game.ScreenWidth /  width1;
          float num6 =  self.Game.ScreenHeight /  num4;
          if ( num5 !=  num6)
          {
            if ( num5 >  num6)
              num6 = num5;
            else
              num5 = num6;
            let mut num7: i32 =  Math.Round( ( width1 * num5));
            let mut num8: i32 =  Math.Round( ( num4 * num6));
            let mut num9: i32 =  Math.Round(0.0 +  (self.Game.ScreenWidth - num7) / 2.0);
            let mut num10: i32 =  Math.Round(0.0 +  (self.Game.ScreenHeight - num8) / 2.0);
            if (num9 > 0)
            {
              num7 += num9;
              num10 -= num9;
              num8 += num9;
              num9 = 0;
            }
            if (num10 > 0)
            {
              num8 += num10;
              num7 += num10;
              num9 -= num10;
              num10 = 0;
            }
             let mut local3: &Graphics = &Expression;
            bitmap: Bitmap = BitmapStore.GetBitmap(backgroundsprite);
             let mut local4: &Bitmap = &bitmap;
            let mut x: i32 = num9;
            let mut y: i32 = num10;
            let mut w: i32 = num7;
            let mut h: i32 = num8;
            DrawMod.DrawScaled( local3,  local4, x, y, w, h);
            break;
          }
           let mut local5: &Graphics = &Expression;
          bitmap1: Bitmap = BitmapStore.GetBitmap(backgroundsprite);
           let mut local6: &Bitmap = &bitmap1;
          let mut screenWidth: i32 = self.Game.ScreenWidth;
          let mut screenHeight: i32 = self.Game.ScreenHeight;
          DrawMod.DrawScaled( local5,  local6, 0, 0, screenWidth, screenHeight);
          break;
      }
      self.OwnBitmap = new Bitmap(self.Game.ScreenWidth, self.Game.ScreenHeight, PixelFormat.Format32bppPArgb);
      self.OwnBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      Expression = Graphics.FromImage((Image) self.OwnBitmap);
      Expression.CompositingMode = CompositingMode.SourceCopy;
      DrawMod.DrawSimpleFast( Expression,  self.OwnBackground,  self.OwnBitmap, 0, 0);
      Expression.CompositingMode = CompositingMode.SourceOver;
      self.WindowCounter = -1;
      self.WindowIDCounter = 0;
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub fn GetNr(id: i32) -> i32
    {
      if (self.WindowCounter <= -1)
        return -1;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut nr: i32 = 0; nr <= windowCounter; nr += 1)
      {
        if (self.WindowID[nr] == id)
          return nr;
      }
      nr1: i32;
      return nr1;
    }

    pub fn GetWindowHeight(id: i32) -> i32
    {
      if (self.WindowCounter <= -1)
        return -1;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        if (self.WindowID[index] == id)
          return self.WindowH[index];
      }
      windowHeight: i32;
      return windowHeight;
    }

    pub fn GetWindowWidth(id: i32) -> i32
    {
      if (self.WindowCounter <= -1)
        return -1;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        if (self.WindowID[index] == id)
          return self.WindowW[index];
      }
      windowWidth: i32;
      return windowWidth;
    }

    pub fn DoRefresh()
    {
      if (self.WindowCounter < 0)
        return;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        self.WindowFlag[index] = true;
        self.WindowList[index].DoRefresh();
      }
    }

    pub fn FlagAll()
    {
      if (self.WindowCounter < 0)
        return;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        self.WindowFlag[index] = true;
        self.WindowList[index].FlagAll();
      }
    }

    pub fn FlagAllIncludingRefresh()
    {
      if (self.WindowCounter < 0)
        return;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        self.WindowList[index].Before_DoRefresh_Called_By_FlagAllIncludingRefresh();
        self.WindowList[index].DoRefresh();
        self.WindowFlag[index] = true;
        self.WindowList[index].FlagAll();
      }
    }

    pub virtual Paint: Bitmap(bool onlyToolTip = false)
    {
      bool flag1 = false;
      if (Information.IsNothing( self.OwnBitmap))
      {
        self.OwnBitmap = new Bitmap(self.Game.ScreenWidth, self.Game.ScreenHeight, PixelFormat.Format32bppPArgb);
        self.OwnBitmap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      }
      if (Information.IsNothing( self.OwnBackground))
      {
        self.OwnBackground = new Bitmap(self.Game.ScreenWidth, self.Game.ScreenHeight, PixelFormat.Format32bppPArgb);
        self.OwnBackground.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
      }
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      if (self.LastToolTipRect.Width > 0)
      {
        DrawMod.DrawSimplePart( graphics,  self.OwnBackground, self.LastToolTipRect);
        let mut num: i32 = 1;
        do
        {
          let mut windowCounter: i32 = self.WindowCounter;
          for (let mut index: i32 = 0; index <= windowCounter; index += 1)
          {
            if (num == 2 & !(Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 | Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass", false) == 0) | num == 1 & (Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 | Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass", false) == 0))
            {
              Rectangle rectangle = Rectangle::new(self.WindowX[index], self.WindowY[index], self.WindowW[index], self.WindowH[index]);
              Rectangle destrect = Rectangle.Intersect(self.LastToolTipRect, rectangle);
              if (!destrect.IsEmpty)
              {
                rectangle = destrect;
                rectangle.X -= self.WindowX[index];
                rectangle.Y -= self.WindowY[index];
                rectangle.Width = Math.Min(rectangle.Width, self.WindowW[index]);
                rectangle.Height = Math.Min(rectangle.Height, self.WindowH[index]);
                if (num == 1)
                  DrawMod.DrawSimplePart2( graphics,  self.WindowList[index].SubPartList[0].OwnBitmap, rectangle, destrect);
                else
                  DrawMod.DrawSimplePart2( graphics,  self.WindowList[index].OwnBitmap, rectangle, destrect);
              }
            }
          }
          num += 1;
        }
        while (num <= 2);
        self.LastToolTipRect.Width = 0;
      }
      if (!onlyToolTip)
      {
        let mut windowCounter1: i32 = self.WindowCounter;
        num: i32;
        for (let mut index: i32 = 0; index <= windowCounter1; index += 1)
        {
          if (!Information.IsNothing( self.WindowList[index]) && Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 | Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass", false) == 0 && self.WindowFlag[index])
          {
            graphics.CompositingMode = CompositingMode.SourceCopy;
            if (self.WindowList[index].DoShowRect)
              DrawMod.DrawSimplePart2Fast( graphics,  self.WindowList[index].SubPartList[0].OwnBitmap,  self.OwnBitmap, self.WindowList[index].ShowRect, Rectangle::new()
              {
                X = self.WindowX[index] + self.WindowList[index].ShowRect.X,
                Y = self.WindowY[index] + self.WindowList[index].ShowRect.Y,
                Width = self.WindowList[index].ShowRect.Width,
                Height = self.WindowList[index].ShowRect.Height
              });
            else
              DrawMod.DrawSimpleFast( graphics,  self.WindowList[index].SubPartList[0].OwnBitmap,  self.OwnBitmap, self.WindowX[index], self.WindowY[index]);
            graphics.CompositingMode = CompositingMode.SourceOver;
            num += 1;
            self.WindowFlag[index] = false;
            if (flag1)
              DrawMod.DrawRectangle( graphics, self.WindowX[index], self.WindowY[index], self.WindowW[index], self.WindowH[index],  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
          }
        }
        let mut windowCounter2: i32 = self.WindowCounter;
        for (let mut index1: i32 = 0; index1 <= windowCounter2; index1 += 1)
        {
          if (!Information.IsNothing( self.WindowList[index1]))
          {
            if (!(Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) == 0 | Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.MapWindowClass", false) == 0) && self.WindowFlag[index1])
            {
              Rectangle rectangle1;
              if (!Information.IsNothing( self.WindowList[index1].LowerWindow) | self.WindowList[index1].BlockBlit)
              {
                graphics.CompositingMode = CompositingMode.SourceCopy;
                 let mut local1: &Graphics = &graphics;
                bitmap: Bitmap = self.WindowList[index1].Paint();
                 let mut local2: &Bitmap = &bitmap;
                 local3: Bitmap =  self.OwnBitmap;
                let mut x: i32 = self.WindowX[index1];
                let mut y: i32 = self.WindowY[index1];
                DrawMod.DrawSimpleFast( local1,  local2,  local3, x, y);
                graphics.CompositingMode = CompositingMode.SourceOver;
              }
              else
              {
                graphics.CompositingMode = CompositingMode.SourceCopy;
                Rectangle rectangle2 = Rectangle::new();
                if (self.WindowList[index1].QuickDrawMode)
                {
                  let mut quickRectCount: i32 = self.WindowList[index1].QuickRectCount;
                  for (let mut index2: i32 = 0; index2 <= quickRectCount; index2 += 1)
                  {
                    rectangle2.X = self.WindowX[index1] + self.WindowList[index1].QuickRect[index2].X;
                    rectangle2.Y = self.WindowY[index1] + self.WindowList[index1].QuickRect[index2].Y;
                    rectangle2.Width = self.WindowList[index1].QuickRect[index2].Width;
                    rectangle2.Height = self.WindowList[index1].QuickRect[index2].Height;
                    DrawMod.DrawSimplePart2( graphics,  self.OwnBackground, rectangle2, rectangle2);
                  }
                }
                else
                {
                   let mut local4: &Graphics = &graphics;
                   local5: Bitmap =  self.OwnBackground;
                  rectangle1 = Rectangle::new(self.WindowX[index1], self.WindowY[index1], self.WindowW[index1], self.WindowH[index1]);
                  let mut rect: &Rectangle = &rectangle1
                  DrawMod.DrawSimplePart( local4,  local5, rect);
                }
                graphics.CompositingMode = CompositingMode.SourceOver;
                if (self.WindowList[index1].QuickDrawMode)
                {
                  let mut quickRectCount: i32 = self.WindowList[index1].QuickRectCount;
                  for (let mut index3: i32 = 0; index3 <= quickRectCount; index3 += 1)
                  {
                    rectangle2.X = self.WindowX[index1] + self.WindowList[index1].QuickRect[index3].X;
                    rectangle2.Y = self.WindowY[index1] + self.WindowList[index1].QuickRect[index3].Y;
                    rectangle2.Width = self.WindowList[index1].QuickRect[index3].Width;
                    rectangle2.Height = self.WindowList[index1].QuickRect[index3].Height;
                    DrawMod.DrawSimplePart2( graphics,  self.WindowList[index1].OwnBitmap, self.WindowList[index1].QuickRect[index3], rectangle2);
                  }
                  self.WindowList[index1].ResetQuickRect();
                }
                else
                {
                   let mut local6: &Graphics = &graphics;
                  bitmap: Bitmap = self.WindowList[index1].Paint();
                   let mut local7: &Bitmap = &bitmap;
                  let mut x: i32 = self.WindowX[index1];
                  let mut y: i32 = self.WindowY[index1];
                  DrawMod.DrawSimple( local6,  local7, x, y);
                }
              }
              if (Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.SpecialWindowClass7", false) == 0 && self.WindowCounter >= index1 + 1 && Operators.CompareString(self.WindowList[index1 + 1].GetType().FullName, "WindowsApplication1.ResourceWindowClass2", false) == 0)
                self.WindowFlag[index1 + 1] = true;
              if (self.WindowList[index1].fixshade)
              {
                 let mut local8: &Graphics = &graphics;
                 local9: Bitmap =  self.OwnBackground;
                rectangle1 = Rectangle::new(0, self.OwnBitmap.Height - 210, self.OwnBitmap.Width, 10);
                let mut rect: &Rectangle = &rectangle1
                DrawMod.DrawSimplePart( local8,  local9, rect);
                DrawMod.DrawBlock( graphics, 0, self.OwnBitmap.Height - 210, self.OwnBitmap.Width, 10,  DrawMod.TGame.VicColor4.R,  DrawMod.TGame.VicColor4.G,  DrawMod.TGame.VicColor4.B,  DrawMod.TGame.VicColor4.A);
                DrawMod.drawLine( graphics, 0, self.OwnBitmap.Height - 200, self.OwnBitmap.Width, self.OwnBitmap.Height - 200,  DrawMod.TGame.VicColor6.R,  DrawMod.TGame.VicColor6.G,  DrawMod.TGame.VicColor6.B,  DrawMod.TGame.VicColor3.A);
              }
              num += 1;
              self.WindowFlag[index1] = false;
              if (flag1)
              {
                let mut w: i32 = self.WindowW[index1] - 10;
                let mut h: i32 = self.WindowH[index1] - 10;
                DrawMod.DrawRectangle( graphics, self.WindowX[index1], self.WindowY[index1], w, h,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                DrawMod.drawLine( graphics,  Math.Round( self.WindowX[index1] +  w / 2.0), self.WindowY[index1],  Math.Round( self.WindowX[index1] +  w / 2.0), self.WindowY[index1] + h,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
                DrawMod.drawLine( graphics, self.WindowX[index1],  Math.Round( self.WindowY[index1] +  h / 2.0), self.WindowX[index1] + w,  Math.Round( self.WindowY[index1] +  h / 2.0),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
              }
            }
            if (Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.UnitSelectWindowClass2", false) == 0)
            {
              graphics.CompositingMode = CompositingMode.SourceCopy;
              DrawMod.DrawSimple( graphics,  self.WindowList[index1].SubPartList[0].OwnBitmap, self.WindowX[index1] + 5, self.WindowY[index1] + 60);
              graphics.CompositingMode = CompositingMode.SourceOver;
              if (flag1)
                DrawMod.DrawRectangle( graphics, self.WindowX[index1], self.WindowY[index1], self.WindowW[index1], self.WindowH[index1],  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
            }
            if (Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.NonCardSelectWindowClass", false) == 0)
            {
              if (Operators.CompareString(self.WindowList[index1].SubPartList[0].GetType().FullName, "WindowsApplication1.MapPartClass", false) == 0)
              {
                graphics.CompositingMode = CompositingMode.SourceCopy;
                DrawMod.DrawSimple( graphics,  self.WindowList[index1].SubPartList[0].OwnBitmap, self.WindowX[index1] + 5, self.WindowY[index1] + 60);
                graphics.CompositingMode = CompositingMode.SourceOver;
              }
              if (flag1)
                DrawMod.DrawRectangle( graphics, self.WindowX[index1], self.WindowY[index1], self.WindowW[index1], self.WindowH[index1],  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
            }
            if (Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.MapSelectWindowClass2", false) == 0)
            {
              graphics.CompositingMode = CompositingMode.SourceCopy;
              DrawMod.DrawSimple( graphics,  self.WindowList[index1].SubPartList[0].OwnBitmap, self.WindowX[index1] + 5, self.WindowY[index1] + 60);
              graphics.CompositingMode = CompositingMode.SourceOver;
              if (flag1)
                DrawMod.DrawRectangle( graphics, self.WindowX[index1], self.WindowY[index1], self.WindowW[index1], self.WindowH[index1],  byte.MaxValue,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue);
            }
          }
        }
        if (num > 0 & self.Game.EditObj.TutMode)
          self.DoTutorial3(graphics);
        if (self.Game.Data.Product < 6)
        {
          if (Operators.CompareString(self.GetType().FullName, "WindowsApplication1.PlayScreenClass", false) == 0)
          {
            DrawMod.DrawBlock( graphics, self.Game.ScreenWidth - 52, 1, 25, 25, 0, 0, 0,  byte.MaxValue);
            DrawMod.DrawBlock( graphics, self.Game.ScreenWidth - 28, 1, 25, 25, 0, 0, 0,  byte.MaxValue);
          }
          bitmap1: Bitmap;
          if (self.doMinimize)
          {
             let mut local10: &Graphics = &graphics;
            bitmap2: Bitmap = BitmapStore.GetBitmap(self.Game.SYSTEM1B);
             let mut local11: &Bitmap = &bitmap2;
            let mut x: i32 = self.Game.ScreenWidth - 52;
            DrawMod.DrawSimple( local10,  local11, x, 1);
          }
          else
          {
             let mut local12: &Graphics = &graphics;
            bitmap1 = BitmapStore.GetBitmap(self.Game.SYSTEM1);
             let mut local13: &Bitmap = &bitmap1;
            let mut x: i32 = self.Game.ScreenWidth - 52;
            DrawMod.DrawSimple( local12,  local13, x, 1);
          }
          if (self.doQuit)
          {
             let mut local14: &Graphics = &graphics;
            bitmap1 = BitmapStore.GetBitmap(self.Game.SYSTEM2B);
             let mut local15: &Bitmap = &bitmap1;
            let mut x: i32 = self.Game.ScreenWidth - 28;
            DrawMod.DrawSimple( local14,  local15, x, 1);
          }
          else
          {
             let mut local16: &Graphics = &graphics;
            bitmap1 = BitmapStore.GetBitmap(self.Game.SYSTEM2);
             let mut local17: &Bitmap = &bitmap1;
            let mut x: i32 = self.Game.ScreenWidth - 28;
            DrawMod.DrawSimple( local16,  local17, x, 1);
          }
        }
        else if (self.Game.Data.Product == 6)
        {
          if (Operators.CompareString(self.GetType().FullName, "WindowsApplication1.PlayScreenClass2", false) != 0 && Operators.CompareString(self.GetType().FullName, "WindowsApplication1.MessagePopUpScreenClass2", false) != 0)
          {
            bitmap3: Bitmap;
            if (self.doMinimize)
            {
               let mut local18: &Graphics = &graphics;
              bitmap4: Bitmap = BitmapStore.GetBitmap(self.Game.SYSTEM1B);
               let mut local19: &Bitmap = &bitmap4;
              let mut x: i32 = self.Game.ScreenWidth - 52;
              DrawMod.DrawSimple( local18,  local19, x, 1);
            }
            else
            {
               let mut local20: &Graphics = &graphics;
              bitmap3 = BitmapStore.GetBitmap(self.Game.SYSTEM1);
               let mut local21: &Bitmap = &bitmap3;
              let mut x: i32 = self.Game.ScreenWidth - 52;
              DrawMod.DrawSimple( local20,  local21, x, 1);
            }
            if (self.doQuit)
            {
               let mut local22: &Graphics = &graphics;
              bitmap3 = BitmapStore.GetBitmap(self.Game.SYSTEM2B);
               let mut local23: &Bitmap = &bitmap3;
              let mut x: i32 = self.Game.ScreenWidth - 28;
              DrawMod.DrawSimple( local22,  local23, x, 1);
            }
            else
            {
               let mut local24: &Graphics = &graphics;
              bitmap3 = BitmapStore.GetBitmap(self.Game.SYSTEM2);
               let mut local25: &Bitmap = &bitmap3;
              let mut x: i32 = self.Game.ScreenWidth - 28;
              DrawMod.DrawSimple( local24,  local25, x, 1);
            }
          }
        }
        else if (Operators.CompareString(self.GetType().FullName, "WindowsApplication1.RandomScreenClass2", false) == 0 | Operators.CompareString(self.GetType().FullName, "WindowsApplication1.GameLoopScreenClass2", false) == 0)
        {
          DrawMod.DrawBlock( graphics, self.Game.ScreenWidth - 52, 1, 25, 25, 0, 0, 0,  byte.MaxValue);
          DrawMod.DrawBlock( graphics, self.Game.ScreenWidth - 28, 1, 25, 25, 0, 0, 0,  byte.MaxValue);
          bitmap5: Bitmap;
          if (self.doMinimize)
          {
             let mut local26: &Graphics = &graphics;
            bitmap6: Bitmap = BitmapStore.GetBitmap(self.Game.SYSTEM1B);
             let mut local27: &Bitmap = &bitmap6;
            let mut x: i32 = self.Game.ScreenWidth - 52;
            DrawMod.DrawSimple( local26,  local27, x, 1);
          }
          else
          {
             let mut local28: &Graphics = &graphics;
            bitmap5 = BitmapStore.GetBitmap(self.Game.SYSTEM1);
             let mut local29: &Bitmap = &bitmap5;
            let mut x: i32 = self.Game.ScreenWidth - 52;
            DrawMod.DrawSimple( local28,  local29, x, 1);
          }
          if (self.doQuit)
          {
             let mut local30: &Graphics = &graphics;
            bitmap5 = BitmapStore.GetBitmap(self.Game.SYSTEM2B);
             let mut local31: &Bitmap = &bitmap5;
            let mut x: i32 = self.Game.ScreenWidth - 28;
            DrawMod.DrawSimple( local30,  local31, x, 1);
          }
          else
          {
             let mut local32: &Graphics = &graphics;
            bitmap5 = BitmapStore.GetBitmap(self.Game.SYSTEM2);
             let mut local33: &Bitmap = &bitmap5;
            let mut x: i32 = self.Game.ScreenWidth - 28;
            DrawMod.DrawSimple( local32,  local33, x, 1);
          }
        }
      }
      if (Information.IsNothing( self.Game.EditObj.TipText))
        self.Game.EditObj.TipText = "";
      if (self.Game.EditObj.TipText.Length > 0 & self.Game.ModIntroType >= 1)
      {
        SizeF sizeF1 = SizeF::new();
        SizeF sizeF2 = SizeF::new();
        str1: String = self.Game.EditObj.TipTitle;
        bool flag2;
        if (Strings.InStr(str1, "<FIXEDSYS>") > 0)
        {
          flag2 = true;
          str1 = str1.Replace("<FIXEDSYS>", "");
        }
        let mut num1: i32 = 100;
        let mut num2: i32 = 20;
        str2: String = self.Game.EditObj.TipText;
        str3: String = "";
        let mut num3: i32 = 0;
        if (Information.IsNothing( str2))
          str2 = "";
        while (str2.Length > 0)
        {
          Left: String = Strings.Mid(str2, 1, 1);
          if (Operators.CompareString(Left, "\r\n", false) == 0 | Operators.CompareString(Left, "\r", false) == 0 | Operators.CompareString(Left, "\n", false) == 0)
          {
            num3 = 0;
            str3 += Left;
            str2 = Strings.Mid(str2, 2);
          }
          else
          {
            num3 += 1;
            bool flag3 = false;
            if (Strings.InStr(str2, "\r\n") > 0 & Strings.InStr(str2, "\r\n") <= num2)
              flag3 = true;
            if (Strings.InStr(str2, "\n") > 0 & Strings.InStr(str2, "\n") <= num2)
              flag3 = true;
            if (!flag3 & num3 > num1 && Operators.CompareString(Left, " ", false) == 0)
            {
              Left = "\r\n";
              num3 = 0;
            }
            str3 += Left;
            str2 = Strings.Mid(str2, 2);
          }
        }
        SizeF sizeF3 = !flag2 ? graphics.MeasureString(str3, self.Game.MarcFont4) : graphics.MeasureString(str3, self.Game.MarcFont4b);
        let mut x1: i32 = self.FormRef.LastTipX + 20;
        let mut num4: i32 = self.FormRef.LastTipY + 20;
        let mut num5: i32 =  Math.Round( (sizeF3.Width + 4f));
        let mut h: i32 =  Math.Round( (sizeF3.Height + 4f));
        if (str1.Length > 0)
        {
          sizeF2 = !flag2 ? graphics.MeasureString(str1, self.Game.MarcFont4) : graphics.MeasureString(str1, self.Game.MarcFont4b);
          h =  Math.Round( ( (h + 4) + sizeF2.Height));
        }
        float width = sizeF3.Width;
        if ( sizeF2.Width >  width)
          width = sizeF2.Width;
        let mut num6: i32 =  Math.Round( (width + 4f));
        if (x1 + num6 > self.Game.ScreenWidth - 64)
          x1 -= x1 + num6 - (self.Game.ScreenWidth - 64);
        if (num4 + h > self.Game.ScreenHeight - 32)
          num4 -= num4 + h - (self.Game.ScreenHeight - 32);
        self.LastToolTipRect = str1.Length <= 0 ? Rectangle::new(x1 - 16, num4, num6 + 1 + 32, h + 1) : Rectangle::new(x1 - 16, num4, num6 + 1 + 32, h + 1 + 16);
        let mut r1: i32 = 240;
        let mut g1: i32 = 240;
        let mut b1: i32 = 160;
        let mut num7: i32 = 40;
        let mut num8: i32 = 40;
        let mut num9: i32 = 20;
        if (self.Game.EditObj.TipColor >= 1)
        {
          r1 =  byte.MaxValue;
          g1 = 180;
          b1 = 0;
          num7 = 40;
          num8 = 40;
          num9 = 20;
          self.Game.EditObj.TipColor = 0;
        }
        let mut r2: i32 = num7;
        let mut g2: i32 = num8;
        let mut b2: i32 = num9;
        if (flag2)
        {
          r1 = 240;
          g1 = 240;
          b1 = 160;
          num7 = 0;
          num8 = 0;
          num9 = 0;
          r2 = 200;
          g2 = 200;
          b2 = 100;
        }
        if (str1.Length > 0)
        {
          DrawMod.DrawBlock( graphics, x1 - 16, num4, num6 + 32,  Math.Round( (sizeF2.Height + 4f)), r1, g1, b1,  byte.MaxValue);
          DrawMod.DrawBlock( graphics, x1 - 16,  Math.Round( num4 +  sizeF2.Height + 4.0), num6 + 32,  Math.Round( h - ( sizeF2.Height + 4.0) + 16.0), r2, g2, b2,  byte.MaxValue);
          if (flag2)
          {
            DrawMod.DrawTextColouredNicely( graphics, str1, self.Game.MarcFont16, x1 + 2, num4 + 2, Color.FromArgb( byte.MaxValue, num7, num8, num9));
            DrawMod.DrawTextColouredNicely( graphics, str3, self.Game.MarcFont4b, x1 + 3,  Math.Round( ( (num4 + 6 + 8) + sizeF2.Height)), Color.FromArgb(178, 0, 0, 0));
            DrawMod.DrawTextColouredNicely( graphics, str3, self.Game.MarcFont4b, x1 + 2,  Math.Round( ( (num4 + 6 + 8) + sizeF2.Height)), Color.Black);
          }
          else
          {
            DrawMod.DrawTextColouredNicely( graphics, str1, self.Game.MarcFont16, x1 + 2, num4 + 2, Color.FromArgb( byte.MaxValue, num7, num8, num9), 12);
            DrawMod.DrawTextColouredNicely( graphics, str3, self.Game.MarcFont4, x1 + 2,  Math.Round( ( (num4 + 8 + 6) + sizeF2.Height)), Color.White);
          }
          DrawMod.DrawRectangle( graphics, x1 - 16, num4, num6 + 32, h + 16, r1, g1, b1,  byte.MaxValue);
        }
        else if (Operators.CompareString(str3, ".", false) == 0)
        {
          DrawMod.DrawBlock( graphics, x1, num4, 8, 4, num7, num8, num9,  byte.MaxValue);
          DrawMod.DrawRectangle( graphics, x1 + 2, num4 + 2, 1, 1, r1, g1, b1,  byte.MaxValue);
          DrawMod.DrawRectangle( graphics, x1, num4, 8, 4, r1, g1, b1,  byte.MaxValue);
        }
        else
        {
          DrawMod.DrawBlock( graphics, x1 - 16, num4, num6 + 32, h, num7, num8, num9,  byte.MaxValue);
          if (flag2)
            DrawMod.DrawTextColouredNicely( graphics, str3, self.Game.MarcFont4b, x1 + 2, num4 + 2, Color.White);
          else
            DrawMod.DrawTextColouredNicely( graphics, str3, self.Game.MarcFont4, x1 + 2, num4 + 2, Color.White);
          DrawMod.DrawRectangle( graphics, x1 - 16, num4, num6 + 32, h, r1, g1, b1,  byte.MaxValue);
        }
      }
      if (!Information.IsNothing( graphics))
        graphics.Dispose();
      return self.OwnBitmap;
    }

    pub fn ClearOverlaySpecificWindow(id: i32)
    {
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        if (self.WindowID[index] == id)
        {
          self.WindowList[index].clearoverlay();
          self.WindowFlag[index] = true;
          self.LastOverlayWindow = 0;
        }
      }
    }

    pub fn AddWindow(WindowClass tmpWindow, x: i32, y: i32, w: i32, h: i32) -> i32
    {
      this += 1.WindowCounter;
      this += 1.WindowIDCounter;
      self.WindowList = (WindowClass[]) Utils.CopyArray((Array) self.WindowList, (Array) new WindowClass[self.WindowCounter + 1]);
      self.WindowFlag = (bool[]) Utils.CopyArray((Array) self.WindowFlag, (Array) new bool[self.WindowCounter + 1]);
      self.WindowX = (int[]) Utils.CopyArray((Array) self.WindowX, (Array) new int[self.WindowCounter + 1]);
      self.WindowInputBlock = (bool[]) Utils.CopyArray((Array) self.WindowInputBlock, (Array) new bool[self.WindowCounter + 1]);
      self.WindowY = (int[]) Utils.CopyArray((Array) self.WindowY, (Array) new int[self.WindowCounter + 1]);
      self.WindowW = (int[]) Utils.CopyArray((Array) self.WindowW, (Array) new int[self.WindowCounter + 1]);
      self.WindowH = (int[]) Utils.CopyArray((Array) self.WindowH, (Array) new int[self.WindowCounter + 1]);
      self.WindowID = (int[]) Utils.CopyArray((Array) self.WindowID, (Array) new int[self.WindowCounter + 1]);
      self.WindowList[self.WindowCounter] = tmpWindow;
      self.WindowList[self.WindowCounter].formref = self.FormRef;
      self.WindowFlag[self.WindowCounter] = true;
      self.WindowX[self.WindowCounter] = x;
      self.WindowY[self.WindowCounter] = y;
      self.WindowW[self.WindowCounter] = w;
      self.WindowH[self.WindowCounter] = h;
      self.WindowID[self.WindowCounter] = self.WindowIDCounter;
      self.WindowList[self.WindowCounter].screenbackref = self.OwnBackground;
      self.WindowList[self.WindowCounter].screenx = x;
      self.WindowList[self.WindowCounter].screeny = y;
      self.WindowList[self.WindowCounter].screenw = w;
      self.WindowList[self.WindowCounter].screenh = h;
      return self.WindowIDCounter;
    }

    pub AddWindow: i32(
      WindowClass tmpWindow,
      x: i32,
      y: i32,
      w: i32,
      h: i32,
      Rectangle tShowRectangle)
    {
      this += 1.WindowCounter;
      this += 1.WindowIDCounter;
      self.WindowList = (WindowClass[]) Utils.CopyArray((Array) self.WindowList, (Array) new WindowClass[self.WindowCounter + 1]);
      self.WindowFlag = (bool[]) Utils.CopyArray((Array) self.WindowFlag, (Array) new bool[self.WindowCounter + 1]);
      self.WindowX = (int[]) Utils.CopyArray((Array) self.WindowX, (Array) new int[self.WindowCounter + 1]);
      self.WindowInputBlock = (bool[]) Utils.CopyArray((Array) self.WindowInputBlock, (Array) new bool[self.WindowCounter + 1]);
      self.WindowY = (int[]) Utils.CopyArray((Array) self.WindowY, (Array) new int[self.WindowCounter + 1]);
      self.WindowW = (int[]) Utils.CopyArray((Array) self.WindowW, (Array) new int[self.WindowCounter + 1]);
      self.WindowH = (int[]) Utils.CopyArray((Array) self.WindowH, (Array) new int[self.WindowCounter + 1]);
      self.WindowID = (int[]) Utils.CopyArray((Array) self.WindowID, (Array) new int[self.WindowCounter + 1]);
      self.WindowList[self.WindowCounter] = tmpWindow;
      self.WindowList[self.WindowCounter].formref = self.FormRef;
      self.WindowFlag[self.WindowCounter] = true;
      self.WindowX[self.WindowCounter] = x;
      self.WindowY[self.WindowCounter] = y;
      self.WindowW[self.WindowCounter] = w;
      self.WindowH[self.WindowCounter] = h;
      self.WindowID[self.WindowCounter] = self.WindowIDCounter;
      self.WindowList[self.WindowCounter].screenbackref = self.OwnBackground;
      self.WindowList[self.WindowCounter].screenx = x;
      self.WindowList[self.WindowCounter].screeny = y;
      self.WindowList[self.WindowCounter].screenw = w;
      self.WindowList[self.WindowCounter].screenh = h;
      self.WindowList[self.WindowCounter].ShowRect = tShowRectangle;
      self.WindowList[self.WindowCounter].DoShowRect = true;
      return self.WindowIDCounter;
    }

    pub fn RemoveWindow(id: i32)
    {
      let mut index1: i32 = -1;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index2: i32 = 0; index2 <= windowCounter; index2 += 1)
      {
        if (self.WindowID[index2] == id)
        {
          index1 = index2;
          break;
        }
      }
      if (index1 == -1 || Information.IsNothing( self.WindowList[index1]))
        return;
      Graphics objGraphics = Graphics.FromImage((Image) self.OwnBitmap);
      if (Operators.CompareString(self.WindowList[index1].GetType().FullName, "WindowsApplication1.UdsUnitOpsWindowClass", false) == 0)
      {
        System.Type WC = typeof (MapWindowClass2);
        WindowClass window = self.GetWindow( WC);
        Rectangle rect = Rectangle::new(self.WindowX[index1], self.WindowY[index1], self.WindowW[index1], self.WindowH[index1]);
        DrawMod.DrawSimplePart( objGraphics,  window.SubPartList[0].OwnBitmap, rect);
      }
      else
      {
        Rectangle rect = Rectangle::new(self.WindowX[index1], self.WindowY[index1], self.WindowW[index1], self.WindowH[index1]);
        DrawMod.DrawSimplePart( objGraphics,  self.OwnBackground, rect);
      }
      self.WindowList[index1].Dispose();
      self.WindowList[index1] = (WindowClass) null;
      if (index1 < self.WindowCounter)
      {
        let mut num1: i32 = index1;
        let mut num2: i32 = self.WindowCounter - 1;
        for (let mut index3: i32 = num1; index3 <= num2; index3 += 1)
        {
          self.WindowList[index3] = self.WindowList[index3 + 1];
          self.WindowFlag[index3] = self.WindowFlag[index3 + 1];
          self.WindowX[index3] = self.WindowX[index3 + 1];
          self.WindowInputBlock[index3] = self.WindowInputBlock[index3 + 1];
          self.WindowY[index3] = self.WindowY[index3 + 1];
          self.WindowW[index3] = self.WindowW[index3 + 1];
          self.WindowH[index3] = self.WindowH[index3 + 1];
          self.WindowID[index3] = self.WindowID[index3 + 1];
        }
      }
      --self.WindowCounter;
      self.WindowList = (WindowClass[]) Utils.CopyArray((Array) self.WindowList, (Array) new WindowClass[self.WindowCounter + 1]);
      self.WindowFlag = (bool[]) Utils.CopyArray((Array) self.WindowFlag, (Array) new bool[self.WindowCounter + 1]);
      self.WindowX = (int[]) Utils.CopyArray((Array) self.WindowX, (Array) new int[self.WindowCounter + 1]);
      self.WindowInputBlock = (bool[]) Utils.CopyArray((Array) self.WindowInputBlock, (Array) new bool[self.WindowCounter + 1]);
      self.WindowY = (int[]) Utils.CopyArray((Array) self.WindowY, (Array) new int[self.WindowCounter + 1]);
      self.WindowW = (int[]) Utils.CopyArray((Array) self.WindowW, (Array) new int[self.WindowCounter + 1]);
      self.WindowH = (int[]) Utils.CopyArray((Array) self.WindowH, (Array) new int[self.WindowCounter + 1]);
      self.WindowID = (int[]) Utils.CopyArray((Array) self.WindowID, (Array) new int[self.WindowCounter + 1]);
      if (Information.IsNothing( objGraphics))
        return;
      objGraphics.Dispose();
    }

    pub virtual ScreenReturnClass HandleMouseClick(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass;
      return screenReturnClass;
    }

    pub fn HandleTooltip(x: i32, y: i32, bool skipReset = false)
    {
      num1: i32;
      index1: i32;
      if (!skipReset)
      {
        self.Game.EditObj.TipColor = 0;
        self.Game.EditObj.TipText = "";
        self.Game.EditObj.TipTitle = "";
        self.Game.EditObj.TipButton = false;
      }
      else
        index1 = num1;
      if (self.Game.ModIntroType == 0 || self.Game.EditObj.InEditor && !self.Game.Data.SimpleEditor)
        return;
      if (!skipReset && self.WindowCounter > -1)
      {
        let mut windowCounter: i32 = self.WindowCounter;
        for (index1 = 0; index1 <= windowCounter; index1 += 1)
        {
          if (x > self.WindowX[index1] & x < self.WindowX[index1] + self.WindowW[index1] && y > self.WindowY[index1] & y < self.WindowY[index1] + self.WindowH[index1])
            self.WindowList[index1].HandleToolTip(x - self.WindowX[index1], y - self.WindowY[index1]);
        }
      }
      if (Information.IsNothing( self.Game.EditObj.TipText))
        self.Game.EditObj.TipText = "";
      if (Strings.InStr(self.Game.EditObj.TipText, "<BR>") > 0)
        self.Game.EditObj.TipText = self.Game.EditObj.TipText.Replace("<BR>", "\r\n");
      if (Strings.InStr(self.Game.EditObj.TipText, "<br>") > 0)
        self.Game.EditObj.TipText = self.Game.EditObj.TipText.Replace("<br>", "\r\n");
      if (self.Game.EditObj.TipText.Length > 0)
      {
        let mut num2: i32 = num2;
      }
      if (self.Game.EditObj.TipText.Length > 90)
      {
        let mut num3: i32 = -1;
        let mut Start: i32 = 1;
        let mut num4: i32 = 1;
        while ((num3 == -1 | num3 - Start > 90) & num4 == 1)
        {
          num3 = Strings.InStr(Start, self.Game.EditObj.TipText, "\r\n");
          num4 = 0;
          if (num3 == 0 | num3 - Start > 90 && self.Game.EditObj.TipText.Length - Start > 90)
          {
            let mut num5: i32 = Start;
            num6: i32;
            while (num5 - Start < 90)
            {
              num6 = num5;
              num5 = Strings.InStr(num5 + 1, self.Game.EditObj.TipText, " ");
              if (num5 == 0)
                break;
            }
            let mut num7: i32 = num6;
            if (num7 > 0 & num7 - Start < 90)
            {
              num4 = 1;
              self.Game.EditObj.TipText = Strings.Left(self.Game.EditObj.TipText, num7 - 1) + "\r\n" + Strings.Right(self.Game.EditObj.TipText, self.Game.EditObj.TipText.Length - num7);
              Start = num7 + 1;
            }
          }
        }
      }
      if (!Information.IsNothing( self.FormRef))
      {
        if (!(self.FormRef.RightMousePressed | self.Game.EditObj.MouseOverVisible) & self.Game.EditObj.TipText.Length > 0)
        {
          self.Game.EditObj.TipTitle = "";
          self.Game.EditObj.TipText = "";
          if (self.Game.EditObj.TipButton)
          {
            if (!(self.FormRef.Cursor == Cursors.WaitCursor))
              self.FormRef.Cursor = Cursors.Hand;
          }
          else if (!(self.FormRef.Cursor == Cursors.WaitCursor))
            self.FormRef.Cursor = Cursors.Help;
        }
        else if (!((self.FormRef.RightMousePressed | self.Game.EditObj.MouseOverVisible) & self.Game.EditObj.TipText.Length > 0) && !(self.FormRef.Cursor == Cursors.WaitCursor) & !(self.FormRef.Cursor == Cursors.Default))
          self.FormRef.Cursor = Cursors.Default;
      }
      num1 = Operators.CompareString(self.Game.EditObj.TipText, "", false) != 0 ? index1 : index1;
      if (((self.Game.Data.Product >= 7 ? 1 : 0) & 0) == 0 || self.Game.HelpCounter <= -1 || Strings.InStr(self.Game.EditObj.TipText, "@") <= 0)
        return;
      SimpleStringList simpleStringList = SimpleStringList::new();
      let mut Start1: i32 = Strings.InStr(self.Game.EditObj.TipText, "@");
      if (Start1 <= 0)
        return;
      let mut num8: i32 = Strings.InStr(Start1 + 1, self.Game.EditObj.TipText, "@");
      if (num8 <= 0)
        return;
      str: String = Strings.Mid(self.Game.EditObj.TipText, Start1, num8 - Start1 + 1);
      oldValue: String = str;
      strArray: Vec<String> = str.Replace("@", "").Split(':');
      newValue: String = "";
      if (strArray.Length >= 2)
      {
        let mut helpCounter: i32 = self.Game.HelpCounter;
        for (let mut index2: i32 = 0; index2 <= helpCounter; index2 += 1)
        {
          if (Operators.CompareString(self.Game.HelpFile[index2], strArray[1], false) == 0 && Operators.CompareString(self.Game.HelpDir[index2], strArray[0], false) == 0)
          {
            newValue = self.Game.HelpText[index2];
            break;
          }
        }
      }
      else if (strArray.Length < 1)
        ;
      self.Game.EditObj.TipText = self.Game.EditObj.TipText.Replace(oldValue, newValue);
    }

    pub virtual ScreenReturnClass HandleMouseUp(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      screenReturnClass.flag = false;
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        windowReturnClass: WindowReturnClass = self.WindowList[index].HandleMouseUp(x - self.WindowX[index], y - self.WindowY[index], b);
        self.WindowFlag[index] = windowReturnClass.Flag;
        if (windowReturnClass.Flag)
          screenReturnClass.flag = windowReturnClass.Flag;
      }
      return screenReturnClass;
    }

    pub fn HandleBLOCKEDMouseUp(x: i32, y: i32, b: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      screenReturnClass.flag = false;
      if (self.WindowCounter <= -1)
        return;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        if (!Information.IsNothing( self.WindowList[index]))
        {
          windowReturnClass: WindowReturnClass = self.WindowList[index].HandleBLOCKEDMouseUp(x - self.WindowX[index], y - self.WindowY[index], b);
          self.WindowFlag[index] = windowReturnClass.Flag;
          if (windowReturnClass.Flag)
            screenReturnClass.flag = windowReturnClass.Flag;
        }
      }
    }

    pub virtual ScreenReturnClass HandleKeyPress(nr: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        windowReturnClass2: WindowReturnClass = self.WindowList[index].HandleKeyPress(nr);
        self.WindowFlag[index] = windowReturnClass2.Flag;
        if (windowReturnClass2.Flag)
          return screenReturnClass;
      }
      screenReturnClass.flag = true;
      return screenReturnClass;
    }

    pub virtual ScreenReturnClass HandleKeyup(nr: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        windowReturnClass2: WindowReturnClass = self.WindowList[index].HandleKeyup(nr);
        self.WindowFlag[index] = windowReturnClass2.Flag;
      }
      screenReturnClass.flag = true;
      return screenReturnClass;
    }

    pub virtual ScreenReturnClass HandleTimer()
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      bool flag = false;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index1: i32 = 0; index1 <= windowCounter; index1 += 1)
      {
        windowReturnClass2: WindowReturnClass = self.WindowList[index1].handleTimer();
        self.WindowFlag[index1] = windowReturnClass2.Flag;
        if (self.WindowFlag[index1])
          flag = true;
        if (windowReturnClass2.Counter > -1)
        {
          let mut counter: i32 = windowReturnClass2.Counter;
          for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
          {
            if (windowReturnClass2.CommandType[index2] == 3)
            {
              screenReturnClass.NewScreen = windowReturnClass2.CommandData[index2];
              return screenReturnClass;
            }
          }
        }
      }
      screenReturnClass.flag = flag;
      return screenReturnClass;
    }

    pub virtual ScreenReturnClass HandleTimerWheel(x: i32, y: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter > -1)
      {
        let mut windowCounter: i32 = self.WindowCounter;
        for (let mut index1: i32 = 0; index1 <= windowCounter; index1 += 1)
        {
          if (x > self.WindowX[index1] & y > self.WindowY[index1] & x < self.WindowX[index1] + self.WindowW[index1] & y < self.WindowY[index1] + self.WindowH[index1])
          {
            windowReturnClass2: WindowReturnClass = self.WindowList[index1].handleTimerWheel(x - self.WindowX[index1], y - self.WindowY[index1]);
            if (windowReturnClass2.Flag)
            {
              screenReturnClass.flag = true;
              self.WindowFlag[index1] = true;
              if (windowReturnClass2.Counter > -1)
              {
                let mut counter: i32 = windowReturnClass2.Counter;
                for (let mut index2: i32 = 0; index2 <= counter; index2 += 1)
                {
                  if (windowReturnClass2.CommandType[index2] == 3)
                  {
                    screenReturnClass.NewScreen = windowReturnClass2.CommandData[index2];
                    return screenReturnClass;
                  }
                }
              }
              return screenReturnClass;
            }
            screenReturnClass.flag = false;
            return screenReturnClass;
          }
        }
        self.Game.EditObj.MouseWheel = 0;
        screenReturnClass.flag = false;
        return screenReturnClass;
      }
      self.Game.EditObj.MouseWheel = 0;
      screenReturnClass.flag = false;
      return screenReturnClass;
    }

    pub virtual ScreenReturnClass HandleMouseMove(x: i32, y: i32)
    {
      ScreenReturnClass screenReturnClass = ScreenReturnClass::new();
      if (self.WindowCounter <= -1)
        return screenReturnClass;
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        self.WindowList[index].MouseInThisWindow = false;
        if (x >= self.WindowX[index] & x < self.WindowX[index] + self.WindowW[index] && y >= self.WindowY[index] & y < self.WindowY[index] + self.WindowH[index])
          self.WindowList[index].MouseInThisWindow = true;
        windowReturnClass: WindowReturnClass = self.WindowList[index].HandleMouseMove(x - self.WindowX[index], y - self.WindowY[index]);
        self.WindowFlag[index] = windowReturnClass.Flag;
        if (windowReturnClass.Overlay | self.LastOverlayWindow > 0)
        {
          if (self.LastOverlayWindow > 0 & self.LastOverlayWindow != self.WindowID[index])
            self.ClearOverlaySpecificWindow(self.LastOverlayWindow);
          if (self.LastOverlayWindow > 0 & !windowReturnClass.Overlay)
            self.LastOverlayWindow = 0;
          if (windowReturnClass.Overlay)
            self.LastOverlayWindow = self.WindowID[index];
          screenReturnClass.flag = true;
          return screenReturnClass;
        }
        if (windowReturnClass.Flag)
        {
          screenReturnClass.flag = windowReturnClass.Flag;
          return screenReturnClass;
        }
      }
      if (self.LastOverlayWindow > 0)
      {
        self.ClearOverlaySpecificWindow(self.LastOverlayWindow);
        screenReturnClass.flag = true;
      }
      else
        screenReturnClass.flag = false;
      if (self.Game.Data.Product < 7)
      {
        if (x > self.Game.ScreenWidth - 52 & x < self.Game.ScreenWidth - 28 & y < 25)
        {
          if (!self.doMinimize)
          {
            self.doMinimize = true;
            screenReturnClass.flag = true;
          }
        }
        else if (self.doMinimize)
        {
          self.doMinimize = false;
          screenReturnClass.flag = true;
        }
        if (x > self.Game.ScreenWidth - 28 & x < self.Game.ScreenWidth - 4 & y < 25)
        {
          if (!self.doQuit)
          {
            self.doQuit = true;
            screenReturnClass.flag = true;
          }
        }
        else if (self.doQuit)
        {
          self.doQuit = false;
          screenReturnClass.flag = true;
        }
      }
      else if (Operators.CompareString(self.GetType().FullName, "WindowsApplication1.RandomScreenClass2", false) == 0 | Operators.CompareString(self.GetType().FullName, "WindowsApplication1.GameLoopMainWindowClass2", false) == 0)
      {
        if (x > self.Game.ScreenWidth - 52 & x < self.Game.ScreenWidth - 28 & y < 25)
        {
          if (!self.doMinimize)
          {
            self.doMinimize = true;
            screenReturnClass.flag = true;
          }
        }
        else if (self.doMinimize)
        {
          self.doMinimize = false;
          screenReturnClass.flag = true;
        }
        if (x > self.Game.ScreenWidth - 28 & x < self.Game.ScreenWidth - 4 & y < 25)
        {
          if (!self.doQuit)
          {
            self.doQuit = true;
            screenReturnClass.flag = true;
          }
        }
        else if (self.doQuit)
        {
          self.doQuit = false;
          screenReturnClass.flag = true;
        }
      }
      return screenReturnClass;
    }

    pub fn DoTutorial(Graphics g)
    {
      color: Color = Color.FromArgb( byte.MaxValue,  byte.MaxValue, 28, 0);
      self.Game.EditObj.Zoom = 0;
      System.Type WC1 = typeof (IntroWindowClass2);
      if (self.WindowPresent( WC1))
      {
        self.Game.EditObj.TutStep = 0;
        DrawMod.DrawTutback(g, 5, 5, 960, 160,  color.R,  color.G,  color.B,  color.A);
        DrawMod.DrawTextColouredOutline( g, "Hi! Let me introduce myself. I am Vic, the designer of the game. Welcome to the tutorial. It will go over some ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
        DrawMod.DrawTextColouredOutline( g, "key concepts and orders. The tutorial is not exhaustive and it is advised that you read the manual too.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
        DrawMod.DrawTextColouredOutline( g, "One of the most important things for new players to know is that you can right click on everything where the mouse", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 70, Color.White);
        DrawMod.DrawTextColouredOutline( g, "shows a question mark or hand.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 95, Color.White);
        DrawMod.DrawTextColouredOutline( g, "Now please press 'start' to start the tutorial.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 135, Color.White);
        let mut num1: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
        let mut num2: i32 =  Math.Round( (self.Game.ScreenHeight - 768) / 2.0);
        let mut num3: i32 = num1 + 845;
        let mut num4: i32 = num2 + 650;
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = num3;
        let mut y: i32 = num4;
        DrawMod.DrawSimple( local1,  local2, x, y);
      }
      else
      {
        System.Type WC2 = typeof (CombatResultWindowClass2);
        num5: i32;
        if (self.WindowPresent( WC2))
        {
          let mut num6: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
          num5 = 0;
          if (self.Game.EditObj.TutStep == 13 | self.Game.EditObj.TutStep == 18)
          {
            self.Game.EditObj.TutStep = 18;
            self.Game.EditObj.TutX =  12;
            self.Game.EditObj.TutY =  7;
            self.Game.EditObj.TutOrder = 9999;
            DrawMod.DrawTutback(g, 5, 5, 800, 200,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "The battle is being fought combat round by combat round.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
            if ((uint) self.Game.TempCombat.BattleEnded > 0U)
            {
              DrawMod.DrawTextColouredOutline( g, "And once ended you can inspect the battle details or return back to the main screen. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Lets go back to main screen. Click OK.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Note: What your basically seeing in the battle screen is each side's participating units.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 100, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Each unit's troops are displayed. Troops in the middle columns are still participating ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 120, Color.White);
              DrawMod.DrawTextColouredOutline( g, "in combat. And troops in the side columns have been killed or have retreated from combat.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 140, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Battle ends when one of the sides has no troops participating anymore. In a nutshell ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 160, Color.White);
              DrawMod.DrawTextColouredOutline( g, "that is what happens. If you are not easily shaken then click on DETAILS to see whats really going on.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 180, Color.White);
            }
            let mut num7: i32 =  Math.Round( self.Game.ScreenWidth / 2.0);
            let mut num8: i32 =  Math.Round( self.Game.ScreenHeight / 2.0);
            let mut x1_1: i32 = num7 - 200;
            let mut y1: i32 = num8 - 150;
            DrawMod.DrawBlock( g, x1_1, y1, 5, 300,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawBlock( g, x1_1 + 90, y1 + 20, 220, 25,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "PARTICIPATING TROOPS", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), x1_1 + 95, y1 + 20, Color.White);
            let mut num9: i32 = x1_1 - 350;
            DrawMod.DrawBlock( g, num9 + 110, y1 + 20, 220, 25,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "RETREATED TROOPS", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), num9 + 120, y1 + 20, Color.White);
            let mut x1_2: i32 =  Math.Round( self.Game.ScreenWidth / 2.0) + 190;
            DrawMod.DrawBlock( g, x1_2, y1, 5, 300,  color.R,  color.G,  color.B,  color.A);
            let mut num10: i32 = x1_2 - 80;
            DrawMod.DrawBlock( g, num10 + 110, y1 + 20, 220, 25,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "RETREATED TROOPS", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), num10 + 120, y1 + 20, Color.White);
            let mut num11: i32 =  Math.Round( self.Game.ScreenWidth / 2.0);
            let mut num12: i32 =  Math.Round( self.Game.ScreenHeight / 2.0);
            num5 = 1;
          }
          if (!(self.Game.EditObj.TutStep == 27 | self.Game.EditObj.TutStep == 30))
            return;
          self.Game.EditObj.TutStep = 30;
          self.Game.EditObj.TutX =  -1;
          self.Game.EditObj.TutY =  -1;
          self.Game.Data.MapObj[0].HexObj[self.Game.EditObj.TargetX, self.Game.EditObj.TargetY].set_BattlePenalty(0, 12);
          DrawMod.DrawTutback(g, 5, 5, 900, 90,  color.R,  color.G,  color.B,  color.A);
          DrawMod.DrawTextColouredOutline( g, "And another battle commences.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
          if ((uint) self.Game.TempCombat.BattleEnded > 0U)
            DrawMod.DrawTextColouredOutline( g, "And of course won by your stronger forces. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
          num5 = 1;
        }
        else
        {
          System.Type WC3 = typeof (PlayExtraWindowClass2);
          let mut num13: i32 = self.WindowPresent( WC3) ? 1 : 0;
          System.Type WC4 = typeof (StrategicWindowClass2);
          let mut num14: i32 = self.WindowPresent( WC4) ? 1 : 0;
          if ((num13 | num14) != 0)
          {
            let mut num15: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
            num5 = 0;
            if (self.Game.EditObj.TutStep == 30)
            {
              DrawMod.DrawTutback(g, 5, 5, 850, 210,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You now see a black box with a number on the hex you just conquered. This is so called remaining 'battle AP penalty' ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "and it will cause a movement penalty on units that did not participate in the combat for taking the hex. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "(This rule makes it possible for the defender to delay the whole enemy army with one properly defended roadblock)", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "And that concludes this short tutorial. It handled some key concepts, but I advise you to read the manual now.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 80, Color.White);
              DrawMod.DrawTextColouredOutline( g, "In a normal game you would now press", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 105, Color.White);
               let mut local3: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.MARCBACK4);
               let mut local4: &Bitmap = &bitmap;
              DrawMod.DrawSimple( local3,  local4, 95, 138);
               let mut local5: &Graphics = &g;
              bitmap = BitmapStore.GetBitmap(self.Game.BUTTONNEXT);
               let mut local6: &Bitmap = &bitmap;
              DrawMod.DrawSimple( local5,  local6, 95, 138);
              DrawMod.DrawTextColouredOutline( g, "the next turn button, but the tutorial has no next turn. You have to use the 'quit' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 130, 155, Color.White);
              DrawMod.DrawTextColouredOutline( g, "It's in the top-right corner. I will be available on the forums for any questions. Thanks for your attention and happy gaming!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 180, Color.White);
              let mut num16: i32 = self.Game.ScreenWidth - 20;
              let mut y1: i32 = 35;
              DrawMod.drawLine( g, num16, y1, num16, y1 + 40,  color.R,  color.G,  color.B,  color.A, 4);
              DrawMod.drawLine( g, num16, y1, num16 - 10, y1 + 10,  color.R,  color.G,  color.B,  color.A, 4);
              DrawMod.drawLine( g, num16, y1, num16 + 10, y1 + 10,  color.R,  color.G,  color.B,  color.A, 4);
              num5 = 1;
            }
            let mut num17: i32 = 0;
            bitmap1: Bitmap;
            if (self.Game.EditObj.TutStep == 27 & num17 == 0 && self.Game.EditObj.OrderType == 2 & self.Game.EditObj.TempUnitList.counter > -1)
            {
              let mut num18: i32 = num15 + 825;
              let mut num19: i32 = self.Game.ScreenHeight - 360;
               let mut local7: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local8: &Bitmap = &bitmap1;
              let mut x: i32 = num18;
              let mut y: i32 = num19;
              DrawMod.DrawSimple( local7,  local8, x, y);
              num5 = 1;
              DrawMod.DrawTutback(g, 5, 5, 960, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Alright and now press attack!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 27 & num17 == 0 && self.Game.EditObj.OrderType == 2)
            {
              self.Game.EditObj.TutStep = 27;
              let mut num20: i32 = num15 + 956;
              let mut num21: i32 = self.Game.ScreenHeight - 360;
               let mut local9: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local10: &Bitmap = &bitmap1;
              let mut x: i32 = num20;
              let mut y: i32 = num21;
              DrawMod.DrawSimple( local9,  local10, x, y);
              num5 = 1;
              DrawMod.DrawTutback(g, 5, 5, 960, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "To select all available units to join in the attack press the 'ALL' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 27 & num17 == 0 && self.Game.SelectX == 15 & self.Game.SelectY == 4)
            {
              if (self.Game.EditObj.TutOrder != 2)
              {
                self.Game.EditObj.TutOrder = 2;
                Graphics g1 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local11 =  WC4;
                self.PaintPresentWindow(g1,  local11);
                Graphics g2 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local12 =  WC4;
                self.PaintPresentWindow(g2,  local12);
                Graphics g3 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local13 =  WC4;
                self.PaintPresentWindow(g3,  local13);
              }
              let mut num22: i32 = num15 + 75;
              let mut num23: i32 = self.Game.ScreenHeight - 360;
               let mut local14: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local15: &Bitmap = &bitmap1;
              let mut x: i32 = num22;
              let mut y: i32 = num23;
              DrawMod.DrawSimple( local14,  local15, x, y);
              num5 = 1;
              DrawMod.DrawTutback(g, 5, 5, 960, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Alright. Now click the attack button so you can start to select the participants in the attack on this hex.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num17 = 1;
            }
            if ((self.Game.EditObj.TutStep == 24 | self.Game.EditObj.TutStep == 27) & num17 == 0 && Operators.CompareString(self.Game.Data.UnitObj[self.Game.Data.UnitObj[self.Game.HandyFunctionsObj.GetUnitByHistorical(281)].HQ].Name, "OKH", false) == 0)
            {
              self.Game.EditObj.TutStep = 27;
              if (self.Game.EditObj.TutOrder != 9999)
              {
                self.Game.EditObj.TutOrder = 9999;
                self.Game.EditObj.TutX =  15;
                self.Game.EditObj.TutY =  4;
                Graphics g4 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local16 =  WC4;
                self.PaintPresentWindow(g4,  local16);
                Graphics g5 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local17 =  WC4;
                self.PaintPresentWindow(g5,  local17);
                Graphics g6 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local18 =  WC4;
                self.PaintPresentWindow(g6,  local18);
              }
              DrawMod.DrawTutback(g, 5, 5, 960, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Very well. You can see the HQ change reflected in the colored bar of the unit. It's now brown just as the OKH. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now I will show the concept of battle AP penalties. For this you have to start a battle first. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please click on the selected enemy unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 24 & num17 == 0 && self.Game.EditObj.OrderType == 3)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g7 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local19 =  WC4;
                self.PaintPresentWindow(g7,  local19);
                Graphics g8 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local20 =  WC4;
                self.PaintPresentWindow(g8,  local20);
                Graphics g9 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local21 =  WC4;
                self.PaintPresentWindow(g9,  local21);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "The game wants to know what unit should be the new HQ for the motorized regiment.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please click on the OKH and then on the confirm button to make that the HQ.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.SelectX == 8 & self.Game.SelectY == 7)
              {
                let mut num24: i32 = num15 + 723;
                let mut num25: i32 = self.Game.ScreenHeight - 360;
                 let mut local22: &Graphics = &g;
                bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
                 let mut local23: &Bitmap = &bitmap1;
                let mut x: i32 = num24;
                let mut y: i32 = num25;
                DrawMod.DrawSimple( local22,  local23, x, y);
              }
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 24 & num17 == 0 && self.Game.SelectX == 10 & self.Game.SelectY == 4 && !self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 24;
              if (self.Game.EditObj.TutOrder != 3)
              {
                self.Game.EditObj.TutOrder = 3;
                Graphics g10 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local24 =  WC4;
                self.PaintPresentWindow(g10,  local24);
                Graphics g11 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local25 =  WC4;
                self.PaintPresentWindow(g11,  local25);
                Graphics g12 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local26 =  WC4;
                self.PaintPresentWindow(g12,  local26);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You have now selected the Motorized Unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Click the highlighted 'HQ' order.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              let mut num26: i32 = num15 + 143;
              let mut num27: i32 = self.Game.ScreenHeight - 360;
               let mut local27: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local28: &Bitmap = &bitmap1;
              let mut x: i32 = num26;
              let mut y: i32 = num27;
              DrawMod.DrawSimple( local27,  local28, x, y);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 24 & num17 == 0 && !(self.Game.SelectX == 10 & self.Game.SelectY == 4) && !self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 24;
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutY,  4, false))))
              {
                self.Game.EditObj.TutX =  10;
                self.Game.EditObj.TutY =  4;
                self.Game.SelectX = 8;
                self.Game.SelectY = 6;
                Graphics g13 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local29 =  WC4;
                self.PaintPresentWindow(g13,  local29);
                Graphics g14 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local30 =  WC4;
                self.PaintPresentWindow(g14,  local30);
                Graphics g15 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local31 =  WC4;
                self.PaintPresentWindow(g15,  local31);
                Graphics g16 = g;
                WC4 = typeof (PlayExtraWindowClass2);
                 System.Type local32 =  WC4;
                self.PaintPresentWindow(g16,  local32);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Now I'll show how to change a units HQ. Please now select the Motorized Regiment.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              let mut num28: i32 = num15 + 143;
              let mut num29: i32 = self.Game.ScreenHeight - 360;
               let mut local33: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local34: &Bitmap = &bitmap1;
              let mut x: i32 = num28;
              let mut y: i32 = num29;
              DrawMod.DrawSimple( local33,  local34, x, y);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 21 | self.Game.EditObj.TutStep == 24 && self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 24;
              DrawMod.DrawTutback(g, 5, 5, 870, 80,  color.R,  color.G,  color.B,  color.A, true);
              DrawMod.DrawTextColouredOutline( g, "You see that the supply flowing through I Corps is strained. This is because the supply from I Corps source, the OKH,", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "has to cross the rivers with blown bridges to get to I Corps. This already costs 148 AP.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Supply being issued from I Corps thus starts with a huge penalty already. De-activate supply layer now please.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              let mut num30: i32 = num15 + 38;
              let mut num31: i32 = self.Game.ScreenHeight - 360;
               let mut local35: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local36: &Bitmap = &bitmap1;
              let mut x: i32 = num30;
              let mut y: i32 = num31;
              DrawMod.DrawSimple( local35,  local36, x, y);
              num5 = 1;
              num17 = 1;
            }
            if ((self.Game.EditObj.TutStep == 20 | self.Game.EditObj.TutStep == 21) & num17 == 0 && !self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 21;
              self.Game.EditObj.TutOrder = 51;
              let mut num32: i32 = num15 + 770;
              let mut num33: i32 = self.Game.ScreenHeight - 360;
               let mut local37: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local38: &Bitmap = &bitmap1;
              let mut x: i32 = num32;
              let mut y: i32 = num33;
              DrawMod.DrawSimple( local37,  local38, x, y);
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  10, false))))
              {
                self.Game.EditObj.TutX =  10;
                self.Game.EditObj.TutY =  0;
                Graphics g17 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local39 =  WC4;
                self.PaintPresentWindow(g17,  local39);
                Graphics g18 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local40 =  WC4;
                self.PaintPresentWindow(g18,  local40);
                Graphics g19 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local41 =  WC4;
                self.PaintPresentWindow(g19,  local41);
                Graphics g20 = g;
                WC4 = typeof (PlayExtraWindowClass2);
                 System.Type local42 =  WC4;
                self.PaintPresentWindow(g20,  local42);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 55,  color.R,  color.G,  color.B,  color.A, true);
              DrawMod.DrawTextColouredOutline( g, "I was saying the Motorized Regiment has the I Corps as its HQ.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please select the I Corps and press the supply layer button again.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
            }
            let mut num34: i32 = 0;
            if (self.Game.EditObj.TutStep == 19 | self.Game.EditObj.TutStep == 20 && self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 20;
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  10, false))))
              {
                self.Game.EditObj.TutX =  12;
                self.Game.EditObj.TutY =  0;
                Graphics g21 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local43 =  WC4;
                self.PaintPresentWindow(g21,  local43);
                Graphics g22 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local44 =  WC4;
                self.PaintPresentWindow(g22,  local44);
                Graphics g23 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local45 =  WC4;
                self.PaintPresentWindow(g23,  local45);
              }
              DrawMod.DrawTutback(g, 5, 5, 800, 175,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "That's it. You now see how supply flows from your HQ to your units. Click on a hex of choice to see exact path.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Click on back button to hide the supply layer again.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "There is something important here you have to understand. It may seem to you that the", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Motorized Regiment is in green supply zone. However the red supply sign on its counter is a tell-tale", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 80, Color.White);
              DrawMod.DrawTextColouredOutline( g, "it is not getting enough supply. Why? That is because supply comes from I Corps and not from the OKH.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 105, Color.White);
              DrawMod.DrawTextColouredOutline( g, "If you would have selected I Corps as HQ and then activated supply layer you would have seen a different picture.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 130, Color.White);
              DrawMod.DrawTextColouredOutline( g, "You will check how the situation looks from I Corps after you have closed the supply layer again.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 155, Color.White);
              let mut num35: i32 = num15 + 38;
              let mut num36: i32 = self.Game.ScreenHeight - 360;
               let mut local46: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local47: &Bitmap = &bitmap1;
              let mut x: i32 = num35;
              let mut y: i32 = num36;
              DrawMod.DrawSimple( local46,  local47, x, y);
              num5 = 1;
              num34 = 1;
            }
            if ((self.Game.EditObj.TutStep == 18 | self.Game.EditObj.TutStep == 19) & num34 == 0 && self.Game.Data.MapObj[0].HexObj[6, 8].UnitCounter > -1)
            {
              self.Game.EditObj.TutStep = 19;
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  8, false))))
              {
                self.Game.EditObj.TutX =  8;
                self.Game.EditObj.TutY =  7;
                Graphics g24 = g;
                WC4 = typeof (StrategicWindowClass2);
                 System.Type local48 =  WC4;
                self.PaintPresentWindow(g24,  local48);
                Graphics g25 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local49 =  WC4;
                self.PaintPresentWindow(g25,  local49);
                Graphics g26 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local50 =  WC4;
                self.PaintPresentWindow(g26,  local50);
                Graphics g27 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local51 =  WC4;
                self.PaintPresentWindow(g27,  local51);
              }
              DrawMod.DrawTutback(g, 5, 5, 960, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "That's how you strategically transfer units. Now I will show how the supply layer can be activated.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "For this you need to select a HQ and then press the supply layer button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please select the OKH unit and activate the supply layer!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              self.Game.EditObj.TutOrder = 51;
              let mut num37: i32 = num15 + 770;
              let mut num38: i32 = self.Game.ScreenHeight - 360;
               let mut local52: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local53: &Bitmap = &bitmap1;
              let mut x: i32 = num37;
              let mut y: i32 = num38;
              DrawMod.DrawSimple( local52,  local53, x, y);
              num34 = 1;
            }
            if (self.Game.EditObj.TutStep == 18 && self.Game.EditObj.OrderType == 18)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  6, false))))
              {
                self.Game.EditObj.TutX =  6;
                self.Game.EditObj.TutY =  8;
                Graphics g28 = g;
                WC4 = typeof (StrategicWindowClass2);
                 System.Type local54 =  WC4;
                self.PaintPresentWindow(g28,  local54);
                Graphics g29 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local55 =  WC4;
                self.PaintPresentWindow(g29,  local55);
                Graphics g30 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local56 =  WC4;
                self.PaintPresentWindow(g30,  local56);
                Graphics g31 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local57 =  WC4;
                self.PaintPresentWindow(g31,  local57);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "The hexes you can strategically transfer the unit to are highlighted.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please select the highlighted hex and press the big 'transfer' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  self.Game.SelectX, false), Operators.CompareObjectEqual(self.Game.EditObj.TutY,  self.Game.SelectY, false))))
              {
                let mut num39: i32 = num15 + 735;
                let mut num40: i32 = self.Game.ScreenHeight - 200;
                 let mut local58: &Graphics = &g;
                bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
                 let mut local59: &Bitmap = &bitmap1;
                let mut x: i32 = num39;
                let mut y: i32 = num40;
                DrawMod.DrawSimple( local58,  local59, x, y);
              }
              num34 = 1;
            }
            if (self.Game.EditObj.TutStep == 18 & self.Game.EditObj.UnitSelected > -1 & self.Game.EditObj.OrderType == 0 && num34 == 0 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 283)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g32 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local60 =  WC4;
                self.PaintPresentWindow(g32,  local60);
                Graphics g33 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local61 =  WC4;
                self.PaintPresentWindow(g33,  local61);
                Graphics g34 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local62 =  WC4;
                self.PaintPresentWindow(g34,  local62);
              }
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Yes thats the engineer unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now to strategically transfer this unit you press the strategic transfer button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.EditObj.TutOrder != 18)
              {
                self.Game.EditObj.TutOrder = 18;
                Graphics g35 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g35,  local);
              }
              let mut num41: i32 = num15 + 165;
              let mut num42: i32 = self.Game.ScreenHeight - 360;
               let mut local63: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local64: &Bitmap = &bitmap1;
              let mut x: i32 = num41;
              let mut y: i32 = num42;
              DrawMod.DrawSimple( local63,  local64, x, y);
              num34 = 1;
            }
            if (self.Game.EditObj.TutStep == 18 & num34 == 0)
            {
              self.Game.EditObj.TutOrder = 9999;
              DrawMod.DrawTutback(g, 5, 5, 720, 155,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "So thats how attacks work. Artillery and Air attack work more or less the same.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "You now see a black oval shape with a number on top of the hex you just attacked.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "This is remembered 'battle stack points' and they will be added to the 'stack' total of your next attack.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "(basically this rule will make it impossible to keep attacking a specific hex over and over)", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 80, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now lets see how to do a strategic transfer.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 105, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please select your highlighted Engineer Unit!.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 130, Color.White);
              let mut unitCounter: i32 = self.Game.Data.UnitCounter;
              for (let mut index1: i32 = 0; index1 <= unitCounter; index1 += 1)
              {
                if (Operators.CompareString(self.Game.Data.UnitObj[index1].Name, "Assault Brigade", false) == 0)
                {
                  let mut sfCount: i32 = self.Game.Data.UnitObj[index1].SFCount;
                  for (let mut index2: i32 = 0; index2 <= sfCount; index2 += 1)
                    self.Game.Data.SFObj[self.Game.Data.UnitObj[index1].SFList[index2]].Ap = 100;
                }
              }
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  12, false))))
              {
                self.Game.EditObj.TutX =  12;
                self.Game.EditObj.TutY =  7;
                Graphics g36 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local65 =  WC4;
                self.PaintPresentWindow(g36,  local65);
                Graphics g37 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local66 =  WC4;
                self.PaintPresentWindow(g37,  local66);
                Graphics g38 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local67 =  WC4;
                self.PaintPresentWindow(g38,  local67);
              }
              num34 = 1;
            }
            if (self.Game.EditObj.TutStep == 13 && self.Game.EditObj.OrderType == 2 & self.Game.EditObj.TempUnitList.counter > -1)
            {
              DrawMod.DrawTutback(g, 5, 5, 800, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "After you have selected one or more units to join the attack you can actually begin the attack. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Lets do so. Press the attack button! ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              num34 = 1;
              let mut num43: i32 = num15 + 842;
              let mut num44: i32 = self.Game.ScreenHeight - 360;
               let mut local68: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local69: &Bitmap = &bitmap1;
              let mut x: i32 = num43;
              let mut y: i32 = num44;
              DrawMod.DrawSimple( local68,  local69, x, y);
            }
            if (self.Game.EditObj.TutStep == 13 & num34 == 0 && self.Game.EditObj.OrderType == 2 & self.Game.EditObj.UnitSelected > -1 && self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 277)
            {
              DrawMod.DrawTutback(g, 5, 5, 800, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "To let the Grenzschutz unit participate in the attack we", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "click the indicated button. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num34 = 1;
              let mut num45: i32 = num15 + 722;
              let mut num46: i32 = self.Game.ScreenHeight - 360;
               let mut local70: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local71: &Bitmap = &bitmap1;
              let mut x: i32 = num45;
              let mut y: i32 = num46;
              DrawMod.DrawSimple( local70,  local71, x, y);
            }
            if (self.Game.EditObj.TutStep == 13 & num34 == 0 && self.Game.EditObj.OrderType == 2)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g39 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local72 =  WC4;
                self.PaintPresentWindow(g39,  local72);
                Graphics g40 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local73 =  WC4;
                self.PaintPresentWindow(g40,  local73);
                Graphics g41 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local74 =  WC4;
                self.PaintPresentWindow(g41,  local74);
              }
              self.Game.EditObj.TutOrder = -1;
              DrawMod.DrawTutback(g, 5, 5, 800, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Attack planning has started. You now have to select friendly and adjacent units to participate in the attack.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please click on our Grenzschutz unit. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              num34 = 1;
            }
            if (self.Game.EditObj.TutStep == 13 & self.Game.SelectX == 15 & self.Game.SelectY == 4 & num34 == 0)
            {
              if (self.Game.EditObj.TutOrder != 2)
              {
                self.Game.EditObj.TutOrder = 2;
                Graphics g42 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local75 =  WC4;
                self.PaintPresentWindow(g42,  local75);
                Graphics g43 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local76 =  WC4;
                self.PaintPresentWindow(g43,  local76);
                Graphics g44 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local77 =  WC4;
                self.PaintPresentWindow(g44,  local77);
              }
              DrawMod.DrawTutback(g, 5, 5, 800, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You have selected the enemy hex/unit. You always need to do this before you can order an attack on it.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please now click on the attack button to start planning an attack on the hex. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              let mut num47: i32 = num15 + 70;
              let mut num48: i32 = self.Game.ScreenHeight - 375;
               let mut local78: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local79: &Bitmap = &bitmap1;
              let mut x: i32 = num47;
              let mut y: i32 = num48;
              DrawMod.DrawSimple( local78,  local79, x, y);
              num34 = 1;
            }
            if ((self.Game.EditObj.TutStep == 11 | self.Game.EditObj.TutStep == 13) & num34 == 0 && self.Game.Data.UnitObj[self.Game.HandyFunctionsObj.GetUnitByHistorical(178)].DidMove)
            {
              self.Game.EditObj.TutStep = 13;
              if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  12, false), Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  15;
                self.Game.EditObj.TutY =  4;
                Graphics g45 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local80 =  WC4;
                self.PaintPresentWindow(g45,  local80);
                Graphics g46 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local81 =  WC4;
                self.PaintPresentWindow(g46,  local81);
              }
              if (self.Game.EditObj.TutOrder != 9999)
              {
                self.Game.EditObj.TutOrder = 9999;
                Graphics g47 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g47,  local);
              }
              DrawMod.DrawTutback(g, 5, 5, 900, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "So thats how you group move a unit. It definitely has its uses in scenarios with a high unit count! ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now I will show how to attack the enemy. Please select the highlighted enemy Engineer unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
            }
            let mut num49: i32 = 0;
            if (self.Game.EditObj.TutStep == 11 & self.Game.EditObj.UnitSelected > -1 && self.Game.EditObj.OrderType == 48 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 178)
            {
              DrawMod.DrawTutback(g, 5, 5, 900, 100,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You now see all the hexes highlighted where the units can move to. Only the hexes where all 73rd division", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "units can move too are highlighted.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Units from different hexes will thus move over different paths to the same target hex.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now please move the selected units (of 73rd div) to the selected target hex!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 80, Color.White);
              self.Game.EditObj.TutOrder = -1;
              if (Operators.ConditionalCompareObjectEqual(self.Game.EditObj.TutX,  -1, false))
              {
                self.Game.EditObj.TutX =  12;
                self.Game.EditObj.TutY =  6;
                Graphics g48 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local82 =  WC4;
                self.PaintPresentWindow(g48,  local82);
                Graphics g49 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local83 =  WC4;
                self.PaintPresentWindow(g49,  local83);
              }
              num49 = 1;
            }
            if (self.Game.EditObj.TutStep == 11 & self.Game.EditObj.UnitSelected > -1 & self.Game.EditObj.OrderType == 0 && num49 == 0 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 178)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g50 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local84 =  WC4;
                self.PaintPresentWindow(g50,  local84);
                Graphics g51 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local85 =  WC4;
                self.PaintPresentWindow(g51,  local85);
              }
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Now click the highlighted Group Move button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.EditObj.TutOrder != 48)
              {
                self.Game.EditObj.TutOrder = 48;
                Graphics g52 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g52,  local);
              }
              let mut num50: i32 = num15 + 106;
              let mut num51: i32 = self.Game.ScreenHeight - 360;
               let mut local86: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local87: &Bitmap = &bitmap1;
              let mut x: i32 = num50;
              let mut y: i32 = num51;
              DrawMod.DrawSimple( local86,  local87, x, y);
              num49 = 1;
            }
            if ((self.Game.EditObj.TutStep == 10 | self.Game.EditObj.TutStep == 11) & num49 == 0 && self.Game.Data.UnitObj[self.Game.HandyFunctionsObj.GetUnitByHistorical(277)].DidMove)
            {
              self.Game.EditObj.TutStep = 11;
              if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  14, false), Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  13;
                self.Game.EditObj.TutY =  7;
                Graphics g53 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local88 =  WC4;
                self.PaintPresentWindow(g53,  local88);
                Graphics g54 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local89 =  WC4;
                self.PaintPresentWindow(g54,  local89);
              }
              if (self.Game.EditObj.TutOrder != 9999)
              {
                self.Game.EditObj.TutOrder = 9999;
                Graphics g55 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g55,  local);
              }
              DrawMod.DrawTutback(g, 5, 5, 900, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "So thats how you move a unit! Its very simple. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "However you can also move a whole division (4 units) with one order. Select one of the units of the 73th div now.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
            }
            let mut num52: i32 = 0;
            if (self.Game.EditObj.TutStep == 10 & self.Game.EditObj.UnitSelected > -1 && self.Game.EditObj.OrderType == 1 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 277)
            {
              DrawMod.DrawTutback(g, 5, 5, 850, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You now see all the hexes highlighted where the unit", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "can move too. Click on the highlighted hex to move the unit there.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              self.Game.EditObj.TutOrder = -1;
              if (Operators.ConditionalCompareObjectEqual(self.Game.EditObj.TutX,  -1, false))
              {
                self.Game.EditObj.TutX =  14;
                self.Game.EditObj.TutY =  4;
                Graphics g56 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local90 =  WC4;
                self.PaintPresentWindow(g56,  local90);
                Graphics g57 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local91 =  WC4;
                self.PaintPresentWindow(g57,  local91);
                Graphics g58 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local92 =  WC4;
                self.PaintPresentWindow(g58,  local92);
              }
              num52 = 1;
            }
            if (self.Game.EditObj.TutStep == 10 & self.Game.EditObj.UnitSelected > -1 & self.Game.EditObj.OrderType == 0 && num52 == 0 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 277)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g59 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local93 =  WC4;
                self.PaintPresentWindow(g59,  local93);
                Graphics g60 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local94 =  WC4;
                self.PaintPresentWindow(g60,  local94);
              }
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Well done. You can now inspect the unit. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "To move it you have to click the highlighted 'move unit' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.EditObj.TutOrder != 1)
              {
                self.Game.EditObj.TutOrder = 1;
                Graphics g61 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g61,  local);
              }
              let mut num53: i32 = num15 + 70;
              let mut num54: i32 = self.Game.ScreenHeight - 360;
               let mut local95: &Graphics = &g;
              bitmap1 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local96: &Bitmap = &bitmap1;
              let mut x: i32 = num53;
              let mut y: i32 = num54;
              DrawMod.DrawSimple( local95,  local96, x, y);
              num52 = 1;
            }
            if (!(self.Game.EditObj.TutStep > 1 & self.Game.EditObj.TutStep <= 10 & num52 == 0))
              return;
            self.Game.EditObj.TutStep = 10;
            if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  13, false))))
            {
              self.Game.EditObj.TutX =  13;
              self.Game.EditObj.TutY =  4;
              Graphics g62 = g;
              WC4 = typeof (MapWindowClass2);
               System.Type local97 =  WC4;
              self.PaintPresentWindow(g62,  local97);
              Graphics g63 = g;
              WC4 = typeof (ResourceWindowClass2);
               System.Type local98 =  WC4;
              self.PaintPresentWindow(g63,  local98);
            }
            if (self.Game.EditObj.TutOrder != 9999)
            {
              self.Game.EditObj.TutOrder = 9999;
              Graphics g64 = g;
              WC4 = typeof (OrderWindowClass2);
               System.Type local =  WC4;
              self.PaintPresentWindow(g64,  local);
            }
            DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "This is the mainscreen. I'll start with showing how to move a unit. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
            DrawMod.DrawTextColouredOutline( g, "Please select the 'Grenzschutze Regiment'. Its highlighted.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
          }
          else
          {
            WC4 = typeof (GameLoopMainWindowClass);
            if (!self.WindowPresent( WC4))
              return;
            if (self.Game.EditObj.TutStep <= 1)
            {
              let mut unitCounter: i32 = self.Game.Data.UnitCounter;
              for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
              {
                if (Operators.CompareString(self.Game.Data.UnitObj[index3].Name, "Assault Brigade", false) == 0)
                {
                  let mut sfCount: i32 = self.Game.Data.UnitObj[index3].SFCount;
                  for (let mut index4: i32 = 0; index4 <= sfCount; index4 += 1)
                    self.Game.Data.SFObj[self.Game.Data.UnitObj[index3].SFList[index4]].Ap = 0;
                }
              }
              DrawMod.DrawTutback(g, 5, 5, 800, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "As first round starts some calculations are done...", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 10, Color.White);
              if (self.Game.EditObj.TutStep != 1)
                return;
              DrawMod.DrawTextColouredOutline( g, "Once the calculations have completed, you can begin playing.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 35, Color.White);
              let mut num55: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
              let mut num56: i32 =  Math.Round( (self.Game.ScreenHeight - 768) / 2.0);
              let mut num57: i32 = num55 + 485;
              let mut num58: i32 = num56 + 630;
               let mut local99: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local100: &Bitmap = &bitmap;
              let mut x: i32 = num57;
              let mut y: i32 = num58;
              DrawMod.DrawSimple( local99,  local100, x, y);
            }
            else if (self.Game.EditObj.TutStep == 2)
            {
              DrawMod.DrawTutback(g, 5, 5, 800, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You have now started your turn. You get synopsis of what", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "happened in the turns of your opponent and any other news.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              let mut num59: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
              let mut num60: i32 =  Math.Round( (self.Game.ScreenHeight - 768) / 2.0);
              let mut num61: i32 = num59 + 485;
              let mut num62: i32 = num60 + 630;
               let mut local101: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local102: &Bitmap = &bitmap;
              let mut x: i32 = num61;
              let mut y: i32 = num62;
              DrawMod.DrawSimple( local101,  local102, x, y);
            }
            else
            {
              if (self.Game.EditObj.TutStep != 3)
                return;
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "When news or messages popup you can just press a key.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "to continue or click the button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              self.Game.EditObj.TutOrder = 9999;
            }
          }
        }
      }
    }

    pub fn DoTutorial3(Graphics g)
    {
      color: Color = Color.FromArgb( byte.MaxValue,  byte.MaxValue, 28, 0);
      self.Game.EditObj.Zoom = 0;
      self.Game.EditObj.HideDetail = false;
      self.Game.EditObj.HideAS = false;
      System.Type WC1 = typeof (IntroWindowClass2);
      if (self.WindowPresent( WC1))
      {
        self.Game.EditObj.TutStep = 0;
        DrawMod.DrawTutback(g, 5, 5, 960, 160,  color.R,  color.G,  color.B,  color.A);
        DrawMod.DrawTextColouredOutline( g, "Hi! Let me introduce myself. I am Vic, the designer of the game. Welcome to the tutorial. It will go over some ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
        DrawMod.DrawTextColouredOutline( g, "key concepts and orders. The tutorial is not exhaustive and it is advised that you read the manual too.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
        DrawMod.DrawTextColouredOutline( g, "One of the most important things for new players to know is that you can right click on everything where the mouse", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 70, Color.White);
        DrawMod.DrawTextColouredOutline( g, "shows a question mark or hand.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 95, Color.White);
        DrawMod.DrawTextColouredOutline( g, "Now please press 'start' to start the tutorial.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 135, Color.White);
        let mut num1: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
        let mut num2: i32 =  Math.Round( (self.Game.ScreenHeight - 768) / 2.0);
        let mut num3: i32 = num1 + 845;
        let mut num4: i32 = num2 + 650;
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = num3;
        let mut y: i32 = num4;
        DrawMod.DrawSimple( local1,  local2, x, y);
      }
      else
      {
        System.Type WC2 = typeof (CombatResultWindowClass2);
        num5: i32;
        if (self.WindowPresent( WC2))
        {
          let mut num6: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
          num5 = 0;
          if (self.Game.EditObj.TutStep == 13 | self.Game.EditObj.TutStep == 18)
          {
            self.Game.EditObj.TutStep = 18;
            self.Game.EditObj.TutX =  12;
            self.Game.EditObj.TutY =  7;
            self.Game.EditObj.TutOrder = 9999;
            DrawMod.DrawTutback(g, 5, 5, 800, 200,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "The battle is being fought combat round by combat round.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
            if ((uint) self.Game.TempCombat.BattleEnded > 0U)
            {
              DrawMod.DrawTextColouredOutline( g, "And once ended you can inspect the battle details or return back to the main screen. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Lets go back to main screen. Click OK.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Note: What your basically seeing in the battle screen is each side's participating units.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 100, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Each unit's troops are displayed. Troops in the middle columns are still participating ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 120, Color.White);
              DrawMod.DrawTextColouredOutline( g, "in combat. And troops in the side columns have been killed or have retreated from combat.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 140, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Battle ends when one of the sides has no troops participating anymore. In a nutshell ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 160, Color.White);
              DrawMod.DrawTextColouredOutline( g, "that is what happens. If you are not easily shaken then click on DETAILS to see whats really going on.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 180, Color.White);
            }
            let mut num7: i32 =  Math.Round( self.Game.ScreenWidth / 2.0);
            let mut num8: i32 =  Math.Round( self.Game.ScreenHeight / 2.0);
            let mut x1_1: i32 = num7 - 200;
            let mut y1: i32 = num8 - 150;
            DrawMod.DrawBlock( g, x1_1, y1, 5, 300,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawBlock( g, x1_1 + 90, y1 + 20, 220, 25,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "PARTICIPATING TROOPS", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), x1_1 + 95, y1 + 20, Color.White);
            let mut num9: i32 = x1_1 - 350;
            DrawMod.DrawBlock( g, num9 + 110, y1 + 20, 220, 25,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "RETREATED TROOPS", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), num9 + 120, y1 + 20, Color.White);
            let mut x1_2: i32 =  Math.Round( self.Game.ScreenWidth / 2.0) + 190;
            DrawMod.DrawBlock( g, x1_2, y1, 5, 300,  color.R,  color.G,  color.B,  color.A);
            let mut num10: i32 = x1_2 - 80;
            DrawMod.DrawBlock( g, num10 + 110, y1 + 20, 220, 25,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "RETREATED TROOPS", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), num10 + 120, y1 + 20, Color.White);
            let mut num11: i32 =  Math.Round( self.Game.ScreenWidth / 2.0);
            let mut num12: i32 =  Math.Round( self.Game.ScreenHeight / 2.0);
            num5 = 1;
          }
          if (!(self.Game.EditObj.TutStep == 27 | self.Game.EditObj.TutStep == 30))
            return;
          self.Game.EditObj.TutStep = 30;
          self.Game.EditObj.TutX =  -1;
          self.Game.EditObj.TutY =  -1;
          self.Game.Data.MapObj[0].HexObj[self.Game.EditObj.TargetX, self.Game.EditObj.TargetY].set_BattlePenalty(0, 12);
          DrawMod.DrawTutback(g, 5, 5, 900, 90,  color.R,  color.G,  color.B,  color.A);
          DrawMod.DrawTextColouredOutline( g, "And another battle commences.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
          if ((uint) self.Game.TempCombat.BattleEnded > 0U)
            DrawMod.DrawTextColouredOutline( g, "And of course won by your stronger forces. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
          num5 = 1;
        }
        else
        {
          System.Type WC3 = typeof (PlayExtraWindowClass2);
          let mut num13: i32 = self.WindowPresent( WC3) ? 1 : 0;
          System.Type WC4 = typeof (StrategicWindowClass2);
          let mut num14: i32 = self.WindowPresent( WC4) ? 1 : 0;
          if ((num13 | num14) != 0)
          {
            let mut num15: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
            num5 = 0;
            if (self.Game.EditObj.TutStep == 30)
            {
              DrawMod.DrawTutback(g, 5, 5, 850, 210,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You now see a black box with a number on the hex you just conquered. This is so called remaining 'battle AP penalty' ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "and it will cause a movement penalty on units that did not participate in the combat for taking the hex. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "(This rule makes it possible for the defender to delay the whole enemy army with one properly defended roadblock)", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "And that concludes this short tutorial. It handled some key concepts, but I advise you to read the manual now.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 80, Color.White);
              DrawMod.DrawTextColouredOutline( g, "In a normal game you would now press", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 105, Color.White);
               let mut local3: &Graphics = &g;
              bitmap1: Bitmap = BitmapStore.GetBitmap(self.Game.MARCBACK4);
               let mut local4: &Bitmap = &bitmap1;
              DrawMod.DrawSimple( local3,  local4, 95, 138);
               let mut local5: &Graphics = &g;
              bitmap2: Bitmap = BitmapStore.GetBitmap(self.Game.BUTTONNEXT);
               let mut local6: &Bitmap = &bitmap2;
              DrawMod.DrawSimple( local5,  local6, 95, 138);
              DrawMod.DrawTextColouredOutline( g, "the next turn button, but the tutorial has no next turn. You have to use the 'quit' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 130, 155, Color.White);
              DrawMod.DrawTextColouredOutline( g, "It's in the top-right corner. I will be available on the forums for any questions. Thanks for your attention and happy gaming!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 180, Color.White);
              let mut num16: i32 = self.Game.ScreenWidth - 20;
              let mut y1: i32 = 35;
              DrawMod.drawLine( g, num16, y1, num16, y1 + 40,  color.R,  color.G,  color.B,  color.A, 4);
              DrawMod.drawLine( g, num16, y1, num16 - 10, y1 + 10,  color.R,  color.G,  color.B,  color.A, 4);
              DrawMod.drawLine( g, num16, y1, num16 + 10, y1 + 10,  color.R,  color.G,  color.B,  color.A, 4);
              num5 = 1;
            }
            let mut num17: i32 = 0;
            if (self.Game.EditObj.TutStep == 27 & num17 == 0 && self.Game.EditObj.OrderType == 2 & self.Game.EditObj.TempUnitList.counter > -1)
            {
              let mut num18: i32 = num15 + 825;
              let mut num19: i32 = self.Game.ScreenHeight - 360;
               let mut local7: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local8: &Bitmap = &bitmap;
              let mut x: i32 = num18;
              let mut y: i32 = num19;
              DrawMod.DrawSimple( local7,  local8, x, y);
              num5 = 1;
              DrawMod.DrawTutback(g, 5, 5, 960, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Alright and now press attack!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 27 & num17 == 0 && self.Game.EditObj.OrderType == 2)
            {
              self.Game.EditObj.TutStep = 27;
              let mut num20: i32 = num15 + 956;
              let mut num21: i32 = self.Game.ScreenHeight - 360;
               let mut local9: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local10: &Bitmap = &bitmap;
              let mut x: i32 = num20;
              let mut y: i32 = num21;
              DrawMod.DrawSimple( local9,  local10, x, y);
              num5 = 1;
              DrawMod.DrawTutback(g, 5, 5, 960, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "To select all available units to join in the attack press the 'ALL' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 27 & num17 == 0 && self.Game.SelectX == 15 & self.Game.SelectY == 4)
            {
              if (self.Game.EditObj.TutOrder != 2)
              {
                self.Game.EditObj.TutOrder = 2;
                Graphics g1 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local11 =  WC4;
                self.PaintPresentWindow(g1,  local11);
                Graphics g2 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local12 =  WC4;
                self.PaintPresentWindow(g2,  local12);
                Graphics g3 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local13 =  WC4;
                self.PaintPresentWindow(g3,  local13);
              }
              let mut num22: i32 = num15 + 75;
              let mut num23: i32 = self.Game.ScreenHeight - 360;
               let mut local14: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local15: &Bitmap = &bitmap;
              let mut x: i32 = num22;
              let mut y: i32 = num23;
              DrawMod.DrawSimple( local14,  local15, x, y);
              num5 = 1;
              DrawMod.DrawTutback(g, 5, 5, 960, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Alright. Now click the attack button so you can start to select the participants in the attack on this hex.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num17 = 1;
            }
            if ((self.Game.EditObj.TutStep == 24 | self.Game.EditObj.TutStep == 27) & num17 == 0 && Operators.CompareString(self.Game.Data.UnitObj[self.Game.Data.UnitObj[self.Game.HandyFunctionsObj.GetUnitByHistorical(540)].HQ].Name, "OKH", false) == 0)
            {
              self.Game.EditObj.TutStep = 27;
              if (self.Game.EditObj.TutOrder != 9999)
              {
                self.Game.EditObj.TutOrder = 9999;
                self.Game.EditObj.TutX =  15;
                self.Game.EditObj.TutY =  4;
                Graphics g4 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local16 =  WC4;
                self.PaintPresentWindow(g4,  local16);
                Graphics g5 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local17 =  WC4;
                self.PaintPresentWindow(g5,  local17);
                Graphics g6 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local18 =  WC4;
                self.PaintPresentWindow(g6,  local18);
              }
              if (self.Game.Data.MapObj[0].HexObj[15, 4].get_BattlePenalty(0) < 1)
                self.Game.Data.MapObj[0].HexObj[12, 1].set_BattlePenalty(0, 7);
              DrawMod.DrawTutback(g, 5, 5, 960, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Very well. You can see the HQ change reflected in the colored bar of the unit. It's now brown just as the OKH. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now I will show the concept of battle AP penalties. For this you have to start a battle first. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please click on the selected enemy unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              num17 = 1;
            }
            if (self.Game.EditObj.TutStep == 24 & num17 == 0 && self.Game.EditObj.OrderType == 3)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                let mut unitCounter: i32 = self.Game.Data.UnitCounter;
                for (let mut index1: i32 = 0; index1 <= unitCounter; index1 += 1)
                {
                  if (Operators.CompareString(self.Game.Data.UnitObj[index1].Name, "1st SS Brigade", false) == 0)
                  {
                    let mut sfCount: i32 = self.Game.Data.UnitObj[index1].SFCount;
                    for (let mut index2: i32 = 0; index2 <= sfCount; index2 += 1)
                      self.Game.Data.SFObj[self.Game.Data.UnitObj[index1].SFList[index2]].Ap = 100;
                  }
                  if (Strings.InStr(self.Game.Data.UnitObj[index1].Name, "59th Panzer") > 0)
                  {
                    let mut sfCount: i32 = self.Game.Data.UnitObj[index1].SFCount;
                    for (let mut index3: i32 = 0; index3 <= sfCount; index3 += 1)
                      self.Game.Data.SFObj[self.Game.Data.UnitObj[index1].SFList[index3]].Ap = 100;
                  }
                }
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g7 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local19 =  WC4;
                self.PaintPresentWindow(g7,  local19);
                Graphics g8 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local20 =  WC4;
                self.PaintPresentWindow(g8,  local20);
                Graphics g9 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local21 =  WC4;
                self.PaintPresentWindow(g9,  local21);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "The game wants to know what unit should be the new HQ for the motorized regiment.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please click on the OKH and then on the confirm button to make that the HQ.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.SelectX == 8 & self.Game.SelectY == 7)
              {
                let mut num24: i32 = num15 + 723;
                let mut num25: i32 = self.Game.ScreenHeight - 360;
                 let mut local22: &Graphics = &g;
                bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
                 let mut local23: &Bitmap = &bitmap;
                let mut x: i32 = num24;
                let mut y: i32 = num25;
                DrawMod.DrawSimple( local22,  local23, x, y);
              }
              num17 = 1;
            }
            bitmap3: Bitmap;
            if (self.Game.EditObj.TutStep == 24 & num17 == 0 && self.Game.SelectX == 10 & self.Game.SelectY == 4 && !self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 24;
              if (self.Game.EditObj.TutOrder != 3)
              {
                self.Game.EditObj.TutOrder = 3;
                Graphics g10 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local24 =  WC4;
                self.PaintPresentWindow(g10,  local24);
                Graphics g11 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local25 =  WC4;
                self.PaintPresentWindow(g11,  local25);
                Graphics g12 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local26 =  WC4;
                self.PaintPresentWindow(g12,  local26);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You have now selected the Motorized Unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Click the highlighted 'HQ' order.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              let mut num26: i32 = num15 + 143;
              let mut num27: i32 = self.Game.ScreenHeight - 360;
               let mut local27: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local28: &Bitmap = &bitmap3;
              let mut x: i32 = num26;
              let mut y: i32 = num27;
              DrawMod.DrawSimple( local27,  local28, x, y);
              num17 = 1;
            }
            if ((self.Game.EditObj.TutStep == 20 | self.Game.EditObj.TutStep == 24) & num17 == 0 && !(self.Game.SelectX == 10 & self.Game.SelectY == 4) && !self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 24;
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutY,  4, false))))
              {
                self.Game.EditObj.TutX =  10;
                self.Game.EditObj.TutY =  4;
                self.Game.SelectX = 8;
                self.Game.SelectY = 6;
                Graphics g13 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local29 =  WC4;
                self.PaintPresentWindow(g13,  local29);
                Graphics g14 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local30 =  WC4;
                self.PaintPresentWindow(g14,  local30);
                Graphics g15 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local31 =  WC4;
                self.PaintPresentWindow(g15,  local31);
                Graphics g16 = g;
                WC4 = typeof (PlayExtraWindowClass2);
                 System.Type local32 =  WC4;
                self.PaintPresentWindow(g16,  local32);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Now I'll show how to change a units HQ. Please now select the Motorized Regiment.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              num5 = 1;
            }
            let mut num28: i32 = 0;
            if ((self.Game.EditObj.TutStep == 19 | self.Game.EditObj.TutStep == 20) & num28 == 0 && self.Game.EditObj.LayerSupplyOn)
            {
              self.Game.EditObj.TutStep = 20;
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  10, false))))
              {
                self.Game.EditObj.TutX =  12;
                self.Game.EditObj.TutY =  0;
                Graphics g17 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local33 =  WC4;
                self.PaintPresentWindow(g17,  local33);
                Graphics g18 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local34 =  WC4;
                self.PaintPresentWindow(g18,  local34);
                Graphics g19 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local35 =  WC4;
                self.PaintPresentWindow(g19,  local35);
              }
              DrawMod.DrawTutback(g, 5, 5, 800, 150,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "That's it. You now see how supply flows from your HQ to your units. Click on a hex of choice to see exact path.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Click on back button to hide the supply layer again.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "The colors indicate if an area is in good supply. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 75, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Note that our Flak unit is in bad supply due to the river and broken bridges.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 100, Color.White);
              let mut num29: i32 = num15 + 38;
              let mut num30: i32 = self.Game.ScreenHeight - 360;
               let mut local36: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local37: &Bitmap = &bitmap3;
              let mut x: i32 = num29;
              let mut y: i32 = num30;
              DrawMod.DrawSimple( local36,  local37, x, y);
              num5 = 1;
              num28 = 1;
            }
            if ((self.Game.EditObj.TutStep == 18 | self.Game.EditObj.TutStep == 19) & num28 == 0 && self.Game.Data.MapObj[0].HexObj[6, 8].UnitCounter > -1)
            {
              self.Game.EditObj.TutStep = 19;
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  8, false))))
              {
                self.Game.EditObj.TutX =  8;
                self.Game.EditObj.TutY =  7;
                Graphics g20 = g;
                WC4 = typeof (StrategicWindowClass2);
                 System.Type local38 =  WC4;
                self.PaintPresentWindow(g20,  local38);
                Graphics g21 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local39 =  WC4;
                self.PaintPresentWindow(g21,  local39);
                Graphics g22 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local40 =  WC4;
                self.PaintPresentWindow(g22,  local40);
                Graphics g23 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local41 =  WC4;
                self.PaintPresentWindow(g23,  local41);
              }
              DrawMod.DrawTutback(g, 5, 5, 960, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "That's how you strategically transfer units. Now I will show how the supply layer can be activated.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "For this you need to select a HQ and then press the supply layer button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please select the OKH unit and activate the supply layer!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              if (self.Game.SelectX == 8 & self.Game.SelectY == 7)
              {
                self.Game.EditObj.TutOrder = 51;
                let mut num31: i32 = num15 + 770;
                let mut num32: i32 = self.Game.ScreenHeight - 360;
                 let mut local42: &Graphics = &g;
                bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
                 let mut local43: &Bitmap = &bitmap3;
                let mut x: i32 = num31;
                let mut y: i32 = num32;
                DrawMod.DrawSimple( local42,  local43, x, y);
              }
              else
                self.Game.EditObj.TutOrder = 9999;
              num28 = 1;
            }
            if (self.Game.EditObj.TutStep == 18 && self.Game.EditObj.OrderType == 18)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  6, false))))
              {
                self.Game.EditObj.TutX =  6;
                self.Game.EditObj.TutY =  8;
                Graphics g24 = g;
                WC4 = typeof (StrategicWindowClass2);
                 System.Type local44 =  WC4;
                self.PaintPresentWindow(g24,  local44);
                Graphics g25 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local45 =  WC4;
                self.PaintPresentWindow(g25,  local45);
                Graphics g26 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local46 =  WC4;
                self.PaintPresentWindow(g26,  local46);
                Graphics g27 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local47 =  WC4;
                self.PaintPresentWindow(g27,  local47);
              }
              DrawMod.DrawTutback(g, 5, 5, 860, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "The hexes you can strategically transfer the unit to are highlighted.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please select the highlighted hex and press the big 'transfer' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  self.Game.SelectX, false), Operators.CompareObjectEqual(self.Game.EditObj.TutY,  self.Game.SelectY, false))))
              {
                let mut num33: i32 = num15 + 735;
                let mut num34: i32 = self.Game.ScreenHeight - 200;
                 let mut local48: &Graphics = &g;
                bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
                 let mut local49: &Bitmap = &bitmap3;
                let mut x: i32 = num33;
                let mut y: i32 = num34;
                DrawMod.DrawSimple( local48,  local49, x, y);
              }
              num28 = 1;
            }
            if (self.Game.EditObj.TutStep == 18 & self.Game.EditObj.UnitSelected > -1 & self.Game.EditObj.OrderType == 0 && num28 == 0 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 581)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g28 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local50 =  WC4;
                self.PaintPresentWindow(g28,  local50);
                Graphics g29 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local51 =  WC4;
                self.PaintPresentWindow(g29,  local51);
                Graphics g30 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local52 =  WC4;
                self.PaintPresentWindow(g30,  local52);
              }
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Yes thats the engineer unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now to strategically transfer this unit you press the strategic transfer button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.EditObj.TutOrder != 18)
              {
                self.Game.EditObj.TutOrder = 18;
                Graphics g31 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g31,  local);
              }
              let mut num35: i32 = num15 + 165;
              let mut num36: i32 = self.Game.ScreenHeight - 360;
               let mut local53: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local54: &Bitmap = &bitmap3;
              let mut x: i32 = num35;
              let mut y: i32 = num36;
              DrawMod.DrawSimple( local53,  local54, x, y);
              num28 = 1;
            }
            if (self.Game.EditObj.TutStep == 18 & num28 == 0)
            {
              self.Game.EditObj.TutOrder = 9999;
              DrawMod.DrawTutback(g, 5, 5, 720, 155,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "So thats how attacks work. Artillery and Air attack work more or less the same.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "You now see a black oval shape with a number on top of the hex you just attacked.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "This is remembered 'battle stack points' and they will be added to the 'stack' total of your next attack.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "(basically this rule will make it impossible to keep attacking a specific hex over and over)", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 80, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now lets see how to do a strategic transfer.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 105, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please select your highlighted Engineer Unit!.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 130, Color.White);
              let mut unitCounter: i32 = self.Game.Data.UnitCounter;
              for (let mut index4: i32 = 0; index4 <= unitCounter; index4 += 1)
              {
                if (Operators.CompareString(self.Game.Data.UnitObj[index4].Name, "1st SS Brigade", false) == 0)
                {
                  let mut sfCount: i32 = self.Game.Data.UnitObj[index4].SFCount;
                  for (let mut index5: i32 = 0; index5 <= sfCount; index5 += 1)
                    self.Game.Data.SFObj[self.Game.Data.UnitObj[index4].SFList[index5]].Ap = 100;
                }
              }
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  12, false))))
              {
                self.Game.EditObj.TutX =  12;
                self.Game.EditObj.TutY =  7;
                Graphics g32 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local55 =  WC4;
                self.PaintPresentWindow(g32,  local55);
                Graphics g33 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local56 =  WC4;
                self.PaintPresentWindow(g33,  local56);
                Graphics g34 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local57 =  WC4;
                self.PaintPresentWindow(g34,  local57);
              }
              num28 = 1;
            }
            if (self.Game.EditObj.TutStep == 13 && self.Game.EditObj.OrderType == 2 & self.Game.EditObj.TempUnitList.counter > -1)
            {
              DrawMod.DrawTutback(g, 5, 5, 800, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "After you have selected one or more units to join the attack you can actually begin the attack. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Lets do so. Press the attack button! ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              num28 = 1;
              let mut num37: i32 = num15 + 842;
              let mut num38: i32 = self.Game.ScreenHeight - 360;
               let mut local58: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local59: &Bitmap = &bitmap3;
              let mut x: i32 = num37;
              let mut y: i32 = num38;
              DrawMod.DrawSimple( local58,  local59, x, y);
            }
            if (self.Game.EditObj.TutStep == 13 & num28 == 0 && self.Game.EditObj.OrderType == 2 & self.Game.EditObj.UnitSelected > -1 && self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 1258)
            {
              DrawMod.DrawTutback(g, 5, 5, 800, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "To let the Cossack unit participate in the attack we", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "click the indicated button. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              num28 = 1;
              let mut num39: i32 = num15 + 722;
              let mut num40: i32 = self.Game.ScreenHeight - 360;
               let mut local60: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local61: &Bitmap = &bitmap3;
              let mut x: i32 = num39;
              let mut y: i32 = num40;
              DrawMod.DrawSimple( local60,  local61, x, y);
            }
            if (self.Game.EditObj.TutStep == 13 & num28 == 0 && self.Game.EditObj.OrderType == 2)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g35 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local62 =  WC4;
                self.PaintPresentWindow(g35,  local62);
                Graphics g36 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local63 =  WC4;
                self.PaintPresentWindow(g36,  local63);
                Graphics g37 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local64 =  WC4;
                self.PaintPresentWindow(g37,  local64);
              }
              self.Game.EditObj.TutOrder = -1;
              DrawMod.DrawTutback(g, 5, 5, 800, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Attack planning has started. You now have to select friendly and adjacent units to participate in the attack.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please click on our Cossack unit. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              num28 = 1;
            }
            if (self.Game.EditObj.TutStep == 13 & self.Game.SelectX == 15 & self.Game.SelectY == 4 & num28 == 0)
            {
              if (self.Game.EditObj.TutOrder != 2)
              {
                self.Game.EditObj.TutOrder = 2;
                Graphics g38 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local65 =  WC4;
                self.PaintPresentWindow(g38,  local65);
                Graphics g39 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local66 =  WC4;
                self.PaintPresentWindow(g39,  local66);
                Graphics g40 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local67 =  WC4;
                self.PaintPresentWindow(g40,  local67);
              }
              DrawMod.DrawTutback(g, 5, 5, 800, 90,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You have selected the enemy hex/unit. You always need to do this before you can order an attack on it.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Please now click on the attack button to start planning an attack on the hex. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, " ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              let mut num41: i32 = num15 + 70;
              let mut num42: i32 = self.Game.ScreenHeight - 375;
               let mut local68: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local69: &Bitmap = &bitmap3;
              let mut x: i32 = num41;
              let mut y: i32 = num42;
              DrawMod.DrawSimple( local68,  local69, x, y);
              num28 = 1;
            }
            if ((self.Game.EditObj.TutStep == 11 | self.Game.EditObj.TutStep == 13) & num28 == 0 && self.Game.Data.UnitObj[self.Game.HandyFunctionsObj.GetUnitByHistorical(531)].DidMove)
            {
              self.Game.EditObj.TutStep = 13;
              if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  12, false), Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  15;
                self.Game.EditObj.TutY =  4;
                Graphics g41 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local70 =  WC4;
                self.PaintPresentWindow(g41,  local70);
                Graphics g42 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local71 =  WC4;
                self.PaintPresentWindow(g42,  local71);
              }
              if (self.Game.EditObj.TutOrder != 9999)
              {
                self.Game.EditObj.TutOrder = 9999;
                Graphics g43 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g43,  local);
              }
              DrawMod.DrawTutback(g, 5, 5, 900, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "So thats how you group move a unit. It definitely has its uses in scenarios with a high unit count! ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now I will show how to attack the enemy. Please select the highlighted enemy Engineer unit.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
            }
            let mut num43: i32 = 0;
            if (self.Game.EditObj.TutStep == 11 & self.Game.EditObj.UnitSelected > -1 && self.Game.EditObj.OrderType == 48 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 531)
            {
              DrawMod.DrawTutback(g, 5, 5, 900, 100,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You now see all the hexes highlighted where the units can move to. Only the hexes where all 73rd division", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "units can move too are highlighted.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Units from different hexes will thus move over different paths to the same target hex.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 55, Color.White);
              DrawMod.DrawTextColouredOutline( g, "Now please move the selected units (of 73rd div) to the selected target hex!", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 80, Color.White);
              self.Game.EditObj.TutOrder = -1;
              if (Operators.ConditionalCompareObjectEqual(self.Game.EditObj.TutX,  -1, false))
              {
                self.Game.EditObj.TutX =  12;
                self.Game.EditObj.TutY =  6;
                Graphics g44 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local72 =  WC4;
                self.PaintPresentWindow(g44,  local72);
                Graphics g45 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local73 =  WC4;
                self.PaintPresentWindow(g45,  local73);
              }
              num43 = 1;
            }
            if (self.Game.EditObj.TutStep == 11 & self.Game.EditObj.UnitSelected > -1 & self.Game.EditObj.OrderType == 0 && num43 == 0 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 531)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g46 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local74 =  WC4;
                self.PaintPresentWindow(g46,  local74);
                Graphics g47 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local75 =  WC4;
                self.PaintPresentWindow(g47,  local75);
              }
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Now click the highlighted Group Move button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.EditObj.TutOrder != 48)
              {
                self.Game.EditObj.TutOrder = 48;
                Graphics g48 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g48,  local);
              }
              let mut num44: i32 = num15 + 106;
              let mut num45: i32 = self.Game.ScreenHeight - 360;
               let mut local76: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local77: &Bitmap = &bitmap3;
              let mut x: i32 = num44;
              let mut y: i32 = num45;
              DrawMod.DrawSimple( local76,  local77, x, y);
              num43 = 1;
            }
            if ((self.Game.EditObj.TutStep == 10 | self.Game.EditObj.TutStep == 11) & num43 == 0 && self.Game.Data.UnitObj[self.Game.HandyFunctionsObj.GetUnitByHistorical(1258)].DidMove)
            {
              self.Game.EditObj.TutStep = 11;
              if (Conversions.ToBoolean(Operators.OrObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  14, false), Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  13;
                self.Game.EditObj.TutY =  7;
                Graphics g49 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local78 =  WC4;
                self.PaintPresentWindow(g49,  local78);
                Graphics g50 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local79 =  WC4;
                self.PaintPresentWindow(g50,  local79);
              }
              if (self.Game.EditObj.TutOrder != 9999)
              {
                self.Game.EditObj.TutOrder = 9999;
                Graphics g51 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g51,  local);
              }
              DrawMod.DrawTutback(g, 5, 5, 900, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "So thats how you move a unit! Its very simple. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "However you can also move a whole division (4 units) with one order. Select one of the units of the 73th div now.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
            }
            let mut num46: i32 = 0;
            if (self.Game.EditObj.TutStep == 10 & self.Game.EditObj.UnitSelected > -1 && self.Game.EditObj.OrderType == 1 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 1258)
            {
              DrawMod.DrawTutback(g, 5, 5, 850, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You now see all the hexes highlighted where the unit", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "can move too. Click on the highlighted hex to move the unit there.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              self.Game.EditObj.TutOrder = -1;
              if (Operators.ConditionalCompareObjectEqual(self.Game.EditObj.TutX,  -1, false))
              {
                self.Game.EditObj.TutX =  14;
                self.Game.EditObj.TutY =  4;
                Graphics g52 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local80 =  WC4;
                self.PaintPresentWindow(g52,  local80);
                Graphics g53 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local81 =  WC4;
                self.PaintPresentWindow(g53,  local81);
                Graphics g54 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local82 =  WC4;
                self.PaintPresentWindow(g54,  local82);
              }
              num46 = 1;
            }
            if (self.Game.EditObj.TutStep == 10 & self.Game.EditObj.UnitSelected > -1 & self.Game.EditObj.OrderType == 0 && num46 == 0 & self.Game.Data.UnitObj[self.Game.EditObj.UnitSelected].Historical == 1258)
            {
              if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  -1, false))))
              {
                self.Game.EditObj.TutX =  -1;
                self.Game.EditObj.TutY =  -1;
                Graphics g55 = g;
                WC4 = typeof (MapWindowClass2);
                 System.Type local83 =  WC4;
                self.PaintPresentWindow(g55,  local83);
                Graphics g56 = g;
                WC4 = typeof (ResourceWindowClass2);
                 System.Type local84 =  WC4;
                self.PaintPresentWindow(g56,  local84);
              }
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "Well done. You can now inspect the unit. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "To move it you have to click the highlighted 'move unit' button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              if (self.Game.EditObj.TutOrder != 1)
              {
                self.Game.EditObj.TutOrder = 1;
                Graphics g57 = g;
                WC4 = typeof (OrderWindowClass2);
                 System.Type local =  WC4;
                self.PaintPresentWindow(g57,  local);
              }
              let mut num47: i32 = num15 + 70;
              let mut num48: i32 = self.Game.ScreenHeight - 360;
               let mut local85: &Graphics = &g;
              bitmap3 = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local86: &Bitmap = &bitmap3;
              let mut x: i32 = num47;
              let mut y: i32 = num48;
              DrawMod.DrawSimple( local85,  local86, x, y);
              num46 = 1;
            }
            if (!(self.Game.EditObj.TutStep > 1 & self.Game.EditObj.TutStep <= 10 & num46 == 0))
              return;
            self.Game.EditObj.TutStep = 10;
            if (Conversions.ToBoolean(Operators.NotObject(Operators.CompareObjectEqual(self.Game.EditObj.TutX,  13, false))))
            {
              self.Game.EditObj.TutX =  13;
              self.Game.EditObj.TutY =  4;
              Graphics g58 = g;
              WC4 = typeof (MapWindowClass2);
               System.Type local87 =  WC4;
              self.PaintPresentWindow(g58,  local87);
              Graphics g59 = g;
              WC4 = typeof (ResourceWindowClass2);
               System.Type local88 =  WC4;
              self.PaintPresentWindow(g59,  local88);
            }
            if (self.Game.EditObj.TutOrder != 9999)
            {
              self.Game.EditObj.TutOrder = 9999;
              Graphics g60 = g;
              WC4 = typeof (OrderWindowClass2);
               System.Type local =  WC4;
              self.PaintPresentWindow(g60,  local);
            }
            DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
            DrawMod.DrawTextColouredOutline( g, "This is the mainscreen. I'll start with showing how to move a unit. ", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
            DrawMod.DrawTextColouredOutline( g, "Please select the 'Cossacks'. Its highlighted.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
          }
          else
          {
            WC4 = typeof (GameLoopMainWindowClass2);
            if (!self.WindowPresent( WC4))
              return;
            if (self.Game.EditObj.TutStep <= 1)
            {
              let mut unitCounter: i32 = self.Game.Data.UnitCounter;
              for (let mut index6: i32 = 0; index6 <= unitCounter; index6 += 1)
              {
                if (Operators.CompareString(self.Game.Data.UnitObj[index6].Name, "1st SS Brigade", false) == 0)
                {
                  let mut sfCount: i32 = self.Game.Data.UnitObj[index6].SFCount;
                  for (let mut index7: i32 = 0; index7 <= sfCount; index7 += 1)
                    self.Game.Data.SFObj[self.Game.Data.UnitObj[index6].SFList[index7]].Ap = 0;
                }
                if (Strings.InStr(self.Game.Data.UnitObj[index6].Name, "59th Panzer") > 0)
                {
                  let mut sfCount: i32 = self.Game.Data.UnitObj[index6].SFCount;
                  for (let mut index8: i32 = 0; index8 <= sfCount; index8 += 1)
                    self.Game.Data.SFObj[self.Game.Data.UnitObj[index6].SFList[index8]].Ap = 0;
                }
              }
              DrawMod.DrawTutback(g, 5, 5, 800, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "As first round starts some calculations are done...", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 10, Color.White);
              if (self.Game.EditObj.TutStep != 1)
                return;
              DrawMod.DrawTextColouredOutline( g, "Once the calculations have completed, you can begin playing.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 35, Color.White);
              let mut num49: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
              let mut num50: i32 =  Math.Round( (self.Game.ScreenHeight - 768) / 2.0);
              let mut num51: i32 = num49 + 485;
              let mut num52: i32 = num50 + 630;
               let mut local89: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local90: &Bitmap = &bitmap;
              let mut x: i32 = num51;
              let mut y: i32 = num52;
              DrawMod.DrawSimple( local89,  local90, x, y);
            }
            else if (self.Game.EditObj.TutStep == 2)
            {
              DrawMod.DrawTutback(g, 5, 5, 800, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "You have now started your turn. You get synopsis of what", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "happened in the turns of your opponent and any other news.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              let mut num53: i32 =  Math.Round( (self.Game.ScreenWidth - 1024) / 2.0);
              let mut num54: i32 =  Math.Round( (self.Game.ScreenHeight - 768) / 2.0);
              let mut num55: i32 = num53 + 485;
              let mut num56: i32 = num54 + 630;
               let mut local91: &Graphics = &g;
              bitmap: Bitmap = BitmapStore.GetBitmap(self.Game.TUTARROW);
               let mut local92: &Bitmap = &bitmap;
              let mut x: i32 = num55;
              let mut y: i32 = num56;
              DrawMod.DrawSimple( local91,  local92, x, y);
            }
            else
            {
              if (self.Game.EditObj.TutStep != 3)
                return;
              DrawMod.DrawTutback(g, 5, 5, 700, 60,  color.R,  color.G,  color.B,  color.A);
              DrawMod.DrawTextColouredOutline( g, "When news or messages popup you can just press a key.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 5, Color.White);
              DrawMod.DrawTextColouredOutline( g, "to continue or click the button.", Font::new(self.Game.FontCol.Families[1], 16f, FontStyle.Regular, GraphicsUnit.Pixel), 10, 30, Color.White);
              self.Game.EditObj.TutOrder = 9999;
            }
          }
        }
      }
    }

    pub WindowPresent: bool( System.Type WC)
    {
      if (self.WindowCounter > -1)
      {
        let mut windowCounter: i32 = self.WindowCounter;
        for (let mut index: i32 = 0; index <= windowCounter; index += 1)
        {
          if (self.WindowList[index].GetType().Equals(WC))
            return true;
        }
      }
      return false;
    }

    pub WindowClass GetWindow( System.Type WC)
    {
      if (self.WindowCounter > -1)
      {
        let mut windowCounter: i32 = self.WindowCounter;
        for (let mut index: i32 = 0; index <= windowCounter; index += 1)
        {
          if (self.WindowList[index].GetType().Equals(WC))
            return self.WindowList[index];
        }
      }
      return (WindowClass) null;
    }

    pub fn GetWindowID( System.Type WC) -> i32
    {
      if (self.WindowCounter > -1)
      {
        let mut windowCounter: i32 = self.WindowCounter;
        for (let mut index: i32 = 0; index <= windowCounter; index += 1)
        {
          if (self.WindowList[index].GetType().Equals(WC))
            return self.WindowID[index];
        }
      }
      return 0;
    }

    pub fn PaintPresentWindow(Graphics g,  System.Type WC)
    {
      let mut windowCounter: i32 = self.WindowCounter;
      for (let mut index: i32 = 0; index <= windowCounter; index += 1)
      {
        if (self.WindowList[index].GetType().Equals(WC))
        {
          self.WindowList[index].DoRefresh();
          if (Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass", false) != 0 & Operators.CompareString(self.WindowList[index].GetType().FullName, "WindowsApplication1.MapWindowClass2", false) != 0)
          {
            g.CompositingMode = CompositingMode.SourceCopy;
            DrawMod.DrawSimplePart( g,  self.OwnBackground, Rectangle::new(self.WindowX[index], self.WindowY[index], self.WindowW[index], self.WindowH[index]));
            g.CompositingMode = CompositingMode.SourceOver;
             let mut local1: &Graphics = &g;
            bitmap: Bitmap = self.WindowList[index].Paint();
             let mut local2: &Bitmap = &bitmap;
            let mut x: i32 = self.WindowX[index];
            let mut y: i32 = self.WindowY[index];
            DrawMod.DrawSimple( local1,  local2, x, y);
          }
          else
          {
            g.CompositingMode = CompositingMode.SourceCopy;
            DrawMod.DrawSimple( g,  self.WindowList[index].SubPartList[0].OwnBitmap, self.WindowX[index], self.WindowY[index]);
            g.CompositingMode = CompositingMode.SourceOver;
          }
        }
      }
    }
  }
}
