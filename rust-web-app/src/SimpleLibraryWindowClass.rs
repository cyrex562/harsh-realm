// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleLibraryWindowClass
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
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleLibraryWindowClass : WindowClass
  {
     LibListId: i32;
     ListClass LibListObj;
     CatListId: i32;
     ListClass CatListObj;
     IndListId: i32;
     ListClass IndListObj;
     AddLibEventId: i32;
     AddLibTroopsId: i32;
     AddLibTextId: i32;
     RemoveLibEventId: i32;
     RemoveLibEventIdb: i32;
     loadVarsId: i32;
     LibVarListId: i32;
     ListClass LibVarListObj;
     AddLibVarId: i32;
     AddLibVarTextId: i32;
     RemoveLibVarId: i32;
     RemoveLibVarTextId: i32;
     LibVarTypeId: i32;
     LibVarTypeTextId: i32;
     importId: i32;
     LibVarNameId: i32;
     LibVarNameTextId: i32;
     LibVarInfoId: i32;
     LibVarInfoTextId: i32;
     LibVarValueTypeId: i32;
     LibVarValueTypeTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     B4Id: i32;
     B4TextId: i32;
     text1id: i32;
     text2id: i32;
     ChangeValId: i32;
     ExecuteId: i32;
     TaId: i32;
     loadEventPic: i32;
     loadSmallGfx: i32;
     removeSmallGfx: i32;
     removeEventPic: i32;
     saveId: i32;
     save2id: i32;
     save3id: i32;
     save4id: i32;
     reloadEventPic: i32;
     reloadSmallGfx: i32;
     LibId: i32;
     LibVarId: i32;
     IndId: i32;
     CatId: i32;
     ss: String;

    pub SimpleLibraryWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 50, 9, tDoBorders: 1, tHeaderString: "Libraries")
    {
      self.LibId = -1;
      self.LibVarId = -1;
      self.CatId = -1;
      self.IndId = -1;
      self.DoStuff();
    }

    pub fn DoRefresh() => self.DoStuff();

    pub fn PopUpRefresh() => self.DoStuff();

     void InfoLibVar( Graphics g, usex1: i32, usey1: i32)
    {
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.LibVarObj[self.LibVarId].name + "(inst.id=" + self.game.Data.LibVarObj[self.LibVarId].instanceId.id.ToString() + ")", self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Value:", self.game.MarcFont4, usex1 + 300, usey1 + 30, Color.White);
      let mut libVarUseId: i32 = self.game.Data.GetLibVarUseId(self.LibVarId, self.IndId);
      tstring1: String = self.game.Data.LibVarObj[libVarUseId].valueType.ToString();
      tstring2: String = self.game.Data.LibVarObj[libVarUseId].GetValue( self.game.Data);
      if (self.game.Data.LibVarObj[libVarUseId].instanceId.id > -1)
        tstring2 = self.game.Data.LibVarObj[libVarUseId].GetValue( self.game.Data) + " (NumericValue=" + self.game.Data.LibVarObj[libVarUseId].value.ToString() + ")";
      if (libVarUseId == self.LibVarId & self.game.Data.LibVarObj[libVarUseId].instanceId.id == -1 & !(self.game.Data.LibVarObj[libVarUseId].type == NewEnums.LibVarType.General | self.game.Data.LibVarObj[libVarUseId].type == NewEnums.LibVarType.Hex))
        tstring2 = "-not set-";
      if (tstring2.Length > 30)
        DrawMod.DrawTextColouredMarc( g, tstring2, self.game.MarcFont5, usex1 + 300, usey1 + 50, Color.White);
      else if (tstring2.Length > 15)
        DrawMod.DrawTextColouredMarc( g, tstring2, self.game.MarcFont4, usex1 + 300, usey1 + 50, Color.White);
      else
        DrawMod.DrawTextColouredMarc( g, tstring2, self.game.MarcFont3, usex1 + 300, usey1 + 50, Color.White);
      DrawMod.DrawTextColouredMarc( g, "ValueType:", self.game.MarcFont4, usex1 + 20, usey1 + 75, Color.White);
      DrawMod.DrawTextColouredMarc( g, tstring1, self.game.MarcFont3, usex1 + 20, usey1 + 95, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Variable information:", self.game.MarcFont4, usex1 + 20, usey1 + 125, Color.White);
      let mut tsubpart1: SubPartClass =  new TextAreaClass2(self.game, 580, 4, self.game.MarcFont4, self.game.Data.LibVarObj[self.LibVarId].information, 27,  self.OwnBitmap, usex1 + 20, usey1 + 120);
      self.text2id = self.AddSubPart( tsubpart1, usex1 + 20, usey1 + 120, 580, 100, 0);
      self.ss = "Click to change this value";
      if (self.game.Data.LibVarObj[libVarUseId].type == NewEnums.LibVarType.Hex)
        return;
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Change this value", 200, self.ss,  self.OwnBitmap, usex1 + 300, usey1 + 75, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.ChangeValId = self.AddSubPart( tsubpart2, usex1 + 300, usey1 + 75, 200, 35, 1);
    }

     void InfoEvent( Graphics g, usex1: i32, usey1: i32)
    {
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Event name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.EventObj[self.LibVarId].Name, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      if (self.game.Data.EventObj[self.LibVarId].AllowExecute)
      {
        self.ss = "Click to execute this event. Please read the library information, if available, to better understand what will happen when you do.";
        let mut tsubpart: SubPartClass =  new TextButtonPartClass("Execute this event", 200, self.ss,  self.OwnBitmap, usex1 + 300, usey1 + 30, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.ExecuteId = self.AddSubPart( tsubpart, usex1 + 300, usey1 + 30, 200, 35, 1);
      }
      DrawMod.DrawTextColouredMarc( g, "Event Library slot ID:", self.game.MarcFont4, usex1 + 20, usey1 + 80, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.EventObj[self.LibVarId].LibId.id.ToString(), self.game.MarcFont3, usex1 + 20, usey1 + 100, Color.White);
    }

     void InfoStringlist( Graphics g, usex1: i32, usey1: i32)
    {
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Table name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.StringListObj[self.LibVarId].Name, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Table information:", self.game.MarcFont4, usex1 + 20, usey1 + 85, Color.White);
      let mut tsubpart: SubPartClass =  new TextAreaClass2(self.game, 580, 4, self.game.MarcFont4, self.game.Data.StringListObj[self.LibVarId].Description, 27,  self.OwnBitmap, usex1 + 20, usey1 + 80);
      self.text1id = self.AddSubPart( tsubpart, usex1 + 20, usey1 + 80, 580, 120, 0);
    }

     void InfoLibrary( Graphics g, usex1: i32, usey1: i32)
    {
      usex1 = 10;
      DrawMod.DrawBlock( g, usex1, usey1, self.game.ScreenWidth - 20, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.LibraryObj[self.LibId].name, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Version:", self.game.MarcFont4, usex1 + 20, usey1 + 140, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.LibraryObj[self.LibId].version.ToString(), self.game.MarcFont3, usex1 + 20, usey1 + 160, Color.White);
      DrawMod.DrawTextColouredMarc( g, "By:", self.game.MarcFont4, usex1 + 20, usey1 + 85, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.LibraryObj[self.LibId].creator, self.game.MarcFont3, usex1 + 20, usey1 + 105, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Library information:", self.game.MarcFont4, usex1 + 420, usey1 + 15, Color.White);
      let mut tsubpart: SubPartClass =  new TextAreaClass2(self.game, self.game.ScreenWidth - 460, 13, self.game.MarcFont4, self.game.Data.LibraryObj[self.LibId].information, 17,  self.OwnBitmap, usex1 + 420, usey1 + 30);
      self.text1id = self.AddSubPart( tsubpart, usex1 + 420, usey1 + 30, self.game.ScreenWidth - 440, 240, 0);
    }

     void InfoHisUnit( Graphics g, usex1: i32, usey1: i32)
    {
    }

     void InfoOfficer( Graphics g, usex1: i32, usey1: i32)
    {
    }

     void InfoRegime( Graphics g, usex1: i32, usey1: i32)
    {
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.RegimeObj[self.IndId].Name, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      let mut hqSpriteNr: i32 = self.game.Data.RegimeObj[self.IndId].HQSpriteNr;
      DrawMod.DrawTextColouredMarc( g, "HQ Graphic:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
      DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, 76, 76,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
      if (hqSpriteNr > -1)
      {
        if (Strings.InStr(BitmapStore.GetFileName(hqSpriteNr), "systemgraphics/trans.bmp") > 0)
        {
          DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 39, usey1 + 110 + 40, Color.Black);
        }
        else
        {
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(hqSpriteNr, 1);
           let mut local2: &Bitmap = &bitmap;
          Rectangle srcrect = Rectangle::new(0, 0, BitmapStore.GetWidth(hqSpriteNr, 1), BitmapStore.Getheight(hqSpriteNr, 1));
          Rectangle destrect = Rectangle::new(usex1 + 20, usey1 + 110, 76, 76);
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(hqSpriteNr, 1).ToString() + "x" + BitmapStore.Getheight(hqSpriteNr, 1).ToString(), self.game.MarcFont4, usex1 + 20 + 38, usey1 + 110 + 52, Color.Salmon);
        }
      }
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, 76, 76, -1, -1);
      DrawMod.DrawTextColouredMarc( g, "People:", self.game.MarcFont4, usex1 + 20, usey1 + 200, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.PeopleObj[self.game.Data.RegimeObj[self.IndId].People].Name, self.game.MarcFont3, usex1 + 20, usey1 + 220, Color.White);
    }

     void InfoPeople( Graphics g, usex1: i32, usey1: i32)
    {
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.PeopleObj[self.IndId].Name, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      let mut sidewaysSpriteId: i32 = self.game.Data.PeopleObj[self.IndId].SidewaysSpriteID;
      DrawMod.DrawTextColouredMarc( g, "Sideways Graphic:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
      DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, 140, 80,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
      if (sidewaysSpriteId > -1)
      {
        if (Strings.InStr(BitmapStore.GetFileName(sidewaysSpriteId), "systemgraphics/trans.bmp") > 0)
        {
          DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 40, Color.Black);
        }
        else
        {
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
           let mut local2: &Bitmap = &bitmap;
          Rectangle srcrect = Rectangle::new(0, 0, BitmapStore.GetWidth(sidewaysSpriteId), BitmapStore.Getheight(sidewaysSpriteId));
          Rectangle destrect = Rectangle::new(usex1 + 20, usey1 + 110, 140, 80);
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(sidewaysSpriteId).ToString() + "x" + BitmapStore.Getheight(sidewaysSpriteId).ToString(), self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 55, Color.Salmon);
        }
      }
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, 140, 80, -1, -1);
    }

     void InfoEventPic( Graphics g, usex1: i32, usey1: i32)
    {
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      if (self.IndId > -1)
      {
        str: String = "" + ", id=" + self.game.Data.eventPicLibId[self.IndId].id.ToString() + ", slot=" + self.IndId.ToString();
        DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
        DrawMod.DrawTextColouredMarc( g, self.game.Data.EventPicName[self.IndId] + str, self.game.MarcFont4, usex1 + 20, usey1 + 50, Color.White);
        let mut nr: i32 = self.game.Data.EventPicNr[self.IndId];
        DrawMod.DrawTextColouredMarc( g, "Graphic:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
        if (nr > -1 & !Information.IsNothing( BitmapStore.GetBitmap(nr)))
        {
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(nr);
           let mut local2: &Bitmap = &bitmap;
          Rectangle srcrect = Rectangle::new(0, 0, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr));
          Rectangle destrect = Rectangle::new(usex1 + 20, usey1 + 110, 362, 175);
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(nr).ToString() + "x" + BitmapStore.Getheight(nr).ToString(), self.game.MarcFont4, usex1 + 20 + 180, usey1 + 110 + 90, Color.Salmon);
        }
        DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, 362, 175, -1, -1);
      }
      else
        DrawMod.DrawTextColouredMarc( g, "No specific event picture selected", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      if (self.LibId != -1)
        return;
      if (self.IndId > -1)
      {
        self.ss = "Click to reload image used for this Event Pic.";
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Change Event Pic", 170, self.ss,  self.OwnBitmap, usex1 + 490, usey1 + 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.reloadEventPic = self.AddSubPart( tsubpart1, usex1 + 490, usey1 + 40, 170, 35, 1);
        self.ss = "Click to remove this Event Pic.";
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Remove Event Pic", 170, self.ss,  self.OwnBitmap, usex1 + 490, usey1 + 90, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeEventPic = self.AddSubPart( tsubpart2, usex1 + 490, usey1 + 90, 170, 35, 1);
      }
      self.ss = "Click to reload image used for this Event Pic.";
      let mut tsubpart: SubPartClass =  new TextButtonPartClass("Add Event Pic", 170, self.ss,  self.OwnBitmap, usex1 + 490, 80, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadEventPic = self.AddSubPart( tsubpart, usex1 + 490, 80, 170, 35, 1);
    }

     void InfoCommander( Graphics g, usex1: i32, usey1: i32)
    {
      SizeF sizeF1 = SizeF::new();
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      let mut num1: i32 = usex1 + 25 - 110;
      let mut num2: i32 = usey1 + 25 - 5;
      let mut indId: i32 = self.IndId;
      DrawMod.DrawTextColouredMarc( g, "Deckcards:", self.game.MarcFont4, num1 + 110, usey1 + 160, Color.White);
      DrawMod.DrawTextColouredMarc( g, (self.game.Data.HistoricalUnitObj[indId].DeckCardCounter + 1).ToString(), self.game.MarcFont4, num1 + 110, usey1 + 185, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Statistics:", self.game.MarcFont4, num1 + 110, usey1 + 20, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Portrait:", self.game.MarcFont4, num1 + 460, usey1 + 20, Color.White);
      if (self.game.Data.HistoricalUnitObj[indId].CommanderSpriteID > -1)
      {
        let mut commanderSpriteId: i32 = self.game.Data.HistoricalUnitObj[indId].CommanderSpriteID;
         let mut local1: &Graphics = &g;
        bitmap: Bitmap = BitmapStore.GetBitmap(commanderSpriteId);
         let mut local2: &Bitmap = &bitmap;
        let mut x: i32 = num1 + 460;
        let mut y: i32 = num2 + 35;
        DrawMod.DrawSimple( local1,  local2, x, y);
        DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, num1 + 460, num2 + 35, 177, 194, -1, -1);
        DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(commanderSpriteId).ToString() + "x" + BitmapStore.Getheight(commanderSpriteId).ToString(), self.game.MarcFont4, num1 + 460 + 85, num2 + 182, Color.Salmon);
      }
      else
      {
        DrawMod.DrawBlock( g, num1 + 460, num2 + 35, 177, 194,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
        DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, num1 + 460 + 85, num2 + 35 + 92, Color.Black);
        DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, num1 + 460, num2 + 35, 177, 194, -1, -1);
      }
      TextAreaClass2 textAreaClass2 = new TextAreaClass2(self.game, 340, 4, self.game.MarcFont13, "", 12,  self.OwnBitmap, num1 + 110, num2 + 22, true);
       let mut local3: &Graphics = &g;
      bitmap1: Bitmap = textAreaClass2.Paint();
       let mut local4: &Bitmap = &bitmap1;
      let mut x1: i32 = num1 + 110;
      let mut y1: i32 = num2 + 22;
      DrawMod.DrawSimple( local3,  local4, x1, y1);
      Rectangle trect1 = Rectangle::new(num1 + 105, num2 + 34, 280, 45);
      self.AddMouse( trect1, "OFFICER INFO", "Click to get full stats and biography", 50);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.HistoricalUnitObj[indId].CommanderName, self.game.MarcFont6, num1 + 125, num2 + 44, Color.White);
      let mut num3: i32 = 110;
      DrawMod.DrawBlock( g, num1 + num3, num2 + 67, 247, 2,  self.game.MarcCol3.R,  self.game.MarcCol3.G,  self.game.MarcCol3.B,  byte.MaxValue);
      for (; num3 < 365; num3 += 35)
      {
        DrawMod.DrawBlockGradient2( g, num1 + num3, num2 + 68, 2, 41, self.game.MarcCol3, self.game.MarcCol2);
        index: i32;
        if (self.game.Data.HistoricalUnitObj[indId].HisVarCount >= index)
        {
          str: String = Strings.Trim(Conversion.Str( self.game.Data.HistoricalUnitObj[indId].HisVarValue[index]));
          SizeF sizeF2 = g.MeasureString(str, self.game.MarcFont8b);
          let mut x2: i32 =  Math.Round( ( (num1 + num3 + 18) - sizeF2.Width / 2f));
          DrawMod.DrawTextColouredMarc( g, str, self.game.MarcFont8b, x2, num2 + 90, Color.White);
          bitmap2: Bitmap;
          if (self.game.Data.HistoricalUnitObj[indId].HisVarSmall[index] > -1)
          {
             let mut local5: &Graphics = &g;
            bitmap2 = BitmapStore.GetBitmap(self.game.Data.SmallPicNr[self.game.Data.HistoricalUnitObj[indId].HisVarSmall[index]]);
             let mut local6: &Bitmap = &bitmap2;
            let mut x3: i32 = x2;
            let mut y2: i32 = num2 + 71;
            DrawMod.DrawSimple( local5,  local6, x3, y2);
          }
          else if (self.game.Data.HistoricalUnitObj[indId].HisVarNato[index] > -1)
          {
             let mut local7: &Graphics = &g;
            bitmap2 = BitmapStore.GetBitmap(self.game.NATO[self.game.Data.HistoricalUnitObj[indId].HisVarNato[index]]);
             let mut local8: &Bitmap = &bitmap2;
            let mut x4: i32 = x2;
            let mut y3: i32 = num2 + 71;
            DrawMod.DrawSimple( local7,  local8, x4, y3);
          }
          trect1 = Rectangle::new(x2, num2 + 71, 35, 50);
          let mut trect2: &Rectangle = &trect1
          self.AddMouse( trect2, "", self.game.Data.TempString[1200 + self.game.Data.HistoricalUnitObj[indId].HisVarType[index]]);
        }
        index += 1;
      }
    }

     void InfoSmallGraphic( Graphics g, usex1: i32, usey1: i32)
    {
      let mut num: i32 = usex1;
      if (self.IndId > -1)
      {
        DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
        DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
        DrawMod.DrawTextColouredMarc( g, self.game.Data.SmallPicName[self.IndId], self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
        let mut nr: i32 = self.game.Data.SmallPicNr[self.IndId];
        if (nr > -1)
        {
          DrawMod.DrawTextColouredMarc( g, "Big version:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
          DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr, 1), BitmapStore.Getheight(nr, 1),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
          bitmap: Bitmap;
          Rectangle rectangle1;
          Rectangle rectangle2;
          if (Strings.InStr(BitmapStore.GetFileName(nr), "systemgraphics/trans.bmp") > 0)
          {
            DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 40, Color.Black);
          }
          else
          {
             let mut local1: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(nr, 1);
             let mut local2: &Bitmap = &bitmap;
            rectangle1 = Rectangle::new(0, 0, BitmapStore.GetWidth(nr, 1), BitmapStore.Getheight(nr, 1));
            let mut srcrect: &Rectangle = &rectangle1
            rectangle2 = Rectangle::new(usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr, 1), BitmapStore.Getheight(nr, 1));
            let mut destrect: &Rectangle = &rectangle2
            DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
            DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(nr, 1).ToString() + "x" + BitmapStore.Getheight(nr, 1).ToString(), self.game.MarcFont4,  Math.Round( (usex1 + 20) +  BitmapStore.GetWidth(nr, 1) / 2.0),  Math.Round( (usey1 + 110) +  BitmapStore.Getheight(nr, 1) / 2.0), Color.Salmon);
          }
          DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr, 1), BitmapStore.Getheight(nr, 1), -1, -1);
          usex1 += 140;
          DrawMod.DrawTextColouredMarc( g, "Reg version:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
          DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
          if (Strings.InStr(BitmapStore.GetFileName(nr), "systemgraphics/trans.bmp") > 0)
          {
            DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 40, Color.Black);
          }
          else
          {
             let mut local3: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(nr);
             let mut local4: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr));
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
            DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(nr).ToString() + "x" + BitmapStore.Getheight(nr).ToString(), self.game.MarcFont4,  Math.Round( (usex1 + 20) +  BitmapStore.GetWidth(nr) / 2.0),  Math.Round( (usey1 + 110) +  BitmapStore.Getheight(nr) / 2.0), Color.Salmon);
          }
          DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr), BitmapStore.Getheight(nr), -1, -1);
          usex1 += 100;
          DrawMod.DrawTextColouredMarc( g, "Small version:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
          DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr, -1), BitmapStore.Getheight(nr, -1),  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
          if (Strings.InStr(BitmapStore.GetFileName(nr), "systemgraphics/trans.bmp") > 0)
          {
            DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 40, Color.Black);
          }
          else
          {
             let mut local5: &Graphics = &g;
            bitmap = BitmapStore.GetBitmap(nr, -1);
             let mut local6: &Bitmap = &bitmap;
            rectangle2 = Rectangle::new(0, 0, BitmapStore.GetWidth(nr, -1), BitmapStore.Getheight(nr, -1));
            let mut srcrect: &Rectangle = &rectangle2
            rectangle1 = Rectangle::new(usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr, -1), BitmapStore.Getheight(nr, -1));
            let mut destrect: &Rectangle = &rectangle1
            DrawMod.DrawSimplePart2( local5,  local6, srcrect, destrect);
            DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(nr, -1).ToString() + "x" + BitmapStore.Getheight(nr, -1).ToString(), self.game.MarcFont4,  Math.Round( (usex1 + 20) +  BitmapStore.GetWidth(nr, -1) / 2.0), usey1 + 110 + BitmapStore.Getheight(nr, -1) + 4, Color.Salmon);
          }
          DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, BitmapStore.GetWidth(nr, -1), BitmapStore.Getheight(nr, -1), -1, -1);
        }
        else
          DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 37, usey1 + 110 + 40, Color.Black);
      }
      else
      {
        DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
        DrawMod.DrawTextColouredMarc( g, "No specific small graphic selected", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      }
      usex1 = num;
      if (self.LibId != -1)
        return;
      if (self.IndId > -1)
      {
        self.ss = "Click to reload image used for this Small Graphic.";
        let mut tsubpart1: SubPartClass =  new TextButtonPartClass("Change SmallGfx", 170, self.ss,  self.OwnBitmap, usex1 + 490, usey1 + 40, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.reloadSmallGfx = self.AddSubPart( tsubpart1, usex1 + 490, usey1 + 40, 170, 35, 1);
        self.ss = "Click to remove this Small Graphic.";
        let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Remove SmallGfx", 170, self.ss,  self.OwnBitmap, usex1 + 490, usey1 + 90, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.removeSmallGfx = self.AddSubPart( tsubpart2, usex1 + 490, usey1 + 90, 170, 35, 1);
      }
      self.ss = "Click to reload image used for this Small Graphic.";
      let mut tsubpart: SubPartClass =  new TextButtonPartClass("Add SmallGfx", 170, self.ss,  self.OwnBitmap, usex1 + 490, 80, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.loadSmallGfx = self.AddSubPart( tsubpart, usex1 + 490, 80, 170, 35, 1);
    }

     void InfoHisModel( Graphics g, usex1: i32, usey1: i32)
    {
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.HistoricalUnitObj[self.IndId].Name, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      let mut nr: i32 = -1;
      if (self.game.Data.HistoricalUnitObj[self.IndId].SmallGfx > -1 & self.game.Data.HistoricalUnitObj[self.IndId].SmallGfx <= self.game.Data.SmallPicCounter)
        nr = self.game.Data.SmallPicNr[self.game.Data.HistoricalUnitObj[self.IndId].SmallGfx];
      else if (self.game.Data.HistoricalUnitObj[self.IndId].Counter > 0)
        nr = self.game.NATO[self.game.Data.HistoricalUnitObj[self.IndId].Counter];
      DrawMod.DrawTextColouredMarc( g, "His Unit Graphic:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
      DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, 76, 76,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
      if (nr > -1)
      {
        if (Strings.InStr(BitmapStore.GetFileName(nr), "systemgraphics/trans.bmp") > 0)
        {
          DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 40, Color.Black);
        }
        else
        {
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(nr, 1);
           let mut local2: &Bitmap = &bitmap;
          Rectangle srcrect = Rectangle::new(0, 0, BitmapStore.GetWidth(nr, 1), BitmapStore.Getheight(nr, 1));
          Rectangle destrect = Rectangle::new(usex1 + 20, usey1 + 110, 76, 76);
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(nr, 1).ToString() + "x" + BitmapStore.Getheight(nr, 1).ToString(), self.game.MarcFont4, usex1 + 20 + 38, usey1 + 110 + 52, Color.Salmon);
        }
      }
      else
        DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 37, usey1 + 110 + 40, Color.Black);
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, 76, 76, -1, -1);
    }

     void InfoSFType( Graphics g, usex1: i32, usey1: i32)
    {
      let mut num: i32 = usex1;
      DrawMod.DrawBlock( g, usex1, usey1, 680, 304, 0, 0, 0, 80);
      DrawMod.DrawTextColouredMarc( g, "Name:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.SFTypeObj[self.IndId].Name, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
      DrawMod.DrawTextColouredMarc( g, "Lib ID:", self.game.MarcFont4, usex1 + 320, usey1 + 30, Color.White);
      DrawMod.DrawTextColouredMarc( g, self.game.Data.SFTypeObj[self.IndId].LibId.id.ToString(), self.game.MarcFont3, usex1 + 320, usey1 + 50, Color.White);
      let mut sidewaysSpriteId: i32 = self.game.Data.SFTypeObj[self.IndId].SidewaysSpriteID;
      DrawMod.DrawTextColouredMarc( g, "Sideways graphic:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
      DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, 140, 100,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
      Rectangle rectangle1;
      Rectangle rectangle2;
      if (sidewaysSpriteId > -1)
      {
        if (Strings.InStr(BitmapStore.GetFileName(sidewaysSpriteId), "systemgraphics/trans.bmp") > 0)
        {
          DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 40, Color.Black);
        }
        else
        {
           let mut local1: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(sidewaysSpriteId);
           let mut local2: &Bitmap = &bitmap;
          rectangle1 = Rectangle::new(0, 0, BitmapStore.GetWidth(sidewaysSpriteId), BitmapStore.Getheight(sidewaysSpriteId));
          let mut srcrect: &Rectangle = &rectangle1
          rectangle2 = Rectangle::new(usex1 + 20, usey1 + 110, 140, 100);
          let mut destrect: &Rectangle = &rectangle2
          DrawMod.DrawSimplePart2( local1,  local2, srcrect, destrect);
          DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(sidewaysSpriteId).ToString() + "x" + BitmapStore.Getheight(sidewaysSpriteId).ToString(), self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 5, Color.Salmon);
        }
      }
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, 140, 100, -1, -1);
      let mut picSpriteId: i32 = self.game.Data.SFTypeObj[self.IndId].PicSpriteID;
      usex1 += 180;
      DrawMod.DrawTextColouredMarc( g, "Illustration graphic:", self.game.MarcFont4, usex1 + 20, usey1 + 90, Color.White);
      DrawMod.DrawBlock( g, usex1 + 20, usey1 + 110, 140, 100,  byte.MaxValue,  byte.MaxValue,  byte.MaxValue, 210);
      if (picSpriteId > -1)
      {
        if (Strings.InStr(BitmapStore.GetFileName(picSpriteId), "systemgraphics/trans.bmp") > 0)
        {
          DrawMod.DrawTextColouredMarcCenter( g, "No Graphic", self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 40, Color.Black);
        }
        else
        {
           let mut local3: &Graphics = &g;
          bitmap: Bitmap = BitmapStore.GetBitmap(picSpriteId);
           let mut local4: &Bitmap = &bitmap;
          rectangle2 = Rectangle::new(0, 0, BitmapStore.GetWidth(picSpriteId), BitmapStore.Getheight(picSpriteId));
          let mut srcrect: &Rectangle = &rectangle2
          rectangle1 = Rectangle::new(usex1 + 20, usey1 + 110, 140, 100);
          let mut destrect: &Rectangle = &rectangle1
          DrawMod.DrawSimplePart2( local3,  local4, srcrect, destrect);
          DrawMod.DrawTextColouredMarcCenter( g, BitmapStore.GetWidth(picSpriteId).ToString() + "x" + BitmapStore.Getheight(picSpriteId).ToString(), self.game.MarcFont4, usex1 + 20 + 70, usey1 + 110 + 5, Color.Salmon);
        }
      }
      DrawMod.DrawFrame( self.OwnBitmap,  self.BackBitmap,  g, usex1 + 20, usey1 + 110, 140, 100, -1, -1);
      usey1 += 190;
      usex1 = num;
      DrawMod.DrawTextColouredMarc( g, "ReinfType#1:", self.game.MarcFont4, usex1 + 20, usey1 + 30, Color.White);
      tstring: String = "none";
      let mut reinforcementType: i32 = self.game.Data.SFTypeObj[self.IndId].ReinforcementType;
      if (reinforcementType > -1)
      {
        tstring = "'" + self.game.Data.ReinfName[reinforcementType] + "'";
        if (self.game.Data.ReinfLibId[reinforcementType].libSlot > -1)
          tstring = tstring + " (libslot: " + self.game.Data.ReinfLibId[reinforcementType].libSlot.ToString() + ", libid: " + self.game.Data.ReinfLibId[reinforcementType].id.ToString() + ")";
      }
      DrawMod.DrawTextColouredMarc( g, tstring, self.game.MarcFont3, usex1 + 20, usey1 + 50, Color.White);
    }

     void DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      let mut num2: i32 = self.game.ScreenHeight - 768 - 30;
      if (num2 < 0)
        num2 = 0;
      if (self.text1id > 0)
        self.RemoveSubPart(self.text1id);
      if (self.text2id > 0)
        self.RemoveSubPart(self.text2id);
      if (self.ChangeValId > 0)
        self.RemoveSubPart(self.ChangeValId);
      if (self.ExecuteId > 0)
        self.RemoveSubPart(self.ExecuteId);
      if (self.loadSmallGfx > 0)
        self.RemoveSubPart(self.loadSmallGfx);
      if (self.loadEventPic > 0)
        self.RemoveSubPart(self.loadEventPic);
      if (self.reloadSmallGfx > 0)
        self.RemoveSubPart(self.reloadSmallGfx);
      if (self.reloadEventPic > 0)
        self.RemoveSubPart(self.reloadEventPic);
      if (self.removeSmallGfx > 0)
        self.RemoveSubPart(self.removeSmallGfx);
      if (self.removeEventPic > 0)
        self.RemoveSubPart(self.removeEventPic);
      if (self.loadVarsId > 0)
        self.RemoveSubPart(self.loadVarsId);
      if (self.saveId > 0)
      {
        self.RemoveSubPart(self.saveId);
        self.saveId = 0;
      }
      if (self.save2id > 0)
      {
        self.RemoveSubPart(self.save2id);
        self.save2id = 0;
      }
      if (self.save3id > 0)
      {
        self.RemoveSubPart(self.save3id);
        self.save3id = 0;
      }
      if (self.save4id > 0)
      {
        self.RemoveSubPart(self.save4id);
        self.save4id = 0;
      }
      if (self.TaId > 0)
        self.RemoveSubPart(self.TaId);
      if (self.importId > 0)
        self.RemoveSubPart(self.importId);
      Graphics graphics = Graphics.FromImage((Image) self.OwnBitmap);
      graphics.SmoothingMode = SmoothingMode.AntiAlias;
      graphics.TextRenderingHint = TextRenderingHint.AntiAlias;
      graphics.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 50, -1);
      let mut num3: i32 = 10 + num1 + 240;
      let mut num4: i32 = num3 + 320 + 40;
      let mut num5: i32 = 328 + num2;
      let mut num6: i32 = num5 + num2;
      if (self.LibVarId > -1)
      {
        if (self.CatId < 100)
          self.InfoLibVar( graphics, num3, num5);
        else if (self.CatId == 101)
        {
          if (self.LibVarId > -1)
          {
            self.InfoEvent( graphics, num3, num5);
          }
          else
          {
            DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
            DrawMod.DrawTextColouredMarc( graphics, "No specific event selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
          }
        }
        else if (self.CatId == 109)
        {
          if (self.LibVarId > -1)
          {
            self.InfoEvent( graphics, num3, num5);
          }
          else
          {
            DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
            DrawMod.DrawTextColouredMarc( graphics, "No specific event selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
          }
        }
        else if (self.CatId == 108)
        {
          if (self.LibVarId > -1)
          {
            self.InfoStringlist( graphics, num3, num5);
          }
          else
          {
            DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
            DrawMod.DrawTextColouredMarc( graphics, "No specific table selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
          }
        }
      }
      else if (self.CatId == 102)
      {
        if (self.IndId > -1)
        {
          self.InfoSFType( graphics, num3, num5);
        }
        else
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific trooptype selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId > 0 & self.CatId <= 20)
      {
        if (self.LibVarId <= -1)
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific libvar selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 101)
      {
        if (self.LibVarId <= -1)
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific event selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 112)
      {
        if (self.LibVarId <= -1)
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "Detailed info still to be provided by VR coding.", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 109)
      {
        if (self.LibVarId <= -1)
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific event selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 104)
      {
        if (self.IndId > -1)
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "Detailed info still to be provided by VR coding.", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
        else
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific historical unit selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 108)
      {
        if (self.LibVarId <= -1)
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific table selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 110)
      {
        if (self.IndId >= -1)
          self.InfoEventPic( graphics, num3, num5);
      }
      else if (self.CatId == 111)
      {
        if (self.IndId >= -1)
          self.InfoSmallGraphic( graphics, num3, num5);
      }
      else if (self.CatId == 103)
      {
        if (self.IndId > -1)
        {
          self.InfoHisModel( graphics, num3, num5);
        }
        else
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific historical model selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 105)
      {
        if (self.IndId > -1)
        {
          self.InfoCommander( graphics, num3, num5);
        }
        else
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific commander selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 106)
      {
        if (self.IndId > -1)
        {
          self.InfoPeople( graphics, num3, num5);
        }
        else
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific people selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.CatId == 107)
      {
        if (self.IndId > -1)
        {
          self.InfoRegime( graphics, num3, num5);
        }
        else
        {
          DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 80);
          DrawMod.DrawTextColouredMarc( graphics, "No specific regime selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
        }
      }
      else if (self.LibId > -1)
      {
        self.InfoLibrary( graphics, num3, num5);
      }
      else
      {
        DrawMod.DrawBlock( graphics, num3, num5, 680, 304, 0, 0, 0, 50);
        DrawMod.DrawTextColouredMarc( graphics, "No library selected", self.game.MarcFont4, num3 + 20, num5 + 30, Color.White);
      }
      if (self.LibListId > 0)
        self.RemoveSubPart(self.LibListId);
      if (self.AddLibEventId > 0)
        self.RemoveSubPart(self.AddLibEventId);
      if (self.RemoveLibEventId > 0)
        self.RemoveSubPart(self.RemoveLibEventId);
      if (self.RemoveLibEventIdb > 0)
        self.RemoveSubPart(self.RemoveLibEventIdb);
      if (self.AddLibTroopsId > 0)
        self.RemoveSubPart(self.AddLibTroopsId);
      self.LibListObj = ListClass::new();
      let mut num7: i32 = 0;
      let mut tlistselect1: i32 = -1;
      self.LibListObj.add(Conversion.Str( -1) + ") ** No library **", -2);
      if (self.LibId == -1)
        tlistselect1 = 0;
      let mut libraryCounter: i32 = self.game.Data.LibraryCounter;
      for (let mut index: i32 = 0; index <= libraryCounter; index += 1)
      {
        num7 += 1;
        self.LibListObj.add(Conversion.Str( index) + ") " + self.game.Data.LibraryObj[index].name, index);
        if (self.LibId == index)
          tlistselect1 = num7;
      }
      if (tlistselect1 == -1)
        self.LibId = -1;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(self.LibListObj, 16,  Math.Round(200.0 +  num1 / 2.0), tlistselect1, self.game, true, "Libraries", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: ( Math.Round(10.0 +  num1 / 2.0)), bby: 72, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
      self.LibListId = self.AddSubPart( tsubpart1,  Math.Round(10.0 +  num1 / 2.0), 72,  Math.Round(200.0 +  num1 / 2.0), 408, 0);
      self.ss = "Click to load a library compatible with the current ruleset";
      let mut tsubpart2: SubPartClass =  new TextButtonPartClass("Load library", 200, self.ss,  self.OwnBitmap, num1 + 10, 486, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.AddLibEventId = self.AddSubPart( tsubpart2, num1 + 10, 486, 200, 35, 1);
      if (self.LibId > -1)
      {
        DependencyClass dependencyClass1 = DependencyClass::new();
        DependencyClass dependencyClass2 = self.game.HandyFunctionsObj.Libraries_CheckDependency( self.game.Data, self.LibId, false);
        if (dependencyClass2.ok)
        {
          self.ss = "Click to remove library";
          let mut tsubpart3: SubPartClass =  new TextButtonPartClass("Remove library", 200, self.ss,  self.OwnBitmap, num1 + 10, 526, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.RemoveLibEventId = self.AddSubPart( tsubpart3, num1 + 10, 526, 200, 35, 1);
        }
        else
        {
          self.ss = dependencyClass2.text;
          let mut tsubpart4: SubPartClass =  new TextButtonPartClass("Remove library", 200, self.ss,  self.OwnBitmap, num1 + 10, 526, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.RemoveLibEventIdb = self.AddSubPart( tsubpart4, num1 + 10, 526, 200, 35, 1);
        }
        if (self.game.SuperAdminRights)
        {
          self.ss = "Click to load a the libvar settings from selected library in another scenario. Expert use advise. ";
          let mut tsubpart5: SubPartClass =  new TextButtonPartClass("Load libvars", 200, self.ss,  self.OwnBitmap, num1 + 10, 566, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.loadVarsId = self.AddSubPart( tsubpart5, num1 + 10, 566, 200, 35, 1);
        }
      }
      else
      {
        self.ss = "No library selected";
        let mut tsubpart6: SubPartClass =  new TextButtonPartClass("Remove library", 200, self.ss,  self.OwnBitmap, num1 + 10, 526, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RemoveLibEventIdb = self.AddSubPart( tsubpart6, num1 + 10, 526, 200, 35, 1);
      }
      bool flag1 = false;
      bool flag2 = false;
      bool flag3 = false;
      let mut historicalUnitCounter1: i32 = self.game.Data.HistoricalUnitCounter;
      for (let mut index: i32 = 0; index <= historicalUnitCounter1; index += 1)
      {
        if (self.game.Data.HistoricalUnitObj[index].CommanderName.Length > 1)
        {
          if (self.game.Data.HistoricalUnitObj[index].LibId.libSlot == self.LibId)
            flag1 = true;
        }
        else if (self.game.Data.HistoricalUnitObj[index].LibId.libSlot == self.LibId)
          flag3 = true;
      }
      let mut sfTypeCounter1: i32 = self.game.Data.SFTypeCounter;
      for (let mut index: i32 = 0; index <= sfTypeCounter1; index += 1)
      {
        if (self.game.Data.SFTypeObj[index].LibId.libSlot == self.LibId && self.game.Data.SFTypeObj[index].CopyDataFromBackup > -1)
          flag2 = true;
      }
      let mut num8: i32 = 0;
      SubPartClass tsubpart7;
      if (self.LibId > -1)
      {
        if (flag1)
        {
          self.ss = "Save a library; Note this is still EXPERIMENTAL.";
          tsubpart7 =  new TextButtonPartClass("Save Officer Lib", 150, self.ss,  self.OwnBitmap, num1 - 150, 486 + num8, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.saveId = self.AddSubPart( tsubpart7, num1 - 150, 486 + num8, 150, 35, 1);
          num8 += 40;
        }
        if (flag3)
        {
          self.ss = "Save a library; Note this is still EXPERIMENTAL.";
          tsubpart7 =  new TextButtonPartClass("Save His Lib", 150, self.ss,  self.OwnBitmap, num1 - 150, 486 + num8, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.save2id = self.AddSubPart( tsubpart7, num1 - 150, 486 + num8, 150, 35, 1);
          num8 += 40;
        }
        if (flag2)
        {
          self.ss = "Save a library; Note this is still EXPERIMENTAL.";
          tsubpart7 =  new TextButtonPartClass("Save Troops Lib", 150, self.ss,  self.OwnBitmap, num1 - 150, 486 + num8, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.save3id = self.AddSubPart( tsubpart7, num1 - 150, 486 + num8, 150, 35, 1);
          num8 += 40;
        }
      }
      self.ss = "Save the map; Note this is still EXPERIMENTAL.";
      tsubpart7 =  new TextButtonPartClass("Save Map", 150, self.ss,  self.OwnBitmap, num1 - 150, 486 + num8, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
      self.save4id = self.AddSubPart( tsubpart7, num1 - 150, 486 + num8, 150, 35, 1);
      let mut num9: i32 = num8 + 40;
      if (self.CatListId > 0)
        self.RemoveSubPart(self.CatListId);
      self.CatListObj = ListClass::new();
      let mut tlistselect2: i32 = -1;
      let mut num10: i32 = -1;
      if (self.LibId >= -1)
      {
        let mut num11: i32 = 0;
        do
        {
          let mut num12: i32 = 0;
          let mut libVarCounter: i32 = self.game.Data.LibVarCounter;
          for (let mut index: i32 = 0; index <= libVarCounter; index += 1)
          {
            if (self.game.Data.LibVarObj[index].libId.libSlot == self.LibId & self.game.Data.LibVarObj[index].libId.id == -1 & self.game.Data.LibVarObj[index].type == (NewEnums.LibVarType) num11 && self.game.Data.LibVarObj[index].instanceId.id == -1)
              num12 += 1;
          }
          if (num12 > 0)
          {
            num10 += 1;
            if (num11 == 0)
              self.CatListObj.add("Global LibVars [" + num12.ToString() + "]", 0);
            if (num11 == 1)
              self.CatListObj.add("Landscape LibVars [" + num12.ToString() + "]", 1);
            if (num11 == 2)
              self.CatListObj.add("Road Type LibVars [" + num12.ToString() + "]", 2);
            if (num11 == 3)
              self.CatListObj.add("River Type LibVars [" + num12.ToString() + "]", 3);
            if (num11 == 4)
              self.CatListObj.add("Hex LibVars [" + num12.ToString() + "]", 4);
            if (num11 == 7)
              self.CatListObj.add("Historical Unit LibVars [" + num12.ToString() + "]", 7);
            if (num11 == 8)
              self.CatListObj.add("Historical Model LibVars [" + num12.ToString() + "]", 8);
            if (num11 == 5)
              self.CatListObj.add("TroopType LibVars [" + num12.ToString() + "]", 5);
            if (num11 == 6)
              self.CatListObj.add("Location Type LibVars [" + num12.ToString() + "]", 6);
            if (num11 == 9)
              self.CatListObj.add("Commander LibVars [" + num12.ToString() + "]", 9);
            if (num11 == 10)
              self.CatListObj.add("People LibVars [" + num12.ToString() + "]", 10);
            if (num11 == 11)
              self.CatListObj.add("Regime LibVars [" + num12.ToString() + "]", 11);
            if (self.CatId == num11)
              tlistselect2 = num10;
          }
          num11 += 1;
        }
        while (num11 <= 19);
        let mut num13: i32 = 0;
        let mut eventCounter1: i32 = self.game.Data.EventCounter;
        for (let mut index: i32 = 0; index <= eventCounter1; index += 1)
        {
          if (self.game.Data.EventObj[index].LibId.libSlot == self.LibId & self.game.Data.EventObj[index].AllowExecute)
            num13 += 1;
        }
        if (num13 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Executable events [" + num13.ToString() + "]", 101);
          if (self.CatId == 101)
            tlistselect2 = num10;
        }
        let mut num14: i32 = 0;
        let mut eventCounter2: i32 = self.game.Data.EventCounter;
        for (let mut index: i32 = 0; index <= eventCounter2; index += 1)
        {
          if (self.game.Data.EventObj[index].LibId.libSlot == self.LibId & !self.game.Data.EventObj[index].AllowExecute)
            num14 += 1;
        }
        if (num14 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Non-executable events [" + num14.ToString() + "]", 109);
          if (self.CatId == 109)
            tlistselect2 = num10;
        }
        let mut num15: i32 = 0;
        let mut stringListCounter: i32 = self.game.Data.StringListCounter;
        for (let mut index: i32 = 0; index <= stringListCounter; index += 1)
        {
          if (self.game.Data.StringListObj[index].LibId.libSlot == self.LibId & self.game.Data.StringListObj[index].Editable)
            num15 += 1;
        }
        if (num15 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Tables [" + num15.ToString() + "]", 108);
          if (self.CatId == 108)
            tlistselect2 = num10;
        }
        let mut num16: i32 = 0;
        let mut sfTypeCounter2: i32 = self.game.Data.SFTypeCounter;
        for (let mut index: i32 = 0; index <= sfTypeCounter2; index += 1)
        {
          if (self.game.Data.SFTypeObj[index].LibId.libSlot == self.LibId & !self.game.Data.SFTypeObj[index].DontShowInList)
            num16 += 1;
        }
        if (num16 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Trooptypes [" + num16.ToString() + "]", 102);
          if (self.CatId == 102)
            tlistselect2 = num10;
        }
        let mut num17: i32 = 0;
        let mut historicalUnitCounter2: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut index: i32 = 0; index <= historicalUnitCounter2; index += 1)
        {
          if (Information.IsNothing( self.game.Data.HistoricalUnitObj[index].CommanderName))
            self.game.Data.HistoricalUnitObj[index].CommanderName = "";
          if (self.game.Data.HistoricalUnitObj[index].LibId.libSlot == self.LibId & self.game.Data.HistoricalUnitObj[index].Model & self.game.Data.HistoricalUnitObj[index].CommanderName.Length < 1)
            num17 += 1;
        }
        if (num17 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Historical models [" + num17.ToString() + "]", 103);
          if (self.CatId == 103)
            tlistselect2 = num10;
        }
        let mut num18: i32 = 0;
        let mut historicalUnitCounter3: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut his: i32 = 0; his <= historicalUnitCounter3; his += 1)
        {
          if (self.game.Data.HistoricalUnitObj[his].LibId.libSlot == self.LibId & !self.game.Data.HistoricalUnitObj[his].Model & (self.game.Data.HistoricalUnitObj[his].CommanderName.Length < 1 | self.game.HandyFunctionsObj.GetUnitByHistorical(his) > -1))
            num18 += 1;
        }
        if (num18 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Historical units [" + num18.ToString() + "]", 104);
          if (self.CatId == 104)
            tlistselect2 = num10;
        }
        let mut num19: i32 = 0;
        let mut historicalUnitCounter4: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut index: i32 = 0; index <= historicalUnitCounter4; index += 1)
        {
          if (!self.game.Data.HistoricalUnitObj[index].Model & self.game.Data.HistoricalUnitObj[index].CommanderName.Length > 0 && self.LibId != -1 & self.game.Data.HistoricalUnitObj[index].OffLibId.libSlot == self.LibId)
            num19 += 1;
          if (!self.game.Data.HistoricalUnitObj[index].Model & self.game.Data.HistoricalUnitObj[index].CommanderName.Length > 0 && self.LibId != -1 & self.game.Data.HistoricalUnitObj[index].LibId.libSlot == self.LibId & self.game.Data.HistoricalUnitObj[index].OffLibId.libSlot == -1)
            num19 += 1;
        }
        if (num19 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Officers [" + num19.ToString() + "]", 105);
          if (self.CatId == 105)
            tlistselect2 = num10;
        }
        let mut num20: i32 = 0;
        let mut peopleCounter: i32 = self.game.Data.PeopleCounter;
        for (let mut index: i32 = 0; index <= peopleCounter; index += 1)
        {
          if (self.game.Data.PeopleObj[index].LibId.libSlot == self.LibId)
            num20 += 1;
        }
        if (num20 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Peoples [" + num20.ToString() + "]", 106);
          if (self.CatId == 106)
            tlistselect2 = num10;
        }
        let mut num21: i32 = 0;
        let mut eventPicCounter: i32 = self.game.Data.EventPicCounter;
        for (let mut index: i32 = 0; index <= eventPicCounter; index += 1)
        {
          if (self.game.Data.eventPicLibId[index].libSlot == self.LibId)
            num21 += 1;
        }
        if (num21 >= 0)
        {
          num10 += 1;
          self.CatListObj.add("Event Pictures [" + num21.ToString() + "]", 110);
          if (self.CatId == 110)
            tlistselect2 = num10;
        }
        let mut num22: i32 = 0;
        let mut smallPicCounter: i32 = self.game.Data.SmallPicCounter;
        for (let mut index: i32 = 0; index <= smallPicCounter; index += 1)
        {
          if (self.game.Data.SmallLibId[index].libSlot == self.LibId)
            num22 += 1;
        }
        if (num22 >= 0)
        {
          num10 += 1;
          self.CatListObj.add("Small Graphics [" + num22.ToString() + "]", 111);
          if (self.CatId == 111)
            tlistselect2 = num10;
        }
        let mut num23: i32 = 0;
        let mut actionCardCounter: i32 = self.game.Data.ActionCardCounter;
        for (let mut index: i32 = 0; index <= actionCardCounter; index += 1)
        {
          if (self.game.Data.ActionCardObj[index].LibId.libSlot == self.LibId)
            num23 += 1;
        }
        if (num23 > 0)
        {
          num10 += 1;
          self.CatListObj.add("Action Cards [" + num23.ToString() + "]", 112);
          if (self.CatId == 112)
            tlistselect2 = num10;
        }
        let mut num24: i32 = 0;
        let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
        for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
        {
          if (self.game.Data.RegimeObj[index].libId.libSlot == self.LibId)
            num24 += 1;
        }
        if (num24 > 0)
        {
          let mut num25: i32 = num10 + 1;
          self.CatListObj.add("Regimes [" + num24.ToString() + "]", 107);
          if (self.CatId == 107)
            tlistselect2 = num25;
        }
      }
      tsubpart7 =  new ListSubPartClass(self.CatListObj, 4 +  Math.Round( num2 / 24.0), 200, tlistselect2, self.game, true, "Variable categories", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( self.OwnBitmap), bbx: (10 + num1 + 240), bby: 72, tMarcStyle: true, overruleFont: ( self.game.MarcFont4), overruleItemSize: 24);
      self.CatListId = self.AddSubPart( tsubpart7, 10 + num1 + 240, 72, 200, (5 +  Math.Round( num2 / 24.0)) * 24, 0);
      if (self.IndListId > 0)
        self.RemoveSubPart(self.IndListId);
      self.IndListObj = ListClass::new();
      let mut num26: i32 = -1;
      let mut num27: i32 = -1;
      if (self.CatId > -1)
      {
        if (self.CatId == 0)
        {
          num27 += 1;
          self.IndListObj.add("All", 0);
          if (self.IndId == 0)
            num26 = num27;
        }
        if (self.CatId == 4)
        {
          num27 += 1;
          self.IndListObj.add("All", 0);
          if (self.IndId == 0)
            num26 = num27;
        }
        if (self.CatId == 5)
        {
          let mut sfTypeCounter3: i32 = self.game.Data.SFTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= sfTypeCounter3; tdata += 1)
          {
            self.IndListObj.add(self.game.Data.SFTypeObj[tdata].Name, tdata);
            num27 += 1;
            if (self.IndId == tdata)
              num26 = num27;
          }
        }
        if (self.CatId == 7)
        {
          let mut historicalUnitCounter5: i32 = self.game.Data.HistoricalUnitCounter;
          for (let mut index: i32 = 0; index <= historicalUnitCounter5; index += 1)
          {
            if (!self.game.Data.HistoricalUnitObj[index].Model & (self.game.Data.HistoricalUnitObj[index].CommanderName.Length < 1 | self.game.HandyFunctionsObj.GetUnitByHistorical(index) > -1))
            {
              self.IndListObj.add(self.game.Data.HistoricalUnitObj[index].Name, index);
              num27 += 1;
              if (self.IndId == index)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 8)
        {
          let mut historicalUnitCounter6: i32 = self.game.Data.HistoricalUnitCounter;
          for (let mut tdata: i32 = 0; tdata <= historicalUnitCounter6; tdata += 1)
          {
            if (self.game.Data.HistoricalUnitObj[tdata].Model & self.game.Data.HistoricalUnitObj[tdata].CommanderName.Length < 1)
            {
              self.IndListObj.add(self.game.Data.HistoricalUnitObj[tdata].Name, tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 9)
        {
          let mut historicalUnitCounter7: i32 = self.game.Data.HistoricalUnitCounter;
          for (let mut tdata: i32 = 0; tdata <= historicalUnitCounter7; tdata += 1)
          {
            if (!self.game.Data.HistoricalUnitObj[tdata].Model & self.game.Data.HistoricalUnitObj[tdata].CommanderName.Length > 0)
            {
              self.IndListObj.add(self.game.Data.HistoricalUnitObj[tdata].CommanderName, tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 1)
        {
          let mut landscapeTypeCounter: i32 = self.game.Data.LandscapeTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= landscapeTypeCounter; tdata += 1)
          {
            self.IndListObj.add(self.game.Data.LandscapeTypeObj[tdata].Name, tdata);
            num27 += 1;
            if (self.IndId == tdata)
              num26 = num27;
          }
        }
        if (self.CatId == 6)
        {
          let mut locTypeCounter: i32 = self.game.Data.LocTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= locTypeCounter; tdata += 1)
          {
            self.IndListObj.add(self.game.Data.LocTypeObj[tdata].Name, tdata);
            num27 += 1;
            if (self.IndId == tdata)
              num26 = num27;
          }
        }
        if (self.CatId == 10)
        {
          let mut peopleCounter: i32 = self.game.Data.PeopleCounter;
          for (let mut tdata: i32 = 0; tdata <= peopleCounter; tdata += 1)
          {
            self.IndListObj.add(self.game.Data.PeopleObj[tdata].Name, tdata);
            num27 += 1;
            if (self.IndId == tdata)
              num26 = num27;
          }
        }
        if (self.CatId == 11)
        {
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut tdata: i32 = 0; tdata <= regimeCounter; tdata += 1)
          {
            self.IndListObj.add(self.game.Data.RegimeObj[tdata].Name, tdata);
            num27 += 1;
            if (self.IndId == tdata)
              num26 = num27;
          }
        }
        if (self.CatId == 3)
        {
          let mut riverTypeCounter: i32 = self.game.Data.RiverTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= riverTypeCounter; tdata += 1)
          {
            self.IndListObj.add(self.game.Data.RiverTypeObj[tdata].Name, tdata);
            num27 += 1;
            if (self.IndId == tdata)
              num26 = num27;
          }
        }
        if (self.CatId == 2)
        {
          let mut roadTypeCounter: i32 = self.game.Data.RoadTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= roadTypeCounter; tdata += 1)
          {
            self.IndListObj.add(self.game.Data.RoadTypeObj[tdata].Name, tdata);
            num27 += 1;
            if (self.IndId == tdata)
              num26 = num27;
          }
        }
        if (self.CatId == 101)
        {
          num27 += 1;
          self.IndListObj.add("All events", 0);
          if (self.IndId == 0)
            num26 = num27;
        }
        if (self.CatId == 109)
        {
          num27 += 1;
          self.IndListObj.add("All events", 0);
          if (self.IndId == 0)
            num26 = num27;
        }
        if (self.CatId == 108)
        {
          num27 += 1;
          self.IndListObj.add("All tables", 0);
          if (self.IndId == 0)
            num26 = num27;
        }
        if (self.CatId == 110)
        {
          let mut eventPicCounter: i32 = self.game.Data.EventPicCounter;
          for (let mut tdata: i32 = 0; tdata <= eventPicCounter; tdata += 1)
          {
            if (self.game.Data.eventPicLibId[tdata].libSlot == self.LibId)
            {
              self.IndListObj.add(self.game.Data.EventPicName[tdata] + " (id=" + self.game.Data.eventPicLibId[tdata].id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 111)
        {
          let mut smallPicCounter: i32 = self.game.Data.SmallPicCounter;
          for (let mut tdata: i32 = 0; tdata <= smallPicCounter; tdata += 1)
          {
            if (self.game.Data.SmallLibId[tdata].libSlot == self.LibId)
            {
              self.IndListObj.add(self.game.Data.SmallPicName[tdata] + " (id=" + self.game.Data.SmallLibId[tdata].id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 112)
        {
          let mut actionCardCounter: i32 = self.game.Data.ActionCardCounter;
          for (let mut tdata: i32 = 0; tdata <= actionCardCounter; tdata += 1)
          {
            if (self.game.Data.ActionCardObj[tdata].LibId.libSlot == self.LibId)
            {
              self.IndListObj.add(self.game.Data.ActionCardObj[tdata].Title + " (id=" + self.game.Data.ActionCardObj[tdata].LibId.id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 102)
        {
          let mut sfTypeCounter4: i32 = self.game.Data.SFTypeCounter;
          for (let mut tdata: i32 = 0; tdata <= sfTypeCounter4; tdata += 1)
          {
            if (self.game.Data.SFTypeObj[tdata].LibId.libSlot == self.LibId & !self.game.Data.SFTypeObj[tdata].DontShowInList)
            {
              self.IndListObj.add(self.game.Data.SFTypeObj[tdata].Name + " (id=" + self.game.Data.SFTypeObj[tdata].LibId.id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 103)
        {
          let mut historicalUnitCounter8: i32 = self.game.Data.HistoricalUnitCounter;
          for (let mut tdata: i32 = 0; tdata <= historicalUnitCounter8; tdata += 1)
          {
            if (self.game.Data.HistoricalUnitObj[tdata].LibId.libSlot == self.LibId & self.game.Data.HistoricalUnitObj[tdata].Model & self.game.Data.HistoricalUnitObj[tdata].CommanderName.Length < 1)
            {
              self.IndListObj.add(self.game.Data.HistoricalUnitObj[tdata].Name + " (id=" + self.game.Data.HistoricalUnitObj[tdata].LibId.id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 104)
        {
          let mut historicalUnitCounter9: i32 = self.game.Data.HistoricalUnitCounter;
          for (let mut index: i32 = 0; index <= historicalUnitCounter9; index += 1)
          {
            if (self.game.Data.HistoricalUnitObj[index].LibId.libSlot == self.LibId & !self.game.Data.HistoricalUnitObj[index].Model & (self.game.Data.HistoricalUnitObj[index].CommanderName.Length < 1 | self.game.HandyFunctionsObj.GetUnitByHistorical(index) > -1))
            {
              self.IndListObj.add(self.game.Data.HistoricalUnitObj[index].Name + " (id=" + self.game.Data.HistoricalUnitObj[index].LibId.id.ToString() + ")", index);
              num27 += 1;
              if (self.IndId == index)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 105)
        {
          let mut historicalUnitCounter10: i32 = self.game.Data.HistoricalUnitCounter;
          for (let mut tdata: i32 = 0; tdata <= historicalUnitCounter10; tdata += 1)
          {
            if (!self.game.Data.HistoricalUnitObj[tdata].Model & self.game.Data.HistoricalUnitObj[tdata].CommanderName.Length > 0 && self.LibId != -1 & self.game.Data.HistoricalUnitObj[tdata].LibId.libSlot == self.LibId & self.game.Data.HistoricalUnitObj[tdata].OffLibId.libSlot == -1)
            {
              self.IndListObj.add(self.game.Data.HistoricalUnitObj[tdata].CommanderName + " (id=" + self.game.Data.HistoricalUnitObj[tdata].LibId.id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
            if (!self.game.Data.HistoricalUnitObj[tdata].Model & self.game.Data.HistoricalUnitObj[tdata].CommanderName.Length > 0 && self.LibId != -1 & self.game.Data.HistoricalUnitObj[tdata].OffLibId.libSlot == self.LibId)
            {
              self.IndListObj.add(self.game.Data.HistoricalUnitObj[tdata].CommanderName + " (id=" + self.game.Data.HistoricalUnitObj[tdata].OffLibId.id.ToString() + ") <assigned>", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 106)
        {
          let mut peopleCounter: i32 = self.game.Data.PeopleCounter;
          for (let mut tdata: i32 = 0; tdata <= peopleCounter; tdata += 1)
          {
            if (self.game.Data.PeopleObj[tdata].LibId.libSlot == self.LibId)
            {
              self.IndListObj.add(self.game.Data.PeopleObj[tdata].Name + " (id=" + self.game.Data.PeopleObj[tdata].LibId.id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
        if (self.CatId == 107)
        {
          let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
          for (let mut tdata: i32 = 0; tdata <= regimeCounter; tdata += 1)
          {
            if (self.game.Data.RegimeObj[tdata].libId.libSlot == self.LibId)
            {
              self.IndListObj.add(self.game.Data.RegimeObj[tdata].Name + " (id=" + self.game.Data.RegimeObj[tdata].libId.id.ToString() + ")", tdata);
              num27 += 1;
              if (self.IndId == tdata)
                num26 = num27;
            }
          }
        }
      }
      ListClass indListObj = self.IndListObj;
      let mut tlistsize1: i32 = 7 +  Math.Round( num2 / 16.0);
      let mut tlistselect3: i32 = num26;
      let mut game1: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      let mut bbx1: i32 = 10 + num1 + 480;
      font: Font =  null;
       local2: Font =  font;
      tsubpart7 =  new ListSubPartClass(indListObj, tlistsize1, 200, tlistselect3, game1, true, "Instances", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx1, bby: 72, tMarcStyle: true, overruleFont: ( local2));
      self.IndListId = self.AddSubPart( tsubpart7, 10 + num1 + 480, 72, 220, (9 +  Math.Round( num2 / 16.0)) * 16, 0);
      if (self.LibVarListId > 0)
        self.RemoveSubPart(self.LibVarListId);
      self.LibVarListObj = ListClass::new();
      let mut num28: i32 = -1;
      let mut num29: i32 = -1;
      bool flag4;
      if (self.IndId > -1)
      {
        if (self.CatId <= 19)
        {
          let mut libVarCounter: i32 = self.game.Data.LibVarCounter;
          for (let mut index: i32 = 0; index <= libVarCounter; index += 1)
          {
            if (self.game.Data.LibVarObj[index].libId.libSlot == self.LibId & self.game.Data.LibVarObj[index].type == (NewEnums.LibVarType) self.CatId)
            {
              bool flag5 = false;
              let mut num30: i32 = -1;
              num31: i32;
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.HistoricalUnit)
              {
                num31 = self.game.Data.HistoricalUnitObj[self.IndId].LibId.libSlot;
                num30 = self.game.Data.HistoricalUnitObj[self.IndId].LibId.id;
                if (num30 == -1)
                  num30 = self.IndId;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.HistoricalUnitModel)
              {
                num31 = self.game.Data.HistoricalUnitObj[self.IndId].LibId.libSlot;
                num30 = self.game.Data.HistoricalUnitObj[self.IndId].LibId.id;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.Officer)
              {
                if (self.game.Data.HistoricalUnitObj[self.IndId].OffLibId.id > -1)
                {
                  num31 = self.game.Data.HistoricalUnitObj[self.IndId].OffLibId.libSlot;
                  num30 = self.game.Data.HistoricalUnitObj[self.IndId].OffLibId.id;
                }
                else
                {
                  num31 = self.game.Data.HistoricalUnitObj[self.IndId].LibId.libSlot;
                  num30 = self.game.Data.HistoricalUnitObj[self.IndId].LibId.id;
                }
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.Landscape)
              {
                num31 = -1;
                num30 = self.IndId;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.LocationType)
              {
                num31 = -1;
                num30 = self.IndId;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.People)
              {
                num31 = self.game.Data.PeopleObj[self.IndId].LibId.libSlot;
                num30 = self.game.Data.PeopleObj[self.IndId].LibId.id;
                if (num30 == -1)
                  num30 = self.IndId;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.Regime)
              {
                num31 = self.game.Data.RegimeObj[self.IndId].libId.libSlot;
                num30 = self.game.Data.RegimeObj[self.IndId].libId.id;
                if (num30 == -1)
                  num30 = self.IndId;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.River)
              {
                num31 = -1;
                num30 = self.IndId;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.Road)
              {
                num31 = -1;
                num30 = self.IndId;
              }
              if (self.game.Data.LibVarObj[index].type == NewEnums.LibVarType.SFtype)
              {
                num31 = self.game.Data.SFTypeObj[self.IndId].LibId.libSlot;
                num30 = self.game.Data.SFTypeObj[self.IndId].LibId.id;
              }
              tvalue: String;
              if (num30 > -1 & num31 == self.game.Data.LibVarObj[index].instanceId.libSlot & num30 == self.game.Data.LibVarObj[index].instanceId.id)
              {
                flag4 = true;
                num29 += 1;
                if (self.LibVarId == index)
                  num28 = num29;
                flag5 = true;
                tvalue = self.game.Data.LibVarObj[self.game.Data.GetLibVarUseId(index, self.IndId)].GetValue( self.game.Data);
              }
              else if (self.game.Data.LibVarObj[index].instanceId.id == -1)
              {
                let mut libVarUseId: i32 = self.game.Data.GetLibVarUseId(index, self.IndId);
                if (libVarUseId == index)
                {
                  flag5 = true;
                  if (Operators.CompareString(self.game.Data.LibVarObj[libVarUseId].information, "hidden", false) == 0)
                    flag5 = false;
                  if (flag5)
                  {
                    num29 += 1;
                    if (self.LibVarId == index)
                      num28 = num29;
                    tvalue = !(self.game.Data.LibVarObj[libVarUseId].type == NewEnums.LibVarType.General | self.game.Data.LibVarObj[libVarUseId].type == NewEnums.LibVarType.Hex) ? "-not set-" : self.game.Data.LibVarObj[libVarUseId].GetValue( self.game.Data);
                  }
                }
              }
              else
                index = index;
              if (flag5)
                self.LibVarListObj.add(self.game.Data.LibVarObj[index].name, index, tvalue);
            }
          }
          if (self.LibVarListObj.ListCount > -1)
          {
            ListClass libVarListObj = self.LibVarListObj;
            let mut tlistsize2: i32 = 13 +  Math.Round( num2 / 16.0);
            let mut twidth: i32 =  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0));
            let mut tlistselect4: i32 = num28;
            let mut game2: GameClass = self.game;
            let mut tValueWidth: i32 =  Math.Round(Conversion.Int(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0) * 0.66));
             local3: Bitmap =  self.OwnBitmap;
            let mut bbx2: i32 = 10 + num1 + 720;
            font =  null;
             local4: Font =  font;
            tsubpart7 =  new ListSubPartClass(libVarListObj, tlistsize2, twidth, tlistselect4, game2, true, "Variables", false, tShowPair: true, tValueWidth: tValueWidth, tdotopandbottom: false, tbackbitmap: ( local3), bbx: bbx2, bby: 72, tMarcStyle: true, overruleFont: ( local4));
            self.LibVarListId = self.AddSubPart( tsubpart7, 10 + num1 + 720, 72,  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0)), (15 +  Math.Round( num2 / 16.0)) * 16, 0);
          }
        }
        else if (self.CatId == 5)
        {
          let mut libVarCounter1: i32 = self.game.Data.LibVarCounter;
          for (let mut tdata: i32 = 0; tdata <= libVarCounter1; tdata += 1)
          {
            if (self.game.Data.LibVarObj[tdata].libId.libSlot == self.LibId & self.game.Data.LibVarObj[tdata].type == (NewEnums.LibVarType) self.CatId && -1 == self.game.Data.LibVarObj[tdata].instanceId.id)
            {
              flag4 = true;
              num29 += 1;
              if (self.LibVarId == tdata)
                num28 = num29;
              tvalue: String = "-not set-";
              flag4 = false;
              let mut libVarCounter2: i32 = self.game.Data.LibVarCounter;
              for (let mut index: i32 = 0; index <= libVarCounter2; index += 1)
              {
                if (self.game.Data.LibVarObj[tdata].libId.libSlot == self.game.Data.LibVarObj[index].libId.libSlot && Operators.CompareString(self.game.Data.LibVarObj[tdata].name, self.game.Data.LibVarObj[index].name, false) == 0 && self.game.Data.SFTypeObj[self.IndId].LibId.libSlot == self.game.Data.LibVarObj[index].instanceId.libSlot && self.game.Data.SFTypeObj[self.IndId].LibId.id == self.game.Data.LibVarObj[index].instanceId.id)
                {
                  flag4 = true;
                  if (self.game.Data.LibVarObj[tdata].valueType == NewEnums.LibVarValueType.Number)
                  {
                    tvalue = self.game.Data.LibVarObj[index].value.ToString();
                    break;
                  }
                  break;
                }
              }
              self.LibVarListObj.add(self.game.Data.LibVarObj[tdata].name, tdata, tvalue);
            }
          }
          ListClass libVarListObj = self.LibVarListObj;
          let mut tlistsize3: i32 = 13 +  Math.Round( num2 / 16.0);
          let mut twidth: i32 =  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0));
          let mut tlistselect5: i32 = num28;
          let mut game3: GameClass = self.game;
           local5: Bitmap =  self.OwnBitmap;
          let mut bbx3: i32 = 10 + num1 + 720;
          font =  null;
           local6: Font =  font;
          tsubpart7 =  new ListSubPartClass(libVarListObj, tlistsize3, twidth, tlistselect5, game3, true, "Variables", false, tShowPair: true, tValueWidth: 75, tdotopandbottom: false, tbackbitmap: ( local5), bbx: bbx3, bby: 72, tMarcStyle: true, overruleFont: ( local6));
          self.LibVarListId = self.AddSubPart( tsubpart7, 10 + num1 + 720, 72,  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0)), (15 +  Math.Round( num2 / 16.0)) * 16, 0);
        }
        else if (self.CatId == 101)
        {
          let mut eventCounter: i32 = self.game.Data.EventCounter;
          for (let mut tdata: i32 = 0; tdata <= eventCounter; tdata += 1)
          {
            if (self.game.Data.EventObj[tdata].LibId.libSlot == self.LibId & self.game.Data.EventObj[tdata].AllowExecute)
            {
              num29 += 1;
              if (self.LibVarId == tdata)
                num28 = num29;
              self.LibVarListObj.add(self.game.Data.EventObj[tdata].Name, tdata);
            }
          }
          ListClass libVarListObj = self.LibVarListObj;
          let mut tlistsize4: i32 = 13 +  Math.Round( num2 / 16.0);
          let mut twidth: i32 =  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0));
          let mut tlistselect6: i32 = num28;
          let mut game4: GameClass = self.game;
           local7: Bitmap =  self.OwnBitmap;
          let mut bbx4: i32 = 10 + num1 + 720;
          font =  null;
           local8: Font =  font;
          tsubpart7 =  new ListSubPartClass(libVarListObj, tlistsize4, twidth, tlistselect6, game4, true, "Events", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local7), bbx: bbx4, bby: 72, tMarcStyle: true, overruleFont: ( local8));
          self.LibVarListId = self.AddSubPart( tsubpart7, 10 + num1 + 720, 72,  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0)), (15 +  Math.Round( num2 / 16.0)) * 16, 0);
        }
        else if (self.CatId == 109)
        {
          let mut eventCounter: i32 = self.game.Data.EventCounter;
          for (let mut tdata: i32 = 0; tdata <= eventCounter; tdata += 1)
          {
            if (self.game.Data.EventObj[tdata].LibId.libSlot == self.LibId & !self.game.Data.EventObj[tdata].AllowExecute)
            {
              num29 += 1;
              if (self.LibVarId == tdata)
                num28 = num29;
              self.LibVarListObj.add(self.game.Data.EventObj[tdata].Name, tdata);
            }
          }
          ListClass libVarListObj = self.LibVarListObj;
          let mut tlistsize5: i32 = 13 +  Math.Round( num2 / 16.0);
          let mut twidth: i32 =  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0));
          let mut tlistselect7: i32 = num28;
          let mut game5: GameClass = self.game;
           local9: Bitmap =  self.OwnBitmap;
          let mut bbx5: i32 = 10 + num1 + 720;
          font =  null;
           local10: Font =  font;
          tsubpart7 =  new ListSubPartClass(libVarListObj, tlistsize5, twidth, tlistselect7, game5, true, "Events", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local9), bbx: bbx5, bby: 72, tMarcStyle: true, overruleFont: ( local10));
          self.LibVarListId = self.AddSubPart( tsubpart7, 10 + num1 + 720, 72,  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0)), (15 +  Math.Round( num2 / 16.0)) * 16, 0);
        }
        else if (self.CatId == 108)
        {
          let mut stringListCounter: i32 = self.game.Data.StringListCounter;
          for (let mut tdata: i32 = 0; tdata <= stringListCounter; tdata += 1)
          {
            if (self.game.Data.StringListObj[tdata].LibId.libSlot == self.LibId & self.game.Data.StringListObj[tdata].Editable)
            {
              num29 += 1;
              if (self.LibVarId == tdata)
                num28 = num29;
              self.LibVarListObj.add(self.game.Data.StringListObj[tdata].Name, tdata);
            }
          }
          ListClass libVarListObj = self.LibVarListObj;
          let mut tlistsize6: i32 = 13 +  Math.Round( num2 / 16.0);
          let mut twidth: i32 =  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0));
          let mut tlistselect8: i32 = num28;
          let mut game6: GameClass = self.game;
           local11: Bitmap =  self.OwnBitmap;
          let mut bbx6: i32 = 10 + num1 + 720;
          font =  null;
           local12: Font =  font;
          tsubpart7 =  new ListSubPartClass(libVarListObj, tlistsize6, twidth, tlistselect8, game6, true, "Tables", false, tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local11), bbx: bbx6, bby: 72, tMarcStyle: true, overruleFont: ( local12));
          self.LibVarListId = self.AddSubPart( tsubpart7, 10 + num1 + 720, 72,  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0)), (15 +  Math.Round( num2 / 16.0)) * 16, 0);
        }
        else
        {
          let mut libVarCounter: i32 = self.game.Data.LibVarCounter;
          for (let mut tdata: i32 = 0; tdata <= libVarCounter; tdata += 1)
          {
            if (self.game.Data.LibVarObj[tdata].libId.libSlot == self.LibId & self.game.Data.LibVarObj[tdata].type == (NewEnums.LibVarType) self.CatId)
            {
              bool flag6 = false;
              if (self.CatId == 0 && self.IndId == 0)
                flag6 = true;
              if (self.CatId == 4)
              {
                num29 += 1;
                if (self.LibVarId == tdata)
                  num28 = num29;
                self.LibVarListObj.add(self.game.Data.LibVarObj[tdata].name, tdata, "map pato: i32 set value");
              }
              if (flag6)
              {
                num29 += 1;
                if (self.LibVarId == tdata)
                  num28 = num29;
                tvalue: String = "";
                if (self.game.Data.LibVarObj[tdata].valueType == NewEnums.LibVarValueType.Number)
                  tvalue = self.game.Data.LibVarObj[tdata].value.ToString();
                self.LibVarListObj.add(self.game.Data.LibVarObj[tdata].name, tdata, tvalue);
              }
            }
          }
          ListClass libVarListObj = self.LibVarListObj;
          let mut tlistsize7: i32 = 13 +  Math.Round( num2 / 16.0);
          let mut twidth: i32 =  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0));
          let mut tlistselect9: i32 = num28;
          let mut game7: GameClass = self.game;
           local13: Bitmap =  self.OwnBitmap;
          let mut bbx7: i32 = 10 + num1 + 720;
          font =  null;
           local14: Font =  font;
          tsubpart7 =  new ListSubPartClass(libVarListObj, tlistsize7, twidth, tlistselect9, game7, true, "Variables", false, tShowPair: true, tValueWidth: 75, tdotopandbottom: false, tbackbitmap: ( local13), bbx: bbx7, bby: 72, tMarcStyle: true, overruleFont: ( local14));
          self.LibVarListId = self.AddSubPart( tsubpart7, 10 + num1 + 720, 72,  Math.Round(Math.Max(200.0,  self.game.ScreenWidth / 2.0 - 312.0)), (15 +  Math.Round( num2 / 16.0)) * 16, 0);
        }
      }
      if (!(self.LibVarListObj.ListCount == -1 & self.LibVarListId > 0))
        return;
      self.RemoveSubPart(self.LibVarListId);
      self.LibVarListId = 0;
    }

     void MakeLibItemGUI()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      if (self.LibVarListId > 0)
        self.RemoveSubPart(self.LibVarListId);
      if (self.BNameId > 0)
        self.RemoveSubPart(self.BNameId);
      if (self.BNameTextId > 0)
        self.RemoveSubPart(self.BNameTextId);
      if (self.RemoveLibEventId > 0)
        self.RemoveSubPart(self.RemoveLibEventId);
      if (self.RemoveLibEventIdb > 0)
        self.RemoveSubPart(self.RemoveLibEventIdb);
      if (self.AddLibVarId > 0)
        self.RemoveSubPart(self.AddLibVarId);
      if (self.AddLibVarTextId > 0)
        self.RemoveSubPart(self.AddLibVarTextId);
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
      if (self.LibId > -1)
      {
        self.ss = "Click to change the name of this Library. Make sure its a unique name. So maybe use part of your name or something thats not easily thought of by other designer.";
        let mut tsubpart1: SubPartClass =  TextPartClass::new("name: " + self.game.Data.LibraryObj[self.LibId].name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.BNameTextId = self.AddSubPart( tsubpart1, 510, 49, 400, 20, 0);
        self.ss = "Click to change the name of this Library. Make sure its a unique name. So maybe use part of your name or something thats not easily thought of by other designer.";
        let mut tsubpart2: SubPartClass =  TextPartClass::new("creator: " + self.game.Data.LibraryObj[self.LibId].creator, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.B2TextId = self.AddSubPart( tsubpart2, 510, 79, 400, 20, 0);
        self.ss = "Click to document how to use your library.";
        let mut tsubpart3: SubPartClass =  TextPartClass::new("information: " + Strings.Left(self.game.Data.LibraryObj[self.LibId].information, 20) + "...", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.B3TextId = self.AddSubPart( tsubpart3, 510, 109, 400, 20, 0);
        self.ss = "Click to change the name of this Library. Make sure its a unique name. So maybe use part of your name or something thats not easily thought of by other designer.";
        let mut tsubpart4: SubPartClass =  TextPartClass::new("version: " + self.game.Data.LibraryObj[self.LibId].version.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.B4TextId = self.AddSubPart( tsubpart4, 510, 139, 400, 20, 0);
        if (self.LibVarListId > 0)
          self.RemoveSubPart(self.LibVarListId);
        let mut num2: i32 = -1;
        let mut num3: i32 = -1;
        self.LibVarListObj = ListClass::new();
        let mut libVarCounter: i32 = self.game.Data.LibVarCounter;
        for (let mut index: i32 = 0; index <= libVarCounter; index += 1)
        {
          if (self.game.Data.LibVarObj[index].libId.libSlot == self.LibId)
          {
            num3 += 1;
            self.LibVarListObj.add(Conversion.Str( index) + ") " + self.game.Data.LibVarObj[index].name, index);
            if (self.LibVarId == index)
              num2 = num3;
          }
        }
        if (num2 == -1)
          self.LibVarId = -1;
        ListClass libVarListObj = self.LibVarListObj;
        let mut tlistselect: i32 = num2;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart5: SubPartClass =  new ListSubPartClass(libVarListObj, 14, 400, tlistselect, game, tHeader: "LibVars", tbackbitmap: ( local1), bbx: 470, bby: 200, overruleFont: ( local2));
        self.LibVarListId = self.AddSubPart( tsubpart5, 470, 200, 400, 272, 0);
        if (self.AddLibVarId > 0)
          self.RemoveSubPart(self.AddLibVarId);
        if (self.AddLibVarTextId > 0)
          self.RemoveSubPart(self.AddLibVarTextId);
        self.ss = "Click to add a new LibVar";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart6: SubPartClass =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
          self.AddLibVarId = self.AddSubPart( tsubpart6, 470, 500, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart7: SubPartClass =  TextPartClass::new("Add LibVar", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
          self.AddLibVarTextId = self.AddSubPart( tsubpart7, 520, 499, 300, 20, 0);
        }
      }
      self.MakeLibVarItemGUI();
    }

     void MakeLibVarItemGUI()
    {
      if (self.LibVarTypeId > 0)
        self.RemoveSubPart(self.LibVarTypeId);
      if (self.LibVarValueTypeId > 0)
        self.RemoveSubPart(self.LibVarValueTypeId);
      if (self.LibVarNameId > 0)
        self.RemoveSubPart(self.LibVarNameId);
      if (self.LibVarTypeTextId > 0)
        self.RemoveSubPart(self.LibVarTypeTextId);
      if (self.LibVarValueTypeTextId > 0)
        self.RemoveSubPart(self.LibVarValueTypeTextId);
      if (self.LibVarNameTextId > 0)
        self.RemoveSubPart(self.LibVarNameTextId);
      if (self.LibVarInfoId > 0)
        self.RemoveSubPart(self.LibVarInfoId);
      if (self.LibVarInfoTextId > 0)
        self.RemoveSubPart(self.LibVarInfoTextId);
      if (self.RemoveLibVarId > 0)
        self.RemoveSubPart(self.RemoveLibVarId);
      if (self.RemoveLibVarTextId > 0)
        self.RemoveSubPart(self.RemoveLibVarTextId);
      if (self.LibVarId <= -1)
        return;
      self.ss = "";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.LibVarTypeId = self.AddSubPart( tsubpart, 470, 550, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Type: " + self.game.Data.LibVarObj[self.LibVarId].type.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.LibVarTypeTextId = self.AddSubPart( tsubpart1, 510, 549, 400, 20, 0);
      self.ss = "";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.LibVarNameId = self.AddSubPart( tsubpart1, 470, 580, 32, 16, 1);
      }
      tsubpart1 =  TextPartClass::new("Name: " + self.game.Data.LibVarObj[self.LibVarId].name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.LibVarNameTextId = self.AddSubPart( tsubpart1, 510, 579, 400, 20, 0);
      self.ss = "";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.LibVarValueTypeId = self.AddSubPart( tsubpart1, 470, 610, 32, 16, 1);
      }
      tsubpart1 =  TextPartClass::new("ValueType: " + self.game.Data.LibVarObj[self.LibVarId].valueType.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.LibVarValueTypeTextId = self.AddSubPart( tsubpart1, 510, 609, 400, 20, 0);
      self.ss = "";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.LibVarInfoId = self.AddSubPart( tsubpart1, 470, 640, 32, 16, 1);
      }
      tsubpart1 =  TextPartClass::new("Information: " + Strings.Left(self.game.Data.LibVarObj[self.LibVarId].information, 20) + "...", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.LibVarInfoTextId = self.AddSubPart( tsubpart1, 510, 639, 400, 20, 0);
      self.ss = "Click to remove this libvar.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart1 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
        self.RemoveLibVarId = self.AddSubPart( tsubpart1, 470, 520, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      tsubpart1 =  TextPartClass::new("Remove Libvar", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
      self.RemoveLibVarTextId = self.AddSubPart( tsubpart1, 520, 519, 200, 20, 0);
    }

    pub fn SaveOfficerLib()
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
        let mut num1: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        self.game.FormRef.Cursor = Cursors.WaitCursor;
        self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadOfficers;
        self.game.HandyFunctionsObj.ActuallyExportLib(self.LibId).serialize(str);
        self.game.HandyFunctionsObj.ZipFile(str);
        self.game.FormRef.Cursor = Cursors.Default;
        let mut num2: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
        self.game.EditObj.TempFileType = NewEnums.LibFileType.None;
      }
    }

    pub fn SaveMap()
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
      str: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Map file(*.se1map)|*.se1map", "Give save name...", tinitdir, false);
      if (Strings.Len(str) < 2)
      {
        let mut num1: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        self.game.FormRef.Cursor = Cursors.WaitCursor;
        self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadMap;
        self.game.HandyFunctionsObj.ActuallyExportLib(self.LibId).serialize(str);
        self.game.HandyFunctionsObj.ZipFile(str);
        self.game.FormRef.Cursor = Cursors.Default;
        let mut num2: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
        self.game.EditObj.TempFileType = NewEnums.LibFileType.None;
      }
    }

    pub fn SaveTroopTypeLib()
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
      str: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Troops&Equipment library(*.se1troops)|*.se1troops", "Give save name...", tinitdir, false);
      if (Strings.Len(str) < 2)
      {
        let mut num1: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        self.game.FormRef.Cursor = Cursors.WaitCursor;
        self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadTroops;
        self.game.HandyFunctionsObj.ActuallyExportLib(self.LibId).serialize(str);
        self.game.HandyFunctionsObj.ZipFile(str);
        self.game.FormRef.Cursor = Cursors.Default;
        let mut num2: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
        self.game.EditObj.TempFileType = NewEnums.LibFileType.None;
      }
    }

    pub fn SaveHisLib()
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
      str: String = self.game.HandyFunctionsObj.SaveSomething("SE1 Historical library(*.se1his)|*.se1his", "Give save name...", tinitdir, false);
      if (Strings.Len(str) < 2)
      {
        let mut num1: i32 =  Interaction.MsgBox( "Operation is Cancelled", Title: ( "Shadow Empire : Planetary Conquest"));
      }
      else
      {
        self.game.FormRef.Cursor = Cursors.WaitCursor;
        self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadHistoricals;
        self.game.HandyFunctionsObj.ActuallyExportLib(self.LibId).serialize(str);
        self.game.HandyFunctionsObj.ZipFile(str);
        self.game.FormRef.Cursor = Cursors.Default;
        let mut num2: i32 =  Interaction.MsgBox( "Completed & Saved", Title: ( "Shadow Empire : Planetary Conquest"));
        self.game.EditObj.TempFileType = NewEnums.LibFileType.None;
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
            if (num1 == self.LibListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.LibId = num2;
                self.LibVarId = -1;
                self.CatId = -1;
                self.IndId = -1;
                self.DoStuff();
              }
              else if (num2 == -2)
              {
                self.LibId = -1;
                self.LibVarId = -1;
                self.CatId = -1;
                self.IndId = -1;
                self.DoStuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.saveId)
            {
              self.SaveOfficerLib();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.save2id)
            {
              self.SaveHisLib();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.save3id)
            {
              self.SaveTroopTypeLib();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.save4id)
            {
              self.SaveMap();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.CatListId)
            {
              let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num3 > -1)
              {
                self.CatId = num3;
                self.IndId = -1;
                if (self.CatId == 4 | self.CatId == 0)
                  self.IndId = 0;
                self.LibVarId = -1;
                self.DoStuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            num4: i32;
            if (num1 == self.text2id)
            {
              num4 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.IndListId)
            {
              let mut num5: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num5 > -1)
              {
                self.IndId = num5;
                self.LibVarId = -1;
                self.DoStuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.text1id)
            {
              num4 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.LibVarListId)
            {
              let mut num6: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num6 > -1)
              {
                self.LibVarId = num6;
                self.DoStuff();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeValId)
            {
              let mut tnr: i32 = self.game.Data.GetLibVarUseId(self.LibVarId, self.IndId);
              if (!(self.game.Data.LibVarObj[tnr].type == NewEnums.LibVarType.General & self.game.Data.LibVarObj[tnr].type == NewEnums.LibVarType.General) && tnr == self.LibVarId & self.game.Data.LibVarObj[tnr].instanceId.id == -1)
              {
                self.game.Data.AddLibVar(self.game.Data.LibVarObj[self.LibVarId].libId.libSlot);
                self.game.Data.LibVarObj[self.game.Data.LibVarCounter] = self.game.Data.LibVarObj[self.LibVarId].Clone();
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.SFtype)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.game.Data.SFTypeObj[self.IndId].LibId.id;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = self.game.Data.SFTypeObj[self.IndId].LibId.libSlot;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.Road)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = -1;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.River)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = -1;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.Regime)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.game.Data.RegimeObj[self.IndId].libId.id;
                  if (self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id == -1)
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = self.game.Data.RegimeObj[self.IndId].libId.libSlot;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.People)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.game.Data.PeopleObj[self.IndId].LibId.id;
                  if (self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id == -1)
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = self.game.Data.PeopleObj[self.IndId].LibId.libSlot;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.Officer)
                {
                  if (self.game.Data.HistoricalUnitObj[self.IndId].OffLibId.id > -1)
                  {
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.game.Data.HistoricalUnitObj[self.IndId].OffLibId.id;
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = self.game.Data.HistoricalUnitObj[self.IndId].OffLibId.libSlot;
                  }
                  else
                  {
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.game.Data.HistoricalUnitObj[self.IndId].LibId.id;
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = self.game.Data.HistoricalUnitObj[self.IndId].LibId.libSlot;
                  }
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.Landscape)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = -1;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.LocationType)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = -1;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.HistoricalUnitModel)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.game.Data.HistoricalUnitObj[self.IndId].LibId.id;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = self.game.Data.HistoricalUnitObj[self.IndId].LibId.libSlot;
                }
                if (self.game.Data.LibVarObj[self.LibVarId].type == NewEnums.LibVarType.HistoricalUnit)
                {
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.game.Data.HistoricalUnitObj[self.IndId].LibId.id;
                  if (self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id == -1)
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                  self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot = self.game.Data.HistoricalUnitObj[self.IndId].LibId.libSlot;
                  if (self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.libSlot == -1)
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter].instanceId.id = self.IndId;
                }
                tnr = self.game.Data.LibVarCounter;
              }
              if (tnr == -1)
                tnr = self.LibVarId;
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.Number)
              {
                DefaultResponse: String = "";
                if (self.game.Data.LibVarObj[tnr].value > 0)
                  DefaultResponse = self.game.Data.LibVarObj[tnr].value.ToString();
                let mut num7: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new number value for variable", "Shadow Empire : Planetary Conquest", DefaultResponse)));
                self.game.Data.LibVarObj[tnr].value = num7;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.Text)
              {
                str: String = Interaction.InputBox("Give new text for variable", "Shadow Empire : Planetary Conquest", self.game.Data.LibVarObj[tnr].valueText);
                self.game.Data.LibVarObj[tnr].valueText = str;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.HistoricalUnitId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 118, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.HistoricalUnitModelId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 119, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.LandscapeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 120, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.OfficerId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 121, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.PeopleId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 122, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.ActionCardId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 146, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.RegimeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 123, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.RiverId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 124, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.RoadId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 125, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.LocationTypeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 128, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.SFTypeId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 126, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.HistoricalUnitId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 118, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.SmallGfxId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 142, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.EventPicId)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 143, tnr, tGame: self.game);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.YesNo)
              {
                if (Interaction.MsgBox( "Set value of to Yes or No?", MsgBoxStyle.YesNo,  "Variable value") == MsgBoxResult.Yes)
                  self.game.Data.LibVarObj[tnr].value = 1;
                else
                  self.game.Data.LibVarObj[tnr].value = 0;
              }
              if (self.game.Data.LibVarObj[tnr].valueType == NewEnums.LibVarValueType.DateString)
              {
                str1: String = Interaction.InputBox("Give new Day.", "Shadow Empire : Planetary Conquest");
                if (Conversions.ToInteger(str1) >= 1 & Conversions.ToInteger(str1) <= 31)
                {
                  str2: String = str1;
                  str3: String = Interaction.InputBox("Give new Month.", "Shadow Empire : Planetary Conquest");
                  if (Conversions.ToInteger(str3) >= 1 & Conversions.ToInteger(str3) <= 12)
                  {
                    str4: String = str2 + "/" + str3;
                    str5: String = Interaction.InputBox("Give new Year.", "Shadow Empire : Planetary Conquest");
                    if (Conversions.ToInteger(str5) >= 1 & Conversions.ToInteger(str5) <= 9999)
                    {
                      str6: String = str4 + "/" + str5;
                      self.game.Data.LibVarObj[tnr].valueText = str6;
                    }
                  }
                }
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.removeEventPic)
            {
              self.game.Data.RemoveEventPic(self.IndId);
              if (self.IndId > self.game.Data.EventPicCounter)
                self.IndId = self.game.Data.EventPicCounter;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.loadEventPic)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps|*.bmp|Jpegs|*.jpg", "Select File For New Event Picture:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.AddEventPic(filename);
              }
              else
              {
                let mut num8: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.reloadEventPic)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps|*.bmp|Jpegs|*.jpg", "Select File For New Event Picture:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.EventPicReplaceprite(self.IndId, filename);
              }
              else
              {
                let mut num9: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.removeSmallGfx)
            {
              self.game.Data.RemoveSmallPic(self.IndId);
              if (self.IndId > self.game.Data.EventPicCounter)
                self.IndId = self.game.Data.EventPicCounter;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.loadSmallGfx)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps|*.bmp|Jpegs|*.jpg", "Select File For New Small Gfx:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.AddSmallPic(filename);
              }
              else
              {
                let mut num10: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.reloadSmallGfx)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps|*.bmp|Jpegs|*.jpg", "Select File For New Small Gfx:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.SmallPicReplaceprite(self.IndId, filename);
              }
              else
              {
                let mut num11: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ExecuteId)
            {
              self.game.HandyFunctionsObj.RedimStats();
              let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
              for (let mut regnr: i32 = 0; regnr <= regimeCounter; regnr += 1)
              {
                self.game.HandyFunctionsObj.ClearHistory( regnr);
                self.game.ProcessingObj.SetInitialReconAndZOC(regnr);
              }
              let mut turn: i32 = self.game.Data.Turn;
              self.game.Data.Turn = 0;
              self.game.EventRelatedObj.DoCheckSpecificEvent(self.LibVarId);
              self.game.Data.Turn = turn;
              let mut num12: i32 =  Interaction.MsgBox( "Event has been executed", Title: ( "Shadow Empire : Planetary Conquest"));
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.RemoveLibEventId)
            {
              if (Interaction.MsgBox( "Are you sure? Removing a library will usually cause any stringlist or units or other data related to this  library to be removed from your scenario.", MsgBoxStyle.YesNo,  "Shadow Empire : Planetary Conquest") == MsgBoxResult.Yes)
              {
                self.game.Data.RemoveLibrary(self.LibId);
                self.game.HandyFunctionsObj.Libraries_ClearUpAllRemnants();
                self.game.EditObj.OldUnit = -1;
                self.game.EditObj.UnitSelected = -1;
                self.LibId = -1;
                self.LibVarId = -1;
                self.CatId = -1;
                self.IndId = -1;
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.loadVarsId)
            {
              str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off", "Pick file to load libraries from...", self.game.AppPath + self.game.ModScenarioDir, false);
              try
              {
                if (File.Exists(str))
                {
                  self.game.HandyFunctionsObj.Unzip(str);
                  dataClass: DataClass = DataClass.deserialize(str);
                  self.game.HandyFunctionsObj.ZipFile(str);
                  let mut mapWidth1: i32 = self.game.Data.MapObj[0].MapWidth;
                  for (let mut index2: i32 = 0; index2 <= mapWidth1; index2 += 1)
                  {
                    let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                    for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                    {
                      for (let mut hexLibVarCounter: i32 = self.game.Data.MapObj[0].HexObj[index2, index3].HexLibVarCounter; hexLibVarCounter >= 0; hexLibVarCounter += -1)
                      {
                        if (self.game.Data.LibVarObj[self.game.Data.MapObj[0].HexObj[index2, index3].HexLibVarSlotNr[hexLibVarCounter]].libId.libSlot == self.LibId)
                          self.game.Data.MapObj[0].HexObj[index2, index3].RemoveHexLibVar(hexLibVarCounter);
                      }
                    }
                  }
                  for (let mut libVarCounter: i32 = self.game.Data.LibVarCounter; libVarCounter >= 0; libVarCounter += -1)
                  {
                    if (self.game.Data.LibVarObj[libVarCounter].libId.libSlot == self.LibId)
                      self.game.Data.RemoveLibVar(libVarCounter);
                  }
                  let mut libVarCounter1: i32 = dataClass.LibVarCounter;
                  for (let mut index4: i32 = 0; index4 <= libVarCounter1; index4 += 1)
                  {
                    let mut libSlot: i32 = dataClass.LibVarObj[index4].libId.libSlot;
                    if (libSlot > -1 && Operators.CompareString(dataClass.LibraryObj[libSlot].name, self.game.Data.LibraryObj[self.LibId].name, false) == 0)
                    {
                      self.game.Data.AddLibVar(self.LibId);
                      self.game.Data.LibVarObj[self.game.Data.LibVarCounter] = dataClass.LibVarObj[index4].Clone();
                    }
                  }
                  let mut mapWidth2: i32 = dataClass.MapObj[0].MapWidth;
                  for (let mut index5: i32 = 0; index5 <= mapWidth2; index5 += 1)
                  {
                    let mut mapHeight: i32 = dataClass.MapObj[0].MapHeight;
                    for (let mut index6: i32 = 0; index6 <= mapHeight; index6 += 1)
                    {
                      for (let mut hexLibVarCounter: i32 = dataClass.MapObj[0].HexObj[index5, index6].HexLibVarCounter; hexLibVarCounter >= 0; hexLibVarCounter += -1)
                      {
                        let mut tLibVarSlotNr: i32 = dataClass.MapObj[0].HexObj[index5, index6].HexLibVarSlotNr[hexLibVarCounter];
                        let mut tValue: i32 = dataClass.MapObj[0].HexObj[index5, index6].HexLibVarValue[hexLibVarCounter];
                        if (self.game.Data.LibVarObj[tLibVarSlotNr].libId.libSlot == self.LibId)
                          self.game.Data.MapObj[0].HexObj[index5, index6].AddHexLibVar(tLibVarSlotNr, tValue);
                      }
                    }
                  }
                }
                let mut num13: i32 =  Interaction.MsgBox( "Done.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              catch (Exception ex)
              {
                ProjectData.SetProjectError(ex);
                let mut num14: i32 =  Interaction.MsgBox( "Something went wrong. Sorry. Could not execute.", Title: ( "Shadow Empire : Planetary Conquest"));
                ProjectData.ClearProjectError();
              }
            }
            else if (num1 == self.AddLibEventId)
            {
              str: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Event library(*.se1evlib)|*.se1evlib|SE1 Troops&Equipment library(*.se1troops)|*.se1troops|SE1 Historical library(*.se1his)|*.se1his|SE1 Officer Card Library(*.se1offcard)|*.se1offcard|SE1 Officer library(*.se1off)|*.se1off", "Pick file to load libraries from...", self.game.AppPath + self.game.ModScenarioDir, false);
              if (File.Exists(str))
              {
                self.game.EditObj.TempFileName = str;
                if (Strings.InStr(str, ".se1troops") > 0 & ".se1troops".Length > 0)
                  self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadTroops;
                else if (Strings.InStr(str, ".se1his") > 0 & ".se1his".Length > 0)
                  self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadHistoricals;
                else if (Strings.InStr(str, ".se1evlib") > 0 & ".se1evlib".Length > 0)
                  self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadEvents;
                else if (Strings.InStr(str, ".se1offcard") > 0 & ".se1offcard".Length > 0)
                  self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadOfficerCards;
                else if (Strings.InStr(str, ".se1off") > 0 & ".se1off".Length > 0)
                {
                  self.game.EditObj.TempFileType = NewEnums.LibFileType.LoadOfficers;
                }
                else
                {
                  let mut num15: i32 =  Interaction.MsgBox( "Sorry no go. You can only import Troops, Historical, Events, Officer or Officer Card Libraries in the Simple Editor.", Title: ( "Shadow Empire : Planetary Conquest"));
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                self.LibId = -1;
                self.LibVarId = -1;
                self.CatId = -1;
                self.IndId = -1;
                self.game.EditObj.PopupValue = 17;
                self.game.EditObj.MyDelegate = new EditClass.AfterPopUpRefresh(self.PopUpRefresh);
                windowReturnClass.AddCommand(5, 10);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num16: i32 =  Interaction.MsgBox( "Could not find file", Title: ( "Shadow Empire : Planetary Conquest"));
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

    pub fn HandleToolTip(x: i32, y: i32)
    {
      if (self.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = self.SubPartCounter;
        for (let mut index: i32 = 0; index <= subPartCounter; index += 1)
        {
          if (x > self.SubPartX[index] & x < self.SubPartX[index] + self.SubPartW[index] && y > self.SubPartY[index] & y < self.SubPartY[index] + self.SubPartH[index] && Operators.CompareString(self.SubPartList[index].Descript, "", false) > 0)
          {
            self.game.EditObj.TipButton = true;
            self.game.EditObj.TipTitle = "";
            self.game.EditObj.TipText = self.SubPartList[index].Descript;
            return;
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

    pub fn Import()
    {
      path: String = self.game.HandyFunctionsObj.LoadSomething("SE1 Scenario file (*.se1)|*.se1", "Pick a scenario to import libvars from...", self.game.AppPath + self.game.ModScenarioDir, false);
      if (!File.Exists(path))
        return;
      let mut num1: i32 =  Interaction.MsgBox( "Ok hold on... this can take some time", Title: ( "Shadow Empire : Planetary Conquest"));
      self.game.FormRef.Cursor = Cursors.WaitCursor;
      self.game.EditObj.TempFileName = path;
      tempFileName: String = self.game.EditObj.TempFileName;
      self.game.HandyFunctionsObj.Unzip(tempFileName);
      dataClass1: DataClass = new DataClass(DontLoadGraphics: true);
      dataClass2: DataClass = DataClass.deserialize(tempFileName);
      self.game.HandyFunctionsObj.ZipFile(tempFileName);
      let mut libVarCounter: i32 = dataClass2.LibVarCounter;
      num2: i32;
      num3: i32;
      for (let mut index: i32 = 0; index <= libVarCounter; index += 1)
      {
        let mut libSlot: i32 = dataClass2.LibVarObj[index].libId.libSlot;
        bool flag = false;
        let mut library1: i32 = self.game.Data.FindLibrary(dataClass2.LibraryObj[libSlot].name);
        let mut libVar: i32 = self.game.Data.FindLibVar( dataClass2.LibVarObj[index], dataClass2.LibraryObj[libSlot].name);
        if (libVar > -1 && self.game.Data.LibVarObj[libVar].valueType == dataClass2.LibVarObj[index].valueType)
        {
          if (self.game.Data.LibVarObj[libVar].type == NewEnums.LibVarType.General)
          {
            self.game.Data.LibVarObj[libVar].value = dataClass2.LibVarObj[index].value;
            self.game.Data.LibVarObj[libVar].valueText = dataClass2.LibVarObj[index].valueText;
            flag = true;
          }
          else if (self.game.Data.LibVarObj[libVar].type != NewEnums.LibVarType.Hex)
          {
            if (self.game.Data.LibVarObj[libVar].type == NewEnums.LibVarType.HistoricalUnit & dataClass2.LibVarObj[index].instanceId.libSlot > -1 & dataClass2.LibVarObj[index].instanceId.id > -1)
            {
              let mut library2: i32 = self.game.Data.FindLibrary(dataClass2.LibraryObj[dataClass2.LibVarObj[index].instanceId.libSlot].name);
              let mut id: i32 = dataClass2.LibVarObj[index].instanceId.id;
              let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
              for (let mut slotId: i32 = 0; slotId <= historicalUnitCounter; slotId += 1)
              {
                if (self.game.Data.HistoricalUnitObj[slotId].LibId.libSlot == library2 && self.game.Data.HistoricalUnitObj[slotId].LibId.id == id)
                {
                  let mut libVarUseId: i32 = self.game.Data.GetLibVarUseId(libVar, slotId);
                  if (libVarUseId == libVar)
                  {
                    self.game.Data.AddLibVar(library1);
                    self.game.Data.LibVarObj[self.game.Data.LibVarCounter] = self.game.Data.LibVarObj[libVar].Clone();
                    self.game.Data.LibVarObj[libVarUseId].value = dataClass2.LibVarObj[index].value;
                    self.game.Data.LibVarObj[libVarUseId].valueText = dataClass2.LibVarObj[index].valueText;
                    self.game.Data.LibVarObj[libVarUseId].instanceId.libSlot = library2;
                    self.game.Data.LibVarObj[libVarUseId].instanceId.id = id;
                  }
                  else
                  {
                    self.game.Data.LibVarObj[libVarUseId].value = dataClass2.LibVarObj[index].value;
                    self.game.Data.LibVarObj[libVarUseId].valueText = dataClass2.LibVarObj[index].valueText;
                    flag = true;
                  }
                }
              }
            }
            else if (self.game.Data.LibVarObj[libVar].type != NewEnums.LibVarType.HistoricalUnitModel && self.game.Data.LibVarObj[libVar].type != NewEnums.LibVarType.Landscape && self.game.Data.LibVarObj[libVar].type != NewEnums.LibVarType.LocationType && self.game.Data.LibVarObj[libVar].type != NewEnums.LibVarType.Officer && self.game.Data.LibVarObj[libVar].type != NewEnums.LibVarType.People)
            {
              if (self.game.Data.LibVarObj[libVar].type == NewEnums.LibVarType.Regime)
              {
                let mut library3: i32 = self.game.Data.FindLibrary(dataClass2.LibraryObj[dataClass2.LibVarObj[index].instanceId.libSlot].name);
                let mut id: i32 = dataClass2.LibVarObj[index].instanceId.id;
                let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
                for (let mut slotId: i32 = 0; slotId <= regimeCounter; slotId += 1)
                {
                  if (self.game.Data.RegimeObj[slotId].libId.libSlot == library3 && self.game.Data.RegimeObj[slotId].libId.id == id)
                  {
                    let mut libVarUseId: i32 = self.game.Data.GetLibVarUseId(libVar, slotId);
                    if (libVarUseId == libVar)
                    {
                      self.game.Data.AddLibVar(library1);
                      self.game.Data.LibVarObj[self.game.Data.LibVarCounter] = self.game.Data.LibVarObj[libVar].Clone();
                      self.game.Data.LibVarObj[libVarUseId].value = dataClass2.LibVarObj[index].value;
                      self.game.Data.LibVarObj[libVarUseId].valueText = dataClass2.LibVarObj[index].valueText;
                      self.game.Data.LibVarObj[libVarUseId].instanceId.libSlot = library3;
                      self.game.Data.LibVarObj[libVarUseId].instanceId.id = id;
                    }
                    else
                    {
                      self.game.Data.LibVarObj[libVarUseId].value = dataClass2.LibVarObj[index].value;
                      self.game.Data.LibVarObj[libVarUseId].valueText = dataClass2.LibVarObj[index].valueText;
                      flag = true;
                    }
                  }
                }
              }
              else if (self.game.Data.LibVarObj[libVar].type == NewEnums.LibVarType.River || self.game.Data.LibVarObj[libVar].type == NewEnums.LibVarType.Road || self.game.Data.LibVarObj[libVar].type != NewEnums.LibVarType.SFtype)
                ;
            }
          }
        }
        if (flag)
          num2 += 1;
        else
          num3 += 1;
      }
      dataClass1 = (DataClass) null;
      self.game.EditObj.UnitSelected = -1;
      self.game.EditObj.OldUnit = -1;
      self.game.FormRef.Cursor = Cursors.Default;
      let mut num4: i32 =  Interaction.MsgBox( ("Import completed succesfully. Imported " + num2.ToString() + " libvars and skipped " + num3.ToString() + "."), Title: ( "Shadow Empire : Planetary Conquest"));
    }
  }
}
