// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MiniMapPartClass
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
  pub class MiniMapPartClass : SubPartClass
  {
     game: GameClass;
     bool paintview;
     bool alsounits;
     bool realhex;
     specialMode1: i32;
     minix: i32;
     miniy: i32;
     minimap: i32;
     MapWidth: i32;
     MapHeight: i32;
    pub tZoomLevel: i32;
    pub tempAi2Use: bool;
    pub tempValue4mustBe: i32;
    pub tempValue3usedForAlpha: bool;
    pub tempZones: bool;
    pub blockMapMove: bool;

    pub MiniMapPartClass(
      tgame: GameClass,
      bool tpaintview = true,
      let mut tx: i32 =  200,
      let mut ty: i32 =  150,
      bool talsounits = false,
      bool trealhex = false,
      let mut tMapWidth: i32 =  -1,
      let mut tMapHeight: i32 =  -1,
      let mut ZoomLevel: i32 =  -1,
      let mut humanplayer: i32 =  -1,
      bool alsoHQ = false,
      let mut ttempValue4mustBe: i32 =  -1,
      bool tblockMapMove = false,
      bool tTempValue3usedForAlpha = false,
      bool tTempAi2use = false,
      bool tTempZones = false,
      let mut tspecialMode1: i32 =  -1)
      : base(tx, ty)
    {
      this.minix = tx;
      this.miniy = ty;
      this.alsounits = talsounits;
      this.game = tgame;
      this.realhex = trealhex;
      this.tempAi2Use = tTempAi2use;
      this.tempZones = tTempZones;
      this.paintview = tpaintview;
      this.specialMode1 = tspecialMode1;
      this.MapWidth = tMapWidth;
      this.MapHeight = tMapHeight;
      this.tempValue4mustBe = ttempValue4mustBe;
      this.tempValue3usedForAlpha = tTempValue3usedForAlpha;
      this.blockMapMove = tblockMapMove;
      this.tZoomLevel = ZoomLevel != -1 ? ZoomLevel : this.game.EditObj.Zoom;
      if (this.game.Data.MapCounter > 0)
        this.game.EditObj.StratMap = (Bitmap) null;
      if (Information.IsNothing( this.game.EditObj.MiniMap))
      {
        this.game.EditObj.MiniMap = new Bitmap(205, 110, PixelFormat.Format32bppPArgb);
        this.game.EditObj.MiniMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
        this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, 205, 110, false);
      }
      if (!(this.game.EditObj.MiniMap.Width == this.minix & this.game.EditObj.MiniMap.Height == this.miniy) | humanplayer > -1)
      {
        if (tx == this.game.ScreenWidth & !Information.IsNothing( this.game.EditObj.StratMap))
        {
          if (!Information.IsNothing( this.game.CustomBitmapObj.miniMapPredrawnCache))
          {
            this.game.CustomBitmapObj.miniMapPredrawnCache.Dispose();
            this.game.CustomBitmapObj.miniMapPredrawnCache = (Bitmap) null;
          }
          this.game.EditObj.MiniMap = (Bitmap) this.game.EditObj.StratMap.Clone();
          if ( this.game.Data.RuleVar[839] < 1.0)
            this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex, predrawn: true, humanplayer: humanplayer, showflag: true, alsoHQ: alsoHQ, highlightTempvar4: this.tempValue4mustBe, useTempVar3asAlpha: this.tempValue3usedForAlpha, useTempAi2: this.tempAi2Use, useTempZones: this.tempZones, specialMode1: this.specialMode1);
          else
            this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex, predrawn: true, humanplayer: humanplayer, alsoHQ: alsoHQ, highlightTempvar4: this.tempValue4mustBe, useTempVar3asAlpha: this.tempValue3usedForAlpha, useTempAi2: this.tempAi2Use, useTempZones: this.tempZones, specialMode1: this.specialMode1);
        }
        else
        {
          if (!Information.IsNothing( this.game.CustomBitmapObj.miniMapPredrawnCache))
          {
            this.game.CustomBitmapObj.miniMapPredrawnCache.Dispose();
            this.game.CustomBitmapObj.miniMapPredrawnCache = (Bitmap) null;
          }
          this.game.EditObj.MiniMap = new Bitmap(this.minix, this.miniy, PixelFormat.Format32bppPArgb);
          this.game.EditObj.MiniMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
          if ( this.game.Data.RuleVar[839] < 1.0)
            this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex, humanplayer: humanplayer, showflag: true, alsoHQ: alsoHQ, highlightTempvar4: this.tempValue4mustBe, useTempVar3asAlpha: this.tempValue3usedForAlpha, useTempAi2: this.tempAi2Use, useTempZones: this.tempZones, specialMode1: this.specialMode1);
          else
            this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex, humanplayer: humanplayer, alsoHQ: alsoHQ, highlightTempvar4: this.tempValue4mustBe, useTempVar3asAlpha: this.tempValue3usedForAlpha, useTempAi2: this.tempAi2Use, useTempZones: this.tempZones, specialMode1: this.specialMode1);
        }
      }
      else
      {
        if (Information.IsNothing( this.game.EditObj.StratMap))
          return;
        this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex, humanplayer: humanplayer, alsoHQ: alsoHQ, highlightTempvar4: this.tempValue4mustBe, useTempVar3asAlpha: this.tempValue3usedForAlpha, useTempAi2: this.tempAi2Use, useTempZones: this.tempZones, specialMode1: this.specialMode1);
      }
    }

    pub Paint: Bitmap()
    {
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawSimpleFast( Expression,  this.game.EditObj.MiniMap,  this.OwnBitmap, 0, 0);
      if (this.game.EditObj.MiniMap.Width < 310 | this.specialMode1 > -1)
      {
        Coordinate realCoord1 = this.GetRealCoord(this.game.CornerX, this.game.CornerY);
        let mut num1: i32 =  realCoord1.x;
        let mut num2: i32 =  realCoord1.y;
        num3: i32;
        num4: i32;
        if (this.tZoomLevel == -1)
        {
          num3 = 27;
          num4 = 24;
        }
        else if (this.tZoomLevel == 0)
        {
          num3 = 52;
          num4 = 48;
        }
        else if (this.tZoomLevel == 1)
        {
          num3 = 104;
          num4 = 96;
        }
        num5: i32;
        num6: i32;
        if (this.game.EditObj.OrderType == 24)
        {
          num5 =  Math.Round( (this.game.ScreenWidth - 0) /  num3);
          num6 =  Math.Round( (this.game.ScreenHeight - 305) /  num4);
        }
        else
        {
          num5 =  this.game.Data.RuleVar[839] != 0.0 ?  Math.Round( (this.game.ScreenWidth - 0 - 106) /  num3) : (this.game.EditObj.Layout != 1 ?  Math.Round( (this.game.ScreenWidth - 220 - 106) /  num3) :  Math.Round( (this.game.ScreenWidth - 440 - 106) /  num3));
          num6 = this.game.Data.Product != 7 ?  Math.Round( (this.game.ScreenHeight - 265) /  num4) : (!(Operators.CompareString(this.game.FormRef.Screeny.GetType().FullName, "WindowsApplication1.RandomScreenClass2", false) == 0 | this.game.EditObj.GuiDown) ?  Math.Round( (this.game.ScreenHeight - 265) /  num4) :  Math.Round( (this.game.ScreenHeight - 45) /  num4));
        }
        if (this.MapWidth > -1)
        {
          num5 =  Math.Round( this.MapWidth /  num3);
          num6 =  Math.Round( this.MapHeight /  num4);
        }
        Coordinate realCoord2 = this.GetRealCoord(this.game.CornerX + (num5 + 1) + 1, this.game.CornerY + num6 + 1);
        let mut w: i32 =  realCoord2.x - num1;
        let mut h: i32 =  realCoord2.y - num2;
        let mut x1: i32 =  this.game.EditObj.MiniMapOffset - 1;
        if (x1 < 0)
          x1 = this.game.Data.MapObj[0].MapWidth + x1 + 1;
        realCoord2 = this.GetRealCoord(x1, this.game.CornerY + h + 1);
        let mut x2: i32 =  realCoord2.x;
        realCoord2 = this.GetRealCoord(this.game.EditObj.MiniMapOffset, this.game.CornerY + h + 1);
        let mut x3: i32 =  realCoord2.x;
        if (w < 0)
          w = x2 + w;
        if (0 > num1)
          num1 = 0;
        if (0 > num2)
          num2 = 0;
        if (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0)
        {
          if (num1 + w > this.OwnBitmap.Width)
            num1 = this.OwnBitmap.Width - w;
          if (num2 + h > this.OwnBitmap.Height - 3)
            num2 = this.OwnBitmap.Height - 3 - h;
        }
        if (this.game.Data.Round == 0 & this.game.Data.ShrowdOn)
        {
          Expression.Clear(Color.Black);
          DrawMod.DrawText( Expression, "Shrouded Map", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 60, 68);
        }
        DrawMod.DrawRectangle( Expression, 0, 0, this.OwnBitmap.Width - 1, this.OwnBitmap.Height - 1, 0, 0, 0,  byte.MaxValue);
        if (this.paintview)
        {
          if (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0 | this.game.Data.Round == 0 | num1 + w <= x2)
          {
            DrawMod.DrawRectangle( Expression, num1, num2, w, h,  byte.MaxValue, 0, 0,  byte.MaxValue);
            DrawMod.DrawRectangle( Expression, num1 + 1, num2 + 1, w - 2, h - 2, 0, 0, 0,  byte.MaxValue);
          }
          else
          {
            DrawMod.drawLine( Expression, num1, num2, num1 + w - (num1 + w - x2), num2,  byte.MaxValue, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, num1, num2 + h, num1 + w - (num1 + w - x2), num2 + h,  byte.MaxValue, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, num1, num2, num1, num2 + h,  byte.MaxValue, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, num1 + 1, num2 + 1, num1 + w - (num1 + w - x2), num2 + 1, 0, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, num1 + 1, num2 + h - 1, num1 + w - (num1 + w - x2), num2 + h - 1, 0, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, num1 + 1, num2 + 1, num1 + 1, num2 + h - 1, 0, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, x3, num2, x3 + (num1 + w - x2), num2,  byte.MaxValue, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, x3, num2 + h, x3 + (num1 + w - x2), num2 + h,  byte.MaxValue, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, x3 + (num1 + w - x2), num2, x3 + (num1 + w - x2), num2 + h,  byte.MaxValue, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, x3 + 1, num2 + 1, x3 + (num1 + w - x2), num2 + 1, 0, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, x3 + 1, num2 + h - 1, x3 + (num1 + w - x2), num2 + h - 1, 0, 0, 0,  byte.MaxValue);
            DrawMod.drawLine( Expression, x3 + 1 + (num1 + w - x2), num2 + 1, x3 + (num1 + w - x2) + 1, num2 + h - 1, 0, 0, 0,  byte.MaxValue);
          }
        }
      }
      if (!Information.IsNothing( Expression))
      {
        Expression.Dispose();
        Expression = (Graphics) null;
      }
      return this.OwnBitmap;
    }

    pub Coordinate GetRealCoord(x: i32, y: i32)
    {
      if (this.game.EditObj.MiniMapOffset > 0)
      {
        x -= this.game.EditObj.MiniMapOffset;
        if (x < 0)
          x = this.game.Data.MapObj[0].MapWidth + 1 + x;
      }
      float d1 =  this.minix /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1);
      float d2 =  this.miniy /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1);
      if (this.MapWidth > 310)
      {
        d1 =  Math.Floor( d1);
        d2 =  Math.Floor( d2);
      }
      float num1;
      if ( d1 >  d2)
      {
        num1 =  ( this.minix / 2.0 -  d2 /  d1 * ( this.minix / 2.0));
        d1 = d2;
      }
      float num2;
      if ( d2 >  d1)
      {
        num2 =  ( this.miniy / 2.0 -  d1 /  d2 * ( this.miniy / 2.0));
        d2 = d1;
      }
      if (this.minix > 310)
      {
        float num3 =  this.minix -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1) * d1;
        if ( num3 > 0.0)
        {
          float num4 =   Math.Round( (num3 / 2f));
          if ( num4 >  num1)
            num1 = num4;
        }
      }
      if (this.miniy > 220)
      {
        float num5 =  this.miniy -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1) * d2;
        if ( num5 > 0.0)
        {
          float num6 =   Math.Round( (num5 / 2f));
          if ( num6 >  num2)
            num2 = num6;
        }
      }
      float a1 = Conversion.Int(d1 *  x) + num1;
      float a2 = Conversion.Int(d2 *  y) - d2 / 2f + num2;
      Coordinate realCoord;
      realCoord.x =  Math.Round( a1);
      realCoord.y =  Math.Round( a2);
      return realCoord;
    }

    pub Coordinate GetHexCoord(x: i32, y: i32)
    {
      float d1 =  this.minix /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1);
      float d2 =  this.miniy /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1);
      float num1;
      if ( d1 >  d2)
      {
        num1 =  ( this.minix / 2.0 -  d2 /  d1 * ( this.minix / 2.0));
        d1 = d2;
      }
      float num2;
      if ( d2 >  d1)
      {
        num2 =  ( this.miniy / 2.0 -  d1 /  d2 * ( this.miniy / 2.0));
        d2 = d1;
      }
      if (this.MapWidth > 310)
      {
        d1 =  Math.Floor( d1);
        d2 =  Math.Floor( d2);
      }
      if (this.minix > 310)
      {
        float num3 =  this.minix -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1) * d1;
        if ( num3 > 0.0)
        {
          float num4 =   Math.Round( (num3 / 2f));
          if ( num4 >  num1)
            num1 = num4;
        }
      }
      if (this.miniy > 220)
      {
        float num5 =  this.miniy -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1) * d2;
        if ( num5 > 0.0)
        {
          float num6 =   Math.Round( (num5 / 2f));
          if ( num6 >  num2)
            num2 = num6;
        }
      }
      float a1 =   Math.Round(Math.Floor(( x -  num1) /  d1));
      if (( a1 + 10.0) % 2.0 > 0.0)
        y =  Math.Round( ( y - d2 / 2f));
      float a2 =   Math.Round(Math.Floor(( y -  num2 +   Math.Round( (d2 / 2f))) /  d2));
      if (this.game.EditObj.MiniMapOffset > 0)
      {
        a1 +=  this.game.EditObj.MiniMapOffset;
        if ( a1 >  this.game.Data.MapObj[0].MapWidth)
          a1 -=  (this.game.Data.MapObj[0].MapWidth + 1);
      }
      Coordinate hexCoord;
      hexCoord.x =  Math.Round( a1);
      hexCoord.y =  Math.Round( a2);
      hexCoord.onmap = true;
      if (hexCoord.x < 0 | hexCoord.y < 0)
        hexCoord.onmap = false;
      if (hexCoord.x > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
        hexCoord.onmap = false;
      if (hexCoord.y > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
        hexCoord.onmap = false;
      return hexCoord;
    }

    pub bool HandleTimerWheel(x: i32, y: i32,  WindowClass tWindow)
    {
      if (this.game.Data.MapObj[0].MapLoop)
      {
        let mut num: i32 =   Math.Round(Math.Ceiling( this.game.Data.MapObj[0].MapWidth / 20.0));
        if (this.game.EditObj.MouseWheel > 0)
        {
          this.game.EditObj.MiniMapOffset -= num;
          if (this.game.EditObj.MiniMapOffset < 0)
            this.game.EditObj.MiniMapOffset = this.game.Data.MapObj[0].MapWidth + this.game.EditObj.MiniMapOffset;
          if ((this.game.EditObj.MiniMapOffset + 2) % 2 > 0)
            --this.game.EditObj.MiniMapOffset;
          this.game.EditObj.MouseWheel = 0;
          this.game.EditObj.MouseWheelWait = 4;
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex);
          this.Paint();
          return true;
        }
        if (this.game.EditObj.MouseWheel < 0)
        {
          this.game.EditObj.MiniMapOffset += num;
          if (this.game.EditObj.MiniMapOffset >= this.game.Data.MapObj[0].MapWidth)
            this.game.EditObj.MiniMapOffset -= this.game.Data.MapObj[0].MapWidth;
          if ((this.game.EditObj.MiniMapOffset + 2) % 2 > 0)
            --this.game.EditObj.MiniMapOffset;
          this.game.EditObj.MouseWheel = 0;
          this.game.EditObj.MouseWheelWait = 4;
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex);
          this.Paint();
          return true;
        }
      }
      return false;
    }

    pub fn Click(x: i32, y: i32, let mut b: i32 =  1) -> i32
    {
      let mut cornerX: i32 =  this.game.CornerX;
      let mut cornerY: i32 =  this.game.CornerY;
      float d1 =  this.minix /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1);
      float d2 =  this.miniy /  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1);
      float num1;
      if ( d1 >  d2)
      {
        num1 =  ( this.minix / 2.0 -  d2 /  d1 * ( this.minix / 2.0));
        d1 = d2;
      }
      float num2;
      if ( d2 >  d1)
      {
        num2 =  ( this.miniy / 2.0 -  d1 /  d2 * ( this.miniy / 2.0));
        d2 = d1;
      }
      float num3 =  Math.Floor( d1);
      float num4 =  Math.Floor( d2);
      if (this.minix > 310)
      {
        let mut num5: i32 =   Math.Round( ( this.minix -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1) * num3));
        if (num5 > 0)
        {
          let mut num6: i32 =   Math.Round( num5 / 2.0);
          if ( num6 >  num1)
            num1 =  num6;
        }
      }
      if (this.miniy > 220)
      {
        let mut num7: i32 =   Math.Round( ( this.miniy -  (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1) * num4));
        if (num7 > 0)
        {
          let mut num8: i32 =   Math.Round( num7 / 2.0);
          if ( num8 >  num2)
            num2 =  num8;
        }
      }
      Number1: i32;
      Number2: i32;
      if (this.MapWidth < 310)
      {
        Number1 =  Math.Round( ( (( x -  num1) / ( this.minix -  num1 * 2.0)) *  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth));
        Number2 =  Math.Round( ( (( y -  num2) / ( this.miniy -  num2 * 2.0)) *  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight));
      }
      else
      {
        Number1 =  Math.Round( (Math.Max(0.0f,  x - num1 - num3) / num3));
        if ((Number1 + 2) % 2 == 1)
        {
          num9: i32;
          num9 -=  Math.Round( (num4 / 2f));
        }
        Number2 =  Math.Round( (( y - num2) / num4));
      }
      this.game.EditObj.CurrentMiniX = Conversion.Int(Number1);
      this.game.EditObj.CurrentMiniY = Conversion.Int(Number2);
      if (this.tZoomLevel == 1)
        Number2 += 3;
      if (this.tZoomLevel == 0)
        Number2 += 3;
      if (this.tZoomLevel == -1)
        Number2 += 6;
      let mut num10: i32 =   this.game.Data.RuleVar[839] != 0.0 ? this.game.ScreenWidth - 0 : (this.game.EditObj.Layout != 1 ? this.game.ScreenWidth - 220 : this.game.ScreenWidth - 440);
      let mut num11: i32 =  this.game.ScreenHeight - 280;
      let mut num12: i32 =   this.game.Data.RuleVar[839] != 1.0 ? (this.tZoomLevel != -1 ? (this.tZoomLevel != 0 ? 128 : 64) : 32) : (this.tZoomLevel != -1 ? (this.tZoomLevel != 0 ? 106 : 53) : 27);
      num13: i32;
      num14: i32;
      num15: i32;
      num16: i32;
      if (this.MapWidth > -1)
      {
        let mut num17: i32 =   Math.Round( this.MapWidth /  num12);
        let mut num18: i32 =   Math.Round( this.MapHeight /  (24 * (this.tZoomLevel + 2)));
        num13 = Number1 -  Math.Round( num17 / 2.0);
        num14 = Number2 -  Math.Round( num18 / 2.0);
        num15 = num17;
        num16 = num18;
      }
      else
      {
        num13 = this.game.EditObj.Layout != 0 ?  Math.Round( Number1 -  (num10 - 0) /  num12 / 2.0) :  Math.Round( Number1 -  (num10 - 0) /  num12 / 2.0);
        num14 =  Math.Round( Number2 -  num11 /  (24 * (this.tZoomLevel + 2)) / 2.0);
        let mut num19: i32 =  265;
        if ( this.game.Data.RuleVar[839] == 0.0)
          num19 = 305;
        if (this.game.Data.Round == 0)
          num19 += 100;
        num15 =  Math.Round( num10 /  num12);
        num16 =  Math.Round( (num11 - num19) /  (24 * (this.tZoomLevel + 2)));
      }
      if (0 > num14)
        num14 = 0;
      let mut num20: i32 =  num14 - 1;
      if (this.game.EditObj.MiniMapOffset > 0)
      {
        num13 += this.game.EditObj.MiniMapOffset;
        if (num13 > this.game.Data.MapObj[0].MapWidth)
          num13 -= this.game.Data.MapObj[0].MapWidth + 1;
      }
      this.game.CornerX = num13;
      this.game.CornerY = num20;
      let mut num21: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - this.game.CornerX + 2;
      let mut num22: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - this.game.CornerY + 3;
      if (num15 > num21 & (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0 | this.game.Data.Round == 0))
        this.game.CornerX -= num15 - num21;
      if (num16 > num22)
        this.game.CornerY -= num16 - num22;
      if (0 > this.game.CornerX & (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0 | this.game.Data.Round == 0))
        this.game.CornerX = 0;
      if (-1 > this.game.CornerY)
        this.game.CornerY = -1;
      if (0 > this.game.CornerX & this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop &  this.game.Data.RuleVar[329] == 0.0 & this.game.Data.Round > 0)
        this.game.CornerX = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + this.game.CornerX + 1;
      if (!this.realhex)
        this.game.CustomBitmapObj.MakeMiniMap(this.game.EditObj.MiniMap, this.minix, this.miniy, this.alsounits, this.realhex);
      this.game.EditObj.TempCoordList = CoordList::new();
      this.Paint();
      if (b == 3)
      {
        this.game.CornerX = cornerX;
        this.game.CornerY = cornerY;
      }
      return 0;
    }
  }
}
