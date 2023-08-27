// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.TabReportsWindowClass2
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class TabReportsWindowClass2 : WindowClass
  {
     Info1Id: i32;
     info2id: i32;
     ShowString: String;
     DateTime ShowTime;
     w: i32;
     h: i32;
     detailnr: i32;
     CurrentView: i32;
     OptionsList5id: i32;
     Text1Id: i32;
     Text2Id: i32;
     ListClass OptionsList5Obj;
     okId: i32;

    pub TabReportsWindowClass2(
       tGame: GameClass,
       WindowClass tLowerWindow,
       Rectangle tLowerRect,
      Rectangle trect)
      : base( tGame, trect.Width, trect.Height, 8)
    {
      self.w = trect.Width;
      self.h = trect.Height;
      self.LowerWindow = tLowerWindow;
      self.LowerRect = tLowerRect;
      self.detailnr = -1;
      self.dostuff();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (nr == 40)
      {
        self.SubPartList[self.SubpartNr(self.OptionsList5id)].ShiftDown();
        self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
        self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsList5id)].GetSelect();
        if (self.Text1Id > 0)
        {
          self.RemoveSubPart(self.Text1Id);
          self.Text1Id = 0;
        }
        if (self.Text2Id > 0)
        {
          self.RemoveSubPart(self.Text2Id);
          self.Text2Id = 0;
        }
        self.dostuff();
        windowReturnClass.SetFlag(true);
      }
      if (nr == 38)
      {
        self.SubPartList[self.SubpartNr(self.OptionsList5id)].ShiftUp();
        self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
        self.detailnr = self.SubPartList[self.SubpartNr(self.OptionsList5id)].GetSelect();
        if (self.Text1Id > 0)
        {
          self.RemoveSubPart(self.Text1Id);
          self.Text1Id = 0;
        }
        if (self.Text2Id > 0)
        {
          self.RemoveSubPart(self.Text2Id);
          self.Text2Id = 0;
        }
        self.dostuff();
        windowReturnClass.SetFlag(true);
      }
      if (self.Text2Id > 0)
      {
        if (nr == 39)
        {
          self.SubPartList[self.SubpartNr(self.Text2Id)].ShiftDown();
          self.SubPartFlag[self.SubpartNr(self.Text2Id)] = true;
          windowReturnClass.SetFlag(true);
        }
        if (nr == 37)
        {
          self.SubPartList[self.SubpartNr(self.Text2Id)].ShiftUp();
          self.SubPartFlag[self.SubpartNr(self.Text2Id)] = true;
          windowReturnClass.SetFlag(true);
        }
      }
      return windowReturnClass;
    }

    pub fn DoRefresh() => self.dostuff();

    pub fn PopUpRefresh() => self.DoRefresh();

    pub fn dostuff()
    {
      if (self.okId > 0)
      {
        self.RemoveSubPart(self.okId);
        self.okId = 0;
      }
      self.NewBackGroundAndClearAll(self.w, self.h, -1);
      Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
      Rectangle trect1 = DrawMod.DrawBackTab(g, self.w, self.h, "REPS", 4);
      self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
      self.ClearMouse();
      self.AddMouse( trect1, "CLOSE TAB", "Click here to close this tab. [ESC/F5]", 999);
      self.OptionsList5Obj = ListClass::new();
      let mut tlistselect1: i32 = -1;
      let mut num1: i32 = -1;
      let mut num2: i32 = 0;
      if (self.h > 380)
        num2 = self.h - 380;
      let mut messCounter: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].MessCounter;
      for (let mut tdata: i32 = 0; tdata <= messCounter; tdata += 1)
      {
        if (self.game.Data.RegimeObj[self.game.Data.Turn].MesName[tdata] != null & self.game.Data.RegimeObj[self.game.Data.Turn].MessBackPic[tdata] == -2 & !self.game.Data.RegimeObj[self.game.Data.Turn].MesHideFromTab[tdata])
        {
          num1 += 1;
          if (self.detailnr == -1)
            self.detailnr = 0;
          if (self.detailnr == tdata)
            tlistselect1 = num1;
          str: String;
          if (self.game.Data.RegimeObj[self.game.Data.Turn].MesName[tdata].Length > 1)
          {
            str = self.game.Data.RegimeObj[self.game.Data.Turn].MesName[tdata];
          }
          else
          {
            let mut num3: i32 = Strings.InStr(self.game.Data.RegimeObj[self.game.Data.Turn].MessString[tdata], "\r\n");
            if (Information.IsNothing( num3) | num3 <= 0)
            {
              str = Strings.Left(self.game.Data.RegimeObj[self.game.Data.Turn].MessString[tdata], 50) + "...";
            }
            else
            {
              str = Strings.Left(self.game.Data.RegimeObj[self.game.Data.Turn].MessString[tdata], num3);
              if (Strings.Len(str) > 50)
                str = Strings.Left(str, 50) + "...";
            }
          }
          bool flag = false;
          DynamicData dynamicData = new DynamicData(self.game.Data.RegimeObj[self.game.Data.Turn].MessString[tdata]);
          let mut elementCounter: i32 = dynamicData.elementCounter;
          for (let mut index: i32 = 0; index <= elementCounter; index += 1)
          {
            if (dynamicData.element[index].type == DynamicType.OptionField)
              flag = true;
          }
          if (flag)
          {
            if (self.game.Data.RegimeObj[self.game.Data.Turn].MesChosen[tdata] == 0)
              self.OptionsList5Obj.add(str, tdata, tbmp: BitmapStore.GetBitmap(self.game.SMALLCHAR1));
            else
              self.OptionsList5Obj.add(str, tdata, tbmp: BitmapStore.GetBitmap(self.game.SMALLCHAR2));
          }
          else
            self.OptionsList5Obj.add(str, tdata);
        }
      }
      if (self.OptionsList5Obj.ListCount > -1)
      {
        SubPartClass tsubpart;
        if (self.OptionsList5id > 0)
        {
          self.SubPartList[self.SubpartNr(self.OptionsList5id)].Refresh(self.OptionsList5Obj, tlistselect1);
          self.SubPartFlag[self.SubpartNr(self.OptionsList5id)] = true;
        }
        else
        {
          ListClass optionsList5Obj = self.OptionsList5Obj;
          let mut tlistsize: i32 =  Math.Round(9.0 + Conversion.Int( num2 / 16.0));
          let mut tlistselect2: i32 = tlistselect1;
          let mut game: GameClass = self.game;
           local1: Bitmap =  self.OwnBitmap;
          font: Font =  null;
           local2: Font =  font;
          tsubpart =  new ListSubPartClass(optionsList5Obj, tlistsize, 250, tlistselect2, game, tHeaderCenter: false, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 25, bby: 17, tMarcStyle: true, overruleFont: ( local2));
          self.OptionsList5id = self.AddSubPart( tsubpart, 25, 17, 250,  Math.Round((10.0 + Conversion.Int( num2 / 16.0)) * 16.0), 0);
        }
        Rectangle rectangle = Rectangle::new(25, 17, 250,  Math.Round((10.0 + Conversion.Int( num2 / 16.0)) * 16.0));
        let mut trect2: &Rectangle = &rectangle
        self.AddMouse( trect2, "List of messages", "Click on a message to read full text");
        if (self.detailnr <= -1)
          return;
        if (self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.detailnr] > -1)
        {
          let mut commanderSpriteId: i32 = self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.detailnr];
          if (commanderSpriteId >= 10000)
            commanderSpriteId = self.game.Data.HistoricalUnitObj[commanderSpriteId - 10000].CommanderSpriteID;
          else if (commanderSpriteId <= self.game.Data.EventPicCounter)
            commanderSpriteId = self.game.Data.EventPicNr[commanderSpriteId];
          if (commanderSpriteId > -1)
          {
            let mut num4: i32 = BitmapStore.GetWidth(commanderSpriteId);
            let mut num5: i32 = BitmapStore.Getheight(commanderSpriteId);
            if (num4 > 250)
            {
              num5 =  Math.Round( num5 * (250.0 /  num4));
              num4 =  Math.Round( num4 * (250.0 /  num4));
            }
            if (num5 > 120)
            {
              num4 =  Math.Round( num4 * (120.0 /  num5));
              num5 =  Math.Round( num5 * (120.0 /  num5));
            }
            if (num4 > 64)
            {
              if (self.game.Data.RegimeObj[self.game.Data.Turn].MesNote2[self.detailnr].Length > 0)
              {
                 let mut local3: &Graphics = &g;
                bitmap: Bitmap = BitmapStore.GetBitmap(commanderSpriteId);
                 let mut local4: &Bitmap = &bitmap;
                trect2 = Rectangle::new(0, 0, BitmapStore.GetWidth(commanderSpriteId), BitmapStore.Getheight(commanderSpriteId));
                let mut srcrect: &Rectangle = &trect2
                rectangle = Rectangle::new(25, 215 + num2, num4, num5);
                let mut destrect: &Rectangle = &rectangle
                DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
              }
              else
              {
                 let mut local5: &Graphics = &g;
                bitmap: Bitmap = BitmapStore.GetBitmap(commanderSpriteId);
                 let mut local6: &Bitmap = &bitmap;
                trect2 = Rectangle::new(0, 0, BitmapStore.GetWidth(commanderSpriteId), BitmapStore.Getheight(commanderSpriteId));
                let mut srcrect: &Rectangle = &trect2
                rectangle = Rectangle::new(25, 200 + num2, num4, num5);
                let mut destrect: &Rectangle = &rectangle
                DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
              }
              if (self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.detailnr] >= 10000)
              {
                let mut overdrawSpriteId: i32 = self.game.Data.HistoricalUnitObj[self.game.Data.RegimeObj[self.game.Data.Turn].MessFrontPic[self.detailnr] - 10000].OverdrawSpriteID;
                if (self.game.Data.RegimeObj[self.game.Data.Turn].MesNote2[self.detailnr].Length > 0)
                {
                  if (overdrawSpriteId > -1)
                  {
                     let mut local7: &Graphics = &g;
                    bitmap: Bitmap = BitmapStore.GetBitmap(overdrawSpriteId);
                     let mut local8: &Bitmap = &bitmap;
                    let mut y: i32 = 215 + num2;
                    let mut w: i32 = num4;
                    let mut h: i32 = num5;
                    DrawMod.DrawScaled( local7,  local8, 25, y, w, h);
                  }
                }
                else if (overdrawSpriteId > -1)
                {
                   let mut local9: &Graphics = &g;
                  bitmap: Bitmap = BitmapStore.GetBitmap(overdrawSpriteId);
                   let mut local10: &Bitmap = &bitmap;
                  let mut y: i32 = 200 + num2;
                  let mut w: i32 = num4;
                  let mut h: i32 = num5;
                  DrawMod.DrawScaled( local9,  local10, 25, y, w, h);
                }
              }
              if (self.game.Data.RegimeObj[self.game.Data.Turn].MesNote2[self.detailnr].Length > 0)
                DrawMod.DrawTextColouredMarc( g, self.game.Data.RegimeObj[self.game.Data.Turn].MesNote2[self.detailnr], self.game.MarcFont7, 25, 190 + num2, Color.White);
              else
                DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, 25, 200 + num2, num4, num5, 25, 200);
              let mut num6: i32 = num5 + 10;
            }
          }
        }
        if (self.game.Data.RegimeObj[self.game.Data.Turn].MesStyle[self.detailnr] == 3)
        {
          if (self.Text2Id == 0)
          {
            tsubpart =  new DynamicArea(self.game, 565, self.h - 100, self.game.Data.RegimeObj[self.game.Data.Turn].MessString[self.detailnr],  self.OwnBitmap, 294, 0);
            self.Text2Id = self.AddSubPart( tsubpart, 294, 0, 565, self.h - 100, 0);
          }
          else
            self.SubPartFlag[self.SubpartNr(self.Text2Id)] = true;
          if (Strings.InStr(self.game.Data.RegimeObj[self.game.Data.Turn].MessString[self.detailnr].ToLower(), "[type]option[/type]".ToLower()) <= 0)
            return;
          if (self.game.Data.RegimeObj[self.game.Data.Turn].MesChosen[self.detailnr] > 0)
          {
            tsubpart =  new TextButtonPartClass("REVIEW DECISION", 305, "You already made a decision, but you can click to review it.",  self.OwnBitmap, 424, self.h - 90, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.okId = self.AddSubPart( tsubpart, 424, self.h - 90, 305, 40, 1);
          }
          else
          {
            tsubpart =  new TextButtonPartClass("MAKE DECISION", 305, "Click to review possible decisions for this report.",  self.OwnBitmap, 424, self.h - 90, theight: 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
            self.okId = self.AddSubPart( tsubpart, 424, self.h - 90, 305, 40, 1);
          }
        }
        else if (self.Text2Id == 0)
        {
          tsubpart =  new TextAreaClass2(self.game, 360 + (self.w - 684),  Math.Round(Conversion.Int( (self.h - 100) / 17.0)), self.game.MarcFont8, "[tab]Message," + self.game.Data.RegimeObj[self.game.Data.Turn].MessString[self.detailnr] + "[/tab]", 17,  self.OwnBitmap, 294, 0);
          self.Text2Id = self.AddSubPart( tsubpart, 294, 0, 360 + (self.w - 684), ( Math.Round(Math.Ceiling( (self.h - 100 - 0) / 17.0)) + 1) * 17, 0);
        }
        else
          self.SubPartFlag[self.SubpartNr(self.Text2Id)] = true;
      }
      else
        DrawMod.DrawText( g, "No reports available.", self.game.GameFont1, 10, 150);
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height)
        {
          self.game.EditObj.TipButton = true;
          self.game.EditObj.TipTitle = self.MouseTitle[index];
          self.game.EditObj.TipText = self.MouseText[index];
          return;
        }
      }
      if (self.SubPartCounter <= -1)
        return;
      let mut subPartCounter: i32 = self.SubPartCounter;
      for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
      {
        if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
        {
          self.game.EditObj.TipButton = false;
          self.SubPartList[index].HandleToolTip(x - self.SubPartX[index], y - self.SubPartY[index]);
          if (self.game.EditObj.TipButton)
            break;
          if (!self.game.EditObj.TipButton & Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            break;
          }
        }
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            if (num1 == self.Text2Id)
            {
              self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsList5id)
            {
              let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              if (num2 > -1)
              {
                self.detailnr = num2;
                if (self.Text1Id > 0)
                {
                  self.RemoveSubPart(self.Text1Id);
                  self.Text1Id = 0;
                }
                if (self.Text2Id > 0)
                {
                  self.RemoveSubPart(self.Text2Id);
                  self.Text2Id = 0;
                }
                self.dostuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.okId)
            {
              self.game.EditObj.FromMessage = self.detailnr;
              self.game.EditObj.PopupValue = 19;
              windowReturnClass.AddCommand(5, 14);
              self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
          }
        }
      }
      let mut mouseCounter: i32 = self.MouseCounter;
      for (let mut index: i32 = 0; index <= mouseCounter; index += 1)
      {
        if (self.MouseData[index] > 0 && x > self.MouseRect[index].X & x < self.MouseRect[index].X + self.MouseRect[index].Width && y > self.MouseRect[index].Y & y < self.MouseRect[index].Y + self.MouseRect[index].Height && self.MouseData[index] == 999)
        {
          self.game.EditObj.SetViewMode2 = 0;
          windowReturnClass.AddCommand(1, 9);
          windowReturnClass.AddCommand(7, 12);
          windowReturnClass.AddCommand(4, 67);
          windowReturnClass.SetFlag(true);
          return windowReturnClass;
        }
      }
      if (x > 8 & x < self.w - 8 && y < self.h - 24)
        windowReturnClass.NoMouseClickBelow = true;
      windowReturnClass.SetFlag(false);
      return windowReturnClass;
    }
  }
}
