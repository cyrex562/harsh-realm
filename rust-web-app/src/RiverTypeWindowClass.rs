// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.RiverTypeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;

namespace WindowsApplication1
{
  pub class RiverTypeWindowClass : WindowClass
  {
     riverListId: i32;
     ListClass riverListObj;
     BAddriverId: i32;
     BAddriverTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     BCostModId: i32;
     BCostModTextid: i32;
     OptionsListId: i32;
     ListClass OptionsListObj;
     BRemoveriverId: i32;
     BRemoveriverTextId: i32;
     BDrawId: i32;
     b1Id: i32;
     BDrawTextId: i32;
     txt1: i32;
     txt2: i32;
     b1TextId: i32;
    pub b3id: i32;
    pub b3textid: i32;
    pub b4id: i32;
    pub b4textid: i32;
    pub b5id: i32;
    pub b5textid: i32;
    pub b6id: i32;
    pub b6textid: i32;
     ListClass BasicListObj;
     BasicListId: i32;
     BBasicSpriteId: i32;
     BChangeBasicSpriteId: i32;
     ListClass BasicList2Obj;
     BasicList2Id: i32;
     ChangeMvId: i32;
     ListClass BasicList3Obj;
     BasicList3Id: i32;
     ChangePenaltyId: i32;
     riverNr: i32;
     TabSheetNr: i32;
     DetailNr: i32;
     ss: String;

    pub fn DoRefresh() => self.MakeriverTypeItemGUI();

    pub RiverTypeWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "River Types")
    {
      self.riverNr = -1;
      self.TabSheetNr = -1;
      self.DetailNr = -1;
      self.MakeriverListGUI(-1);
    }

