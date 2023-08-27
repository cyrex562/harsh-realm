// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.ResearchWindowClass
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
  pub class ResearchWindowClass : WindowClass
  {
     ResearchListId: i32;
     ListClass ResearchListObj;
     BAddResearchId: i32;
     BAddResearchTextId: i32;
     BNameId: i32;
     BNameTextId: i32;
     B1Id: i32;
     B1TextId: i32;
     B2Id: i32;
     B2TextId: i32;
     B3Id: i32;
     B3TextId: i32;
     BRemoveResearchId: i32;
     BRemoveResearchTextId: i32;
     BRemoveResearchId2: i32;
     BRemoveResearchTextId2: i32;
     B4Id: i32;
     B4TextId: i32;
     B5Id: i32;
     B5TextId: i32;
     B6Id: i32;
     B6TextId: i32;
     B7Id: i32;
     B7TextId: i32;
     B8Id: i32;
     B8TextId: i32;
     B9Id: i32;
     B9TextId: i32;
     B10Id: i32;
     B10TextId: i32;
     B11Id: i32;
     B11TextId: i32;
     PGListId: i32;
     ListClass PGListObj;
     B3bId: i32;
     B3bTextId: i32;
     ResearchNr: i32;
     detailnr: i32;
     ss: String;

    pub ResearchWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Decision Room")
    {
      self.ResearchNr = -1;
      self.detailnr = -1;
      self.MakeResearchListGUI(-1);
    }

    pub fn DoRefresh() => self.MakeResearchTypeItemGUI();

     void MakeResearchListGUI(tResearchnr: i32)
    {
      if (self.ResearchListId > 0)
        self.RemoveSubPart(self.ResearchListId);
      SubPartClass tsubpart;
      if (self.game.Data.ResearchCounter > -1)
      {
        self.ResearchListObj = ListClass::new();
        let mut researchCounter: i32 = self.game.Data.ResearchCounter;
        for (let mut index: i32 = 0; index <= researchCounter; index += 1)
          self.ResearchListObj.add(Conversion.Str( index) + ") " + self.game.Data.ResearchObj[index].Name, index);
        ListClass researchListObj = self.ResearchListObj;
        let mut tlistselect: i32 = tResearchnr;
        let mut game: GameClass = self.game;
         local1: Bitmap =  self.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        tsubpart =  new ListSubPartClass(researchListObj, 9, 200, tlistselect, game, tHeader: "Researchfields", tbackbitmap: ( local1), bbx: 10, bby: 50, overruleFont: ( local2));
        self.ResearchListId = self.AddSubPart( tsubpart, 10, 50, 200, 192, 0);
        self.ResearchNr = tResearchnr;
        self.MakeResearchTypeItemGUI();
      }
      else
      {
        self.ResearchNr = tResearchnr;
        self.MakeResearchTypeItemGUI();
      }
      if (self.BAddResearchId > 0)
        self.RemoveSubPart(self.BAddResearchId);
      if (self.BAddResearchTextId > 0)
        self.RemoveSubPart(self.BAddResearchTextId);
      self.ss = "Click to add a researchfield";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONPLUS, tDescript: self.ss);
        self.BAddResearchId = self.AddSubPart( tsubpart, 10, 270, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  TextPartClass::new("Add a Research", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
        self.BAddResearchTextId = self.AddSubPart( tsubpart, 50, 269, 300, 20, 0);
      }
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B7TextId > 0)
        self.RemoveSubPart(self.B7TextId);
      self.ss = "Clicking this button sets cost for all pplgroups equal to that of the first peoplegroup.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart =  ButtonPartClass::new(self.game.BUTTONYELLOW, tDescript: self.ss);
        self.B7Id = self.AddSubPart( tsubpart, 10, 310, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) != 0)
        return;
      tsubpart =  TextPartClass::new("Set all to pplgroup0", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
      self.B7TextId = self.AddSubPart( tsubpart, 50, 309, 300, 20, 0);
    }

     void MakeResearchTypeItemGUI()
    {
      if (self.BNameId > 0)
        self.RemoveSubPart(self.BNameId);
      if (self.BNameTextId > 0)
        self.RemoveSubPart(self.BNameTextId);
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
      if (self.B3bId > 0)
        self.RemoveSubPart(self.B3bId);
      if (self.B3bTextId > 0)
        self.RemoveSubPart(self.B3bTextId);
      if (self.PGListId > 0)
        self.RemoveSubPart(self.PGListId);
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
      if (self.B7Id > 0)
        self.RemoveSubPart(self.B7Id);
      if (self.B7TextId > 0)
        self.RemoveSubPart(self.B7TextId);
      if (self.B8Id > 0)
        self.RemoveSubPart(self.B8Id);
      if (self.B8TextId > 0)
        self.RemoveSubPart(self.B8TextId);
      if (self.B9Id > 0)
        self.RemoveSubPart(self.B9Id);
      if (self.B9TextId > 0)
        self.RemoveSubPart(self.B9TextId);
      if (self.B10Id > 0)
        self.RemoveSubPart(self.B10Id);
      if (self.B10TextId > 0)
        self.RemoveSubPart(self.B10TextId);
      if (self.B11Id > 0)
        self.RemoveSubPart(self.B11Id);
      if (self.B11TextId > 0)
        self.RemoveSubPart(self.B11TextId);
      if (self.BRemoveResearchId > 0)
        self.RemoveSubPart(self.BRemoveResearchId);
      if (self.BRemoveResearchTextId > 0)
        self.RemoveSubPart(self.BRemoveResearchTextId);
      if (self.BRemoveResearchId2 > 0)
        self.RemoveSubPart(self.BRemoveResearchId2);
      if (self.BRemoveResearchTextId2 > 0)
        self.RemoveSubPart(self.BRemoveResearchTextId2);
      self.ss = "Clicking this button sets cost for all pplgroups equal to that of the first peoplegroup.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B7Id = self.AddSubPart( tsubpart, 10, 310, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  TextPartClass::new("Set all to pplgroup0", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: self.ss);
        self.B7TextId = self.AddSubPart( tsubpart, 50, 309, 300, 20, 0);
      }
      if (self.ResearchNr <= -1)
        return;
      self.ss = "Click to change the name of the researchfield";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.BNameId = self.AddSubPart( tsubpart, 370, 50, 32, 16, 1);
      }
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Name: " + self.game.Data.ResearchObj[self.ResearchNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.BNameTextId = self.AddSubPart( tsubpart1, 410, 49, 400, 20, 0);
      self.ss = "Click to give a short one sentence description of this researchfield";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B1Id = self.AddSubPart( tsubpart2, 370, 70, 32, 16, 1);
      }
      let mut tsubpart3: SubPartClass =  TextPartClass::new("Text: " + self.game.Data.ResearchObj[self.ResearchNr].Text, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B1TextId = self.AddSubPart( tsubpart3, 410, 69, 400, 20, 0);
      self.ss = "Click to assign the picture of a sftype to this researchfield. -1=none. This is also used by the AI for research directions!";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart4: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B2Id = self.AddSubPart( tsubpart4, 370, 90, 32, 16, 1);
      }
      if (self.game.Data.ResearchObj[self.ResearchNr].SFTypePic > -1)
      {
        let mut tsubpart5: SubPartClass =  TextPartClass::new("SFTypeNr Sprite: " + self.game.Data.SFTypeObj[self.game.Data.ResearchObj[self.ResearchNr].SFTypePic].Name + " (" + Conversion.Str( self.game.Data.ResearchObj[self.ResearchNr].SFTypePic) + ")", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.B2TextId = self.AddSubPart( tsubpart5, 410, 89, 400, 20, 0);
      }
      else
      {
        let mut tsubpart6: SubPartClass =  TextPartClass::new("SFTypeNr Sprite: -1", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
        self.B2TextId = self.AddSubPart( tsubpart6, 410, 89, 400, 20, 0);
      }
      self.ss = "Click to set which other researchfield is a prerequisite for this one";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B4Id = self.AddSubPart( tsubpart7, 370, 110, 32, 16, 1);
      }
      let mut tsubpart8: SubPartClass =  TextPartClass::new("PreReq: " + Conversion.Str( self.game.Data.ResearchObj[self.ResearchNr].PreReq), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B4TextId = self.AddSubPart( tsubpart8, 410, 109, 400, 20, 0);
      self.ss = "Click to set which other researchfield is a prerequisite for this one";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B5Id = self.AddSubPart( tsubpart8, 370, 130, 32, 16, 1);
      }
      tsubpart8 =  TextPartClass::new("PreReq2: " + Conversion.Str( self.game.Data.ResearchObj[self.ResearchNr].PreReq2), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B5TextId = self.AddSubPart( tsubpart8, 410, 129, 400, 20, 0);
      self.ss = "Click to set which researchfield is blocked once this researchfield is bought by a regime. -1=none";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B6Id = self.AddSubPart( tsubpart8, 370, 150, 32, 16, 1);
      }
      tsubpart8 =  TextPartClass::new("Blocks: " + Conversion.Str( self.game.Data.ResearchObj[self.ResearchNr].Blocks), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B6TextId = self.AddSubPart( tsubpart8, 410, 149, 400, 20, 0);
      self.ss = "Click to set tech level (used in regime menu and by random game). -1/0 = no tech level assigned.";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B9Id = self.AddSubPart( tsubpart8, 370, 170, 32, 16, 1);
      }
      tsubpart8 =  TextPartClass::new("TechLevel: " + Conversion.Str( self.game.Data.ResearchObj[self.ResearchNr].TechLevel), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B9TextId = self.AddSubPart( tsubpart8, 410, 169, 400, 20, 0);
      self.ss = "Click to set cost type. -1=PP =>0 is the regimevar specified";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B10Id = self.AddSubPart( tsubpart8, 370, 190, 32, 16, 1);
      }
      tsubpart8 =  TextPartClass::new("CostType: " + Conversion.Str( self.game.Data.ResearchObj[self.ResearchNr].CostType), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: self.ss);
      self.B10TextId = self.AddSubPart( tsubpart8, 410, 189, 400, 20, 0);
      self.ss = "Click to remove this researchfield";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
        self.BRemoveResearchId = self.AddSubPart( tsubpart8, 10, 290, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  TextPartClass::new("Remove this Research", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BRemoveResearchTextId = self.AddSubPart( tsubpart8, 50, 289, 200, 20, 0);
      }
      self.ss = "Click to remove ALL researchfield";
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  ButtonPartClass::new(self.game.BUTTONKILL, tDescript: self.ss);
        self.BRemoveResearchId2 = self.AddSubPart( tsubpart8, 10, 330, 32, 16, 1);
      }
      if (Strings.Len(self.game.Data.MasterFile) == 0)
      {
        tsubpart8 =  TextPartClass::new("Remove ALL Research", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: self.ss);
        self.BRemoveResearchTextId2 = self.AddSubPart( tsubpart8, 50, 329, 200, 20, 0);
      }
      self.PGListObj = ListClass::new();
      if (self.detailnr < -1 | self.detailnr > 99)
        self.detailnr = -1;
      let mut index: i32 = 0;
      do
      {
        self.PGListObj.add(Conversion.Str( index) + ") " + self.game.Data.TempString[index + 200] + " = " + Conversion.Str( self.game.Data.ResearchObj[self.ResearchNr].PointCost[index]), index);
        index += 1;
      }
      while (index <= 99);
      ListClass pgListObj = self.PGListObj;
      let mut detailnr: i32 = self.detailnr;
      let mut game: GameClass = self.game;
       local1: Bitmap =  self.OwnBitmap;
      font: Font =  null;
       local2: Font =  font;
      tsubpart8 =  new ListSubPartClass(pgListObj, 6, 200, detailnr, game, tHeader: "Research Cost", tbackbitmap: ( local1), bbx: 10, bby: 350, overruleFont: ( local2));
      self.PGListId = self.AddSubPart( tsubpart8, 10, 350, 200, 144, 0);
      self.maketabsheet3b();
    }

     void maketabsheet3b()
    {
      if (self.detailnr <= -1)
        return;
      if (self.B3bId > 0)
        self.RemoveSubPart(self.B3bId);
      if (self.B3bTextId > 0)
        self.RemoveSubPart(self.B3bTextId);
      self.ss = "Click to change the cost in pol.pts for selected peoplegroup to research this field. -1 = impossible to research this for this peoplegroup";
      if (Strings.Len(self.game.Data.MasterFile) == 0 | !self.game.Data.MasterfileReadPeople)
      {
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(self.game.BUTTONBLOCK, tDescript: self.ss);
        self.B3bId = self.AddSubPart( tsubpart, 215, 350, 32, 16, 1);
      }
      if (!(Strings.Len(self.game.Data.MasterFile) == 0 | !self.game.Data.MasterfileReadPeople))
        return;
      let mut tsubpart1: SubPartClass =  TextPartClass::new("Change", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: self.ss);
      self.B3bTextId = self.AddSubPart( tsubpart1, 250, 349, 200, 20, 0);
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
            if (num1 == self.ResearchListId)
            {
              let mut num2: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
              self.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                self.ResearchNr = num2;
                self.MakeResearchTypeItemGUI();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.BAddResearchId)
            {
              self.game.Data.AddResearch();
              self.MakeResearchListGUI(self.ResearchNr);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == self.B7Id)
            {
              str: String = Interaction.InputBox("We will set all to ppg0 pp cost, with the following mod (1.0 is none)", "Shadow Empire : Planetary Conquest", "1");
              if (Operators.CompareString(str, "", false) != 0)
              {
                float num3 =  Conversion.Val(str);
                let mut researchCounter: i32 = self.game.Data.ResearchCounter;
                for (let mut index2: i32 = 0; index2 <= researchCounter; index2 += 1)
                {
                  let mut num4: i32 = self.game.Data.ResearchObj[index2].PointCost[0];
                  let mut index3: i32 = 1;
                  do
                  {
                    self.game.Data.ResearchObj[index2].PointCost[index3] =  Math.Round( Conversion.Int( num4 * num3));
                    index3 += 1;
                  }
                  while (index3 <= 99);
                }
                self.MakeResearchListGUI(self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
            }
            else
            {
              if (num1 == self.BNameId)
              {
                self.game.Data.ResearchObj[self.ResearchNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest", self.game.Data.ResearchObj[self.ResearchNr].Name);
                self.MakeResearchListGUI(self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B1Id)
              {
                Form2::new( self.formref).Initialize(self.game.Data, 5, self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B4Id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 22, self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B5Id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 23, self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B6Id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 24, self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B9Id)
              {
                let mut num5: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("TechLevel (-1 = no tech level..no problem) .. ", "Shadow Empire : Planetary Conquest")));
                if (num5 >= -1 & num5 <= 999)
                {
                  self.game.Data.ResearchObj[self.ResearchNr].TechLevel = num5;
                }
                else
                {
                  let mut num6: i32 =  Interaction.MsgBox( "number between -1 and 999 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeResearchListGUI(self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B10Id)
              {
                let mut num7: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("CostType... -1=pp >=0 is regimevar ", "Shadow Empire : Planetary Conquest")));
                if (num7 >= -1 & num7 <= 499)
                {
                  self.game.Data.ResearchObj[self.ResearchNr].CostType = num7;
                }
                else
                {
                  let mut num8: i32 =  Interaction.MsgBox( "number between -1 and 499 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeResearchListGUI(self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B11Id)
              {
                let mut num9: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("UpgradeCost in prodpoints for improvement.. -1=not possible ", "Shadow Empire : Planetary Conquest")));
                if (num9 >= -1 & num9 <= 999999)
                {
                  self.game.Data.ResearchObj[self.ResearchNr].UpgradeCost = num9;
                }
                else
                {
                  let mut num10: i32 =  Interaction.MsgBox( "number between -1 and 999999 plz", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                self.MakeResearchListGUI(self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B2Id)
              {
                Form3::new( self.formref).Initialize(self.game.Data, 21, self.ResearchNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BRemoveResearchId)
              {
                self.game.Data.RemoveResearch(self.ResearchNr);
                self.MakeResearchListGUI(-1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.BRemoveResearchId2)
              {
                for (let mut researchCounter: i32 = self.game.Data.ResearchCounter; researchCounter >= 0; researchCounter += -1)
                  self.game.Data.RemoveResearch(researchCounter);
                self.MakeResearchListGUI(-1);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.PGListId)
              {
                let mut num11: i32 = self.SubPartList[index1].Click(x - self.SubPartX[index1], y - self.SubPartY[index1]);
                self.SubPartFlag[index1] = true;
                if (num11 > -1)
                {
                  self.detailnr = num11;
                  self.MakeResearchTypeItemGUI();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == self.B3bId)
              {
                let mut num12: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give new pocost: i32 for this peoplegroup 0-9999 (-1 or 9999 = impossible) ", "Shadow Empire : Planetary Conquest")));
                if (num12 < -1 | num12 > 9999)
                {
                  let mut num13: i32 =  Interaction.MsgBox( "Between 0 and 1000 please!", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                {
                  self.game.Data.ResearchObj[self.ResearchNr].PointCost[self.detailnr] = num12;
                  self.MakeResearchTypeItemGUI();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
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
