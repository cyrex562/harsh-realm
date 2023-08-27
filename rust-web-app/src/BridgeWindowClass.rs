// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.BridgeWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;

namespace WindowsApplication1
{
  pub class BridgeWindowClass : WindowClass
  {
     int OptionsListId;
     ListClass OptionsListObj;
     int BDrawId;
     int BDrawTextId;
     int BAltId;
     int BAltTextId;
     ListClass BasicListObj;
     int BasicListId;
     int BBasicSpriteId;
     int BChangeBasicSpriteId;
     int BAlternateSpriteId;
     int BChangeAlternateSpriteId;
     int[] Bitemid;
     int[] bitemtextid;
     int BEP;
     int BEPText;
     int bridgeNr;
     int TabSheetNr;
     int DetailNr;
     ss: String;

    pub BridgeWindowClass(ref tGame: GameClass)
      : base(ref tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Bridge")
    {
      this.Bitemid = new int[5];
      this.bitemtextid = new int[5];
      this.bridgeNr = 0;
      this.TabSheetNr = -1;
      this.DetailNr = -1;
      this.MakebridgeListGUI(-1);
    }

     void MakebridgeListGUI(int tbridgenr) => this.MakebridgeTypeItemGUI();

     void MakebridgeTypeItemGUI()
    {
      if (this.BAltId > 0)
        this.RemoveSubPart(this.BAltId);
      if (this.BAltTextId > 0)
        this.RemoveSubPart(this.BAltTextId);
      if (this.BDrawId > 0)
        this.RemoveSubPart(this.BDrawId);
      if (this.BDrawTextId > 0)
        this.RemoveSubPart(this.BDrawTextId);
      if (this.OptionsListId > 0)
        this.RemoveSubPart(this.OptionsListId);
      let mut index: i32 =  0;
      do
      {
        if (this.Bitemid[index] > 0)
          this.RemoveSubPart(this.Bitemid[index]);
        if (this.bitemtextid[index] > 0)
          this.RemoveSubPart(this.bitemtextid[index]);
        index += 1;
      }
      while (index <= 4);
      if (this.BEP > 0)
        this.RemoveSubPart(this.BEP);
      if (this.BEPText > 0)
        this.RemoveSubPart(this.BEPText);
      if (this.bridgeNr > -1)
      {
        this.ss = "Click to set the basic EP Cost to build a bridge. This is modified by the size of the river.";
        let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BEP = this.AddSubPart(ref tsubpart1, 10, 140, 32, 16, 1);
        let mut tsubpart2: SubPartClass =  TextPartClass::new("EPCOST=" + Conversion.Str( this.game.Data.BridgeObj[0].EPCost), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 400, 20, false, tDescript: this.ss);
        this.BEPText = this.AddSubPart(ref tsubpart2, 50, 139, 400, 20, 0);
        this.ss = "Click to draw a bridge on the map.";
        let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONDRAW, tDescript: this.ss);
        this.BDrawId = this.AddSubPart(ref tsubpart3, 10, 160, 32, 16, 1);
        let mut tsubpart4: SubPartClass =  TextPartClass::new("Select as pencil", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BDrawTextId = this.AddSubPart(ref tsubpart4, 50, 159, 200, 20, 0);
        this.ss = "Click to set with which roadtype the alternate graphic is used. -1=never";
        let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BAltId = this.AddSubPart(ref tsubpart5, 10, 180, 32, 16, 1);
        let mut tsubpart6: SubPartClass =  TextPartClass::new("Alt=" + Conversion.Str( this.game.Data.BridgeObj[0].AlternateIfRoadType) + ", " + Conversion.Str( this.game.Data.BridgeObj[0].AlternateIfRoadType2) + ", " + Conversion.Str( this.game.Data.BridgeObj[0].AlternateIfRoadType3) + ", " + Conversion.Str( this.game.Data.BridgeObj[0].AlternateIfRoadType4), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.BAltTextId = this.AddSubPart(ref tsubpart6, 50, 179, 200, 20, 0);
        this.OptionsListObj = ListClass::new();
        this.OptionsListObj.add("The 6 Sprites", 0);
        this.OptionsListObj.add("Statistics", 1);
        ListClass optionsListObj = this.OptionsListObj;
        let mut tabSheetNr: i32 =  this.TabSheetNr;
        let mut game: GameClass = this.game;
        ref local1: Bitmap = ref this.OwnBitmap;
        font: Font =  null;
        ref local2: Font = ref font;
        let mut tsubpart7: SubPartClass =  new ListSubPartClass(optionsListObj, 3, 300, tabSheetNr, game, tHeader: "Property Sheets", tbackbitmap: (ref local1), bbx: 370, bby: 140, overruleFont: (ref local2));
        this.OptionsListId = this.AddSubPart(ref tsubpart7, 370, 140, 300, 96, 0);
      }
      this.maketabsheet();
    }

     void maketabsheet()
    {
      if (this.BasicListId > 0)
        this.RemoveSubPart(this.BasicListId);
      if (this.BBasicSpriteId > 0)
        this.RemoveSubPart(this.BBasicSpriteId);
      if (this.BChangeBasicSpriteId > 0)
        this.RemoveSubPart(this.BChangeBasicSpriteId);
      if (this.BAlternateSpriteId > 0)
        this.RemoveSubPart(this.BAlternateSpriteId);
      if (this.BChangeAlternateSpriteId > 0)
        this.RemoveSubPart(this.BChangeAlternateSpriteId);
      if (!(this.bridgeNr > -1 & this.TabSheetNr > -1))
        return;
      if (this.TabSheetNr == 0)
        this.maketabsheetnr0();
      if (this.TabSheetNr != 1)
        return;
      this.maketabsheetnr1();
    }

     void maketabsheetnr0()
    {
      if (this.game.Data.LandscapeTypeObj[this.bridgeNr].BasicSpriteCounter <= -1)
        return;
      this.BasicListObj = ListClass::new();
      let mut tdata: i32 =  0;
      do
      {
        this.BasicListObj.add(this.game.Data.BridgeObj[this.bridgeNr].BasicSpriteFileName[tdata], tdata);
        tdata += 1;
      }
      while (tdata <= 5);
      ListClass basicListObj = this.BasicListObj;
      let mut detailNr: i32 =  this.DetailNr;
      let mut game: GameClass = this.game;
      ref local1: Bitmap = ref this.OwnBitmap;
      font: Font =  null;
      ref local2: Font = ref font;
      let mut tsubpart: SubPartClass =  new ListSubPartClass(basicListObj, 10, 300, detailNr, game, tHeader: "Sprites", tbackbitmap: (ref local1), bbx: 10, bby: 350, overruleFont: (ref local2));
      this.BasicListId = this.AddSubPart(ref tsubpart, 10, 350, 300, 208, 0);
      if (this.DetailNr > 5)
        this.DetailNr = -1;
      if (this.DetailNr <= -1)
        return;
      this.maketabsheetnr0b();
    }

     void maketabsheetnr0b()
    {
      if (this.BBasicSpriteId > 0)
        this.RemoveSubPart(this.BBasicSpriteId);
      if (this.BChangeBasicSpriteId > 0)
        this.RemoveSubPart(this.BChangeBasicSpriteId);
      if (this.BAlternateSpriteId > 0)
        this.RemoveSubPart(this.BAlternateSpriteId);
      if (this.BChangeAlternateSpriteId > 0)
        this.RemoveSubPart(this.BChangeAlternateSpriteId);
      this.ss = "Click to change this sprite";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.Data.BridgeObj[this.bridgeNr].BasicSpriteID[this.DetailNr], tDescript: this.ss);
      this.BBasicSpriteId = this.AddSubPart(ref tsubpart1, 400, 350, 64, 48, 0);
      if (Strings.Len(this.game.Data.MasterFile) == 0)
      {
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BChangeBasicSpriteId = this.AddSubPart(ref tsubpart2, 400, 410, 32, 16, 1);
      }
      this.ss = "Click to change this alternate sprite";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.Data.BridgeObj[this.bridgeNr].AlternateSpriteID[this.DetailNr], tDescript: this.ss);
      this.BAlternateSpriteId = this.AddSubPart(ref tsubpart3, 600, 350, 64, 48, 0);
      if (Strings.Len(this.game.Data.MasterFile) != 0)
        return;
      let mut tsubpart4: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.BChangeAlternateSpriteId = this.AddSubPart(ref tsubpart4, 600, 410, 32, 16, 1);
    }

