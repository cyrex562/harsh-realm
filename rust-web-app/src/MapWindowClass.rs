// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MapWindowClass
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
  pub class MapWindowClass : WindowClass
  {
     MapId: i32;
     minheight: i32;
     minwidth: i32;
     ZoomTimer: i32;
     float LastZoom;

    pub MapWindowClass( tGame: GameClass, let mut tminheight: i32 =  0, let mut tminwidth: i32 =  200, let mut tZoomLevel: i32 =  -2)
      : base( tGame, tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight)
    {
      this.minheight = tminheight;
      this.minwidth = tminwidth;
      let mut tsubpart: SubPartClass =  new MapPartClass(tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight, tGame, ZoomLevel: tZoomLevel);
      this.MapId = this.AddSubPart( tsubpart, 0, 0, tGame.ScreenWidth - tminwidth, tGame.ScreenHeight - tminheight, 0);
      this.game.EditObj.TempCoordList = CoordList::new();
      this.SubPartList[this.SubpartNr(this.MapId)].Paint();
      this.mapframe = true;
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
      if (this.game.EditObj.ScrollDir == 1 & !this.game.EditObj.BattleTimerActive)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(38, false);
      }
      if (this.game.EditObj.ScrollDir == 2 & !this.game.EditObj.BattleTimerActive)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(39, false);
      }
      if (this.game.EditObj.ScrollDir == 3 & !this.game.EditObj.BattleTimerActive)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(40, false);
      }
      if (this.game.EditObj.ScrollDir == 4 & !this.game.EditObj.BattleTimerActive)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(37, false);
      }
      if (!this.game.AIThreadRunning && this.game.EditObj.BattleTimerActive)
      {
        if (this.game.EditObj.BattleAnimNr == 0 && this.game.EditObj.SoundOn)
          SoundMod.PlayAWave(this.game.AppPath + "sound/att_arm3.wav",  this.game.EditObj);
        if (DateTime.Compare(DateTime.Now, this.game.EditObj.BattleTimer) > 0)
        {
          this.game.EditObj.BattleTimerActive = false;
          this.game.EditObj.BattleAnimNr = 0;
          this.game.TempCombat = new CombatClass(this.game);
          if (this.game.TempCombat.Init(Coordinate::new()
          {
            x = this.game.EditObj.TargetX,
            y = this.game.EditObj.TargetY,
            map = this.game.EditObj.TargetMap
          }, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
          {
            if ( this.game.Data.RuleVar[839] == 1.0)
            {
              this.game.EditObj.PopupValue = 7;
              windowReturnClass.AddCommand(5, 10);
              this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            windowReturnClass.AddCommand(3, 5);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
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
      bool flag1 = false;
      let mut cornerX: i32 =  this.game.CornerX;
      let mut cornerY: i32 =  this.game.CornerY;
      if (this.game.Data.Round < 1 && this.game.Data.PermanentOverlayUse)
      {
        if (nr == 56 | nr == 104)
        {
          this.game.EditObj.overlayOffsetY += 20;
          flag1 = true;
        }
        if (nr == 52 | nr == 100)
        {
          this.game.EditObj.overlayOffsetX += 20;
          flag1 = true;
        }
        if (nr == 54 | nr == 102)
        {
          this.game.EditObj.overlayOffsetX -= 20;
          flag1 = true;
        }
        if (nr == 50 | nr == 98)
        {
          this.game.EditObj.overlayOffsetY -= 20;
          flag1 = true;
        }
        if (flag1)
        {
          this.game.EditObj.TempCoordList = CoordList::new();
          this.DoRefresh();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (nr == 46)
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
        flag1 = true;
      }
      if (nr == 37 & (this.game.CornerX > 0 | this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & ( this.game.Data.RuleVar[329] == 0.0 | this.game.Data.Round > 0)))
      {
        --this.game.CornerX;
        if (0 > this.game.CornerX & (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0 | this.game.Data.Round == 0))
          this.game.CornerX = 0;
        flag1 = true;
      }
      if (nr == 40)
      {
        this += 1.game.CornerY;
        flag1 = true;
      }
      if (nr == 38 & this.game.CornerY > -1)
      {
        --this.game.CornerY;
        if (this.game.CornerY < -1)
          this.game.CornerY = -1;
        flag1 = true;
      }
      bool flag2;
      if (this.game.Data.Round == 0)
      {
        if (nr == 49)
        {
          this += 1.game.EditObj.OverlayMode;
          if (this.game.EditObj.OverlayMode > 2)
            this.game.EditObj.OverlayMode = 0;
          flag2 = true;
        }
        if (nr == 75 && this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          let mut landscapeType: i32 =  this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType;
          let mut spriteNr: i32 =  this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr;
          if (landscapeType > -1)
          {
            if (this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].randomOverrule < 1)
              this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].randomOverrule = 1;
            HexClass[,] hexObj = this.game.Data.MapObj[0].HexObj;
            HexClass[,] hexClassArray = hexObj;
            let mut selectX: i32 =  this.game.SelectX;
            let mut index1: i32 =  selectX;
            let mut selectY: i32 =  this.game.SelectY;
            let mut index2: i32 =  selectY;
            hexClassArray[index1, index2].randomOverrule = hexObj[selectX, selectY].randomOverrule + 1;
            if (this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].randomOverrule > 9)
              this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].randomOverrule = 1;
            this.game.EditObj.TempCoordList = CoordList::new();
            this.DoRefresh();
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          }
        }
      }
      if (this.game.Data.Round == 0 & this.game.SelectX > -1)
      {
        switch (nr)
        {
          case 33:
            Form3::new( this.formref).Initialize(this.game.Data, 87, this.game.SelectX, this.game.SelectY, DrawMod.TGame);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
          case 35:
            if (this.game.SelectX > -1)
            {
              this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].VP =  Math.Round(Conversion.Val(Interaction.InputBox("Give new VP for hex", "Shadow Empire : Planetary Conquest")));
              break;
            }
            break;
          case 36:
            Form3::new( this.formref).Initialize(this.game.Data, 47, this.game.SelectX, this.game.SelectY, DrawMod.TGame);
            windowReturnClass.SetFlag(true);
            return windowReturnClass;
        }
      }
      if (this.game.Data.Round == 0 & this.game.EditObj.UnitSelected > -1)
      {
        if (nr == 55)
        {
          Form3::new( this.formref).Initialize(this.game.Data, 37, this.game.EditObj.UnitSelected);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 48 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1)
        {
          Form3::new( this.formref).Initialize(this.game.Data, 36, this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 57 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1)
        {
          this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].CounterString = Interaction.InputBox("Give Shortname for this historical unit");
          this.SubPartList[this.SubpartNr(this.MapId)].Paint();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 56 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1)
        {
          this.game.Data.HistoricalUnitObj[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical].Counter =  Math.Round(Conversion.Val(Interaction.InputBox("Give NATO counter # for this historical unit. (see systemgraphics/nato directory for numbers and pictures)")));
          this.SubPartList[this.SubpartNr(this.MapId)].Paint();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
        if (nr == 45 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical == -1)
        {
          let mut unitSelected: i32 =  this.game.EditObj.UnitSelected;
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[unitSelected].X, this.game.Data.UnitObj[unitSelected].Y].Regime > -1)
            this.game.Data.UnitObj[unitSelected].Regime = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[unitSelected].X, this.game.Data.UnitObj[unitSelected].Y].Regime;
          if (this.game.Data.UnitObj[unitSelected].IsHQ)
          {
            this.game.Data.AddHistoricalUnit();
            let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Name = this.game.Data.UnitObj[unitSelected].Name;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Type = 5 + this.game.HandyFunctionsObj.HowmanyHQsBelow(unitSelected);
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Counter = -1;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].TempRegime = this.game.Data.UnitObj[unitSelected].Regime;
            if (Strings.InStr(this.game.Data.UnitObj[unitSelected].Name, " ") > 0)
              this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Strings.Left(this.game.Data.UnitObj[unitSelected].Name, Math.Min(6, Strings.InStr(this.game.Data.UnitObj[unitSelected].Name, " ") - 1));
            else
              this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Strings.Left(this.game.Data.UnitObj[unitSelected].Name, 6);
            this.game.Data.UnitObj[unitSelected].Historical = historicalUnitCounter;
          }
          else
          {
            this.game.Data.AddHistoricalUnit();
            let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].TempRegime = this.game.Data.UnitObj[unitSelected].Regime;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Name = this.game.Data.UnitObj[unitSelected].Name;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Type = 1;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Counter =  Math.Round(Conversion.Val(Interaction.InputBox("NATO counter #")));
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Interaction.InputBox("Shortname");
            let mut length: i32 =  this.game.Data.UnitObj[unitSelected].Name.Length;
            for (let mut Start: i32 =  1; Start <= length; Start += 1)
            {
              let mut Number: i32 =   Math.Round(Conversion.Val(Strings.Mid(this.game.Data.UnitObj[unitSelected].Name, Start)));
              if (Number > 0)
              {
                this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Strings.Trim(Conversion.Str( Number));
                break;
              }
            }
            this.game.Data.UnitObj[unitSelected].Historical = historicalUnitCounter;
          }
        }
        if (nr == 8 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical > -1)
        {
          let mut num: i32 =  0;
          let mut unitCounter: i32 =  this.game.Data.UnitCounter;
          for (let mut index: i32 =  0; index <= unitCounter; index += 1)
          {
            if (this.game.Data.UnitObj[index].PreDef == -1 && this.game.Data.UnitObj[index].Historical == this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical)
              num += 1;
          }
          if (num == 1)
          {
            if (Interaction.MsgBox( "This is last unit attached to this historical unit. Delete Historical Unit? ", MsgBoxStyle.YesNo) == MsgBoxResult.Yes)
              this.game.Data.RemoveHistoricalUnit(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical);
          }
          else
            this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical = -1;
          this.SubPartList[this.SubpartNr(this.MapId)].Paint();
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      let mut num1: i32 =  0;
      let mut num2: i32 =   Math.Round(Conversion.Int( this.OwnBitmap.Width /  (30 * (this.game.EditObj.Zoom + 2))));
      let mut num3: i32 =  this.game.EditObj.Zoom != 1 ?  Math.Round(Conversion.Int( (this.OwnBitmap.Height - num1) /  (24 * (this.game.EditObj.Zoom + 2)))) :  Math.Round(Conversion.Int( (this.OwnBitmap.Height - num1) /  (24 * (this.game.EditObj.Zoom + 3))));
      let mut num4: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - this.game.CornerX + 2;
      let mut num5: i32 =   this.game.Data.RuleVar[839] != 1.0 ? this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - this.game.CornerY + 3 : this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 2 - this.game.CornerY + 3;
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
      if (this.game.CornerX > cornerX && num2 > num4 & (!this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop |  this.game.Data.RuleVar[329] == 1.0 | this.game.Data.Round == 0))
      {
        flag1 = true;
        this.game.CornerX = cornerX;
      }
      if ( this.game.Data.RuleVar[329] == 0.0 & this.game.Data.Round > 0)
      {
        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & 0 > this.game.CornerX)
          this.game.CornerX = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + this.game.CornerX + 1;
        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & this.game.CornerX > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
          this.game.CornerX -= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1;
      }
      if (this.game.CornerY > cornerY && num3 > num5)
      {
        flag1 = true;
        this.game.CornerY = cornerY;
      }
      if (this.game.CornerX == cornerX & this.game.CornerY == cornerY)
        flag1 = false;
      if (flag2)
        flag1 = true;
      if (!flag1)
        return windowReturnClass;
      if (nr == 39)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftLeft();
      if (nr == 37)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftRight();
      if (nr == 40)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftUp();
      if (nr == 38)
        this.SubPartList[this.SubpartNr(this.MapId)].ShiftDown();
      if (flag2)
        this.SubPartList[this.SubpartNr(this.MapId)].Paint();
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
          this.game.EditObj.PurelyOrderRedrawRefresh = true;
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.AddCommand(4, 68);
          windowReturnClass.AddCommand(4, 9);
        }
      }
      return windowReturnClass;
    }

    pub fn DoRefresh() => this.SubPartList[this.SubpartNr(this.MapId)].Paint();

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult1 = OrderResult::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
label_329:
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1] && this.SubPartID[index1] == this.MapId)
          {
            let mut num1: i32 =  this.game.SelectX;
            let mut selectY1: i32 =  this.game.SelectY;
            let mut mapSelected1: i32 =  this.game.EditObj.MapSelected;
            Coordinate coordinate1 = this.SubPartList[index1].ClickMap(x - this.SubPartX[index1], y - this.SubPartY[index1]);
            if (coordinate1.onmap)
            {
              if (this.game.EditObj.TutMode & this.game.EditObj.OrderType > 0 & this.game.EditObj.OrderType < 9999 && Operators.ConditionalCompareObjectGreater(this.game.EditObj.TutX,  -1, false) && Conversions.ToBoolean(Operators.NotObject(Operators.AndObject(Operators.CompareObjectEqual(this.game.EditObj.TutX,  coordinate1.x, false), Operators.CompareObjectEqual(this.game.EditObj.TutY,  coordinate1.y, false)))))
                return windowReturnClass;
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, 0);
              this.game.SelectX = coordinate1.x;
              this.game.SelectY = coordinate1.y;
              this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, 0);
              let mut num2: i32 =  !(num1 == this.game.SelectX & selectY1 == this.game.SelectY) ? this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, true, b, coordinate1.data1, coordinate1.penalty) : this.game.HandyFunctionsObj.ClickOnHexGivesUnit(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, false, b, coordinate1.data1, coordinate1.penalty);
              if (this.game.EditObj.UnitSelected != num2)
                this.game.EditObj.SFSelected = -1;
              this.game.EditObj.UnitSelected = num2;
              if (!(this.game.SelectX > -1 & this.game.SelectY > -1 & this.game.SelectX <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth & this.game.SelectY <= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight) || !this.game.Data.PermanentOverlayUse)
                ;
              if (b == 2)
              {
                if (this.game.Data.Round == 0)
                {
                  this.game.EditObj.RightClickX = this.game.SelectX;
                  this.game.EditObj.RightCLickY = this.game.SelectY;
                  if (this.game.EditObj.PencilType == 12 && this.game.EditObj.PencilMode == 0 & this.game.EditObj.PencilData1 == 1)
                  {
                    let mut num3: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel - 1;
                    if (num3 < 0)
                      num3 = 0;
                    this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel = num3;
                    let mut num4: i32 =  num3;
                    let mut num5: i32 =  num3;
                    let mut tfacing: i32 =  1;
                    do
                    {
                      Coordinate coordinate2 = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
                      if (coordinate2.onmap)
                      {
                        if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].HeightLevel + 3 < num5)
                          num5 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].HeightLevel + 3;
                        if (this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].HeightLevel - 3 > num4)
                          num4 = this.game.Data.MapObj[0].HexObj[coordinate2.x, coordinate2.y].HeightLevel - 3;
                      }
                      tfacing += 1;
                    }
                    while (tfacing <= 6);
                    if (num4 > num3)
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel = num4;
                    else if (num5 < num3)
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel = num5;
                  }
                  if (this.game.Data.SimpleEditor)
                  {
                    windowReturnClass.AddCommand(4, 12);
                    windowReturnClass.AddCommand(4, 69);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                }
                if (this.game.Data.Round > 0)
                {
                  if (this.game.EditObj.OrderType == 9 && this.game.EditObj.UnitSelected > -1 && this.game.EditObj.OrderTarget != this.game.EditObj.UnitSelected && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.Round > 0 & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn)
                  {
                    this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                    windowReturnClass.AddCommand(4, 30);
                  }
                  if (this.game.EditObj.LayerSupplyOn)
                  {
                    if (!(this.game.EditObj.RightClickX == this.game.SelectX & this.game.EditObj.RightCLickY == this.game.SelectY & this.game.EditObj.RightClickMap == this.game.EditObj.MapSelected))
                    {
                      this.game.EditObj.SupplyPath = CoordList::new();
                      this.game.EditObj.RightClickX = this.game.SelectX;
                      this.game.EditObj.RightCLickY = this.game.SelectY;
                      this.game.EditObj.RightClickMap = this.game.EditObj.MapSelected;
                      let mut x1: i32 =  this.game.EditObj.RightClickX;
                      let mut y1: i32 =  this.game.EditObj.RightCLickY;
                      let mut map1: i32 =  this.game.EditObj.RightClickMap;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      map2: i32;
                      for (; this.game.EditObj.TempSupCameFrom[map1].Value[x1, y1].onmap; map1 = map2)
                      {
                        this.game.EditObj.SupplyPath.AddCoord(x1, y1, map1);
                        let mut x2: i32 =  this.game.EditObj.TempSupCameFrom[map1].Value[x1, y1].x;
                        let mut y2: i32 =  this.game.EditObj.TempSupCameFrom[map1].Value[x1, y1].y;
                        map2 = this.game.EditObj.TempSupCameFrom[map1].Value[x1, y1].map;
                        x1 = x2;
                        y1 = y2;
                      }
                    }
                    else
                    {
                      this.game.EditObj.RightClickX = -1;
                      this.game.EditObj.RightCLickY = -1;
                      this.game.EditObj.SupplyPath = (CoordList) null;
                    }
                  }
                  windowReturnClass.AddCommand(4, 44);
                  windowReturnClass.AddCommand(4, 18);
                  windowReturnClass.AddCommand(4, 20);
                  windowReturnClass.AddCommand(4, 66);
                  windowReturnClass.AddCommand(4, 12);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
              if (b == 1)
              {
                if (this.game.Data.Round == 0)
                {
                  integer: i32;
                  if (this.game.EditObj.PencilType == 1)
                  {
                    if (this.game.EditObj.PencilMode == 0)
                    {
                      num1 = 6;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1)
                        num1 =  Interaction.MsgBox( "There is a location in this hex. Are you sure to change landscape?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest");
                      if (num1 == 6)
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
                      num1 = 6;
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
                  else if (this.game.EditObj.PencilType == 12)
                  {
                    if (this.game.EditObj.PencilMode == 0)
                    {
                      let mut num6: i32 =  this.game.EditObj.PencilData2;
                      if (this.game.EditObj.PencilData1 == 1)
                        num6 = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel + 1;
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel = num6;
                      let mut num7: i32 =  num6;
                      let mut num8: i32 =  num6;
                      let mut tfacing: i32 =  1;
                      do
                      {
                        Coordinate coordinate3 = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
                        if (coordinate3.onmap)
                        {
                          if (this.game.Data.MapObj[0].HexObj[coordinate3.x, coordinate3.y].HeightLevel + 3 < num8)
                            num8 = this.game.Data.MapObj[0].HexObj[coordinate3.x, coordinate3.y].HeightLevel + 3;
                          if (this.game.Data.MapObj[0].HexObj[coordinate3.x, coordinate3.y].HeightLevel - 3 > num7)
                            num7 = this.game.Data.MapObj[0].HexObj[coordinate3.x, coordinate3.y].HeightLevel - 3;
                        }
                        tfacing += 1;
                      }
                      while (tfacing <= 6);
                      if (num7 > num6)
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel = num7;
                      else if (num8 < num6)
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].HeightLevel = num8;
                    }
                    else if (this.game.EditObj.PencilData1 == 0)
                    {
                      this.heightLevelFill(this.game.EditObj.PencilData2);
                      this.game.EditObj.TempCoordList = CoordList::new();
                      this.DoRefresh();
                    }
                  }
                  else if (this.game.EditObj.PencilType == 2)
                  {
                    let mut index2: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    let mut index3: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected);
                    if (index2 > -1)
                    {
                      let mut index4: i32 =  index2 - 1;
                      let mut index5: i32 =  index3 - 1;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index5] == -1)
                      {
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RoadType[index4] = this.game.EditObj.PencilData1;
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index5] = this.game.EditObj.PencilData1;
                      }
                      else
                      {
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RoadType[index4] = -1;
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index5] = -1;
                      }
                    }
                    else if (index2 > -1)
                    {
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RoadType[index2] = -1;
                      this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index3] = -1;
                    }
                    this.game.EditObj.RightClickX = this.game.SelectX;
                    this.game.EditObj.RightCLickY = this.game.SelectY;
                  }
                  else if (this.game.EditObj.PencilType == 3)
                  {
                    if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType].IsSea && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime != this.game.EditObj.PencilData1)
                    {
                      if (this.game.EditObj.PencilMode == 0)
                      {
                        if (this.game.EditObj.PencilData1 > this.game.Data.RegimeCounter)
                        {
                          this.game.EditObj.TempCoordList = CoordList::new();
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime = -1;
                        }
                        else
                        {
                          this.game.EditObj.TempCoordList = CoordList::new();
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime = this.game.EditObj.PencilData1;
                        }
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
                      integer = Conversions.ToInteger(this.EditorPlaceLocation());
                    else if (Interaction.MsgBox( "Replace existing location?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                    {
                      this.game.Data.RemoveLoc(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location);
                      integer = Conversions.ToInteger(this.EditorPlaceLocation());
                    }
                  }
                  else if (this.game.EditObj.PencilType == 5)
                  {
                    let mut num9: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    let mut num10: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected);
                    if (num9 > -1)
                    {
                      let mut index6: i32 =  num9 - 1;
                      let mut index7: i32 =  num10 - 1;
                      bool flag = false;
                      if (this.game.Data.RiverTypeObj[this.game.EditObj.PencilData1].drawInteriorOnly)
                        flag = true;
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RiverType[index6] == -1)
                      {
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RiverType[index6] = this.game.EditObj.PencilData1;
                        if (!flag)
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RiverType[index7] = this.game.EditObj.PencilData1;
                      }
                      else
                      {
                        if (!flag)
                          this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RiverType[index7] = -1;
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].RiverType[index6] = -1;
                      }
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RiverType[index7] == -1)
                        ;
                      this.game.EditObj.RightClickX = this.game.SelectX;
                      this.game.EditObj.RightCLickY = this.game.SelectY;
                    }
                  }
                  else if (this.game.EditObj.PencilType == 6)
                  {
                    let mut num11: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                    let mut num12: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY, this.game.EditObj.MapSelected);
                    if (num11 > -1)
                    {
                      let mut index8: i32 =  num11 - 1;
                      let mut index9: i32 =  num12 - 1;
                      if (!this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].Bridge[index8])
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].Bridge[index8] = true;
                      else
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.RightClickX, this.game.EditObj.RightCLickY].Bridge[index8] = false;
                      if (!this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Bridge[index9])
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Bridge[index9] = true;
                      else
                        this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Bridge[index9] = false;
                      this.game.EditObj.RightClickX = this.game.SelectX;
                      this.game.EditObj.RightCLickY = this.game.SelectY;
                    }
                  }
                  if (this.game.EditObj.OrderType == 0)
                  {
                    windowReturnClass.AddCommand(4, 18);
                    windowReturnClass.AddCommand(4, 20);
                    windowReturnClass.AddCommand(4, 44);
                  }
                  else
                  {
                    if (this.game.EditObj.OrderType == 1)
                    {
                      this.game.ProcessingObj.EditorMovement(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                      this.game.EditObj.OrderType = 0;
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 66);
                        windowReturnClass.AddCommand(4, 44);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 9);
                      }
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    if (this.game.EditObj.OrderType == 3)
                    {
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 4)
                    {
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 5)
                    {
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 20)
                    {
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 44);
                    }
                    else if (this.game.EditObj.OrderType == 21)
                    {
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 44);
                    }
                    else
                      windowReturnClass.AddCommand(4, 12);
                  }
                }
                if (this.game.Data.Round > 0)
                {
                  if (this.game.EditObj.OrderType == 0 | this.game.EditObj.OrderType == 26)
                  {
                    if ( this.game.Data.RuleVar[839] == 0.0)
                    {
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 44);
                    }
                    else
                    {
                      windowReturnClass.AddCommand(4, 69);
                      windowReturnClass.AddCommand(4, 68);
                    }
                  }
                  else if (this.game.EditObj.OrderType == 43)
                  {
                    if (this.game.EditObj.TempValue[0].Value[this.game.SelectX, this.game.SelectY] == 0)
                    {
                      Form3::new( this.formref).Initialize(this.game.Data, 41, 0, 0, this.game);
                      this.game.EditObj.OrderType = 0;
                      this.game.EditObj.TempCoordList = CoordList::new();
                      windowReturnClass.AddCommand(4, 12);
                      windowReturnClass.AddCommand(4, 29);
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                  }
                  else
                  {
                    if (this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48)
                    {
                      SimpleList simpleList = SimpleList::new();
                      if (this.game.EditObj.OrderType == 1)
                      {
                        simpleList.Add(this.game.EditObj.OrderUnit, 1);
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
                      for (let mut index10: i32 =  0; index10 <= counter; index10 += 1)
                      {
                        this.game.EditObj.OrderUnit = simpleList.Id[index10];
                        if (this.game.EditObj.OrderType == 48)
                          this.game.HandyFunctionsObj.MakeMovePrediction(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, attackoptions: true, ismove: true);
                        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].UnitCounter > 14 & (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn | this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == -1))
                        {
                          this.game.EditObj.FeedBackString = "Already max ammount of 16 units in that hex.";
                          if ( this.game.Data.RuleVar[839] == 0.0)
                          {
                            windowReturnClass.AddCommand(4, 29);
                            windowReturnClass.AddCommand(4, 66);
                          }
                          else
                          {
                            windowReturnClass.AddCommand(4, 12);
                            windowReturnClass.AddCommand(4, 68);
                            windowReturnClass.AddCommand(4, 69);
                            windowReturnClass.AddCommand(4, 67);
                            windowReturnClass.AddCommand(4, 9);
                          }
                          this.game.SelectX = num1;
                          this.game.SelectY = selectY1;
                          return windowReturnClass;
                        }
                        if (this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X == this.game.SelectX & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y == this.game.SelectY & this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map == this.game.EditObj.MapSelected & this.game.EditObj.OrderType != 48)
                        {
                          this.game.EditObj.FeedBackString = "Unit is already in this hex.";
                          if ( this.game.Data.RuleVar[839] == 0.0)
                          {
                            windowReturnClass.AddCommand(4, 29);
                            windowReturnClass.AddCommand(4, 66);
                          }
                          else
                          {
                            windowReturnClass.AddCommand(4, 12);
                            windowReturnClass.AddCommand(4, 68);
                            windowReturnClass.AddCommand(4, 69);
                            windowReturnClass.AddCommand(4, 67);
                            windowReturnClass.AddCommand(4, 9);
                          }
                          this.game.SelectX = num1;
                          this.game.SelectY = selectY1;
                          if (this.game.EditObj.OrderType == 1)
                            return windowReturnClass;
                        }
                        else if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.SelectX, this.game.SelectY] < 9999)
                        {
                          if (!(this.game.HandyFunctionsObj.HasUnitAirSF(this.game.EditObj.OrderUnit) | this.game.HandyFunctionsObj.HasUnitNavySF(this.game.EditObj.OrderUnit)) || this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location <= -1 || !(index10 == 0 & this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].StructuralPts < this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].StructuralPts) || Interaction.MsgBox( "Are you sure you want to move into a damaged location?", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") != MsgBoxResult.No)
                          {
                            this.game.EditObj.DoCardSlot = -1;
                            this.game.EditObj.AreaX = -1;
                            this.game.EditObj.AreaY = -1;
                            this.game.EditObj.AreaMap = -1;
                            OrderResult orderResult2 = this.game.ProcessingObj.ExecuteMovement(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Map, this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                            let mut lowestSpeed1: i32 =  this.game.HandyFunctionsObj.GetLowestSpeed(this.game.EditObj.OrderUnit, -1);
                            let mut lowestSpeed2: i32 =  this.game.HandyFunctionsObj.GetLowestSpeed(this.game.EditObj.OrderUnit, -1, true);
                            if (lowestSpeed1 > -1 & index10 == 0)
                            {
                              if (this.game.Data.SFObj[lowestSpeed2].MoveType == -1)
                              {
                                if (Strings.Len(this.game.Data.SFTypeObj[lowestSpeed1].MoveWAV) > 0)
                                  SoundMod.PlayAWave(this.game.AppPath + "sound/" + this.game.Data.SFTypeObj[lowestSpeed1].MoveWAV,  this.game.EditObj);
                              }
                              else if (Strings.Len(this.game.Data.TempString[900 + this.game.Data.SFObj[lowestSpeed2].MoveType]) > 0)
                                SoundMod.PlayAWave(this.game.AppPath + "sound/" + this.game.Data.TempString[900 + this.game.Data.SFObj[lowestSpeed2].MoveType],  this.game.EditObj);
                            }
                            if (orderResult2.BattleUnit == -1)
                            {
                              if (this.game.EditObj.OrderType != 48)
                                this.game.EditObj.OrderType = 0;
                              this.game.EditObj.UnitSelected = this.game.EditObj.OrderUnit;
                              if (orderResult2.OK)
                                this.game.EditObj.TempCoordList = orderResult2.CList;
                              if ( this.game.Data.RuleVar[839] == 0.0)
                              {
                                windowReturnClass.AddCommand(4, 44);
                                windowReturnClass.AddCommand(4, 20);
                                windowReturnClass.AddCommand(4, 18);
                                windowReturnClass.AddCommand(4, 12);
                                windowReturnClass.AddCommand(4, 29);
                                windowReturnClass.AddCommand(4, 66);
                              }
                              else
                              {
                                windowReturnClass.AddCommand(4, 12);
                                windowReturnClass.AddCommand(4, 68);
                                windowReturnClass.AddCommand(4, 69);
                                windowReturnClass.AddCommand(4, 67);
                                windowReturnClass.AddCommand(4, 9);
                              }
                              windowReturnClass.SetFlag(true);
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
                                  windowReturnClass.AddCommand(5, 10);
                                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                  windowReturnClass.SetFlag(true);
                                  this.game.EditObj.OrderType = 0;
                                  return windowReturnClass;
                                }
                                if (this.game.Data.ActionCardObj[this.game.EditObj.DoCardSlot].UnitSelect)
                                {
                                  this.game.ProcessingObj.PlayCardPreEvent(this.game.EditObj.DoCardSlot);
                                  this.game.EditObj.PopupValue = 3;
                                  windowReturnClass.AddCommand(5, 10);
                                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                  windowReturnClass.SetFlag(true);
                                  this.game.EditObj.OrderType = 0;
                                  return windowReturnClass;
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
                                  windowReturnClass.AddCommand(3, 4);
                                  this.game.EditObj.OrderType = 0;
                                  return windowReturnClass;
                                }
                                let mut num13: i32 =  0;
                                let mut locCounter: i32 =  this.game.Data.LocCounter;
                                for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
                                {
                                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
                                  {
                                    let mut index11: i32 =  0;
                                    do
                                    {
                                      if (this.game.Data.LocObj[locnr].Production[index11] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index11]).result)
                                      {
                                        num13 += 1;
                                        this.game.Data.LocObj[locnr].Production[index11] = -1;
                                        this.game.Data.LocObj[locnr].ProdPointRemainder[index11] = 0;
                                        this.game.Data.LocObj[locnr].ProdPercent[index11] = 0;
                                      }
                                      index11 += 1;
                                    }
                                    while (index11 <= 3);
                                  }
                                }
                                if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
                                {
                                  this.game.EditObj.PopupValue = 0;
                                  this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                                  windowReturnClass.AddCommand(5, 10);
                                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                  windowReturnClass.SetFlag(true);
                                  return windowReturnClass;
                                }
                              }
                            }
                            else
                            {
                              this.game.TempCombat = new CombatClass(this.game);
                              Coordinate Target = Coordinate::new();
                              Target.x = orderResult2.BattleX;
                              Target.y = orderResult2.BattleY;
                              Target.map = orderResult2.BattleMap;
                              this.game.EditObj.TempUnitList = UnitList::new();
                              this.game.EditObj.TempUnitList.add(orderResult2.BattleUnit);
                              this.game.EditObj.DoCardSlot = -1;
                              if (this.game.TempCombat.Init(Target, 1, this.game.EditObj.TempUnitList, this.game.EditObj.OrderType).OK)
                              {
                                if ( this.game.Data.RuleVar[839] == 1.0)
                                {
                                  this.game.EditObj.PopupValue = 7;
                                  windowReturnClass.AddCommand(5, 10);
                                  this.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(this.PopUpRefresh);
                                  windowReturnClass.SetFlag(true);
                                  return windowReturnClass;
                                }
                                windowReturnClass.AddCommand(3, 5);
                                windowReturnClass.SetFlag(true);
                                return windowReturnClass;
                              }
                              if ( this.game.Data.RuleVar[839] == 0.0)
                              {
                                windowReturnClass.AddCommand(4, 44);
                                windowReturnClass.AddCommand(4, 20);
                                windowReturnClass.AddCommand(4, 18);
                                windowReturnClass.AddCommand(4, 12);
                                windowReturnClass.AddCommand(4, 29);
                                windowReturnClass.AddCommand(4, 66);
                              }
                              else
                              {
                                windowReturnClass.AddCommand(4, 68);
                                windowReturnClass.AddCommand(4, 69);
                                windowReturnClass.AddCommand(4, 12);
                                windowReturnClass.AddCommand(4, 67);
                                windowReturnClass.AddCommand(4, 9);
                              }
                              windowReturnClass.SetFlag(true);
                              return windowReturnClass;
                            }
                          }
                          else
                            goto label_329;
                        }
                        else
                        {
                          if ( this.game.Data.RuleVar[839] == 0.0)
                          {
                            this.game.EditObj.FeedBackString = "Cannot reach that hex";
                            windowReturnClass.AddCommand(4, 29);
                            windowReturnClass.AddCommand(4, 12);
                          }
                          else
                          {
                            windowReturnClass.AddCommand(4, 12);
                            windowReturnClass.AddCommand(4, 67);
                            windowReturnClass.AddCommand(4, 68);
                            windowReturnClass.AddCommand(4, 9);
                          }
                          windowReturnClass.SetFlag(true);
                          return windowReturnClass;
                        }
                      }
                      this.game.EditObj.OrderType = 0;
                      return windowReturnClass;
                    }
                    if (this.game.EditObj.OrderType == 25)
                    {
                      if (this.game.EditObj.OrderSubType == 1 && this.game.HandyFunctionsObj.CanAddRoadToHex(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, this.game.EditObj.OrderUnit, this.game.EditObj.TempCoordList))
                      {
                        this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected);
                        windowReturnClass.AddCommand(4, 40);
                        windowReturnClass.AddCommand(4, 12);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      if (this.game.EditObj.OrderSubType == 2)
                        windowReturnClass.AddCommand(4, 40);
                      if (this.game.EditObj.OrderSubType == 3)
                        windowReturnClass.AddCommand(4, 40);
                      if (this.game.EditObj.OrderSubType == 4)
                        windowReturnClass.AddCommand(4, 40);
                      if (this.game.EditObj.OrderSubType == 5)
                        windowReturnClass.AddCommand(4, 40);
                    }
                    else if (this.game.EditObj.OrderType == 2)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 12)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 11)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 13)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 14)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 35)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 36)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 33)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 40)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                        windowReturnClass.AddCommand(4, 20);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 15)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 3)
                    {
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 9)
                    {
                      if (this.game.EditObj.UnitSelected > -1 && this.game.EditObj.OrderTarget != this.game.EditObj.UnitSelected && this.game.EditObj.UnitSelected != this.game.EditObj.OrderUnit && this.game.Data.Round > 0 & (this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime == this.game.Data.Turn |  this.game.Data.RuleVar[528] == 1.0 & this.game.Data.RegimeObj[this.game.Data.Turn].RegimeRel[this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Regime] == 2))
                      {
                        this.game.EditObj.OrderTarget = this.game.EditObj.UnitSelected;
                        windowReturnClass.AddCommand(4, 30);
                      }
                      if ( this.game.Data.RuleVar[839] == 0.0)
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 68);
                        windowReturnClass.AddCommand(4, 69);
                        windowReturnClass.AddCommand(4, 67);
                        windowReturnClass.AddCommand(4, 9);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 6)
                    {
                      if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location > -1 && !this.game.EditObj.InsideSlotty && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Regime == this.game.Data.Turn | this.game.Data.Round == 0 && this.game.Data.LocTypeObj[this.game.Data.LocObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location].Type].MaxProd > 0)
                      {
                        this.game.EditObj.OrderLoc = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 25);
                        if (!this.game.EditObj.ProdFlap)
                          windowReturnClass.AddCommand(4, 18);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 45)
                    {
                      if (this.game.EditObj.UnitSelected > -1 && this.game.Data.UnitObj[this.game.EditObj.UnitSelected].IsHQ)
                      {
                        this.game.EditObj.OrderUnit = this.game.EditObj.UnitSelected;
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 61);
                        windowReturnClass.AddCommand(4, 18);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 18 | this.game.EditObj.OrderType == 49)
                    {
                      if (this.game.EditObj.OrderUnit > -1 & this.game.EditObj.OrderTarget > -1)
                      {
                        this.game.EditObj.TargetX = this.game.SelectX;
                        this.game.EditObj.TargetY = this.game.SelectY;
                        windowReturnClass.AddCommand(4, 35);
                      }
                      else
                      {
                        windowReturnClass.AddCommand(4, 44);
                        windowReturnClass.AddCommand(4, 20);
                        windowReturnClass.AddCommand(4, 18);
                        windowReturnClass.AddCommand(4, 66);
                      }
                    }
                    else if (this.game.EditObj.OrderType == 34)
                    {
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 66);
                    }
                    else if (this.game.EditObj.OrderType == 19 | this.game.EditObj.OrderType == 42)
                    {
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 66);
                    }
                    else if (this.game.EditObj.OrderType == 20)
                    {
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 66);
                    }
                    else if (this.game.EditObj.OrderType == 21)
                    {
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 66);
                    }
                    else if (this.game.EditObj.OrderType == 4)
                    {
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 20);
                      windowReturnClass.AddCommand(4, 18);
                      windowReturnClass.AddCommand(4, 66);
                    }
                    else if (this.game.EditObj.OrderType == 5)
                    {
                      windowReturnClass.AddCommand(4, 44);
                      windowReturnClass.AddCommand(4, 20);
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
                  let mut x3: i32 =  this.game.EditObj.RightClickX;
                  let mut y3: i32 =  this.game.EditObj.RightCLickY;
                  map3: i32;
                  for (let mut map4: i32 =  this.game.EditObj.RightClickMap; this.game.EditObj.TempSupCameFrom[map4].Value[x3, y3].onmap; map4 = map3)
                  {
                    this.game.EditObj.SupplyPath.AddCoord(x3, y3, map4);
                    let mut x4: i32 =  this.game.EditObj.TempSupCameFrom[map4].Value[x3, y3].x;
                    let mut y4: i32 =  this.game.EditObj.TempSupCameFrom[map4].Value[x3, y3].y;
                    map3 = this.game.EditObj.TempSupCameFrom[map4].Value[x3, y3].map;
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
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 44);
                windowReturnClass.AddCommand(4, 12);
              }
              else if (this.game.EditObj.OrderType == 0 | this.game.EditObj.OrderType == 26 &&  this.game.Data.RuleVar[839] == 0.0)
              {
                windowReturnClass.AddCommand(4, 18);
                windowReturnClass.AddCommand(4, 20);
                windowReturnClass.AddCommand(4, 44);
              }
              if (!this.game.Data.PermanentOverlayUse | this.game.Data.PermanentOverlayUse & this.game.Data.Round > 0)
              {
                let mut subPart1: SubPartClass = this.SubPartList[index1];
                let mut x5: i32 =  num1;
                let mut y5: i32 =  selectY1;
                let mut map5: i32 =  mapSelected1;
                bitmap1: Bitmap = (Bitmap) null;
                 let mut local1: &Bitmap = &bitmap1;
                subPart1.PaintCoordinate((Graphics) null, x5, y5, map5, gBitmap: ( local1));
                bitmap2: Bitmap;
                if (this.game.Data.Round == 0)
                {
                  let mut subPart2: SubPartClass = this.SubPartList[index1];
                  let mut selectX: i32 =  this.game.SelectX;
                  let mut selectY2: i32 =  this.game.SelectY;
                  let mut mapSelected2: i32 =  this.game.EditObj.MapSelected;
                  bitmap3: Bitmap = (Bitmap) null;
                   let mut local2: &Bitmap = &bitmap3;
                  subPart2.PaintCoordinate((Graphics) null, selectX, selectY2, mapSelected2, gBitmap: ( local2));
                  let mut tfacing: i32 =  1;
                  do
                  {
                    Coordinate coordinate4 = this.game.HandyFunctionsObj.HexNeighbour(this.game.SelectX, this.game.SelectY, 0, tfacing);
                    if (coordinate4.onmap)
                    {
                      let mut subPart3: SubPartClass = this.SubPartList[index1];
                      let mut x6: i32 =  coordinate4.x;
                      let mut y6: i32 =  coordinate4.y;
                      let mut mapSelected3: i32 =  this.game.EditObj.MapSelected;
                      bitmap2 = (Bitmap) null;
                       let mut local3: &Bitmap = &bitmap2;
                      subPart3.PaintCoordinate((Graphics) null, x6, y6, mapSelected3, gBitmap: ( local3));
                    }
                    tfacing += 1;
                  }
                  while (tfacing <= 6);
                }
                else
                {
                  let mut subPart4: SubPartClass = this.SubPartList[index1];
                  let mut selectX: i32 =  this.game.SelectX;
                  let mut selectY3: i32 =  this.game.SelectY;
                  let mut mapSelected4: i32 =  this.game.EditObj.MapSelected;
                  bitmap4: Bitmap = (Bitmap) null;
                   let mut local4: &Bitmap = &bitmap4;
                  subPart4.PaintCoordinate((Graphics) null, selectX, selectY3, mapSelected4, gBitmap: ( local4));
                }
                if (this.game.Data.Round == 0 & this.game.EditObj.PencilType == 1)
                {
                  let mut tfacing: i32 =  1;
                  do
                  {
                    Coordinate coordinate5 = this.game.HandyFunctionsObj.HexNeighbourSameMap(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, tfacing);
                    if (coordinate5.onmap)
                    {
                      let mut subPart5: SubPartClass = this.SubPartList[index1];
                      let mut x7: i32 =  coordinate5.x;
                      let mut y7: i32 =  coordinate5.y;
                      let mut map6: i32 =  coordinate5.map;
                      bitmap2 = (Bitmap) null;
                       let mut local5: &Bitmap = &bitmap2;
                      subPart5.PaintCoordinate((Graphics) null, x7, y7, map6, gBitmap: ( local5));
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
                this.SubPartFlag[index1] = true;
              }
              windowReturnClass.SetFlag(true);
              if ( this.game.Data.RuleVar[839] == 0.0)
              {
                windowReturnClass.AddCommand(4, 66);
              }
              else
              {
                windowReturnClass.AddCommand(4, 67);
                windowReturnClass.AddCommand(4, 9);
              }
              return windowReturnClass;
            }
          }
        }
        windowReturnClass.SetFlag(false);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleMouseMove: WindowReturnClass(x: i32, y: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      currentDescript: String = this.game.EditObj.CurrentDescript;
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1] && this.SubPartID[index1] == this.MapId)
          {
            Coordinate coordinate = this.SubPartList[index1].ClickMap(x - this.SubPartX[index1], y - this.SubPartY[index1]);
            if (coordinate.onmap)
            {
              this.game.EditObj.MouseOverX = coordinate.x;
              this.game.EditObj.MouseOverY = coordinate.y;
              if (this.game.EditObj.OrderType == 0)
              {
                if (!this.game.EditObj.LayerSupplyOn)
                {
                  str1: String = this.game.HandyFunctionsObj.GetHexName(coordinate.x, coordinate.y, coordinate.map) + "(" + Strings.Trim(Conversion.Str( coordinate.x)) + "," + Strings.Trim(Conversion.Str( coordinate.y)) + ")";
                  str2: String = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].Regime != -1 ? str1 + " - " + this.game.Data.RegimeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].Regime].Name : str1 + " - Unoccupied Territory";
                  if (!this.game.EditObj.HideAS & this.game.Data.Round > 0)
                  {
                    let mut x1: i32 =  coordinate.x;
                    let mut y1: i32 =  coordinate.y;
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_SupplyLost(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_SupplyLost(this.game.Data.Turn);
                      str2 = str2 + ", SupplyLost = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_PowerPointsLost(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_PowerPointsLost(this.game.Data.Turn);
                      str2 = str2 + ", PowerPtsLost = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_SupplyKilled(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_SupplyKilled(this.game.Data.Turn);
                      str2 = str2 + ", SupplyKilled = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_PowerPointsKilled(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_PowerPointsKilled(this.game.Data.Turn);
                      str2 = str2 + ", PowerPtsKilled = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattleStack(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattleStack(this.game.Data.Turn);
                      str2 = str2 + ", BattleStack = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattleStackArt(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattleStackArt(this.game.Data.Turn);
                      str2 = str2 + ", BattleStackArt = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattleStackAir(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattleStackAir(this.game.Data.Turn);
                      str2 = str2 + ", BattleStackAir = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattlePenalty(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x1, y1].get_BattlePenalty(this.game.Data.Turn);
                      str2 = str2 + ", Battle AP Penalty = " + Strings.Trim(Conversion.Str( Number));
                    }
                  }
                  this.game.EditObj.CurrentDescript = str2;
                }
                else
                {
                  str3: String = Strings.Trim(Conversion.Str( this.game.EditObj.TempSup[coordinate.map].Value[coordinate.x, coordinate.y])) + "ap/" + Strings.Trim(Conversion.Str( this.game.Data.RuleVar[3]));
                  str4: String = "";
                  if ( this.game.EditObj.TempSup[coordinate.map].Value[coordinate.x, coordinate.y] >  this.game.Data.RuleVar[51])
                    str4 = ", 25% supply penalty";
                  if ( this.game.EditObj.TempSup[coordinate.map].Value[coordinate.x, coordinate.y] >  this.game.Data.RuleVar[52])
                    str4 = ", 50% supply penalty";
                  if ( this.game.EditObj.TempSup[coordinate.map].Value[coordinate.x, coordinate.y] >  this.game.Data.RuleVar[53])
                    str4 = ", 75% supply penalty";
                  if ( this.game.EditObj.TempSup[coordinate.map].Value[coordinate.x, coordinate.y] >  this.game.Data.RuleVar[3])
                    str4 = ", No supply";
                  if (this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea)
                  {
                    str3 = "Sea.";
                    str4 = "";
                  }
                  this.game.EditObj.CurrentDescript = str3 + str4;
                  let mut x2: i32 =  coordinate.x;
                  let mut y2: i32 =  coordinate.y;
                  if (this.game.Data.Round > 0)
                  {
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_SupplyLost(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_SupplyLost(this.game.Data.Turn);
                      editObj: EditClass = this.game.EditObj;
                      editObj.CurrentDescript = editObj.CurrentDescript + ", SupplyLost = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_PowerPointsLost(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_PowerPointsLost(this.game.Data.Turn);
                      editObj: EditClass = this.game.EditObj;
                      editObj.CurrentDescript = editObj.CurrentDescript + ", PowerPtsLost = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_SupplyKilled(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_SupplyKilled(this.game.Data.Turn);
                      editObj: EditClass = this.game.EditObj;
                      editObj.CurrentDescript = editObj.CurrentDescript + ", SupplyKilled = " + Strings.Trim(Conversion.Str( Number));
                    }
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_PowerPointsKilled(this.game.Data.Turn) > 0)
                    {
                      let mut Number: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[x2, y2].get_PowerPointsKilled(this.game.Data.Turn);
                      editObj: EditClass = this.game.EditObj;
                      editObj.CurrentDescript = editObj.CurrentDescript + ", PowerPtsKilled = " + Strings.Trim(Conversion.Str( Number));
                    }
                    this.game.EditObj.CurrentDescript += ", Right click a hex to see supply path";
                  }
                }
                if (this.game.Data.Round > 0 && this.game.Data.ShrowdOn & this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].get_SeeNow(this.game.Data.Turn) < 1)
                  this.game.EditObj.CurrentDescript = "Shrouded...";
              }
              else if ((this.game.EditObj.OrderType == 1 | this.game.EditObj.OrderType == 48) & this.game.Data.Round > 0)
              {
                if (this.game.EditObj.MouseOverX > -1 & this.game.EditObj.MouseOverY > -1)
                {
                  if (this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] < 999)
                  {
                    this.game.EditObj.CurrentDescript = "Move Cost = " + Conversion.Str( this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY]);
                    if (this.game.EditObj.OrderType == 1)
                    {
                      let mut integer: i32 =  Conversions.ToInteger(this.game.HandyFunctionsObj.UnitFuelPrognosis(this.game.EditObj.OrderUnit, this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY]));
                      if (integer > 0)
                      {
                        editObj: EditClass = this.game.EditObj;
                        editObj.CurrentDescript = editObj.CurrentDescript + " , Fuel Cost = " + Conversion.Str( integer);
                      }
                    }
                  }
                  else if (this.game.EditObj.TempValue2[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY] > 0)
                  {
                    this.game.EditObj.CurrentDescript = "Impossible to move here. Move Cost = " + Conversion.Str( this.game.EditObj.TempValue2[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY]);
                    if (this.game.EditObj.OrderType == 1)
                    {
                      let mut integer: i32 =  Conversions.ToInteger(this.game.HandyFunctionsObj.UnitFuelPrognosis(this.game.EditObj.OrderUnit, this.game.EditObj.TempValue[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY]));
                      if (integer > 0)
                      {
                        editObj: EditClass = this.game.EditObj;
                        editObj.CurrentDescript = editObj.CurrentDescript + " , Fuel Cost = " + Conversion.Str( integer);
                      }
                    }
                  }
                  else
                    this.game.EditObj.CurrentDescript = "Impossible to move here";
                  if (Strings.Len(this.game.EditObj.TempString[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY]) > 0)
                    this.game.EditObj.CurrentDescript = this.game.EditObj.TempString[this.game.EditObj.MapSelected].Value[this.game.EditObj.MouseOverX, this.game.EditObj.MouseOverY];
                }
              }
              else
              {
                if (this.game.EditObj.OrderType == 35)
                {
                  str: String = "To far away.";
                  let mut num: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map);
                  if (num > 0)
                  {
                    let mut index2: i32 =  num - 1;
                    str = "No river.";
                    if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[index2] > -1)
                    {
                      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Bridge[index2])
                      {
                        let mut Number1: i32 =  Conversion.Int(this.game.HandyFunctionsObj.GetBlowBridgePts(this.game.EditObj.OrderUnit));
                        let mut Number2: i32 =   Math.Round( Conversion.Int(this.game.Data.RuleVar[7] * this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[index2]].BridgeCostModifier));
                        str = "Unit has a chance of " + Conversion.Str( Number1) + " : " + Conversion.Str( Number2) + " to demolish this bridge. ";
                      }
                      else
                        str = "No bridge.";
                    }
                  }
                  this.game.EditObj.CurrentDescript = str;
                  if ( this.game.Data.RuleVar[839] == 0.0)
                    windowReturnClass.AddCommand(4, 29);
                  else
                    windowReturnClass.AddCommand(4, 44);
                  windowReturnClass.SetFlag(false);
                  windowReturnClass.SetOverlay(false);
                  return windowReturnClass;
                }
                if (this.game.EditObj.OrderType == 26)
                  this.game.EditObj.CurrentDescript = "The history map";
                else if (this.game.EditObj.OrderType == 36)
                {
                  str5: String = "To far away.";
                  let mut num: i32 =  this.game.HandyFunctionsObj.HexFacing(this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map);
                  if (num > -1)
                  {
                    let mut x3: i32 =  this.game.HandyFunctionsObj.MoveApCostPreview(this.game.EditObj.OrderUnit, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].X, this.game.Data.UnitObj[this.game.EditObj.OrderUnit].Y, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, coordinate.x, coordinate.y, coordinate.map, EngineerTest: true).x;
                    if (x3 <= this.game.HandyFunctionsObj.GetLowestAp(this.game.EditObj.OrderUnit))
                    {
                      let mut facing: i32 =  num - 1;
                      str6: String;
                      if ( this.game.Data.RuleVar[32] == -1.0)
                      {
                        if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing] > -1 & this.game.HandyFunctionsObj.CanConstructBridge(this.game.EditObj.OrderUnit, this.game.EditObj.OrderX, this.game.EditObj.OrderY, this.game.EditObj.OrderMap, facing))
                        {
                          if ( this.game.Data.RuleVar[822] == -1.0 |  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[ Math.Round( Math.Max(0.0f, this.game.Data.RuleVar[822]))] >=  this.game.Data.RuleVar[823] +  this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.RuleVar[825])
                          {
                            str6 =  this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.BridgeObj[0].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier >  this.game.HandyFunctionsObj.GetUnitEP(this.game.EditObj.OrderUnit) ? "Not enough EP. Bridge Cost: " + Conversion.Str(  ( this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.BridgeObj[0].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP" : "Bridge Cost: " + Conversion.Str(  ( this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.BridgeObj[0].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP";
                          }
                          else
                          {
                            str6 = "Not enough resources to build bridge.";
                            if ( this.game.Data.RuleVar[822] > -1.0)
                              str6 = str6 + ". " + Conversion.Str(  ( this.game.Data.RuleVar[823] +  this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.RuleVar[825])) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                          }
                        }
                        else
                          str6 = "Not a suitable location for a bridge.";
                      }
                      else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing] > -1 & !this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].Bridge[facing])
                      {
                        if ( this.game.Data.RuleVar[822] == -1.0 |  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[ Math.Round( Math.Max(0.0f, this.game.Data.RuleVar[822]))] >=  this.game.Data.RuleVar[823] +  this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.RuleVar[825])
                        {
                          if ( this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.BridgeObj[0].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier <=  this.game.HandyFunctionsObj.GetUnitEP(this.game.EditObj.OrderUnit))
                          {
                            str6 = "Bridge Cost: " + Conversion.Str(  ( this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.BridgeObj[0].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP";
                            if ( this.game.Data.RuleVar[822] > -1.0)
                              str6 = str6 + ". " + Conversion.Str(  ( this.game.Data.RuleVar[823] +  this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.RuleVar[825])) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                          }
                          else
                            str6 = "Not enough EP. Bridge Cost: " + Conversion.Str(  ( this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.BridgeObj[0].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP";
                        }
                        else
                        {
                          str6 = "Not enough resources to build bridge.";
                          if ( this.game.Data.RuleVar[822] > -1.0)
                            str6 = str6 + ". " + Conversion.Str(  ( this.game.Data.RuleVar[823] +  this.game.Data.RiverTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RiverType[facing]].BridgeCostModifier *  this.game.Data.RuleVar[825])) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                        }
                      }
                      else if (!this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].CanBuildRoad)
                        str6 = "You are not allowed to build roads on this landscape type";
                      else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RoadType[facing] == -1)
                      {
                        if ( this.game.Data.RuleVar[822] == -1.0 |  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[ Math.Round( Math.Max(0.0f, this.game.Data.RuleVar[822]))] >=  this.game.Data.RuleVar[823])
                        {
                          if ( this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[32])].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier <=  this.game.HandyFunctionsObj.GetUnitEP(this.game.EditObj.OrderUnit))
                          {
                            str6 = "Road Cost: " + Conversion.Str(  ( this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[32])].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP";
                            if ( this.game.Data.RuleVar[822] > -1.0)
                              str6 = str6 + ". " + Conversion.Str( this.game.Data.RuleVar[823]) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                          }
                          else
                            str6 = "Not enough EP. Road Cost: " + Conversion.Str(  ( this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[32])].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP";
                        }
                        else
                        {
                          str6 = "Not enough resources to construct road.";
                          if ( this.game.Data.RuleVar[822] > -1.0)
                            str6 = str6 + ". " + Conversion.Str( this.game.Data.RuleVar[823]) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                        }
                      }
                      else if ( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RoadType[facing] ==  this.game.Data.RuleVar[821])
                      {
                        if ( this.game.Data.RuleVar[822] == -1.0 |  this.game.Data.RegimeObj[this.game.Data.Turn].RegimeSlot[ Math.Round( Math.Max(0.0f, this.game.Data.RuleVar[822]))] >=  this.game.Data.RuleVar[824])
                        {
                          if ( this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[821])].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier <=  this.game.HandyFunctionsObj.GetUnitEP(this.game.EditObj.OrderUnit))
                          {
                            str6 = "Road Cost: " + Conversion.Str(  ( this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[821])].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP";
                            if ( this.game.Data.RuleVar[822] > -1.0)
                              str6 = str6 + ". " + Conversion.Str( this.game.Data.RuleVar[823]) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                          }
                          else
                            str6 = "Not enough EP. Road Cost: " + Conversion.Str(  ( this.game.Data.RoadTypeObj[ Math.Round( this.game.Data.RuleVar[821])].EPCost *  this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].RoadCostModifier)) + " EP";
                        }
                        else
                        {
                          str6 = "Not enough resources to construct road.";
                          if ( this.game.Data.RuleVar[822] > -1.0)
                            str6 = str6 + ". " + Conversion.Str( this.game.Data.RuleVar[823]) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                        }
                      }
                      else if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RoadType[facing] > -1 & !( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.EditObj.OrderX, this.game.EditObj.OrderY].RoadType[facing] ==  this.game.Data.RuleVar[821] &  this.game.Data.RuleVar[820] > -1.0))
                      {
                        str6 = "Already road/bridge here.";
                      }
                      else
                      {
                        str6 = "Not enough resources to construct road.";
                        if ( this.game.Data.RuleVar[822] > -1.0)
                          str6 = str6 + ". " + Conversion.Str( this.game.Data.RuleVar[823]) + "x " + this.game.Data.RegimeSlotName[ Math.Round( this.game.Data.RuleVar[822])];
                      }
                      str5 = str6 + " AP Cost = " + x3.ToString();
                    }
                    else
                      str5 = "Not enough AP. ";
                  }
                  this.game.EditObj.CurrentDescript = str5;
                }
              }
              if ( this.game.Data.RuleVar[839] == 0.0)
                windowReturnClass.AddCommand(4, 29);
              else if (Operators.CompareString(currentDescript, this.game.EditObj.CurrentDescript, false) != 0)
                windowReturnClass.AddCommand(4, 68);
              windowReturnClass.SetFlag(false);
              windowReturnClass.SetOverlay(false);
              return windowReturnClass;
            }
          }
        }
        windowReturnClass.SetFlag(false);
        windowReturnClass.SetOverlay(false);
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
        if (this.game.Data.PeopleCounter > 0)
        {
          str1: String = "\r\n";
          let mut peopleCounter: i32 =  this.game.Data.PeopleCounter;
          for (let mut index: i32 =  0; index <= peopleCounter; index += 1)
            str1 = str1 + index.ToString() + ") " + this.game.Data.PeopleObj[index].Name + "\r\n";
          str2: String = Conversions.ToString(Conversion.Val(Interaction.InputBox("Give People # for town..." + str1, "Shadow Empire : Planetary Conquest")));
          if (Operators.CompareString(str2, "", false) == 0)
            return  -1;
          num1 =  Math.Round(Conversion.Val(str2));
          if (num1 == -1 | num1 > this.game.Data.PeopleCounter)
          {
            let mut num2: i32 =   Interaction.MsgBox( "Invalid people#", Title: ( "Shadow Empire : Planetary Conquest"));
            return  -1;
          }
        }
        else
          num1 = 0;
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

    pub object heightLevelFill(newHeight: i32)
    {
      object[,] objArray = new object[this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1, this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight + 1];
      objArray[this.game.SelectX, this.game.SelectY] =  1;
      let mut landscapeType: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LandscapeType;
      let mut spriteNr: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SpriteNr;
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
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
              num1 = 1;
              this.game.Data.MapObj[mapSelected].HexObj[cx, cy].HeightLevel = newHeight;
              let mut tfacing: i32 =  1;
              do
              {
                Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbourSameMap(cx, cy, mapSelected, tfacing);
                if (coordinate.onmap && Operators.ConditionalCompareObjectEqual(objArray[coordinate.x, coordinate.y],  0, false) && this.game.Data.LandscapeTypeObj[this.game.Data.MapObj[mapSelected].HexObj[coordinate.x, coordinate.y].LandscapeType].IsSea == this.game.Data.LandscapeTypeObj[landscapeType].IsSea && this.game.Data.MapObj[mapSelected].HexObj[coordinate.x, coordinate.y].HeightLevel == spriteNr)
                  objArray[coordinate.x, coordinate.y] =  (Right + 1);
                tfacing += 1;
              }
              while (tfacing <= 6);
            }
          }
        }
      }
      let mut mapWidth1: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut cx: i32 =  0; cx <= mapWidth1; cx += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (let mut cy: i32 =  0; cy <= mapHeight; cy += 1)
        {
          let mut num2: i32 =  newHeight;
          let mut num3: i32 =  newHeight;
          let mut tfacing: i32 =  1;
          do
          {
            Coordinate coordinate = this.game.HandyFunctionsObj.HexNeighbour(cx, cy, 0, tfacing);
            if (coordinate.onmap)
            {
              if (this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel + 3 < num3)
                num3 = this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel + 3;
              if (this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel - 3 > num2)
                num2 = this.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].HeightLevel - 3;
            }
            tfacing += 1;
          }
          while (tfacing <= 6);
          if (num2 > newHeight)
            this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].HeightLevel = num2;
          else if (num3 < newHeight)
            this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[cx, cy].HeightLevel = num3;
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
      this.game.EditObj.AreaSlot = -1;
      this.game.EditObj.AreaX = -1;
      this.game.EditObj.AreaY = -1;
      this.game.EditObj.AreaMap = -1;
      this.game.EditObj.AreaValue = -1;
      this.DoRefresh();
    }
  }
}
