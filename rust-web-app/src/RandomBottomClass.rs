// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RandomBottomClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class RandomBottomClass : WindowClass
  {
     w: i32;
     h: i32;
     CurrentView: i32;

    pub RandomBottomClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, 32, 8)
    {
      self.NewGfx = true;
      self.w = tGame.ScreenWidth;
      self.h = 32;
      self.CurrentView = 0;
      self.dostuff();
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32) => base.HandleMouseMove(x, y);

    pub handleTimerWheel: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      windowReturnClass.Flag = false;
      if (self.game.EditObj.MouseWheel > 0 & self.game.EditObj.Zoom < 1 & self.game.EditObj.TutOrder == -1)
      {
        if (self.game.EditObj.MouseOverX > -1 & self.game.EditObj.MouseOverY > -1)
          self.game.HandyFunctionsObj.CenterOnXY(self.game.EditObj.MouseOverX, self.game.EditObj.MouseOverY, true);
        self.game.EditObj.MouseWheel = 0;
        self.game.EditObj.MouseWheelWait = 4;
        return self.actionZoomIn();
      }
      if (self.game.EditObj.MouseWheel < 0 & self.game.EditObj.Zoom > -1 & self.game.EditObj.TutOrder == -1)
      {
        let mut num: i32 = self.game.EditObj.MouseOverX > -1 & self.game.EditObj.MouseOverY > -1 & self.game.Data.Product <= 5 ? 1 : 0;
        self.game.EditObj.MouseWheel = 0;
        self.game.EditObj.MouseWheelWait = 4;
        return self.actionZoomOut();
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub actionZoomOut: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 = 0;
      if (self.game.EditObj.GuiDown)
        num1 = 222;
      let mut num2: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 53.0));
      let mut num3: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 106.0));
      let mut num4: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 53.0));
      let mut num5: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 106.0));
      num6: i32;
      num7: i32;
      if (self.game.EditObj.Zoom == 0)
      {
        self.game.EditObj.Zoom = -1;
        self.game.CornerX -=  Math.Round(Conversion.Int( num2 / 2.0));
        self.game.CornerY -=  Math.Round(Conversion.Int( num4 / 2.0));
        num6 = 27;
        num7 = 24;
      }
      else
      {
        self.game.EditObj.Zoom = 0;
        self.game.CornerX -=  Math.Round(Conversion.Int( num3 / 2.0));
        self.game.CornerY -=  Math.Round(Conversion.Int( num5 / 2.0));
        num6 = 53;
        num7 = 48;
      }
      if (((!self.game.Data.MapObj[0].MapLoop ? 1 : 0) | 1) != 0)
      {
        if ( self.game.CornerX +  self.game.ScreenWidth /  num6 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
          self.game.CornerX =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth) -  (self.game.ScreenWidth - 200) /  num6);
        if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num7 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
          self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num7);
        if (self.game.CornerX < 0)
          self.game.CornerX = 0;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
      }
      else
      {
        if (self.game.CornerX > self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
        {
          self.game.CornerX =  Math.Round( self.game.CornerX +  self.game.ScreenWidth /  num6);
          self.game.CornerX -= self.game.Data.MapObj[0].MapWidth;
        }
        if ( self.game.CornerY +  (self.game.ScreenHeight - (256 - num1)) /  num7 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight)
          self.game.CornerY =  Math.Round( (1 + self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight) -  (self.game.ScreenHeight - (256 - num1)) /  num7);
        if (self.game.CornerX < 0)
          self.game.CornerX = self.game.Data.MapObj[0].MapWidth + self.game.CornerX;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
      }
      self.game.EditObj.Save(self.game.AppPath + "editobj.txt");
      self.game.EditObj.TempCoordList = CoordList::new();
      windowReturnClass.AddCommand(1, 12);
      windowReturnClass.AddCommand(2, 12);
      windowReturnClass.AddCommand(4, 114);
      windowReturnClass.AddCommand(4, 9);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub actionZoomIn: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 = 0;
      if (self.game.EditObj.GuiDown)
        num1 = 222;
      let mut num2: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 53.0));
      let mut num3: i32 =  Math.Round(Conversion.Int( self.game.ScreenWidth / 106.0));
      let mut num4: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 53.0));
      let mut num5: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - (265 - num1)) / 106.0));
      num6: i32;
      num7: i32;
      if (self.game.EditObj.Zoom == 0)
      {
        self.game.EditObj.Zoom = 1;
        self.game.CornerX +=  Math.Round(Conversion.Int( num3 / 2.0));
        self.game.CornerY +=  Math.Round(Conversion.Int( num5 / 2.0));
        num6 = 106;
        num7 = 96;
      }
      else
      {
        self.game.EditObj.Zoom = 0;
        self.game.CornerX +=  Math.Round(Conversion.Int( num2 / 2.0));
        self.game.CornerY +=  Math.Round(Conversion.Int( num4 / 2.0));
        num6 = 53;
        num7 = 48;
      }
      if (((!self.game.Data.MapObj[0].MapLoop ? 1 : 0) | 1) != 0)
      {
        if ( self.game.CornerX +  self.game.ScreenWidth /  num6 >  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
          self.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num6);
        if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num1)) /  num7 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
          this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num1)) /  num7);
        if (this.game.CornerX < 0)
          this.game.CornerX = 0;
        if (this.game.CornerY < 0)
          this.game.CornerY = 0;
      }
      else
      {
        if (this.game.CornerX > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
          this.game.CornerX -= this.game.Data.MapObj[0].MapWidth;
        if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num1)) /  num7 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
          this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num1)) /  num7);
        if (this.game.CornerX < 0)
          this.game.CornerX = this.game.Data.MapObj[0].MapWidth + this.game.CornerX;
        if (this.game.CornerY < 0)
          this.game.CornerY = 0;
      }
      this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
      this.game.EditObj.TempCoordList = CoordList::new();
      windowReturnClass.AddCommand(1, 12);
      windowReturnClass.AddCommand(2, 12);
      windowReturnClass.AddCommand(4, 114);
      windowReturnClass.AddCommand(4, 9);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }

    pub fn DoRefresh()
    {
      this.game.EditObj.se1_map_data3cache_set = false;
      this.dostuff();
    }

    pub fn dostuff()
    {
      this.CurrentView = this.game.EditObj.SetViewMode2;
      this.ClearMouse();
      this.NewBackGroundAndClearAll(this.w, this.h, -1);
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      let mut num: i32 = 0;
      BitmapStore.GetBitmap(this.game.MARCTOPBAR).RotateFlip(RotateFlipType.Rotate180FlipX);
      for (; num < this.game.ScreenWidth; num += 100)
      {
         let mut local1: &Graphics = &Expression;
        bitmap: Bitmap = BitmapStore.GetBitmap(this.game.MARCTOPBAR);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = num;
        DrawMod.DrawSimple( local1,  local2, x, -4);
      }
      BitmapStore.GetBitmap(this.game.MARCTOPBAR).RotateFlip(RotateFlipType.Rotate180FlipX);
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 = this.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width && y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height)
        {
          if (this.MouseData[index] > 0)
            this.game.EditObj.TipButton = true;
          this.game.EditObj.TipTitle = this.MouseTitle[index];
          this.game.EditObj.TipText = this.MouseText[index];
          break;
        }
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.TutOrder > -1)
        return windowReturnClass;
      let mut mouseCounter: i32 = this.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (this.MouseData[index] > 0 && x > this.MouseRect[index].X & x < this.MouseRect[index].X + this.MouseRect[index].Width)
        {
          let mut num: i32 = y > this.MouseRect[index].Y & y < this.MouseRect[index].Y + this.MouseRect[index].Height ? 1 : 0;
        }
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut num1: i32 = 262;
      if ((nr == 187 | nr == 191 | nr == 107) & this.game.EditObj.Zoom < 1 & this.game.EditObj.TutOrder == -1)
      {
        let mut num2: i32 =  Math.Round(Conversion.Int( this.game.ScreenWidth / 53.0));
        let mut num3: i32 =  Math.Round(Conversion.Int( this.game.ScreenWidth / 106.0));
        let mut num4: i32 =  Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num1)) / 53.0));
        let mut num5: i32 =  Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num1)) / 106.0));
        num6: i32;
        num7: i32;
        if (this.game.EditObj.Zoom == 0)
        {
          this.game.EditObj.Zoom = 1;
          this.game.CornerX +=  Math.Round(Conversion.Int( num3 / 2.0));
          this.game.CornerY +=  Math.Round(Conversion.Int( num5 / 2.0));
          num6 = 106;
          num7 = 96;
        }
        else
        {
          this.game.EditObj.Zoom = 0;
          this.game.CornerX +=  Math.Round(Conversion.Int( num2 / 2.0));
          this.game.CornerY +=  Math.Round(Conversion.Int( num4 / 2.0));
          num6 = 53;
          num7 = 48;
        }
        if ( this.game.CornerX +  this.game.ScreenWidth /  num6 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
          this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num6);
        if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num1)) /  num7 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
          this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num1)) /  num7);
        if (this.game.CornerX < 0)
          this.game.CornerX = 0;
        if (this.game.CornerY < 0)
          this.game.CornerY = 0;
        this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
        this.game.EditObj.TempCoordList = CoordList::new();
        windowReturnClass.AddCommand(1, 12);
        windowReturnClass.AddCommand(2, 12);
        windowReturnClass.AddCommand(4, 114);
        windowReturnClass.AddCommand(4, 9);
        this.dostuff();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (!((nr == 189 | nr == 219 | nr == 109) & this.game.EditObj.Zoom > -1 & this.game.EditObj.TutOrder == -1))
        return windowReturnClass;
      let mut num8: i32 =  Math.Round(Conversion.Int( this.game.ScreenWidth / 53.0));
      let mut num9: i32 =  Math.Round(Conversion.Int( this.game.ScreenWidth / 106.0));
      let mut num10: i32 =  Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num1)) / 53.0));
      let mut num11: i32 =  Math.Round(Conversion.Int( (this.game.ScreenHeight - (265 - num1)) / 106.0));
      num12: i32;
      num13: i32;
      if (this.game.EditObj.Zoom == 0)
      {
        this.game.EditObj.Zoom = -1;
        this.game.CornerX -=  Math.Round(Conversion.Int( num8 / 2.0));
        this.game.CornerY -=  Math.Round(Conversion.Int( num10 / 2.0));
        num12 = 27;
        num13 = 24;
      }
      else
      {
        this.game.EditObj.Zoom = 0;
        this.game.CornerX -=  Math.Round(Conversion.Int( num9 / 2.0));
        this.game.CornerY -=  Math.Round(Conversion.Int( num11 / 2.0));
        num12 = 53;
        num13 = 48;
      }
      if ( this.game.CornerX +  this.game.ScreenWidth /  num12 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
        this.game.CornerX =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth) -  (this.game.ScreenWidth - 200) /  num12);
      if ( this.game.CornerY +  (this.game.ScreenHeight - (256 - num1)) /  num13 >  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
        this.game.CornerY =  Math.Round( (1 + this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) -  (this.game.ScreenHeight - (256 - num1)) /  num13);
      if (this.game.CornerX < 0)
        this.game.CornerX = 0;
      if (this.game.CornerY < 0)
        this.game.CornerY = 0;
      this.game.EditObj.Save(this.game.AppPath + "editobj.txt");
      this.game.EditObj.TempCoordList = CoordList::new();
      windowReturnClass.AddCommand(1, 12);
      windowReturnClass.AddCommand(2, 12);
      windowReturnClass.AddCommand(4, 114);
      windowReturnClass.AddCommand(4, 9);
      this.dostuff();
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }
  }
}
