// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleOfficerWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Runtime.CompilerServices;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleOfficerWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     cellinfoid: i32;
     tableId: i32;
     loadId: i32;
     text1id: i32;
     loadMasterId: i32;
     detailnr: i32;
     addId: i32;
     versionid: i32;
     addReinf: i32;
     exportCsv: i32;
     importCsv: i32;
     removeReinf: i32;
     RenameReinf: i32;
     RenameReinfb: i32;
     RemoveReinfb: i32;
     a1id: i32;
     a2id: i32;
     a3id: i32;
     removeId: i32;
     changeId: i32;
     exitId: i32;
     saveId: i32;
     editId: i32;
     removeIdb: i32;
     saveIdb: i32;
     tab1id: i32;
     tab2id: i32;
     tab3id: i32;
     tab4id: i32;
     tab1idb: i32;
     tab2idb: i32;
     tab3idb: i32;
     tab4idb: i32;
     editIdb: i32;
     strId: i32;
     detailx: i32;
     detaily: i32;
     tabId: i32;
     StringListClass stringy;
     VarsStartOn: i32;
     bool AddNew;
     bool Change;
     currentPplNr: i32;
     currentUnitNr: i32;
     currentHisNr: i32;
     currentInstNr: i32;
     int[] ColIsVar;
     masterfileStart: String;
     oldTopX: i32;
     oldTopY: i32;

    pub SimpleOfficerWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight, 9, tDoBorders: 1, tHeaderString: "Intermediate Officer Editor")
    {
      self.ColIsVar = new int[100];
      self.detailx = -1;
      self.detaily = -1;
      self.detailnr = -1;
      self.tabId = 1;
      self.currentPplNr = -1;
      self.currentUnitNr = -1;
      self.currentHisNr = -1;
      self.currentInstNr = -1;
      self.AddNew = false;
      self.Change = false;
      self.masterfileStart = self.game.Data.MasterFile;
      self.game.EditObj.TempSFType = -1;
      self.DoStuff();
    }

    pub fn RefreshCellInfo()
    {
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      txt: String;
      if (self.detaily == -1 | self.detailx == -1)
        txt = "No cell selected";
      else
        txt = "(row" + self.detailx.ToString() + ",col" + self.detaily.ToString() + ")             " + self.stringy.ColumnName[self.detaily].ToUpper() + "                " + self.stringy.Data[self.detailx, self.detaily];
      let mut tsubpart: SubPartClass =  TextPartClass::new(txt, self.game.MarcFont4, self.game.ScreenWidth - 323, 20, false, tMarc: true);
      self.cellinfoid = self.AddSubPart( tsubpart, 312, 152, self.game.ScreenWidth - 323, 20, 0);
    }

    pub fn DoRefresh()
    {
      if (!(self.Change & (self.currentUnitNr > -1 | self.currentHisNr > -1 | self.currentInstNr > -1 | self.currentPplNr > -1)))
        return;
      if (self.tableId > 0)
        self.RemoveSubPart(self.tableId);
      self.tableId = 0;
      self.Change = false;
      self.currentUnitNr = -1;
      self.currentHisNr = -1;
      self.currentPplNr = -1;
      self.currentInstNr = -1;
      self.DoStuff();
    }

    pub fn PopUpRefresh() => self.DoStuff();

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight, -1);
      if (self.versionid > 0)
        self.RemoveSubPart(self.versionid);
      if (self.addId > 0)
        self.RemoveSubPart(self.addId);
      if (self.removeId > 0)
        self.RemoveSubPart(self.removeId);
      if (self.editId > 0)
        self.RemoveSubPart(self.editId);
      if (self.saveId > 0)
        self.RemoveSubPart(self.saveId);
      if (self.removeIdb > 0)
        self.RemoveSubPart(self.removeIdb);
      if (self.editIdb > 0)
        self.RemoveSubPart(self.editIdb);
      if (self.saveIdb > 0)
        self.RemoveSubPart(self.saveIdb);
      if (self.exitId > 0)
        self.RemoveSubPart(self.exitId);
      if (self.a1id > 0)
        self.RemoveSubPart(self.a1id);
      if (self.a2id > 0)
        self.RemoveSubPart(self.a2id);
      if (self.a3id > 0)
        self.RemoveSubPart(self.a3id);
      if (self.cellinfoid > 0)
      {
        self.RemoveSubPart(self.cellinfoid);
        self.cellinfoid = 0;
      }
      if (self.loadId > 0)
        self.RemoveSubPart(self.loadId);
      if (self.tab1id > 0)
        self.RemoveSubPart(self.tab1id);
      if (self.tab2id > 0)
        self.RemoveSubPart(self.tab2id);
      if (self.tab3id > 0)
        self.RemoveSubPart(self.tab3id);
      if (self.tab4id > 0)
        self.RemoveSubPart(self.tab4id);
      if (self.tab1idb > 0)
        self.RemoveSubPart(self.tab1idb);
      if (self.tab2idb > 0)
        self.RemoveSubPart(self.tab2idb);
      if (self.tab3idb > 0)
        self.RemoveSubPart(self.tab3idb);
      if (self.tab4idb > 0)
        self.RemoveSubPart(self.tab4idb);
      if (self.loadMasterId > 0)
        self.RemoveSubPart(self.loadMasterId);
      if (self.addReinf > 0)
        self.RemoveSubPart(self.addReinf);
      if (self.removeReinf > 0)
        self.RemoveSubPart(self.removeReinf);
      if (self.RenameReinf > 0)
        self.RemoveSubPart(self.RenameReinf);
      if (self.RemoveReinfb > 0)
        self.RemoveSubPart(self.RemoveReinfb);
      if (self.RenameReinfb > 0)
        self.RemoveSubPart(self.RenameReinfb);
      if (self.exportCsv > 0)
        self.RemoveSubPart(self.exportCsv);
      if (self.importCsv > 0)
        self.RemoveSubPart(self.importCsv);
      if (self.listId > 0)
        self.RemoveSubPart(self.listId);
      if (self.game.Data.LibraryCounter == -1)
      {
        self.game.Data.AddLibrary();
        self.game.Data.LibraryObj[0].name = "New Officer Library";
        self.game.Data.LibraryObj[0].information = "no info specified";
        self.game.Data.LibraryObj[0].creator = "no creator specified";
      }
      if (self.game.Data.RegimeCounter == -1)
      {
        self.game.Data.AddRegime();
        self.game.Data.RegimeObj[0].Name = "Only regime in library";
      }
      self.game.Data.RegimeObj[0].libId.libSlot = 0;
      self.game.Data.RegimeObj[0].libId.id = -1;
      if (self.game.Data.PeopleCounter == -1)
      {
        self.game.Data.AddPeople();
        self.game.Data.PeopleObj[0].Name = "Only people in library";
      }
      else
        self.game.Data.PeopleObj[0].Name = "Only people in library";
      DrawMod.DrawTextColouredMarc( graphics, "Name:", self.game.MarcFont4, num1 + 10, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, self.game.Data.LibraryObj[0].name, self.game.MarcFont3, num1 + 10, 75, Color.White);
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Change Library Name", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a1id = self.AddSubPart( tsubpart1, num1 + 10, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "Author:", self.game.MarcFont4, num1 + 310, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, self.game.Data.LibraryObj[0].creator, self.game.MarcFont3, num1 + 310, 75, Color.White);
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Change Author", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 310), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a2id = self.AddSubPart( tsubpart2, num1 + 310, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "Information:", self.game.MarcFont4, num1 + 610, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, Strings.Left(self.game.Data.LibraryObj[0].information, 15) + "...", self.game.MarcFont3, num1 + 610, 75, Color.White);
      tsubpart2 =  new TextButtonPartClass("Change information", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 610), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.a3id = self.AddSubPart( tsubpart2, num1 + 610, 100, 190, 35, 1);
      DrawMod.DrawTextColouredMarc( graphics, "Version:", self.game.MarcFont4, num1 + 910, 60, Color.White);
      DrawMod.DrawTextColouredMarc( graphics, self.game.Data.LibraryObj[0].version.ToString(), self.game.MarcFont3, num1 + 910, 75, Color.White);
      tsubpart2 =  new TextButtonPartClass("Change version", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 910), bby: 100, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.versionid = self.AddSubPart( tsubpart2, num1 + 910, 100, 190, 35, 1);
      if (self.detailx > -1 & self.detaily > -1 & !Information.IsNothing( self.stringy))
        self.RefreshCellInfo();
      let mut y1: i32 = 60;
      let mut num3: i32 = 40;
      DrawMod.DrawTextColouredMarc( graphics, "Ops:", self.game.MarcFont3, 40, y1, Color.White);
      let mut num4: i32 = y1 + 40;
      if (self.game.Data.PeopleCounter > -1)
      {
        tsubpart2 =  new TextButtonPartClass("Save Officer Library", 190, "Save a officer Library",  self.OwnBitmap, num3, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveId = self.AddSubPart( tsubpart2, num3, num4, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Save Officer Library", 190, "Nothing to save",  self.OwnBitmap, num3, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.saveIdb = self.AddSubPart( tsubpart2, num3, num4, 190, 35, 1);
      }
      let mut num5: i32 = num4 + 50;
      tsubpart2 =  new TextButtonPartClass("Load Officer Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: num5, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadId = self.AddSubPart( tsubpart2, num3, num5, 190, 35, 1);
      let mut num6: i32 = num5 + 50;
      tsubpart2 =  new TextButtonPartClass("Exit", 190, tBackbitmap: ( self.OwnBitmap), bbx: num3, bby: num6, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exitId = self.AddSubPart( tsubpart2, num3, num6, 190, 35, 1);
      let mut y2: i32 = num6 + 50;
      DrawMod.DrawTextColouredMarc( graphics, "Officer Card Libraries:", self.game.MarcFont3, 40, y2, Color.White);
      let mut y3: i32 = y2 + 40;
      self.listObj = ListClass::new();
      let mut num7: i32 = -1;
      let mut num8: i32 = -1;
      let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
      for (let mut tdata: i32 = 0; tdata <= libraryCounter; tdata += 1)
      {
        bool flag = false;
        let mut actionCardCounter: i32 = self.game.Data.ActionCardCounter;
        for (let mut index: i32 = 0; index <= actionCardCounter; index += 1)
        {
          if (self.game.Data.ActionCardObj[index].LibId.libSlot == tdata)
            flag = true;
        }
        if (flag)
        {
          num7 += 1;
          if (self.detailnr == tdata)
            num8 = num7;
          self.listObj.add(self.game.Data.LibraryObj[tdata].name, tdata);
        }
      }
      let mut smallPicCounter: i32 = self.game.Data.SmallPicCounter;
      for (let mut index: i32 = 0; index <= smallPicCounter; index += 1)
      {
        self.game.Data.SmallLibId[index].libSlot = -1;
        self.game.Data.SmallLibId[index].id = -1;
      }
      let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
      for (let mut index: i32 = 0; index <= historicalUnitCounter; index += 1)
      {
        if (self.game.Data.HistoricalUnitObj[index].CommanderName.Length > 0)
        {
          self.game.Data.HistoricalUnitObj[index].LibId.libSlot = 0;
          self.game.Data.HistoricalUnitObj[index].LibId.id = -1;
        }
      }
      ListClass listObj = self.listObj;
      let mut tlistselect: i32 = num8;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      let mut bby: i32 = y3;
      font: Font =  null;
       local2: Font =  font;
      tsubpart2 =  new ListSubPartClass(listObj, 6, 210, tlistselect, game, true, "Officer Card Libraries", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: 40, bby: bby, tMarcStyle: true, overruleFont: ( local2));
      self.listId = self.AddSubPart( tsubpart2, 40, y3, 210, 144, 0);
      let mut num9: i32 = y3 + 135;
      tsubpart2 =  new TextButtonPartClass("Add Off Card Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: 40, bby: num9, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addReinf = self.AddSubPart( tsubpart2, 40, num9, 190, 35, 1);
      let mut num10: i32 = num9 + 50;
      if (self.detailnr > -1)
      {
        tsubpart2 =  new TextButtonPartClass("Remove Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: 40, bby: num10, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeReinf = self.AddSubPart( tsubpart2, 40, num10, 190, 35, 1);
      }
      else
      {
        tsubpart2 =  new TextButtonPartClass("Remove Library", 190, tBackbitmap: ( self.OwnBitmap), bbx: 40, bby: num10, tinactive: true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RemoveReinfb = self.AddSubPart( tsubpart2, 40, num10, 190, 35, 1);
      }
      let mut num11: i32 = num10 + 50;
      tsubpart2 =  new TextButtonPartClass("Reload Master", 190, "For if you want to update this library with the latest masterfile.",  self.OwnBitmap, 40, num11, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadMasterId = self.AddSubPart( tsubpart2, 40, num11, 190, 35, 1);
      if (self.tabId != 1)
        return;
      self.Tab1( graphics);
    }

    pub fn Tab1( Graphics g)
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768;
      let mut num3: i32 =  Math.Round(Conversion.Int( (self.game.ScreenHeight - 322) / 24.0)) - 1;
      let mut num4: i32 = 172 + num3 * 24 + 56;
      if (self.tableId == 0)
      {
        self.stringy = new StringListClass(-1);
        let mut col1: i32 = 1 - 1;
        self.stringy.AddCol(col1, "Slot#");
        let mut col2: i32 = col1 + 1;
        self.stringy.AddCol(col2, "ID#");
        let mut col3: i32 = col2 + 1;
        self.stringy.AddCol(col3, "Name");
        let mut col4: i32 = col3 + 1;
        self.stringy.AddCol(col4, "Portrait");
        let mut col5: i32 = col4 + 1;
        self.stringy.AddCol(col5, "Description");
        let mut col6: i32 = col5 + 1;
        self.stringy.AddCol(col6, "CombatMod");
        let mut col7: i32 = col6 + 1;
        self.stringy.AddCol(col7, "MoraleMod");
        let mut col8: i32 = col7 + 1;
        self.stringy.AddCol(col8, "Staff Pts");
        let mut col9: i32 = col8 + 1;
        self.stringy.AddCol(col9, "PP");
        let mut col10: i32 = col9 + 1;
        self.stringy.AddCol(col10, "Card1");
        let mut col11: i32 = col10 + 1;
        self.stringy.AddCol(col11, "Card2");
        let mut col12: i32 = col11 + 1;
        self.stringy.AddCol(col12, "Card3");
        let mut col13: i32 = col12 + 1;
        self.stringy.AddCol(col13, "Card4");
        let mut col14: i32 = col13 + 1;
        self.stringy.AddCol(col14, "Card5");
        let mut col15: i32 = col14 + 1;
        self.stringy.AddCol(col15, "Card6");
        let mut col16: i32 = col15 + 1;
        self.stringy.AddCol(col16, "Card7");
        let mut col17: i32 = col16 + 1;
        self.stringy.AddCol(col17, "Card8");
        let mut num5: i32 = 0;
        do
        {
          if (self.game.Data.TempString[1200 + num5].Length > 0)
          {
            col17 += 1;
            self.stringy.AddCol(col17, self.game.Data.TempString[1200 + num5]);
            self.ColIsVar[col17] = num5;
          }
          num5 += 1;
        }
        while (num5 <= 99);
        let mut index1: i32 = -1;
        let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut index2: i32 = 0; index2 <= historicalUnitCounter; index2 += 1)
        {
          if (self.game.Data.HistoricalUnitObj[index2].CommanderName.Length > 0)
          {
            self.game.Data.HistoricalUnitObj[index2].TempRegime = 0;
            self.game.Data.HistoricalUnitObj[index2].People = 0;
            index1 += 1;
            self.stringy.AddRow(index1 - 1);
            let mut index3: i32 = 1 - 1;
            self.stringy.Data[index1, index3] = index2.ToString();
            let mut index4: i32 = index3 + 1;
            self.stringy.Data[index1, index4] = self.game.Data.HistoricalUnitObj[index2].ID.ToString();
            let mut index5: i32 = index4 + 1;
            self.stringy.Data[index1, index5] = self.game.Data.HistoricalUnitObj[index2].CommanderName;
            let mut index6: i32 = index5 + 1;
            self.stringy.Data[index1, index6] = self.game.Data.HistoricalUnitObj[index2].CommanderFileName;
            if (self.game.Data.HistoricalUnitObj[index2].CommanderSpriteID > -1)
              self.stringy.TempBmp[index1, index6] = self.game.Data.HistoricalUnitObj[index2].CommanderSpriteID;
            let mut index7: i32 = index6 + 1;
            self.stringy.Data[index1, index7] = "DESCRIPT";
            let mut index8: i32 = index7 + 1;
            self.stringy.Data[index1, index8] = self.game.Data.HistoricalUnitObj[index2].CombatMod.ToString();
            let mut index9: i32 = index8 + 1;
            self.stringy.Data[index1, index9] = self.game.Data.HistoricalUnitObj[index2].MoraleMod.ToString();
            let mut index10: i32 = index9 + 1;
            self.stringy.Data[index1, index10] = self.game.Data.HistoricalUnitObj[index2].StaffSize.ToString();
            let mut index11: i32 = index10 + 1;
            self.stringy.Data[index1, index11] = self.game.Data.HistoricalUnitObj[index2].PP.ToString();
            let mut index12: i32 = 0;
            do
            {
              if (index12 <= self.game.Data.HistoricalUnitObj[index2].DeckCardCounter)
              {
                index11 += 1;
                self.stringy.Data[index1, index11] = self.game.Data.ActionCardObj[self.game.Data.HistoricalUnitObj[index2].DeckCard[index12]].Title;
              }
              else
              {
                index11 += 1;
                self.stringy.Data[index1, index11] = "none";
              }
              index12 += 1;
            }
            while (index12 <= 7);
            let mut typ: i32 = 0;
            do
            {
              if (self.game.Data.TempString[1200 + typ].Length > 0)
              {
                index11 += 1;
                self.stringy.Data[index1, index11] = Conversions.ToString(self.game.Data.HistoricalUnitObj[index2].GiveHisVarValue(typ));
                self.game.Data.HistoricalUnitObj[index2].SetHisVarValue(typ, self.game.Data.HistoricalUnitObj[index2].GiveHisVarValue(typ),  Math.Round(Conversion.Val(self.game.Data.TempString[1300 + typ])));
              }
              typ += 1;
            }
            while (typ <= 99);
          }
        }
        let mut tsubpart: SubPartClass =  new MatrixSubPartClass(self.stringy, num3 - 1, self.game.ScreenWidth - 323, self.detailx, self.detaily, self.game, tbackbitmap: ( self.BackBitmap), bbx: 312, bby: 172, trowheight: 24, tfontsize: 16, tfontoffsety: 4, tMarcy: true);
        self.tableId = self.AddSubPart( tsubpart, 312, 172, self.game.ScreenWidth - 323, num3 * 25, 0);
        if (self.oldTopX > 0 | self.oldTopY > 0)
        {
          ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemX = self.oldTopX;
          ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemY = self.oldTopY;
        }
      }
      else
      {
        self.oldTopX = ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemX;
        self.oldTopY = ((MatrixSubPartClass) self.SubPartList[self.SubpartNr(self.tableId)]).TopItemY;
      }
      let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Add Officer", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 10), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.addId = self.AddSubPart( tsubpart1, num1 + 10, num4, 190, 35, 1);
      if (self.detailx > -1)
      {
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Remove Officer", 190, "Click and remove selected row",  self.OwnBitmap, num1 + 210, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeId = self.AddSubPart( tsubpart2, num1 + 210, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Remove Officer", 190, "No row selected",  self.OwnBitmap, num1 + 210, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeIdb = self.AddSubPart( tsubpart3, num1 + 210, num4, 190, 35, 1);
      }
      if (self.detailx > -1 & self.detaily > -1)
      {
        let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "Click to edit selected value or text",  self.OwnBitmap, num1 + 410, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editId = self.AddSubPart( tsubpart4, num1 + 410, num4, 190, 35, 1);
      }
      else
      {
        let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Change Cell", 190, "No cell selected",  self.OwnBitmap, num1 + 410, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.editIdb = self.AddSubPart( tsubpart5, num1 + 410, num4, 190, 35, 1);
      }
      let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Export CSV", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 610), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.exportCsv = self.AddSubPart( tsubpart6, num1 + 610, num4, 190, 35, 1);
      let mut tsubpart7: SubPartClass =  new TextButtonPartClass("Import CSV", 190, tBackbitmap: ( self.OwnBitmap), bbx: (num1 + 810), bby: num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.importCsv = self.AddSubPart( tsubpart7, num1 + 810, num4, 190, 35, 1);
      let mut peopleCounter: i32 = self.game.Data.PeopleCounter;
      for (let mut index: i32 = 0; index <= peopleCounter; index += 1)
      {
        self.game.Data.PeopleObj[index].LibId.libSlot = 0;
        self.game.Data.PeopleObj[index].LibId.id = -1;
      }
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.strId == -1 || Information.IsNothing( self.stringy))
        return windowReturnClass1;
      if (nr == 32 & self.detailx > -1 & self.editId > 0)
      {
        windowReturnClass2: WindowReturnClass = self.HandleMouseClick(self.SubPartX[self.SubpartNr(self.editId)] + 1, self.SubPartY[self.SubpartNr(self.editId)] + 1, 1);
        windowReturnClass2.SetFlag(true);
        return windowReturnClass2;
      }
      if (nr == 38 & self.detailx > 0)
      {
        --self.detailx;
        self.RefreshCellInfo();
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftUp();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 40 & self.detailx < self.stringy.Length)
      {
        this += 1.detailx;
        self.RefreshCellInfo();
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftDown();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (nr == 37 & self.detaily > 0 & self.stringy.Length > -1 & self.detailx > -1)
      {
        --self.detaily;
        self.RefreshCellInfo();
        self.SubPartList[self.SubpartNr(self.tableId)].ShiftLeft();
        self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
        windowReturnClass1.SetFlag(true);
        return windowReturnClass1;
      }
      if (!(nr == 39 & self.detaily < self.stringy.Width & self.stringy.Length > -1 & self.detailx > -1))
        return windowReturnClass1;
      this += 1.detaily;
      self.RefreshCellInfo();
      self.SubPartList[self.SubpartNr(self.tableId)].ShiftRight();
      self.SubPartFlag[self.SubpartNr(self.tableId)] = true;
      windowReturnClass1.SetFlag(true);
      return windowReturnClass1;
    }

    [MethodImpl(MethodImplOptions.NoInlining | MethodImplOptions.NoOptimization)]
    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > self.SubPartX[index1] & x < self.SubPartX[index1] + self.SubPartW[index1] && y > self.SubPartY[index1] & y < self.SubPartY[index1] + self.SubPartH[index1])
          {
            let mut num1: i32 = self.SubPartID[index1];
            if (num1 == self.loadMasterId)
            {
              str1: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  str1 = str1.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  str1 = str1.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                str1 = str1.Replace("scenarios", self.game.ModScenarioDir);
              str2: String = str1 + self.masterfileStart;
              if (File.Exists(str2))
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.HandyFunctionsObj.LoadMasterFile(str2);
                self.game.Data.Round = 0;
                self.game.Data.Turn = 0;
                if ( self.game.Data.RuleVar[344] == 1.0 & self.game.EditObj.HideUnit == 0)
                  self.game.EditObj.HideUnit = 2;
                self.game.EditObj.TempValue = new MapMatrix2[self.game.Data.MapCounter + 1];
                self.game.EditObj.TempValue2 = new MapMatrix2[self.game.Data.MapCounter + 1];
                let mut mapCounter: i32 = self.game.Data.MapCounter;
                for (let mut index2: i32 = 0; index2 <= mapCounter; index2 += 1)
                {
                  self.game.EditObj.TempValue[index2] = new MapMatrix2(self.game.Data.MapObj[index2].MapWidth, self.game.Data.MapObj[index2].MapHeight);
                  self.game.EditObj.TempValue2[index2] = new MapMatrix2(self.game.Data.MapObj[index2].MapWidth, self.game.Data.MapObj[index2].MapHeight);
                }
                if (Strings.Len(self.game.Data.LoadPass) > 0)
                {
                  self.game.FormRef.Cursor = Cursors.Default;
                  if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(self.game.Data.LoadPass), false) == 0)
                  {
                    let mut num2: i32 =  Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    let mut num3: i32 =  Interaction.MsgBox( "Wrong Password. You cannot Load this file. Exiting whole app.", Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.EndApp();
                  }
                }
                BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                self.game.Data.LoadGraphics((Form1) null);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                self.game.EditObj.StratMap = new Bitmap(self.game.ScreenWidth, self.game.ScreenHeight - 265);
                self.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.StratMap, self.game.ScreenWidth, self.game.ScreenHeight - 265, false, true, false);
                self.game.FormRef.Cursor = Cursors.Default;
                let mut num4: i32 =  Interaction.MsgBox( "Loaded Master", Title: ( "Shadow Empire : Planetary Conquest"));
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.loadId)
            {
              tinitdir: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Officer library(*.se1off)|*.se1off", "Pick a officer library...", tinitdir, false);
              if (File.Exists(str))
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.EditObj.TutMode = false;
                if (self.game.Data.UseAI == 1)
                  self.game.NewAIObj.LastRegime = -1;
                self.game.SelectX = -1;
                self.game.SelectY = -1;
                self.game.Data = DataClass::new();
                GC.Collect();
                Application.DoEvents();
                self.game.HandyFunctionsObj.Unzip(str);
                self.game.Data = DataClass.deserialize(str);
                self.game.HandyFunctionsObj.ZipFile(str);
                if (Operators.CompareString(self.game.Data.MasterFile, "", false) == 0)
                {
                  if (Strings.Len(self.masterfileStart) > 0 && Interaction.MsgBox( ("Update data with masterfile '" + self.masterfileStart + "' data"), MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
                  {
                    self.game.Data.MasterfileReadPeople = false;
                    masterfileStart: String = self.masterfileStart;
                    self.game.HandyFunctionsObj.LoadMasterFile(self.game.HandyFunctionsObj.ReturnLongMaster(str, masterfileStart));
                    self.game.Data.MasterFile = self.masterfileStart;
                  }
                }
                else
                {
                  self.game.Data.MasterfileReadPeople = false;
                  masterFile: String = self.game.Data.MasterFile;
                  self.game.HandyFunctionsObj.LoadMasterFile(self.game.HandyFunctionsObj.ReturnLongMaster(str, masterFile));
                }
                self.game.Data.Round = 0;
                self.game.Data.Turn = 0;
                if ( self.game.Data.RuleVar[344] == 1.0 & self.game.EditObj.HideUnit == 0)
                  self.game.EditObj.HideUnit = 2;
                self.game.EditObj.TempValue = new MapMatrix2[self.game.Data.MapCounter + 1];
                self.game.EditObj.TempValue2 = new MapMatrix2[self.game.Data.MapCounter + 1];
                let mut mapCounter: i32 = self.game.Data.MapCounter;
                for (let mut index3: i32 = 0; index3 <= mapCounter; index3 += 1)
                {
                  self.game.EditObj.TempValue[index3] = new MapMatrix2(self.game.Data.MapObj[index3].MapWidth, self.game.Data.MapObj[index3].MapHeight);
                  self.game.EditObj.TempValue2[index3] = new MapMatrix2(self.game.Data.MapObj[index3].MapWidth, self.game.Data.MapObj[index3].MapHeight);
                }
                if (Strings.Len(self.game.Data.LoadPass) > 0)
                {
                  self.game.FormRef.Cursor = Cursors.Default;
                  if (Operators.CompareString(Strings.LCase(Interaction.InputBox("This File is protected by a load password. Please give it in order to load it.", "Shadow Empire : Planetary Conquest")), Strings.LCase(self.game.Data.LoadPass), false) == 0)
                  {
                    let mut num5: i32 =  Interaction.MsgBox( "You are cleared.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    let mut num6: i32 =  Interaction.MsgBox( "Wrong Password. You cannot Load this file. Exiting whole app.", Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.EndApp();
                  }
                }
                BitmapStore.ReloadSystemGraphics(self.game.Data.SystemGfx);
                self.game.Data.LoadGraphics((Form1) null);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.MiniMap, 200, 150, false);
                self.game.EditObj.StratMap = new Bitmap(self.game.ScreenWidth, self.game.ScreenHeight - 265);
                self.game.EditObj.StratMap.SetResolution( DrawMod.DPIx,  DrawMod.DPIy);
                self.game.CustomBitmapObj.MakeMiniMap(self.game.EditObj.StratMap, self.game.ScreenWidth, self.game.ScreenHeight - 265, false, true, false);
                self.game.FormRef.Cursor = Cursors.Default;
                let mut num7: i32 =  Interaction.MsgBox( "Loaded Historical Library", Title: ( "Shadow Empire : Planetary Conquest"));
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
            }
            else if (num1 == self.saveId)
            {
              tinitdir: String = self.game.AppPath + "scenarios\\";
              if (!Information.IsNothing( self.game.Data.ScenarioDir))
              {
                if (self.game.Data.ScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.Data.ScenarioDir);
                else if (self.game.ModScenarioDir.Length > 1)
                  tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              }
              else if (self.game.ModScenarioDir.Length > 1)
                tinitdir = tinitdir.Replace("scenarios", self.game.ModScenarioDir);
              str: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Officer library(*.se1off)|*.se1off", "Give save name...", tinitdir, false);
              if (Strings.Len(str) < 2)
              {
                let mut num8: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
              {
                self.game.FormRef.Cursor = Cursors.WaitCursor;
                self.game.Data.serialize(str);
                self.game.HandyFunctionsObj.ZipFile(str);
                windowReturnClass1.SetFlag(true);
                self.game.FormRef.Cursor = Cursors.Default;
                self.game.Data.LoadGraphics(self.formref);
                let mut num9: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
              }
            }
            else
            {
              if (num1 == self.a1id)
              {
                str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].name);
                if (str.Length > 0)
                  self.game.Data.LibraryObj[0].name = str;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (num1 == self.a2id)
              {
                str: String = Interaction.InputBox("Give author name.", "Shadow Empire : Planetary Conquest", self.game.Data.LibraryObj[0].creator);
                if (str.Length > 0)
                  self.game.Data.LibraryObj[0].creator = str;
                self.RemoveSubPart(self.tableId);
                self.tableId = 0;
                self.DoStuff();
                windowReturnClass1.SetFlag(true);
                return windowReturnClass1;
              }
              if (num1 == self.a3id)
              {
                Form2::new( self.formref).Initialize(self.game.Data, 13, 0);
                self.Change = true;
                return windowReturnClass1;
              }
              if (num1 == self.exitId)
              {
                self.game.EditObj.InEditor = false;
                if (self.game.EditorBlock)
                  self.game.EditObj.ShowInitialMenu = true;
                if (self.game.ModIntroType == 0)
                  windowReturnClass1.AddCommand(3, 1);
                else
                  windowReturnClass1.AddCommand(3, 12);
              }
              else if (num1 == self.editId)
              {
                if (self.tabId == 1)
                {
                  if (self.detaily <= 1)
                  {
                    let mut num10: i32 =  Interaction.MsgBox( "Cannot change", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    if (self.detaily == 2)
                    {
                      self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                      str: String = Interaction.InputBox("Give name.", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                      if (str.Length == 0)
                      {
                        let mut num11: i32 =  Interaction.MsgBox( "An officer MUST ALWAYS have a name. always.", Title: ( "Shadow Empire : Planetary Conquest"));
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      self.stringy.Data[self.detailx, self.detaily] = str;
                      self.game.Data.HistoricalUnitObj[self.currentHisNr].CommanderName = str;
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      return windowReturnClass1;
                    }
                    if (self.detaily == 3)
                    {
                      str: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of symbol Sprite:", self.game.AppPath + "graphics\\", true);
                      if (File.Exists(self.game.AppPath + "graphics/" + str))
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].CommanderFileName = str;
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].LoadSprites();
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      let mut num12: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                    }
                    else
                    {
                      if (self.detaily == 4)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        Form2::new( self.formref).Initialize(self.game.Data, 6, self.currentHisNr);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 5)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give value 0-100%", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].CombatMod = Conversions.ToInteger(str);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 6)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give value 0-100%", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].MoraleMod = Conversions.ToInteger(str);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 7)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].StaffSize = Conversions.ToInteger(str);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily == 8)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        str: String = Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = str;
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].PP = Conversions.ToInteger(str);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily >= 9 & self.detaily <= 16)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        let mut tnr2: i32 = self.detaily - 9;
                        self.Change = true;
                        Form3::new( self.formref).Initialize(self.game.Data, 139, self.currentHisNr, tnr2, self.game);
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                      if (self.detaily >= 17)
                      {
                        self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                        InputStr: String = Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest", self.stringy.Data[self.detailx, self.detaily]);
                        self.stringy.Data[self.detailx, self.detaily] = InputStr;
                        let mut smallGfx: i32 = self.game.Data.TempString[1300 + self.ColIsVar[self.detaily]].ToString().Length <= 0 ? -1 :  Math.Round(Conversion.Val(self.game.Data.TempString[1300 + self.ColIsVar[self.detaily]]));
                        self.game.Data.HistoricalUnitObj[self.currentHisNr].SetHisVarValue(self.ColIsVar[self.detaily],  Math.Round(Conversion.Val(InputStr)), smallGfx);
                        self.RemoveSubPart(self.tableId);
                        self.tableId = 0;
                        self.DoStuff();
                        windowReturnClass1.SetFlag(true);
                        return windowReturnClass1;
                      }
                    }
                  }
                }
              }
              else
              {
                if (num1 == self.removeId)
                {
                  if (self.tabId == 1)
                  {
                    self.currentHisNr =  Math.Round(Conversion.Val(self.stringy.Data[self.detailx, 0]));
                    --self.detailx;
                    self.game.Data.RemoveHistoricalUnit(self.currentHisNr);
                  }
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.addId)
                {
                  if (self.tabId == 1)
                  {
                    let mut num13: i32 = 0;
                    let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
                    for (let mut index4: i32 = 0; index4 <= historicalUnitCounter; index4 += 1)
                    {
                      if (self.game.Data.HistoricalUnitObj[index4].ID > num13)
                        num13 = self.game.Data.HistoricalUnitObj[index4].ID;
                    }
                    if (num13 + 100 < self.game.Data.HistoricalIDCounter)
                      self.game.Data.HistoricalIDCounter = num13 + 100;
                    self.game.Data.AddHistoricalUnit();
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].CommanderName = "New Commander";
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].TempRegime = 0;
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].People = 0;
                    self.game.Data.HistoricalUnitObj[self.game.Data.HistoricalUnitCounter].LibId.libSlot = 0;
                  }
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.removeReinf)
                {
                  self.game.Data.RemoveLibrary(self.detailnr);
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.detailnr = -1;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.addReinf)
                {
                  path: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Officer Card Library(*.se1offcard)|*.se1offcard", "Pick Officer card library...", self.game.AppPath + self.game.ModScenarioDir, false);
                  if (File.Exists(path))
                  {
                    self.game.EditObj.TempFileName = path;
                    self.game.EditObj.TempFileType = NewEnums.LibFileType.ImportCardsInOfficerEditor;
                    self.game.FormRef.Cursor = Cursors.WaitCursor;
                    dataClass: DataClass;
                    self.game.HandyFunctionsObj.LoadLibrary( dataClass);
                    bool[] import = new bool[dataClass.LibraryCounter + 1];
                    int[] subreg = new int[dataClass.RegimeCounter + 1];
                    int[] subPpl = new int[dataClass.PeopleCounter + 1];
                    int[] sublib = new int[dataClass.LibraryCounter + 1];
                    let mut libraryCounter1: i32 = dataClass.LibraryCounter;
                    for (let mut index5: i32 = 0; index5 <= libraryCounter1; index5 += 1)
                    {
                      sublib[index5] = -1;
                      import[index5] = false;
                    }
                    let mut regimeCounter: i32 = dataClass.RegimeCounter;
                    for (let mut index6: i32 = 0; index6 <= regimeCounter; index6 += 1)
                      subreg[index6] = 0;
                    let mut peopleCounter: i32 = dataClass.PeopleCounter;
                    for (let mut index7: i32 = 0; index7 <= peopleCounter; index7 += 1)
                      subPpl[index7] = -1;
                    let mut libraryCounter2: i32 = dataClass.LibraryCounter;
                    for (let mut index8: i32 = 0; index8 <= libraryCounter2; index8 += 1)
                    {
                      sublib[index8] = -1;
                      let mut libraryCounter3: i32 = self.game.Data.LibraryCounter;
                      for (let mut index9: i32 = 0; index9 <= libraryCounter3; index9 += 1)
                      {
                        if (Operators.CompareString(self.game.Data.LibraryObj[index9].name, dataClass.LibraryObj[index8].name, false) == 0)
                        {
                          bool flag;
                          sublib[-(flag ? 1 : 0)] = index9;
                        }
                      }
                      let mut actionCardCounter: i32 = dataClass.ActionCardCounter;
                      for (let mut index10: i32 = 0; index10 <= actionCardCounter; index10 += 1)
                      {
                        if (dataClass.ActionCardObj[index10].LibId.libSlot == index8)
                          import[index8] = true;
                      }
                    }
                    self.game.HandyFunctionsObj.ActuallyImportLibs( dataClass,  import,  sublib,  subPpl,  subreg);
                    let mut num14: i32 =  Interaction.MsgBox( "Import completed", Title: ( "Shadow Empire : Planetary Conquest"));
                    self.game.FormRef.Cursor = Cursors.Default;
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
                  }
                  let mut num15: i32 =  Interaction.MsgBox( "Could not find file", Title: ( "Shadow Empire : Planetary Conquest"));
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.versionid)
                {
                  InputStr: String = Interaction.InputBox("Give version.", "Shadow Empire : Planetary Conquest");
                  if (InputStr.Length > 0)
                    self.game.Data.LibraryObj[0].version =  Math.Round(Conversion.Val(InputStr));
                  self.RemoveSubPart(self.tableId);
                  self.tableId = 0;
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.listId)
                {
                  let mut num16: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (num16 > -1)
                  {
                    self.detailnr = num16;
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                  }
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.tableId)
                {
                  Coordinate coordinate = self.SubPartList[index1].Click2(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                  self.SubPartFlag[index1] = true;
                  if (coordinate.x > -1)
                  {
                    self.detailx = coordinate.x;
                    self.detaily = coordinate.y;
                    if (self.detaily > self.stringy.Width)
                      self.detaily = self.stringy.Width;
                    if (self.detailx > self.stringy.Length)
                      self.detailx = self.stringy.Length;
                  }
                  self.DoStuff();
                  windowReturnClass1.SetFlag(true);
                  return windowReturnClass1;
                }
                if (num1 == self.importCsv)
                {
                  str3: String = self.game.HandyFunctionsObj.LoadSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Select file to import...", self.game.AppPath + "csv/", false);
                  if (Strings.Len(str3) < 2)
                  {
                    let mut num17: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    StreamReader Expression;
                    windowReturnClass2: WindowReturnClass;
                    try
                    {
                      Expression = File.OpenText(str3);
                      let mut num18: i32 = 0;
                      str4: String = ",";
                      while (!Expression.EndOfStream)
                      {
                        str5: String = Expression.ReadLine();
                        if ((uint) Strings.InStr(str5, "sep=") > 0U)
                        {
                          str4 = Strings.Mid(str5, 5, 1);
                          num18 = 1;
                        }
                        else
                        {
                          switch (num18)
                          {
                            case 0:
                              if (Strings.InStr(str5, "\t") > 0)
                                str4 = "\t";
                              else if (Strings.InStr(str5, ",") > 0)
                                str4 = ",";
                              else if (Strings.InStr(str5, ";") > 0)
                                str4 = ";";
                              num18 = 2;
                              continue;
                            case 1:
                              num18 = 2;
                              continue;
                            case 2:
                              strArray: Vec<String> = str5.Split(Conversions.ToChar(str4));
                              let mut id: i32 =  Math.Round(Conversion.Val(strArray[0]));
                              index11: i32;
                              if (id > 0)
                              {
                                index11 = self.game.HandyFunctionsObj.GetHistoricalUnitByID(id);
                                if (index11 == -1)
                                {
                                  self.game.Data.AddHistoricalUnit();
                                  index11 = self.game.Data.HistoricalUnitCounter;
                                  self.game.Data.HistoricalUnitObj[index11].ID = Conversions.ToInteger(strArray[0]);
                                }
                                else if (self.game.Data.HistoricalUnitObj[index11].CommanderName.Length < 1)
                                {
                                  self.game.Data.AddHistoricalUnit();
                                  index11 = self.game.Data.HistoricalUnitCounter;
                                  strArray[0] = Conversions.ToString(self.game.Data.HistoricalUnitObj[index11].ID);
                                }
                              }
                              else
                              {
                                self.game.Data.AddHistoricalUnit();
                                index11 = self.game.Data.HistoricalUnitCounter;
                                strArray[0] = Conversions.ToString(self.game.Data.HistoricalUnitObj[index11].ID);
                              }
                              self.game.Data.HistoricalUnitObj[index11].ID =  Math.Round(Conversion.Val(strArray[0]));
                              self.game.Data.HistoricalUnitObj[index11].CommanderName = strArray[1];
                              self.game.Data.HistoricalUnitObj[index11].CommanderFileName = strArray[2];
                              self.game.Data.HistoricalUnitObj[index11].CombatMod =  Math.Round(Conversion.Val(strArray[3]));
                              self.game.Data.HistoricalUnitObj[index11].MoraleMod =  Math.Round(Conversion.Val(strArray[4]));
                              self.game.Data.HistoricalUnitObj[index11].StaffSize =  Math.Round(Conversion.Val(strArray[5]));
                              self.game.Data.HistoricalUnitObj[index11].PP =  Math.Round(Conversion.Val(strArray[6]));
                              let mut index12: i32 = 0;
                              do
                              {
                                if (Conversion.Val(strArray[index12 + 7]) > -1.0)
                                {
                                  if (index12 <= self.game.Data.HistoricalUnitObj[index11].DeckCardCounter)
                                  {
                                    self.game.Data.HistoricalUnitObj[index11].DeckCard[index12] =  Math.Round(Conversion.Val(strArray[index12 + 7]));
                                  }
                                  else
                                  {
                                    self.game.Data.HistoricalUnitObj[index11].DeckCard = (int[]) Utils.CopyArray((Array) self.game.Data.HistoricalUnitObj[index11].DeckCard, (Array) new int[index12 + 1]);
                                    self.game.Data.HistoricalUnitObj[index11].DeckChance = (int[]) Utils.CopyArray((Array) self.game.Data.HistoricalUnitObj[index11].DeckChance, (Array) new int[index12 + 1]);
                                    self.game.Data.HistoricalUnitObj[index11].DeckChance[index12] = 100;
                                    self.game.Data.HistoricalUnitObj[index11].DeckCard[index12] =  Math.Round(Conversion.Val(strArray[index12 + 7]));
                                    self.game.Data.HistoricalUnitObj[index11].DeckCardCounter = index12;
                                    let mut num19: i32 = index12;
                                    for (let mut index13: i32 = 0; index13 <= num19; index13 += 1)
                                    {
                                      if (self.game.Data.HistoricalUnitObj[index11].DeckCard[index13] == -1)
                                      {
                                        self.game.Data.HistoricalUnitObj[index11].DeckCard[index13] =  Math.Round(Conversion.Val(strArray[index12 + 7]));
                                        self.game.Data.HistoricalUnitObj[index11].DeckChance[index13] = 100;
                                      }
                                    }
                                  }
                                }
                                index12 += 1;
                              }
                              while (index12 <= 7);
                              let mut num20: i32 = 0;
                              let mut typ: i32 = 0;
                              do
                              {
                                if (self.game.Data.TempString[1200 + typ].Length > 0)
                                {
                                  num20 += 1;
                                  self.game.Data.HistoricalUnitObj[index11].SetHisVarValue(typ,  Math.Round(Conversion.Val(strArray[14 + num20])));
                                }
                                typ += 1;
                              }
                              while (typ <= 99);
                              continue;
                            default:
                              continue;
                          }
                        }
                      }
                    }
                    catch (Exception ex)
                    {
                      ProjectData.SetProjectError(ex);
                      Exception exception = ex;
                      if (!Information.IsNothing( Expression))
                        Expression.Close();
                      self.game.Data.LoadGraphics(self.formref);
                      let mut num21: i32 =  Interaction.MsgBox( ("Import problem encountered: '" + exception.Message + "'. Your current data might be corrupted. "), Title: ( "Shadow Empire : Planetary Conquest"));
                      self.RemoveSubPart(self.tableId);
                      self.tableId = 0;
                      self.DoStuff();
                      windowReturnClass1.SetFlag(true);
                      windowReturnClass2 = windowReturnClass1;
                      ProjectData.ClearProjectError();
                      goto label_224;
                    }
                    Expression.Close();
                    self.game.Data.LoadGraphics(self.formref);
                    let mut num22: i32 =  Interaction.MsgBox( "Import finished", Title: ( "Shadow Empire : Planetary Conquest"));
                    self.RemoveSubPart(self.tableId);
                    self.tableId = 0;
                    self.DoStuff();
                    windowReturnClass1.SetFlag(true);
                    return windowReturnClass1;
label_224:
                    return windowReturnClass2;
                  }
                }
                else if (num1 == self.exportCsv)
                {
                  str6: String = self.game.HandyFunctionsObj.SaveSomething("CSV (*.csv)|*.csv|TXT (*.txt)|*.txt|All (*.*)|*.*", "Give save name...", self.game.AppPath + "csv/", false);
                  if (Strings.Len(str6) < 2)
                  {
                    let mut num23: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    try
                    {
                      StreamWriter text = File.CreateText(str6);
                      try
                      {
                        text.WriteLine("sep=\t");
                        str7: String = "" + "ID#" + "\t" + "Name" + "\t" + "Portrait" + "\t" + "CombatMod" + "\t" + "MoraleMod" + "\t" + "Staff Pts" + "\t" + "PP" + "\t" + "Card1" + "\t" + "Card2" + "\t" + "Card3" + "\t" + "Card4" + "\t" + "Card5" + "\t" + "Card6" + "\t" + "Card7" + "\t" + "Card8";
                        let mut num24: i32 = 0;
                        do
                        {
                          if (self.game.Data.TempString[1200 + num24].Length > 0)
                            str7 = str7 + "\t" + self.game.Data.TempString[1200 + num24];
                          num24 += 1;
                        }
                        while (num24 <= 99);
                        text.WriteLine(str7);
                        let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
                        for (let mut index14: i32 = 0; index14 <= historicalUnitCounter; index14 += 1)
                        {
                          if (self.game.Data.HistoricalUnitObj[index14].CommanderName.Length > 0)
                          {
                            str8: String = "" + self.game.Data.HistoricalUnitObj[index14].ID.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index14].CommanderName + "\t" + self.game.Data.HistoricalUnitObj[index14].CommanderFileName + "\t" + self.game.Data.HistoricalUnitObj[index14].CombatMod.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index14].MoraleMod.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index14].StaffSize.ToString() + "\t" + self.game.Data.HistoricalUnitObj[index14].PP.ToString();
                            let mut index15: i32 = 0;
                            do
                            {
                              str8 = index15 > self.game.Data.HistoricalUnitObj[index14].DeckCardCounter ? str8 + "\t" + "-1" : str8 + "\t" + self.game.Data.HistoricalUnitObj[index14].DeckCard[index15].ToString();
                              index15 += 1;
                            }
                            while (index15 <= 7);
                            let mut typ: i32 = 0;
                            do
                            {
                              if (self.game.Data.TempString[1200 + typ].Length > 0)
                                str8 = str8 + "\t" + self.game.Data.HistoricalUnitObj[index14].GiveHisVarValue(typ).ToString();
                              typ += 1;
                            }
                            while (typ <= 99);
                            text.WriteLine(str8);
                          }
                        }
                        text.Close();
                        let mut num25: i32 =  Interaction.MsgBox( "Export has been written to the csv/ directory", Title: ( "Shadow Empire : Planetary Conquest"));
                      }
                      catch (Exception ex)
                      {
                        ProjectData.SetProjectError(ex);
                        Exception exception = ex;
                        text.Close();
                        let mut num26: i32 =  Interaction.MsgBox( ("Problem writing: " + exception.Message), Title: ( "Shadow Empire : Planetary Conquest"));
                        ProjectData.ClearProjectError();
                      }
                    }
                    catch (Exception ex)
                    {
                      ProjectData.SetProjectError(ex);
                      let mut num27: i32 =  Interaction.MsgBox( "Problem writing. Check if the file is not opened in other application please.", Title: ( "Shadow Empire : Planetary Conquest"));
                      ProjectData.ClearProjectError();
                    }
                  }
                }
              }
            }
          }
        }
        windowReturnClass1.SetFlag(false);
        return windowReturnClass1;
      }
      windowReturnClass1.SetFlag(false);
      return windowReturnClass1;
    }

    pub fn Interpret()
    {
    }
  }
}
