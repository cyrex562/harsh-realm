// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.NonCardSelectWindowClass
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
  pub class NonCardSelectWindowClass : WindowClass
  {
     okid: i32;
     ok2id: i32;
     cancelid: i32;
     cancel2id: i32;
     resultMesOkId: i32;
     oktextid: i32;
     subId: i32;
     quitId: i32;
     deselectId: i32;
     deselectid2: i32;
     tMiniWidth: i32;
     tMiniHeight: i32;
     Pic1Id: i32;
     FinalId: i32;
     unitheaderid: i32;
     unitsfid: i32;
     mapid: i32;
     tUnitSelected: i32;
     tcx: i32;
     tcy: i32;
     tSelectX: i32;
     tSelectY: i32;
     tSelectMap: i32;
     tCornerX: i32;
     tCornerY: i32;
     SimpleList UL;
     CoordList HL;
     OptionsListId: i32;
     textAreaId: i32;
     ListClass OptionsListObj;
     SelectUsageMode usageMode;
     targetUnitSelected: i32;
     bool subordinatesToo;
     resultString: String;

    pub NonCardSelectWindowClass( tGame: GameClass, SelectUsageMode tUsageMode)
      : base( tGame, 1280, 768, 8)
    {
      self.targetUnitSelected = -1;
      self.subordinatesToo = false;
      self.resultString = "";
      self.UL = SimpleList::new();
      self.HL = CoordList::new();
      self.subordinatesToo = true;
      self.usageMode = tUsageMode;
      if (tUsageMode == SelectUsageMode.selectHQ)
      {
        let mut unitCounter1: i32 =  self.game.Data.UnitCounter;
        for (let mut index: i32 =  0; index <= unitCounter1; index += 1)
          self.game.Data.UnitObj[index].TempUnitSelectable = false;
        let mut unitSelected: i32 =  self.game.EditObj.UnitSelected;
        let mut unitCounter2: i32 =  self.game.Data.UnitCounter;
        for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
        {
          if (self.game.Data.UnitObj[unr].PreDef == -1 & self.game.Data.UnitObj[unr].X > -1 && unitSelected > -1 && unitSelected != unr & self.game.Data.UnitObj[unitSelected].HQ != unr & self.game.Data.UnitObj[unr].IsHQ & (self.game.Data.UnitObj[unr].Regime == self.game.Data.Turn | self.game.Data.UnitObj[unr].Regime == self.game.Data.UnitObj[unitSelected].Regime) && self.game.HandyFunctionsObj.CanUnitBecomeHQfor(unr, unitSelected))
          {
            let mut num: i32 =  0;
            if (self.game.Data.UnitObj[unitSelected].IsHQ)
              num = 1;
            if ( self.game.Data.RuleVar[304] == 0.0 |  (self.game.HandyFunctionsObj.HowmanyHQsAbove(unr) + self.game.HandyFunctionsObj.HowmanyHQsBelow(unitSelected) + 1 + num) <=  self.game.Data.RuleVar[304])
              self.game.Data.UnitObj[unr].TempUnitSelectable = true;
          }
        }
      }
      if (tUsageMode == SelectUsageMode.autoMove)
      {
        let mut unitSelected: i32 =  self.game.EditObj.UnitSelected;
        let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut x: i32 =  0; x <= mapWidth; x += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut y: i32 =  0; y <= mapHeight; y += 1)
          {
            self.game.Data.MapObj[0].HexObj[x, y].tempSelectable = false;
            if (self.game.Data.MapObj[0].HexObj[x, y].Regime == self.game.Data.Turn && self.game.Data.LandscapeTypeObj[self.game.Data.MapObj[0].HexObj[x, y].LandscapeType].AIBlock == 0)
            {
              self.game.Data.MapObj[0].HexObj[x, y].tempSelectable = true;
              self.HL.AddCoord(x, y, 0, 0, 0);
            }
          }
        }
      }
      if (tUsageMode == SelectUsageMode.blowBridge)
      {
        let mut unitSelected: i32 =  self.game.EditObj.UnitSelected;
        let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index1: i32 =  0; index1 <= mapWidth; index1 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index2: i32 =  0; index2 <= mapHeight; index2 += 1)
            self.game.Data.MapObj[0].HexObj[index1, index2].tempSelectable = false;
        }
        let mut num: i32 =  1;
        do
        {
          Coordinate coordinate = self.game.HandyFunctionsObj.HexNeighbourSameMap(self.game.Data.UnitObj[unitSelected].X, self.game.Data.UnitObj[unitSelected].Y, 0, num);
          if (coordinate.onmap && self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].Bridge[num - 1] & self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].RiverType[num - 1] > -1)
          {
            self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].tempSelectable = true;
            self.HL.AddCoord(coordinate.x, coordinate.y, 0, num, 0);
          }
          num += 1;
        }
        while (num <= 6);
      }
      if (tUsageMode == SelectUsageMode.repairBridge)
      {
        CoordList coordList = self.game.HandyFunctionsObj.InfraHexHighlight_getCoordList(self.game.SelectX, self.game.SelectY, 0, self.game.EditObj.UnitSelected);
        let mut mapWidth: i32 =  self.game.Data.MapObj[0].MapWidth;
        for (let mut index3: i32 =  0; index3 <= mapWidth; index3 += 1)
        {
          let mut mapHeight: i32 =  self.game.Data.MapObj[0].MapHeight;
          for (let mut index4: i32 =  0; index4 <= mapHeight; index4 += 1)
            self.game.Data.MapObj[0].HexObj[index3, index4].tempSelectable = false;
        }
        let mut counter: i32 =  coordList.counter;
        for (let mut index: i32 =  0; index <= counter; index += 1)
        {
          Coordinate coordinate = coordList.coord[index];
          if (coordinate.onmap)
          {
            let mut dat1: i32 =  self.game.HandyFunctionsObj.HexFacing(self.game.SelectX, self.game.SelectY, 0, coordinate.x, coordinate.y, 0);
            self.game.Data.MapObj[0].HexObj[coordinate.x, coordinate.y].tempSelectable = true;
            self.HL.AddCoord(coordinate.x, coordinate.y, 0, dat1, 0);
          }
        }
      }
      if (self.usageMode != SelectUsageMode.none)
      {
        if (self.usageMode == SelectUsageMode.joinAttack)
        {
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
          {
            if (self.game.Data.UnitObj[tid].TempUnitSelectable)
              self.UL.Add(tid, 0);
          }
        }
        else if (self.usageMode == SelectUsageMode.selectHQ)
        {
          let mut unitCounter: i32 =  self.game.Data.UnitCounter;
          for (let mut tid: i32 =  0; tid <= unitCounter; tid += 1)
          {
            if (self.game.Data.UnitObj[tid].TempUnitSelectable)
              self.UL.Add(tid, 0);
          }
        }
      }
      self.tUnitSelected = self.game.EditObj.UnitSelected;
      if (!Information.IsNothing( self.game.EditObj.MiniMap))
      {
        self.tMiniWidth = self.game.EditObj.MiniMap.Width;
        self.tMiniHeight = self.game.EditObj.MiniMap.Height;
      }
      self.tSelectX = self.game.SelectX;
      self.tSelectY = self.game.SelectY;
      self.tCornerX = self.game.CornerX;
      self.tCornerY = self.game.CornerY;
      self.tSelectMap = self.game.EditObj.MapSelected;
      self.tcx = self.game.CornerX;
      self.tcy = self.game.CornerY;
      self.game.SelectX = self.game.EditObj.TargetX;
      self.game.SelectY = self.game.EditObj.TargetY;
      self.game.HandyFunctionsObj.CenterOnXY(self.game.SelectX, self.game.SelectY, useWidth: 939, useHeight: 538);
      if (!(self.usageMode == SelectUsageMode.blowBridge | self.usageMode == SelectUsageMode.repairBridge))
      {
        if (self.usageMode == SelectUsageMode.autoMove)
        {
          if (self.game.Data.UnitObj[self.tUnitSelected].autoMoveX > -1)
          {
            self.game.SelectX = self.game.Data.UnitObj[self.tUnitSelected].autoMoveX;
            self.game.SelectY = self.game.Data.UnitObj[self.tUnitSelected].autoMoveY;
          }
        }
        else if (self.usageMode == SelectUsageMode.joinAttack)
        {
          self.game.SelectX = self.game.EditObj.TargetX;
          self.game.SelectY = self.game.EditObj.TargetY;
        }
        else
        {
          self.game.SelectX = -1;
          self.game.SelectY = -1;
        }
      }
      self.game.EditObj.UnitSelected = -1;
      self.OptionsListObj = ListClass::new();
      self.game.EditObj.TempCoordList = CoordList::new();
      self.ViewMessage();
    }

    pub fn HandleToolTip(x: i32, y: i32)
    {
      base.HandleToolTip(x, y);
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  self.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
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
      let mut mouseCounter: i32 =  self.MouseCounter;
      for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
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
      if (self.ok2id > 0)
      {
        self.RemoveSubPart(self.ok2id);
        self.ok2id = 0;
      }
      if (self.cancel2id > 0)
      {
        self.RemoveSubPart(self.cancel2id);
        self.cancel2id = 0;
      }
      if (self.quitId > 0)
      {
        self.RemoveSubPart(self.quitId);
        self.quitId = 0;
      }
      if (self.FinalId > 0)
      {
        self.RemoveSubPart(self.FinalId);
        self.FinalId = 0;
      }
      if (self.subId > 0)
      {
        self.RemoveSubPart(self.subId);
        self.subId = 0;
      }
      if (self.deselectId > 0)
      {
        self.RemoveSubPart(self.deselectId);
        self.deselectId = 0;
      }
      if (self.deselectid2 > 0)
      {
        self.RemoveSubPart(self.deselectid2);
        self.deselectid2 = 0;
      }
      if (self.resultMesOkId > 0)
      {
        self.RemoveSubPart(self.resultMesOkId);
        self.resultMesOkId = 0;
      }
      if (self.textAreaId > 0)
      {
        self.RemoveSubPart(self.textAreaId);
        self.textAreaId = 0;
      }
      self.ClearMouse();
      if (self.resultString.Length > 1)
      {
        Graphics g = Graphics.FromImage((Image) self.OwnBitmap);
        self.NewBackGroundAndClearAll(1280, 768, -1);
        DrawMod.DrawMessFrame( self.OwnBitmap,  g, 0, 0, 1280, 768);
        self.BackBitmap = (Bitmap) self.OwnBitmap.Clone();
        if (self.textAreaId < 1)
        {
          let mut tsubpart: SubPartClass =  new TextAreaClass2(self.game, 1100, 26, self.game.MarcFont8, "[tab]result," + self.resultString + "[/tab]", 17,  self.OwnBitmap, 100, 100);
          self.textAreaId = self.AddSubPart( tsubpart, 100, 100, 1100, 476, 0);
        }
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("OK", 180, "Click to acknowledge message and return to map.",  self.OwnBitmap, 540, 600, theight: 60, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.resultMesOkId = self.AddSubPart( tsubpart1, 540, 600, 180, 60, 1);
        g.Dispose();
      }
      else
      {
        let mut num1: i32 =  0;
        let mut num2: i32 =  self.usageMode == SelectUsageMode.blowBridge | self.usageMode == SelectUsageMode.repairBridge ? 1 : 0;
        if (self.mapid == 0)
        {
          let mut tsubpart: SubPartClass =  new MapPartClass(947, 460 + num1, self.game, ZoomLevel: 0, tFromPopupMap: true);
          self.mapid = self.AddSubPart( tsubpart, 5, 60, 947, 460 + num1, 0);
        }
        Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
        self.NewBackGroundAndClearAll(1280, 768, -1);
        DrawMod.DrawMessFrame( self.OwnBitmap,  graphics, 0, 0, 1280, 768);
        if (self.usageMode == SelectUsageMode.joinAttack)
          DrawMod.DrawTextColouredMarc( graphics, "Select units to join in the planned attack.", self.game.MarcFont2, 30, 17, Color.White);
        else if (self.usageMode == SelectUsageMode.selectHQ)
          DrawMod.DrawTextColouredMarc( graphics, "Select HQ for " + self.game.Data.UnitObj[self.tUnitSelected].Name, self.game.MarcFont2, 30, 17, Color.White);
        else if (self.usageMode == SelectUsageMode.blowBridge)
          DrawMod.DrawTextColouredMarc( graphics, "Select Bridge to be blown by " + self.game.Data.UnitObj[self.tUnitSelected].Name, self.game.MarcFont2, 30, 17, Color.White);
        else if (self.usageMode == SelectUsageMode.repairBridge)
          DrawMod.DrawTextColouredMarc( graphics, "Select Bridge to be repaired by " + self.game.Data.UnitObj[self.tUnitSelected].Name, self.game.MarcFont2, 30, 17, Color.White);
        else if (self.usageMode == SelectUsageMode.autoMove)
        {
          if (self.game.Data.UnitObj[self.tUnitSelected].autoMoveX > -1)
            DrawMod.DrawTextColouredMarc( graphics, "Auto-move destination for " + self.game.Data.UnitObj[self.tUnitSelected].Name + " is " + self.game.Data.UnitObj[self.tUnitSelected].autoMoveX.ToString() + "," + self.game.Data.UnitObj[self.tUnitSelected].autoMoveY.ToString(), self.game.MarcFont2, 30, 17, Color.White);
          else
            DrawMod.DrawTextColouredMarc( graphics, "Select Auto-move destination for " + self.game.Data.UnitObj[self.tUnitSelected].Name, self.game.MarcFont2, 30, 17, Color.White);
        }
        SubPartClass tsubpart2;
        if (self.usageMode == SelectUsageMode.joinAttack)
        {
          if (self.UL.Counter <= -1)
          {
            DrawMod.DrawTextColouredMarc( graphics, "No units selectable.", self.game.MarcFont3, 970, 11, Color.White);
            DrawMod.DrawTextColouredMarc( graphics, "None selected.", self.game.MarcFont3, 970, 31, Color.White);
          }
          else
          {
             let mut local1: &Graphics = &graphics;
            let mut num3: i32 =  self.UL.Counter + 1;
            tstring1: String = num3.ToString() + " units selectable.";
            marcFont3_1: Font = self.game.MarcFont3;
            white1: Color = Color.White;
            DrawMod.DrawTextColouredMarc( local1, tstring1, marcFont3_1, 970, 11, white1);
             let mut local2: &Graphics = &graphics;
            num3 = self.game.EditObj.TempUnitList.counter + 1;
            tstring2: String = num3.ToString() + " selected.";
            marcFont3_2: Font = self.game.MarcFont3;
            white2: Color = Color.White;
            DrawMod.DrawTextColouredMarc( local2, tstring2, marcFont3_2, 970, 31, white2);
          }
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Back", 86, "Click to go back to the combat setup window [ESC].",  self.BackBitmap, 1210, 11, theight: 43, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.quitId = self.AddSubPart( tsubpart3, 1160, 11, 86, 43, 1);
        }
        else if (self.usageMode == SelectUsageMode.selectHQ)
        {
          DrawMod.DrawTextColouredMarc( graphics, "Current HQ is:", self.game.MarcFont3, 970, 11, Color.White);
          DrawMod.DrawTextColouredMarc( graphics, self.game.Data.UnitObj[self.tUnitSelected].Name, self.game.MarcFont3, 970, 31, Color.White);
          tsubpart2 =  new TextButtonPartClass("Cancel", 50, "Abort changing HQ [ESC]",  self.BackBitmap, 1210, 5, theight: 50, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.quitId = self.AddSubPart( tsubpart2, 1210, 5, 50, 50, 1);
        }
        else if (self.usageMode == SelectUsageMode.blowBridge)
        {
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Cancel", 50, "Abort blowing bridge [ESC]",  self.BackBitmap, 1210, 5, theight: 50, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.quitId = self.AddSubPart( tsubpart4, 1210, 5, 50, 50, 1);
        }
        else if (self.usageMode == SelectUsageMode.autoMove)
        {
          let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Cancel", 50, "Abort auto-move [ESC]",  self.BackBitmap, 1210, 5, theight: 50, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.quitId = self.AddSubPart( tsubpart5, 1210, 5, 50, 50, 1);
        }
        else if (self.usageMode == SelectUsageMode.repairBridge)
        {
          let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Cancel", 50, "Abort repairing bridge [ESC]",  self.BackBitmap, 1210, 5, theight: 50, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
          self.quitId = self.AddSubPart( tsubpart6, 1210, 5, 50, 50, 1);
        }
        tsubpart2 =  new MiniMapPartClass(DrawMod.TGame, tx: DrawMod.GetWidthForMiniMap(), ty: 200);
        self.Pic1Id = self.AddSubPart( tsubpart2, 957, 322, DrawMod.GetWidthForMiniMap(), 200, 0);
        if ((self.UL.Counter <= -1 | self.usageMode == SelectUsageMode.blowBridge | self.usageMode == SelectUsageMode.repairBridge) & self.usageMode != SelectUsageMode.autoMove)
        {
          if (self.usageMode == SelectUsageMode.blowBridge | self.usageMode == SelectUsageMode.repairBridge)
          {
            self.OptionsListObj = ListClass::new();
            let mut num4: i32 =  -1;
            let mut tlistselect: i32 =  -1;
            let mut counter: i32 =  self.HL.counter;
            for (let mut index: i32 =  0; index <= counter; index += 1)
            {
              let mut x: i32 =  self.HL.coord[index].x;
              let mut y: i32 =  self.HL.coord[index].y;
              let mut data1: i32 =  self.HL.coord[index].data1;
              num4 += 1;
              if (self.game.SelectX == x & self.game.SelectY == y)
                tlistselect = num4;
              self.OptionsListObj.add("Bridge towards " + x.ToString() + "," + y.ToString(), data1);
            }
            if (self.OptionsListId <= 0)
            {
              tsubpart2 =  new ListSubPartClass(self.OptionsListObj, 18, 292, tlistselect, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: 956, bby: 55, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 20);
              self.OptionsListId = self.AddSubPart( tsubpart2, 956, 55, 292, 380, 0);
            }
            else
            {
              self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
              self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
              self.SubPartList[self.SubpartNr(self.OptionsListId)].Paint();
            }
          }
        }
        else if (self.usageMode == SelectUsageMode.autoMove)
        {
          self.OptionsListObj = ListClass::new();
          let mut num5: i32 =  -1;
          let mut tlistselect: i32 =  -1;
          let mut counter: i32 =  self.HL.counter;
          for (let mut tdata: i32 =  0; tdata <= counter; tdata += 1)
          {
            let mut x: i32 =  self.HL.coord[tdata].x;
            let mut y: i32 =  self.HL.coord[tdata].y;
            let mut data1: i32 =  self.HL.coord[tdata].data1;
            if (self.game.Data.MapObj[0].HexObj[x, y].Location > -1 && self.game.Data.LocTypeObj[self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x, y].Location].Type].cityLevel >= 3)
            {
              num5 += 1;
              if (self.game.SelectX == x & self.game.SelectY == y)
                tlistselect = num5;
              self.OptionsListObj.add(self.game.Data.LocObj[self.game.Data.MapObj[0].HexObj[x, y].Location].Name + " " + x.ToString() + "," + y.ToString(), tdata);
            }
          }
          if (self.OptionsListId <= 0)
          {
            tsubpart2 =  new ListSubPartClass(self.OptionsListObj, 18, 292, tlistselect, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: 956, bby: 55, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 20);
            self.OptionsListId = self.AddSubPart( tsubpart2, 956, 55, 292, 380, 0);
          }
          else
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Paint();
          }
        }
        else
        {
          self.OptionsListObj = ListClass::new();
          let mut num6: i32 =  -1;
          let mut tlistselect: i32 =  -1;
          let mut counter: i32 =  self.UL.Counter;
          for (let mut index1: i32 =  0; index1 <= counter; index1 += 1)
          {
            let mut index2: i32 =  self.UL.Id[index1];
            num6 += 1;
            if (index2 == self.game.EditObj.UnitSelected)
              tlistselect = num6;
            let mut nr: i32 =  self.game.SMALLCHAR1;
            if (self.game.EditObj.TempUnitList.CheckIfPresent(index2))
              nr = self.game.SMALLCHAR2;
            name: String = self.game.Data.UnitObj[index2].Name;
            if (self.tUnitSelected > -1 && index2 == self.game.Data.UnitObj[self.tUnitSelected].HQ)
              name += " *";
            if (self.usageMode == SelectUsageMode.selectHQ)
              self.OptionsListObj.add(name, index2);
            else
              self.OptionsListObj.add(name, index2, tbmp: BitmapStore.GetBitmap(nr));
          }
          if (self.OptionsListId <= 0)
          {
            tsubpart2 =  new ListSubPartClass(self.OptionsListObj, 12, 292, tlistselect, self.game, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: 956, bby: 55, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 20);
            self.OptionsListId = self.AddSubPart( tsubpart2, 956, 55, 292, 260, 0);
          }
          else
          {
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Refresh(self.OptionsListObj, tlistselect);
            self.SubPartFlag[self.SubpartNr(self.OptionsListId)] = true;
            self.SubPartList[self.SubpartNr(self.OptionsListId)].Paint();
          }
        }
        if (self.usageMode == SelectUsageMode.joinAttack | self.usageMode == SelectUsageMode.selectHQ)
        {
          let mut screenWidth: i32 =  self.game.ScreenWidth;
          self.game.ScreenWidth = 1280;
          PlayExtraWindowClass2 extraWindowClass2 = new PlayExtraWindowClass2( self.game, tcalledFromNonCardSelectWindow: true);
          extraWindowClass2.Paint();
          let mut mouseCounter: i32 =  extraWindowClass2.MouseCounter;
          for (let mut index: i32 =  0; index <= mouseCounter; index += 1)
          {
            Rectangle trect = extraWindowClass2.MouseRect[index];
            trect.X += 0;
            trect.Y += 419;
            extraWindowClass2.MouseText[index] = extraWindowClass2.MouseText[index].Replace("Click to get more information on these troops.", "");
            extraWindowClass2.MouseText[index] = extraWindowClass2.MouseText[index].Replace("Click to change name of unit", "");
            extraWindowClass2.MouseText[index] = extraWindowClass2.MouseText[index].Replace("Click to jump to its map location.", "");
            self.AddMouse( trect, extraWindowClass2.MouseTitle[index], extraWindowClass2.MouseText[index]);
          }
          self.game.ScreenWidth = screenWidth;
          DrawMod.DrawSimplePart2( graphics,  extraWindowClass2.OwnBitmap, Rectangle::new(10, 0, 1240, 222), Rectangle::new(10, 529, 1240, 222));
          extraWindowClass2.Dispose();
        }
        if (self.game.EditObj.UnitSelected > -1 && self.usageMode == SelectUsageMode.selectHQ)
        {
          if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].TempUnitSelectable)
          {
            tsubpart2 =  new TextButtonPartClass("SELECT", 100, "Click to select this Unit as the new HQ [SPACE].",  self.OwnBitmap, 450, 627, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.FinalId = self.AddSubPart( tsubpart2, 450, 627, 100, 40, 1);
          }
          else
          {
            tsubpart2 =  new TextButtonPartClass("SELECT", 100, "Unit is not a selectable HQ.",  self.OwnBitmap, 450, 627, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.cancel2id = self.AddSubPart( tsubpart2, 450, 627, 100, 40, 0);
          }
        }
        if ((self.usageMode == SelectUsageMode.blowBridge | self.usageMode == SelectUsageMode.repairBridge) & self.game.SelectX > -1)
        {
          DrawMod.DrawTextColouredMarcCenter( graphics, "Selected Hex: " + self.game.SelectX.ToString() + "," + self.game.SelectY.ToString(), self.game.MarcFont3, 1106, 617, Color.White);
          if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].tempSelectable)
          {
            tsubpart2 =  new TextButtonPartClass("SELECT", 180, "Click to select this Bridge [SPACE].",  self.OwnBitmap, 1016, 657, theight: 60, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.FinalId = self.AddSubPart( tsubpart2, 1016, 657, 180, 60, 1);
          }
          else
          {
            tsubpart2 =  new TextButtonPartClass("SELECT", 180, "Hex is not selectable.",  self.OwnBitmap, 1016, 657, true, theight: 60, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.cancel2id = self.AddSubPart( tsubpart2, 1016, 657, 180, 60, 0);
          }
        }
        if (self.usageMode == SelectUsageMode.autoMove & self.game.SelectX > -1)
        {
          DrawMod.DrawTextColouredMarcCenter( graphics, "Selected Hex: " + self.game.SelectX.ToString() + "," + self.game.SelectY.ToString(), self.game.MarcFont3, 206, 617, Color.White);
          if (self.game.Data.MapObj[0].HexObj[self.game.SelectX, self.game.SelectY].tempSelectable)
          {
            tsubpart2 =  new TextButtonPartClass("SELECT", 180, "Click to select this Hex as the target for Auto-move [SPACE].",  self.OwnBitmap, 116, 657, theight: 60, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.FinalId = self.AddSubPart( tsubpart2, 116, 657, 180, 60, 1);
          }
          else
          {
            tsubpart2 =  new TextButtonPartClass("SELECT", 180, "Hex is not selectable.",  self.OwnBitmap, 116, 657, true, theight: 60, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.cancel2id = self.AddSubPart( tsubpart2, 116, 657, 180, 60, 0);
          }
          let mut index: i32 =  -1;
          if (self.tUnitSelected > -1)
            index = !self.game.Data.UnitObj[self.tUnitSelected].IsHQ ? self.game.Data.UnitObj[self.tUnitSelected].HQ : self.tUnitSelected;
          DrawMod.DrawTextColouredMarc( graphics, "Include Subordinates", self.game.MarcFont3, 456, 617, Color.White);
          if (index > -1)
            DrawMod.DrawTextColouredMarc( graphics, "(All units of " + self.game.Data.UnitObj[index].Name + ")", self.game.MarcFont3, 456, 641, Color.White);
          tsubpart2 =  new MarcRadioPartClass(0, self.subordinatesToo, "If subordinates are included changing (or cancelling) the target will apply to all non-HQ unit under command of selected HQ",  self.OwnBitmap, 416, 611);
          self.subId = self.AddSubPart( tsubpart2, 416, 611, 35, 35, 1);
          DrawMod.DrawTextColouredMarcCenter( graphics, "Stop auto-move", self.game.MarcFont3, 1006, 617, Color.White);
          if (self.game.Data.UnitObj[self.tUnitSelected].autoMoveX > -1)
          {
            tsubpart2 =  new TextButtonPartClass("STOP", 180, "Click to get unit out of auto-move mode.",  self.OwnBitmap, 936, 657, theight: 60, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.deselectId = self.AddSubPart( tsubpart2, 936, 657, 180, 60, 1);
          }
          else
          {
            tsubpart2 =  new TextButtonPartClass("STOP", 180, "Unit is not in auto-move mode.",  self.OwnBitmap, 936, 657, true, theight: 60, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.deselectid2 = self.AddSubPart( tsubpart2, 936, 657, 180, 60, 0);
          }
        }
        if (self.game.EditObj.UnitSelected > -1)
        {
          if (self.game.Data.UnitObj[self.game.EditObj.UnitSelected].TempUnitSelectable)
          {
            if (self.usageMode == SelectUsageMode.joinAttack)
            {
              if (!self.game.EditObj.TempUnitList.CheckIfPresent(self.game.EditObj.UnitSelected))
              {
                tsubpart2 =  new TextButtonPartClass("SELECT", 200, "Click to select [SPACE].",  self.OwnBitmap, 330, 590, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
                self.okid = self.AddSubPart( tsubpart2, 330, 590, 200, 40, 1);
                tsubpart2 =  new TextButtonPartClass("DESELECT", 200, "Unit is not selected.",  self.OwnBitmap, 330, 650, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
                self.cancel2id = self.AddSubPart( tsubpart2, 330, 650, 200, 40, 0);
              }
              else
              {
                tsubpart2 =  new TextButtonPartClass("SELECT", 200, "This unit is already selected.",  self.OwnBitmap, 330, 590, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
                self.ok2id = self.AddSubPart( tsubpart2, 330, 590, 200, 40, 0);
                tsubpart2 =  new TextButtonPartClass("DESELECT", 200, "Click to deselect this unit. [SPACE]",  self.OwnBitmap, 330, 650, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
                self.cancelid = self.AddSubPart( tsubpart2, 330, 650, 200, 40, 1);
              }
            }
            else if (self.usageMode == SelectUsageMode.selectHQ & self.UL.Counter > -1)
            {
              tsubpart2 =  new TextButtonPartClass("SELECT", 100, "HQ is already selected.",  self.OwnBitmap, 450, 627, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
              self.ok2id = self.AddSubPart( tsubpart2, 450, 627, 100, 40, 0);
              tsubpart2 =  new TextButtonPartClass("DESELECT", 100, "HQ is already selected.",  self.OwnBitmap, 560, 627, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
              self.cancel2id = self.AddSubPart( tsubpart2, 560, 627, 100, 40, 0);
            }
          }
          else if (self.usageMode != SelectUsageMode.autoMove)
          {
            tsubpart2 =  new TextButtonPartClass("SELECT", 200, "This unit is already selected.",  self.OwnBitmap, 330, 590, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.ok2id = self.AddSubPart( tsubpart2, 330, 590, 200, 40, 0);
            tsubpart2 =  new TextButtonPartClass("DESELECT", 200, "Unit is not selected.",  self.OwnBitmap, 330, 650, true, theight: 40, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
            self.cancel2id = self.AddSubPart( tsubpart2, 330, 650, 200, 40, 0);
          }
        }
        graphics.Dispose();
        graphics = (Graphics) null;
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  self.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 =  self.SubPartID[index1];
            if (num1 == self.Pic1Id)
            {
              let mut selectX: i32 =  self.game.SelectX;
              let mut selectY: i32 =  self.game.SelectY;
              self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.game.EditObj.TempCoordList = CoordList::new();
              self.SubPartList[self.SubpartNr(self.mapid)].Paint();
              self.ViewMessage();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.OptionsListId)
            {
              let mut index2: i32 =  self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (index2 > -1)
              {
                self.SubpartNr(self.mapid);
                index3: i32;
                if (self.usageMode == SelectUsageMode.autoMove)
                {
                  self.game.SelectX = self.HL.coord[index2].x;
                  self.game.SelectY = self.HL.coord[index2].y;
                }
                else if (self.usageMode == SelectUsageMode.repairBridge | self.usageMode == SelectUsageMode.blowBridge)
                {
                  let mut counter: i32 =  self.HL.counter;
                  for (let mut index4: i32 =  0; index4 <= counter; index4 += 1)
                  {
                    if (self.HL.coord[index4].data1 == index2)
                    {
                      self.game.SelectX = self.HL.coord[index4].x;
                      self.game.SelectY = self.HL.coord[index4].y;
                    }
                  }
                }
                else
                {
                  index3 = index2;
                  self.game.SelectX = self.game.Data.UnitObj[index3].X;
                  self.game.SelectY = self.game.Data.UnitObj[index3].Y;
                }
                let mut zoom: i32 =  self.game.EditObj.Zoom;
                self.game.EditObj.Zoom = 0;
                self.game.HandyFunctionsObj.CenterOnXY(self.game.SelectX, self.game.SelectY, useWidth: 939, useHeight: 538);
                self.game.EditObj.Zoom = zoom;
                if (self.game.EditObj.UnitSelected != index3)
                  self.game.EditObj.SFSelected = -1;
                self.game.EditObj.UnitSelected = index3;
                self.SubPartList[self.SubpartNr(self.mapid)].Paint();
                self.ViewMessage();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.mapid)
            {
              let mut selectX: i32 =  self.game.SelectX;
              let mut selectY: i32 =  self.game.SelectY;
              Coordinate coordinate = self.SubPartList[index1].ClickMap(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (coordinate.onmap)
              {
                self.game.SelectX = coordinate.x;
                self.game.SelectY = coordinate.y;
                self.game.EditObj.TempCoordList = CoordList::new();
                let mut num2: i32 =  !(selectX == self.game.SelectX & selectY == self.game.SelectY) ? self.game.HandyFunctionsObj.ClickOnHexGivesUnit(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected, true, b, coordinate.data1, coordinate.penalty, true) : self.game.HandyFunctionsObj.ClickOnHexGivesUnit(self.game.SelectX, self.game.SelectY, self.game.EditObj.MapSelected, false, b, coordinate.data1, coordinate.penalty, true);
                if (self.game.EditObj.UnitSelected != num2)
                  self.game.EditObj.SFSelected = -1;
                self.game.EditObj.UnitSelected = num2;
                self.SubPartList[self.SubpartNr(self.mapid)].Paint();
                self.ViewMessage();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.deselectId)
            {
              self.game.ProcessingObj.AutoMoveChange(self.tUnitSelected, false, self.subordinatesToo, -1, -1);
              self.game.EditObj.AreaX = -1;
              self.game.EditObj.AreaY = -1;
              self.game.EditObj.DoCardSlot = -1;
              self.game.CornerX = self.tCornerX;
              self.game.CornerY = self.tCornerY;
              self.game.SelectX = self.tSelectX;
              self.game.SelectY = self.tSelectY;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              self.game.EditObj.MapSelected = self.tSelectMap;
              self.game.EditObj.TempCoordList = CoordList::new();
              self.game.EditObj.AreaSlot = -1;
              let mut unitCounter: i32 =  self.game.Data.UnitCounter;
              for (let mut index5: i32 =  0; index5 <= unitCounter; index5 += 1)
                self.game.Data.UnitObj[index5].TempUnitSelectable = false;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              DrawMod.GetWidthForMiniMap();
              self.game.EditObj.MiniMap = (Bitmap) null;
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: self.tMiniWidth, ty: self.tMiniHeight);
              self.game.HandyFunctionsObj.RedimTempValue(9999);
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.FinalId)
            {
              self.targetUnitSelected = self.game.EditObj.UnitSelected;
              self.resultString = "";
              if (self.usageMode == SelectUsageMode.selectHQ)
                self.game.ProcessingObj.SetUnitHq(self.tUnitSelected, self.targetUnitSelected);
              if (self.usageMode == SelectUsageMode.autoMove)
                self.resultString = self.game.ProcessingObj.AutoMoveChange(self.tUnitSelected, true, self.subordinatesToo, self.game.SelectX, self.game.SelectY);
              if (self.usageMode == SelectUsageMode.blowBridge)
              {
                let mut slot: i32 =  self.HL.FindSlot(self.game.SelectX, self.game.SelectY, 0);
                self.game.EditObj.UnitSelected = self.tUnitSelected;
                self.game.EditObj.form3windowClass.form3_orderResult = self.game.ProcessingObj.BlowBridge(self.tUnitSelected, self.tSelectX, self.tSelectY, 0, self.HL.coord[slot].data1 - 1);
                self.game.EditObj.form3windowClass.form3_returnInstruction = true;
                self.game.EditObj.form3windowClass.form3_data1 = 35;
              }
              if (self.usageMode == SelectUsageMode.repairBridge)
              {
                let mut slot: i32 =  self.HL.FindSlot(self.game.SelectX, self.game.SelectY, 0);
                self.game.EditObj.UnitSelected = self.tUnitSelected;
                self.game.EditObj.form3windowClass.form3_orderResult = self.game.ProcessingObj.BuildInfra(self.game.EditObj.UnitSelected, self.tSelectX, self.tSelectY, 0, self.HL.coord[slot].data1 - 1);
                self.game.EditObj.form3windowClass.form3_returnInstruction = true;
                self.game.EditObj.form3windowClass.form3_data1 = 36;
              }
              if (self.resultString.Length > 1)
              {
                if (self.OptionsListId > 0)
                {
                  self.RemoveSubPart(self.OptionsListId);
                  self.OptionsListId = 0;
                }
                if (self.mapid > 0)
                {
                  self.RemoveSubPart(self.mapid);
                  self.mapid = 0;
                }
                if (self.Pic1Id > 0)
                {
                  self.RemoveSubPart(self.Pic1Id);
                  self.Pic1Id = 0;
                }
                self.ViewMessage();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.EditObj.AreaX = -1;
              self.game.EditObj.AreaY = -1;
              self.game.EditObj.DoCardSlot = -1;
              self.game.CornerX = self.tCornerX;
              self.game.CornerY = self.tCornerY;
              self.game.SelectX = self.tSelectX;
              self.game.SelectY = self.tSelectY;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              self.game.EditObj.MapSelected = self.tSelectMap;
              self.game.EditObj.TempCoordList = CoordList::new();
              self.game.EditObj.AreaSlot = -1;
              let mut unitCounter: i32 =  self.game.Data.UnitCounter;
              for (let mut index6: i32 =  0; index6 <= unitCounter; index6 += 1)
                self.game.Data.UnitObj[index6].TempUnitSelectable = false;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              DrawMod.GetWidthForMiniMap();
              self.game.EditObj.MiniMap = (Bitmap) null;
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: self.tMiniWidth, ty: self.tMiniHeight);
              self.game.HandyFunctionsObj.RedimTempValue(9999);
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.resultMesOkId)
            {
              self.game.EditObj.AreaX = -1;
              self.game.EditObj.AreaY = -1;
              self.game.EditObj.DoCardSlot = -1;
              self.game.CornerX = self.tCornerX;
              self.game.CornerY = self.tCornerY;
              self.game.SelectX = self.tSelectX;
              self.game.SelectY = self.tSelectY;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              self.game.EditObj.MapSelected = self.tSelectMap;
              self.game.EditObj.TempCoordList = CoordList::new();
              self.game.EditObj.AreaSlot = -1;
              let mut unitCounter: i32 =  self.game.Data.UnitCounter;
              for (let mut index7: i32 =  0; index7 <= unitCounter; index7 += 1)
                self.game.Data.UnitObj[index7].TempUnitSelectable = false;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              DrawMod.GetWidthForMiniMap();
              self.game.EditObj.MiniMap = (Bitmap) null;
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: self.tMiniWidth, ty: self.tMiniHeight);
              self.game.HandyFunctionsObj.RedimTempValue(9999);
              windowReturnClass.AddCommand(6, 0);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.quitId)
            {
              self.game.EditObj.AreaX = -1;
              self.game.EditObj.AreaY = -1;
              self.game.EditObj.DoCardSlot = -1;
              self.game.CornerX = self.tCornerX;
              self.game.CornerY = self.tCornerY;
              self.game.SelectX = self.tSelectX;
              self.game.SelectY = self.tSelectY;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              self.game.EditObj.MapSelected = self.tSelectMap;
              self.game.EditObj.TempCoordList = CoordList::new();
              self.game.EditObj.AreaSlot = -1;
              let mut unitCounter: i32 =  self.game.Data.UnitCounter;
              for (let mut index8: i32 =  0; index8 <= unitCounter; index8 += 1)
                self.game.Data.UnitObj[index8].TempUnitSelectable = false;
              self.game.EditObj.UnitSelected = self.tUnitSelected;
              DrawMod.GetWidthForMiniMap();
              self.game.EditObj.MiniMap = (Bitmap) null;
              MiniMapPartClass miniMapPartClass = new MiniMapPartClass(DrawMod.TGame, tx: self.tMiniWidth, ty: self.tMiniHeight);
              self.game.HandyFunctionsObj.RedimTempValue(9999);
              if (self.usageMode == SelectUsageMode.autoMove | self.usageMode == SelectUsageMode.selectHQ | self.usageMode == SelectUsageMode.blowBridge | self.usageMode == SelectUsageMode.repairBridge)
              {
                windowReturnClass.AddCommand(6, 0);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              self.game.EditObj.PopupValue = 22;
              windowReturnClass.AddCommand(5, 14);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.textAreaId)
            {
              self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.cancelid)
            {
              if (self.usageMode == SelectUsageMode.joinAttack)
                self.game.EditObj.TempUnitList.remove(self.game.EditObj.UnitSelected);
              self.ViewMessage();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.subId)
            {
              self.subordinatesToo = !self.subordinatesToo;
              self.ViewMessage();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.okid)
            {
              if (self.usageMode == SelectUsageMode.joinAttack)
                self.game.EditObj.TempUnitList.add(self.game.EditObj.UnitSelected);
              self.ViewMessage();
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
      let mut cornerX: i32 =  self.game.CornerX;
      let mut cornerY: i32 =  self.game.CornerY;
      if (nr == 27)
        return self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.quitId)] + 1, self.SubPartY[self.SubpartNr(self.quitId)] + 1, 1);
      if (nr == 32 & self.okid > 0)
        return self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.okid)] + 1, self.SubPartY[self.SubpartNr(self.okid)] + 1, 1);
      return nr == 32 & self.cancelid > 0 ? self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.cancelid)] + 1, self.SubPartY[self.SubpartNr(self.cancelid)] + 1, 1) : windowReturnClass;
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      bool flag = false;
      let mut cornerX: i32 =  self.game.CornerX;
      let mut cornerY: i32 =  self.game.CornerY;
      if (nr == 39)
      {
        this += 1.game.CornerX;
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
        this += 1.game.CornerY;
        flag = true;
      }
      if (nr == 38 & self.game.CornerY > 0)
      {
        --self.game.CornerY;
        if (self.game.CornerY < 0)
          self.game.CornerY = 0;
        flag = true;
      }
      let mut num1: i32 =  230;
      if (self.game.Data.Round == 0)
        num1 += 100;
      let mut num2: i32 =   Math.Round(Conversion.Int( (self.OwnBitmap.Width - 250) /  (53 * (self.game.EditObj.Zoom + 1))));
      let mut num3: i32 =   Math.Round(Conversion.Int( (self.OwnBitmap.Height - num1) /  (48 * (self.game.EditObj.Zoom + 1))));
      let mut num4: i32 =  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapWidth - self.game.CornerX + 1;
      let mut num5: i32 =  self.game.Data.MapObj[self.game.EditObj.MapSelected].MapHeight - self.game.CornerY + 1;
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