     void maketabsheetnr1()
    {
    }

    pub HandleMouseClick: WindowReturnClass(int x, int y, int b)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index: i32 =  0; index <= subPartCounter; index += 1)
        {
          if (x > this.SubPartX[index] & x < this.SubPartX[index] + this.SubPartW[index] && y > this.SubPartY[index] & y < this.SubPartY[index] + this.SubPartH[index])
          {
            let mut num1: i32 =  this.SubPartID[index];
            if (num1 == this.OptionsListId)
            {
              let mut num2: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num2 > -1)
              {
                this.TabSheetNr = num2;
                this.maketabsheet();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BasicListId)
            {
              let mut num3: i32 =  this.SubPartList[index].Click(x - this.SubPartX[index], y - this.SubPartY[index]);
              this.SubPartFlag[index] = true;
              if (num3 > -1)
              {
                this.DetailNr = num3;
                this.maketabsheetnr0b();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BChangeBasicSpriteId)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("Bitmaps (*.bmp)|*.bmp|Png|*.png", "Select File For Bridge Sprite:", this.game.AppPath + "graphics\\", true);
              if (File.Exists(this.game.AppPath + "graphics/" + filename))
              {
                this.game.Data.BridgeObj[this.bridgeNr].ReplaceBasicSprite(this.DetailNr, filename);
              }
              else
              {
                let mut num4: i32 =  (int) Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BChangeAlternateSpriteId)
            {
              filename: String = this.game.HandyFunctionsObj.LoadSomething("Bitmaps (*.bmp)|*.bmp|Png|*.png", "Select File For Bridge Sprite:", this.game.AppPath + "graphics\\", true);
              if (File.Exists(this.game.AppPath + "graphics/" + filename))
              {
                this.game.Data.BridgeObj[this.bridgeNr].ReplaceAlternateSprite(this.DetailNr, filename);
              }
              else
              {
                let mut num5: i32 =  (int) Interaction.MsgBox( "File does not exist. Operation ordered is canceled due to this.", Title: ( "Shadow Empire : Planetary Conquest"));
              }
              this.maketabsheet();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BDrawId)
            {
              this.game.EditObj.PencilType = 6;
              this.game.EditObj.PencilData1 = this.bridgeNr;
              windowReturnClass.AddCommand(1, 13);
              windowReturnClass.AddCommand(2, 13);
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAltId)
            {
              let mut num6: i32 =  (int) Math.Round(Conversion.Val(Interaction.InputBox("TYPE: Give roadtype or -1", "Shadow Empire : Planetary Conquest")));
              if (num6 >= -1 & num6 <= this.game.Data.RoadTypeCounter)
                this.game.Data.BridgeObj[0].AlternateIfRoadType = num6;
              let mut num7: i32 =  (int) Math.Round(Conversion.Val(Interaction.InputBox("SECOND TYPE: Give roadtype or -1", "Shadow Empire : Planetary Conquest")));
              if (num7 >= -1 & num7 <= this.game.Data.RoadTypeCounter)
                this.game.Data.BridgeObj[0].AlternateIfRoadType2 = num7;
              let mut num8: i32 =  (int) Math.Round(Conversion.Val(Interaction.InputBox("THIRD TYPE: Give roadtype or -1", "Shadow Empire : Planetary Conquest")));
              if (num8 >= -1 & num8 <= this.game.Data.RoadTypeCounter)
                this.game.Data.BridgeObj[0].AlternateIfRoadType3 = num8;
              let mut num9: i32 =  (int) Math.Round(Conversion.Val(Interaction.InputBox("FOURTH TYPE: Give roadtype or -1", "Shadow Empire : Planetary Conquest")));
              if (num9 >= -1 & num9 <= this.game.Data.RoadTypeCounter)
                this.game.Data.BridgeObj[0].AlternateIfRoadType4 = num9;
              this.MakebridgeTypeItemGUI();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BEP)
            {
              let mut num10: i32 =  (int) Math.Round(Conversion.Val(Interaction.InputBox("TYPE: Give new EP cost", "Shadow Empire : Planetary Conquest")));
              if (num10 > -1 & num10 < 9999)
                this.game.Data.BridgeObj[0].EPCost = num10;
              this.MakebridgeTypeItemGUI();
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
