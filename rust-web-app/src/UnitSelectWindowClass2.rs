// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.UnitSelectWindowClass2
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
  pub class UnitSelectWindowClass2 : WindowClass
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
     tempUnitHisId: i32;
     tempUnitHis2Id: i32;
     tcx: i32;
     tcy: i32;
     tSelectX: i32;
     tSelectY: i32;
     tSelectMap: i32;
     tCornerX: i32;
     tCornerY: i32;
     SimpleList UL;
     OptionsListId: i32;
     ListClass OptionsListObj;
     MapMatrix2 cacheTemp;
     MapMatrix2Coordinate cacheTempCameFrom;
     MapMatrix2 cacheTempValueSpecial;
     MapMatrix2 cacheTempValueSpecial2;
     MapMatrix2 cacheTemp2;
     MapMatrix2Plus6 cacheTempAttack;

    pub fn Close()
    {
      if (self.game.Data.Product != 6)
        return;
      self.game.EditObj.TempValue[0] = self.cacheTemp;
      self.game.EditObj.TempCameFrom[0] = self.cacheTempCameFrom;
      self.game.EditObj.TempValueSpecial[0] = self.cacheTempValueSpecial;
      self.game.EditObj.TempValueSpecial2[0] = self.cacheTempValueSpecial2;
      self.game.EditObj.TempValue2[0] = self.cacheTemp2;
      self.game.EditObj.TempAttack[0] = self.cacheTempAttack;
    }

    pub fn Dispose()
    {
      base.Dispose();
      DrawMod.TGame.EditObj.MapPopupMode = false;
    }

    pub UnitSelectWindowClass2( tGame: GameClass)
      : base( tGame, 1010, 700, 8)
    {
      self.FromMessage = tGame.EditObj.FromMessage;
      tGame.EditObj.MapPopupMode = true;
      if (Information.IsNothing( self.game.EditObj.TempValue[0]))
        self.game.HandyFunctionsObj.RedimTempValue(9999);
      if (Information.IsNothing( self.game.EditObj.TempValueSpecial[0]))
        self.game.HandyFunctionsObj.RedimTempValueSpecial(0);
      if (Information.IsNothing( self.game.EditObj.TempValueSpecial2[0]))
        self.game.HandyFunctionsObj.RedimTempValueSpecial2(0);
      if (Information.IsNothing( self.game.EditObj.TempValue2[0]))
        self.game.HandyFunctionsObj.RedimTempValue2(9999);
      if (Information.IsNothing( self.game.EditObj.TempCameFrom[0]))
        self.game.HandyFunctionsObj.RedimTempCameFrom();
      if (Information.IsNothing( self.game.EditObj.TempAttack[0]))
        self.game.HandyFunctionsObj.RedimTempAttack(true);
      self.cacheTemp = self.game.EditObj.TempValue[0].Clone();
      self.cacheTempCameFrom = self.game.EditObj.TempCameFrom[0].Clone();
      self.cacheTempValueSpecial = self.game.EditObj.TempValueSpecial[0].Clone();
      self.cacheTempValueSpecial2 = self.game.EditObj.TempValueSpecial2[0].Clone();
      self.cacheTemp2 = self.game.EditObj.TempValue2[0].Clone();
      self.cacheTempAttack = self.game.EditObj.TempAttack[0].Clone();
      self.UL = SimpleList::new();
      let mut unitCounter: i32 = self.game.Data.UnitCounter;
      for (let mut tid: i32 = 0; tid <= unitCounter; tid += 1)
      {
        if (self.game.Data.UnitObj[tid].TempUnitSelectable)
          self.UL.Add(tid, 0);
      }
      self.UnitSelected = self.game.EditObj.UnitSelected;
      self.tempUnitHisId = -1;
      self.tempUnitHis2Id = -1;
      if (self.game.EditObj.UnitSelected > -1)
      {
        self.tempUnitHisId = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical;
        self.tempUnitHis2Id = self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HistoricalSubPart;
      }
      self.tSelectX = self.game.SelectX;
      self.tSelectY = self.game.SelectY;
      self.tCornerX = self.game.CornerX;
      self.tCornerY = self.game.CornerY;
      self.tSelectMap = self.game.EditObj.MapSelected;
      self.tcx = self.game.CornerX;
      self.tcy = self.game.CornerY;
      self.game.CornerX = self.game.SelectX - 5;
      self.game.CornerY = self.game.SelectY - 4;
      if (self.game.CornerX > self.game.Data.MapObj[0].MapWidth - 15)
        self.game.CornerX -= self.game.CornerX - (self.game.Data.MapObj[0].MapWidth - 15);
      if (self.game.CornerY > self.game.Data.MapObj[0].MapHeight - 10)
        self.game.CornerY -= self.game.CornerY - (self.game.Data.MapObj[0].MapHeight - 10);
      if (0 > self.game.CornerX)
        self.game.CornerX = 0;
      if (0 > self.game.CornerY)
        self.game.CornerY = 0;
      self.game.SelectX = -1;
      self.game.SelectY = -1;
      self.game.EditObj.UnitSelected = -1;
      self.NewBackGroundAndClearAll(1010, 700, -1);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawMessFrame( self.OwnBitmap,  graphics, 0, 0, 1010, 700);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      bitmap1: Bitmap;
      if (self.game.EditObj.HandCard == -1)
      {
         let mut local1: &Graphics = &graphics;
        bitmap1 = self.game.CustomBitmapObj.DrawActionCardMarc2(self.game.Data.Turn, self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.game.EditObj.DoCardSlot], size: 2);
         let mut local2: &Bitmap = &bitmap1;
        DrawMod.DrawSimple( local1,  local2, 210, 538);
      }
      else
      {
         let mut local3: &Graphics = &graphics;
        bitmap2: Bitmap = self.game.CustomBitmapObj.DrawActionCardMarc2(self.game.Data.Turn, self.game.EditObj.HandCard, size: 2);
         let mut local4: &Bitmap = &bitmap2;
        DrawMod.DrawSimple( local3,  local4, 210, 538);
      }
       let mut local5: &Graphics = &graphics;
      bitmap1 = BitmapStore.GetBitmap(self.game.MARCMESFRAME);
       let mut local6: &Bitmap = &bitmap1;
      Rectangle rectangle1 = Rectangle::new(1, 300, 4, 42);
      let mut srcrect1: &Rectangle = &rectangle1
      Rectangle rectangle2 = Rectangle::new(844, 5, 4, 680);
      let mut destrect1: &Rectangle = &rectangle2
      DrawMod.DrawSimplePart2( local5,  local6, srcrect1, destrect1);
      self.OptionsListObj = ListClass::new();
      if (self.UL.Counter <= -1)
      {
        DrawMod.DrawTextColouredMarc( graphics, "No units", self.game.MarcFont3, 860, 11, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, "selectable.", self.game.MarcFont3, 860, 31, Color.White);
      }
      else if (self.UL.Counter > 12)
      {
        DrawMod.DrawTextColouredMarc( graphics, (self.UL.Counter + 1).ToString() + " units", self.game.MarcFont3, 860, 11, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, "selectable.", self.game.MarcFont3, 860, 31, Color.White);
      }
      else
      {
        DrawMod.DrawTextColouredMarc( graphics, "Selectable", self.game.MarcFont3, 860, 11, Color.White);
        DrawMod.DrawTextColouredMarc( graphics, "units:", self.game.MarcFont3, 860, 31, Color.White);
      }
       let mut local7: &Graphics = &graphics;
      bitmap1 = BitmapStore.GetBitmap(self.game.MARCMESFRAME);
       let mut local8: &Bitmap = &bitmap1;
      rectangle2 = Rectangle::new(32, 1, 10, 4);
      let mut srcrect2: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(4, 56, 842, 4);
      let mut destrect2: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local7,  local8, srcrect2, destrect2);
       let mut local9: &Graphics = &graphics;
      bitmap1 = BitmapStore.GetBitmap(self.game.MARCMESFRAME);
       let mut local10: &Bitmap = &bitmap1;
      rectangle2 = Rectangle::new(32, 385, 10, 4);
      let mut srcrect3: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(4, 531, 842, 4);
      let mut destrect3: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local9,  local10, srcrect3, destrect3);
       let mut local11: &Graphics = &graphics;
      bitmap1 = BitmapStore.GetBitmap(self.game.MARCMESFRAME);
       let mut local12: &Bitmap = &bitmap1;
      rectangle2 = Rectangle::new(1, 300, 4, 42);
      let mut srcrect4: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(205, 535, 4, 150);
      let mut destrect4: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local11,  local12, srcrect4, destrect4);
       let mut local13: &Graphics = &graphics;
      bitmap1 = BitmapStore.GetBitmap(self.game.MARCMESFRAME);
       let mut local14: &Bitmap = &bitmap1;
      rectangle2 = Rectangle::new(1, 300, 4, 42);
      let mut srcrect5: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(320, 535, 4, 150);
      let mut destrect5: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local13,  local14, srcrect5, destrect5);
       let mut local15: &Graphics = &graphics;
      bitmap1 = BitmapStore.GetBitmap(self.game.MARCMESFRAME);
       let mut local16: &Bitmap = &bitmap1;
      rectangle2 = Rectangle::new(1, 300, 4, 42);
      let mut srcrect6: &Rectangle = &rectangle2
      rectangle1 = Rectangle::new(615, 535, 4, 150);
      let mut destrect6: &Rectangle = &rectangle1
      DrawMod.DrawSimplePart2( local15,  local16, srcrect6, destrect6);
      if (self.UnitSelected == -1 | self.game.EditObj.HandCard == -1)
      {
        if (self.game.EditObj.HandCard > -1)
        {
           let mut local17: &Graphics = &graphics;
          bitmap1 = self.game.CustomBitmapObj.DrawActionCardMarc2(self.game.Data.Turn, self.game.EditObj.HandCard, size: 3);
           let mut local18: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local17,  local18, 10, 7);
        }
        else
        {
           let mut local19: &Graphics = &graphics;
          bitmap1 = self.game.CustomBitmapObj.DrawActionCardMarc2(self.game.Data.Turn, self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.game.EditObj.DoCardSlot], size: 3);
           let mut local20: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local19,  local20, 10, 7);
        }
        DrawMod.DrawTextColouredMarc( graphics, "Select a unit to play card on", self.game.MarcFont1, 50, 15, Color.White);
      }
      else
      {
        DrawMod.DrawTextColouredMarc( graphics, "Unit", self.game.MarcFont1, 10, 15, Color.White);
         let mut local21: &Graphics = &graphics;
        bitmap1 = self.game.CustomBitmapObj.DrawUnit(self.UnitSelected);
         let mut local22: &Bitmap = &bitmap1;
        DrawMod.DrawSimple( local21,  local22, 72, 12);
        DrawMod.DrawTextColouredMarc( graphics, "is playing card ", self.game.MarcFont1, 110, 15, Color.White);
        if (self.game.EditObj.HandCard == -1)
        {
           let mut local23: &Graphics = &graphics;
          bitmap1 = self.game.CustomBitmapObj.DrawActionCardMarc2(self.game.Data.Turn, self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.game.EditObj.DoCardSlot], size: 3);
           let mut local24: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local23,  local24, 290, 7);
        }
        else
        {
           let mut local25: &Graphics = &graphics;
          bitmap1 = self.game.CustomBitmapObj.DrawActionCardMarc2(self.game.Data.Turn, self.game.EditObj.HandCard, size: 3);
           let mut local26: &Bitmap = &bitmap1;
          DrawMod.DrawSimple( local25,  local26, 290, 7);
        }
        DrawMod.DrawTextColouredMarc( graphics, "You need to select a unit to play card on.", self.game.MarcFont1, 330, 15, Color.White);
      }
      self.game.EditObj.TempCoordList = CoordList::new();
      self.ViewMessage();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            self.SubPartList[index].DescriptInfo(x - self.SubPartX[index], y - self.SubPartY[index]);
            if (Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
            {
              self.game.EditObj.TipButton = true;
              self.game.EditObj.TipTitle = "";
              self.game.EditObj.TipText = self.SubPartList[index].Descript;
              return;
            }
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

    pub fn ViewMessage()
    {
      if (self.Pic1Id > 0)
      {
        self.RemoveSubPart(self.Pic1Id);
        self.Pic1Id = 0;
      }
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
      self.ClearMouse();
      Rectangle trect1;
      if (self.game.EditObj.HandCard > -1)
      {
        let mut handCard: i32 = self.game.EditObj.HandCard;
        if (self.game.Data.ActionCardObj[handCard].MouseOver.Length > 0)
        {
          trect1 = Rectangle::new(210, 538, 105, 147);
          self.AddMouse( trect1, "", self.game.Data.ActionCardObj[handCard].MouseOver);
        }
      }
      else
      {
        let mut index: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].ActionCard[self.game.EditObj.DoCardSlot];
        if (self.game.Data.ActionCardObj[index].MouseOver.Length > 0)
        {
          trect1 = Rectangle::new(210, 538, 105, 147);
          let mut trect2: &Rectangle = &trect1
          self.AddMouse( trect2, "", self.game.Data.ActionCardObj[index].MouseOver);
        }
      }
      if (self.mapid == 0)
      {
        let mut tsubpart: SubPartClass =  new MapPartClass(839, 470, self.game, ZoomLevel: 0, tFromPopupMap: true);
        self.mapid = self.AddSubPart( tsubpart, 5, 60, 839, 470, 0);
      }
      Graphics objgraphics = Graphics.FromImage((Image) self.OwnBitmap);
       let mut local1: &Graphics = &objgraphics;
       local2: Bitmap =  self.BackBitmap;
      trect1 = Rectangle::new(370, 540, 240, 120);
      let mut rect: &Rectangle = &trect1
      DrawMod.DrawSimplePart( local1,  local2, rect);
      if (self.game.EditObj.UnitSelected > -1)
      {
        DrawMod.DrawTextColouredMarc( objgraphics, "UNIT SELECTED:", self.game.MarcFont4, 370, 550, Color.White);
         let mut local3: &Graphics = &objgraphics;
        bitmap: Bitmap = self.game.CustomBitmapObj.DrawUnit(self.game.EditObj.UnitSelected);
         let mut local4: &Bitmap = &bitmap;
        DrawMod.DrawSimple( local3,  local4, 370, 575);
        DrawMod.DrawTextColouredMarc( objgraphics, self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Name, self.game.MarcFont4, 420, 585, Color.White);
        if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].TempUnitSelectable)
        {
          let mut tsubpart: SubPartClass =  new TextButtonPartClass("PLAY CARD", 200, "Click to play the card on the selected unit. [SPACE]",  self.BackBitmap, 630, 550, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.okid = self.AddSubPart( tsubpart, 630, 550, 200, 40, 1);
          DrawMod.DrawTextColouredMarc( objgraphics, "CAN PLAY CARD", self.game.MarcFont4, 370, 620, Color.White);
        }
        else
          DrawMod.DrawTextColouredMarc( objgraphics, "CANNOT PLAY CARD", self.game.MarcFont4, 370, 620, Color.Red);
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("CANCEL", 200, "Click to cancel and not play the card. [ESC]",  self.BackBitmap, 630, 610, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
      self.cancelid = self.AddSubPart( tsubpart1, 630, 610, 200, 40, 1);
      let mut tsubpart2: SubPartClass =  new MiniMapPartClass(self.game, tMapWidth: 834, tMapHeight: 470, ZoomLevel: 0);
      self.Pic1Id = self.AddSubPart( tsubpart2, 5, 535, 200, 150, 0);
      if (self.UL.Counter <= -1)
        return;
      self.OptionsListObj = ListClass::new();
      let mut num: i32 = -1;
      let mut tlistselect1: i32 = -1;
      let mut counter: i32 = self.UL.Counter;
      for (let mut index: i32 = 0; index <= counter; index += 1)
      {
        let mut tdata: i32 = self.UL.Id[index];
        num += 1;
        if (tdata == self.game.EditObj.UnitSelected)
          tlistselect1 = num;
        self.OptionsListObj.add(self.game.Data.UnitObj[tdata].Name, tdata);
      }
      if (self.OptionsListId <= 0)
      {
        ListClass optionsListObj = self.OptionsListObj;
        let mut tlistselect2: i32 = tlistselect1;
        let mut game: GameClass = self.game;
         local5: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local6: Font =  font;
        let mut tsubpart3: SubPartClass =  new ListSubPartClass(optionsListObj, 28, 142, tlistselect2, game, tdotopandbottom: false, tbackbitmap: ( local5), bbx: 850, bby: 55, tMarcStyle: true, overruleFont: ( local6), overruleItemSize: 20);
        self.OptionsListId = self.AddSubPart( tsubpart3, 850, 55, 142, 580, 0);
      }
      else
      {
        self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect1);
        self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
        self.SubPartList[self.SubpartNr(self.OptionsListId)].Paint();
      }
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
              self.game.EditObj.TempCoordList = CoordList::new();
              self.SubPartList[self.SubpartNr(self.mapid)].Paint();
              self.ViewMessage();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.SubpartNr(self.mapid);
                let mut index2: i32 = num2;
                self.game.SelectX = self.game.Data.UnitObj[index2].X;
                self.game.SelectY = self.game.Data.UnitObj[index2].Y;
                self.game.HandyFunctionsObj.CenterOnXY(self.game.SelectX, self.game.SelectY, useWidth: 834, useHeight: 470);
                if (self.game.EditObj.UnitSelected != index2)
                  self.game.EditObj.SFSelected = -1;
                self.game.EditObj.UnitSelected = index2;
                self.SubPartList[self.SubpartNr(self.mapid)].Paint();
                self.ViewMessage();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.mapid)
            {
              let mut selectX: i32 = self.game.SelectX;
              let mut selectY: i32 = self.game.SelectY;
              Coordinate coordinate = self.SubPartList[index1].ClickMap(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (coordinate.onmap)
              {
                self.game.SelectX = coordinate.x;
                self.game.SelectY = coordinate.y;
                self.game.EditObj.TempCoordList = CoordList::new();
                let mut num3: i32 = !(selectX == self.game.SelectX & selectY == self.game.SelectY) ? self.game.HandyFunctionsObj.ClickOnHexGivesUnit(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected, true, b, coordinate.data1, coordinate.penalty) : self.game.HandyFunctionsObj.ClickOnHexGivesUnit(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected, false, b, coordinate.data1, coordinate.penalty);
                if (self.game.EditObj.UnitSelected != num3)
                  self.game.EditObj.SFSelected = -1;
                self.game.EditObj.UnitSelected = num3;
                self.SubPartList[self.SubpartNr(self.mapid)].Paint();
                self.ViewMessage();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
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
              self.game.EditObj.UnitSelected = self.UnitSelected;
              self.game.EditObj.MapSelected = self.tSelectMap;
              self.game.EditObj.TempCoordList = CoordList::new();
              self.game.EditObj.AreaSlot = -1;
              let mut unitCounter: i32 = self.game.Data.UnitCounter;
              for (let mut index3: i32 = 0; index3 <= unitCounter; index3 += 1)
                self.game.Data.UnitObj[index3].TempUnitSelectable = false;
              self.game.EditObj.UnitSelected = self.UnitSelected;
              if (self.game.EditObj.UnitSelected > -1)
              {
                if (self.game.EditObj.UnitSelected > self.game.Data.UnitCounter)
                  self.game.EditObj.UnitSelected = -1;
                else if (!(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical == self.tempUnitHisId & self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HistoricalSubPart == self.tempUnitHis2Id))
                  self.game.EditObj.UnitSelected = -1;
              }
              let mut widthForMiniMap: i32 = DrawMod.GetWidthForMiniMap();
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: widthForMiniMap, ty: 200);
              self.game.HandyFunctionsObj.RedimTempValue(9999);
              self.Close();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.okid)
            {
              self.game.EditObj.AreaX = self.game.SelectX;
              self.game.EditObj.AreaY = self.game.SelectY;
              let mut unitCounter: i32 = self.game.Data.UnitCounter;
              for (let mut index4: i32 = 0; index4 <= unitCounter; index4 += 1)
                self.game.Data.UnitObj[index4].TempUnitSelectable = false;
              if (!(self.game.EditObj.DoCardSlot > -1 | self.game.EditObj.HandCard > -1))
                return windowReturnClass;
              self.game.CornerX = self.tcx;
              self.game.CornerY = self.tcy;
              let mut widthForMiniMap: i32 = DrawMod.GetWidthForMiniMap();
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: widthForMiniMap, ty: 200);
              if (self.game.EditObj.AreaX == -1)
              {
                self.game.EditObj.DoCardSlot = -1;
                self.game.EditObj.AreaX = -1;
                self.game.EditObj.HandCard = -1;
                self.game.EditObj.AreaY = -1;
                self.game.CornerX = self.tCornerX;
                self.game.CornerY = self.tCornerY;
                self.game.SelectX = self.tSelectX;
                self.game.SelectY = self.tSelectY;
                self.game.EditObj.UnitSelected = self.UnitSelected;
                if (self.game.EditObj.UnitSelected > -1)
                {
                  if (self.game.EditObj.UnitSelected > self.game.Data.UnitCounter)
                    self.game.EditObj.UnitSelected = -1;
                  else if (!(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical == self.tempUnitHisId & self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HistoricalSubPart == self.tempUnitHis2Id))
                    self.game.EditObj.UnitSelected = -1;
                }
                self.Close();
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut messCounter: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter;
              if (self.game.EditObj.HandCard > -1)
                self.game.ProcessingObj.PlayCardByUnit(self.UnitSelected, self.game.EditObj.HandCard);
              else
                self.game.ProcessingObj.PlayCard(self.game.Data.Turn, self.game.EditObj.DoCardSlot);
              self.game.EditObj.AreaX = -1;
              self.game.EditObj.AreaSlot = -1;
              self.game.EditObj.HandCard = -1;
              self.game.EditObj.DoCardSlot = -1;
              self.game.CornerX = self.tCornerX;
              self.game.CornerY = self.tCornerY;
              self.game.SelectX = self.tSelectX;
              self.game.SelectY = self.tSelectY;
              self.game.EditObj.UnitSelected = self.UnitSelected;
              if (self.game.EditObj.UnitSelected > -1)
              {
                if (self.game.EditObj.UnitSelected > self.game.Data.UnitCounter)
                  self.game.EditObj.UnitSelected = -1;
                else if (!(self.game.Data.UnitObj[self.game.EditObj.UnitSelected].Historical == self.tempUnitHisId & self.game.Data.UnitObj[self.game.EditObj.UnitSelected].HistoricalSubPart == self.tempUnitHis2Id))
                  self.game.EditObj.UnitSelected = -1;
              }
              if (Strings.Len(self.game.Data.LoadGame) > 0)
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                Form formRef =  self.game.FormRef;
                self.game.HandyFunctionsObj.LoadGameNow();
                self.game.FormRef = (Form1) formRef;
                self.game.FormRef.Cursor = Cursors.Default;
                windowReturnClass.AddCommand(3, 13);
                return windowReturnClass;
              }
              let mut locCounter: i32 = self.game.Data.LocCounter;
              Number: i32;
              for (let mut locnr: i32 = 0; locnr <= locCounter; locnr += 1)
              {
                if (self.game.Data.MapObj[self.game.EditObj.MapSelected].HexObj[self.game.Data.LocObj[locnr].X, self.game.Data.LocObj[locnr].Y].Regime == self.game.Data.Turn)
                {
                  let mut index5: i32 = 0;
                  do
                  {
                    if (self.game.Data.LocObj[locnr].Production[index5] > -1 && !self.game.HandyFunctionsObj.CanProduceItem(locnr, self.game.Data.Turn, self.game.Data.LocObj[locnr].Production[index5]).result)
                    {
                      Number += 1;
                      self.game.Data.LocObj[locnr].Production[index5] = -1;
                      self.game.Data.LocObj[locnr].ProdPointRemainder[index5] = 0;
                      self.game.Data.LocObj[locnr].ProdPercent[index5] = 0;
                    }
                    index5 += 1;
                  }
                  while (index5 <= 3);
                }
              }
              if ( self.game.Data.RuleVar[701] > 0.0 & self.game.Data.Product >= 6)
                self.game.EditObj.udsReturnFromPopup = true;
              if (Number > 0)
              {
                let mut num4: i32 =  Interaction.MsgBox( (Conversion.Str( Number) + " production lines have been cancelled due to this action card being played."), Title: ( "Shadow Empire : Planetary Conquest"));
              }
              if (self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter > messCounter)
              {
                self.game.EditObj.PopupValue = 0;
                self.game.EditObj.FromMessage = self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter;
                windowReturnClass.AddCommand(5, 14);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.Close();
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              self.game.EditObj.DoCardSlot = -1;
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
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }

    pub HandleKeyup: WindowReturnClass(nr: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      let mut cornerX: i32 = self.game.CornerX;
      let mut cornerY: i32 = self.game.CornerY;
      if (nr == 27)
        return self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.cancelid)] + 1, self.SubPartY[self.SubpartNr(self.cancelid)] + 1, 1);
      return nr == 32 & self.okid > 0 ? self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.okid)] + 1, self.SubPartY[self.SubpartNr(self.okid)] + 1, 1) : windowReturnClass;
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
      windowReturnClass.SetFlag(true);
      self.ViewMessage();
      return windowReturnClass;
    }
  }
}
