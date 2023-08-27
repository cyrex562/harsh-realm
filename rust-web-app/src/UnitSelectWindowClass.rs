// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UnitSelectWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class UnitSelectWindowClass : WindowClass
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
     tcx: i32;
     tcy: i32;
     tSelectX: i32;
     tSelectY: i32;
     tSelectMap: i32;
     tCornerX: i32;
     tCornerY: i32;

    pub UnitSelectWindowClass( tGame: GameClass)
      : base( tGame, 1024, 768, BackSprite: tGame.BACKGROUND2MARC)
    {
      self.FromMessage = tGame.EditObj.FromMessage;
      self.UnitSelected = self.game.EditObj.UnitSelected;
      self.tSelectX = self.game.SelectX;
      self.tSelectY = self.game.SelectY;
      self.tCornerX = self.game.CornerX;
      self.tCornerY = self.game.CornerY;
      self.tSelectMap = self.game.EditObj.MapSelected;
      self.game.SelectX = self.game.Data.MapObj[0].MapWidth;
      self.game.SelectY = self.game.Data.MapObj[0].MapHeight;
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      for (let mut index1: i32 = 0; index1 <= mapWidth; index1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (let mut index2: i32 = 0; index2 <= mapHeight; index2 += 1)
        {
          let mut unitCounter: i32 = self.game.Data.MapObj[0].HexObj[index1, index2].UnitCounter;
          for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
          {
            if (self.game.Data.UnitObj[self.game.Data.MapObj[0].HexObj[index1, index2].UnitList[index3]].TempUnitSelectable && index1 + index2 < self.game.SelectX + self.game.SelectY)
            {
              self.game.SelectX = index1;
              self.game.SelectY = index2;
            }
          }
        }
      }
      self.tcx = self.game.CornerX;
      self.tcy = self.game.CornerY;
      self.game.CornerX = self.game.SelectX - 5;
      self.game.CornerY = self.game.SelectY - 4;
      if (0 > self.game.CornerX)
        self.game.CornerX = 0;
      if (0 > self.game.CornerY)
        self.game.CornerY = 0;
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawTextVic2( Expression, "Select a highlighted unit", self.game.VicFont1, 50, 43, self.game.VicColor2, self.game.VicColor2Shade);
      if (self.game.EditObj.DoCardSlot > -1)
      {
         let mut local1: &Graphics = &Expression;
        bitmap: Bitmap = self.game.CustomBitmapObj.DrawActionCard(self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.game.EditObj.DoCardSlot]);
         let mut local2: &Bitmap = &bitmap;
        DrawMod.DrawScaled( local1,  local2, 685, 210, 267, 400);
      }
      else
      {
         let mut local3: &Graphics = &Expression;
        bitmap: Bitmap = self.game.CustomBitmapObj.DrawActionCard(self.game.EditObj.HandCard);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawScaled( local3,  local4, 685, 210, 267, 400);
      }
      DrawMod.DrawRectangle( Expression, 49, 79, 602, 442,  self.game.GameCol2.R,  self.game.GameCol2.G,  self.game.GameCol2.B,  byte.MaxValue);
      self.ViewMessage();
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
    }

    pub fn ViewMessage()
    {
      if (self.Pic1Id > 0)
        self.RemoveSubPart(self.Pic1Id);
      if (self.okid > 0)
      {
        self.RemoveSubPart(self.okid);
        self.okid = 0;
      }
      if (self.cancelid > 0)
      {
        self.RemoveSubPart(self.cancelid);
        self.cancelid = 0;
      }
      if (self.oktextid > 0)
        self.RemoveSubPart(self.oktextid);
      if (self.unitheaderid > 0)
        self.RemoveSubPart(self.unitheaderid);
      if (self.unitsfid > 0)
        self.RemoveSubPart(self.unitsfid);
      self.game.EditObj.TempCoordList = CoordList::new();
      SubPartClass tsubpart;
      if (self.mapid == 0)
      {
        tsubpart =  new MapPartClass(600, 440, self.game, ZoomLevel: 0);
        self.mapid = self.AddSubPart( tsubpart, 50, 80, 600, 440, 0);
      }
      if (self.game.EditObj.UnitSelected > -1)
      {
        tsubpart =  new UnitHeaderPartClass(self.game.EditObj.UnitSelected, self.game);
        self.unitheaderid = self.AddSubPart( tsubpart, 50, 540, 285, 200, 0);
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].TempUnitSelectable)
        {
          tsubpart =  new TextButtonPartClass("SELECT", 250, tBackbitmap: ( self.OwnBitmap), bbx: 695, bby: 627, theight: 50);
          self.okid = self.AddSubPart( tsubpart, 695, 627, 250, 50, 1);
        }
      }
      if (self.okid == 0)
      {
        tsubpart =  new TextButtonPartClass("SELECT", 250, tBackbitmap: ( self.OwnBitmap), bbx: 695, bby: 627, tinactive: true, theight: 50);
        self.oktextid = self.AddSubPart( tsubpart, 695, 627, 250, 50, 1);
      }
      tsubpart =  new TextButtonPartClass("Cancel", 200, tBackbitmap: ( self.OwnBitmap), bbx: 720, bby: 690);
      self.cancelid = self.AddSubPart( tsubpart, 720, 690, 200, 35, 1);
      tsubpart =  new MiniMapPartClass(self.game, tMapWidth: 600, tMapHeight: 440, ZoomLevel: 0);
      self.Pic1Id = self.AddSubPart( tsubpart, 720, 50, 200, 150, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.Pic1Id)
            {
              let mut selectX: i32 = self.game.SelectX;
              let mut selectY: i32 = self.game.SelectY;
              self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.ViewMessage();
              windowReturnClass.SetFlag(true);
            }
            else
            {
              if (num1 == self.mapid)
              {
                let mut selectX: i32 = self.game.SelectX;
                let mut selectY: i32 = self.game.SelectY;
                Coordinate coordinate = self.SubPartList[index1].ClickMap(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                if (coordinate.onmap)
                {
                  self.game.SelectX = coordinate.x;
                  self.game.SelectY = coordinate.y;
                  self.game.EditObj.SFSelected = -1;
                  let mut num2: i32 = !(selectX == self.game.SelectX & selectY == self.game.SelectY) ? self.game.HandyFunctionsObj.ClickOnHexGivesUnit(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected, true, b, coordinate.data1, coordinate.penalty, true) : self.game.HandyFunctionsObj.ClickOnHexGivesUnit(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected, false, b, coordinate.data1, coordinate.penalty, true);
                  if (selectX > -1)
                  {
                    let mut subPart: SubPartClass = self.SubPartList[index1];
                    let mut x1: i32 = selectX;
                    let mut y1: i32 = selectY;
                    let mut map: i32 = coordinate.map;
                    bitmap: Bitmap = (Bitmap) null;
                     let mut local: &Bitmap = &bitmap;
                    subPart.PaintCoordinate((Graphics) null, x1, y1, map, gBitmap: ( local));
                  }
                  self.game.EditObj.UnitSelected = num2;
                }
                self.ViewMessage();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.cancelid)
              {
                self.game.EditObj.AreaX = -1;
                self.game.EditObj.AreaY = -1;
                self.game.EditObj.DoCardSlot = -1;
                self.game.CornerX = self.tCornerX;
                self.game.CornerY = self.tCornerY;
                self.game.SelectX = self.tSelectX;
                self.game.SelectY = self.tSelectY;
                self.game.EditObj.MapSelected = self.tSelectMap;
                self.game.EditObj.TempCoordList = CoordList::new();
                self.game.EditObj.AreaSlot = -1;
                let mut unitCounter: i32 = self.game.Data.UnitCounter;
                for (let mut index2: i32 = 0; index2 <= unitCounter; index2 += 1)
                  self.game.Data.UnitObj[index2].TempUnitSelectable = false;
                self.game.EditObj.UnitSelected = self.UnitSelected;
                self.game.HandyFunctionsObj.RedimTempValue(9999);
                if (self.game.EditObj.OrderType == 23)
                {
                  self.game.EditObj.PopupValue = 14;
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.okid)
              {
                self.game.EditObj.AreaX = self.game.SelectX;
                self.game.EditObj.AreaY = self.game.SelectY;
                let mut unitCounter: i32 = self.game.Data.UnitCounter;
                for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
                  self.game.Data.UnitObj[index3].TempUnitSelectable = false;
                if (!(self.game.EditObj.DoCardSlot > -1 | self.game.EditObj.HandCard > -1))
                  return windowReturnClass;
                self.game.CornerX = self.tcx;
                self.game.CornerY = self.tcy;
                if (self.game.EditObj.AreaX == -1)
                {
                  self.game.EditObj.DoCardSlot = -1;
                  self.game.EditObj.AreaX = -1;
                  self.game.EditObj.HandCard = -1;
                  self.game.EditObj.AreaY = -1;
                  if (self.game.EditObj.OrderType == 23)
                  {
                    self.game.EditObj.PopupValue = 14;
                    windowReturnClass.AddCommand(5, 10);
                    windowReturnClass.SetFlag(true);
                    return windowReturnClass;
                  }
                  windowReturnClass.AddCommand(6, 0);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut messCounter: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter;
                if (self.game.EditObj.HandCard > -1)
                  self.game.ProcessingObj.PlayCardByUnit(self.UnitSelected, self.game.EditObj.HandCard);
                else if (self.game.EditObj.OrderType == 23)
                  self.game.ProcessingObj.PlayCard(self.game.Data.Turn, self.game.EditObj.DoCardSlot);
                else
                  self.game.ProcessingObj.PlayCard(self.game.EditObj.DoCardSlot);
                self.game.EditObj.AreaX = -1;
                self.game.EditObj.AreaSlot = -1;
                self.game.EditObj.HandCard = -1;
                if (Strings.Len(self.game.Data.LoadGame) > 0)
                {
                  self.game.FormRef.Cursor = Cursors.WaitCursor;
                  Form formRef =  self.game.FormRef;
                  self.game.HandyFunctionsObj.LoadGameNow();
                  self.game.FormRef = (Form1) formRef;
                  self.game.FormRef.Cursor = Cursors.Default;
                  windowReturnClass.AddCommand(3, 4);
                  return windowReturnClass;
                }
                let mut locCounter: i32 = self.game.Data.LocCounter;
                for (let mut locnr: i32 = 0; locnr <= locCounter; locnr += 1)
                {
                  if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y].Regime == self.game.Data.Turn)
                  {
                    let mut index4: i32 = 0;
                    do
                    {
                      if (self.game.Data.LocObj[locnr].Production[index4] > -1 && !self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, self.game.Data.LocObj[locnr].Production[index4]).result)
                      {
                        num3: i32;
                        num3 += 1;
                        self.game.Data.LocObj[locnr].Production[index4] = -1;
                        self.game.Data.LocObj[locnr].ProdPointRemainder[index4] = 0;
                        self.game.Data.LocObj[locnr].ProdPercent[index4] = 0;
                      }
                      index4 += 1;
                    }
                    while (index4 <= 3);
                  }
                }
                if (self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter > messCounter)
                {
                  self.game.EditObj.PopupValue = 0;
                  self.game.EditObj.FromMessage = self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter;
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                if (self.game.EditObj.OrderType == 23)
                {
                  self.game.EditObj.PopupValue = 14;
                  windowReturnClass.AddCommand(5, 10);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                self.game.EditObj.DoCardSlot = -1;
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
      if (self.game.FormRef.WindowState == FormWindowState.Minimized)
        return windowReturnClass;
      if (self.game.EditObj.ScrollDir == 1)
      {
        self.game.EditObj.ScrollDir = 0;
        return self.HandleKeyPress(38, false);
      }
      if (self.game.EditObj.ScrollDir == 2)
      {
        self.game.EditObj.ScrollDir = 0;
        return self.HandleKeyPress(39, false);
      }
      if (self.game.EditObj.ScrollDir == 3)
      {
        self.game.EditObj.ScrollDir = 0;
        return self.HandleKeyPress(40, false);
      }
      if (self.game.EditObj.ScrollDir == 4)
      {
        self.game.EditObj.ScrollDir = 0;
        return self.HandleKeyPress(37, false);
      }
      if (self.game.Data.Round > 0)
      {
        let mut subPart: SubPartClass = self.SubPartList[self.SubpartNr(self.mapid)];
        let mut selectX: i32 = self.game.SelectX;
        let mut selectY: i32 = self.game.SelectY;
        let mut mapSelected: i32 = self.game.EditObj.MapSelected;
        let mut counterAlpha: i32 = self.game.EditObj.CounterAlpha;
        bitmap: Bitmap = (Bitmap) null;
         let mut local: &Bitmap = &bitmap;
        subPart.PaintCoordinate((Graphics) null, selectX, selectY, mapSelected, counterAlpha,  local);
        self.PaintCurrentBitmap(self.mapid);
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
      let mut cornerX: i32 = self.game.CornerX;
      let mut cornerY: i32 = self.game.CornerY;
      if (nr == 39)
      {
        self += 1.game.CornerX;
        flag = true;
      }
      if (nr == 37 & (self.game.CornerX > 0 | self.game.Data.MapObj[self.game.EditObj.MapSelected].MapLoop))
      {
        --self.game.CornerX;
        if (0 > self.game.CornerX & !self.game.Data.MapObj[self.game.EditObj.MapSelected].MapLoop)
          self.game.CornerX = 0;
        flag = true;
      }
      if (nr == 40)
      {
        self += 1.game.CornerY;
        flag = true;
      }
      if (nr == 38 & self.game.CornerY > 0)
      {
        --self.game.CornerY;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
        flag = true;
      }
      if (nr == 27)
        return self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.cancelid)] + 1, self.SubPartY[self.SubpartNr(self.cancelid)] + 1, 1);
      if (nr == 32 & self.okid > 0)
        return self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.okid)] + 1, self.SubPartY[self.SubpartNr(self.okid)] + 1, 1);
      let mut num1: i32 = 230;
      if (self.game.Data.Round == 0)
        num1 += 100;
      let mut num2: i32 =  Math.Round(Conversion.Int( (self.OwnBitmap.Width - 250) /  (53 * (self.game.EditObj.Zoom + 1))));
      let mut num3: i32 =  Math.Round(Conversion.Int( (self.OwnBitmap.Height - num1) /  (48 * (self.game.EditObj.Zoom + 1))));
      let mut num4: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth - self.game.CornerX + 1;
      let mut num5: i32 = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight - self.game.CornerY + 1;
      if (num2 > num4 & !self.game.Data.MapObj[self.game.EditObj.MapSelected].MapLoop)
      {
        flag = true;
        self.game.CornerX = cornerX;
      }
      if (self.game.Data.MapObj[self.game.EditObj.MapSelected].MapLoop & 0 > self.game.CornerX)
        self.game.CornerX = self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + self.game.CornerX + 1;
      if (self.game.Data.MapObj[self.game.EditObj.MapSelected].MapLoop & self.game.CornerX > self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth)
        self.game.CornerX -= self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth + 1;
      if (num3 > num5)
      {
        flag = true;
        self.game.CornerY = cornerY;
      }
      if (self.game.CornerX == cornerX & self.game.CornerY == cornerY)
        flag = false;
      if (!flag)
        return windowReturnClass;
      if (nr == 39)
        self.SubPartList[self.SubpartNr(self.mapid)].ShiftLeft();
      if (nr == 37)
        self.SubPartList[self.SubpartNr(self.mapid)].ShiftRight();
      if (nr == 40)
        self.SubPartList[self.SubpartNr(self.mapid)].ShiftUp();
      if (nr == 38)
        self.SubPartList[self.SubpartNr(self.mapid)].ShiftDown();
      self.PaintCurrentBitmap(self.mapid);
      windowReturnClass.SetFlag(true);
      return windowReturnClass;
    }
  }
}
