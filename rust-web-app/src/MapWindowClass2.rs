// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MapWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class MapWindowClass2 : WindowClass
  {
     MapId: i32;
     minheight: i32;
     minwidth: i32;
     ZoomTimer: i32;
     float LastZoom;
     DateTime lastclickleft;
     DateTime lastscroll;
     lastMouseOverX: i32;
     lastMouseOverY: i32;
     lastUnitSelected: i32;

    pub MapWindowClass2( tGame: GameClass, let mut tminheight: i32 =  0, let mut tminwidth: i32 =  200, let mut tZoomLevel: i32 =  -2)
      : base( tGame, tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight)
    {
      this.game.EditObj.se1_map_data3cache_set = false;
      this.minheight = tminheight;
      this.minwidth = tminwidth;
      if (this.game.EditObj.OrderUnit > this.game.Data.UnitCounter)
        this.game.EditObj.OrderUnit = -1;
      let mut tsubpart: SubPartClass =  new MapPartClass(tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight, tGame, ZoomLevel: tZoomLevel);
      this.MapId = this.AddSubPart( tsubpart, 0, 0, tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight, 0);
      this.game.EditObj.TempCoordList = CoordList::new();
      this.game.EditObj.mouseOverActive = false;
      this.SubPartList[this.SubpartNr(this.MapId)].Paint();
      this.mapframe = true;
      this.lastMouseOverX = -1;
      this.lastUnitSelected = -1;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
    }

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.Zoom == 1)
      {
        this += 1.ZoomTimer;
        if (this.ZoomTimer == 1)
          return windowReturnClass;
        this.ZoomTimer = 0;
      }
      if (this.game.FormRef.WindowState == FormWindowState.Minimized & !this.game.EditObj.BattleTimerActive)
        return windowReturnClass;
      if ( this.game.Data.RuleVar[701] >= 1.0 & this.game.EditObj.udsReturnFromPopup)
      {
        this.DoRefresh();
        return windowReturnClass;
      }
      DateTime now = DateAndTime.Now;
      if (!(this.game.EditObj.ScrollDir > 0 & !this.game.EditObj.BattleTimerActive))
        this.lastscroll = DateAndTime.Now;
      TimeSpan timeSpan = now.Subtract(this.lastscroll);
      let mut num: i32 =  timeSpan.Milliseconds + timeSpan.Seconds * 1000;
      if (this.game.EditObj.ScrollDir == 1 & !this.game.EditObj.BattleTimerActive & num >= 0)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(38, false);
      }
      if (this.game.EditObj.ScrollDir == 2 & !this.game.EditObj.BattleTimerActive & num >= 0)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(39, false);
      }
      if (this.game.EditObj.ScrollDir == 3 & !this.game.EditObj.BattleTimerActive & num >= 0)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(40, false);
      }
      if (this.game.EditObj.ScrollDir == 4 & !this.game.EditObj.BattleTimerActive & num >= 0)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(37, false);
      }
      if (!this.game.AIThreadRunning && this.game.EditObj.BattleTimerActive)
      {
        if (this.game.EditObj.BattleAnimNr == 0 && this.game.EditObj.udsUnitOrderMode != 33 & this.game.EditObj.OrderType != 33)
        {
          let mut volume2: i32 =  this.game.EditObj.Volume2;
          this.game.EditObj.Volume2 =  Math.Round( this.game.EditObj.Volume2 / 2.0);
          if (this.game.EditObj.SoundOn)
            SoundMod.PlayAWave(this.game.AppPath + "sound/blow.wav",  this.game.EditObj);
          this.game.EditObj.Volume2 = volume2;
        }
        if (DateTime.Compare(DateTime.Now, this.game.EditObj.BattleTimer) > 0)
        {
          this.game.EditObj.BattleTimerActive = false;
          this.game.EditObj.BattleAnimNr = 0;
          this.game.TempCombat = new CombatClass(this.game);
          Coordinate Target = Coordinate::new();
          Target.x = this.game.EditObj.TargetX;
          Target.y = this.game.EditObj.TargetY;
          Target.map = this.game.EditObj.TargetMap;
          if (this.game.EditObj.OrderType == 0 && this.game.EditObj.udsUnitOrderMode > 0)
            this.game.EditObj.OrderType = this.game.EditObj.udsUnitOrderMode;
          if (Target.x > -1)
          {
            if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
            {
              if ( this.game.Data.RuleVar[839] == 1.0)
              {
                this.game.EditObj.PopupValue = 7;
                windowReturnClass.AddCommand(5, 14);
                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.AddCommand(3, 5);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
          else
            this.game.EditObj.OrderType = 0;
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        this.game.EditObj.BattleAnimNr += 2;
        if (this.game.EditObj.BattleAnimNr > 32)
          this.game.EditObj.BattleAnimNr = 32;
        let mut subPart: SubPartClass = this.SubPartList[this.SubpartNr(this.MapId)];
        let mut targetX: i32 =  this.game.EditObj.TargetX;
        let mut targetY: i32 =  this.game.EditObj.TargetY;
        let mut mapSelected: i32 =  this.game.EditObj.MapSelected;
        bitmap: Bitmap = (Bitmap) null;
         let mut local: &Bitmap = &bitmap;
        subPart.PaintCoordinate((Graphics) null, targetX, targetY, mapSelected, gBitmap: ( local));
        this.game.EditObj.TempCoordList = CoordList::new();
        this.game.EditObj.TempCoordList.active = true;
        this.PaintCurrentBitmap(this.MapId);
        windowReturnClass.SetFlag(true);
        if ( this.game.Data.RuleVar[839] == 0.0)
        {
          windowReturnClass.AddCommand(4, 66);
        }
        else
        {
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
        }
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.EditObj.BattleTimerActive)
        return windowReturnClass;
      bool flag = false;
      let mut cornerX1: i32 =  this.game.CornerX;
      let mut cornerY1: i32 =  this.game.CornerY;
      if (this.game.EventRelatedObj.Helper_IsDebug() && nr == 46)
      {
        let mut memorySize1: i32 =  this.game.FormRef.GetMemorySize();
        let mut memorySize2: i32 =  BitmapStore.GetMemorySize();
        let mut num: i32 =   Interaction.MsgBox( ("Memory Used Right Now by GUI (" + Conversion.Str( memorySize1) + "KB) + Bitmapstore (" + Conversion.Str( memorySize2) + "KB) =" + Conversion.Str(  Math.Round( (memorySize1 + memorySize2) / 1000.0)) + "MB. (Will write log files after you press OK)"), Title: ( "Shadow Empire : Planetary Conquest"));
        StreamWriter text1 = File.CreateText(this.game.AppPath + "logs/" + this.game.Data.Name + "_missingsystemgraphics.txt");
        let mut counter1: i32 =  BitmapStore.Counter;
        for (let mut nr1: i32 =  0; nr1 <= counter1; nr1 += 1)
        {
          if (BitmapStore.tmpIsSystem[nr1] && BitmapStore.GetWidth(nr1) == 1 & BitmapStore.Getheight(nr1) == 1)
          {
            text1.Write("\r\n");
            text1.Write(BitmapStore.tmpFileName[nr1]);
          }
        }
        text1.Close();
        StreamWriter text2 = File.CreateText(this.game.AppPath + "logs/" + this.game.Data.Name + "_presentscenariographics.txt");
        let mut counter2: i32 =  BitmapStore.Counter;
        for (let mut index: i32 =  0; index <= counter2; index += 1)
        {
          if (!BitmapStore.tmpIsSystem[index])
          {
            text2.Write("\r\n");
            text2.Write(BitmapStore.tmpFileName[index]);
          }
        }
        text2.Close();
      }
      if (nr == 39)
      {
        this += 1.game.CornerX;
        flag = true;
      }
      if (nr == 37 & (this.game.CornerX > 0 | this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & ( this.game.Data.RuleVar[329] == 0.0 | this.game.Data.Round > 0)))
      {
        --this.game.CornerX;
        if (0 > this.game.CornerX & (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0 | this.game.Data.Round == 0))
          this.game.CornerX = 0;
        flag = true;
      }
      if (nr == 40)
      {
        this += 1.game.CornerY;
        flag = true;
      }
      if (nr == 38 & this.game.CornerY > -1)
      {
        --this.game.CornerY;
        if (this.game.CornerY < -1)
          this.game.CornerY = -1;
        flag = true;
      }
      cornerX2: i32;
      cornerY2: i32;
      if (nr == 57 & this.game.EditObj.layerUnits)
      {
        let mut index1: i32 =  this.game.HandyFunctionsObj.NextCycleUnit(this.game.EditObj.UnitSelected, true);
        if (!(index1 != this.game.EditObj.UnitSelected & index1 > -1))
          return windowReturnClass;
        this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
        this.game.EditObj.UnitSelected = index1;
        this.game.SelectX = this.game.Data.UnitObj[index1].X;
        this.game.SelectY = this.game.Data.UnitObj[index1].Y;
        cornerX2 = this.game.CornerX;
        cornerY2 = this.game.CornerY;
        this.game.HandyFunctionsObj.SetcornerXY2(this.game.Data.UnitObj[index1].X, this.game.Data.UnitObj[index1].Y, true);
        let mut index2: i32 =  0;
        let mut num: i32 =  index1;
        while (this.game.Data.MapObj[index2].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0] != num)
        {
          let mut unit: i32 =  this.game.Data.MapObj[index2].HexObj[this.game.SelectX, this.game.SelectY].UnitList[this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter];
          index2 = 0;
          if (this.game.Data.MapObj[index2].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > 0)
          {
            for (let mut unitCounter: i32 =  this.game.Data.MapObj[index2].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter; unitCounter >= 1; unitCounter += -1)
              this.game.Data.MapObj[index2].HexObj[this.game.SelectX, this.game.SelectY].UnitList[unitCounter] = this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].UnitList[unitCounter - 1];
          }
          this.game.Data.MapObj[index2].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0] = unit;
        }
        this.UdsClickUnit(this.game.Data.UnitObj[index1].X, this.game.Data.UnitObj[index1].Y, 0, true);
        windowReturnClass.AddCommand(4, 69);
        windowReturnClass.AddCommand(4, 67);
        windowReturnClass.AddCommand(4, 68);
        windowReturnClass.AddCommand(4, 9);
        this.DoRefresh();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (nr == 56 & this.game.EditObj.layerUnits)
      {
        let mut index3: i32 =  this.game.HandyFunctionsObj.NextCycleUnit(this.game.EditObj.UnitSelected, false);
        if (index3 != this.game.EditObj.UnitSelected & index3 > -1)
        {
          this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.UnitSelected = index3;
          this.game.SelectX = this.game.Data.UnitObj[index3].X;
          this.game.SelectY = this.game.Data.UnitObj[index3].Y;
          cornerX2 = this.game.CornerX;
          cornerY2 = this.game.CornerY;
          this.game.HandyFunctionsObj.SetcornerXY2(this.game.Data.UnitObj[index3].X, this.game.Data.UnitObj[index3].Y, true);
          let mut index4: i32 =  0;
          let mut num: i32 =  index3;
          while (this.game.Data.MapObj[index4].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0] != num)
          {
            let mut unit: i32 =  this.game.Data.MapObj[index4].HexObj[this.game.SelectX, this.game.SelectY].UnitList[this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter];
            index4 = 0;
            if (this.game.Data.MapObj[index4].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > 0)
            {
              for (let mut unitCounter: i32 =  this.game.Data.MapObj[index4].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter; unitCounter >= 1; unitCounter += -1)
                this.game.Data.MapObj[index4].HexObj[this.game.SelectX, this.game.SelectY].UnitList[unitCounter] = this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].UnitList[unitCounter - 1];
            }
            this.game.Data.MapObj[index4].HexObj[this.game.SelectX, this.game.SelectY].UnitList[0] = unit;
          }
          this.UdsClickUnit(this.game.Data.UnitObj[index3].X, this.game.Data.UnitObj[index3].Y, 0, true);
          windowReturnClass.AddCommand(4, 69);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
          this.DoRefresh();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (nr == 48 & this.game.EditObj.layerUnits && this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
      {
        if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].cycleOrder == 0L)
          this.game.HandyFunctionsObj.SetCycleOrder(this.game.EditObj.UnitSelected);
        num: i32;
        if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].cycleOrder < 0L)
        {
          num = 1;
          this.game.Data.UnitObj[this.game.EditObj.UnitSelected].cycleOrder = Math.Abs(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].cycleOrder);
        }
        else
        {
          num = 2;
          this.game.Data.UnitObj[this.game.EditObj.UnitSelected].cycleOrder = -Math.Abs(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].cycleOrder);
        }
        if (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].Type == 5)
        {
          let mut unitCounter: i32 =  this.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (this.game.Data.UnitObj[index].HQ == this.game.EditObj.UnitSelected)
            {
              switch (num)
              {
                case 1:
                  this.game.Data.UnitObj[index].cycleOrder = Math.Abs(this.game.Data.UnitObj[index].cycleOrder);
                  continue;
                case 2:
                  this.game.Data.UnitObj[index].cycleOrder = -Math.Abs(this.game.Data.UnitObj[index].cycleOrder);
                  continue;
                default:
                  continue;
              }
            }
          }
        }
        windowReturnClass.AddCommand(4, 69);
        windowReturnClass.AddCommand(4, 67);
        windowReturnClass.AddCommand(4, 68);
        windowReturnClass.AddCommand(4, 9);
        this.DoRefresh();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      if (nr == 81 & this.game.EditObj.layerUnits)
      {
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        for (let mut index: i32 =  0; index <= unitCounter; index += 1)
        {
          if (this.game.Data.UnitObj[index].Regime == this.game.Data.Turn & this.game.Data.UnitObj[index].PreDef == -1 & this.game.Data.UnitObj[index].Historical > -1 && this.game.Data.UnitObj[index].cycleOrder < 0L)
            this.game.Data.UnitObj[index].cycleOrder = Math.Abs(this.game.Data.UnitObj[index].cycleOrder);
        }
        windowReturnClass.AddCommand(4, 69);
        windowReturnClass.AddCommand(4, 67);
        windowReturnClass.AddCommand(4, 68);
        windowReturnClass.AddCommand(4, 9);
        this.DoRefresh();
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      let mut num1: i32 =  0;
      let mut num2: i32 =   Math.Round(Conversion.Int( this.OwnBitmap.Width /  (30 * (this.game.EditObj.Zoom + 2))));
      let mut num3: i32 =   Math.Round(Conversion.Int( (this.OwnBitmap.Height - num1) /  (24 * (this.game.EditObj.Zoom + 2))));
      let mut num4: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - this.game.CornerX + 2;
      let mut num5: i32 =   this.game.Data.RuleVar[839] != 1.0 ? this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - this.game.CornerY + 3 : this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 4 - this.game.CornerY + 3;
      if (this.game.EditObj.Zoom == 1)
      {
        num2 -= 3;
        num3 = num3;
      }
      if (this.game.EditObj.Zoom == -1)
      {
        num2 += 3;
        num3 = num3;
      }
      if (this.game.CornerX > cornerX1 && num2 > num4 & (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0 | this.game.Data.Round == 0))
      {
        flag = true;
        this.game.CornerX = cornerX1;
      }
      if ( this.game.Data.RuleVar[329] == 0.0 & this.game.Data.Round > 0)
      {
        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & 0 > this.game.CornerX)
          this.game.CornerX = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + this.game.CornerX + 1;
        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & this.game.CornerX > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
          this.game.CornerX -= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1;
      }
      if (this.game.CornerY > cornerY1 && num3 > num5)
      {
        flag = true;
        this.game.CornerY = cornerY1;
      }
      if (this.game.CornerX == cornerX1 & this.game.CornerY == cornerY1)
        flag = false;
      if (!flag)
        return windowReturnClass;
      if (nr == 39)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftLeft();
      if (nr == 37)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftRight();
      if (nr == 40)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftUp();
      if (nr == 38)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftDown();
      index5: i32;
      Coordinate coordinate = this.SubPartList[index5].ClickMap(this.game.FormRef.LastMouseX - this.SubPartX[0], this.game.FormRef.LastMouseY - this.SubPartY[0]);
      if (coordinate.onmap)
      {
        this.game.EditObj.MouseOverX = coordinate.x;
        this.game.EditObj.MouseOverY = coordinate.y;
        this.CheckMovePath(0);
      }
      windowReturnClass.SetFlag(true);
      if (this.game.EditObj.OrderType != 9)
      {
        if ( this.game.Data.RuleVar[839] == 0.0)
        {
          windowReturnClass.AddCommand(4, 18);
          windowReturnClass.AddCommand(4, 66);
        }
        else
        {
          windowReturnClass.AddCommand(4, 67);
          this.game.EditObj.PurelyOrderRedrawRefresh = true;
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
        }
      }
      return windowReturnClass;
    }

    pub fn DoRefresh()
    {
      if (this.game.Data.Round == 0)
        this.game.EditObj.se1_map_data3cache_set = false;
      else if (this.game.EditObj.se1_map_data3cache_set)
      {
        let mut stringListById1: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 143, 0, 0));
        let mut stringListById2: i32 =  DrawMod.TGame.HandyFunctionsObj.GetStringListByID(DrawMod.TGame.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
        let mut num1: i32 =  Math.Max(0, this.game.Data.StringListObj[stringListById1].GetHighestValue(0)) + 5;
        let mut regimeCounter: i32 =  this.game.Data.RegimeCounter;
        for (let mut index: i32 =  0; index <= regimeCounter; index += 1)
        {
          if (this.game.Data.RegimeObj[index].id > num1)
            num1 = this.game.Data.RegimeObj[index].id;
        }
        let mut num2: i32 =  Math.Max(0, this.game.Data.StringListObj[stringListById2].GetHighestValue(0)) + 5;
        if (this.game.CustomBitmapObj.cacheDipClear.GetUpperBound(0) < num1)
          this.game.EditObj.se1_map_data3cache_set = false;
        else if (this.game.CustomBitmapObj.cacheZoneRecon.GetUpperBound(1) < num2)
          this.game.EditObj.se1_map_data3cache_set = false;
      }
      this.game.EditObj.se1_map_data3cache_set = false;
      if (this.game.EditObj.OrderType == 26 | this.game.EditObj.AIMoving)
      {
        this.game.EditObj.TempCoordList = CoordList::new();
        if (!Information.IsNothing( this.SubPartList[0]))
          this.SubPartList[0].Paint();
      }
      else
      {
        ScreenClass screeny = this.game.FormRef.Screeny;
        System.Type type = typeof (HistoryWindowClass2);
         System.Type local =  type;
        if (screeny.WindowPresent( local))
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          if (!Information.IsNothing( this.SubPartList[0]))
            this.SubPartList[0].Paint();
        }
        else
        {
          if (this.game.EditObj.ShowAirRange)
          {
            if (this.game.EditObj.OrderType > 0)
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              if (!Information.IsNothing( this.SubPartList[0]))
                this.SubPartList[0].Paint();
            }
            if (this.game.Data.Turn > -1)
            {
              if (!this.game.Data.RegimeObj[this.game.Data.Turn].AI & this.game.EditObj.OrderType == 0 && !Information.IsNothing( this.SubPartList[0]) && !this.DrawLayersAndSuch())
                this.SubPartList[this.SubpartNr(this.MapId)].Paint();
            }
            else
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              if (!Information.IsNothing( this.SubPartList[0]))
                this.SubPartList[0].Paint();
            }
          }
          else if ( this.game.Data.RuleVar[701] >= 1.0 & this.game.EditObj.UnitSelected > -1)
          {
            if (this.game.EditObj.udsReturnFromPopup)
            {
              this.game.EditObj.udsReturnFromPopup = false;
              this.DrawLayersAndSuch();
              this.UdsClickUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, 0, true);
              if (!Information.IsNothing( this.SubPartList[0]))
                this.SubPartList[0].Paint();
            }
            else
            {
              this.game.EditObj.TempCoordList = CoordList::new();
              if (this.game.EditObj.ShowLISRange)
                this.DrawLayersAndSuch();
              if (!Information.IsNothing( this.SubPartList[0]))
                this.SubPartList[0].Paint();
              this.PaintCurrentBitmap(this.MapId);
            }
          }
          else if ( this.game.Data.RuleVar[701] >= 1.0 & this.game.EditObj.udsUnitOrderMode == 36)
          {
            if (this.game.EditObj.udsReturnFromPopup)
            {
              this.game.EditObj.udsReturnFromPopup = false;
              this.DrawLayersAndSuch();
              this.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, true);
              if (!Information.IsNothing( this.SubPartList[0]))
                this.SubPartList[0].Paint();
            }
            else
            {
              if (this.game.EditObj.ShowLISRange)
                this.DrawLayersAndSuch();
              if (!Information.IsNothing( this.SubPartList[0]))
                this.SubPartList[0].Paint();
            }
          }
          else
          {
            if ( this.game.Data.RuleVar[701] >= 1.0)
            {
              this.game.EditObj.OrderType = 0;
              this.game.HandyFunctionsObj.RedimTempValue(9999);
            }
            this.game.EditObj.TempCoordList = CoordList::new();
            if (this.game.EditObj.ShowLISRange)
              this.DrawLayersAndSuch();
            if (!Information.IsNothing( this.SubPartList[0]))
              this.SubPartList[0].Paint();
          }
          this.CheckMovePath(0);
        }
      }
      this.game.EditObj.udsReturnFromPopup = false;
    }

    pub object UdsClickUnit(tx: i32, ty: i32, tmap: i32, bool outOfWindowCall)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      bool flag1 = false;
      bool flag2 = false;
      bool flag3 = false;
      bool flag4 = false;
      if (this.game.EditObj.udsUnitOrderMode != 48)
        this.game.EditObj.tempGroupMoveCounter = -1;
      if (!outOfWindowCall && this.game.EditObj.ShowLISRange)
        this.DrawLayersAndSuch(true);
      if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
      {
        flag1 = true;
        if (!this.game.Data.UnitObj[this.game.EditObj.UnitSelected].DidAttack)
          flag2 = true;
        if (this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.UnitSelected) >= 10)
        {
          if (this.game.HandyFunctionsObj.GetMaxArtRange(this.game.EditObj.UnitSelected, 0) >= 1)
            flag3 = true;
          if (this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
            flag4 = true;
        }
      }
      this.lastclickleft = DateAndTime.Now;
      if (this.game.EditObj.udsUnitOrderMode == 0 | this.game.EditObj.udsUnitOrderMode == 54)
      {
        this.game.EditObj.OrderUnit = -1;
        if (Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          this.game.EditObj.TempCoordListLastMove = CoordList::new();
        if (this.game.EditObj.TempCoordListLastMove.counter > -1)
        {
          this.game.EditObj.TempCoordList = this.game.EditObj.TempCoordListLastMove.Clone();
          this.game.EditObj.TempCoordListLastMove = (CoordList) null;
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          if (tx > -1)
            this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
        }
        else
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
        }
        this.game.EditObj.OrderType = 0;
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 9);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 67);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 68);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 69);
        if (!outOfWindowCall)
          this.SubPartList[0].Paint();
      }
      else if (this.game.EditObj.udsUnitOrderMode == 1)
      {
        if (flag1)
        {
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.OrderType = 1;
          this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, attackoptions: true, ismove: true, dist1alwaysDirect: true);
          this.game.EditObj.mouseOverActive = false;
          CoordList coordList = this.game.EditObj.TempCoordList.Clone();
          this.game.EditObj.TempCoordList.RemoveCoord(0);
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          {
            if (this.game.EditObj.TempCoordListLastMove.counter == -1)
              this.game.EditObj.TempCoordList = CoordList::new();
            else
              this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempCoordListLastMove);
          }
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = coordList;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else if (this.game.EditObj.OldUnit > -1 & this.game.EditObj.OrderType > 0)
        {
          this.game.EditObj.OrderUnit = -1;
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          {
            this.game.EditObj.TempCoordList = this.game.EditObj.TempCoordListLastMove.Clone();
            if (this.game.EditObj.TempCoordList.counter > -1)
              this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          }
          else
          {
            this.game.EditObj.TempCoordList = CoordList::new();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
            this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          }
          this.game.EditObj.OrderType = 0;
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = (CoordList) null;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else
        {
          this.game.EditObj.OrderUnit = -1;
          this.game.EditObj.OrderType = 0;
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
        }
      }
      else if (this.game.EditObj.udsUnitOrderMode == 48)
      {
        if (flag1)
        {
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.OrderType = 48;
          this.game.EditObj.TempCoordList = this.game.HandyFunctionsObj.MakeMovePredictionGroup(this.game.EditObj.UnitSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, attackoptions: true, ismove: true);
          this.game.EditObj.mouseOverActive = false;
          CoordList coordList = this.game.EditObj.TempCoordList.Clone();
          if (this.game.EditObj.TempCoordList.counter > -1)
            this.game.EditObj.TempCoordList.RemoveCoord(0);
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
            this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempCoordListLastMove);
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = coordList;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else if (this.game.EditObj.OldUnit > -1 & this.game.EditObj.OrderType > 0)
        {
          this.game.EditObj.OrderUnit = -1;
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          {
            this.game.EditObj.TempCoordList = this.game.EditObj.TempCoordListLastMove.Clone();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          }
          else
          {
            this.game.EditObj.TempCoordList = CoordList::new();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
            this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          }
          this.game.EditObj.OrderType = 0;
          this.game.HandyFunctionsObj.RedimTempValue(9999);
          this.game.HandyFunctionsObj.RedimTempAttack(false);
          this.game.HandyFunctionsObj.RedimTempLosValue(0);
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = (CoordList) null;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else
        {
          this.game.EditObj.OrderUnit = -1;
          this.game.EditObj.OrderType = 0;
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
        }
      }
      else if (this.game.EditObj.udsUnitOrderMode == 36)
      {
        this.game.EditObj.OrderUnit = -1;
        this.game.EditObj.OrderType = 36;
        this.game.EditObj.udsOrderPossible = false;
        this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round(Conversion.Val( this.game.Data.RuleVar[702])));
        this.game.EditObj.mouseOverActive = true;
        CoordList coordList = this.game.EditObj.TempCoordList.Clone();
        this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
        if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempCoordListLastMove);
        if (this.game.EditObj.OrderData == -1)
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          coordList = CoordList::new();
        }
        this.game.EditObj.TempMovePathList = CoordList::new();
        this.game.EditObj.mouseOverActive = false;
        if (!outOfWindowCall)
          this.SubPartList[0].Paint();
        this.game.EditObj.TempCoordListLastMove = coordList;
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 9);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 67);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 68);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 69);
      }
      else if (this.game.EditObj.udsUnitOrderMode == 18)
      {
        if (flag2)
        {
          this.game.EditObj.OrderType = 18;
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          let mut unitWeight: i32 =  this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.UnitSelected);
          let mut counter: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].items.list.Counter;
          for (let mut index: i32 =  0; index <= counter; index += 1)
          {
            let mut integer: i32 =  Conversions.ToInteger(this.game.Data.StringListObj[this.game.HandyFunctionsObj.GetStringListByID( Math.Round( this.game.Data.RuleVar[404]))].GetData(0, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].items.list.Id[index], 3));
            unitWeight += integer * this.game.Data.UnitObj[this.game.EditObj.UnitSelected].items.list.Weight[index];
          }
          let mut sizeForAirBridge: i32 =  this.game.HandyFunctionsObj.GetHighestSizeForAirBridge(this.game.EditObj.UnitSelected);
          this.game.HandyFunctionsObj.MakeMovePredictionLIS(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, unitWeight, useAirBridge: true, weightSize: sizeForAirBridge);
          this.game.EditObj.TempCoordList = CoordList::new();
          let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
          for (let mut x: i32 =  0; x <= mapWidth; x += 1)
          {
            let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
            for (let mut y: i32 =  0; y <= mapHeight; y += 1)
            {
              if (this.game.EditObj.TempValue[0].Value[x, y] > 0)
              {
                if (this.game.Data.MapObj[0].HexObj[x, y].UnitCounter <= 14)
                  this.game.EditObj.TempCoordList.AddCoord(x, y, 0);
                else
                  this.game.EditObj.TempValue[0].Value[x, y] = 0;
              }
            }
          }
          CoordList coordList = this.game.EditObj.TempCoordList.Clone();
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
            this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempCoordListLastMove);
          this.game.EditObj.mouseOverActive = false;
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = coordList;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else if (this.game.EditObj.OldUnit > -1 & this.game.EditObj.OrderType > 0)
        {
          this.game.EditObj.OrderUnit = -1;
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          {
            this.game.EditObj.TempCoordList = this.game.EditObj.TempCoordListLastMove.Clone();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          }
          else
          {
            this.game.EditObj.TempCoordList = CoordList::new();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
            this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          }
          this.game.EditObj.OrderType = 0;
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = (CoordList) null;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else
        {
          this.game.EditObj.OrderUnit = -1;
          this.game.EditObj.OrderType = 0;
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
        }
      }
      else if (this.game.EditObj.udsUnitOrderMode == 14 | this.game.EditObj.udsUnitOrderMode == 33 | this.game.EditObj.udsUnitOrderMode == 55)
      {
        if (flag4)
        {
          this.game.EditObj.OrderType = this.game.EditObj.udsUnitOrderMode;
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.TempCoordList = CoordList::new();
          let mut x: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X;
          let mut y: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y;
          let mut num1: i32 =  this.game.HandyFunctionsObj.GetMaxAirRange(this.game.EditObj.UnitSelected);
          if (this.game.EditObj.udsUnitOrderMode == 55)
          {
            let mut num2: i32 =  Math.Min(this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.UnitSelected), this.game.HandyFunctionsObj.GetLowestAirRdn(this.game.EditObj.UnitSelected));
            if (num2 > 100)
              num2 = 100;
            num1 =  Math.Round(Math.Floor( (num1 * num2) / 100.0));
          }
          this.game.HandyFunctionsObj.RedimTempValue(9999);
          let mut minimumAirfieldLevel: i32 =  this.game.HandyFunctionsObj.SE1_GetUnitMinimumAirfieldLevel(this.game.EditObj.UnitSelected);
          this.game.HandyFunctionsObj.GetLowestAirAp(this.game.EditObj.UnitSelected);
          bool flag5 = true;
          if (minimumAirfieldLevel > 0)
          {
            if (this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].Location > -1)
            {
              if (this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y].Location].tempAirfieldLevel < minimumAirfieldLevel)
                flag5 = false;
            }
            else
              flag5 = false;
          }
          if (num1 < 1)
            flag5 = false;
          if (flag5)
          {
            let mut num3: i32 =  x - (num1 + 1);
            let mut num4: i32 =  x + (num1 + 1);
            let mut num5: i32 =  y - (num1 + 1);
            let mut num6: i32 =  y + (num1 + 1);
            if (num1 >= 999)
            {
              num3 = 0;
              num4 = this.game.Data.MapObj[0].MapWidth;
              num5 = 0;
              num6 = this.game.Data.MapObj[0].MapHeight;
            }
            let mut num7: i32 =  num3;
            let mut num8: i32 =  num4;
            for (let mut index1: i32 =  num7; index1 <= num8; index1 += 1)
            {
              let mut num9: i32 =  num5;
              let mut num10: i32 =  num6;
              for (let mut index2: i32 =  num9; index2 <= num10; index2 += 1)
              {
                let mut index3: i32 =  index1;
                let mut index4: i32 =  index2;
                if (index3 == 22 & index4 == 8)
                  index3 = index3;
                if (index3 > this.game.Data.MapObj[0].MapWidth)
                  index3 -= this.game.Data.MapObj[0].MapWidth + 1;
                if (index3 < 0)
                  index3 = this.game.Data.MapObj[0].MapWidth - (Math.Abs(index3) - 1);
                if (index3 >= 0 & index4 >= 0 & index3 <= this.game.Data.MapObj[0].MapWidth & index4 <= this.game.Data.MapObj[0].MapHeight && this.game.EditObj.udsUnitOrderMode == 33 | this.game.Data.MapObj[0].HexObj[index3, index4].Regime > -1 & this.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon > 0)
                {
                  bool flag6 = false;
                  if (this.game.EditObj.udsUnitOrderMode == 14 && this.game.EventRelatedObj.Helper_CanAttack(this.game.Data.Turn, this.game.Data.MapObj[0].HexObj[index3, index4].Regime, false) && this.game.Data.MapObj[0].HexObj[index3, index4].Regime != this.game.Data.Turn & this.game.Data.MapObj[0].HexObj[index3, index4].Regime > -1)
                    flag6 = true;
                  if (this.game.EditObj.udsUnitOrderMode == 55)
                    flag6 = minimumAirfieldLevel <= 0 || this.game.Data.MapObj[0].HexObj[index3, index4].Location > -1 && this.game.Data.LocObj[this.game.Data.MapObj[0].HexObj[index3, index4].Location].tempAirfieldLevel >= minimumAirfieldLevel;
                  if (this.game.EditObj.udsUnitOrderMode == 33)
                  {
                    if (this.game.EventRelatedObj.Helper_CanAttack(this.game.Data.Turn, this.game.Data.MapObj[0].HexObj[index3, index4].Regime, true, index3, index4))
                      flag6 = true;
                    else if (this.game.Data.MapObj[0].HexObj[index3, index4].Regime == this.game.Data.Turn & this.game.Data.MapObj[0].HexObj[index3, index4].UnitCounter == -1)
                      flag6 = true;
                    else if (this.game.Data.MapObj[0].HexObj[index3, index4].MaxRecon < 1)
                    {
                      flag6 = true;
                      if (this.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(this.game.Data.Turn) > -1 && !this.game.EventRelatedObj.Helper_CanAttack(this.game.Data.Turn, this.game.Data.MapObj[0].HexObj[index3, index4].get_LastReg(this.game.Data.Turn), true, index3, index4))
                        flag6 = false;
                    }
                  }
                  if (flag6)
                  {
                    let mut num11: i32 =  this.game.HandyFunctionsObj.Distance(x, y, 0, index3, index4, 0, num1 + 1);
                    if (num11 <= num1)
                    {
                      this.game.EditObj.TempValue[0].Value[index3, index4] = num11;
                      Coordinate tcoord;
                      tcoord.y = 0;
                      this.game.HandyFunctionsObj.MovementSpecialCoordSet(this.game.Data.Turn, index3, index4,  tcoord, this.game.EditObj.udsUnitOrderMode == 33);
                      this.game.EditObj.TempValueSpecial[0].Value[index3, index4] = tcoord.y;
                      this.game.EditObj.TempCoordList.AddCoord(index3, index4, 0);
                    }
                  }
                }
              }
            }
          }
          CoordList coordList = this.game.EditObj.TempCoordList.Clone();
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
            this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempCoordListLastMove);
          this.game.EditObj.mouseOverActive = false;
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = coordList;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else if (this.game.EditObj.OldUnit > -1 & this.game.EditObj.OrderType > 0)
        {
          this.game.EditObj.OrderUnit = -1;
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          {
            this.game.EditObj.TempCoordList = this.game.EditObj.TempCoordListLastMove.Clone();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          }
          else
          {
            this.game.EditObj.TempCoordList = CoordList::new();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
            this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          }
          this.game.EditObj.OrderType = 0;
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = (CoordList) null;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else
        {
          this.game.EditObj.OrderUnit = -1;
          this.game.EditObj.OrderType = 0;
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
        }
      }
      else if (this.game.EditObj.udsUnitOrderMode == 11)
      {
        if (flag3)
        {
          this.game.EditObj.OrderType = this.game.EditObj.udsUnitOrderMode;
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.TempCoordList = CoordList::new();
          let mut x: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X;
          let mut y: i32 =  this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y;
          let mut maxArtRange: i32 =  this.game.HandyFunctionsObj.GetMaxArtRange(this.game.EditObj.UnitSelected, 0);
          this.game.HandyFunctionsObj.RedimTempValue(9999);
          let mut num12: i32 =  x - (maxArtRange + 1);
          let mut num13: i32 =  x + (maxArtRange + 1);
          let mut num14: i32 =  y - (maxArtRange + 1);
          let mut num15: i32 =  y + (maxArtRange + 1);
          if (maxArtRange >= 999)
          {
            num12 = 0;
            num13 = this.game.Data.MapObj[0].MapWidth;
            num14 = 0;
            num15 = this.game.Data.MapObj[0].MapHeight;
          }
          let mut num16: i32 =  num12;
          let mut num17: i32 =  num13;
          for (let mut index5: i32 =  num16; index5 <= num17; index5 += 1)
          {
            let mut num18: i32 =  num14;
            let mut num19: i32 =  num15;
            for (let mut index6: i32 =  num18; index6 <= num19; index6 += 1)
            {
              let mut index7: i32 =  index5;
              let mut index8: i32 =  index6;
              if (index7 == 22 & index8 == 8)
                index7 = index7;
              if (index7 > this.game.Data.MapObj[0].MapWidth)
                index7 -= this.game.Data.MapObj[0].MapWidth + 1;
              if (index7 < 0)
                index7 = this.game.Data.MapObj[0].MapWidth - (Math.Abs(index7) - 1);
              if (index7 >= 0 & index8 >= 0 & index7 <= this.game.Data.MapObj[0].MapWidth & index8 <= this.game.Data.MapObj[0].MapHeight && this.game.Data.MapObj[0].HexObj[index7, index8].Regime > -1 & this.game.Data.MapObj[0].HexObj[index7, index8].MaxRecon > 0 && this.game.EventRelatedObj.Helper_CanAttack(this.game.Data.Turn, this.game.Data.MapObj[0].HexObj[index7, index8].Regime, false) && this.game.Data.MapObj[0].HexObj[index7, index8].Regime != this.game.Data.Turn & this.game.Data.MapObj[0].HexObj[index7, index8].Regime > -1)
              {
                let mut num20: i32 =  this.game.HandyFunctionsObj.Distance(x, y, 0, index7, index8, 0, maxArtRange + 1);
                if (num20 <= maxArtRange)
                {
                  this.game.EditObj.TempValue[0].Value[index7, index8] = num20;
                  Coordinate tcoord;
                  tcoord.y = 0;
                  this.game.HandyFunctionsObj.MovementSpecialCoordSet(this.game.Data.Turn, index7, index8,  tcoord, false);
                  this.game.EditObj.TempValueSpecial[0].Value[index7, index8] = tcoord.y;
                  this.game.EditObj.TempCoordList.AddCoord(index7, index8, 0);
                }
              }
            }
          }
          CoordList coordList = this.game.EditObj.TempCoordList.Clone();
          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
            this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempCoordListLastMove);
          this.game.EditObj.mouseOverActive = false;
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = coordList;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else if (this.game.EditObj.OldUnit > -1 & this.game.EditObj.OrderType > 0)
        {
          this.game.EditObj.OrderUnit = -1;
          if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
          {
            this.game.EditObj.TempCoordList = this.game.EditObj.TempCoordListLastMove.Clone();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          }
          else
          {
            this.game.EditObj.TempCoordList = CoordList::new();
            this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
            this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          }
          this.game.EditObj.OrderType = 0;
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
          this.game.EditObj.TempCoordListLastMove = (CoordList) null;
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
        }
        else
        {
          this.game.EditObj.OrderUnit = -1;
          this.game.EditObj.OrderType = 0;
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 9);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 67);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 68);
          if (!outOfWindowCall)
            windowReturnClass.AddCommand(4, 69);
          if (!outOfWindowCall)
            this.SubPartList[0].Paint();
        }
      }
      else
      {
        this.game.EditObj.OrderUnit = -1;
        if (!Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
        {
          this.game.EditObj.TempCoordList = this.game.EditObj.TempCoordListLastMove.Clone();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
        }
        else
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
          this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
        }
        this.game.EditObj.OrderType = 0;
        this.game.EditObj.TempCoordList = CoordList::new();
        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
        this.game.EditObj.TempCoordList.AddCoord(tx, ty, tmap);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 9);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 67);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 68);
        if (!outOfWindowCall)
          windowReturnClass.AddCommand(4, 69);
        if (!outOfWindowCall)
          this.SubPartList[0].Paint();
      }
      this.CheckMovePath(0);
      windowReturnClass.SetFlag(true);
      return  windowReturnClass;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult1 = OrderResult::new();
      if (this.game.EditObj.OrderType == 26 | this.game.EditObj.AIMoving)
        return windowReturnClass1;
      this.lastclickleft = DateAndTime.Now;
      this.game.EditObj.OldUnit = this.game.EditObj.UnitSelected;
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1] && this.SubPartID[index1] == this.MapId)
          {
            let mut index2: i32 =  this.game.SelectX;
            let mut selectY1: i32 =  this.game.SelectY;
            let mut mapSelected1: i32 =  this.game.EditObj.MapSelected;
            Coordinate coordinate1 = this.SubPartList[index1].ClickMap(x - this.SubPartX[index1], y - this.SubPartY[index1]);
            if (coordinate1.onmap)
            {
              if (this.game.EditObj.TutMode & this.game.EditObj.OrderType > 0 & this.game.EditObj.OrderType < 9999 && Operators.ConditionalCompareObjectGreater(this.game.EditObj.TutX,  -1, false) && Conversions.ToBoolean(Operators.NotObject(Operators.AndObject(Operators.CompareObjectEqual(this.game.EditObj.TutX,  coordinate1.x, false), Operators.CompareObjectEqual(this.game.EditObj.TutY,  coordinate1.y, false)))))
                return windowReturnClass1;
              this.game.SelectX = coordinate1.x;
              this.game.SelectY = coordinate1.y;
              this.game.EditObj.se1_SelectAssetButton = 0;
              if ( this.game.Data.RuleVar[701] > 0.0 && b == 2)
              {
                if (this.game.EditObj.UnitSelected > -1 & !this.game.FormRef.MouseClicked & this.game.FormRef.RightMousePressed && index2 == this.game.SelectX & selectY1 == this.game.SelectY & this.game.SelectX == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X & this.game.SelectY == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y && this.game.Data.MapObj[0].HexObj[index2, selectY1].Regime == this.game.Data.Turn)
                {
                  this.game.EditObj.MyDelegateMap = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                  windowReturnClass1.AddCommand(2, 118);
                  windowReturnClass1.alwaysExecuteWR = true;
                  return windowReturnClass1;
                }
                if (!this.game.FormRef.MouseClicked & this.game.FormRef.RightMousePressed && index2 == this.game.SelectX & selectY1 == this.game.SelectY & this.game.Data.MapObj[0].HexObj[index2, selectY1].Regime == this.game.Data.Turn)
                {
                  this.game.EditObj.MyDelegateMap = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                  windowReturnClass1.AddCommand(2, 118);
                  windowReturnClass1.alwaysExecuteWR = true;
                  return windowReturnClass1;
                }
                if (!this.game.FormRef.MouseClicked & this.game.FormRef.RightMousePressed && this.game.EditObj.UnitSelected == -1 && this.game.EditObj.udsUnitOrderMode == 1 | this.game.EditObj.udsUnitOrderMode == 0)
                {
                  this.game.SelectX = index2;
                  this.game.SelectY = selectY1;
                  this.game.EditObj.MyDelegateMap = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                  windowReturnClass1.AddCommand(2, 118);
                  windowReturnClass1.alwaysExecuteWR = true;
                  return windowReturnClass1;
                }
              }
              let mut num1: i32 =  !(index2 == this.game.SelectX & selectY1 == this.game.SelectY) ? this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, b, coordinate1.data1, coordinate1.penalty, isMainMap: true) : this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, false, b, coordinate1.data1, coordinate1.penalty, isMainMap: true);
              if (this.game.EditObj.UnitSelected != num1)
                this.game.EditObj.SFSelected = -1;
              this.game.EditObj.UnitSelected = num1;
              if (!this.game.EditObj.layerUnits)
                this.game.EditObj.UnitSelected = -1;
              bitmap: Bitmap;
              if (this.game.SelectX > -1 & this.game.SelectY > -1 & this.game.SelectX <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth & this.game.SelectY <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight)
              {
                let mut subPart: SubPartClass = this.SubPartList[this.SubpartNr(this.MapId)];
                let mut selectX: i32 =  this.game.SelectX;
                let mut selectY2: i32 =  this.game.SelectY;
                let mut map: i32 =  mapSelected1;
                let mut counterAlpha: i32 =  this.game.EditObj.CounterAlpha;
                bitmap = (Bitmap) null;
                 let mut local: &Bitmap = &bitmap;
                subPart.PaintCoordinate((Graphics) null, selectX, selectY2, map, counterAlpha,  local);
              }
              Coordinate Target;
              if ( this.game.Data.RuleVar[701] > 0.0)
              {
                if (b == 1)
                {
                  if (this.game.EditObj.udsUnitOrderMode == 54)
                  {
                    data: DataClass = DrawMod.TGame.Data;
                    str: String = "Zones".to_owned();
                     local: String =  str;
                    let mut libVar: i32 =  data.FindLibVar( local, "SE_Data");
                    let mut hexLibVarValue: i32 =  this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].GetHexLibVarValue(libVar);
                    let mut num2: i32 =  -1;
                    let mut index3: i32 =  -1;
                    bool flag = true;
                    if (hexLibVarValue > 0)
                    {
                      let mut stringListById: i32 =  this.game.HandyFunctionsObj.GetStringListByID(this.game.EventRelatedObj.CheckStringlistID("SE_Data", 123, 0, 0));
                      num2 =  Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].GetData(0, hexLibVarValue, 8)));
                      let mut id: i32 =   Math.Round(Conversion.Val(this.game.Data.StringListObj[stringListById].GetData(0, hexLibVarValue, 6)));
                      if (id > 0)
                        index3 = this.game.HandyFunctionsObj.GetLocationByID(id);
                    }
                    if (index3 > -1 && this.game.Data.LocObj[index3].X == this.game.SelectX & this.game.Data.LocObj[index3].Y == this.game.SelectY)
                      flag = false;
                    if (num2 == this.game.Data.RegimeObj[this.game.Data.Turn].id & flag && hexLibVarValue != this.game.EditObj.OrderSubType)
                    {
                      let mut num3: i32 =  0;
                      let mut num4: i32 =  0;
                      if (Information.IsNothing( this.game.EditObj.TempCoordListLastMove))
                        this.game.EditObj.TempCoordListLastMove = CoordList::new();
                      let mut tfacing: i32 =  1;
                      Coordinate coordinate2;
                      do
                      {
                        Coordinate coordinate3 = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
                        if (coordinate3.onmap)
                        {
                          this.game.EditObj.TempCoordListLastMove.AddCoord(coordinate3.x, coordinate3.y, coordinate3.map);
                          if (this.game.Data.MapObj[0].HexObj[coordinate3.x, coordinate3.y].GetHexLibVarValue(libVar) == this.game.EditObj.OrderSubType)
                          {
                            num3 += 1;
                            if (this.game.Data.MapObj[0].HexObj[coordinate3.x, coordinate3.y].Location > -1)
                              num3 += 1;
                            coordinate2 = coordinate3;
                          }
                          if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[0].HexObj[coordinate3.x, coordinate3.y].LandscapeType].IsSea)
                            num4 += 1;
                        }
                        tfacing += 1;
                      }
                      while (tfacing <= 6);
                      if (num4 >= 2 & num3 >= 1 | num3 >= 2)
                        this.game.HandyFunctionsObj.UnitCausesHexOwnershipChange(this.game.Data.Turn, this.game.SelectX, this.game.SelectY, coordinate2.x, coordinate2.y, true);
                    }
                  }
                  if (this.game.EditObj.udsUnitOrderMode == 53 & index2 == this.game.SelectX & selectY1 == this.game.SelectY)
                  {
                    this.game.EditObj.PopupValue = 28;
                    windowReturnClass1.AddCommand(5, 14);
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                }
                if (b == 1 && this.game.EditObj.OrderType != 26)
                {
                  this.DrawLayersAndSuch();
                  return (WindowReturnClass) this.UdsClickUnit(index2, selectY1, mapSelected1, false);
                }
                if (b == 2)
                {
                  if (this.game.EditObj.udsUnitOrderMode == 54)
                  {
                    data: DataClass = DrawMod.TGame.Data;
                    str: String = "Zones".to_owned();
                     local: String =  str;
                    this.game.EditObj.OrderSubType = this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].GetHexLibVarValue(data.FindLibVar( local, "SE_Data"));
                    this.DrawLayersAndSuch();
                    windowReturnClass2: WindowReturnClass = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, 0, false);
                    windowReturnClass2.AddCommand(4, 12);
                    windowReturnClass2.SetFlag(true);
                    return windowReturnClass2;
                  }
                  if (this.game.EditObj.UnitSelected > -1 & !this.game.FormRef.MouseClicked & this.game.FormRef.RightMousePressed && index2 == this.game.SelectX & selectY1 == this.game.SelectY & this.game.SelectX == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X & this.game.SelectY == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y)
                  {
                    windowReturnClass1.AddCommand(2, 118);
                    windowReturnClass1.alwaysExecuteWR = true;
                    return windowReturnClass1;
                  }
                  if (this.game.EditObj.OrderType == 55 | this.game.EditObj.OrderType == 33 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48 && this.game.EditObj.TempValueSpecial[0].Value[this.game.SelectX, this.game.SelectY] >= 2)
                  {
                    let mut eventByLib: i32 =  this.game.EventRelatedObj.CheckGetEventByLib("SE_Present", 506, 0, 0);
                    let mut areaX: i32 =  this.game.EditObj.AreaX;
                    let mut areaY: i32 =  this.game.EditObj.AreaY;
                    this.game.EditObj.AreaX = this.game.SelectX;
                    this.game.EditObj.AreaY = this.game.SelectY;
                    this.game.EditObj.UDSinputCounter = -1;
                    this.game.EditObj.udsLastCalledPopupEventNr = eventByLib;
                    this.game.EventRelatedObj.DoCheckSpecificEvent(eventByLib);
                    this.game.EditObj.AreaX = areaX;
                    this.game.EditObj.AreaY = areaY;
                    windowReturnClass1.SetFlag(true);
                    this.game.EditObj.PopupValue = 21;
                    windowReturnClass1.AddCommand(5, 14);
                    this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                    this.game.EditObj.OrderType = 0;
                    this.game.EditObj.TempCoordList = CoordList::new();
                    this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X;
                    this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y;
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  if (this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 33)
                  {
                    if (this.game.EditObj.TempValue[0].Value[this.game.SelectX, this.game.SelectY] < 999)
                    {
                      this.game.EditObj.TempUnitList = UnitList::new();
                      this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                      this.game.EditObj.OrderX = this.game.SelectX;
                      this.game.EditObj.OrderY = this.game.SelectY;
                      this.game.EditObj.OrderMap = this.game.EditObj.MapSelected;
                      this.game.EditObj.TargetX = this.game.SelectX;
                      this.game.EditObj.TargetY = this.game.SelectY;
                      this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                      this.game.EditObj.PopupValue = 22;
                      windowReturnClass1.AddCommand(5, 14);
                      this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, 0);
                    this.game.EditObj.TempCoordListLastMove.AddCoord(index2, selectY1, 0);
                    this.game.SelectX = index2;
                    this.game.SelectY = selectY1;
                    windowReturnClass1.SetFlag(true);
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    windowReturnClass3: WindowReturnClass = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                    windowReturnClass3.AddCommand(4, 12);
                    windowReturnClass3.AddCommand(4, 67);
                    windowReturnClass3.AddCommand(4, 68);
                    windowReturnClass3.AddCommand(4, 9);
                    return windowReturnClass3;
                  }
                  if (this.game.EditObj.OrderType == 55 & this.game.EditObj.TempValue[0].Value[this.game.SelectX, this.game.SelectY] < 999)
                  {
                    this.game.HandyFunctionsObj.AirBridgeInput(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.SelectX, this.game.SelectY);
                    this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, 0);
                    this.game.EditObj.TempCoordListLastMove.AddCoord(index2, selectY1, 0);
                    this.game.SelectX = index2;
                    this.game.SelectY = selectY1;
                    windowReturnClass1.SetFlag(true);
                    this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                    windowReturnClass4: WindowReturnClass = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                    windowReturnClass4.AddCommand(4, 12);
                    windowReturnClass4.AddCommand(4, 67);
                    windowReturnClass4.AddCommand(4, 68);
                    windowReturnClass4.AddCommand(4, 9);
                    return windowReturnClass4;
                  }
                  if (this.game.EditObj.OrderType == 18)
                  {
                    if (this.game.EditObj.TempValue[0].Value[this.game.SelectX, this.game.SelectY] >= this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.OrderUnit, includeLisWeight: true))
                    {
                      if (this.game.EditObj.SoundOn)
                        SoundMod.PlayAWave(this.game.AppPath + "sound/transfer.wav",  this.game.EditObj);
                      OrderResult orderResult2 = (OrderResult) this.game.ProcessingObj.LIS_DoStrategicTransfer(this.game.EditObj.OrderUnit, this.game.SelectX, this.game.SelectY, 0);
                      if (!this.game.HandyFunctionsObj.VisibleEnemyUnitsInHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.Data.Turn, true))
                      {
                        this += 1.game.Data.StepNr;
                        let mut orderUnit: i32 =  this.game.EditObj.OrderUnit;
                        if (this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.Data.UnitObj[orderUnit].Regime)
                        {
                          infostring: String = "Air Landing: " + this.game.Data.UnitObj[orderUnit].Name + " disembarks...";
                          this.game.HandyFunctionsObj.HistoryAddHex(this.game.SelectX, this.game.SelectY, 0, this.game.Data.UnitObj[orderUnit].Regime, infostring: infostring);
                          this.game.HandyFunctionsObj.UnitCausesHexOwnershipChange(this.game.Data.UnitObj[orderUnit].Regime, this.game.SelectX, this.game.SelectY, -1, -1);
                        }
                        if (this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Regime > -1)
                          this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].set_LastReg(this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Regime, this.game.Data.UnitObj[orderUnit].Regime);
                        this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].set_LastReg(this.game.Data.UnitObj[orderUnit].Regime, this.game.Data.UnitObj[orderUnit].Regime);
                        this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Regime = this.game.Data.UnitObj[orderUnit].Regime;
                        let mut tfacing: i32 =  1;
                        do
                        {
                          Coordinate coordinate4 = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
                          if (coordinate4.onmap)
                            this.game.EditObj.TempCoordListLastMove.AddCoord(coordinate4.x, coordinate4.y, 0);
                          tfacing += 1;
                        }
                        while (tfacing <= 6);
                        this.DrawLayersAndSuch();
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        windowReturnClass5: WindowReturnClass = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                        windowReturnClass5.SetFlag(true);
                        return windowReturnClass5;
                      }
                      this.game.EditObj.TargetX = this.game.SelectX;
                      this.game.EditObj.TargetY = this.game.SelectY;
                      this.game.EditObj.TargetMap = this.game.EditObj.MapSelected;
                      this.game.TempCombat = new CombatClass(this.game);
                      Target = Coordinate::new();
                      Target.x = this.game.EditObj.TargetX;
                      Target.y = this.game.EditObj.TargetY;
                      Target.map = this.game.EditObj.TargetMap;
                      this.game.EditObj.TempUnitList = UnitList::new();
                      this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                      if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                      {
                        this.game.EditObj.BattleTimerActive = true;
                        this.game.EditObj.BattleTimer = DateTime.Now + new TimeSpan(0, 0, 0, 0, 1000);
                      }
                      windowReturnClass1.SetFlag(true);
                    }
                    else
                    {
                      this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, 0);
                      this.game.EditObj.TempCoordListLastMove.AddCoord(index2, selectY1, 0);
                      this.game.SelectX = index2;
                      this.game.SelectY = selectY1;
                      windowReturnClass1.SetFlag(true);
                      this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      windowReturnClass6: WindowReturnClass = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                      windowReturnClass6.AddCommand(4, 12);
                      windowReturnClass6.AddCommand(4, 67);
                      windowReturnClass6.AddCommand(4, 68);
                      windowReturnClass6.AddCommand(4, 9);
                      return windowReturnClass6;
                    }
                  }
                  if (this.game.EditObj.OrderType == 36)
                  {
                    if (this.game.EditObj.mouseOverActive & this.game.EditObj.udsOrderPossible)
                    {
                      if (this.game.EditObj.SoundOn)
                        SoundMod.PlayAWave(this.game.AppPath + "sound/transfer.wav",  this.game.EditObj);
                      this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round(Conversion.Val( this.game.Data.RuleVar[704])));
                      this.DrawLayersAndSuch();
                      this.game.EditObj.UnitSelected = -1;
                      this.game.EditObj.OrderData = -1;
                      this.game.EditObj.udsOrderPossible = false;
                      this.game.EditObj.OrderX = -1;
                      windowReturnClass7: WindowReturnClass = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                      windowReturnClass7.SetFlag(true);
                      return windowReturnClass7;
                    }
                    this.game.SelectX = index2;
                    this.game.SelectY = selectY1;
                    return windowReturnClass1;
                  }
                  if (this.game.EditObj.OrderType == 1)
                  {
                    if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.UnitSelected > -1 & this.game.EditObj.OrderUnit > -1)
                    {
                      bool flag = false;
                      let mut x1: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X;
                      let mut y1: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y;
                      if (x1 > -1 && this.game.HandyFunctionsObj.Distance(x1, y1, 0, this.game.SelectX, this.game.SelectY, 0, 1) == 1)
                      {
                        let mut index4: i32 =  0;
                        do
                        {
                          if (this.game.EditObj.TempAttack[0].Value[this.game.SelectX, this.game.SelectY, index4])
                            flag = true;
                          index4 += 1;
                        }
                        while (index4 <= 5);
                        if (flag)
                        {
                          this.game.EditObj.PopupValue = 22;
                          this.game.EditObj.TempUnitList = UnitList::new();
                          this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                          windowReturnClass1.AddCommand(5, 14);
                          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                          windowReturnClass1.SetFlag(true);
                          this.game.EditObj.OrderType = 2;
                          return windowReturnClass1;
                        }
                      }
                    }
                    b = 1;
                  }
                  if (this.game.EditObj.OrderType == 48)
                  {
                    if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode & this.game.EditObj.UnitSelected > -1 & this.game.EditObj.OrderUnit > -1)
                    {
                      bool flag = false;
                      let mut x2: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X;
                      let mut y2: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y;
                      if (x2 > -1 && this.game.HandyFunctionsObj.Distance(x2, y2, 0, this.game.SelectX, this.game.SelectY, 0, 1) == 1)
                      {
                        let mut index5: i32 =  0;
                        do
                        {
                          if (this.game.EditObj.TempAttack[0].Value[this.game.SelectX, this.game.SelectY, index5])
                            flag = true;
                          index5 += 1;
                        }
                        while (index5 <= 5);
                        if (flag)
                        {
                          this.game.EditObj.PopupValue = 22;
                          this.game.EditObj.TempUnitList = UnitList::new();
                          this.game.EditObj.TempUnitList.add(this.game.EditObj.OrderUnit);
                          windowReturnClass1.AddCommand(5, 14);
                          this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                          windowReturnClass1.SetFlag(true);
                          this.game.EditObj.OrderType = 2;
                          return windowReturnClass1;
                        }
                      }
                    }
                    b = 1;
                  }
                }
              }
              if (b == 2)
              {
                if (this.game.Data.Round == 0)
                {
                  this.game.EditObj.RightClickX = this.game.SelectX;
                  this.game.EditObj.RightCLickY = this.game.SelectY;
                }
                if (this.game.Data.Round > 0 && this.game.EditObj.OrderType == 9)
                {
                  if (this.game.EditObj.UnitSelected > -1 && this.game.EditObj.OrderTarget != this.game.EditObj.UnitSelected && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.Round > 0 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                  {
                    this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                    windowReturnClass1.AddCommand(4, 30);
                  }
                  if ( this.game.Data.RuleVar[839] == 0.0)
                  {
                    windowReturnClass1.AddCommand(4, 44);
                    windowReturnClass1.AddCommand(4, 18);
                    windowReturnClass1.AddCommand(4, 20);
                    windowReturnClass1.AddCommand(4, 66);
                  }
                  else
                  {
                    windowReturnClass1.AddCommand(4, 68);
                    windowReturnClass1.AddCommand(4, 69);
                    windowReturnClass1.AddCommand(4, 67);
                    windowReturnClass1.AddCommand(4, 9);
                  }
                }
              }
              if (b == 1)
              {
                if (this.game.Data.Round == 0)
                {
                  if (this.game.EditObj.PencilType == 1)
                  {
                    if (this.game.EditObj.PencilMode == 0)
                    {
                      index2 = 6;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
                        index2 =  Interaction.MsgBox( "There is a location in this hex. Are you sure to change landscape?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest");
                      if (index2 == 6)
                      {
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType = this.game.EditObj.PencilData1;
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr = this.game.EditObj.PencilData2;
                        this.game.HandyFunctionsObj.RandomizeHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        if (this.game.Data.LandscapeTypeObj[this.game.EditObj.PencilData1].IsSea)
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime = -1;
                      }
                    }
                    else
                    {
                      this.landscapeFill(this.game.EditObj.PencilData1, this.game.EditObj.PencilData2);
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.DoRefresh();
                    }
                  }
                  else if (this.game.EditObj.PencilType == 10)
                  {
                    if (this.game.EditObj.PencilMode == 0)
                    {
                      index2 = 6;
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpecialType = this.game.EditObj.PencilData1;
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpecialSprite = this.game.EditObj.PencilData2;
                      this.DoRefresh();
                    }
                    else
                    {
                      this.specialFill(this.game.EditObj.PencilData1, this.game.EditObj.PencilData2);
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.DoRefresh();
                    }
                  }
                  else if (this.game.EditObj.PencilType == 9)
                  {
                    if (this.game.EditObj.PencilMode == 0)
                    {
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].AreaCode[this.game.EditObj.PencilData1] = this.game.EditObj.PencilData2;
                    }
                    else
                    {
                      this.areacodeFill(this.game.EditObj.PencilData1, this.game.EditObj.PencilData2);
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.DoRefresh();
                    }
                  }
                  else if (this.game.EditObj.PencilType == 11)
                  {
                    if (this.game.EditObj.PencilMode == 0)
                    {
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SetHexLibVarValue(this.game.EditObj.PencilData1, this.game.EditObj.PencilData2);
                    }
                    else
                    {
                      this.hexLibVarFill(this.game.EditObj.PencilData1, this.game.EditObj.PencilData2);
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.DoRefresh();
                    }
                  }
                  else if (this.game.EditObj.PencilType == 2)
                  {
                    let mut num5: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    let mut num6: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected);
                    if (num5 > -1)
                    {
                      let mut index6: i32 =  num5 - 1;
                      let mut index7: i32 =  num6 - 1;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RoadType[index6] == -1)
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RoadType[index6] = this.game.EditObj.PencilData1;
                      else
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RoadType[index6] = -1;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index7] == -1)
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index7] = this.game.EditObj.PencilData1;
                      else
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index7] = -1;
                      this.game.EditObj.RightClickX = this.game.SelectX;
                      this.game.EditObj.RightCLickY = this.game.SelectY;
                    }
                  }
                  else if (this.game.EditObj.PencilType == 3)
                  {
                    if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea)
                    {
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.EditObj.PencilData1)
                      {
                        if (this.game.EditObj.PencilMode == 0)
                        {
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime = -1;
                        }
                        else
                        {
                          this.regimeFill(-1);
                          this.game.EditObj.TempCoordList = CoordList::new();
                          this.DoRefresh();
                        }
                      }
                      else if (this.game.EditObj.PencilMode == 0)
                      {
                        if (this.game.EditObj.PencilData1 > this.game.Data.RegimeCounter)
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime = -1;
                        else
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime = this.game.EditObj.PencilData1;
                      }
                      else
                      {
                        if (this.game.EditObj.PencilData1 > this.game.Data.RegimeCounter)
                          this.regimeFill(-1);
                        else
                          this.regimeFill(this.game.EditObj.PencilData1);
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.DoRefresh();
                      }
                    }
                  }
                  else if (this.game.EditObj.PencilType == 4)
                  {
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location == -1)
                      Conversions.ToInteger(this.EditorPlaceLocation());
                  }
                  else if (this.game.EditObj.PencilType == 5)
                  {
                    let mut num7: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    let mut num8: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected);
                    if (num7 > -1)
                    {
                      let mut index8: i32 =  num7 - 1;
                      let mut index9: i32 =  num8 - 1;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RiverType[index8] == -1)
                      {
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RiverType[index8] = this.game.EditObj.PencilData1;
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RiverType[index9] = this.game.EditObj.PencilData1;
                      }
                      else
                      {
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RiverType[index9] = -1;
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RiverType[index8] = -1;
                      }
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RiverType[index9] == -1)
                        ;
                      this.game.EditObj.RightClickX = this.game.SelectX;
                      this.game.EditObj.RightCLickY = this.game.SelectY;
                    }
                  }
                  else if (this.game.EditObj.PencilType == 6)
                  {
                    let mut num9: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    let mut num10: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected);
                    if (num9 > -1)
                    {
                      let mut index10: i32 =  num9 - 1;
                      let mut index11: i32 =  num10 - 1;
                      if (!this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].Bridge[index10])
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].Bridge[index10] = true;
                      else
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].Bridge[index10] = false;
                      if (!this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Bridge[index11])
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Bridge[index11] = true;
                      else
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Bridge[index11] = false;
                      this.game.EditObj.RightClickX = this.game.SelectX;
                      this.game.EditObj.RightCLickY = this.game.SelectY;
                    }
                  }
                  if (this.game.EditObj.OrderType == 0)
                  {
                    windowReturnClass1.AddCommand(4, 18);
                    windowReturnClass1.AddCommand(4, 20);
                    windowReturnClass1.AddCommand(4, 44);
                  }
                  else
                  {
                    if (this.game.EditObj.OrderType == 1)
                    {
                      this.game.ProcessingObj.EditorMovement(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      this.game.EditObj.OrderType = 0;
                      this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 66);
                        windowReturnClass1.AddCommand(4, 44);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 9);
                        if (this.game.EditObj.inSimpleEditor)
                          windowReturnClass1.AddCommand(4, 99);
                      }
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (this.game.EditObj.OrderType == 3)
                    {
                      windowReturnClass1.AddCommand(4, 18);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 4)
                    {
                      windowReturnClass1.AddCommand(4, 18);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 5)
                    {
                      windowReturnClass1.AddCommand(4, 18);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 20)
                    {
                      windowReturnClass1.AddCommand(4, 18);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 21)
                    {
                      windowReturnClass1.AddCommand(4, 18);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 44);
                    }
                  }
                }
                if (this.game.Data.Round > 0)
                {
                  if (this.game.EditObj.LayerSupplyOn)
                  {
                    if (!(this.game.EditObj.RightClickX == this.game.SelectX & this.game.EditObj.RightCLickY == this.game.SelectY & this.game.EditObj.RightClickMap == this.game.EditObj.MapSelected))
                    {
                      this.game.EditObj.SupplyPath = CoordList::new();
                      this.game.EditObj.RightClickX = this.game.SelectX;
                      this.game.EditObj.RightCLickY = this.game.SelectY;
                      this.game.EditObj.RightClickMap = this.game.EditObj.MapSelected;
                      let mut x3: i32 =  this.game.EditObj.RightClickX;
                      let mut y3: i32 =  this.game.EditObj.RightCLickY;
                      map1: i32;
                      for (let mut map2: i32 =  this.game.EditObj.RightClickMap; this.game.EditObj.TempSupCameFrom[map2].Value[x3, y3].onmap; map2 = map1)
                      {
                        this.game.EditObj.SupplyPath.AddCoord(x3, y3, map2);
                        let mut x4: i32 =  this.game.EditObj.TempSupCameFrom[map2].Value[x3, y3].x;
                        let mut y4: i32 =  this.game.EditObj.TempSupCameFrom[map2].Value[x3, y3].y;
                        map1 = this.game.EditObj.TempSupCameFrom[map2].Value[x3, y3].map;
                        x3 = x4;
                        y3 = y4;
                      }
                    }
                    else
                    {
                      this.game.EditObj.RightClickX = -1;
                      this.game.EditObj.RightCLickY = -1;
                      this.game.EditObj.SupplyPath = (CoordList) null;
                    }
                    windowReturnClass1.AddCommand(4, 18);
                    windowReturnClass1.AddCommand(4, 20);
                    windowReturnClass1.AddCommand(4, 44);
                    windowReturnClass1.AddCommand(4, 12);
                  }
                  if (this.game.EditObj.OrderType == 0 | this.game.EditObj.OrderType == 26)
                  {
                    if ( this.game.Data.RuleVar[839] == 0.0)
                    {
                      windowReturnClass1.AddCommand(4, 18);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 44);
                    }
                    else
                    {
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 68);
                    }
                  }
                  else if (this.game.EditObj.OrderType == 43)
                  {
                    if (this.game.EditObj.TempValue[0].Value[this.game.SelectX, this.game.SelectY] == 0)
                    {
                      Form3::new( this.formref).Initialize(this.game.Data, 41, 0, 0, this.game);
                      this.game.EditObj.OrderType = 0;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 29);
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                  }
                  else
                  {
                    if (this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48)
                    {
                      SimpleList simpleList = SimpleList::new();
                      if (this.game.EditObj.OrderType == 48 | this.game.EditObj.OrderType == 1 && this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] >= 9999)
                      {
                        if ( this.game.Data.RuleVar[839] == 0.0)
                        {
                          this.game.EditObj.FeedBackString = "Cannot reach that hex";
                          windowReturnClass1.AddCommand(4, 29);
                          windowReturnClass1.AddCommand(4, 12);
                        }
                        else
                        {
                          windowReturnClass1.AddCommand(4, 12);
                          windowReturnClass1.AddCommand(4, 67);
                          windowReturnClass1.AddCommand(4, 68);
                          windowReturnClass1.AddCommand(4, 9);
                          if (this.game.EditObj.inSimpleEditor)
                            windowReturnClass1.AddCommand(4, 99);
                        }
                        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, 0);
                        this.game.EditObj.TempCoordListLastMove.AddCoord(this.game.SelectX, this.game.SelectY, 0);
                        this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X;
                        this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y;
                        windowReturnClass1.SetFlag(true);
                        return (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                      }
                      if (this.game.EditObj.OrderType == 1)
                        simpleList.Add(this.game.EditObj.OrderUnit, 1);
                      else if ( this.game.Data.RuleVar[954] == 1.0)
                      {
                        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                        for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
                        {
                          if (this.game.Data.UnitObj[tid].X > -1 & this.game.Data.UnitObj[tid].X == this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X & this.game.Data.UnitObj[tid].Y == this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y & this.game.Data.UnitObj[tid].Regime == this.game.Data.Turn & this.game.Data.UnitObj[tid].PreDef == -1)
                            simpleList.Add(tid, 1);
                        }
                      }
                      else
                      {
                        let mut historical: i32 =  this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Historical;
                        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                        for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
                        {
                          if (this.game.Data.UnitObj[tid].Historical == historical & this.game.Data.UnitObj[tid].Regime == this.game.Data.Turn & this.game.Data.UnitObj[tid].PreDef == -1)
                            simpleList.Add(tid, 1);
                        }
                      }
                      let mut counter: i32 =  simpleList.Counter;
                      for (let mut index12: i32 =  0; index12 <= counter; index12 += 1)
                      {
                        this.game.EditObj.OrderUnit = simpleList.Id[index12];
                        if (this.game.EditObj.OrderType == 48)
                          this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, attackoptions: true, ismove: true);
                        this.game.EditObj.OldUnit = -1;
                        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > 14 & (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn | this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == -1))
                        {
                          this.game.EditObj.FeedBackString = "Already max ammount of 16 units in that hex.";
                          if ( this.game.Data.RuleVar[839] == 0.0)
                          {
                            windowReturnClass1.AddCommand(4, 29);
                            windowReturnClass1.AddCommand(4, 66);
                          }
                          else
                          {
                            windowReturnClass1.AddCommand(4, 12);
                            windowReturnClass1.AddCommand(4, 68);
                            windowReturnClass1.AddCommand(4, 69);
                            windowReturnClass1.AddCommand(4, 67);
                            windowReturnClass1.AddCommand(4, 9);
                            if (this.game.EditObj.inSimpleEditor)
                              windowReturnClass1.AddCommand(4, 99);
                          }
                          this.game.SelectX = index2;
                          this.game.SelectY = selectY1;
                          this.DrawLayersAndSuch();
                          windowReturnClass8: WindowReturnClass = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                          windowReturnClass8.SetFlag(true);
                          return windowReturnClass8;
                        }
                        if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X == this.game.SelectX & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y == this.game.SelectY & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map == this.game.EditObj.MapSelected & this.game.EditObj.OrderType != 48)
                        {
                          this.game.EditObj.FeedBackString = "Unit is already in this hex.";
                          if ( this.game.Data.RuleVar[839] == 0.0)
                          {
                            windowReturnClass1.AddCommand(4, 29);
                            windowReturnClass1.AddCommand(4, 66);
                          }
                          else
                          {
                            windowReturnClass1.AddCommand(4, 12);
                            windowReturnClass1.AddCommand(4, 68);
                            windowReturnClass1.AddCommand(4, 69);
                            windowReturnClass1.AddCommand(4, 67);
                            windowReturnClass1.AddCommand(4, 9);
                            if (this.game.EditObj.inSimpleEditor)
                              windowReturnClass1.AddCommand(4, 99);
                          }
                          this.game.SelectX = index2;
                          this.game.SelectY = selectY1;
                          if (this.game.EditObj.OrderType == 1)
                            return windowReturnClass1;
                        }
                        else if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] < 9999)
                        {
                          this.game.EditObj.DoCardSlot = -1;
                          this.game.EditObj.AreaX = -1;
                          this.game.EditObj.AreaY = -1;
                          this.game.EditObj.AreaMap = -1;
                          OrderResult orderResult3 = this.game.ProcessingObj.ExecuteMovement(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                          let mut lowestSpeed1: i32 =  this.game.HandyFunctionsObj.GetLowestSpeed(this.game.EditObj.OrderUnit, -1);
                          let mut lowestSpeed2: i32 =  this.game.HandyFunctionsObj.GetLowestSpeed(this.game.EditObj.OrderUnit, -1, true);
                          if (lowestSpeed1 > -1 & index12 == 0)
                          {
                            if (this.game.Data.SFObj[lowestSpeed2].MoveType == -1)
                            {
                              if (Strings.Len(this.game.Data.SFTypeObj[lowestSpeed1].MoveWAV) > 0)
                                SoundMod.PlayAWave(this.game.AppPath + "sound/" + this.game.Data.SFTypeObj[lowestSpeed1].MoveWAV,  this.game.EditObj);
                            }
                            else if (Strings.Len(this.game.Data.TempString[900 + this.game.Data.SFObj[lowestSpeed2].MoveType]) > 0)
                              SoundMod.PlayAWave(this.game.AppPath + "sound/" + this.game.Data.TempString[900 + this.game.Data.SFObj[lowestSpeed2].MoveType],  this.game.EditObj);
                          }
                          if (orderResult3.BattleUnit == -1)
                          {
                            if (this.game.EditObj.OrderType != 48)
                              this.game.EditObj.OrderType = 0;
                            this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                            if ( this.game.Data.RuleVar[701] > 0.0 & this.game.EditObj.useLeftRightClickMode)
                            {
                              if (orderResult3.OK)
                                this.game.EditObj.TempCoordListLastMove.AddList( orderResult3.CList);
                            }
                            else if (orderResult3.OK)
                              this.game.EditObj.TempCoordList = orderResult3.CList;
                            if ( this.game.Data.RuleVar[839] == 0.0)
                            {
                              windowReturnClass1.AddCommand(4, 44);
                              windowReturnClass1.AddCommand(4, 20);
                              windowReturnClass1.AddCommand(4, 18);
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 29);
                              windowReturnClass1.AddCommand(4, 66);
                            }
                            else
                            {
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 69);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 9);
                              if (this.game.EditObj.inSimpleEditor)
                                windowReturnClass1.AddCommand(4, 99);
                            }
                            this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X;
                            this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y;
                            windowReturnClass1 = (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                            windowReturnClass1.SetFlag(true);
                            if (this.game.EditObj.LayerSupplyOn)
                            {
                              this.game.EditObj.TempCoordList = CoordList::new();
                              this.game.HandyFunctionsObj.MakeSupplyLayer(this.game.EditObj.OrderUnit);
                            }
                            if (this.game.EditObj.DoCardSlot > -1)
                            {
                              if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot > -1)
                              {
                                this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
                                this.game.EditObj.AreaSlot = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaSlot;
                                this.game.EditObj.AreaValue = this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].AreaValue;
                                this.game.EditObj.PopupValue = 1;
                                windowReturnClass1.AddCommand(5, 14);
                                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                windowReturnClass1.SetFlag(true);
                                this.game.EditObj.OrderType = 0;
                                return windowReturnClass1;
                              }
                              if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].UnitSelect)
                              {
                                this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
                                this.game.EditObj.PopupValue = 3;
                                windowReturnClass1.AddCommand(5, 14);
                                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                windowReturnClass1.SetFlag(true);
                                this.game.EditObj.OrderType = 0;
                                return windowReturnClass1;
                              }
                              let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                              this.game.ProcessingObj.PlayCard(this.game.EditObj.DoCardSlot);
                              if (Strings.Len(this.game.Data.LoadGame) > 0)
                              {
                                this.game.FormRef.Cursor = Cursors.WaitCursor;
                                Form formRef =  this.game.FormRef;
                                this.game.HandyFunctionsObj.LoadGameNow();
                                this.game.FormRef = (Form1) formRef;
                                this.game.FormRef.Cursor = Cursors.Default;
                                windowReturnClass1.AddCommand(3, 13);
                                this.game.EditObj.OrderType = 0;
                                return windowReturnClass1;
                              }
                              let mut num11: i32 =  0;
                              let mut locCounter: i32 =  this.game.Data.LocCounter;
                              for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
                              {
                                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
                                {
                                  let mut index13: i32 =  0;
                                  do
                                  {
                                    if (this.game.Data.LocObj[locnr].Production[index13] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index13]).result)
                                    {
                                      num11 += 1;
                                      this.game.Data.LocObj[locnr].Production[index13] = -1;
                                      this.game.Data.LocObj[locnr].ProdPointRemainder[index13] = 0;
                                      this.game.Data.LocObj[locnr].ProdPercent[index13] = 0;
                                    }
                                    index13 += 1;
                                  }
                                  while (index13 <= 3);
                                }
                              }
                              if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
                              {
                                this.game.EditObj.PopupValue = 0;
                                this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                                windowReturnClass1.AddCommand(5, 14);
                                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                windowReturnClass1.SetFlag(true);
                                return windowReturnClass1;
                              }
                            }
                          }
                          else
                          {
                            this.game.TempCombat = new CombatClass(this.game);
                            Target = Coordinate::new();
                            Target.x = orderResult3.BattleX;
                            Target.y = orderResult3.BattleY;
                            Target.map = orderResult3.BattleMap;
                            this.game.EditObj.TempUnitList = UnitList::new();
                            this.game.EditObj.TempUnitList.add(orderResult3.BattleUnit);
                            this.game.EditObj.DoCardSlot = -1;
                            if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                            {
                              if ( this.game.Data.RuleVar[839] == 1.0)
                              {
                                this.game.EditObj.PopupValue = 7;
                                windowReturnClass1.AddCommand(5, 14);
                                this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                windowReturnClass1.SetFlag(true);
                                return windowReturnClass1;
                              }
                              windowReturnClass1.AddCommand(3, 5);
                              windowReturnClass1.SetFlag(true);
                              return windowReturnClass1;
                            }
                            if ( this.game.Data.RuleVar[839] == 0.0)
                            {
                              windowReturnClass1.AddCommand(4, 44);
                              windowReturnClass1.AddCommand(4, 20);
                              windowReturnClass1.AddCommand(4, 18);
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 29);
                              windowReturnClass1.AddCommand(4, 66);
                            }
                            else
                            {
                              windowReturnClass1.AddCommand(4, 68);
                              windowReturnClass1.AddCommand(4, 69);
                              windowReturnClass1.AddCommand(4, 12);
                              windowReturnClass1.AddCommand(4, 67);
                              windowReturnClass1.AddCommand(4, 9);
                            }
                            windowReturnClass1.SetFlag(true);
                            return windowReturnClass1;
                          }
                        }
                        else
                        {
                          if ( this.game.Data.RuleVar[839] == 0.0)
                          {
                            this.game.EditObj.FeedBackString = "Cannot reach that hex";
                            windowReturnClass1.AddCommand(4, 29);
                            windowReturnClass1.AddCommand(4, 12);
                          }
                          else
                          {
                            windowReturnClass1.AddCommand(4, 12);
                            windowReturnClass1.AddCommand(4, 67);
                            windowReturnClass1.AddCommand(4, 68);
                            windowReturnClass1.AddCommand(4, 9);
                            if (this.game.EditObj.inSimpleEditor)
                              windowReturnClass1.AddCommand(4, 99);
                          }
                          this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                          this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X;
                          this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y;
                          windowReturnClass1.SetFlag(true);
                          return (WindowReturnClass) this.UdsClickUnit(this.game.SelectX, this.game.SelectY, mapSelected1, false);
                        }
                      }
                      if ( this.game.Data.RuleVar[701] < 1.0 | !this.game.EditObj.useLeftRightClickMode)
                        this.game.EditObj.OrderType = 0;
                      return windowReturnClass1;
                    }
                    if (this.game.EditObj.OrderType == 25)
                    {
                      if (this.game.EditObj.OrderSubType == 1 && this.game.HandyFunctionsObj.CanAddRoadToHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.OrderUnit, this.game.EditObj.TempCoordList))
                      {
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        windowReturnClass1.AddCommand(4, 40);
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      if (this.game.EditObj.OrderSubType == 2)
                        windowReturnClass1.AddCommand(4, 40);
                      if (this.game.EditObj.OrderSubType == 3)
                        windowReturnClass1.AddCommand(4, 40);
                      if (this.game.EditObj.OrderSubType == 4)
                        windowReturnClass1.AddCommand(4, 40);
                      if (this.game.EditObj.OrderSubType == 5)
                        windowReturnClass1.AddCommand(4, 40);
                    }
                    else if (this.game.EditObj.OrderType == 2)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 12)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 33 | this.game.EditObj.OrderType == 55)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 13)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 35)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 36)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 40)
                    {
                      if (this.game.EditObj.UnitSelected > -1)
                      {
                        this.game.HandyFunctionsObj.AirSupplyNeeded(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, 0);
                        let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
                        for (let mut index14: i32 =  0; index14 <= mapWidth; index14 += 1)
                        {
                          let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
                          for (let mut index15: i32 =  0; index15 <= mapHeight; index15 += 1)
                            this.game.EditObj.TempValue[0].Value[index14, index15] = this.game.EditObj.TempValue2[0].Value[index14, index15];
                        }
                      }
                      else
                      {
                        this.game.EditObj.TempCoordList = CoordList::new();
                        this.game.HandyFunctionsObj.RedimTempValue3(9999);
                      }
                      windowReturnClass1.AddCommand(4, 12);
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                    }
                    else if (this.game.EditObj.OrderType == 15)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 3)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 9)
                    {
                      if (this.game.EditObj.UnitSelected > -1 && this.game.EditObj.OrderTarget != this.game.EditObj.UnitSelected && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.Round > 0 & (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn |  this.game.Data.RuleVar[528] == 1.0 & this.game.Data.RegimeObj[this.game.Data.Turn].RegimeRel[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime] == 2))
                      {
                        this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                        windowReturnClass1.AddCommand(4, 30);
                      }
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 69);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 45)
                    {
                      if (this.game.EditObj.UnitSelected > -1)
                      {
                        if (this.game.Data.Round > 0 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                        {
                          this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                          windowReturnClass1.AddCommand(4, 87);
                        }
                      }
                      else
                      {
                        this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                        windowReturnClass1.AddCommand(4, 87);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 6)
                    {
                      if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && !this.game.EditObj.InsideSlotty && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn | this.game.Data.Round == 0 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].MaxProd > 0)
                      {
                        this.game.EditObj.OrderLoc = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 25);
                        if (!this.game.EditObj.ProdFlap)
                          windowReturnClass1.AddCommand(4, 18);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 45)
                    {
                      if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                      {
                        this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 61);
                        windowReturnClass1.AddCommand(4, 18);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 18 | this.game.EditObj.OrderType == 49)
                    {
                      if (this.game.EditObj.OrderUnit > -1 & this.game.EditObj.OrderTarget > -1)
                      {
                        this.game.EditObj.TargetX = this.game.SelectX;
                        this.game.EditObj.TargetY = this.game.SelectY;
                        windowReturnClass1.AddCommand(4, 35);
                      }
                      else
                      {
                        windowReturnClass1.AddCommand(4, 44);
                        windowReturnClass1.AddCommand(4, 20);
                        windowReturnClass1.AddCommand(4, 18);
                        windowReturnClass1.AddCommand(4, 66);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 7 | this.game.EditObj.OrderType == 44)
                    {
                      if (this.game.HandyFunctionsObj.IsAlliedOrSelf(this.game.Data.Turn, this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].Regime))
                      {
                        this.game.EditObj.OrderX = this.game.SelectX;
                        this.game.EditObj.OrderY = this.game.SelectY;
                        if (-(this.game.EditObj.OrderType == this.game.EditObj.OrderType ? 1 : 0) == 7)
                          windowReturnClass1.AddCommand(4, 88);
                        if (-(this.game.EditObj.OrderType == this.game.EditObj.OrderType ? 1 : 0) == 44)
                          windowReturnClass1.AddCommand(4, 89);
                      }
                      else
                      {
                        this.game.EditObj.OrderType = 0;
                        windowReturnClass1.AddCommand(4, 12);
                        windowReturnClass1.AddCommand(1, 5);
                        windowReturnClass1.AddCommand(4, 67);
                        windowReturnClass1.AddCommand(2, 69);
                        windowReturnClass1.AddCommand(4, 68);
                        windowReturnClass1.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 34)
                    {
                      windowReturnClass1.AddCommand(4, 44);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 66);
                    }
                    else if (this.game.EditObj.OrderType == 19 | this.game.EditObj.OrderType == 42)
                    {
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                    }
                    else if (this.game.EditObj.OrderType == 20)
                    {
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                    }
                    else if (this.game.EditObj.OrderType == 21)
                    {
                      windowReturnClass1.AddCommand(4, 68);
                      windowReturnClass1.AddCommand(4, 69);
                      windowReturnClass1.AddCommand(4, 67);
                      windowReturnClass1.AddCommand(4, 9);
                    }
                    else if (this.game.EditObj.OrderType == 4)
                    {
                      windowReturnClass1.AddCommand(4, 44);
                      windowReturnClass1.AddCommand(4, 20);
                      windowReturnClass1.AddCommand(4, 18);
                      windowReturnClass1.AddCommand(4, 66);
                    }
                    else if (this.game.EditObj.OrderType == 5)
                    {
                      windowReturnClass1.AddCommand(4, 44);
                      windowReturnClass1.AddCommand(4, 20);
                    }
                  }
                }
              }
              else if (this.game.EditObj.LayerSupplyOn)
              {
                if (!(this.game.EditObj.RightClickX == this.game.SelectX & this.game.EditObj.RightCLickY == this.game.SelectY & this.game.EditObj.RightClickMap == this.game.EditObj.MapSelected))
                {
                  this.game.EditObj.SupplyPath = CoordList::new();
                  this.game.EditObj.RightClickX = this.game.SelectX;
                  this.game.EditObj.RightCLickY = this.game.SelectY;
                  this.game.EditObj.RightClickMap = this.game.EditObj.MapSelected;
                  let mut x5: i32 =  this.game.EditObj.RightClickX;
                  let mut y5: i32 =  this.game.EditObj.RightCLickY;
                  map3: i32;
                  for (let mut map4: i32 =  this.game.EditObj.RightClickMap; this.game.EditObj.TempSupCameFrom[map4].Value[x5, y5].onmap; map4 = map3)
                  {
                    this.game.EditObj.SupplyPath.AddCoord(x5, y5, map4);
                    let mut x6: i32 =  this.game.EditObj.TempSupCameFrom[map4].Value[x5, y5].x;
                    let mut y6: i32 =  this.game.EditObj.TempSupCameFrom[map4].Value[x5, y5].y;
                    map3 = this.game.EditObj.TempSupCameFrom[map4].Value[x5, y5].map;
                    x5 = x6;
                    y5 = y6;
                  }
                }
                else
                {
                  this.game.EditObj.RightClickX = -1;
                  this.game.EditObj.RightCLickY = -1;
                  this.game.EditObj.SupplyPath = (CoordList) null;
                }
                windowReturnClass1.AddCommand(4, 18);
                windowReturnClass1.AddCommand(4, 20);
                windowReturnClass1.AddCommand(4, 44);
                windowReturnClass1.AddCommand(4, 12);
              }
              else if (this.game.EditObj.OrderType == 0 | this.game.EditObj.OrderType == 26 &&  this.game.Data.RuleVar[839] == 0.0)
              {
                windowReturnClass1.AddCommand(4, 18);
                windowReturnClass1.AddCommand(4, 20);
                windowReturnClass1.AddCommand(4, 44);
              }
              if (!this.game.Data.PermanentOverlayUse)
              {
                this.DrawLayersAndSuch();
                let mut subPart1: SubPartClass = this.SubPartList[index1];
                let mut x7: i32 =  index2;
                let mut y7: i32 =  selectY1;
                let mut map5: i32 =  mapSelected1;
                bitmap = (Bitmap) null;
                 let mut local1: &Bitmap = &bitmap;
                subPart1.PaintCoordinate((Graphics) null, x7, y7, map5, gBitmap: ( local1));
                let mut subPart2: SubPartClass = this.SubPartList[index1];
                let mut selectX: i32 =  this.game.SelectX;
                let mut selectY3: i32 =  this.game.SelectY;
                let mut mapSelected2: i32 =  this.game.EditObj.MapSelected;
                bitmap = (Bitmap) null;
                 let mut local2: &Bitmap = &bitmap;
                subPart2.PaintCoordinate((Graphics) null, selectX, selectY3, mapSelected2, gBitmap: ( local2));
                if (this.game.Data.Round == 0 & this.game.EditObj.PencilType == 1)
                {
                  let mut tfacing: i32 =  1;
                  do
                  {
                    Coordinate coordinate5 = this.game.HandyFunctionsObj.HexNeighbourSameMap(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, tfacing);
                    if (coordinate5.onmap)
                    {
                      let mut subPart3: SubPartClass = this.SubPartList[index1];
                      let mut x8: i32 =  coordinate5.x;
                      let mut y8: i32 =  coordinate5.y;
                      let mut map6: i32 =  coordinate5.map;
                      bitmap = (Bitmap) null;
                       let mut local3: &Bitmap = &bitmap;
                      subPart3.PaintCoordinate((Graphics) null, x8, y8, map6, gBitmap: ( local3));
                    }
                    tfacing += 1;
                  }
                  while (tfacing <= 6);
                }
              }
              else
              {
                this.game.EditObj.TempCoordList = CoordList::new();
                this.SubPartList[index1].Paint();
              }
              windowReturnClass1.SetFlag(true);
              if ( this.game.Data.RuleVar[839] == 0.0)
              {
                windowReturnClass1.AddCommand(4, 66);
              }
              else
              {
                windowReturnClass1.AddCommand(4, 67);
                windowReturnClass1.AddCommand(4, 9);
              }
              return windowReturnClass1;
            }
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub DrawLayersAndSuch: bool(bool noClearMap = false)
    {
      let mut num1: i32 =  0;
      if (this.game.EditObj.OrderType == 26 | this.game.EditObj.AIMoving)
        return false;
      bool flag1;
      if (this.game.EditObj.ShowAirRange & this.game.EditObj.OrderType == 0)
      {
        if (this.game.EditObj.OldUnit > -1 && this.game.EditObj.OldUnit != this.game.EditObj.UnitSelected)
        {
          if (this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.OldUnit))
          {
            if (this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.OldUnit].Regime)
            {
              this.game.HandyFunctionsObj.RedimTempValue(9999);
              flag1 = true;
            }
          }
          else if (this.game.HandyFunctionsObj.HasUnitArtSF(this.game.EditObj.OldUnit, this.game.Data))
          {
            this.game.HandyFunctionsObj.RedimTempValue(9999);
            flag1 = true;
          }
        }
        if (this.game.EditObj.UnitSelected > -1 & this.game.EditObj.OldUnit != this.game.EditObj.UnitSelected && this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime)
        {
          if (this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.UnitSelected))
          {
            let mut increaseap: i32 =   Math.Round( (Conversion.Int( this.game.HandyFunctionsObj.GetLowestAirRdn(this.game.EditObj.UnitSelected, true) * this.game.Data.RuleVar[147]) -  this.game.HandyFunctionsObj.GetLowestAirRdn(this.game.EditObj.UnitSelected, true)));
            this.game.HandyFunctionsObj.RedimTempValue(9999);
            if (this.game.HandyFunctionsObj.GetLowestAirRdn(this.game.EditObj.UnitSelected, true) - increaseap > 0)
            {
              this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.UnitSelected, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y, 0, false, PredictAirOnly: true, attack: true, increaseap: increaseap, OnlyFrontline: true);
              flag1 = true;
            }
          }
          else if (this.game.HandyFunctionsObj.HasUnitArtSF(this.game.EditObj.UnitSelected, this.game.Data))
          {
            let mut num2: i32 =  this.game.HandyFunctionsObj.GetUnitBestArtRange(this.game.EditObj.UnitSelected, this.game.Data) + 1;
            this.game.HandyFunctionsObj.RedimTempValue(9999);
            let mut num3: i32 =  Math.Max(0, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X - num2);
            num4: i32;
            let mut num5: i32 =  Math.Min(this.game.Data.MapObj[0].MapWidth, num4 + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X + num2);
            for (let mut index1: i32 =  num3; index1 <= num5; index1 += 1)
            {
              let mut num6: i32 =  Math.Max(0, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y - num2);
              let mut num7: i32 =  Math.Min(this.game.Data.MapObj[0].MapHeight, index1 + this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y + num2);
              for (let mut index2: i32 =  num6; index2 <= num7; index2 += 1)
              {
                if (this.game.HandyFunctionsObj.CanDoArtAttack(this.game.EditObj.UnitSelected, Coordinate::new()
                {
                  onmap = true,
                  x = index1,
                  y = index2
                }, false))
                  this.game.EditObj.TempValue[0].Value[index1, index2] = 0;
              }
            }
            flag1 = true;
          }
        }
      }
      if (this.game.EditObj.ShowLISRange &  this.game.Data.RuleVar[403] > 0.0)
      {
        bool flag2 = false;
        if (!Information.IsNothing( this.game.EditObj.TempSup[0]))
        {
          let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
          for (let mut index3: i32 =  0; index3 <= mapWidth; index3 += 1)
          {
            let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
            for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
            {
              if (this.game.Data.MapObj[0].HexObj[index3, index4].Regime == this.game.Data.Turn && this.game.EditObj.TempSup[0].Value[index3, index4] >= 9999)
                flag2 = true;
            }
          }
        }
        else
          flag2 = true;
        if (flag2)
        {
          this.game.HandyFunctionsObj.RedimTempSup(9999);
          if (Information.IsNothing( this.game.DC2AIObj))
            this.game.DC2AIObj = new DC2AIClass(this.game);
          let mut supplyMaximumRange: i32 =  this.game.DC2AIObj.VAR_SUPPLY_MAXIMUM_RANGE;
          this.game.DC2AIObj.VAR_SUPPLY_MAXIMUM_RANGE = 100;
          AIMatrix aiMatrix = new AIMatrix( this.game.DC2AIObj);
          AIMatrix ownerMatrix = new AIMatrix( this.game.DC2AIObj);
          let mut mapWidth1: i32 =  this.game.Data.MapObj[0].MapWidth;
          for (let mut x: i32 =  0; x <= mapWidth1; x += 1)
          {
            let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
            for (let mut y: i32 =  0; y <= mapHeight; y += 1)
            {
              aiMatrix.Value[x, y] = 9999;
              ownerMatrix.Value[x, y] = 2;
              if (this.game.Data.MapObj[0].HexObj[x, y].Regime == this.game.Data.Turn)
              {
                aiMatrix.Value[x, y] = 9998;
                ownerMatrix.Value[x, y] = 1;
                if (this.game.HandyFunctionsObj.HasHexRoad(x, y, 0))
                {
                  if (this.game.EditObj.layerLisPreview)
                  {
                    if (this.game.Data.MapObj[0].HexObj[x, y].tempPreviewLIS[6] > 0)
                      aiMatrix.Value[x, y] = 0;
                  }
                  else if (this.game.Data.MapObj[0].HexObj[x, y].LIStotalHistory[6] > 0)
                    aiMatrix.Value[x, y] = 0;
                }
              }
            }
          }
          if (this.game.DC2AIObj.TempHexNeighbour.GetUpperBound(0) < this.game.Data.MapObj[0].MapWidth)
            this.game.DC2AIObj.SetTempHexNeighbours();
          aiMatrix.ExpandAsSimplifiedSupplyMatrix(10,  ownerMatrix, 1, (AICoordinateMatrix) null);
          let mut mapWidth2: i32 =  this.game.Data.MapObj[0].MapWidth;
          for (let mut index5: i32 =  0; index5 <= mapWidth2; index5 += 1)
          {
            let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
            for (let mut index6: i32 =  0; index6 <= mapHeight; index6 += 1)
              this.game.EditObj.TempSup[0].Value[index5, index6] = aiMatrix.Value[index5, index6];
          }
          this.game.DC2AIObj.VAR_SUPPLY_MAXIMUM_RANGE = supplyMaximumRange;
          flag1 = true;
        }
      }
      if (this.game.EditObj.ShowHQPower && this.game.EditObj.UnitSelected != this.game.EditObj.OldUnit)
      {
        if (this.game.EditObj.OldUnit > -1 && this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.OldUnit].Regime && this.game.Data.UnitObj[this.game.EditObj.OldUnit].IsHQ)
          flag1 = true;
        if (this.game.EditObj.UnitSelected > -1 && this.game.Data.Turn == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
          flag1 = true;
      }
      if (flag1 & !noClearMap)
      {
        this.game.EditObj.TempCoordList = CoordList::new();
        if (!Information.IsNothing( this.SubPartList[0]))
          this.SubPartList[0].Paint();
      }
      else
      {
        bitmap: Bitmap;
        if (this.game.EditObj.ShowSameHistorical)
        {
          if (this.game.EditObj.OldUnit > -1 && !this.game.Data.UnitObj[this.game.EditObj.OldUnit].IsHQ & this.game.Data.UnitObj[this.game.EditObj.OldUnit].Regime == this.game.Data.Turn & this.game.Data.UnitObj[this.game.EditObj.OldUnit].HQ > -1)
          {
            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
            for (let mut index: i32 =  0; index <= unitCounter; index += 1)
            {
              if (!this.game.Data.UnitObj[index].IsHQ & this.game.Data.UnitObj[index].HQ == this.game.Data.UnitObj[this.game.EditObj.OldUnit].HQ | this.game.Data.UnitObj[this.game.EditObj.OldUnit].HQ == index && this.game.Data.UnitObj[index].HQ > -1 && this.game.Data.UnitObj[this.game.Data.UnitObj[index].HQ].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.Data.UnitObj[index].HQ].Historical].Type < 8 && this.game.Data.UnitObj[index].X > -1 & this.game.Data.UnitObj[index].PreDef == -1 && !Information.IsNothing( this.SubPartList[0]))
              {
                let mut subPart: SubPartClass = this.SubPartList[0];
                let mut x: i32 =  this.game.Data.UnitObj[index].X;
                let mut y: i32 =  this.game.Data.UnitObj[index].Y;
                let mut map: i32 =  num1;
                bitmap = (Bitmap) null;
                 let mut local: &Bitmap = &bitmap;
                subPart.PaintCoordinate((Graphics) null, x, y, map, gBitmap: ( local));
              }
            }
          }
          if (this.game.EditObj.UnitSelected > -1 && !this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ > -1)
          {
            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
            for (let mut index: i32 =  0; index <= unitCounter; index += 1)
            {
              if (this.game.Data.UnitObj[index].HQ == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ | this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HQ == index && this.game.Data.UnitObj[index].HQ > -1 && this.game.Data.UnitObj[this.game.Data.UnitObj[index].HQ].Historical > -1 && this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.Data.UnitObj[index].HQ].Historical].Type < 8 && this.game.Data.UnitObj[index].X > -1 & this.game.Data.UnitObj[index].PreDef == -1 && !Information.IsNothing( this.SubPartList[0]))
              {
                let mut subPart: SubPartClass = this.SubPartList[0];
                let mut x: i32 =  this.game.Data.UnitObj[index].X;
                let mut y: i32 =  this.game.Data.UnitObj[index].Y;
                let mut map: i32 =  num1;
                bitmap = (Bitmap) null;
                 let mut local: &Bitmap = &bitmap;
                subPart.PaintCoordinate((Graphics) null, x, y, map, gBitmap: ( local));
              }
            }
          }
        }
        if (this.game.EditObj.ShowUnderHQ && this.game.EditObj.UnitSelected != this.game.EditObj.OldUnit)
        {
          if (this.game.EditObj.OldUnit > -1 && this.game.Data.UnitObj[this.game.EditObj.OldUnit].IsHQ)
          {
            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
            for (let mut index: i32 =  0; index <= unitCounter; index += 1)
            {
              if (this.game.Data.UnitObj[index].HQ == this.game.EditObj.OldUnit && this.game.Data.UnitObj[index].X > -1 & this.game.Data.UnitObj[index].PreDef == -1 && !Information.IsNothing( this.SubPartList[0]))
              {
                let mut subPart: SubPartClass = this.SubPartList[0];
                let mut x: i32 =  this.game.Data.UnitObj[index].X;
                let mut y: i32 =  this.game.Data.UnitObj[index].Y;
                let mut map: i32 =  num1;
                bitmap = (Bitmap) null;
                 let mut local: &Bitmap = &bitmap;
                subPart.PaintCoordinate((Graphics) null, x, y, map, gBitmap: ( local));
              }
            }
          }
          if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
          {
            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
            for (let mut index: i32 =  0; index <= unitCounter; index += 1)
            {
              if (this.game.Data.UnitObj[index].HQ == this.game.EditObj.UnitSelected && this.game.Data.UnitObj[index].X > -1 & this.game.Data.UnitObj[index].PreDef == -1 && !Information.IsNothing( this.SubPartList[0]))
              {
                let mut subPart: SubPartClass = this.SubPartList[0];
                let mut x: i32 =  this.game.Data.UnitObj[index].X;
                let mut y: i32 =  this.game.Data.UnitObj[index].Y;
                let mut map: i32 =  num1;
                bitmap = (Bitmap) null;
                 let mut local: &Bitmap = &bitmap;
                subPart.PaintCoordinate((Graphics) null, x, y, map, gBitmap: ( local));
              }
            }
          }
        }
      }
      return flag1;
    }

    pub CheckMovePath: bool(subpartMapSlot: i32)
    {
      if ( this.game.Data.RuleVar[540] == 1.0 & this.game.Data.Round > 0 && !(this.game.EditObj.OrderType == 26 | this.game.EditObj.AIMoving))
      {
        ScreenClass screeny = this.game.FormRef.Screeny;
        System.Type type = typeof (HistoryWindowClass2);
         System.Type local =  type;
        if (screeny.WindowPresent( local))
          return false;
        CoordList coordList = CoordList::new();
        if (this.game.EditObj.MouseOverX > -1)
        {
          bool flag;
          if ((this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48) & (this.game.EditObj.TempValue[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] < 999 | this.game.EditObj.TempValue2[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] < 999))
            flag = true;
          if (this.game.EditObj.OrderType == 36 & this.game.EditObj.TempValue[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] < 999)
            flag = true;
          if (this.game.EditObj.OrderUnit > -1 && (this.game.EditObj.OrderType == 18 | this.game.EditObj.OrderType == 49) & this.game.EditObj.TempValue[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] >= this.game.HandyFunctionsObj.GetUnitWeight(this.game.EditObj.OrderUnit))
            flag = true;
          if (flag)
          {
            if (this.game.EditObj.MouseOverX > -1 & this.game.EditObj.MouseOverY > -1 && !(this.lastUnitSelected == this.game.EditObj.UnitSelected & this.lastMouseOverX == this.game.EditObj.MouseOverX & this.lastMouseOverY == this.game.EditObj.MouseOverY) | !this.game.EditObj.mouseOverActive)
            {
              if (this.game.EditObj.TempValue[0].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] >= 9999 && this.lastMouseOverX > -1 & this.lastMouseOverY > -1 && this.game.EditObj.TempValue[0].Value[this.lastMouseOverX, this.lastMouseOverY] >= 9999)
                flag = false;
              if (flag)
              {
                CoordList tempCoordList = this.game.EditObj.TempCoordList;
                this.game.EditObj.TempCoordList = CoordList::new();
                if (this.game.EditObj.mouseOverActive)
                {
                  this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempMovePathList);
                  let mut groupMoveCounter: i32 =  this.game.EditObj.tempGroupMoveCounter;
                  for (let mut index: i32 =  0; index <= groupMoveCounter; index += 1)
                  {
                    if (!Information.IsNothing( this.game.EditObj.tempGroupMovePath[index]))
                      this.game.EditObj.TempCoordList.AddList( this.game.EditObj.tempGroupMovePath[index]);
                  }
                }
                this.game.EditObj.TempMovePathList = CoordList::new();
                Coordinate coordinate;
                coordinate.onmap = true;
                coordinate.x = this.game.EditObj.MouseOverX;
                coordinate.y = this.game.EditObj.MouseOverY;
                for (; coordinate.onmap; coordinate = this.game.EditObj.TempCameFrom[0].Value[coordinate.x, coordinate.y])
                  this.game.EditObj.TempMovePathList.AddCoord(coordinate.x, coordinate.y, coordinate.map);
                this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempMovePathList);
                if (this.game.Data.Product >= 6)
                {
                  let mut groupMoveCounter: i32 =  this.game.EditObj.tempGroupMoveCounter;
                  for (let mut index: i32 =  0; index <= groupMoveCounter; index += 1)
                  {
                    this.game.EditObj.tempGroupMovePath[index] = CoordList::new();
                    if (!Information.IsNothing( this.game.EditObj.tempGroupMoveCameFrom[index]))
                    {
                      coordinate.x = this.game.EditObj.MouseOverX;
                      coordinate.y = this.game.EditObj.MouseOverY;
                      coordinate.onmap = true;
                      while (coordinate.onmap)
                      {
                        this.game.EditObj.tempGroupMovePath[index].AddCoord(coordinate.x, coordinate.y, coordinate.map);
                        let mut slot: i32 =  this.game.EditObj.tempGroupMoveCameFrom[index].FindSlot(coordinate.x, coordinate.y, 0);
                        if (slot > -1)
                        {
                          if (this.game.EditObj.tempGroupMoveCameFrom[index].coord[slot].data1 > -1)
                          {
                            coordinate.onmap = true;
                            coordinate.x = this.game.EditObj.tempGroupMoveCameFrom[index].coord[slot].data1;
                            coordinate.y = this.game.EditObj.tempGroupMoveCameFrom[index].coord[slot].data2;
                          }
                          else
                            coordinate.onmap = false;
                        }
                        else
                          coordinate.onmap = false;
                      }
                      this.game.EditObj.TempCoordList.AddList( this.game.EditObj.tempGroupMovePath[index]);
                    }
                  }
                }
                this.game.EditObj.mouseOverActive = true;
                this.lastMouseOverY = this.game.EditObj.MouseOverY;
                this.lastMouseOverX = this.game.EditObj.MouseOverX;
                this.lastUnitSelected = this.game.EditObj.UnitSelected;
                this.SubPartList[subpartMapSlot].Paint();
                this.game.EditObj.TempCoordList = tempCoordList;
                if (this.game.EditObj.udsUnitOrderMode == 36)
                  this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round(Conversion.Val( this.game.Data.RuleVar[703])));
                return true;
              }
            }
          }
          else if (this.game.EditObj.mouseOverActive)
          {
            CoordList tempCoordList = this.game.EditObj.TempCoordList;
            this.game.EditObj.TempCoordList = CoordList::new();
            this.game.EditObj.mouseOverActive = false;
            this.game.EditObj.TempCoordList.AddList( this.game.EditObj.TempMovePathList);
            let mut groupMoveCounter: i32 =  this.game.EditObj.tempGroupMoveCounter;
            for (let mut index: i32 =  0; index <= groupMoveCounter; index += 1)
            {
              if (!Information.IsNothing( this.game.EditObj.tempGroupMovePath[index]))
              {
                this.game.EditObj.TempCoordList.AddList( this.game.EditObj.tempGroupMovePath[index]);
                this.game.EditObj.tempGroupMovePath[index] = CoordList::new();
              }
            }
            this.SubPartList[subpartMapSlot].Paint();
            this.game.EditObj.TempCoordList = tempCoordList;
            this.game.EditObj.TempMovePathList = CoordList::new();
            if (this.game.EditObj.udsUnitOrderMode == 36)
              this.game.EventRelatedObj.DoCheckSpecificEvent( Math.Round(Conversion.Val( this.game.Data.RuleVar[703])));
            return true;
          }
        }
      }
      return false;
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      currentDescript: String = this.game.EditObj.CurrentDescript;
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        bool t;
        for (let mut subpartMapSlot: i32 =  0; subpartMapSlot <= subPartCounter; subpartMapSlot += 1)
        {
          if (x > this.SubPartX[subpartMapSlot] & x < this.SubPartX[subpartMapSlot] + this.SubPartW[subpartMapSlot] && y > this.SubPartY[subpartMapSlot] & y < this.SubPartY[subpartMapSlot] + this.SubPartH[subpartMapSlot] && this.SubPartID[subpartMapSlot] == this.MapId)
          {
            Coordinate coordinate = this.SubPartList[subpartMapSlot].ClickMap(x - this.SubPartX[subpartMapSlot], y - this.SubPartY[subpartMapSlot]);
            if (coordinate.onmap)
            {
              this.game.EditObj.MouseOverX = coordinate.x;
              this.game.EditObj.MouseOverY = coordinate.y;
              if (!(this.game.EditObj.OrderType == 26 | this.game.EditObj.AIMoving) && this.CheckMovePath(subpartMapSlot))
                t = true;
              this.game.EditObj.CurrentDescript = "";
              windowReturnClass.SetFlag(t);
            }
            else if (!(this.game.EditObj.OrderType == 26 | this.game.EditObj.AIMoving) && this.CheckMovePath(subpartMapSlot))
              t = true;
          }
        }
        windowReturnClass.SetFlag(t);
        if (t)
        {
          windowReturnClass.AddCommand(4, 67);
          this.game.EditObj.PurelyOrderRedrawRefresh = true;
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
        }
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      windowReturnClass.SetOverlay(false);
      return windowReturnClass;
    }

    pub object EditorPlaceLocation()
    {
      num1: i32;
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == -1)
      {
        str: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give People # for town...", "Shadow Empire : Planetary Conquest")));
        if (Operators.CompareString(str, "", false) == 0)
          return  -1;
        num1 =  Math.Round(Conversion.Val(str));
        if (num1 == -1 | num1 > this.game.Data.PeopleCounter)
        {
          let mut num2: i32 =   Interaction.MsgBox( "Invalid people#", Title: ( "Shadow Empire : Planetary Conquest"));
          return  -1;
        }
      }
      Left: String = Interaction.InputBox("Give Name for this location...", "Shadow Empire : Planetary Conquest", this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name);
      if (Operators.CompareString(Left, "", false) == 0)
        return  -1;
      let mut regime: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
      if (this.game.Data.LocTypeObj[this.game.EditObj.PencilData1].OverdrawLTNr > -1)
      {
        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType = this.game.Data.LocTypeObj[this.game.EditObj.PencilData1].OverdrawLTNr;
        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr = this.game.Data.LocTypeObj[this.game.EditObj.PencilData1].OverdrawSpriteNr;
      }
      this.game.Data.AddLoc(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location = this.game.Data.LocCounter;
      if (regime > -1)
        this.game.Data.LocObj[this.game.Data.LocCounter].People = this.game.Data.RegimeObj[regime].People;
      else
        this.game.Data.LocObj[this.game.Data.LocCounter].People = num1;
      this.game.Data.LocObj[this.game.Data.LocCounter].Production[0] = -1;
      this.game.Data.LocObj[this.game.Data.LocCounter].Production[1] = -1;
      this.game.Data.LocObj[this.game.Data.LocCounter].Production[2] = -1;
      this.game.Data.LocObj[this.game.Data.LocCounter].Production[3] = -1;
      this.game.Data.LocObj[this.game.Data.LocCounter].StructuralPts = this.game.Data.LocTypeObj[this.game.EditObj.PencilData1].StructuralPts;
      this.game.Data.LocObj[this.game.Data.LocCounter].HQ = -1;
      this.game.Data.LocObj[this.game.Data.LocCounter].Type = this.game.EditObj.PencilData1;
      this.game.Data.LocObj[this.game.Data.LocCounter].Name = Left;
      object obj;
      return obj;
    }

    pub object hexLibVarFill(slot: i32, code: i32)
    {
      object[,] objArray = new object[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      objArray[this.game.SelectX, this.game.SelectY] =  1;
      let mut hexLibVarValue: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].GetHexLibVarValue(slot);
      bool isSea = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea;
      let mut num: i32 =  1;
      while (num == 1)
      {
        num = 0;
        Right: i32;
        Right += 1;
        let mut mapWidth: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (Operators.ConditionalCompareObjectEqual(objArray[cx, cy],  Right, false))
            {
              num = 1;
              this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].SetHexLibVarValue(slot, code);
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, this.game.EditObj.MapSelected, tfacing);
                if (coordinate.onmap && Operators.ConditionalCompareObjectEqual(objArray[coordinate.x, coordinate.y],  0, false) && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].GetHexLibVarValue(slot) == hexLibVarValue && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea == isSea)
                  objArray[coordinate.x, coordinate.y] =  (Right + 1);
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      object obj;
      return obj;
    }

    pub object regimeFill(newreg: i32)
    {
      object[,] objArray = new object[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      objArray[this.game.SelectX, this.game.SelectY] =  1;
      let mut regime: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime;
      bool isSea = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea;
      let mut num: i32 =  1;
      while (num == 1)
      {
        num = 0;
        Right: i32;
        Right += 1;
        let mut mapWidth: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (Operators.ConditionalCompareObjectEqual(objArray[cx, cy],  Right, false))
            {
              num = 1;
              this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].Regime = newreg;
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, this.game.EditObj.MapSelected, tfacing);
                if (coordinate.onmap && Operators.ConditionalCompareObjectEqual(objArray[coordinate.x, coordinate.y],  0, false) && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].Regime == regime && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea == isSea)
                  objArray[coordinate.x, coordinate.y] =  (Right + 1);
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      object obj;
      return obj;
    }

    pub object areacodeFill(slot: i32, code: i32)
    {
      object[,] objArray = new object[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      objArray[this.game.SelectX, this.game.SelectY] =  1;
      let mut num1: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].AreaCode[slot];
      bool isSea = this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea;
      let mut num2: i32 =  1;
      while (num2 == 1)
      {
        num2 = 0;
        Right: i32;
        Right += 1;
        let mut mapWidth: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (Operators.ConditionalCompareObjectEqual(objArray[cx, cy],  Right, false))
            {
              num2 = 1;
              this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].AreaCode[slot] = code;
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, this.game.EditObj.MapSelected, tfacing);
                if (coordinate.onmap && Operators.ConditionalCompareObjectEqual(objArray[coordinate.x, coordinate.y],  0, false) && this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].AreaCode[slot] == num1 && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[coordinate.map].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea == isSea)
                  objArray[coordinate.x, coordinate.y] =  (Right + 1);
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      object obj;
      return obj;
    }

    pub object landscapeFill(newland: i32, newspr: i32)
    {
      object[,] objArray = new object[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      objArray[this.game.SelectX, this.game.SelectY] =  1;
      let mut landscapeType: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType;
      let mut spriteNr: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr;
      let mut num: i32 =  1;
      while (num == 1)
      {
        num = 0;
        Right: i32;
        Right += 1;
        let mut mapSelected: i32 =  this.game.EditObj.MapSelected;
        let mut mapWidth: i32 =  this.game.Data.MapObj[mapSelected].MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[mapSelected].MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
          {
            if (Operators.ConditionalCompareObjectEqual(objArray[index1, index2],  Right, false))
            {
              num = 1;
              this.game.Data.MapObj[mapSelected].HexObj[index1, index2].LandscapeType = newland;
              this.game.Data.MapObj[mapSelected].HexObj[index1, index2].SpriteNr = newspr;
              this.game.HandyFunctionsObj.RandomizeHex(index1, index2, mapSelected);
              if (this.game.Data.LandscapeTypeObj[newland].IsSea)
                this.game.Data.MapObj[mapSelected].HexObj[index1, index2].Regime = -1;
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(index1, index2, mapSelected, tfacing);
                if (coordinate.onmap && Operators.ConditionalCompareObjectEqual(objArray[coordinate.x, coordinate.y],  0, false) && this.game.Data.MapObj[mapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType == landscapeType & this.game.Data.MapObj[mapSelected].HexObj[coordinate.x, coordinate.y].SpriteNr == spriteNr && this.game.Data.MapObj[mapSelected].HexObj[coordinate.x, coordinate.y].Location == -1)
                  objArray[coordinate.x, coordinate.y] =  (Right + 1);
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      object obj;
      return obj;
    }

    pub object specialFill(newland: i32, newspr: i32)
    {
      object[,] objArray = new object[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      objArray[this.game.SelectX, this.game.SelectY] =  1;
      let mut specialType: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpecialType;
      let mut specialSprite: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpecialSprite;
      let mut num: i32 =  1;
      while (num == 1)
      {
        num = 0;
        Right: i32;
        Right += 1;
        let mut mapSelected: i32 =  this.game.EditObj.MapSelected;
        let mut mapWidth: i32 =  this.game.Data.MapObj[mapSelected].MapWidth;
        for (let mut cx: i32 =  0; cx <= mapWidth; cx += 1)
        {
          let mut mapHeight: i32 =  this.game.Data.MapObj[mapSelected].MapHeight;
          for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
          {
            if (Operators.ConditionalCompareObjectEqual(objArray[cx, cy],  Right, false))
            {
              num = 1;
              this.game.Data.MapObj[mapSelected].HexObj[cx, cy].SpecialType = newland;
              this.game.Data.MapObj[mapSelected].HexObj[cx, cy].SpecialSprite = newspr;
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, mapSelected, tfacing);
                if (coordinate.onmap && Operators.ConditionalCompareObjectEqual(objArray[coordinate.x, coordinate.y],  0, false) && this.game.Data.MapObj[mapSelected].HexObj[coordinate.x, coordinate.y].SpecialType == specialType & this.game.Data.MapObj[mapSelected].HexObj[coordinate.x, coordinate.y].SpecialSprite == specialSprite)
                  objArray[coordinate.x, coordinate.y] =  (Right + 1);
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      object obj;
      return obj;
    }

    pub fn PopUpRefresh()
    {
      if (Information.IsNothing( this.game))
        this.game = DrawMod.TGame;
      this.game.EditObj.MyDelegateMap = (EditClass.AfterPopUpRefresh) null;
      if (this.game.EditObj.OrderType == 2)
      {
        if (this.game.EditObj.TempUnitList.counter > -1)
        {
          this.game.EditObj.BattleTimerActive = true;
          this.game.EditObj.BattleAnimNr = 0;
          this.game.EditObj.BattleTimer = DateTime.Now + new TimeSpan(0, 0, 0, 0, 1000);
          return;
        }
        this.game.EditObj.TempUnitList = UnitList::new();
        this.game.EditObj.OrderType = 0;
      }
      if (this.game.EditObj.OrderType == 11 | this.game.EditObj.OrderType == 14 | this.game.EditObj.OrderType == 33)
      {
        if (this.game.EditObj.TempUnitList.counter > -1)
        {
          this.game.EditObj.BattleTimerActive = true;
          this.game.EditObj.BattleAnimNr = 0;
          this.game.EditObj.BattleTimer = DateTime.Now + new TimeSpan(0, 0, 0, 0, 1000);
          return;
        }
        this.game.EditObj.TempUnitList = UnitList::new();
        this.game.EditObj.OrderType = 0;
      }
      if (this.game.EditObj.OrderUnit > -1 & this.game.EditObj.OrderType > 0)
      {
        this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
        this.game.SelectX = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].X;
        this.game.SelectY = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Y;
      }
      else
        this.game.EditObj.OrderType = 0;
      this.game.EditObj.TargetX = -1;
      this.game.EditObj.TargetY = -1;
      this.game.EditObj.AreaSlot = -1;
      this.game.EditObj.AreaX = -1;
      this.game.EditObj.AreaY = -1;
      this.game.EditObj.AreaMap = -1;
      this.game.EditObj.AreaValue = -1;
      if ( this.game.Data.RuleVar[701] >= 1.0)
        this.game.EditObj.udsReturnFromPopup = true;
      this.DoRefresh();
    }
  }
}
