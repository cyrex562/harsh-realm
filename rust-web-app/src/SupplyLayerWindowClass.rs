// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SupplyLayerWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;

namespace WindowsApplication1
{
  pub class SupplyLayerWindowClass : WindowClass
  {
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     w: i32;
     h: i32;
     detailnr: i32;
     detailnr2: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     OptionsListId2: i32;
     ListClass OptionsListObj2;
     bool firstCall;
     curMode: i32;
     curModeX: i32;
     curModeY: i32;
     SimpleList CacheL;
     SimpleList CacheL2;
     bool firstCallOnNew;

    pub SupplyLayerWindowClass( tGame: GameClass, screenbitmap: Bitmap = null, let mut sx: i32 = -1, let mut sy: i32 = -1)
      : base( tGame, tGame.ScreenWidth, 150, BackSprite: tGame.MARCBOTBAR)
    {
      self.curMode = 0;
      self.curModeX = -1;
      self.curModeY = -1;
      self.CacheL = SimpleList::new();
      self.CacheL2 = SimpleList::new();
      self.game.EditObj.OrderX = self.game.SelectX;
      self.game.EditObj.OrderY = self.game.SelectY;
      self.game.EditObj.OrderType = 51;
      self.game.EditObj.udsUnitOrderMode = 0;
      self.w = tGame.ScreenWidth;
      self.h = 222;
      self.BlockBlit = true;
      self.firstCall = true;
      if (self.game.Data.MapObj[0].HexObj[self.game.EditObj.OrderX, self.game.EditObj.OrderY].Regime == self.game.Data.Turn | self.game.Data.MapObj[0].HexObj[self.game.EditObj.OrderX, self.game.EditObj.OrderY].Regime == -1)
      {
        self.detailnr = -1;
        self.detailnr2 = -2;
      }
      else
      {
        self.detailnr = -2;
        self.detailnr2 = -1;
      }
      self.firstCallOnNew = true;
      self.dostuff();
    }