     void MakeriverListGUI(trivernr: i32)
    {
      if (self.riverListId > 0)
        self.RemoveSubPart(self.riverListId);
      SubPartClass tsubpart;
      if (self.game.Data.RiverTypeCounter > -1)
      {
        self.riverListObj = ListClass::new();
        let mut riverTypeCounter: i32 = self.game.Data.RiverTypeCounter;
        for (let mut index: i32 = 0; index <= riverTypeCounter; index += 1)
          self.riverListObj.add(Conversion.Str( index) + ") " + self.game.Data.RiverTypeObj[index].Name, index);
        ListClass riverListObj = self.riverListObj;
        let mut tlistselect: i32 = trivernr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart =  new ListSubPartClass(riverListObj, 9, 200, tlistselect, game, tHeader: "River Types", tbackbitmap: ( local1), bbx: 10, bby: 50, overruleFont: ( local2));
        self.riverListId = self.AddSubPart( tsubpart, 10, 50, 200, 192, 0);
        self.riverNr = trivernr;
        self.MakeriverTypeItemGUI();
      }
      else
      {
        self.riverNr = trivernr;
        self.MakeriverTypeItemGUI();
      }
      if (self.BAddriverId > 0)
        self.RemoveSubPart(self.BAddriverId);
      if (self.BAddriverTextId > 0)
        self.RemoveSubPart(self.BAddriverTextId);
      self.ss = "Click to add a river type";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.BAddriverId = self.AddSubPart( tsubpart, 10, 270, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      tsubpart =  TextPartClass::new("Add river Type", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.BAddriverTextId = self.AddSubPart( tsubpart, 50, 269, 300, 20, 0);
    }

     void MakeriverTypeItemGUI()
    {
      if (self.BNameId > 0)
        self.RemoveSubPart(self.BNameId);
      if (self.BNameTextId > 0)
        self.RemoveSubPart(self.BNameTextId);
      if (self.BCostModId > 0)
        self.RemoveSubPart(self.BCostModId);
      if (self.BCostModTextid > 0)
        self.RemoveSubPart(self.BCostModTextid);
      if (self.BRemoveriverId > 0)
        self.RemoveSubPart(self.BRemoveriverId);
      if (self.BRemoveriverTextId > 0)
        self.RemoveSubPart(self.BRemoveriverTextId);
      if (self.BDrawId > 0)
        self.RemoveSubPart(self.BDrawId);
      if (self.BDrawTextId > 0)
        self.RemoveSubPart(self.BDrawTextId);
      if (self.OptionsListId > 0)
        self.RemoveSubPart(self.OptionsListId);
      if (self.b3id > 0)
        self.RemoveSubPart(self.b3id);
      if (self.b3textid > 0)
        self.RemoveSubPart(self.b3textid);
      if (self.b4id > 0)
        self.RemoveSubPart(self.b4id);
      if (self.b4textid > 0)
        self.RemoveSubPart(self.b4textid);
      if (self.b5id > 0)
        self.RemoveSubPart(self.b5id);
      if (self.b5textid > 0)
        self.RemoveSubPart(self.b5textid);
      if (self.b6id > 0)
        self.RemoveSubPart(self.b6id);
      if (self.b6textid > 0)
        self.RemoveSubPart(self.b6textid);
      if (self.BasicList2Id > 0)
        self.RemoveSubPart(self.BasicList2Id);
      if (self.ChangeMvId > 0)
        self.RemoveSubPart(self.ChangeMvId);
      if (self.BasicList3Id > 0)
        self.RemoveSubPart(self.BasicList3Id);
      if (self.ChangePenaltyId > 0)
        self.RemoveSubPart(self.ChangePenaltyId);
      if (self.riverNr > -1)
      {
        self.ss = "Click to change name of this rivertype";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.BNameId = self.AddSubPart( tsubpart, 370, 50, 32, 16, 1);
        }
        let mut tsubpart1: SubPartClass =  TextPartClass::new("Name: " + self.game.Data.RiverTypeObj[self.riverNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.BNameTextId = self.AddSubPart( tsubpart1, 410, 49, 400, 20, 0);
        self.ss = "Click to change the modifier for the EP cost to build a bridge over this rivertype and the modifier for the structural points. dc3: set to -1 to never place bridge over this river type";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.BCostModId = self.AddSubPart( tsubpart2, 370, 70, 32, 16, 1);
        }
        let mut tsubpart3: SubPartClass =  TextPartClass::new("BridgeCostMod: " + Conversion.Str( self.game.Data.RiverTypeObj[self.riverNr].BridgeCostModifier), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.BCostModTextid = self.AddSubPart( tsubpart3, 410, 69, 400, 20, 0);
        self.ss = "Click to change transparent setting (works best in combination with overlay map)";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.b3id = self.AddSubPart( tsubpart3, 370, 90, 32, 16, 1);
        }
        tsubpart3 =  TextPartClass::new("Transparent: " + Conversion.Str( self.game.Data.RiverTypeObj[self.riverNr].Transparent), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.b3textid = self.AddSubPart( tsubpart3, 410, 89, 400, 20, 0);
        self.ss = "Click to change the thickness for mini/strat. map";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.b6id = self.AddSubPart( tsubpart3, 370, 110, 32, 16, 1);
        }
        tsubpart3 =  TextPartClass::new("Thickness: " + Conversion.Str( self.game.Data.RiverTypeObj[self.riverNr].Thickness), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.b6textid = self.AddSubPart( tsubpart3, 410, 109, 400, 20, 0);
        self.ss = "Click to set to Snake Mode drawing style";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.b4id = self.AddSubPart( tsubpart3, 770, 90, 32, 16, 1);
        }
        tsubpart3 =  TextPartClass::new("SnakeMode: " + Conversion.Str( self.game.Data.RiverTypeObj[self.riverNr].snakeMode), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.b4textid = self.AddSubPart( tsubpart3, 810, 89, 400, 20, 0);
        self.ss = "Click to set draw mode in editor.";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
          self.b5id = self.AddSubPart( tsubpart3, 770, 70, 32, 16, 1);
        }
        tsubpart3 =  TextPartClass::new("drawInteriorOnly: " + Conversion.Str( self.game.Data.RiverTypeObj[self.riverNr].drawInteriorOnly), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.b5textid = self.AddSubPart( tsubpart3, 810, 69, 400, 20, 0);
        self.ss = "Click to remove this rivertype";
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
          self.BRemoveriverId = self.AddSubPart( tsubpart3, 10, 290, 32, 16, 1);
        }
        if (Strings.Len(self.game.Data.MasterFile) == 0)
        {
          tsubpart3 =  TextPartClass::new("Remove this RiverType", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
          self.BRemoveriverTextId = self.AddSubPart( tsubpart3, 50, 289, 200, 20, 0);
        }
        self.ss = "Click to use this rivertype to draw on the map.";
        tsubpart3 =  ButtonPartClass::new(self.game.BUTTONDRAW, tDescript: self.ss);
        self.BDrawId = self.AddSubPart( tsubpart3, 10, 310, 32, 16, 1);
        tsubpart3 =  TextPartClass::new("Select as pencil", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BDrawTextId = self.AddSubPart( tsubpart3, 50, 309, 200, 20, 0);
        self.OptionsListObj = ListClass::new();
        self.OptionsListObj.add("Sprites", 0);
        self.OptionsListObj.add("Move-over River Penalties", 1);
        self.OptionsListObj.add("Attack-over River Penalties", 2);
        ListClass optionsListObj = self.OptionsListObj;
        let mut tabSheetNr: i32 = self.TabSheetNr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart3 =  new ListSubPartClass(optionsListObj, 3, 300, tabSheetNr, game, tHeader: "Property Sheets", tbackbitmap: ( local1), bbx: 370, bby: 140, overruleFont: ( local2));
        self.OptionsListId = self.AddSubPart( tsubpart3, 370, 140, 300, 96, 0);
      }
      self.maketabsheet();
    }

     void maketabsheet()
    {
      if (self.BasicListId > 0)
        self.RemoveSubPart(self.BasicListId);
      if (self.BBasicSpriteId > 0)
        self.RemoveSubPart(self.BBasicSpriteId);
      if (self.BChangeBasicSpriteId > 0)
        self.RemoveSubPart(self.BChangeBasicSpriteId);
      if (self.BasicList2Id > 0)
        self.RemoveSubPart(self.BasicList2Id);
      if (self.ChangeMvId > 0)
        self.RemoveSubPart(self.ChangeMvId);
      if (self.BasicList3Id > 0)
        self.RemoveSubPart(self.BasicList3Id);
      if (self.ChangePenaltyId > 0)
        self.RemoveSubPart(self.ChangePenaltyId);
      if (!(self.riverNr > -1 & self.TabSheetNr > -1))
        return;
      if (self.TabSheetNr == 0)
        self.maketabsheetnr0();
      if (self.TabSheetNr == 1)
        self.maketabsheetnr1();
      if (self.TabSheetNr != 2)
        return;
      self.maketabsheetnr2();
    }

     void maketabsheetnr0()
    {
      if (self.b1Id > 0)
        self.RemoveSubPart(self.b1Id);
      if (self.b1TextId > 0)
        self.RemoveSubPart(self.b1TextId);
      if (self.txt1 > 0)
        self.RemoveSubPart(self.txt1);
      if (self.txt2 > 0)
        self.RemoveSubPart(self.txt2);
      self.ss = "Click to change name of this rivertype";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLUE, tDescript: self.ss);
        self.b1Id = self.AddSubPart( tsubpart, 500, 350, 32, 16, 1);
      }
      txt1: String;
      if (self.game.Data.RiverTypeObj[self.riverNr].SpecialLayer)
      {
        txt1 = "Change to 6 sprites";
        self.ss = "Click to change to old style 6 sprite river";
      }
      else
      {
        txt1 = "Change to 64 sprites";
        self.ss = "Click to change to 64 sprite river graphics. ";
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new(txt1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.b1TextId = self.AddSubPart( tsubpart1, 550, 349, 400, 20, 0);
      if (self.txt1 > 0)
        self.RemoveSubPart(self.txt1);
      if (!self.game.Data.RiverTypeObj[self.riverNr].SpecialLayer)
      {
        if (self.game.Data.LandscapeTypeObj[self.riverNr].BasicSpriteCounter <= -1)
          return;
        self.BasicListObj = ListClass::new();
        let mut tdata: i32 = 0;
        do
        {
          self.BasicListObj.add(self.game.Data.RiverTypeObj[self.riverNr].BasicSpriteFileName[tdata], tdata);
          tdata += 1;
        }
        while (tdata <= 5);
        ListClass basicListObj = self.BasicListObj;
        let mut detailNr: i32 = self.DetailNr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(basicListObj, 10, 300, detailNr, game, tHeader: "Sprites", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
        self.BasicListId = self.AddSubPart( tsubpart2, 10, 350, 300, 208, 0);
        if (self.DetailNr > 5)
          self.DetailNr = -1;
        if (self.DetailNr <= -1)
          return;
        self.maketabsheetnr0b();
      }
      else
      {
        txt2: String = "Currently using a set of 64 sprites. Gfx: " + self.game.Data.RiverTypeObj[self.riverNr].LayerSpriteFileName[1];
        if (!self.game.Data.RiverTypeObj[self.riverNr].UseSheet)
        {
          let mut tsubpart3: SubPartClass =  TextPartClass::new(txt2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 550, 20, false, tDescript: self.ss);
          self.txt1 = self.AddSubPart( tsubpart3, 10, 398, 550, 20, 0);
        }
        else
        {
          self.ss = "the fred sheet you are currently using. filename = " + self.game.Data.RiverTypeObj[self.riverNr].SheetFileName;
          let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.Data.RiverTypeObj[self.riverNr].SheetSpriteID, tDescript: self.ss);
          self.txt1 = self.AddSubPart( tsubpart4, 10, 400, BitmapStore.GetWidth(self.game.Data.RiverTypeObj[self.riverNr].SheetSpriteID), BitmapStore.Getheight(self.game.Data.RiverTypeObj[self.riverNr].SheetSpriteID), 0);
        }
      }
    }

     void maketabsheetnr0b()
    {
      if (self.BBasicSpriteId > 0)
        self.RemoveSubPart(self.BBasicSpriteId);
      if (self.BChangeBasicSpriteId > 0)
        self.RemoveSubPart(self.BChangeBasicSpriteId);
      self.ss = "Click to change the selected sprite";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(self.game.Data.RiverTypeObj[self.riverNr].BasicSpriteID[self.DetailNr], tDescript: self.ss);
      self.BBasicSpriteId = self.AddSubPart( tsubpart1, 400, 350, BitmapStore.GetBitmap(self.game.Data.RiverTypeObj[self.riverNr].BasicSpriteID[self.DetailNr]).Width, 48, 0);
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.BChangeBasicSpriteId = self.AddSubPart( tsubpart2, 400, 410, 32, 16, 1);
    }

     void maketabsheetnr1()
    {
      if (self.BasicList2Id > 0)
        self.RemoveSubPart(self.BasicList2Id);
      if (self.ChangeMvId > 0)
        self.RemoveSubPart(self.ChangeMvId);
      self.BasicList2Obj = ListClass::new();
      let mut index: i32 = 0;
      do
      {
        self.BasicList2Obj.add(self.game.Data.TempString[index] + "(" + Conversion.Str( index) + ") = " + Conversion.Str( self.game.Data.RiverTypeObj[self.riverNr].MovePenalty[index]) + "ap", index);
        index += 1;
      }
      while (index <= 99);
      ListClass basicList2Obj = self.BasicList2Obj;
      let mut detailNr: i32 = self.DetailNr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(basicList2Obj, 10, 300, detailNr, game, tHeader: "Move-over River Penalties", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
      self.BasicList2Id = self.AddSubPart( tsubpart, 10, 350, 300, 208, 0);
      if (self.DetailNr > 99)
        self.DetailNr = -1;
      if (self.DetailNr <= -1)
        return;
      self.maketabsheetnr1b();
    }

     void maketabsheetnr1b()
    {
      if (self.ChangeMvId > 0)
        self.RemoveSubPart(self.ChangeMvId);
      self.ss = "Click to change the move penalty for crossing this river without a bridge over it";
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.ChangeMvId = self.AddSubPart( tsubpart, 400, 410, 32, 16, 1);
    }

     void maketabsheetnr2()
    {
      if (self.BasicList3Id > 0)
        self.RemoveSubPart(self.BasicList3Id);
      if (self.ChangePenaltyId > 0)
        self.RemoveSubPart(self.ChangePenaltyId);
      self.BasicList3Obj = ListClass::new();
      let mut index: i32 = 0;
      do
      {
        self.BasicList3Obj.add(self.game.Data.TempString[index + 400] + "(" + Conversion.Str( index) + ") = " + Conversion.Str( self.game.Data.RiverTypeObj[self.riverNr].AttackPenalty[index]), index);
        index += 1;
      }
      while (index <= 99);
      ListClass basicList3Obj = self.BasicList3Obj;
      let mut detailNr: i32 = self.DetailNr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(basicList3Obj, 10, 300, detailNr, game, tHeader: "Attack-over River Penalties", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
      self.BasicList3Id = self.AddSubPart( tsubpart, 10, 350, 300, 208, 0);
      if (self.DetailNr > 39)
        self.DetailNr = -1;
      if (self.DetailNr <= -1)
        return;
      self.maketabsheetnr2b();
    }

     void maketabsheetnr2b()
    {
      if (self.ChangePenaltyId > 0)
        self.RemoveSubPart(self.ChangePenaltyId);
      self.ss = "Click to change the attack penalty for unitgroup for attacking over river (for with bridge see rulevar(5)) example: 0.4=-40% 0=no mod";
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
      self.ChangePenaltyId = self.AddSubPart( tsubpart, 400, 410, 32, 16, 1);
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
            if (num1 == self.riverListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.riverNr = num2;
                self.MakeriverTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BAddriverId)
            {
              self.game.Data.AddRiverType();
              self.MakeriverListGUI(self.riverNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BNameId)
            {
              self.game.Data.RiverTypeObj[self.riverNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
              self.MakeriverListGUI(self.riverNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BCostModId)
            {
              float num3 =  Conversion.Val(Interaction.InputBox("Give new attack-over river mod.", "Shadow Empire : Planetary Conquest"));
              if ( num3 < -1.0 |  num3 > 999.0)
              {
                let mut num4: i32 =  Interaction.MsgBox( "Between -1 and 999 please. You can use 0.5 or 3.5 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              else
                self.game.Data.RiverTypeObj[self.riverNr].BridgeCostModifier = num3;
              if ( num3 == -1.0)
              {
                let mut num5: i32 = 0;
                let mut mapWidth: i32 = self.game.Data.MapObj[0].MapWidth;
                for (let mut index2: i32 = 0; index2 <= mapWidth; index2 += 1)
                {
                  let mut mapHeight: i32 = self.game.Data.MapObj[0].MapHeight;
                  for (let mut index3: i32 = 0; index3 <= mapHeight; index3 += 1)
                  {
                    let mut index4: i32 = 0;
                    do
                    {
                      if (self.game.Data.MapObj[0].HexObj[index2, index3].RiverType[index4] == self.riverNr && self.game.Data.MapObj[0].HexObj[index2, index3].Bridge[index4])
                      {
                        num5 += 1;
                        self.game.Data.MapObj[0].HexObj[index2, index3].Bridge[index4] = false;
                      }
                      index4 += 1;
                    }
                    while (index4 <= 5);
                  }
                }
                if (num5 > 0)
                {
                  let mut num6: i32 =  Interaction.MsgBox( ("Removed " + num5.ToString() + " bridges that were across this river type."), Title: ( "Shadow Empire : Planetary Conquest"));
                }
              }
              self.MakeriverListGUI(self.riverNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b3id)
            {
              self.game.Data.RiverTypeObj[self.riverNr].Transparent = !self.game.Data.RiverTypeObj[self.riverNr].Transparent;
              self.MakeriverListGUI(self.riverNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.b6id)
            {
              let mut num7: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new thickness please", "Shadow Empire : Planetary Conquest")));
              if (num7 >= 0 & num7 <= 9)
              {
                self.game.Data.RiverTypeObj[self.riverNr].Thickness = num7;
                self.MakeriverListGUI(self.riverNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              let mut num8: i32 =  Interaction.MsgBox( "Thickness must be in range 0-9.", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else
            {
              if (num1 == self.b4id)
              {
                self.game.Data.RiverTypeObj[self.riverNr].snakeMode = !self.game.Data.RiverTypeObj[self.riverNr].snakeMode;
                self.MakeriverListGUI(self.riverNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.b5id)
              {
                self.game.Data.RiverTypeObj[self.riverNr].drawInteriorOnly = !self.game.Data.RiverTypeObj[self.riverNr].drawInteriorOnly;
                self.MakeriverListGUI(self.riverNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.OptionsListId)
              {
                let mut num9: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num9 > -1)
                {
                  self.TabSheetNr = num9;
                  self.maketabsheet();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BRemoveriverId)
              {
                self.game.Data.RemoveRiverType(self.riverNr);
                self.MakeriverListGUI(-1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BasicListId)
              {
                let mut num10: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num10 > -1)
                {
                  self.DetailNr = num10;
                  self.maketabsheetnr0b();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BChangeBasicSpriteId)
              {
                filename: String = self.game.HandyFunctionsObj.LoadSomething("Bitmaps (*.bmp)|*.bmp|Png|*.png", "Select File For River Sprite:", self.game.AppPath + "graphics\\", true);
                if (File.Exists(self.game.AppPath + "graphics/" + filename))
                {
                  self.game.Data.RiverTypeObj[self.riverNr].ReplaceBasicSprite(self.DetailNr, filename);
                }
                else
                {
                  let mut num11: i32 =  Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to self.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.maketabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BDrawId)
              {
                self.game.EditObj.PencilType = 5;
                self.game.EditObj.PencilData1 = self.riverNr;
                windowReturnClass.AddCommand(1, 13);
                windowReturnClass.AddCommand(2, 13);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BasicList2Id)
              {
                let mut num12: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num12 > -1)
                {
                  self.DetailNr = num12;
                  self.maketabsheetnr1b();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.ChangeMvId)
              {
                let mut num13: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new ap overrule please", "Shadow Empire : Planetary Conquest")));
                if (num13 > -1 & num13 <= 9999)
                {
                  self.game.Data.RiverTypeObj[self.riverNr].MovePenalty[self.DetailNr] = num13;
                }
                else
                {
                  let mut num14: i32 =  Interaction.MsgBox( "Value between 0 and 10000 please...", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.maketabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BasicList3Id)
              {
                let mut num15: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num15 > -1)
                {
                  self.DetailNr = num15;
                  self.maketabsheetnr2b();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.ChangePenaltyId)
              {
                float num16 =  Conversion.Val(Interaction.InputBox("Give new attack-over river mod.", "Shadow Empire : Planetary Conquest"));
                if ( num16 < 0.0 |  num16 > 1.0)
                {
                  let mut num17: i32 =  Interaction.MsgBox( "Between 0 and 1 please. You can use 0.5 or 0.87 like values.", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                  self.game.Data.RiverTypeObj[self.riverNr].AttackPenalty[self.DetailNr] = num16;
                self.maketabsheet();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.b1Id)
              {
                self.game.Data.RiverTypeObj[self.riverNr].SpecialLayer = !self.game.Data.RiverTypeObj[self.riverNr].SpecialLayer;
                if (!self.game.Data.RiverTypeObj[self.riverNr].SpecialLayer)
                {
                  self.game.Data.RiverTypeObj[self.riverNr].UseSheet = false;
                  self.game.Data.RiverTypeObj[self.riverNr].SheetFileName = "systemgraphics/trans.bmp";
                }
                if (self.game.Data.RiverTypeObj[self.riverNr].SpecialLayer)
                {
                  if (Interaction.MsgBox( "Use Fred SpriteSheet?", MsgBoxStyle.YesNo) == MsgBoxResult.No)
                    self.game.Data.RiverTypeObj[self.riverNr].UseSheet = false;
                  else
                    self.game.Data.RiverTypeObj[self.riverNr].UseSheet = true;
                  if (!self.game.Data.RiverTypeObj[self.riverNr].UseSheet)
                  {
                    extstring: String = Interaction.InputBox("Give a graphical extension: .jpg, .png, .bmp");
                    dirstring: String = Interaction.InputBox("Give a directory name under the graphics directory", "Shadow Empire : Planetary Conquest");
                    if (File.Exists(self.game.AppPath + "graphics/" + dirstring + "/a1" + extstring))
                    {
                      self.game.Data.RiverTypeObj[self.riverNr].AutoLoadSpecial(dirstring, extstring);
                      self.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    self.game.Data.RiverTypeObj[self.riverNr].SpecialLayer = false;
                    let mut num18: i32 =  Interaction.MsgBox( "Could not find this dir... give it like 'sea' or 'africa/desert', make sure a1 is present.", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                  else
                  {
                    filename: String = self.game.HandyFunctionsObj.LoadSomething("Png|*.png|Bitmaps (*.bmp)|*.bmp", "Select normal sized sheet (big & small will be auto-linked)", self.game.AppPath + "graphics\\", true);
                    if (File.Exists(self.game.AppPath + "graphics/" + filename))
                    {
                      self.game.Data.RiverTypeObj[self.riverNr].ReplaceSpriteSheet(filename);
                      self.maketabsheet();
                      windowReturnClass.SetFlag(true);
                      return windowReturnClass;
                    }
                    let mut num19: i32 =  Interaction.MsgBox( "Could not find this file... ", Title: ( "Shadow Empire : Planetary Conquest"));
                  }
                }
                self.maketabsheet();
                windowReturnClass.SetFlag(true);
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
  }
}
