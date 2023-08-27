// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.SimpleEditRegimeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.Drawing.Drawing2D;
// usingSystem.Drawing.Text;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class SimpleEditRegimeWindowClass : WindowClass
  {
     listId: i32;
     ListClass listObj;
     list2Id: i32;
     ListClass list2Obj;
     regId: i32;
     offid: i32;
     AddOff: i32;
     RemoveOff: i32;
     RemoveOffB: i32;
     AddRegime: i32;
     RemoveRegime: i32;
     ChangeFlag: i32;
     changeColorExt: i32;
     AddRegimeB: i32;
     RemoveRegimeB: i32;
     ChangeColorBack: i32;
     ChangeColorFront: i32;
     ChangeName: i32;
     ChangePeople: i32;
     ChangeIcon: i32;
     ChangeRoundel: i32;
     ChangeMirror: i32;

    pub SimpleEditRegimeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 50, 9, tDoBorders: 1, tHeaderString: "Regimes")
    {
      self.regId = -1;
      self.offid = -1;
      self.DoStuff();
    }

    pub fn DoRefresh() => self.DoStuff();

    pub fn DoStuff()
    {
      let mut num1: i32 =  Math.Round( (self.game.ScreenWidth - 1024) / 2.0);
      if (self.listId > 0)
        self.RemoveSubPart(self.listId);
      if (self.list2Id > 0)
        self.RemoveSubPart(self.list2Id);
      if (self.AddOff > 0)
        self.RemoveSubPart(self.AddOff);
      if (self.RemoveOff > 0)
        self.RemoveSubPart(self.RemoveOff);
      if (self.RemoveOffB > 0)
        self.RemoveSubPart(self.RemoveOffB);
      if (self.AddRegime > 0)
        self.RemoveSubPart(self.AddRegime);
      if (self.AddRegimeB > 0)
        self.RemoveSubPart(self.AddRegimeB);
      if (self.RemoveRegime > 0)
        self.RemoveSubPart(self.RemoveRegime);
      if (self.RemoveRegimeB > 0)
        self.RemoveSubPart(self.RemoveRegimeB);
      if (self.ChangeColorBack > 0)
        self.RemoveSubPart(self.ChangeColorBack);
      if (self.ChangeColorFront > 0)
        self.RemoveSubPart(self.ChangeColorFront);
      if (self.changeColorExt > 0)
        self.RemoveSubPart(self.changeColorExt);
      if (self.ChangeName > 0)
        self.RemoveSubPart(self.ChangeName);
      if (self.ChangePeople > 0)
        self.RemoveSubPart(self.ChangePeople);
      if (self.ChangeIcon > 0)
        self.RemoveSubPart(self.ChangeIcon);
      if (self.ChangeRoundel > 0)
        self.RemoveSubPart(self.ChangeRoundel);
      if (self.ChangeMirror > 0)
        self.RemoveSubPart(self.ChangeMirror);
      if (self.ChangeFlag > 0)
        self.RemoveSubPart(self.ChangeFlag);
      Graphics Expression = Graphics.FromImage((Image) self.OwnBitmap);
      Expression.SmoothingMode = SmoothingMode.AntiAlias;
      Expression.TextRenderingHint = TextRenderingHint.AntiAlias;
      Expression.TextContrast = 1;
      self.NewBackGroundAndClearAll(DrawMod.TGame.ScreenWidth, DrawMod.TGame.ScreenHeight - 50, -1);
      let mut num2: i32 = -1;
      let mut num3: i32 = -1;
      self.listObj = ListClass::new();
      let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
      for (let mut index: i32 = 0; index <= regimeCounter; index += 1)
      {
        num2 += 1;
        self.listObj.add(Conversion.Str( index) + ") " + self.game.Data.RegimeObj[index].Name, index);
        if (self.regId == index)
          num3 = num2;
      }
      if (num3 == -1)
        self.regId = -1;
      ListClass listObj = self.listObj;
      let mut tlistselect1: i32 = num3;
      let mut game1: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      let mut bbx1: i32 = 10 + num1;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(listObj, 5, 300, tlistselect1, game1, true, "Regimes", tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local1), bbx: bbx1, bby: 92, tMarcStyle: true, overruleFont: ( local2));
      self.listId = self.AddSubPart( tsubpart, 10 + num1, 72, 300, 128, 0);
      let mut num4: i32 = 198;
      if (self.game.Data.RegimeCounter < 1)
      {
        tsubpart =  new TextButtonPartClass("Add Regime", 140, "Click to add a regime",  self.OwnBitmap, 10 + num1, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.AddRegime = self.AddSubPart( tsubpart, 10 + num1, num4, 140, 35, 1);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Add Regime", 140, "Maximum 2 regime supported at the moment",  self.OwnBitmap, 10 + num1, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.AddRegimeB = self.AddSubPart( tsubpart, 10 + num1, num4, 140, 35, 1);
      }
      if (self.regId > -1)
      {
        tsubpart =  new TextButtonPartClass("Remove Regime", 140, "Click to remove selected regime",  self.OwnBitmap, 150 + num1, num4, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RemoveRegime = self.AddSubPart( tsubpart, 150 + num1, num4, 140, 35, 1);
      }
      else
      {
        tsubpart =  new TextButtonPartClass("Remove Regime", 140, "You must first select a regime.",  self.OwnBitmap, 150 + num1, num4, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.RemoveRegimeB = self.AddSubPart( tsubpart, 150 + num1, 262, 140, 35, 1);
      }
      if (self.regId > -1)
      {
        DrawMod.DrawBlock( Expression, 410 + num1, 72, 600, 320, 0, 0, 0, 60);
        DrawMod.DrawTextColouredMarc( Expression, "COMMANDER POOL", self.game.MarcFont4, 430 + num1, 82, Color.White);
        let mut num5: i32 = -1;
        let mut num6: i32 = -1;
        self.list2Obj = ListClass::new();
        let mut historicalUnitCounter: i32 = self.game.Data.HistoricalUnitCounter;
        for (let mut index: i32 = 0; index <= historicalUnitCounter; index += 1)
        {
          if (self.game.Data.HistoricalUnitObj[index].CommanderName.Length > 0 & self.game.Data.HistoricalUnitObj[index].TempRegime == self.regId & self.game.Data.HistoricalUnitObj[index].Pool & self.game.HandyFunctionsObj.GetUnitByHistorical(index) == -1)
          {
            num5 += 1;
            let mut people: i32 = self.game.Data.HistoricalUnitObj[index].People;
            if (people > -1)
              self.list2Obj.add(Conversion.Str( index) + ") " + self.game.Data.HistoricalUnitObj[index].CommanderName + " (" + self.game.Data.PeopleObj[people].Name + ")", index);
            else
              self.list2Obj.add(Conversion.Str( index) + ") " + self.game.Data.HistoricalUnitObj[index].CommanderName, index);
            if (self.offid == index)
              num6 = num5;
          }
        }
        if (num3 == -1)
          self.regId = -1;
        ListClass list2Obj = self.list2Obj;
        let mut tlistselect2: i32 = num6;
        let mut game2: GameClass = self.game;
         local3: Bitmap =  self.OwnBitmap;
        let mut bbx2: i32 = 430 + num1;
        font =  null;
         local4: Font =  font;
        tsubpart =  new ListSubPartClass(list2Obj, 15, 300, tlistselect2, game2, true, "Commander Pool", tValueWidth: 0, tdotopandbottom: false, tbackbitmap: ( local3), bbx: bbx2, bby: 102, tMarcStyle: true, overruleFont: ( local4));
        self.list2Id = self.AddSubPart( tsubpart, 430 + num1, 102, 300, 288, 0);
        tsubpart =  new TextButtonPartClass("Add Commander", 200, "Click to add commander",  self.OwnBitmap, 760 + num1, 102, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
        self.AddOff = self.AddSubPart( tsubpart, 760 + num1, 102, 200, 35, 1);
        if (self.offid > -1)
        {
          tsubpart =  new TextButtonPartClass("Remove Commander", 200, "Click to remove commander",  self.OwnBitmap, 760 + num1, 142, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.RemoveOff = self.AddSubPart( tsubpart, 760 + num1, 142, 200, 35, 1);
        }
        else
        {
          tsubpart =  new TextButtonPartClass("Remove Commander", 200, "Click to remove commander",  self.OwnBitmap, 760 + num1, 142, true, usefont: self.game.MarcFont3, useshadow: true, tMarcStyle: true);
          self.RemoveOffB = self.AddSubPart( tsubpart, 760 + num1, 142, 200, 35, 1);
        }
        let mut num7: i32 = 20 + num1;
        let mut y1: i32 = 266;
        DrawMod.DrawBlock( Expression, 10 + num1, y1 - 10, 340, 492, 0, 0, 0, 60);
        DrawMod.DrawTextColouredMarc( Expression, "Name", self.game.MarcFont4, num7, y1, Color.White);
        DrawMod.DrawTextColouredMarc( Expression, self.game.Data.RegimeObj[self.regId].Name, self.game.MarcFont3, num7, y1 + 20, Color.White);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the name of selected regime",  self.OwnBitmap, num7 + 250, y1 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangeName = self.AddSubPart( tsubpart, num7 + 250, y1 + 5, 60, 35, 1);
        let mut y2: i32 = y1 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "People", self.game.MarcFont4, num7, y2, Color.White);
        DrawMod.DrawTextColouredMarc( Expression, self.game.Data.PeopleObj[self.game.Data.RegimeObj[self.regId].People].Name, self.game.MarcFont3, num7, y2 + 20, Color.White);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the people of selected regime",  self.OwnBitmap, num7 + 250, y2 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangePeople = self.AddSubPart( tsubpart, num7 + 250, y2 + 5, 60, 35, 1);
        let mut y3: i32 = y2 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "Counter background color", self.game.MarcFont4, num7, y3, Color.White);
        DrawMod.DrawBlock( Expression, num7, y3 + 20, 240, 16, self.game.Data.RegimeObj[self.regId].Red, self.game.Data.RegimeObj[self.regId].Green, self.game.Data.RegimeObj[self.regId].Blue,  byte.MaxValue);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the background of: Color the counter. ",  self.OwnBitmap, num7 + 250, y3 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangeColorBack = self.AddSubPart( tsubpart, num7 + 250, y3 + 5, 60, 35, 1);
        let mut y4: i32 = y3 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "Counter forground color", self.game.MarcFont4, num7, y4, Color.White);
        DrawMod.DrawBlock( Expression, num7, y4 + 20, 240, 16, self.game.Data.RegimeObj[self.regId].Red2, self.game.Data.RegimeObj[self.regId].Green2, self.game.Data.RegimeObj[self.regId].Blue2,  byte.MaxValue);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the text and siluet printing on the counters.",  self.OwnBitmap, num7 + 250, y4 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangeColorFront = self.AddSubPart( tsubpart, num7 + 250, y4 + 5, 60, 35, 1);
        let mut y5: i32 = y4 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "Frontline overlay color", self.game.MarcFont4, num7, y5, Color.White);
        DrawMod.DrawBlock( Expression, num7, y5 + 20, 240, 16, self.game.Data.RegimeObj[self.regId].Red3, self.game.Data.RegimeObj[self.regId].Green3, self.game.Data.RegimeObj[self.regId].Blue3,  byte.MaxValue);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the colour of the shading of the hexes and frontlines for this regime.",  self.OwnBitmap, num7 + 250, y5 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.changeColorExt = self.AddSubPart( tsubpart, num7 + 250, y5 + 5, 60, 35, 1);
        let mut y6: i32 = y5 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "Mirror counter siluets", self.game.MarcFont4, num7, y6, Color.White);
        DrawMod.DrawTextColouredMarc( Expression, self.game.Data.RegimeObj[self.regId].Mirror.ToString(), self.game.MarcFont3, num7, y6 + 20, Color.White);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the mirror setting. Mirroring swaps orientation of siluet on counter.",  self.OwnBitmap, num7 + 250, y6 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangeMirror = self.AddSubPart( tsubpart, num7 + 250, y6 + 5, 60, 35, 1);
        let mut y7: i32 = y6 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "National icon", self.game.MarcFont4, num7, y7, Color.White);
         let mut local5: &Graphics = &Expression;
        bitmap1: Bitmap = BitmapStore.GetBitmap(self.game.Data.RegimeObj[self.regId].NationalIconSprite, -1);
         let mut local6: &Bitmap = &bitmap1;
        let mut x1: i32 = num7 + 20;
        let mut y8: i32 = y7 + 23;
        DrawMod.DrawSimple( local5,  local6, x1, y8);
         let mut local7: &Graphics = &Expression;
        bitmap2: Bitmap = BitmapStore.GetBitmap(self.game.Data.RegimeObj[self.regId].NationalIconSprite);
         let mut local8: &Bitmap = &bitmap2;
        let mut x2: i32 = num7 + 40;
        let mut y9: i32 = y7 + 23;
        DrawMod.DrawSimple( local7,  local8, x2, y9);
         let mut local9: &Graphics = &Expression;
        bitmap3: Bitmap = BitmapStore.GetBitmap(self.game.Data.RegimeObj[self.regId].NationalIconSprite, 1);
         let mut local10: &Bitmap = &bitmap3;
        let mut x3: i32 = num7 + 60;
        let mut y10: i32 = y7 + 23;
        DrawMod.DrawSimple( local9,  local10, x3, y10);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the national icon of the selected regime",  self.OwnBitmap, num7 + 250, y7 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangeIcon = self.AddSubPart( tsubpart, num7 + 250, y7 + 5, 60, 35, 1);
        let mut y11: i32 = y7 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "Roundel sprite", self.game.MarcFont4, num7, y11, Color.White);
         let mut local11: &Graphics = &Expression;
        bitmap4: Bitmap = BitmapStore.GetBitmap(self.game.Data.RegimeObj[self.regId].RoundelIconSprite);
         let mut local12: &Bitmap = &bitmap4;
        let mut x4: i32 = num7 + 110;
        let mut y12: i32 = y11 + 3;
        DrawMod.DrawScaled( local11,  local12, x4, y12, 64, 64, true);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the roundel sprite of the selected regime",  self.OwnBitmap, num7 + 230, y11 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangeRoundel = self.AddSubPart( tsubpart, num7 + 250, y11 + 5, 60, 35, 1);
        let mut y13: i32 = y11 + 52;
        DrawMod.DrawTextColouredMarc( Expression, "(HQ) Flag", self.game.MarcFont4, num7, y13, Color.White);
         let mut local13: &Graphics = &Expression;
        bitmap5: Bitmap = BitmapStore.GetBitmap(self.game.Data.RegimeObj[self.regId].HQSpriteNr, -1);
         let mut local14: &Bitmap = &bitmap5;
        let mut x5: i32 = num7 + 90;
        let mut y14: i32 = y13 + 23;
        DrawMod.DrawSimple( local13,  local14, x5, y14);
         let mut local15: &Graphics = &Expression;
        bitmap6: Bitmap = BitmapStore.GetBitmap(self.game.Data.RegimeObj[self.regId].HQSpriteNr);
         let mut local16: &Bitmap = &bitmap6;
        let mut x6: i32 = num7 + 110;
        let mut y15: i32 = y13 + 13;
        DrawMod.DrawSimple( local15,  local16, x6, y15);
         let mut local17: &Graphics = &Expression;
        bitmap6 = BitmapStore.GetBitmap(self.game.Data.RegimeObj[self.regId].HQSpriteNr, 1);
         let mut local18: &Bitmap = &bitmap6;
        let mut x7: i32 = num7 + 140;
        let mut y16: i32 = y13 - 3;
        DrawMod.DrawSimple( local17,  local18, x7, y16);
        tsubpart =  new TextButtonPartClass("Change", 70, "Click to change the HQ flag of the selected regime",  self.OwnBitmap, num7 + 250, y13 + 5, usefont: self.game.MarcFont4, useshadow: true, tMarcStyle: true);
        self.ChangeFlag = self.AddSubPart( tsubpart, num7 + 250, y13 + 5, 60, 35, 1);
      }
      if (Information.IsNothing( Expression))
        return;
      Expression.Dispose();
      Expression = (Graphics) null;
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
            if (num1 == self.listId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (num2 > -1)
              {
                self.regId = num2;
                self.offid = -1;
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.list2Id)
            {
              let mut num3: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              if (num3 > -1)
                self.offid = num3;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.AddOff)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 102, self.regId, tGame: self.game);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.RemoveOff)
            {
              self.game.Data.HistoricalUnitObj[self.offid].Pool = false;
              self.offid = -1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.AddRegime)
            {
              if (self.game.Data.PeopleCounter < 0)
              {
                self.game.Data.AddPeople();
                self.game.Data.PeopleObj[0].Name = "Universals";
              }
              self.game.Data.AddRegime();
              self.regId = self.game.Data.RegimeCounter;
              self.game.Data.RegimeObj[self.game.Data.RegimeCounter].Name = "Regime #" + self.game.Data.RegimeCounter.ToString();
              self.game.Data.RegimeObj[self.game.Data.RegimeCounter].People = 0;
              let mut regimeCounter: i32 = self.game.Data.RegimeCounter;
              for (let mut index2: i32 = 0; index2 <= regimeCounter; index2 += 1)
              {
                self.game.Data.RegimeObj[self.game.Data.RegimeCounter].RegimeRel[index2] = 0;
                self.game.Data.RegimeObj[index2].RegimeRel[self.game.Data.RegimeCounter] = 0;
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.RemoveRegime)
            {
              self.game.Data.RemoveRegime(self.regId);
              self.regId = -1;
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeName)
            {
              self.game.Data.RegimeObj[self.regId].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangePeople)
            {
              Form3::new( self.formref).Initialize(self.game.Data, 3, self.regId);
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeColorBack)
            {
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[self.regId].Red, self.game.Data.RegimeObj[self.regId].Green, self.game.Data.RegimeObj[self.regId].Blue);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                RegimeClass regimeClass1 = self.game.Data.RegimeObj[self.regId];
                color: Color = colorDialog.Color;
                let mut r: i32 =  color.R;
                regimeClass1.Red = r;
                RegimeClass regimeClass2 = self.game.Data.RegimeObj[self.regId];
                color = colorDialog.Color;
                let mut g: i32 =  color.G;
                regimeClass2.Green = g;
                RegimeClass regimeClass3 = self.game.Data.RegimeObj[self.regId];
                color = colorDialog.Color;
                let mut b1: i32 =  color.B;
                regimeClass3.Blue = b1;
                self.game.Data.RegimeObj[self.regId].HexBack = (Bitmap) null;
                self.game.Data.RegimeObj[self.regId].TempCounter = (Bitmap) null;
                self.game.Data.RegimeObj[self.regId].DoTempCounter();
                self.game.Data.RegimeObj[self.regId].DoTempCounterBig();
                self.game.Data.RegimeObj[self.regId].DoTempCounterSmall();
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeColorFront)
            {
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[self.regId].Red2, self.game.Data.RegimeObj[self.regId].Green2, self.game.Data.RegimeObj[self.regId].Blue2);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                RegimeClass regimeClass4 = self.game.Data.RegimeObj[self.regId];
                color: Color = colorDialog.Color;
                let mut r: i32 =  color.R;
                regimeClass4.Red2 = r;
                RegimeClass regimeClass5 = self.game.Data.RegimeObj[self.regId];
                color = colorDialog.Color;
                let mut g: i32 =  color.G;
                regimeClass5.Green2 = g;
                RegimeClass regimeClass6 = self.game.Data.RegimeObj[self.regId];
                color = colorDialog.Color;
                let mut b2: i32 =  color.B;
                regimeClass6.Blue2 = b2;
                self.game.Data.RegimeObj[self.regId].HexBack = (Bitmap) null;
                self.game.Data.RegimeObj[self.regId].TempCounter = (Bitmap) null;
                self.game.Data.RegimeObj[self.regId].DoTempCounter();
                self.game.Data.RegimeObj[self.regId].DoTempCounterBig();
                self.game.Data.RegimeObj[self.regId].DoTempCounterSmall();
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.changeColorExt)
            {
              ColorDialog colorDialog = ColorDialog::new();
              colorDialog.Color = Color.FromArgb( byte.MaxValue, self.game.Data.RegimeObj[self.regId].Red3, self.game.Data.RegimeObj[self.regId].Green3, self.game.Data.RegimeObj[self.regId].Blue3);
              if (colorDialog.ShowDialog() == DialogResult.OK)
              {
                RegimeClass regimeClass7 = self.game.Data.RegimeObj[self.regId];
                color: Color = colorDialog.Color;
                let mut r: i32 =  color.R;
                regimeClass7.Red3 = r;
                RegimeClass regimeClass8 = self.game.Data.RegimeObj[self.regId];
                color = colorDialog.Color;
                let mut g: i32 =  color.G;
                regimeClass8.Green3 = g;
                RegimeClass regimeClass9 = self.game.Data.RegimeObj[self.regId];
                color = colorDialog.Color;
                let mut b3: i32 =  color.B;
                regimeClass9.Blue3 = b3;
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeIcon)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.png)|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of National Identifier of this regime:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.RegimeObj[self.regId].ReplaceNationalSprite(filename);
              }
              else
              {
                let mut num4: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeFlag)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.png)|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of HQ Symbol Sprite:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.RegimeObj[self.regId].ReplaceHQSprite(filename);
              }
              else
              {
                let mut num5: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeRoundel)
            {
              filename: String = self.game.HandyFunctionsObj.LoadSomething("Png (*.png)|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of National Identifier of this regime:", self.game.AppPath + "graphics\\", true);
              if (File.Exists(self.game.AppPath + "graphics/" + filename))
              {
                self.game.Data.RegimeObj[self.regId].ReplaceRoundelSprite(filename);
              }
              else
              {
                let mut num6: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              self.DoStuff();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.ChangeMirror)
            {
              self.game.Data.RegimeObj[self.regId].Mirror = !self.game.Data.RegimeObj[self.regId].Mirror;
              self.DoStuff();
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