     void dostuff()
    {
      self.CacheL = SimpleList::new();
      self.CacheL2 = SimpleList::new();
      let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
      tdata1: i32;
      tdata2: i32;
      Coordinate coordinate1;
      Coordinate coordinate2;
      for (tdata1 = 0; tdata1 <= mapWidth; tdata1 += 1)
      {
        let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
        for (tdata2 = 0; tdata2 <= mapHeight; tdata2 += 1)
        {
          if (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Regime == self.game.Data.Turn)
          {
            let mut location2: i32 = self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location2;
            let mut num: i32 = 0;
            if (location2 > -1)
            {
              let mut type: i32 = self.game.Data.LocObj[location2].Type;
              if (self.game.Data.LocTypeObj[type].isSupplySource)
              {
                num = 1;
                self.CacheL.Add(location2, 100, tdata1, tdata2);
              }
              else if (self.game.Data.LocTypeObj[type].isSupplyBase)
              {
                num = 1;
                self.CacheL.Add(location2, 5, tdata1, tdata2);
              }
            }
            if (num == 0)
            {
              let mut location: i32 = self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location;
              if (location > -1)
              {
                num = 1;
                let mut type: i32 = self.game.Data.LocObj[location].Type;
                if (self.game.Data.LocTypeObj[type].isSupplySource)
                  self.CacheL.Add(location, 100, tdata1, tdata2);
                else if (self.game.Data.LocTypeObj[type].isSupplyBase)
                  self.CacheL.Add(location, 5, tdata1, tdata2);
              }
            }
            if (num == 0 && !coordinate1.onmap)
            {
              coordinate1.x = tdata1;
              coordinate1.y = tdata2;
              coordinate1.onmap = true;
            }
          }
          else if (self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Regime > -1)
          {
            let mut location2: i32 = self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location2;
            let mut num: i32 = 0;
            if (location2 > -1)
            {
              let mut type: i32 = self.game.Data.LocObj[location2].Type;
              if (self.game.Data.LocTypeObj[type].isSupplySource)
              {
                num = 1;
                self.CacheL2.Add(location2, 100, tdata1, tdata2);
              }
              else if (self.game.Data.LocTypeObj[type].isSupplyBase)
              {
                num = 1;
                self.CacheL2.Add(location2, 5, tdata1, tdata2);
              }
            }
            if (num == 0)
            {
              let mut location: i32 = self.game.Data.MapObj[0].HexObj[tdata1, tdata2].Location;
              if (location > -1)
              {
                num = 1;
                let mut type: i32 = self.game.Data.LocObj[location].Type;
                if (self.game.Data.LocTypeObj[type].isSupplySource)
                  self.CacheL2.Add(location, 100, tdata1, tdata2);
                else if (self.game.Data.LocTypeObj[type].isSupplyBase)
                  self.CacheL2.Add(location, 5, tdata1, tdata2);
              }
            }
            if (num == 0 && !coordinate2.onmap)
            {
              coordinate2.x = tdata1;
              coordinate2.y = tdata2;
              coordinate2.onmap = true;
            }
          }
        }
      }
      self.CacheL.ReverseSortHighSpeed();
      self.CacheL2.ReverseSortHighSpeed();
      if (self.B1Id > 0)
        self.RemoveSubPart(self.B1Id);
      if (self.B1TextId > 0)
        self.RemoveSubPart(self.B1TextId);
      if (self.B2Id > 0)
        self.RemoveSubPart(self.B2Id);
      if (self.B2TextId > 0)
        self.RemoveSubPart(self.B2TextId);
      if (self.B3Id > 0)
        self.RemoveSubPart(self.B3Id);
      if (self.B3TextId > 0)
        self.RemoveSubPart(self.B3TextId);
      if (self.B4Id > 0)
        self.RemoveSubPart(self.B4Id);
      if (self.B4TextId > 0)
        self.RemoveSubPart(self.B4TextId);
      if (self.B5Id > 0)
        self.RemoveSubPart(self.B5Id);
      if (self.B5TextId > 0)
        self.RemoveSubPart(self.B5TextId);
      if (self.B6Id > 0)
        self.RemoveSubPart(self.B6Id);
      if (self.B6TextId > 0)
        self.RemoveSubPart(self.B6TextId);
      if (self.OptionsListId > 0)
      {
        self.RemoveSubPart(self.OptionsListId);
        self.OptionsListId = 0;
      }
      if (self.OptionsListId2 > 0)
      {
        self.RemoveSubPart(self.OptionsListId2);
        self.OptionsListId2 = 0;
      }
      let mut num1: i32 =  Math.Round( (self.w - 1280) / 2.0);
      self.NewBackGroundAndClearAll(self.w, self.h, self.game.MARCBOTBAR);
      self.ClearMouse();
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      DrawMod.DrawBlock( Expression, num1 + 20, 10, 400, 135, 0, 0, 0, 100);
      DrawMod.DrawBlock( Expression, num1 + 440, 10, 400, 135, 0, 0, 0, 100);
      DrawMod.DrawBlock( Expression, num1 + 860, 10, 400, 135, 0, 0, 0, 100);
      self.OptionsListObj = ListClass::new();
      let mut num2: i32 = -1;
      let mut tlistselect1: i32 = -1;
      let mut num3: i32 = -1;
      if (!coordinate1.onmap)
        num3 = 0;
      if (self.firstCallOnNew & self.game.EditObj.dc4_last_supply_x > -1)
      {
        self.firstCallOnNew = false;
        let mut counter1: i32 = self.CacheL.Counter;
        for (let mut index: i32 = 0; index <= counter1; index += 1)
        {
          let mut num4: i32 = self.CacheL.Id[index];
          tdata1 = self.CacheL.Data1[index];
          tdata2 = self.CacheL.Data2[index];
          if (self.game.EditObj.dc4_last_supply_x == tdata1 & tdata2 == self.game.EditObj.dc4_last_supply_y)
          {
            self.detailnr = num4;
            self.detailnr2 = -2;
          }
        }
        let mut counter2: i32 = self.CacheL2.Counter;
        for (let mut index: i32 = 0; index <= counter2; index += 1)
        {
          let mut num5: i32 = self.CacheL2.Id[index];
          tdata1 = self.CacheL2.Data1[index];
          tdata2 = self.CacheL2.Data2[index];
          if (self.game.EditObj.dc4_last_supply_x == tdata1 & tdata2 == self.game.EditObj.dc4_last_supply_y)
          {
            self.detailnr2 = num5;
            self.detailnr = -2;
          }
        }
      }
      let mut counter3: i32 = self.CacheL.Counter;
      for (let mut index: i32 = -1; index <= counter3; index += 1)
      {
        if (index > -1)
        {
          let mut tdata: i32 = self.CacheL.Id[index];
          tdata1 = self.CacheL.Data1[index];
          tdata2 = self.CacheL.Data2[index];
          let mut type: i32 = self.game.Data.LocObj[tdata].Type;
          num2 += 1;
          if (self.detailnr == -1)
            self.detailnr = tdata;
          if (tdata == self.detailnr)
          {
            tlistselect1 = num2;
            self.game.EditObj.dc4_last_supply_x = tdata1;
            self.game.EditObj.dc4_last_supply_y = tdata2;
          }
          tname: String = self.game.Data.LocTypeObj[type].Name + " (" + tdata1.ToString() + "," + tdata2.ToString() + ")";
          if (Operators.CompareString(Strings.LCase(self.game.Data.LocTypeObj[type].Name), Strings.LCase(self.game.Data.LocObj[tdata].Name), false) != 0)
            tname = tname + " '" + self.game.Data.LocObj[tdata].Name + "'";
          else if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tdata].X, self.game.Data.LocObj[tdata].Y].Location != tdata)
          {
            let mut location: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tdata].X, self.game.Data.LocObj[tdata].Y].Location;
            if (location > -1)
              tname = tname + " '" + self.game.Data.LocObj[location].Name + "'";
          }
          self.OptionsListObj.add(tname, tdata);
        }
        else
        {
          let mut tdata: i32 = 999999;
          num2 += 1;
          if (self.detailnr == -1)
            self.detailnr = 999999;
          if (tdata == self.detailnr)
            tlistselect1 = num2;
          self.OptionsListObj.add("All friendly supply sources", tdata);
        }
      }
      SubPartClass tsubpart;
      if (self.OptionsListId > 0)
      {
        self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect1);
        self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
      }
      else
      {
        tsubpart =  new ListSubPartClass(self.OptionsListObj, 4, 380, tlistselect1, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 450), bby: 18, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
        self.OptionsListId = self.AddSubPart( tsubpart, num1 + 450, 18, 380, 120, 0);
      }
      self.OptionsListObj2 = ListClass::new();
      let mut num6: i32 = -1;
      let mut tlistselect2: i32 = -1;
      num3 = -1;
      if (!coordinate2.onmap)
        num3 = 0;
      let mut counter4: i32 = self.CacheL2.Counter;
      for (let mut index: i32 = -1; index <= counter4; index += 1)
      {
        if (index > -1)
        {
          let mut tdata: i32 = self.CacheL2.Id[index];
          tdata1 = self.CacheL2.Data1[index];
          tdata2 = self.CacheL2.Data2[index];
          let mut type: i32 = self.game.Data.LocObj[tdata].Type;
          num6 += 1;
          if (self.detailnr2 == -1)
            self.detailnr2 = tdata;
          if (tdata == self.detailnr2)
          {
            self.game.EditObj.dc4_last_supply_x = tdata1;
            self.game.EditObj.dc4_last_supply_y = tdata2;
            tlistselect2 = num6;
          }
          tname: String = self.game.Data.LocTypeObj[type].Name + " (" + tdata1.ToString() + "," + tdata2.ToString() + ")";
          if (Operators.CompareString(Strings.LCase(self.game.Data.LocTypeObj[type].Name), Strings.LCase(self.game.Data.LocObj[tdata].Name), false) != 0)
            tname = tname + " '" + self.game.Data.LocObj[tdata].Name + "'";
          else if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tdata].X, self.game.Data.LocObj[tdata].Y].Location != tdata)
          {
            let mut location: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[tdata].X, self.game.Data.LocObj[tdata].Y].Location;
            if (location > -1)
              tname = tname + " '" + self.game.Data.LocObj[location].Name + "'";
          }
          self.OptionsListObj2.add(tname, tdata);
        }
        else
        {
          let mut tdata: i32 = 999999;
          num6 += 1;
          if (self.detailnr2 == -1)
            self.detailnr2 = 999999;
          if (tdata == self.detailnr2)
            tlistselect2 = num6;
          self.OptionsListObj2.add("All enemy supply sources", tdata);
        }
      }
      if (self.OptionsListId2 > 0)
      {
        self.SubPartList[self.SubpartNr(self.OptionsListId2)].Refresh(self.OptionsListObj2, tlistselect2);
        self.SubPartFlag[self.SubpartNr(self.OptionsListId2)] = true;
      }
      else
      {
        tsubpart =  new ListSubPartClass(self.OptionsListObj2, 4, 380, tlistselect2, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (num1 + 870), bby: 18, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
        self.OptionsListId2 = self.AddSubPart( tsubpart, num1 + 870, 18, 380, 120, 0);
      }
      bool flag = false;
      str1: String = "";
      str2: String = "";
      if (self.game.Data.MapObj[0].HexObj[self.game.EditObj.OrderX, self.game.EditObj.OrderY].Regime == self.game.Data.Turn & self.detailnr2 <= -2 | self.detailnr > -2)
      {
        str2 = "Friendly";
        if (self.detailnr == 999999)
        {
          str1 = "All friendly supply sources";
          str2 = "";
          if (self.firstCall | !(self.game.EditObj.OrderX == coordinate1.x & self.game.EditObj.OrderY == coordinate1.y))
          {
            self.curMode = 1;
            self.game.EditObj.OrderX = coordinate1.x;
            self.game.EditObj.OrderY = coordinate1.y;
            self.game.HandyFunctionsObj.MakeSupplyLayer3(self.game.EditObj.OrderX, self.game.EditObj.OrderY, 0);
            flag = true;
          }
        }
        else if (self.detailnr > -1)
        {
          let mut index: i32 = self.CacheL.Id[self.CacheL.FindNr(self.detailnr)];
          let mut type: i32 = self.game.Data.LocObj[index].Type;
          str1 = self.game.Data.LocTypeObj[type].Name + " (" + self.game.Data.LocObj[index].X.ToString() + "," + self.game.Data.LocObj[index].Y.ToString() + ")";
          if (Operators.CompareString(Strings.LCase(self.game.Data.LocTypeObj[type].Name), Strings.LCase(self.game.Data.LocObj[index].Name), false) != 0)
            str1 = str1 + " '" + self.game.Data.LocObj[index].Name + "'";
          else if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index].X, self.game.Data.LocObj[index].Y].Location != index)
          {
            let mut location: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index].X, self.game.Data.LocObj[index].Y].Location;
            if (location > -1)
              str1 = str1 + " '" + self.game.Data.LocObj[location].Name + "'";
          }
          let mut nr: i32 = self.CacheL.FindNr(self.detailnr);
          if (self.firstCall | !(self.curModeX == self.CacheL.Data1[nr] & self.curModeY == self.CacheL.Data2[nr] & self.curMode == 2))
          {
            self.curMode = 2;
            self.curModeX = self.CacheL.Data1[nr];
            self.curModeY = self.CacheL.Data2[nr];
            self.game.EditObj.OrderX = self.curModeX;
            self.game.EditObj.OrderY = self.curModeY;
            self.game.HandyFunctionsObj.MakeSupplyLayer2(self.game.EditObj.OrderX, self.game.EditObj.OrderY, self.game.EditObj.MapSelected);
            flag = true;
          }
        }
      }
      if (self.game.Data.MapObj[0].HexObj[self.game.EditObj.OrderX, self.game.EditObj.OrderY].Regime != self.game.Data.Turn & self.detailnr <= -2 | self.detailnr2 > -2)
      {
        str2 = "Enemy";
        if (self.detailnr2 == 999999)
        {
          str1 = "All enemy supply sources";
          str2 = "";
          if (self.firstCall | !(self.game.EditObj.OrderX == coordinate2.x & self.game.EditObj.OrderY == coordinate2.y))
          {
            self.curMode = 1;
            flag = true;
            self.game.EditObj.OrderX = coordinate2.x;
            self.game.EditObj.OrderY = coordinate2.y;
            self.game.HandyFunctionsObj.MakeSupplyLayer3(self.game.EditObj.OrderX, self.game.EditObj.OrderY, 0);
          }
        }
        else if (self.detailnr2 > -1)
        {
          let mut index: i32 = self.CacheL2.Id[self.CacheL2.FindNr(self.detailnr2)];
          let mut type: i32 = self.game.Data.LocObj[index].Type;
          str1 = self.game.Data.LocTypeObj[type].Name + " (" + tdata1.ToString() + "," + tdata2.ToString() + ")";
          if (Operators.CompareString(Strings.LCase(self.game.Data.LocTypeObj[type].Name), Strings.LCase(self.game.Data.LocObj[index].Name), false) != 0)
            str1 = str1 + " '" + self.game.Data.LocObj[index].Name + "'";
          else if (self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index].X, self.game.Data.LocObj[index].Y].Location != index)
          {
            let mut location: i32 = self.game.Data.MapObj[0].HexObj[self.game.Data.LocObj[index].X, self.game.Data.LocObj[index].Y].Location;
            if (location > -1)
              str1 = str1 + " '" + self.game.Data.LocObj[location].Name + "'";
          }
          let mut nr: i32 = self.CacheL2.FindNr(self.detailnr2);
          if (self.firstCall | !(self.curModeX == self.CacheL2.Data1[nr] & self.curModeY == self.CacheL2.Data2[nr] & self.curMode == 3))
          {
            self.curMode = 3;
            flag = true;
            self.curModeX = self.CacheL2.Data1[nr];
            self.curModeY = self.CacheL2.Data2[nr];
            self.game.EditObj.OrderX = self.curModeX;
            self.game.EditObj.OrderY = self.curModeY;
            self.game.HandyFunctionsObj.MakeSupplyLayer2(self.game.EditObj.OrderX, self.game.EditObj.OrderY, self.game.EditObj.MapSelected);
          }
        }
      }
      if (flag)
      {
        self.game.EditObj.SupplyPath = CoordList::new();
        let mut x1: i32 = self.game.SelectX;
        let mut y1: i32 = self.game.SelectY;
        map1: i32;
        for (let mut map2: i32 = 0; self.game.EditObj.TempSupCameFrom[map2].Value[x1, y1].onmap; map2 = map1)
        {
          self.game.EditObj.SupplyPath.AddCoord(x1, y1, map2);
          let mut x2: i32 = self.game.EditObj.TempSupCameFrom[map2].Value[x1, y1].x;
          let mut y2: i32 = self.game.EditObj.TempSupCameFrom[map2].Value[x1, y1].y;
          map1 = self.game.EditObj.TempSupCameFrom[map2].Value[x1, y1].map;
          x1 = x2;
          y1 = y2;
        }
        self.game.EditObj.TempCoordList = CoordList::new();
      }
      self.firstCall = false;
      let mut x: i32 = num1 + 220;
      let mut y3: i32 = 30;
      tstring1: String = "current supply layer mode:";
      DrawMod.DrawTextColouredMarcCenter( Expression, tstring1, self.game.MarcFont8, x, y3, Color.LightGray);
      tstring2: String = str1;
      let mut y4: i32 = y3 + 20;
      DrawMod.DrawTextColouredMarcCenter( Expression, tstring2, self.game.MarcFont6, x, y4, Color.WhiteSmoke);
      if (str2.Length > 1)
      {
        y4 += 20;
        str3: String = str2;
        DrawMod.DrawTextColouredMarcCenter( Expression, "(" + str3 + ")", self.game.MarcFont8b, x, y4, Color.WhiteSmoke);
      }
      let mut y5: i32 = y4 + 30;
      tstring3: String = "";
      if (self.curMode == 1)
        tstring3 = "Shows all sources as white hexes and traces path to selected hex from closest source.";
      else if (self.curMode == 2 | self.curMode == 3)
        tstring3 = "Shows the specific source/base with a white hexe and traces path to selected hex from this source.";
      DrawMod.DrawTextColouredConsoleMultiline( Expression, tstring3, self.game.MarcFont8c, x - 200, y5, Color.LightGray, 390, 80, true);
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      OrderResult orderResult = OrderResult::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index])
          {
            let mut num1: i32 = self.SubPartID[index];
            selectX: i32;
            selectY: i32;
            if (num1 == self.OptionsListId)
            {
              let mut num2: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              bool flag = false;
              if (num2 > -1)
              {
                if (self.detailnr == num2)
                  flag = true;
                self.detailnr = num2;
                self.detailnr2 = -2;
                if (self.detailnr < 999999 & !Information.IsNothing( self.CacheL))
                {
                  let mut nr: i32 = self.CacheL.FindNr(self.detailnr);
                  self.game.EditObj.OrderX = self.CacheL.Data1[nr];
                  self.game.EditObj.OrderY = self.CacheL.Data2[nr];
                }
                self.dostuff();
              }
              if (flag & self.detailnr < 999999)
              {
                selectX = self.game.SelectX;
                selectY = self.game.SelectY;
                self.game.SelectX = self.game.EditObj.OrderX;
                self.game.SelectY = self.game.EditObj.OrderY;
                self.game.HandyFunctionsObj.SetcornerXY2();
              }
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId2)
            {
              let mut num3: i32 = self.SubPartList[index].Click(x - self.SubPartX[index], y - self.SubPartY[index]);
              self.SubPartFlag[index] = true;
              bool flag = false;
              if (num3 > -1)
              {
                if (self.detailnr2 == num3)
                  flag = true;
                self.detailnr2 = num3;
                self.detailnr = -2;
                if (self.detailnr2 < 999999 & !Information.IsNothing( self.CacheL2))
                {
                  let mut nr: i32 = self.CacheL2.FindNr(self.detailnr2);
                  self.game.EditObj.OrderX = self.CacheL2.Data1[nr];
                  self.game.EditObj.OrderY = self.CacheL2.Data2[nr];
                }
                self.dostuff();
              }
              if (flag & self.detailnr2 < 999999)
              {
                selectX = self.game.SelectX;
                selectY = self.game.SelectY;
                self.game.SelectX = self.game.EditObj.OrderX;
                self.game.SelectY = self.game.EditObj.OrderY;
                self.game.HandyFunctionsObj.SetcornerXY2();
              }
              windowReturnClass.AddCommand(4, 12);
              windowReturnClass.AddCommand(4, 9);
              windowReturnClass.AddCommand(4, 67);
              windowReturnClass.AddCommand(4, 68);
              windowReturnClass.AddCommand(4, 69);
              windowReturnClass.SetFlag(true);
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
  }
}
