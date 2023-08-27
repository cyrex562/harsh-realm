// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MapSelectWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class MapSelectWindowClass2 : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     unitheaderid: i32;
     unitsfid: i32;
     mapid: i32;
     FromMessage: i32;
     UnitSelected: i32;
     oldShowSetting: i32;
     tempUnitHisId: i32;
     tempUnitHis2Id: i32;
     tcx: i32;
     tcy: i32;
     bool UdsMode;
     tSelectX: i32;
     tSelectY: i32;
     tSelectMap: i32;
     tCornerX: i32;
     tCornerY: i32;
     MapMatrix2 cacheTemp;
     MapMatrix2Coordinate cacheTempCameFrom;
     MapMatrix2 cacheTempValueSpecial;
     MapMatrix2 cacheTempValueSpecial2;
     MapMatrix2 cacheTemp2;
     MapMatrix2Plus6 cacheTempAttack;

    pub fn Close()
    {
      if (this.game.Data.Product != 6)
        return;
      this.game.EditObj.TempValue[0] = this.cacheTemp;
      this.game.EditObj.TempCameFrom[0] = this.cacheTempCameFrom;
      this.game.EditObj.TempValueSpecial[0] = this.cacheTempValueSpecial;
      this.game.EditObj.TempValueSpecial2[0] = this.cacheTempValueSpecial2;
      this.game.EditObj.TempValue2[0] = this.cacheTemp2;
      this.game.EditObj.TempAttack[0] = this.cacheTempAttack;
    }

    pub MapSelectWindowClass2( tGame: GameClass)
      : base( tGame, 860, 700, 8)
    {
      if (Information.IsNothing( this.game.EditObj.TempValue[0]))
        this.game.HandyFunctionsObj.RedimTempValue(9999);
      if (Information.IsNothing( this.game.EditObj.TempValueSpecial[0]))
        this.game.HandyFunctionsObj.RedimTempValueSpecial(0);
      if (Information.IsNothing( this.game.EditObj.TempValueSpecial2[0]))
        this.game.HandyFunctionsObj.RedimTempValueSpecial2(0);
      if (Information.IsNothing( this.game.EditObj.TempValue2[0]))
        this.game.HandyFunctionsObj.RedimTempValue2(9999);
      if (Information.IsNothing( this.game.EditObj.TempCameFrom[0]))
        this.game.HandyFunctionsObj.RedimTempCameFrom();
      if (Information.IsNothing( this.game.EditObj.TempAttack[0]))
        this.game.HandyFunctionsObj.RedimTempAttack(true);
      this.cacheTemp = this.game.EditObj.TempValue[0].Clone();
      this.cacheTempCameFrom = this.game.EditObj.TempCameFrom[0].Clone();
      this.cacheTempValueSpecial = this.game.EditObj.TempValueSpecial[0].Clone();
      this.cacheTempValueSpecial2 = this.game.EditObj.TempValueSpecial2[0].Clone();
      this.cacheTemp2 = this.game.EditObj.TempValue2[0].Clone();
      this.cacheTempAttack = this.game.EditObj.TempAttack[0].Clone();
      this.FromMessage = tGame.EditObj.FromMessage;
      this.oldShowSetting = this.game.EditObj.HideUnit;
      this.game.EditObj.MapPopupMode = true;
      if (this.game.Data.Product < 6)
        this.game.EditObj.HideUnit = 0;
      this.UnitSelected = this.game.EditObj.UnitSelected;
      this.tempUnitHisId = -1;
      this.tempUnitHis2Id = -1;
      if (this.game.EditObj.UnitSelected > -1)
      {
        this.tempUnitHisId = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical;
        this.tempUnitHis2Id = this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HistoricalSubPart;
      }
      this.tSelectX = this.game.SelectX;
      this.tSelectY = this.game.SelectY;
      this.tCornerX = this.game.CornerX;
      this.tCornerY = this.game.CornerY;
      this.tSelectMap = this.game.EditObj.MapSelected;
      this.tcx = this.game.CornerX;
      this.tcy = this.game.CornerY;
      this.game.CornerX = this.game.SelectX - 5;
      this.game.CornerY = this.game.SelectY - 4;
      if (this.game.CornerX > this.game.Data.MapObj[0].MapWidth - 15)
        this.game.CornerX -= this.game.CornerX - (this.game.Data.MapObj[0].MapWidth - 15);
      if (this.game.CornerY > this.game.Data.MapObj[0].MapHeight - 10)
        this.game.CornerY -= this.game.CornerY - (this.game.Data.MapObj[0].MapHeight - 10);
      if (0 > this.game.CornerX)
        this.game.CornerX = 0;
      if (0 > this.game.CornerY)
        this.game.CornerY = 0;
      this.NewBackGroundAndClearAll(860, 700, -1);
      Graphics graphics = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawMessFrame( this.OwnBitmap,  graphics, 0, 0, 860, 700);
      this.BackBitmap = (Bitmap) this.OwnBitmap.Clone();
      this.UdsMode = false;
      if (this.game.EditObj.HandCard == -1 & this.game.EditObj.DoCardSlot == -1 & this.game.Data.Product > 6)
        this.UdsMode = true;
      bitmap: Bitmap;
      if (!this.UdsMode)
      {
        if (this.game.EditObj.HandCard == -1)
        {
           let mut local1: &Graphics = &graphics;
          bitmap = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.game.EditObj.DoCardSlot], size: 2);
           let mut local2: &Bitmap = &bitmap;
          DrawMod.DrawSimple( local1,  local2, 210, 538);
        }
        else
        {
           let mut local3: &Graphics = &graphics;
          bitmap = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.EditObj.HandCard, size: 2);
           let mut local4: &Bitmap = &bitmap;
          DrawMod.DrawSimple( local3,  local4, 210, 538);
        }
      }
       let mut local5: &Graphics = &graphics;
      bitmap = BitmapStore.GetBitmap(this.game.MARCMESFRAME);
       let mut local6: &Bitmap = &bitmap;
      Rectangle rectangle1 = Rectangle::new(32, 1, 10, 4);
      let mut srcrect1: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(4, 56, 842, 4);
      let mut destrect1: &Rectangle = &rectangle2
      DrawMod.DrawSimplePart2( local5,  local6, srcrect1, destrect1);
       let mut local7: &Graphics = &graphics;
      bitmap = BitmapStore.GetBitmap(this.game.MARCMESFRAME);
       let mut local8: &Bitmap = &bitmap;
      rectangle2 = Rectangle::new(32, 385, 10, 4);
      let mut srcrect2: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(4, 531, 842, 4);
      let mut destrect2: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local7,  local8, srcrect2, destrect2);
       let mut local9: &Graphics = &graphics;
      bitmap = BitmapStore.GetBitmap(this.game.MARCMESFRAME);
       let mut local10: &Bitmap = &bitmap;
      rectangle2 = Rectangle::new(1, 300, 4, 42);
      let mut srcrect3: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(205, 535, 4, 150);
      let mut destrect3: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local9,  local10, srcrect3, destrect3);
       let mut local11: &Graphics = &graphics;
      bitmap = BitmapStore.GetBitmap(this.game.MARCMESFRAME);
       let mut local12: &Bitmap = &bitmap;
      rectangle2 = Rectangle::new(1, 300, 4, 42);
      let mut srcrect4: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(320, 535, 4, 150);
      let mut destrect4: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local11,  local12, srcrect4, destrect4);
       let mut local13: &Graphics = &graphics;
      bitmap = BitmapStore.GetBitmap(this.game.MARCMESFRAME);
       let mut local14: &Bitmap = &bitmap;
      rectangle2 = Rectangle::new(1, 300, 4, 42);
      let mut srcrect5: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(615, 535, 4, 150);
      let mut destrect5: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local13,  local14, srcrect5, destrect5);
      if (!this.UdsMode)
      {
        if (this.UnitSelected == -1 | this.game.EditObj.HandCard == -1)
        {
          if (this.game.EditObj.HandCard > -1)
          {
             let mut local15: &Graphics = &graphics;
            bitmap = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.EditObj.HandCard, size: 3);
             let mut local16: &Bitmap = &bitmap;
            DrawMod.DrawSimple( local15,  local16, 10, 7);
          }
          else
          {
             let mut local17: &Graphics = &graphics;
            bitmap = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.game.EditObj.DoCardSlot], size: 3);
             let mut local18: &Bitmap = &bitmap;
            DrawMod.DrawSimple( local17,  local18, 10, 7);
          }
          DrawMod.DrawTextColouredMarc( graphics, "Select a hex to play card on", this.game.MarcFont1, 50, 15, Color.White);
        }
        else
        {
          DrawMod.DrawTextColouredMarc( graphics, "Unit", this.game.MarcFont1, 10, 15, Color.White);
           let mut local19: &Graphics = &graphics;
          bitmap = this.game.CustomBitmapObj.DrawUnit(this.UnitSelected);
           let mut local20: &Bitmap = &bitmap;
          DrawMod.DrawSimple( local19,  local20, 72, 12);
          DrawMod.DrawTextColouredMarc( graphics, "is playing card ", this.game.MarcFont1, 110, 15, Color.White);
          if (this.game.EditObj.HandCard == -1)
          {
             let mut local21: &Graphics = &graphics;
            bitmap = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.game.EditObj.DoCardSlot], size: 3);
             let mut local22: &Bitmap = &bitmap;
            DrawMod.DrawSimple( local21,  local22, 290, 7);
          }
          else
          {
             let mut local23: &Graphics = &graphics;
            bitmap = this.game.CustomBitmapObj.DrawActionCardMarc2(this.game.Data.Turn, this.game.EditObj.HandCard, size: 3);
             let mut local24: &Bitmap = &bitmap;
            DrawMod.DrawSimple( local23,  local24, 290, 7);
          }
          DrawMod.DrawTextColouredMarc( graphics, "You need to select a hex to play card on.", this.game.MarcFont1, 330, 15, Color.White);
        }
      }
      else
        DrawMod.DrawTextColouredMarc( graphics, this.game.EditObj.QuestionText, this.game.MarcFont1, 50, 15, Color.White);
      this.game.EditObj.TempCoordList = CoordList::new();
      this.ViewMessage();
    }

    pub fn Dispose()
    {
      base.Dispose();
      DrawMod.TGame.EditObj.MapPopupMode = false;
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            this.SubPartList[index].DescriptInfo(x - this.SubPartX[index], y - this.SubPartY[index]);
            if (Operators.CompareString(this.SubPartList[index].Descript, "", false) > 0)
            {
              this.game.EditObj.TipButton = true;
              this.game.EditObj.TipTitle = "";
              this.game.EditObj.TipText = this.SubPartList[index].Descript;
              return;
            }
          }
        }
      }
      let mut mouseCounter: i32 =  this.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
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

    pub fn ViewMessage()
    {
      if (this.Pic1Id > 0)
      {
        this.RemoveSubPart(this.Pic1Id);
        this.Pic1Id = 0;
      }
      if (this.okid > 0)
      {
        this.RemoveSubPart(this.okid);
        this.okid = 0;
      }
      if (this.cancelid > 0)
      {
        this.RemoveSubPart(this.cancelid);
        this.cancelid = 0;
      }
      this.ClearMouse();
      Rectangle trect1;
      if (!this.UdsMode)
      {
        if (this.game.EditObj.HandCard > -1)
        {
          let mut handCard: i32 =  this.game.EditObj.HandCard;
          if (this.game.Data.ActionCardObj[handCard].MouseOver.Length > 0)
          {
            trect1 = Rectangle::new(210, 538, 105, 147);
            this.AddMouse( trect1, "", this.game.Data.ActionCardObj[handCard].MouseOver);
          }
        }
        else
        {
          let mut index: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.game.EditObj.DoCardSlot];
          if (this.game.Data.ActionCardObj[index].MouseOver.Length > 0)
          {
            trect1 = Rectangle::new(210, 538, 105, 147);
            let mut trect2: &Rectangle = &trect1
            this.AddMouse( trect2, "", this.game.Data.ActionCardObj[index].MouseOver);
          }
        }
      }
      if (this.mapid == 0)
      {
        let mut tsubpart: SubPartClass =  new MapPartClass(839, 470, this.game, ZoomLevel: 0, tFromPopupMap: true);
        this.mapid = this.AddSubPart( tsubpart, 5, 60, 839, 470, 0);
      }
      Graphics objgraphics = Graphics.FromImage((Image) this.OwnBitmap);
       let mut local1: &Graphics = &objgraphics;
       local2: Bitmap =  this.BackBitmap;
      trect1 = Rectangle::new(370, 540, 240, 140);
      let mut rect: &Rectangle = &trect1
      DrawMod.DrawSimplePart( local1,  local2, rect);
      if (!this.UdsMode)
      {
        if (this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          DrawMod.DrawTextColouredMarc( objgraphics, "HEX SELECTED:", this.game.MarcFont4, 370, 550, Color.White);
           let mut local3: &Graphics = &objgraphics;
          customBitmapObj: CustomBitmapClass = this.game.CustomBitmapObj;
          let mut selectX: i32 =  this.game.SelectX;
          let mut selectY: i32 =  this.game.SelectY;
          bitmap1: Bitmap = (Bitmap) null;
           let mut local4: &Bitmap = &bitmap1;
          bool flag = false;
           bool local5 =  flag;
          bitmap2: Bitmap = customBitmapObj.DrawHex(selectX, selectY, 0, gBitmap: ( local4), tFromMapPopup: ( local5));
           let mut local6: &Bitmap = &bitmap2;
          DrawMod.DrawSimple( local3,  local6, 370, 575);
          DrawMod.DrawTextColouredMarc( objgraphics, this.game.HandyFunctionsObj.GetHexName(this.game.SelectX, this.game.SelectY, 0), this.game.MarcFont4, 440, 585, Color.White);
          if (this.game.EditObj.AreaSlot > -1)
          {
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].AreaCode[this.game.EditObj.AreaSlot] == this.game.EditObj.AreaValue)
            {
              let mut tsubpart: SubPartClass =  new TextButtonPartClass("PLAY CARD", 200, "Click to play the card on selected hex. [SPACE]",  this.BackBitmap, 630, 550, theight: 40, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
              this.okid = this.AddSubPart( tsubpart, 630, 550, 200, 40, 1);
              DrawMod.DrawTextColouredMarc( objgraphics, "CAN PLAY CARD", this.game.MarcFont4, 370, 640, Color.White);
            }
            else
              DrawMod.DrawTextColouredMarc( objgraphics, "CANNOT PLAY CARD", this.game.MarcFont4, 370, 640, Color.Red);
          }
        }
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("CANCEL", 200, "Click to cancel and not play the card. [ESC]",  this.BackBitmap, 630, 610, theight: 40, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.cancelid = this.AddSubPart( tsubpart1, 630, 610, 200, 40, 1);
      }
      else
      {
        if (this.game.SelectX > -1 & this.game.SelectY > -1)
        {
          DrawMod.DrawTextColouredMarc( objgraphics, "HEX SELECTED:", this.game.MarcFont4, 370, 550, Color.White);
           let mut local7: &Graphics = &objgraphics;
          customBitmapObj: CustomBitmapClass = this.game.CustomBitmapObj;
          let mut selectX: i32 =  this.game.SelectX;
          let mut selectY: i32 =  this.game.SelectY;
          bitmap3: Bitmap = (Bitmap) null;
           let mut local8: &Bitmap = &bitmap3;
          bool flag = false;
           bool local9 =  flag;
          bitmap4: Bitmap = customBitmapObj.DrawHex(selectX, selectY, 0, gBitmap: ( local8), tFromMapPopup: ( local9));
           let mut local10: &Bitmap = &bitmap4;
          DrawMod.DrawSimple( local7,  local10, 370, 575);
          DrawMod.DrawTextColouredMarc( objgraphics, this.game.HandyFunctionsObj.GetHexName(this.game.SelectX, this.game.SelectY, 0), this.game.MarcFont4, 440, 585, Color.White);
          if (this.game.EditObj.AreaSlot > -1)
          {
            if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].AreaCode[this.game.EditObj.AreaSlot] == this.game.EditObj.AreaValue)
            {
              let mut tsubpart: SubPartClass =  new TextButtonPartClass("CONFIRM", 200, "Click to confirm this hex as a destination [SPACE]",  this.BackBitmap, 630, 550, theight: 40, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
              this.okid = this.AddSubPart( tsubpart, 630, 550, 200, 40, 1);
              DrawMod.DrawTextColouredMarc( objgraphics, "CAN CONFIRM HEX", this.game.MarcFont4, 370, 640, Color.White);
            }
            else
              DrawMod.DrawTextColouredMarc( objgraphics, "CANNOT CONFIRM HEX", this.game.MarcFont4, 370, 640, Color.Red);
          }
        }
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("CANCEL", 200, "Click to cancel selection of destination hex [ESC]",  this.BackBitmap, 630, 610, theight: 40, usefont: this.game.MarcFont4, useshadow: true, tMarcStyle: true);
        this.cancelid = this.AddSubPart( tsubpart2, 630, 610, 200, 40, 1);
      }
      let mut tsubpart3: SubPartClass =  new MiniMapPartClass(this.game, tMapWidth: 834, tMapHeight: 470, ZoomLevel: 0);
      this.Pic1Id = this.AddSubPart( tsubpart3, 5, 535, 200, 150, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.Pic1Id)
            {
              let mut selectX: i32 =  this.game.SelectX;
              let mut selectY: i32 =  this.game.SelectY;
              this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.game.EditObj.TempCoordList = CoordList::new();
              this.SubPartList[this.SubpartNr(this.mapid)].Paint();
              this.ViewMessage();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.mapid)
            {
              let mut selectX: i32 =  this.game.SelectX;
              let mut selectY: i32 =  this.game.SelectY;
              Coordinate coordinate = this.SubPartList[index1].ClickMap(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              if (coordinate.onmap)
              {
                this.game.SelectX = coordinate.x;
                this.game.SelectY = coordinate.y;
                this.game.EditObj.TempCoordList.AddCoord(selectX, selectY, 0);
                this.game.EditObj.TempCoordList.AddCoord(this.game.SelectX, this.game.SelectY, 0);
              }
              this.SubPartList[this.SubpartNr(this.mapid)].Paint();
              this.ViewMessage();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.cancelid)
            {
              this.game.EditObj.HideUnit = this.oldShowSetting;
              this.game.EditObj.AreaX = -1;
              this.game.EditObj.AreaY = -1;
              this.game.EditObj.DoCardSlot = -1;
              this.game.CornerX = this.tCornerX;
              this.game.CornerY = this.tCornerY;
              this.game.SelectX = this.tSelectX;
              this.game.SelectY = this.tSelectY;
              this.game.EditObj.AreaSlot = -1;
              this.game.EditObj.MapSelected = this.tSelectMap;
              this.game.EditObj.TempCoordList = CoordList::new();
              this.game.EditObj.UnitSelected = this.UnitSelected;
              if (this.game.EditObj.UnitSelected > -1)
              {
                if (this.game.EditObj.UnitSelected > this.game.Data.UnitCounter)
                  this.game.EditObj.UnitSelected = -1;
                else if (!(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical == this.tempUnitHisId & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HistoricalSubPart == this.tempUnitHis2Id))
                  this.game.EditObj.UnitSelected = -1;
              }
              let mut widthForMiniMap: i32 =  DrawMod.GetWidthForMiniMap();
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: widthForMiniMap, ty: 200);
              this.game.HandyFunctionsObj.RedimTempValue(9999);
              this.Close();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.okid)
            {
              this.game.EditObj.HideUnit = this.oldShowSetting;
              this.game.EditObj.AreaX = this.game.SelectX;
              this.game.EditObj.AreaY = this.game.SelectY;
              let mut widthForMiniMap: i32 =  DrawMod.GetWidthForMiniMap();
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: widthForMiniMap, ty: 200);
              if (this.UdsMode)
              {
                this.game.EventRelatedObj.DoCheckSpecificEvent(this.game.EditObj.nextEventSlot);
                windowReturnClass.SetFlag(true);
                this.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                windowReturnClass.SetFlag(true);
                this.game.CornerX = this.tCornerX;
                this.game.CornerY = this.tCornerY;
                this.game.SelectX = this.tSelectX;
                this.game.SelectY = this.tSelectY;
                this.game.EditObj.UnitSelected = this.UnitSelected;
                return windowReturnClass;
              }
              if ( this.game.Data.RuleVar[408] > 0.0 & this.game.EditObj.DoCardSlot > -1)
              {
                this.game.ProcessingObj.PlayCard(this.game.Data.Turn, this.game.EditObj.DoCardSlot);
                windowReturnClass.SetFlag(true);
                this.game.EditObj.PopupValue = 21;
                windowReturnClass.AddCommand(5, 14);
                windowReturnClass.SetFlag(true);
                this.game.CornerX = this.tCornerX;
                this.game.CornerY = this.tCornerY;
                this.game.SelectX = this.tSelectX;
                this.game.SelectY = this.tSelectY;
                this.game.EditObj.UnitSelected = this.UnitSelected;
                return windowReturnClass;
              }
              if (!(this.game.EditObj.DoCardSlot > -1 | this.game.EditObj.HandCard > -1))
                return windowReturnClass;
              this.game.CornerX = this.tcx;
              this.game.CornerY = this.tcy;
              if (this.game.EditObj.AreaX == -1)
              {
                this.game.EditObj.DoCardSlot = -1;
                this.game.EditObj.AreaX = -1;
                this.game.EditObj.HandCard = -1;
                this.game.EditObj.AreaY = -1;
                this.game.CornerX = this.tCornerX;
                this.game.CornerY = this.tCornerY;
                this.game.SelectX = this.tSelectX;
                this.game.SelectY = this.tSelectY;
                this.game.EditObj.UnitSelected = this.UnitSelected;
                if (this.game.EditObj.UnitSelected > -1)
                {
                  if (this.game.EditObj.UnitSelected > this.game.Data.UnitCounter)
                    this.game.EditObj.UnitSelected = -1;
                  else if (!(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical == this.tempUnitHisId & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HistoricalSubPart == this.tempUnitHis2Id))
                    this.game.EditObj.UnitSelected = -1;
                }
                this.Close();
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
              if (this.game.EditObj.HandCard > -1)
                this.game.ProcessingObj.PlayCardByUnit(this.UnitSelected, this.game.EditObj.HandCard);
              else
                this.game.ProcessingObj.PlayCard(this.game.Data.Turn, this.game.EditObj.DoCardSlot);
              this.game.EditObj.AreaX = -1;
              this.game.EditObj.AreaSlot = -1;
              this.game.EditObj.HandCard = -1;
              this.game.EditObj.DoCardSlot = -1;
              this.game.CornerX = this.tCornerX;
              this.game.CornerY = this.tCornerY;
              this.game.SelectX = this.tSelectX;
              this.game.SelectY = this.tSelectY;
              this.game.EditObj.UnitSelected = this.UnitSelected;
              if (this.game.EditObj.UnitSelected > -1)
              {
                if (this.game.EditObj.UnitSelected > this.game.Data.UnitCounter)
                  this.game.EditObj.UnitSelected = -1;
                else if (!(this.game.Data.UnitObj[this.game.EditObj.UnitSelected].Historical == this.tempUnitHisId & this.game.Data.UnitObj[this.game.EditObj.UnitSelected].HistoricalSubPart == this.tempUnitHis2Id))
                  this.game.EditObj.UnitSelected = -1;
              }
              if (Strings.Len(this.game.Data.LoadGame) > 0)
              {
                this.game.FormRef.Cursor = Cursors.WaitCursor;
                Form formRef =  this.game.FormRef;
                this.game.HandyFunctionsObj.LoadGameNow();
                this.game.FormRef = (Form1) formRef;
                this.game.FormRef.Cursor = Cursors.Default;
                windowReturnClass.AddCommand(3, 13);
                return windowReturnClass;
              }
              let mut locCounter: i32 =  this.game.Data.LocCounter;
              Number: i32;
              for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
              {
                if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
                {
                  let mut index2: i32 =  0;
                  do
                  {
                    if (this.game.Data.LocObj[locnr].Production[index2] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index2]).result)
                    {
                      Number += 1;
                      this.game.Data.LocObj[locnr].Production[index2] = -1;
                      this.game.Data.LocObj[locnr].ProdPointRemainder[index2] = 0;
                      this.game.Data.LocObj[locnr].ProdPercent[index2] = 0;
                    }
                    index2 += 1;
                  }
                  while (index2 <= 3);
                }
              }
              if ( this.game.Data.RuleVar[701] > 0.0 & this.game.Data.Product >= 6)
                this.game.EditObj.udsReturnFromPopup = true;
              if (Number > 0)
              {
                let mut num2: i32 =   Interaction.MsgBox( (Conversion.Str( Number) + " production lines have been cancelled due to this action card being played."), Title: ( "Shadow Empire : Planetary Conquest"));
              }
              if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
              {
                this.game.EditObj.PopupValue = 0;
                this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                windowReturnClass.AddCommand(5, 14);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              this.Close();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              this.game.EditObj.DoCardSlot = -1;
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

    pub handleTimer: WindowReturnClass()
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.game.FormRef.WindowState == FormWindowState.Minimized)
        return windowReturnClass;
      if (this.game.EditObj.ScrollDir == 1)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(38, false);
      }
      if (this.game.EditObj.ScrollDir == 2)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(39, false);
      }
      if (this.game.EditObj.ScrollDir == 3)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(40, false);
      }
      if (this.game.EditObj.ScrollDir == 4)
      {
        this.game.EditObj.ScrollDir = 0;
        return this.HandleKeyPress(37, false);
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut cornerX: i32 =  this.game.CornerX;
      let mut cornerY: i32 =  this.game.CornerY;
      if (nr == 27)
        return this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.cancelid)] + 1, this.SubPartY[this.SubpartNr(this.cancelid)] + 1, 1);
      return nr == 32 & this.okid > 0 ? this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.okid)] + 1, this.SubPartY[this.SubpartNr(this.okid)] + 1, 1) : windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      bool flag = false;
      let mut cornerX: i32 =  this.game.CornerX;
      let mut cornerY: i32 =  this.game.CornerY;
      if (nr == 39)
      {
        this += 1.game.CornerX;
        flag = true;
      }
      if (nr == 37 & (this.game.CornerX > 0 | this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop))
      {
        --this.game.CornerX;
        if (0 > this.game.CornerX & !this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop)
          this.game.CornerX = 0;
        flag = true;
      }
      if (nr == 40)
      {
        this += 1.game.CornerY;
        flag = true;
      }
      if (nr == 38 & this.game.CornerY > 0)
      {
        --this.game.CornerY;
        if (this.game.CornerY < 0)
          this.game.CornerY = 0;
        flag = true;
      }
      let mut num1: i32 =  230;
      if (this.game.Data.Round == 0)
        num1 += 100;
      let mut num2: i32 =   Math.Round(Conversion.Int( (this.OwnBitmap.Width - 250) /  (53 * (this.game.EditObj.Zoom + 1))));
      let mut num3: i32 =   Math.Round(Conversion.Int( (this.OwnBitmap.Height - num1) /  (48 * (this.game.EditObj.Zoom + 1))));
      let mut num4: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth - this.game.CornerX + 1;
      let mut num5: i32 =  this.game.Data.MapObj[this.game.EditObj.MapSelected].MapHeight - this.game.CornerY + 1;
      if (num2 > num4 & !this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop)
      {
        flag = true;
        this.game.CornerX = cornerX;
      }
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & 0 > this.game.CornerX)
        this.game.CornerX = this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + this.game.CornerX + 1;
      if (this.game.Data.MapObj[this.game.EditObj.MapSelected].MapLoop & this.game.CornerX > this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth)
        this.game.CornerX -= this.game.Data.MapObj[this.game.EditObj.MapSelected].MapWidth + 1;
      if (num3 > num5)
      {
        flag = true;
        this.game.CornerY = cornerY;
      }
      if (this.game.CornerX == cornerX & this.game.CornerY == cornerY)
        flag = false;
      if (!flag)
        return windowReturnClass;
      if (nr == 39)
        this.SubPartList[this.SubpartNr(this.mapid)].ShiftLeft();
      if (nr == 37)
        this.SubPartList[this.SubpartNr(this.mapid)].ShiftRight();
      if (nr == 40)
        this.SubPartList[this.SubpartNr(this.mapid)].ShiftUp();
      if (nr == 38)
        this.SubPartList[this.SubpartNr(this.mapid)].ShiftDown();
      windowReturnClass.SetFlag(true);
      this.ViewMessage();
      return windowReturnClass;
    }
  }
}
