// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.MapSelectWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class MapSelectWindowClass : WindowClass
  {
     okid: i32;
     cancelid: i32;
     oktextid: i32;
     Pic1Id: i32;
     mapid: i32;
     FromMessage: i32;
     UnitSelected: i32;
     tcx: i32;
     tcy: i32;
     tSelectX: i32;
     tSelectY: i32;
     tSelectMap: i32;
     tCornerX: i32;
     tCornerY: i32;

    pub MapSelectWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      this.FromMessage = tGame.EditObj.FromMessage;
      this.tSelectX = this.game.SelectX;
      this.tSelectY = this.game.SelectY;
      this.tCornerX = this.game.CornerX;
      this.tCornerY = this.game.CornerY;
      this.tSelectMap = this.game.EditObj.MapSelected;
      let mut mapWidth: i32 =  this.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 =  this.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
        {
          if (this.game.Data.MapObj[0].HexObj[index1, index2].AreaCode[this.game.EditObj.AreaSlot] == this.game.EditObj.AreaValue && index1 + index2 < this.game.SelectX + this.game.SelectY)
          {
            this.game.SelectX = index1;
            this.game.SelectY = index2;
          }
        }
      }
      this.tcx = this.game.CornerX;
      this.tcy = this.game.CornerY;
      this.game.CornerX = this.game.SelectX - 5;
      this.game.CornerY = this.game.SelectY - 4;
      if (0 > this.game.CornerX)
        this.game.CornerX = 0;
      if (0 > this.game.CornerY)
        this.game.CornerY = 0;
      this.UnitSelected = this.game.EditObj.UnitSelected;
      Graphics Expression = Graphics.FromImage((Image) this.OwnBitmap);
      DrawMod.DrawTextVic2( Expression, "Select a highlighted hex", this.game.VicFont1, 50, 43, this.game.VicColor2, this.game.VicColor2Shade);
      if (this.game.EditObj.DoCardSlot > -1)
      {
         let mut local1: &Graphics = &Expression;
        bitmap: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.Data.RegimeObj[this.game.Data.Turn].ActionCard[this.game.EditObj.DoCardSlot]);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawScaled( local1,  local2, 685, 210, 267, 400);
      }
      else
      {
         let mut local3: &Graphics = &Expression;
        bitmap: Bitmap = this.game.CustomBitmapObj.DrawActionCard(this.game.EditObj.HandCard);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawScaled( local3,  local4, 685, 210, 267, 400);
      }
      DrawMod.DrawRectangle( Expression, 49, 79, 602, 582,  this.game.GameCol2.R,  this.game.GameCol2.G,  this.game.GameCol2.B,  byte.MaxValue);
      if (!Information.IsNothing( Expression))
        Expression.Dispose();
      this.ViewMessage();
    }

    pub fn ViewMessage()
    {
      if (this.Pic1Id > 0)
        this.RemoveSubPart(this.Pic1Id);
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
      if (this.oktextid > 0)
        this.RemoveSubPart(this.oktextid);
      if (this.mapid > 0)
        this.RemoveSubPart(this.mapid);
      this.game.EditObj.TempCoordList = CoordList::new();
      let mut tsubpart: SubPartClass =  new MapPartClass(600, 580, this.game, ZoomLevel: 0);
      this.mapid = this.AddSubPart( tsubpart, 50, 80, 600, 580, 0);
      if (this.game.SelectX > -1 & this.game.SelectY > -1 && this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].AreaCode[this.game.EditObj.AreaSlot] == this.game.EditObj.AreaValue)
      {
        tsubpart =  new TextButtonPartClass("SELECT", 250, tBackbitmap: ( this.OwnBitmap), bbx: 695, bby: 627, theight: 50);
        this.okid = this.AddSubPart( tsubpart, 695, 627, 250, 50, 1);
      }
      if (this.okid == 0)
      {
        tsubpart =  new TextButtonPartClass("SELECT", 250, tBackbitmap: ( this.OwnBitmap), bbx: 695, bby: 627, tinactive: true, theight: 50);
        this.oktextid = this.AddSubPart( tsubpart, 695, 627, 250, 50, 1);
      }
      tsubpart =  new TextButtonPartClass("Cancel", 200, tBackbitmap: ( this.OwnBitmap), bbx: 720, bby: 690);
      this.cancelid = this.AddSubPart( tsubpart, 720, 690, 200, 35, 1);
      tsubpart =  new MiniMapPartClass(this.game, tMapWidth: 600, tMapHeight: 580, ZoomLevel: 0);
      this.Pic1Id = this.AddSubPart( tsubpart, 720, 50, 200, 150, 0);
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
              this.ViewMessage();
              windowReturnClass.SetFlag(true);
            }
            else
            {
              if (num1 == this.mapid)
              {
                let mut selectX: i32 =  this.game.SelectX;
                let mut selectY: i32 =  this.game.SelectY;
                Coordinate coordinate = this.SubPartList[index1].ClickMap(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                if (coordinate.onmap)
                {
                  this.game.SelectX = coordinate.x;
                  this.game.SelectY = coordinate.y;
                  if (selectX > -1)
                  {
                    let mut subPart: SubPartClass = this.SubPartList[index1];
                    let mut x1: i32 =  selectX;
                    let mut y1: i32 =  selectY;
                    let mut map: i32 =  coordinate.map;
                    bitmap: Bitmap = (Bitmap) null;
                     let mut local: &Bitmap = &bitmap;
                    subPart.PaintCoordinate((Graphics) null, x1, y1, map, gBitmap: ( local));
                  }
                }
                this.ViewMessage();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.cancelid)
              {
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
                this.game.HandyFunctionsObj.RedimTempValue(9999);
                if (this.game.EditObj.OrderType == 23)
                {
                  this.game.EditObj.PopupValue = 14;
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.okid)
              {
                this.game.EditObj.AreaX = this.game.SelectX;
                this.game.EditObj.AreaY = this.game.SelectY;
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
                  if (this.game.EditObj.OrderType == 23)
                  {
                    this.game.EditObj.PopupValue = 14;
                    windowReturnClass.AddCommand(5, 10);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut messCounter: i32 =  this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                if (this.game.EditObj.HandCard > -1)
                  this.game.ProcessingObj.PlayCardByUnit(this.UnitSelected, this.game.EditObj.HandCard);
                else if (this.game.EditObj.OrderType == 23)
                  this.game.ProcessingObj.PlayCard(this.game.Data.Turn, this.game.EditObj.DoCardSlot);
                else
                  this.game.ProcessingObj.PlayCard(this.game.EditObj.DoCardSlot);
                this.game.EditObj.AreaX = -1;
                this.game.EditObj.AreaSlot = -1;
                this.game.EditObj.HandCard = -1;
                if (Strings.Len(this.game.Data.LoadGame) > 0)
                {
                  this.game.FormRef.Cursor = Cursors.WaitCursor;
                  Form formRef =  this.game.FormRef;
                  this.game.HandyFunctionsObj.LoadGameNow();
                  this.game.FormRef = (Form1) formRef;
                  this.game.FormRef.Cursor = Cursors.Default;
                  windowReturnClass.AddCommand(3, 4);
                  return windowReturnClass;
                }
                let mut locCounter: i32 =  this.game.Data.LocCounter;
                for (let mut locnr: i32 =  0; locnr <= locCounter; locnr += 1)
                {
                  if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.LocObj[locnr].X, this.game.Data.LocObj[locnr].Y].Regime == this.game.Data.Turn)
                  {
                    let mut index2: i32 =  0;
                    do
                    {
                      if (this.game.Data.LocObj[locnr].Production[index2] > -1 && !this.game.HandyFunctionsObj.CanProduceItem(locnr, this.game.Data.Turn, this.game.Data.LocObj[locnr].Production[index2]).result)
                      {
                        num2: i32;
                        num2 += 1;
                        this.game.Data.LocObj[locnr].Production[index2] = -1;
                        this.game.Data.LocObj[locnr].ProdPointRemainder[index2] = 0;
                        this.game.Data.LocObj[locnr].ProdPercent[index2] = 0;
                      }
                      index2 += 1;
                    }
                    while (index2 <= 3);
                  }
                }
                if (this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter > messCounter)
                {
                  this.game.EditObj.PopupValue = 0;
                  this.game.EditObj.FromMessage = this.game.Data.RegimeObj[this.game.Data.Turn].MessCounter;
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (this.game.EditObj.OrderType == 23)
                {
                  this.game.EditObj.PopupValue = 14;
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                this.game.EditObj.DoCardSlot = -1;
                return windowReturnClass;
              }
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
      if (this.game.Data.Round > 0)
      {
        let mut subPart: SubPartClass = this.SubPartList[this.SubpartNr(this.mapid)];
        let mut selectX: i32 =  this.game.SelectX;
        let mut selectY: i32 =  this.game.SelectY;
        let mut counterAlpha: i32 =  this.game.EditObj.CounterAlpha;
        bitmap: Bitmap = (Bitmap) null;
         let mut local: &Bitmap = &bitmap;
        subPart.PaintCoordinate((Graphics) null, selectX, selectY, counterAlpha, gBitmap: ( local));
        this.PaintCurrentBitmap(this.mapid);
        windowReturnClass.SetFlag(true);
        return windowReturnClass;
      }
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
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
      this.PaintCurrentBitmap(this.mapid);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }
  }
}
