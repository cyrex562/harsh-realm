// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.EditHisWindowClass
// Assembly: WindowsApplication1, Version=1.0.8020.28903, Culture=neutral, PublicKeyToken=null
// MVID: F52869E5-0850-48AD-BBBE-68E7A4900AFE
// Assembly location: C:\Program Files (x86)\Steam\steamapps\common\Shadow Empire\ShadowEmpire.exe

// usingMicrosoft.VisualBasic;
// usingMicrosoft.VisualBasic.CompilerServices;
// usingSystem;
// usingSystem.Drawing;
// usingSystem.IO;
// usingSystem.Windows.Forms;

namespace WindowsApplication1
{
  pub class EditHisWindowClass : WindowClass
  {
     locNr: i32;
     BNameId: i32;
     BNameTextId: i32;
     BPopId: i32;
     BPopTextId: i32;
     BPplId: i32;
     BPplTextId: i32;
     BRemoveId: i32;
     BRemoveTextid: i32;
     apoolid: i32;
     apoolbid: i32;
     impid: i32;
     expid: i32;
     a1id: i32;
     a1bid: i32;
     a2id: i32;
     a2bid: i32;
     a3id: i32;
     a3bid: i32;
     a4id: i32;
     a4bid: i32;
     a5id: i32;
     a5bid: i32;
     a6id: i32;
     a6bid: i32;
     a7id: i32;
     a7bid: i32;
     a8id: i32;
     a8bid: i32;
     a9id: i32;
     a9bid: i32;
     x1id: i32;
     x1bid: i32;
     x2id: i32;
     x2bid: i32;
     x3id: i32;
     x3bid: i32;
     x4id: i32;
     x4bid: i32;
     x5id: i32;
     x5bid: i32;
     x6id: i32;
     x6bid: i32;
     x7id: i32;
     x7bid: i32;
     xx7id: i32;
     xx7bid: i32;
     a10id: i32;
     a10bid: i32;
     a11id: i32;
     a11bid: i32;
     aa11id: i32;
     aa11bid: i32;
     ab11id: i32;
     ab11bid: i32;
     a12id: i32;
     a12bid: i32;
     a13id: i32;
     a13bid: i32;
     a14id: i32;
     a14bid: i32;
     a15id: i32;
     a15bid: i32;
     a16id: i32;
     a16bid: i32;
     a20id: i32;
     a20bid: i32;
     a17id: i32;
     a17bid: i32;
     a18id: i32;
     a18bid: i32;
     a19id: i32;
     a19bid: i32;
     a21id: i32;
     a21bid: i32;
     v19id: i32;
     v19bid: i32;
     v21id: i32;
     v21bid: i32;
     v22id: i32;
     v22bid: i32;
     v23id: i32;
     v23bid: i32;
     v20id: i32;
     v20bid: i32;
     a22id: i32;
     a22bid: i32;
     a221id: i32;
     a221bid: i32;
     a23id: i32;
     a23bid: i32;
     a24id: i32;
     a24bid: i32;
     a25id: i32;
     a25bid: i32;
     a26id: i32;
     a26bid: i32;
     a27id: i32;
     a27bid: i32;
     a28id: i32;
     a28bid: i32;
     a29id: i32;
     a29bid: i32;
     a30id: i32;
     a30bid: i32;
     a31id: i32;
     a31bid: i32;
     a32id: i32;
     a32bid: i32;
     a320id: i32;
     a320bid: i32;
     a33id: i32;
     a33bid: i32;
     a38id: i32;
     a38bid: i32;
     a34id: i32;
     a34bid: i32;
     a35id: i32;
     a35bid: i32;
     a36id: i32;
     a36bid: i32;
     a37id: i32;
     a37bid: i32;
     a39id: i32;
     a39bid: i32;
     a40id: i32;
     a40bid: i32;
     a440id: i32;
     a440bid: i32;
     a41id: i32;
     a41bid: i32;
     a42id: i32;
     a42bid: i32;
     a43id: i32;
     a43bid: i32;
     a45id: i32;
     a45bid: i32;
     asid: i32;
     asidb: i32;
     a44id: i32;
     HisListId: i32;
     ListClass HisListObj;
     LibListId: i32;
     ListClass LibListObj;
     VarListId: i32;
     ListClass VarListObj;
     DeckListId: i32;
     ListClass deckListObj;
     AutoListId: i32;
     ListClass AutoListObj;
     BAddHisId: i32;
     bUpId: i32;
     bDownId: i32;
     BAddHisTextId: i32;
     SPListId: i32;
     ListClass SPListObj;
     BRemoveHisId: i32;
     BRemoveHisTextId: i32;
     libnr: i32;
     detailnr: i32;
     detailnr2: i32;
     detailnr3: i32;
     detailnr4: i32;
     detailnr5: i32;
     detailnr6: i32;
     DescBox: i32;
     ss: String;

    pub EditHisWindowClass(ref tGame: GameClass)
      : base(ref tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Historical Units (&officers)")
    {
      this.locNr = !(this.game.SelectX > -1 & this.game.SelectY > -1) ? -1 : this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      this.detailnr = -1;
      this.libnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.detailnr4 = -1;
      this.detailnr5 = -1;
      this.MakeHisList();
      this.showinfo();
    }

    pub HandleKeyPress: WindowReturnClass(nr: i32, bool fromTimer = false)
    {
      windowReturnClass1: WindowReturnClass = WindowReturnClass::new();
      switch (nr)
      {
        case 13:
          windowReturnClass2: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.BAddHisId)] + 1, this.SubPartY[this.SubpartNr(this.BAddHisId)] + 1, 1);
          windowReturnClass2.SetFlag(true);
          return windowReturnClass2;
        case 38:
          --this.detailnr;
          if (this.detailnr < 0)
            this.detailnr = 0;
          if (this.game.Data.HistoricalUnitCounter == -1)
            this.detailnr = -1;
          this.showinfo();
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        case 40:
          this += 1.detailnr;
          if (this.detailnr > this.game.Data.HistoricalUnitCounter)
            this.detailnr = this.game.Data.HistoricalUnitCounter;
          this.showinfo();
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        case 77:
          windowReturnClass3: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.x5id)] + 1, this.SubPartY[this.SubpartNr(this.x5id)] + 1, 1);
          windowReturnClass3.SetFlag(true);
          return windowReturnClass3;
        case 83:
          windowReturnClass4: WindowReturnClass = this.HandleMouseClick(this.SubPartX[this.SubpartNr(this.a11id)] + 1, this.SubPartY[this.SubpartNr(this.a11id)] + 1, 1);
          windowReturnClass4.SetFlag(true);
          return windowReturnClass4;
        case 84:
          if (this.detailnr > -1)
          {
            this.game.Data.HistoricalUnitObj[this.detailnr].TempVar3 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Tempvar3")));
            this.game.Data.HistoricalUnitObj[this.detailnr].TempVar4 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Tempvar4")));
            this.game.Data.HistoricalUnitObj[this.detailnr].TempVar5 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Tempvar5")));
          }
          this.showinfo();
          windowReturnClass1.SetFlag(true);
          return windowReturnClass1;
        default:
          return windowReturnClass1;
      }
    }

    pub fn MakeHisList()
    {
      int[] numArray1 = new int[this.game.Data.HistoricalUnitCounter + 1];
      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
      for (let mut index1: i32 =  0; index1 <= unitCounter; index1 += 1)
      {
        if (this.game.Data.UnitObj[index1].Historical > -1)
        {
          int[] numArray2 = numArray1;
          int[] numArray3 = numArray2;
          let mut historical: i32 =  this.game.Data.UnitObj[index1].Historical;
          let mut index2: i32 =  historical;
          let mut num: i32 =  numArray2[historical] + 1;
          numArray3[index2] = num;
        }
      }
      if (this.HisListId > 0)
        this.RemoveSubPart(this.HisListId);
      if (this.BAddHisId > 0)
        this.RemoveSubPart(this.BAddHisId);
      if (this.BAddHisTextId > 0)
        this.RemoveSubPart(this.BAddHisTextId);
      if (this.LibListId > 0)
        this.RemoveSubPart(this.LibListId);
      this.LibListObj = ListClass::new();
      this.LibListObj.add("All", -2);
      let mut num1: i32 =  -1;
      let mut num2: i32 =  0;
      let mut libraryCounter: i32 =  this.game.Data.LibraryCounter;
      for (let mut index: i32 =  0; index <= libraryCounter; index += 1)
      {
        num2 += 1;
        if (this.libnr == index)
          num1 = num2;
        this.LibListObj.add(Conversion.Str( index) + ") " + this.game.Data.LibraryObj[index].name, index);
      }
      if (this.libnr == -1)
        num1 = 0;
      ListClass libListObj = this.LibListObj;
      let mut tlistselect1: i32 =  num1;
      let mut game1: GameClass = this.game;
      ref local1: Bitmap = ref this.OwnBitmap;
      font: Font =  null;
      ref local2: Font = ref font;
      let mut tsubpart1: SubPartClass =  new ListSubPartClass(libListObj, 16, 200, tlistselect1, game1, tHeader: "Libraries", tbackbitmap: (ref local1), bbx: 5, bby: 50, overruleFont: (ref local2));
      this.LibListId = this.AddSubPart(ref tsubpart1, 5, 50, 200, 304, 0);
      if (this.game.Data.HistoricalUnitCounter > -1)
      {
        this.HisListObj = ListClass::new();
        let mut num3: i32 =  -1;
        let mut num4: i32 =  -1;
        let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
        for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
        {
          if (this.game.Data.HistoricalUnitObj[index].LibId.libSlot == this.libnr | this.libnr == -1)
          {
            num4 += 1;
            tvalue: String;
            if (this.game.Data.HistoricalUnitObj[index].Type == 1)
              tvalue = "1.Ind";
            if (this.game.Data.HistoricalUnitObj[index].Type == 2)
              tvalue = "2.Div";
            if (this.game.Data.HistoricalUnitObj[index].Type == 5)
              tvalue = "5.Corps";
            if (this.game.Data.HistoricalUnitObj[index].Type == 6)
              tvalue = "6.Army";
            if (this.game.Data.HistoricalUnitObj[index].Type == 7)
              tvalue = "7.AG";
            if (this.game.Data.HistoricalUnitObj[index].Type == 8)
              tvalue = "8.High";
            tvalue4: String = Strings.Trim(Conversion.Str( numArray1[index]));
            tvalue3: String = Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[index].TempRegime));
            tvalue2: String = Conversion.Str( this.game.Data.HistoricalUnitObj[index].SmallGfx) + "," + Conversion.Str( this.game.Data.HistoricalUnitObj[index].Counter) + "," + this.game.Data.HistoricalUnitObj[index].CounterString;
            if (!this.game.Data.HistoricalUnitObj[index].Model)
            {
              if (this.game.Data.HistoricalUnitObj[index].ModelMaster > -1)
                this.HisListObj.add(Strings.Trim(Conversion.Str( index)) + ") ID=" + Conversion.Str( this.game.Data.HistoricalUnitObj[index].ID) + ", " + this.game.Data.HistoricalUnitObj[index].Name + " (" + this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[index].ModelMaster].Name + ")", index, tvalue, tvalue2, tvalue3, tvalue4);
              else
                this.HisListObj.add(Strings.Trim(Conversion.Str( index)) + ") ID=" + Conversion.Str( this.game.Data.HistoricalUnitObj[index].ID) + ", " + this.game.Data.HistoricalUnitObj[index].Name, index, tvalue, tvalue2, tvalue3, tvalue4);
            }
            else
              this.HisListObj.add(Strings.Trim(Conversion.Str( index)) + ") ID=" + Conversion.Str( this.game.Data.HistoricalUnitObj[index].ID) + ", MODEL " + this.game.Data.HistoricalUnitObj[index].Name, index, tvalue, tvalue2, tvalue3, tvalue4);
            if (this.detailnr == index)
              num3 = num4;
          }
        }
        if (num3 == -1)
          this.detailnr = -1;
        ListClass hisListObj = this.HisListObj;
        let mut tlistselect2: i32 =  num3;
        let mut game2: GameClass = this.game;
        ref local3: Bitmap = ref this.OwnBitmap;
        font =  null;
        ref local4: Font = ref font;
        let mut tsubpart2: SubPartClass =  new ListSubPartClass(hisListObj, 16, 580, tlistselect2, game2, tHeader: "HisUnits ID+Name                                                                    Level           SmlGf,Nato,Sh        Regime      Units", tShowPair: true, tValueWidth: 250, tbackbitmap: (ref local3), bbx: 215, bby: 50, overruleFont: (ref local4));
        this.HisListId = this.AddSubPart(ref tsubpart2, 215, 50, 580, 304, 0);
      }
      if (this.BAddHisId > 0)
        this.RemoveSubPart(this.BAddHisId);
      if (this.BAddHisTextId > 0)
        this.RemoveSubPart(this.BAddHisTextId);
      this.ss = "Click to add another Historical Unit";
      let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
      this.BAddHisId = this.AddSubPart(ref tsubpart3, 830, 50, 32, 16, 1);
      let mut tsubpart4: SubPartClass =  TextPartClass::new("Add Historic Unit", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.BAddHisTextId = this.AddSubPart(ref tsubpart4, 880, 50, 300, 20, 0);
    }

    pub fn DoRefresh()
    {
      this.MakeHisList();
      this.showinfo();
    }

     void showinfo()
    {
      if (this.bUpId > 0)
        this.RemoveSubPart(this.bUpId);
      if (this.bDownId > 0)
        this.RemoveSubPart(this.bDownId);
      if (this.impid > 0)
        this.RemoveSubPart(this.impid);
      if (this.expid > 0)
        this.RemoveSubPart(this.expid);
      if (this.BRemoveHisId > 0)
        this.RemoveSubPart(this.BRemoveHisId);
      if (this.BRemoveHisTextId > 0)
        this.RemoveSubPart(this.BRemoveHisTextId);
      if (this.BNameId > 0)
        this.RemoveSubPart(this.BNameId);
      if (this.BNameTextId > 0)
        this.RemoveSubPart(this.BNameTextId);
      if (this.BPopId > 0)
        this.RemoveSubPart(this.BPopId);
      if (this.BPopTextId > 0)
        this.RemoveSubPart(this.BPopTextId);
      if (this.BPplId > 0)
        this.RemoveSubPart(this.BPplId);
      if (this.BPplTextId > 0)
        this.RemoveSubPart(this.BPplTextId);
      if (this.BRemoveId > 0)
        this.RemoveSubPart(this.BRemoveId);
      if (this.BRemoveTextid > 0)
        this.RemoveSubPart(this.BRemoveTextid);
      if (this.DescBox > 0)
        this.RemoveSubPart(this.DescBox);
      if (this.AutoListId > 0)
        this.RemoveSubPart(this.AutoListId);
      if (this.DeckListId > 0)
        this.RemoveSubPart(this.DeckListId);
      if (this.VarListId > 0)
        this.RemoveSubPart(this.VarListId);
      if (this.SPListId > 0)
        this.RemoveSubPart(this.SPListId);
      if (this.x1id > 0)
        this.RemoveSubPart(this.x1id);
      if (this.x1bid > 0)
        this.RemoveSubPart(this.x1bid);
      if (this.x2id > 0)
        this.RemoveSubPart(this.x2id);
      if (this.x2bid > 0)
        this.RemoveSubPart(this.x2bid);
      if (this.x3id > 0)
        this.RemoveSubPart(this.x3id);
      if (this.x3bid > 0)
        this.RemoveSubPart(this.x3bid);
      if (this.x4id > 0)
        this.RemoveSubPart(this.x4id);
      if (this.x4bid > 0)
        this.RemoveSubPart(this.x4bid);
      if (this.x5id > 0)
        this.RemoveSubPart(this.x5id);
      if (this.x5bid > 0)
        this.RemoveSubPart(this.x5bid);
      if (this.x6id > 0)
        this.RemoveSubPart(this.x6id);
      if (this.x6bid > 0)
        this.RemoveSubPart(this.x6bid);
      if (this.x7id > 0)
        this.RemoveSubPart(this.x7id);
      if (this.x7bid > 0)
        this.RemoveSubPart(this.x7bid);
      if (this.xx7id > 0)
        this.RemoveSubPart(this.xx7id);
      if (this.xx7bid > 0)
        this.RemoveSubPart(this.xx7bid);
      if (this.apoolid > 0)
        this.RemoveSubPart(this.apoolid);
      if (this.apoolbid > 0)
        this.RemoveSubPart(this.apoolbid);
      if (this.a1id > 0)
        this.RemoveSubPart(this.a1id);
      if (this.a1bid > 0)
        this.RemoveSubPart(this.a1bid);
      if (this.a2id > 0)
        this.RemoveSubPart(this.a2id);
      if (this.a2bid > 0)
        this.RemoveSubPart(this.a2bid);
      if (this.a3id > 0)
        this.RemoveSubPart(this.a3id);
      if (this.a3bid > 0)
        this.RemoveSubPart(this.a3bid);
      if (this.a4id > 0)
        this.RemoveSubPart(this.a4id);
      if (this.a4bid > 0)
        this.RemoveSubPart(this.a4bid);
      if (this.a5id > 0)
        this.RemoveSubPart(this.a5id);
      if (this.a5bid > 0)
        this.RemoveSubPart(this.a5bid);
      if (this.a6id > 0)
        this.RemoveSubPart(this.a6id);
      if (this.a6bid > 0)
        this.RemoveSubPart(this.a6bid);
      if (this.a7id > 0)
        this.RemoveSubPart(this.a7id);
      if (this.a7bid > 0)
        this.RemoveSubPart(this.a7bid);
      if (this.x7id > 0)
        this.RemoveSubPart(this.x7id);
      if (this.x7bid > 0)
        this.RemoveSubPart(this.x7bid);
      if (this.a8id > 0)
        this.RemoveSubPart(this.a8id);
      if (this.a8bid > 0)
        this.RemoveSubPart(this.a8bid);
      if (this.a9id > 0)
        this.RemoveSubPart(this.a9id);
      if (this.a9bid > 0)
        this.RemoveSubPart(this.a9bid);
      if (this.a10id > 0)
        this.RemoveSubPart(this.a10id);
      if (this.a10bid > 0)
        this.RemoveSubPart(this.a10bid);
      if (this.a12id > 0)
        this.RemoveSubPart(this.a12id);
      if (this.a12bid > 0)
        this.RemoveSubPart(this.a12bid);
      if (this.a11id > 0)
        this.RemoveSubPart(this.a11id);
      if (this.a11bid > 0)
        this.RemoveSubPart(this.a11bid);
      if (this.aa11id > 0)
        this.RemoveSubPart(this.aa11id);
      if (this.aa11bid > 0)
        this.RemoveSubPart(this.aa11bid);
      if (this.ab11id > 0)
        this.RemoveSubPart(this.ab11id);
      if (this.ab11bid > 0)
        this.RemoveSubPart(this.ab11bid);
      if (this.a13id > 0)
        this.RemoveSubPart(this.a13id);
      if (this.a13bid > 0)
        this.RemoveSubPart(this.a13bid);
      if (this.a14id > 0)
        this.RemoveSubPart(this.a14id);
      if (this.a14bid > 0)
        this.RemoveSubPart(this.a14bid);
      if (this.a15id > 0)
        this.RemoveSubPart(this.a15id);
      if (this.a15bid > 0)
        this.RemoveSubPart(this.a15bid);
      if (this.a16id > 0)
        this.RemoveSubPart(this.a16id);
      if (this.a16bid > 0)
        this.RemoveSubPart(this.a16bid);
      if (this.a17id > 0)
        this.RemoveSubPart(this.a17id);
      if (this.a17bid > 0)
        this.RemoveSubPart(this.a17bid);
      if (this.a18id > 0)
        this.RemoveSubPart(this.a18id);
      if (this.a44id > 0)
        this.RemoveSubPart(this.a44id);
      if (this.a18bid > 0)
        this.RemoveSubPart(this.a18bid);
      if (this.a19id > 0)
        this.RemoveSubPart(this.a19id);
      if (this.a19bid > 0)
        this.RemoveSubPart(this.a19bid);
      if (this.a20id > 0)
        this.RemoveSubPart(this.a20id);
      if (this.a20bid > 0)
        this.RemoveSubPart(this.a20bid);
      if (this.a21id > 0)
        this.RemoveSubPart(this.a21id);
      if (this.a21bid > 0)
        this.RemoveSubPart(this.a21bid);
      if (this.v19id > 0)
        this.RemoveSubPart(this.v19id);
      if (this.v19bid > 0)
        this.RemoveSubPart(this.v19bid);
      if (this.v20id > 0)
        this.RemoveSubPart(this.v20id);
      if (this.v20bid > 0)
        this.RemoveSubPart(this.v20bid);
      if (this.v21id > 0)
        this.RemoveSubPart(this.v21id);
      if (this.v21bid > 0)
        this.RemoveSubPart(this.v21bid);
      if (this.v22id > 0)
        this.RemoveSubPart(this.v22id);
      if (this.v22bid > 0)
        this.RemoveSubPart(this.v22bid);
      if (this.v23id > 0)
        this.RemoveSubPart(this.v23id);
      if (this.v23bid > 0)
        this.RemoveSubPart(this.v23bid);
      if (this.a22id > 0)
        this.RemoveSubPart(this.a22id);
      if (this.a22bid > 0)
        this.RemoveSubPart(this.a22bid);
      if (this.a221id > 0)
        this.RemoveSubPart(this.a221id);
      if (this.a221bid > 0)
        this.RemoveSubPart(this.a221bid);
      if (this.a23id > 0)
        this.RemoveSubPart(this.a23id);
      if (this.a23bid > 0)
        this.RemoveSubPart(this.a23bid);
      if (this.a24id > 0)
        this.RemoveSubPart(this.a24id);
      if (this.a24bid > 0)
        this.RemoveSubPart(this.a24bid);
      if (this.a25id > 0)
        this.RemoveSubPart(this.a25id);
      if (this.a25bid > 0)
        this.RemoveSubPart(this.a25bid);
      if (this.a26id > 0)
        this.RemoveSubPart(this.a26id);
      if (this.a26bid > 0)
        this.RemoveSubPart(this.a26bid);
      if (this.a27id > 0)
        this.RemoveSubPart(this.a27id);
      if (this.a27bid > 0)
        this.RemoveSubPart(this.a27bid);
      if (this.a28id > 0)
        this.RemoveSubPart(this.a28id);
      if (this.a28bid > 0)
        this.RemoveSubPart(this.a28bid);
      if (this.a29id > 0)
        this.RemoveSubPart(this.a29id);
      if (this.a29bid > 0)
        this.RemoveSubPart(this.a29bid);
      if (this.a30id > 0)
        this.RemoveSubPart(this.a30id);
      if (this.a30bid > 0)
        this.RemoveSubPart(this.a30bid);
      if (this.a31id > 0)
        this.RemoveSubPart(this.a31id);
      if (this.a31bid > 0)
        this.RemoveSubPart(this.a31bid);
      if (this.a32id > 0)
        this.RemoveSubPart(this.a32id);
      if (this.a320id > 0)
        this.RemoveSubPart(this.a320id);
      if (this.a32bid > 0)
        this.RemoveSubPart(this.a32bid);
      if (this.a320bid > 0)
        this.RemoveSubPart(this.a320bid);
      if (this.a33id > 0)
        this.RemoveSubPart(this.a33id);
      if (this.a33bid > 0)
        this.RemoveSubPart(this.a33bid);
      if (this.a34id > 0)
        this.RemoveSubPart(this.a34id);
      if (this.a34bid > 0)
        this.RemoveSubPart(this.a34bid);
      if (this.a35id > 0)
        this.RemoveSubPart(this.a35id);
      if (this.a35bid > 0)
        this.RemoveSubPart(this.a35bid);
      if (this.a36id > 0)
        this.RemoveSubPart(this.a36id);
      if (this.a36bid > 0)
        this.RemoveSubPart(this.a36bid);
      if (this.a37id > 0)
        this.RemoveSubPart(this.a37id);
      if (this.a37bid > 0)
        this.RemoveSubPart(this.a37bid);
      if (this.a38id > 0)
        this.RemoveSubPart(this.a38id);
      if (this.a38bid > 0)
        this.RemoveSubPart(this.a38bid);
      if (this.a39id > 0)
        this.RemoveSubPart(this.a39id);
      if (this.a39bid > 0)
        this.RemoveSubPart(this.a39bid);
      if (this.a40id > 0)
        this.RemoveSubPart(this.a40id);
      if (this.a40bid > 0)
        this.RemoveSubPart(this.a40bid);
      if (this.a440id > 0)
        this.RemoveSubPart(this.a440id);
      if (this.a440bid > 0)
        this.RemoveSubPart(this.a440bid);
      if (this.a41id > 0)
        this.RemoveSubPart(this.a41id);
      if (this.a41bid > 0)
        this.RemoveSubPart(this.a41bid);
      if (this.a42id > 0)
        this.RemoveSubPart(this.a42id);
      if (this.a42bid > 0)
        this.RemoveSubPart(this.a42bid);
      if (this.a43id > 0)
        this.RemoveSubPart(this.a43id);
      if (this.a43bid > 0)
        this.RemoveSubPart(this.a43bid);
      if (this.a45id > 0)
        this.RemoveSubPart(this.a45id);
      if (this.a45bid > 0)
        this.RemoveSubPart(this.a45bid);
      if (this.asid > 0)
        this.RemoveSubPart(this.asid);
      if (this.asidb > 0)
        this.RemoveSubPart(this.asidb);
      this.MakeHisList();
      if (this.detailnr > -1)
      {
        this.ss = "Change Shortname (# or short name displayed on unit counter)";
        let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a11id = this.AddSubPart(ref tsubpart1, 830, 100, 32, 16, 1);
        let mut tsubpart2: SubPartClass =  TextPartClass::new("Sh", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 20, false, tDescript: this.ss);
        this.a11bid = this.AddSubPart(ref tsubpart2, 865, 100, 30, 20, 0);
        this.ss = "Just change the nato counter (you can find these graphics and their number in the systemgraphics/nato directory)";
        let mut tsubpart3: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.aa11id = this.AddSubPart(ref tsubpart3, 895, 100, 32, 16, 1);
        let mut tsubpart4: SubPartClass =  TextPartClass::new("Nat", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 20, false, tDescript: this.ss);
        this.aa11bid = this.AddSubPart(ref tsubpart4, 930, 100, 30, 20, 0);
        this.ss = "Change Type";
        let mut tsubpart5: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a12id = this.AddSubPart(ref tsubpart5, 960, 100, 32, 16, 1);
        let mut tsubpart6: SubPartClass =  TextPartClass::new("Typ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 40, 20, false, tDescript: this.ss);
        this.a12bid = this.AddSubPart(ref tsubpart6, 995, 99, 40, 20, 0);
        this.ss = "Just change the small graphic used (instead of nato)";
        let mut tsubpart7: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.ab11id = this.AddSubPart(ref tsubpart7, 1030, 100, 32, 16, 1);
        let mut tsubpart8: SubPartClass =  TextPartClass::new("Sml", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 30, 20, false, tDescript: this.ss);
        this.ab11bid = this.AddSubPart(ref tsubpart8, 1065, 100, 30, 20, 0);
      }
      if (this.detailnr > -1)
      {
        this.ss = "Move HisUnit Up (just for sorting purposes)";
        if (this.detailnr > 0)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONUP, tDescript: this.ss);
          this.bUpId = this.AddSubPart(ref tsubpart, 830, 80, 32, 16, 1);
        }
        this.ss = "Move HisUnit Down (just for sorting purposes)";
        if (this.detailnr < this.game.Data.HistoricalUnitCounter)
        {
          let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONDOWN, tDescript: this.ss);
          this.bDownId = this.AddSubPart(ref tsubpart, 865, 80, 32, 16, 1);
        }
        this.ss = "Remove Historical Unit";
        let mut tsubpart9: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.BRemoveHisId = this.AddSubPart(ref tsubpart9, 900, 80, 32, 16, 1);
        this.ss = "Export NATO (+names as reference)";
        let mut tsubpart10: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.expid = this.AddSubPart(ref tsubpart10, 935, 80, 32, 16, 1);
        this.ss = "Import NATO (only number before first ',')";
        let mut tsubpart11: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLUE, tDescript: this.ss);
        this.impid = this.AddSubPart(ref tsubpart11, 970, 80, 32, 16, 1);
        this.ss = "Assign to library";
        let mut tsubpart12: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.asid = this.AddSubPart(ref tsubpart12, 1005, 80, 32, 16, 1);
      }
      this.ss = "Clear All Historical Units";
      let mut tsubpart13: SubPartClass =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.a13id = this.AddSubPart(ref tsubpart13, 830, 120, 32, 16, 1);
      let mut tsubpart14: SubPartClass =  TextPartClass::new("ClearAll", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a13bid = this.AddSubPart(ref tsubpart14, 860, 119, 60, 20, 0);
      this.ss = "Auto-Create Historical Units";
      let mut tsubpart15: SubPartClass =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.a14id = this.AddSubPart(ref tsubpart15, 930, 120, 32, 16, 1);
      let mut tsubpart16: SubPartClass =  TextPartClass::new("Auto-Create", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a14bid = this.AddSubPart(ref tsubpart16, 960, 119, 300, 20, 0);
      this.ss = "Sort Historical units 1st on regime, then on Level in OOB";
      let mut tsubpart17: SubPartClass =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.a15id = this.AddSubPart(ref tsubpart17, 830, 140, 32, 16, 1);
      let mut tsubpart18: SubPartClass =  TextPartClass::new("Sort", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a15bid = this.AddSubPart(ref tsubpart18, 860, 139, 60, 20, 0);
      this.ss = "Set regime of historical unit";
      let mut tsubpart19: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a16id = this.AddSubPart(ref tsubpart19, 930, 140, 32, 16, 1);
      let mut tsubpart20: SubPartClass =  TextPartClass::new("Regime", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a16bid = this.AddSubPart(ref tsubpart20, 960, 139, 300, 20, 0);
      str1: String = "";
      SubPartClass tsubpart21;
      if (this.detailnr > -1)
      {
        let mut unitCounter: i32 =  this.game.Data.UnitCounter;
        for (let mut index: i32 =  0; index <= unitCounter; index += 1)
        {
          if (this.game.Data.UnitObj[index].Historical == this.detailnr)
            str1 = str1 + this.game.Data.UnitObj[index].Name + "(" + Conversion.Str( this.game.Data.UnitObj[index].X) + "," + Conversion.Str( this.game.Data.UnitObj[index].Y) + "), reg = " + Conversion.Str( this.game.Data.UnitObj[index].Regime) + "\r\n";
        }
        let mut game: GameClass = this.game;
        let mut twidth: i32 =  Math.Min(300, this.game.ScreenWidth - 830);
        tfont: Font = Font::new(this.game.FontCol.Families[1], 10f, FontStyle.Regular, GraphicsUnit.Pixel);
        tText: String = str1;
        white: Color = Color.White;
        bitmap: Bitmap = (Bitmap) null;
        ref local: Bitmap = ref bitmap;
        tsubpart21 =  new TextAreaClass(game, twidth, 3, tfont, "Map units attached", true, tText, white, tbackbitmap: (ref local));
        this.DescBox = this.AddSubPart(ref tsubpart21, 830, 160, Math.Min(300, this.game.ScreenWidth - 830), 48, 0);
      }
      if (this.detailnr > -1)
      {
        if (this.game.Data.HistoricalUnitObj[this.detailnr].SmallGfx > -1)
        {
          tsubpart21 =  ButtonPartClass::new(this.game.Data.SmallPicNr[this.game.Data.HistoricalUnitObj[this.detailnr].SmallGfx], tDescript: "the NATO graphic of the historical unit", tResizeX: 38, tresizeY: 38);
          this.a44id = this.AddSubPart(ref tsubpart21, 20, 345, 38, 38, 1);
        }
        else if (this.game.Data.HistoricalUnitObj[this.detailnr].Counter > 0 && this.game.Data.HistoricalUnitObj[this.detailnr].Counter > -1)
        {
          tsubpart21 =  ButtonPartClass::new(this.game.NATO[this.game.Data.HistoricalUnitObj[this.detailnr].Counter], tDescript: "the NATO graphic of the historical unit", tResizeX: 38, tresizeY: 38);
          this.a44id = this.AddSubPart(ref tsubpart21, 20, 345, 38, 38, 1);
        }
        this.ss = "Click to change the people of this historical unit. (important for officers and HQs)";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.x7id = this.AddSubPart(ref tsubpart21, 10, 420, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("People = " + this.game.Data.HistoricalUnitObj[this.detailnr].People.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: this.ss);
        this.x7bid = this.AddSubPart(ref tsubpart21, 50, 419, 150, 20, 0);
        this.ss = "Click to change Gfx People overrule use (only graphical not rule implications)";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.xx7id = this.AddSubPart(ref tsubpart21, 220, 420, 32, 16, 1);
        if (this.game.Data.HistoricalUnitObj[this.detailnr].UsePeopleGfx > -1)
        {
          tsubpart21 =  TextPartClass::new("GfxPeopleUse = " + this.game.Data.HistoricalUnitObj[this.detailnr].UsePeopleGfx.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: this.ss);
          this.xx7bid = this.AddSubPart(ref tsubpart21, 250, 419, 150, 20, 0);
        }
        else
        {
          tsubpart21 =  TextPartClass::new("GfxPeopleUse = -1", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: this.ss);
          this.xx7bid = this.AddSubPart(ref tsubpart21, 250, 419, 150, 20, 0);
        }
        this.ss = "Click to set if AI will be allowed to split this group (only for HQs)";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a39id = this.AddSubPart(ref tsubpart21, 10, 440, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("NoSplit = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].NoSplit), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: this.ss);
        this.a39bid = this.AddSubPart(ref tsubpart21, 50, 440, 100, 20, 0);
        this.ss = "Click to change name of the historical unit selected...";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a25id = this.AddSubPart(ref tsubpart21, 10, 460, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Name = " + this.game.Data.HistoricalUnitObj[this.detailnr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 350, 20, false, tDescript: this.ss);
        this.a25bid = this.AddSubPart(ref tsubpart21, 50, 459, 350, 20, 0);
        this.ss = "Click to set as part of an AI model list.";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a440id = this.AddSubPart(ref tsubpart21, 610, 660, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("AIZoneListNr = " + this.game.Data.HistoricalUnitObj[this.detailnr].AIlist.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a440bid = this.AddSubPart(ref tsubpart21, 650, 659, 150, 20, 0);
        this.ss = "Set if Historical Unit is a Model Unit";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.x1id = this.AddSubPart(ref tsubpart21, 10, 480, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Model=" + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Model), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: this.ss);
        this.x1bid = this.AddSubPart(ref tsubpart21, 50, 479, 100, 20, 0);
        this.ss = "Set the maximum number of units with this model to be present. -1=unlimited. 0=none can be created in game by player";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a41id = this.AddSubPart(ref tsubpart21, 150, 480, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("MaxPresent=" + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].MaxPresent), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 120, 20, false, tDescript: this.ss);
        this.a41bid = this.AddSubPart(ref tsubpart21, 190, 479, 120, 20, 0);
        this.ss = "Set blender: Color -255 to +255, it adds or diminishes from the counter color";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.x2id = this.AddSubPart(ref tsubpart21, 410, 540, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Color = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Red) + "," + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Green) + "," + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Blue), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.x2bid = this.AddSubPart(ref tsubpart21, 450, 539, 200, 20, 0);
        this.SPListObj = ListClass::new();
        let mut index: i32 =  0;
        do
        {
          str2: String = Conversion.Str( index) + ") ";
          this.SPListObj.add(this.game.Data.HistoricalUnitObj[this.detailnr].SubParts[index] <= -1 ? (this.game.Data.HistoricalUnitObj[this.detailnr].Designation[index] <= -1 ? str2 + " -" : str2 + "No Predef" + ", " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].DesignationSmall[index]) + ", " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Designation[index])) : str2 + this.game.Data.UnitObj[this.game.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[this.detailnr].SubParts[index])].Name + ", " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].DesignationSmall[index]) + ", " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Designation[index]), index);
          index += 1;
        }
        while (index <= 9);
        ListClass spListObj = this.SPListObj;
        let mut detailnr5: i32 =  this.detailnr5;
        let mut game1: GameClass = this.game;
        bitmap1: Bitmap = (Bitmap) null;
        ref local1: Bitmap = ref bitmap1;
        font1: Font =  null;
        ref local2: Font = ref font1;
        tsubpart21 =  new ListSubPartClass(spListObj, 3, 400, detailnr5, game1, tHeader: "Subpart Units             smallGfx,Nato", tbackbitmap: (ref local1), overruleFont: (ref local2));
        this.SPListId = this.AddSubPart(ref tsubpart21, 410, 360, 400, 80, 0);
        if (this.detailnr5 > -1)
        {
          this.ss = "Remove Subpart";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.x3id = this.AddSubPart(ref tsubpart21, 410, 460, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Remove Subpart", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.x3bid = this.AddSubPart(ref tsubpart21, 440, 459, 100, 20, 0);
          this.ss = "Set Subpart";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.x4id = this.AddSubPart(ref tsubpart21, 570, 460, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Set Subpart", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.x4bid = this.AddSubPart(ref tsubpart21, 600, 459, 100, 20, 0);
        }
        if (!this.game.Data.HistoricalUnitObj[this.detailnr].Model)
        {
          this.ss = "Set this Unit to a specific Model";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.x5id = this.AddSubPart(ref tsubpart21, 10, 400, 32, 16, 1);
          if (this.game.Data.HistoricalUnitObj[this.detailnr].ModelMaster > -1)
          {
            tsubpart21 =  TextPartClass::new("His.Units Model: " + this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitObj[this.detailnr].ModelMaster].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
            this.x5bid = this.AddSubPart(ref tsubpart21, 50, 399, 300, 20, 0);
          }
          else
          {
            tsubpart21 =  TextPartClass::new("His.Units Model: Not set!", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
            this.x5bid = this.AddSubPart(ref tsubpart21, 50, 399, 300, 20, 0);
          }
        }
        this.VarListObj = ListClass::new();
        let mut hisVarCount: i32 =  this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount;
        for (let mut tdata: i32 =  0; tdata <= hisVarCount; tdata += 1)
        {
          tname: String;
          if (this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall[tdata] > -1)
            tname = "Type " + this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType[tdata].ToString() + " = " + this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue[tdata].ToString() + " (smallgfx" + this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall[tdata].ToString() + ")";
          else
            tname = "Type " + this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType[tdata].ToString() + " = " + this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue[tdata].ToString() + " (nato" + this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato[tdata].ToString() + ")";
          this.VarListObj.add(tname, tdata);
        }
        ListClass varListObj = this.VarListObj;
        let mut detailnr6: i32 =  this.detailnr6;
        let mut game2: GameClass = this.game;
        bitmap2: Bitmap = (Bitmap) null;
        ref local3: Bitmap = ref bitmap2;
        font2: Font =  null;
        ref local4: Font = ref font2;
        tsubpart21 =  new ListSubPartClass(varListObj, 3, 300, detailnr6, game2, tHeader: "HisVar List", tbackbitmap: (ref local3), overruleFont: (ref local4));
        this.VarListId = this.AddSubPart(ref tsubpart21, 50, 504, 300, 80, 0);
        this.ss = "Add hisvar type+value";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.v19id = this.AddSubPart(ref tsubpart21, 50, 592, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Add", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 60, 20, false, tDescript: this.ss);
        this.v19bid = this.AddSubPart(ref tsubpart21, 90, 591, 60, 20, 0);
        if (this.detailnr6 > -1)
        {
          this.ss = "Remove this hisvar type + value";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.v20id = this.AddSubPart(ref tsubpart21, 190, 592, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Remove", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: this.ss);
          this.v20bid = this.AddSubPart(ref tsubpart21, 230, 591, 100, 20, 0);
          this.ss = "Change hisvar Type + Value";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.v21id = this.AddSubPart(ref tsubpart21, 50, 612, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Change all", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 120, 20, false, tDescript: this.ss);
          this.v21bid = this.AddSubPart(ref tsubpart21, 90, 611, 120, 20, 0);
          this.ss = "Change only value";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.v22id = this.AddSubPart(ref tsubpart21, 190, 612, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Change val", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: this.ss);
          this.v22bid = this.AddSubPart(ref tsubpart21, 230, 611, 100, 20, 0);
          this.ss = "Copy stats from different historical unit";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
          this.v23id = this.AddSubPart(ref tsubpart21, 50, 632, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Copy", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: this.ss);
          this.v23bid = this.AddSubPart(ref tsubpart21, 90, 631, 100, 20, 0);
        }
        this.ss = "Click to change the political cost of the commander or of the Unit";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a26id = this.AddSubPart(ref tsubpart21, 410, 500, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("PP = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].PP), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a26bid = this.AddSubPart(ref tsubpart21, 450, 499, 100, 20, 0);
        this.ss = "Click to set the combatmod of the general (0=none, -100 - +999)";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a27id = this.AddSubPart(ref tsubpart21, 570, 500, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("CombatMod = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].CombatMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 200, 20, false, tDescript: this.ss);
        this.a27bid = this.AddSubPart(ref tsubpart21, 600, 499, 200, 20, 0);
        this.ss = "Click to set the moralemod of the general (0=none, -100 - +999)";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a28id = this.AddSubPart(ref tsubpart21, 410, 520, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("MorMod = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].MoraleMod), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a28bid = this.AddSubPart(ref tsubpart21, 450, 519, 100, 20, 0);
        this.ss = "Click to set the staff pts size the general can maximum command and apply its combat and morale mods too (0=none - 99999)";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a29id = this.AddSubPart(ref tsubpart21, 570, 520, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Staff= " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].StaffSize), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a29bid = this.AddSubPart(ref tsubpart21, 600, 519, 100, 20, 0);
        this.ss = "Click to set the all Historical Units with Model specified exactly as their Models";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
        this.a30id = this.AddSubPart(ref tsubpart21, 830, 240, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Set ALL to their MODEL ", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a30bid = this.AddSubPart(ref tsubpart21, 860, 239, 300, 20, 0);
        this.ss = "Click to set the this hist.unit to Fixed";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a31id = this.AddSubPart(ref tsubpart21, 410, 560, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Fixed =" + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Fixed), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a31bid = this.AddSubPart(ref tsubpart21, 450, 560, 100, 20, 0);
        this.ss = "Click to set the name Counter of this model to that of another model or historical unit.";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a320id = this.AddSubPart(ref tsubpart21, 570, 540, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("UseModelCounter= " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].UseModelCounter), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a320bid = this.AddSubPart(ref tsubpart21, 600, 540, 200, 20, 0);
        this.ss = "Click to set the numeric name counter, chance to use an older number not used at moment, +if to use romans or not";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a32id = this.AddSubPart(ref tsubpart21, 570, 560, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Counter= " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].NameCounter) + " O= " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].PercentOldName) + "%, R=" + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].UseRomans), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a32bid = this.AddSubPart(ref tsubpart21, 600, 560, 200, 20, 0);
        this.ss = "Click to set tempvar1";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a34id = this.AddSubPart(ref tsubpart21, 410, 580, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("tv1 = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].TempVar1), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a34bid = this.AddSubPart(ref tsubpart21, 450, 579, 100, 20, 0);
        this.ss = "Click to set tempvar2";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a35id = this.AddSubPart(ref tsubpart21, 570, 580, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("tv2 = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].TempVar2), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a35bid = this.AddSubPart(ref tsubpart21, 600, 579, 100, 20, 0);
        this.ss = "Click to set tempvar3";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a36id = this.AddSubPart(ref tsubpart21, 410, 600, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("tv3 = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].TempVar3), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a36bid = this.AddSubPart(ref tsubpart21, 450, 599, 100, 20, 0);
        this.ss = "Click to set the numeric name counter";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a37id = this.AddSubPart(ref tsubpart21, 570, 600, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("XP = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].Xp), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a37bid = this.AddSubPart(ref tsubpart21, 600, 599, 100, 20, 0);
        this.ss = "Click to set description";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a38id = this.AddSubPart(ref tsubpart21, 410, 480, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Descr=" + this.game.Data.HistoricalUnitObj[this.detailnr].Descript, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 100, 20, false, tDescript: this.ss);
        this.a38bid = this.AddSubPart(ref tsubpart21, 450, 480, 100, 20, 0);
        this.ss = "Click to set officer to pool - make sure it is not a historical unit assigned to an actual unit!";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.apoolid = this.AddSubPart(ref tsubpart21, 570, 480, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Pool=" + this.game.Data.HistoricalUnitObj[this.detailnr].Pool.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 150, 20, false, tDescript: this.ss);
        this.apoolbid = this.AddSubPart(ref tsubpart21, 600, 480, 150, 20, 0);
        this.ss = "Click to set tempvar4";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a42id = this.AddSubPart(ref tsubpart21, 410, 620, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("tv4 = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].TempVar4), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a42bid = this.AddSubPart(ref tsubpart21, 450, 619, 100, 20, 0);
        this.ss = "Click to set tempvar5";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a43id = this.AddSubPart(ref tsubpart21, 570, 620, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("tv5 = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].TempVar5), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a43bid = this.AddSubPart(ref tsubpart21, 600, 619, 100, 20, 0);
        this.ss = "Click to set Battlegroup flag.. 0=no, 1=yes";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a45id = this.AddSubPart(ref tsubpart21, 570, 640, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Battlegroup = " + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].BattleGroup), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a45bid = this.AddSubPart(ref tsubpart21, 600, 639, 100, 20, 0);
        if (this.game.Data.HistoricalUnitObj[this.detailnr].LibId.libSlot > -1)
        {
          tsubpart21 =  TextPartClass::new("Lib.Slot: " + this.game.Data.HistoricalUnitObj[this.detailnr].LibId.libSlot.ToString() + ",ID: " + this.game.Data.HistoricalUnitObj[this.detailnr].LibId.id.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 410, 660, false, tDescript: this.ss);
          this.a40bid = this.AddSubPart(ref tsubpart21, 410, 659, 350, 20, 0);
        }
        this.ss = "Set name of commander. No name means no commander.";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.a17id = this.AddSubPart(ref tsubpart21, 830, 265, 32, 16, 1);
        if (Operators.CompareString(this.game.Data.HistoricalUnitObj[this.detailnr].CommanderName, "", false) == 0)
        {
          tsubpart21 =  TextPartClass::new("- No Commander -", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a17bid = this.AddSubPart(ref tsubpart21, 880, 264, 300, 20, 0);
        }
        else
        {
          tsubpart21 =  TextPartClass::new(this.game.Data.HistoricalUnitObj[this.detailnr].CommanderName, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a17bid = this.AddSubPart(ref tsubpart21, 880, 264, 300, 20, 0);
        }
        this.ss = "Set sprite for commander. No filename means no sprite..";
        if (Operators.CompareString(this.game.Data.HistoricalUnitObj[this.detailnr].CommanderFileName, "", false) == 0)
        {
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.a18id = this.AddSubPart(ref tsubpart21, 830, 285, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("- No Sprite -", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a18bid = this.AddSubPart(ref tsubpart21, 880, 284, 300, 20, 0);
        }
        else
        {
          tsubpart21 =  ButtonPartClass::new(this.game.Data.HistoricalUnitObj[this.detailnr].CommanderSpriteID, tDescript: this.ss, tResizeX: 32, tresizeY: 32);
          this.a18id = this.AddSubPart(ref tsubpart21, 830, 285, 32, 32, 1);
          tsubpart21 =  TextPartClass::new(this.game.Data.HistoricalUnitObj[this.detailnr].CommanderFileName, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a18bid = this.AddSubPart(ref tsubpart21, 880, 284, 300, 20, 0);
        }
        this.ss = "Set sprite for commander. No filename means no sprite..";
        if (Operators.CompareString(this.game.Data.HistoricalUnitObj[this.detailnr].OverdrawFileName, "", false) == 0)
        {
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.a33id = this.AddSubPart(ref tsubpart21, 830, 325, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("- No Sprite -", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a33bid = this.AddSubPart(ref tsubpart21, 880, 324, 300, 20, 0);
        }
        else
        {
          tsubpart21 =  ButtonPartClass::new(this.game.Data.HistoricalUnitObj[this.detailnr].OverdrawSpriteID, tDescript: this.ss, tResizeX: 32, tresizeY: 32);
          this.a33id = this.AddSubPart(ref tsubpart21, 830, 325, 32, 32, 1);
          tsubpart21 =  TextPartClass::new(this.game.Data.HistoricalUnitObj[this.detailnr].OverdrawFileName, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a33bid = this.AddSubPart(ref tsubpart21, 880, 324, 300, 20, 0);
        }
        this.deckListObj = ListClass::new();
        let mut deckCardCounter: i32 =  this.game.Data.HistoricalUnitObj[this.detailnr].DeckCardCounter;
        for (let mut tdata: i32 =  0; tdata <= deckCardCounter; tdata += 1)
          this.deckListObj.add(Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].DeckChance[tdata])) + "% on " + this.game.Data.ActionCardObj[this.game.Data.HistoricalUnitObj[this.detailnr].DeckCard[tdata]].Title + " (#" + Conversion.Str( this.game.Data.HistoricalUnitObj[this.detailnr].DeckCard[tdata]) + ")", tdata);
        ListClass deckListObj = this.deckListObj;
        let mut twidth: i32 =  Math.Min(300, this.game.ScreenWidth - 830);
        let mut detailnr4: i32 =  this.detailnr4;
        let mut game3: GameClass = this.game;
        bitmap3: Bitmap = (Bitmap) null;
        ref local5: Bitmap = ref bitmap3;
        font3: Font =  null;
        ref local6: Font = ref font3;
        tsubpart21 =  new ListSubPartClass(deckListObj, 4, twidth, detailnr4, game3, tHeader: "Actioncard List", tbackbitmap: (ref local5), overruleFont: (ref local6));
        this.DeckListId = this.AddSubPart(ref tsubpart21, 830, 515, Math.Min(300, this.game.ScreenWidth - 830), 96, 0);
        this.ss = "Add Card";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
        this.a22id = this.AddSubPart(ref tsubpart21, 830, 615, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Add", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a22bid = this.AddSubPart(ref tsubpart21, 880, 614, 60, 20, 0);
        this.ss = "Copy Cards From";
        tsubpart21 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
        this.a221id = this.AddSubPart(ref tsubpart21, 930, 635, 32, 16, 1);
        tsubpart21 =  TextPartClass::new("Copy", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a221bid = this.AddSubPart(ref tsubpart21, 960, 634, 60, 20, 0);
        if (this.detailnr4 > -1)
        {
          this.ss = "Remove Card";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
          this.a23id = this.AddSubPart(ref tsubpart21, 930, 615, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Remove", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a23bid = this.AddSubPart(ref tsubpart21, 960, 614, 300, 20, 0);
          this.ss = "Set Chance %";
          tsubpart21 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
          this.a24id = this.AddSubPart(ref tsubpart21, 830, 635, 32, 16, 1);
          tsubpart21 =  TextPartClass::new("Set%", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
          this.a24bid = this.AddSubPart(ref tsubpart21, 880, 634, 300, 20, 0);
        }
      }
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 =  this.SubPartCounter;
        for (let mut index1: i32 =  0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 =  this.SubPartID[index1];
            if (num1 == this.LibListId)
            {
              let mut num2: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.libnr = num2;
                this.detailnr = -1;
                this.detailnr3 = -1;
                this.detailnr4 = -1;
                this.showinfo();
              }
              else if (num2 == -2)
              {
                this.libnr = -1;
                this.detailnr = -1;
                this.detailnr3 = -1;
                this.detailnr4 = -1;
                this.showinfo();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.expid)
            {
              StreamWriter text = File.CreateText(this.game.AppPath + "logs/" + this.game.Data.Name + "_hisunits_nato.txt");
              let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
              for (let mut index2: i32 =  0; index2 <= historicalUnitCounter; index2 += 1)
              {
                text.Write(Strings.Trim(Conversion.Str( this.game.Data.HistoricalUnitObj[index2].Counter)) + "," + this.game.Data.HistoricalUnitObj[index2].Name);
                text.Write("\r\n");
              }
              text.Close();
              let mut num3: i32 =   Interaction.MsgBox( "Export completed correctly", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else if (num1 == this.impid)
            {
              try
              {
                StreamReader streamReader = File.OpenText(this.game.AppPath + "logs/" + this.game.Data.Name + "_hisunits_nato.txt");
                let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
                for (let mut Number: i32 =  0; Number <= historicalUnitCounter; Number += 1)
                {
                  try
                  {
                    str: String = streamReader.ReadLine();
                    let mut num4: i32 =  Strings.InStr(str, ",") <= 0 ?  Math.Round(Conversion.Val(str)) :  Math.Round(Conversion.Val(Strings.Left(str, Strings.InStr(str, ","))));
                    this.game.Data.HistoricalUnitObj[Number].Counter = num4;
                  }
                  catch (Exception ex)
                  {
                    ProjectData.SetProjectError(ex);
                    let mut num5: i32 =   Interaction.MsgBox( ("Error in line " + Conversion.Str( Number)), Title: ( "Shadow Empire : Planetary Conquest"));
                    ProjectData.ClearProjectError();
                    break;
                  }
                }
              }
              catch (Exception ex)
              {
                ProjectData.SetProjectError(ex);
                let mut num6: i32 =   Interaction.MsgBox( "Couldnt find file. make export first.", Title: ( "Shadow Empire : Planetary Conquest"));
                ProjectData.ClearProjectError();
              }
              let mut num7: i32 =   Interaction.MsgBox( "Import completed correctly", Title: ( "Shadow Empire : Planetary Conquest"));
            }
            else
            {
              if (num1 == this.HisListId)
              {
                let mut num8: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num8 > -1)
                {
                  this.detailnr = num8;
                  this.detailnr3 = -1;
                  this.detailnr4 = -1;
                  this.showinfo();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.DeckListId)
              {
                let mut num9: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num9 > -1)
                {
                  this.detailnr4 = num9;
                  this.showinfo();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.VarListId)
              {
                let mut num10: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num10 > -1)
                {
                  this.detailnr6 = num10;
                  this.showinfo();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.SPListId)
              {
                let mut num11: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num11 > -1)
                {
                  this.detailnr5 = num11;
                  this.showinfo();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.AutoListId)
              {
                let mut num12: i32 =  this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                this.SubPartFlag[index1] = true;
                if (num12 > -1)
                {
                  this.detailnr3 = num12;
                  this.showinfo();
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.DescBox)
              {
                this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BRemoveHisId)
              {
                this.game.Data.RemoveHistoricalUnit(this.detailnr);
                this.MakeHisList();
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BAddHisId)
              {
                this.game.Data.AddHistoricalUnit();
                this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitCounter].Name = Interaction.InputBox("Give name for historical unit", "Shadow Empire : Planetary Conquest");
                this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitCounter].TempRegime =  Math.Round(Conversion.Val(Interaction.InputBox("Give regime# for historical unit", "Shadow Empire : Planetary Conquest")));
                this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitCounter].LibId.libSlot = this.libnr;
                this.detailnr = this.game.Data.HistoricalUnitCounter;
                this.game.Data.HistoricalUnitObj[this.game.Data.HistoricalUnitCounter].Type = 1;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a16id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 15, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.x1id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].Model = !this.game.Data.HistoricalUnitObj[this.detailnr].Model;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.x2id)
              {
                let mut num13: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give red.. (-255 to +255)", "Shadow Empire : Planetary Conquest")));
                if (num13 >= -255 & num13 <=  byte.MaxValue)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].Red = num13;
                  this.showinfo();
                }
                else
                {
                  let mut num14: i32 =   Interaction.MsgBox( "wrong input");
                }
                let mut num15: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give green.. (-255 to +255)", "Shadow Empire : Planetary Conquest")));
                if (num15 >= -255 & num15 <=  byte.MaxValue)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].Green = num15;
                  this.showinfo();
                }
                else
                {
                  let mut num16: i32 =   Interaction.MsgBox( "wrong input");
                }
                let mut num17: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give blue.. (-255 to +255)", "Shadow Empire : Planetary Conquest")));
                if (num17 >= -255 & num17 <=  byte.MaxValue)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].Blue = num17;
                  this.showinfo();
                }
                else
                {
                  let mut num18: i32 =   Interaction.MsgBox( "wrong input");
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.x3id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].SubParts[this.detailnr5] = -1;
                this.game.Data.HistoricalUnitObj[this.detailnr].Designation[this.detailnr5] = -1;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.x4id)
              {
                let mut num19: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give slot designation for Small Graphics, overrule counter.. -1=no small counter, use nato instead", "Shadow Empire : Planetary Conquest")));
                if (num19 < 0 | num19 > this.game.Data.SmallPicCounter)
                {
                  num19 =  Math.Round(Conversion.Val(Interaction.InputBox("Give nato counter number", "Shadow Empire : Planetary Conquest")));
                  this.game.Data.HistoricalUnitObj[this.detailnr].Designation[this.detailnr5] = num19;
                  this.game.Data.HistoricalUnitObj[this.detailnr].DesignationSmall[this.detailnr5] = -1;
                }
                else
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].DesignationSmall[this.detailnr5] = num19;
                  this.game.Data.HistoricalUnitObj[this.detailnr].Designation[this.detailnr5] = -1;
                }
                if (num19 >= -1)
                {
                  Form3::new( this.formref).Initialize(this.game.Data, 45, this.detailnr, this.detailnr5);
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
                let mut num20: i32 =   Interaction.MsgBox( "wrong input");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.x5id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 46, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.aa11id)
              {
                let mut num21: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give NATO counter #", "Shadow Empire : Planetary Conquest")));
                if (num21 != 0 & num21 >= -1 & num21 <= this.game.NATO.GetUpperBound(0))
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].Counter = num21;
                }
                else
                {
                  let mut num22: i32 =   Interaction.MsgBox( "Invalid NATO counter value. Either -1, 1-X");
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.ab11id)
              {
                let mut num23: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Small Graphic slot", "Shadow Empire : Planetary Conquest")));
                if (num23 >= -1 & num23 <= this.game.Data.SmallPicCounter)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].SmallGfx = num23;
                }
                else
                {
                  let mut num24: i32 =   Interaction.MsgBox( "Invalid NATO counter value. Either -1, 1-X");
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a41id)
              {
                let mut num25: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give MaxPresent", "Shadow Empire : Planetary Conquest")));
                if (num25 >= -1 & num25 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].MaxPresent = num25;
                }
                else
                {
                  let mut num26: i32 =   Interaction.MsgBox( "Invalid NATO counter value. Either -1, 1-X");
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a11id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].CounterString = Interaction.InputBox("Give Short Name", "Shadow Empire : Planetary Conquest");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a25id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].Name = Interaction.InputBox("Give Name (also sets units names assigned to this historical unit)", "Shadow Empire : Planetary Conquest");
                let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                for (let mut index3: i32 =  0; index3 <= unitCounter; index3 += 1)
                {
                  if (this.game.Data.UnitObj[index3].Historical == this.detailnr)
                    this.game.Data.UnitObj[index3].Name = this.game.Data.HistoricalUnitObj[this.detailnr].Name;
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a15id)
              {
                this.SortHis();
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a17id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].CommanderName = Interaction.InputBox("Give Commander Name", "Shadow Empire : Planetary Conquest");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.v19id)
              {
                let mut num27: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give type number", "Shadow Empire : Planetary Conquest")));
                let mut num28: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest")));
                let mut num29: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give small gfx counter graphic #", "Shadow Empire : Planetary Conquest")));
                let mut num30: i32 =  -1;
                if (num29 < 0)
                  num30 =  Math.Round(Conversion.Val(Interaction.InputBox("Give nato counter graphic #", "Shadow Empire : Planetary Conquest")));
                HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
                HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
                let mut detailnr: i32 =  this.detailnr;
                let mut index4: i32 =  detailnr;
                historicalUnitClassArray[index4].HisVarCount = historicalUnitObj[detailnr].HisVarCount + 1;
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount] = num27;
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount] = num28;
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount] = num30;
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount] = num29;
                this.detailnr6 = this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.v20id)
              {
                if (this.detailnr6 < this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount)
                {
                  let mut detailnr6: i32 =  this.detailnr6;
                  let mut num31: i32 =  this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount - 1;
                  for (let mut index5: i32 =  detailnr6; index5 <= num31; index5 += 1)
                  {
                    this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType[index5] = this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType[index5 + 1];
                    this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue[index5] = this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue[index5 + 1];
                    this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato[index5] = this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato[index5 + 1];
                    this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall[index5] = this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall[index5 + 1];
                  }
                }
                --this.detailnr6;
                HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
                HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
                let mut detailnr: i32 =  this.detailnr;
                let mut index6: i32 =  detailnr;
                historicalUnitClassArray[index6].HisVarCount = historicalUnitObj[detailnr].HisVarCount - 1;
                if (this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount > -1 & this.detailnr6 == -1)
                  this.detailnr6 = 0;
                if (this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount > -1)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                  this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                  this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                  this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].HisVarCount + 1]);
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.v21id)
              {
                let mut num32: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give type number", "Shadow Empire : Planetary Conquest")));
                let mut num33: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest")));
                let mut num34: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give small gfx counter graphic #", "Shadow Empire : Planetary Conquest")));
                let mut num35: i32 =  -1;
                if (num34 < 0)
                  num35 =  Math.Round(Conversion.Val(Interaction.InputBox("Give nato counter graphic #", "Shadow Empire : Planetary Conquest")));
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarType[this.detailnr6] = num32;
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue[this.detailnr6] = num33;
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarNato[this.detailnr6] = num34;
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarSmall[this.detailnr6] = num34;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.v22id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].HisVarValue[this.detailnr6] =  Math.Round(Conversion.Val(Interaction.InputBox("Give value", "Shadow Empire : Planetary Conquest")));
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a20id)
              {
                if (this.detailnr3 < this.game.Data.HistoricalUnitObj[this.detailnr].AutoEventCounter)
                {
                  let mut detailnr3: i32 =  this.detailnr3;
                  let mut num36: i32 =  this.game.Data.HistoricalUnitObj[this.detailnr].AutoEventCounter - 1;
                  for (let mut index7: i32 =  detailnr3; index7 <= num36; index7 += 1)
                  {
                    this.game.Data.HistoricalUnitObj[this.detailnr].AutoEvent[index7] = this.game.Data.HistoricalUnitObj[this.detailnr].AutoEvent[index7 + 1];
                    this.game.Data.HistoricalUnitObj[this.detailnr].AutoChance[index7] = this.game.Data.HistoricalUnitObj[this.detailnr].AutoChance[index7 + 1];
                  }
                }
                --this.detailnr3;
                HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
                HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
                let mut detailnr: i32 =  this.detailnr;
                let mut index8: i32 =  detailnr;
                historicalUnitClassArray[index8].AutoEventCounter = historicalUnitObj[detailnr].AutoEventCounter - 1;
                if (this.game.Data.HistoricalUnitObj[this.detailnr].AutoEventCounter > -1 & this.detailnr3 == -1)
                  this.detailnr3 = 0;
                if (this.game.Data.HistoricalUnitObj[this.detailnr].AutoEventCounter > -1)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].AutoEvent = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].AutoEvent, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].AutoEventCounter + 1]);
                  this.game.Data.HistoricalUnitObj[this.detailnr].AutoChance = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].AutoChance, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].AutoEventCounter + 1]);
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a23id)
              {
                if (this.detailnr4 < this.game.Data.HistoricalUnitObj[this.detailnr].DeckCardCounter)
                {
                  let mut detailnr4: i32 =  this.detailnr4;
                  let mut num37: i32 =  this.game.Data.HistoricalUnitObj[this.detailnr].DeckCardCounter - 1;
                  for (let mut index9: i32 =  detailnr4; index9 <= num37; index9 += 1)
                  {
                    this.game.Data.HistoricalUnitObj[this.detailnr].DeckCard[index9] = this.game.Data.HistoricalUnitObj[this.detailnr].DeckCard[index9 + 1];
                    this.game.Data.HistoricalUnitObj[this.detailnr].DeckChance[index9] = this.game.Data.HistoricalUnitObj[this.detailnr].DeckChance[index9 + 1];
                  }
                }
                --this.detailnr4;
                HistoricalUnitClass[] historicalUnitObj = this.game.Data.HistoricalUnitObj;
                HistoricalUnitClass[] historicalUnitClassArray = historicalUnitObj;
                let mut detailnr: i32 =  this.detailnr;
                let mut index10: i32 =  detailnr;
                historicalUnitClassArray[index10].DeckCardCounter = historicalUnitObj[detailnr].DeckCardCounter - 1;
                if (this.game.Data.HistoricalUnitObj[this.detailnr].DeckCardCounter > -1 & this.detailnr4 == -1)
                  this.detailnr4 = 0;
                if (this.game.Data.HistoricalUnitObj[this.detailnr].DeckCardCounter > -1)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].DeckCard = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].DeckCard, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].DeckCardCounter + 1]);
                  this.game.Data.HistoricalUnitObj[this.detailnr].DeckChance = (int[]) Utils.CopyArray((Array) this.game.Data.HistoricalUnitObj[this.detailnr].DeckChance, (Array) new int[this.game.Data.HistoricalUnitObj[this.detailnr].DeckCardCounter + 1]);
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a21id)
              {
                let mut num38: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give chance on event occuring.. (0-100)", "Shadow Empire : Planetary Conquest")));
                if (num38 >= 0 & num38 <= 100)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].AutoChance[this.detailnr3] = num38;
                  this.showinfo();
                }
                else
                {
                  let mut num39: i32 =   Interaction.MsgBox( "Between 0-100 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.asid)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 94, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a24id)
              {
                let mut num40: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give chance on event occuring.. (0-100)", "Shadow Empire : Planetary Conquest")));
                if (num40 >= 0 & num40 <= 100)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].DeckChance[this.detailnr4] = num40;
                  this.showinfo();
                }
                else
                {
                  let mut num41: i32 =   Interaction.MsgBox( "Between 0-100 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a26id)
              {
                let mut num42: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("PP Cost", "Shadow Empire : Planetary Conquest")));
                if (num42 >= -9999 & num42 <= 9999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].PP = num42;
                  this.showinfo();
                }
                else
                {
                  let mut num43: i32 =   Interaction.MsgBox( "Between 0-9999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a440id)
              {
                let mut num44: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("AI Zone List Nr #", "Shadow Empire : Planetary Conquest")));
                if (num44 >= 0 & num44 <= 9999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].AIlist = num44;
                  this.showinfo();
                }
                else
                {
                  let mut num45: i32 =   Interaction.MsgBox( "Between 0-9999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a39id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].NoSplit = !this.game.Data.HistoricalUnitObj[this.detailnr].NoSplit;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.apoolid)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].Pool = !this.game.Data.HistoricalUnitObj[this.detailnr].Pool;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a27id)
              {
                let mut num46: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("CombatMod", "Shadow Empire : Planetary Conquest")));
                if (num46 >= -100 & num46 <= 9999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].CombatMod = num46;
                  this.showinfo();
                }
                else
                {
                  let mut num47: i32 =   Interaction.MsgBox( "Between -100 and 9999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a28id)
              {
                let mut num48: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("MoraleMod", "Shadow Empire : Planetary Conquest")));
                if (num48 >= -100 & num48 <= 9999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].MoraleMod = num48;
                  this.showinfo();
                }
                else
                {
                  let mut num49: i32 =   Interaction.MsgBox( "Between -100 and 9999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a29id)
              {
                let mut num50: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("StaffSize", "Shadow Empire : Planetary Conquest")));
                if (num50 >= 0 & num50 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].StaffSize = num50;
                  this.showinfo();
                }
                else
                {
                  let mut num51: i32 =   Interaction.MsgBox( "Between -100 and 9999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a31id)
              {
                this.game.Data.HistoricalUnitObj[this.detailnr].Fixed = !this.game.Data.HistoricalUnitObj[this.detailnr].Fixed;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a32id)
              {
                let mut num52: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("NameCounter (-1=for not use)", "Shadow Empire : Planetary Conquest")));
                if (num52 >= -1 & num52 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].NameCounter = num52;
                  this.showinfo();
                }
                else
                {
                  let mut num53: i32 =   Interaction.MsgBox( "Between 0 and 99999 plz");
                }
                let mut num54: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Chance on Old number currently not used in % (0=for not use)", "Shadow Empire : Planetary Conquest")));
                if (num54 >= 0 & num54 <= 100)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].PercentOldName = num54;
                  this.showinfo();
                }
                else
                {
                  let mut num55: i32 =   Interaction.MsgBox( "Between 0 and 100 plz");
                }
                if ( Math.Round(Conversion.Val(Interaction.InputBox("0=use no romans, 1=use romans", "Shadow Empire : Planetary Conquest"))) == 1)
                  this.game.Data.HistoricalUnitObj[this.detailnr].UseRomans = true;
                else
                  this.game.Data.HistoricalUnitObj[this.detailnr].UseRomans = false;
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a34id)
              {
                let mut num56: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Tempvar", "Shadow Empire : Planetary Conquest")));
                if (num56 >= -99999 & num56 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].TempVar1 = num56;
                  this.showinfo();
                }
                else
                {
                  let mut num57: i32 =   Interaction.MsgBox( "Between -99999 and 99999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a42id)
              {
                let mut num58: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Tempvar4", "Shadow Empire : Planetary Conquest")));
                if (num58 >= -99999 & num58 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].TempVar4 = num58;
                  this.showinfo();
                }
                else
                {
                  let mut num59: i32 =   Interaction.MsgBox( "Between -99999 and 99999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a43id)
              {
                let mut num60: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Tempvar5", "Shadow Empire : Planetary Conquest")));
                if (num60 >= -99999 & num60 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].TempVar5 = num60;
                  this.showinfo();
                }
                else
                {
                  let mut num61: i32 =   Interaction.MsgBox( "Between -99999 and 99999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a45id)
              {
                let mut num62: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Battlegroup flag (0=no,1=yes): ", "Shadow Empire : Planetary Conquest")));
                if (num62 >= 0 & num62 <= 1)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].BattleGroup = num62;
                  this.showinfo();
                }
                else
                {
                  let mut num63: i32 =   Interaction.MsgBox( "Either 0 or 1 as value plz.");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a35id)
              {
                let mut num64: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Tempvar", "Shadow Empire : Planetary Conquest")));
                if (num64 >= -99999 & num64 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].TempVar2 = num64;
                  this.showinfo();
                }
                else
                {
                  let mut num65: i32 =   Interaction.MsgBox( "Between -99999 and 99999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a36id)
              {
                let mut num66: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Tempvar", "Shadow Empire : Planetary Conquest")));
                if (num66 >= -99999 & num66 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].TempVar3 = num66;
                  this.showinfo();
                }
                else
                {
                  let mut num67: i32 =   Interaction.MsgBox( "Between -99999 and 99999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a37id)
              {
                let mut num68: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("XP", "Shadow Empire : Planetary Conquest")));
                if (num68 >= 0 & num68 <= 99999)
                {
                  this.game.Data.HistoricalUnitObj[this.detailnr].Xp = num68;
                  this.showinfo();
                }
                else
                {
                  let mut num69: i32 =   Interaction.MsgBox( "Between 0 and 99999 plz");
                }
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a38id)
              {
                Form2::new( this.formref).Initialize(this.game.Data, 6, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a30id)
              {
                let mut num70: i32 =   Interaction.MsgBox( "Do you want to OVERWRITE all the units equipment with the BASIC MODELS?? .... advice No..", MsgBoxStyle.YesNo);
                let mut num71: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Overwite how many first tempvars? 0=none, 1=only tv1, 2=tv1+tv2, etc...", "Shadow Empire : Planetary Conquest")));
                let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
                for (let mut index11: i32 =  0; index11 <= historicalUnitCounter; index11 += 1)
                {
                  if (this.game.Data.HistoricalUnitObj[index11].ModelMaster > -1)
                  {
                    let mut modelMaster: i32 =  this.game.Data.HistoricalUnitObj[index11].ModelMaster;
                    this.game.Data.HistoricalUnitObj[index11].Counter = this.game.Data.HistoricalUnitObj[modelMaster].Counter;
                    this.game.Data.HistoricalUnitObj[index11].Green = this.game.Data.HistoricalUnitObj[modelMaster].Green;
                    this.game.Data.HistoricalUnitObj[index11].Red = this.game.Data.HistoricalUnitObj[modelMaster].Red;
                    this.game.Data.HistoricalUnitObj[index11].SmallGfx = this.game.Data.HistoricalUnitObj[modelMaster].SmallGfx;
                    this.game.Data.HistoricalUnitObj[index11].Blue = this.game.Data.HistoricalUnitObj[modelMaster].Blue;
                    this.game.Data.HistoricalUnitObj[index11].Type = this.game.Data.HistoricalUnitObj[modelMaster].Type;
                    this.game.Data.HistoricalUnitObj[index11].TempRegime = this.game.Data.HistoricalUnitObj[modelMaster].TempRegime;
                    this.game.Data.HistoricalUnitObj[index11].People = this.game.Data.HistoricalUnitObj[modelMaster].People;
                    this.game.Data.HistoricalUnitObj[index11].UsePeopleGfx = this.game.Data.HistoricalUnitObj[modelMaster].UsePeopleGfx;
                    if (num71 >= 1)
                      this.game.Data.HistoricalUnitObj[index11].TempVar1 = this.game.Data.HistoricalUnitObj[modelMaster].TempVar1;
                    if (num71 >= 2)
                      this.game.Data.HistoricalUnitObj[index11].TempVar2 = this.game.Data.HistoricalUnitObj[modelMaster].TempVar2;
                    if (num71 >= 3)
                      this.game.Data.HistoricalUnitObj[index11].TempVar3 = this.game.Data.HistoricalUnitObj[modelMaster].TempVar3;
                    if (num71 >= 4)
                      this.game.Data.HistoricalUnitObj[index11].TempVar4 = this.game.Data.HistoricalUnitObj[modelMaster].TempVar4;
                    if (num71 >= 5)
                      this.game.Data.HistoricalUnitObj[index11].TempVar5 = this.game.Data.HistoricalUnitObj[modelMaster].TempVar5;
                    let mut unitCounter1: i32 =  this.game.Data.UnitCounter;
                    for (let mut index12: i32 =  0; index12 <= unitCounter1; index12 += 1)
                    {
                      if (this.game.Data.UnitObj[index12].Historical == index11)
                        this.game.Data.UnitObj[index12].HistoricalSubPart = -1;
                    }
                    let mut index13: i32 =  0;
                    do
                    {
                      this.game.Data.HistoricalUnitObj[index11].SubParts[index13] = this.game.Data.HistoricalUnitObj[modelMaster].SubParts[index13];
                      this.game.Data.HistoricalUnitObj[index11].Designation[index13] = this.game.Data.HistoricalUnitObj[modelMaster].Designation[index13];
                      this.game.Data.HistoricalUnitObj[index11].DesignationSmall[index13] = this.game.Data.HistoricalUnitObj[modelMaster].DesignationSmall[index13];
                      if (this.game.Data.HistoricalUnitObj[index11].SubParts[index13] > -1)
                      {
                        let mut unitCounter2: i32 =  this.game.Data.UnitCounter;
                        for (let mut unr: i32 =  0; unr <= unitCounter2; unr += 1)
                        {
                          if (this.game.Data.UnitObj[unr].Historical == index11 & this.game.Data.UnitObj[unr].HistoricalSubPart == -1)
                          {
                            this.game.Data.UnitObj[unr].HistoricalSubPart = index13;
                            if (num70 == 6)
                            {
                              let mut hq: i32 =  this.game.Data.UnitObj[unr].HQ;
                              DrawMod.TGame.HandyFunctionsObj.CopyUnit(unr, DrawMod.TGame.HandyFunctionsObj.GetPreDef(this.game.Data.HistoricalUnitObj[index11].SubParts[index13]), false);
                              this.game.Data.UnitObj[unr].HQ = hq;
                              break;
                            }
                            break;
                          }
                        }
                      }
                      index13 += 1;
                    }
                    while (index13 <= 9);
                  }
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a18id)
              {
                s: String = this.game.HandyFunctionsObj.LoadSomething("All|*.*|Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of Commander Sprite. Cancel is set to nothing.", this.game.AppPath + "graphics\\", true);
                if (File.Exists(this.game.AppPath + "graphics/" + s))
                  this.game.Data.HistoricalUnitObj[this.detailnr].ReplaceSprite1(s);
                else
                  this.game.Data.HistoricalUnitObj[this.detailnr].ReplaceSprite1("");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a33id)
              {
                s2: String = this.game.HandyFunctionsObj.LoadSomething("All|*.*|Png|*.png|Bitmaps (*.bmp)|*.bmp", "Give File Name For Replacement of Commander Sprite. Cancel is set to nothing.", this.game.AppPath + "graphics\\", true);
                if (File.Exists(this.game.AppPath + "graphics/" + s2))
                  this.game.Data.HistoricalUnitObj[this.detailnr].ReplaceSprite2(s2);
                else
                  this.game.Data.HistoricalUnitObj[this.detailnr].ReplaceSprite2("");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a12id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 36, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a320id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 84, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.x7id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 83, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.xx7id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 88, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a19id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 39, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a22id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 40, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a221id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 85, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.v23id)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 86, this.detailnr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.bDownId)
              {
                if (this.detailnr > -1)
                {
                  if (this.detailnr < this.game.Data.HistoricalUnitCounter)
                  {
                    HistoricalUnitClass historicalUnitClass = this.game.Data.HistoricalUnitObj[this.detailnr + 1].Clone();
                    this.game.Data.HistoricalUnitObj[this.detailnr + 1] = this.game.Data.HistoricalUnitObj[this.detailnr];
                    this.game.Data.HistoricalUnitObj[this.detailnr] = historicalUnitClass;
                    let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                    for (let mut index14: i32 =  0; index14 <= unitCounter; index14 += 1)
                    {
                      if (this.game.Data.UnitObj[index14].Historical == this.detailnr)
                        this.game.Data.UnitObj[index14].Historical = this.detailnr + 1;
                      else if (this.game.Data.UnitObj[index14].Historical == this.detailnr + 1)
                        this.game.Data.UnitObj[index14].Historical = this.detailnr;
                    }
                    let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
                    for (let mut index15: i32 =  0; index15 <= historicalUnitCounter; index15 += 1)
                    {
                      if (this.game.Data.HistoricalUnitObj[index15].ModelMaster == this.detailnr)
                        this.game.Data.HistoricalUnitObj[index15].ModelMaster = this.detailnr + 1;
                      else if (this.game.Data.HistoricalUnitObj[index15].ModelMaster == this.detailnr + 1)
                        this.game.Data.HistoricalUnitObj[index15].ModelMaster = this.detailnr;
                    }
                  }
                  this += 1.detailnr;
                }
                this.MakeHisList();
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.bUpId)
              {
                if (this.detailnr > 0)
                {
                  HistoricalUnitClass historicalUnitClass = this.game.Data.HistoricalUnitObj[this.detailnr - 1].Clone();
                  this.game.Data.HistoricalUnitObj[this.detailnr - 1] = this.game.Data.HistoricalUnitObj[this.detailnr];
                  this.game.Data.HistoricalUnitObj[this.detailnr] = historicalUnitClass;
                  let mut unitCounter: i32 =  this.game.Data.UnitCounter;
                  for (let mut index16: i32 =  0; index16 <= unitCounter; index16 += 1)
                  {
                    if (this.game.Data.UnitObj[index16].Historical == this.detailnr)
                      this.game.Data.UnitObj[index16].Historical = this.detailnr - 1;
                    else if (this.game.Data.UnitObj[index16].Historical == this.detailnr - 1)
                      this.game.Data.UnitObj[index16].Historical = this.detailnr;
                  }
                  let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
                  for (let mut index17: i32 =  0; index17 <= historicalUnitCounter; index17 += 1)
                  {
                    if (this.game.Data.HistoricalUnitObj[index17].ModelMaster == this.detailnr)
                      this.game.Data.HistoricalUnitObj[index17].ModelMaster = this.detailnr - 1;
                    else if (this.game.Data.HistoricalUnitObj[index17].ModelMaster == this.detailnr - 1)
                      this.game.Data.HistoricalUnitObj[index17].ModelMaster = this.detailnr;
                  }
                  --this.detailnr;
                }
                this.MakeHisList();
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BNameId)
              {
                this.game.Data.LocObj[this.locNr].Name = Interaction.InputBox("Give new name, please.", "Shadow Empire : Planetary Conquest");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BPplId)
              {
                Form3::new( this.formref).Initialize(this.game.Data, 16, this.locNr);
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.BRemoveId)
              {
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location = -1;
                this.game.Data.RemoveLoc(this.locNr);
                this.locNr = -1;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a1id)
              {
                let mut num72: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give New VP for hex 0-99", "Shadow Empire : Planetary Conquest")));
                if (num72 < 0 | num72 > 99)
                {
                  let mut num73: i32 =   Interaction.MsgBox( "Oh.. please.. between 0 and 99!", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].VP = num72;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a2id)
              {
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name = Interaction.InputBox("Give new Name for hex...", "Shadow Empire : Planetary Conquest");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a3id)
              {
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelText1 = Interaction.InputBox("Give new label for top of hex (label1)", "Shadow Empire : Planetary Conquest");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a4id)
              {
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelText2 = Interaction.InputBox("Give new label for top of hex (label1)", "Shadow Empire : Planetary Conquest");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a5id)
              {
                let mut num74: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Label Type for top of hex (label1)", "Shadow Empire : Planetary Conquest")));
                if (num74 < 0 | num74 > 10)
                {
                  let mut num75: i32 =   Interaction.MsgBox( "Oh.. please.. between 0 and 10 (0=dont show, 1-5 = small font, 6-10 = big font)", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType1 = num74;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a6id)
              {
                let mut num76: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Label Type for bottom of hex (label2)", "Shadow Empire : Planetary Conquest")));
                if (num76 < 0 | num76 > 10)
                {
                  let mut num77: i32 =   Interaction.MsgBox( "Oh.. please.. between 0 and 10 (0=dont show, 1-5 = small font, 6-10 = big font)", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType2 = num76;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a7id)
              {
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType2 = 0;
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType1 = 0;
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelText1 = "";
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelText2 = "";
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a8id)
              {
                str: String = Interaction.InputBox("Give new label multiple hexes (starting on this hex)", "Shadow Empire : Planetary Conquest", this.game.HandyFunctionsObj.GetHexName(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected));
                let mut usetype: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Label Type for top of hex (label1)", "Shadow Empire : Planetary Conquest")));
                if (usetype > 0 & usetype <= 10 && Strings.Len(str) > 0)
                {
                  this.game.HandyFunctionsObj.MakeSpecificAutoLabels(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, usetype, str, true);
                  let mut num78: i32 =   Interaction.MsgBox( "Done");
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a13id)
              {
                for (let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
                  this.game.Data.RemoveHistoricalUnit(historicalUnitCounter);
                this.game.Data.HistoricalIDCounter = 0;
                this.showinfo();
                this.MakeHisList();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a14id)
              {
                this.AutoDetectHistoricals();
                this.showinfo();
                this.MakeHisList();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a9id)
              {
                let mut index18: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give Hex Side (0=top,1=NE, 2=SE, 3=S, 4=SW, 5=NW)", "Shadow Empire : Planetary Conquest")));
                if (index18 >= 0 & index18 <= 5)
                {
                  let mut num79: i32 =   Math.Round(Conversion.Val(Interaction.InputBox("Give RoadType", "Shadow Empire : Planetary Conquest")));
                  if (num79 >= -1 & num79 <= this.game.Data.RoadTypeCounter)
                  {
                    this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index18] = num79;
                    let mut num80: i32 =   Interaction.MsgBox( "Done");
                  }
                }
                this.showinfo();
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

    pub fn AutoDetectHistoricals()
    {
      for (let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter; historicalUnitCounter >= 0; historicalUnitCounter += -1)
        this.game.Data.RemoveHistoricalUnit(historicalUnitCounter);
      this.game.Data.HistoricalIDCounter = 0;
      let mut unitCounter: i32 =  this.game.Data.UnitCounter;
      for (let mut unr: i32 =  0; unr <= unitCounter; unr += 1)
      {
        if (this.game.Data.UnitObj[unr].PreDef == -1 & this.game.Data.UnitObj[unr].X > -1)
        {
          if (this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].Regime > -1)
            this.game.Data.UnitObj[unr].Regime = this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.Data.UnitObj[unr].X, this.game.Data.UnitObj[unr].Y].Regime;
          if (this.game.Data.UnitObj[unr].IsHQ)
          {
            this.game.Data.AddHistoricalUnit();
            let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Name = this.game.Data.UnitObj[unr].Name;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Type = 5 + this.game.HandyFunctionsObj.HowmanyHQsBelow(unr);
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].Counter = -1;
            this.game.Data.HistoricalUnitObj[historicalUnitCounter].TempRegime = this.game.Data.UnitObj[unr].Regime;
            if (Strings.InStr(this.game.Data.UnitObj[unr].Name, " ") > 0)
              this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Strings.Left(this.game.Data.UnitObj[unr].Name, Math.Min(6, Strings.InStr(this.game.Data.UnitObj[unr].Name, " ") - 1));
            else
              this.game.Data.HistoricalUnitObj[historicalUnitCounter].CounterString = Strings.Left(this.game.Data.UnitObj[unr].Name, 6);
            this.game.Data.UnitObj[unr].Historical = historicalUnitCounter;
          }
          else
          {
            let mut index1: i32 =  -1;
            let mut historicalUnitCounter1: i32 =  this.game.Data.HistoricalUnitCounter;
            for (let mut index2: i32 =  0; index2 <= historicalUnitCounter1; index2 += 1)
            {
              if (Operators.CompareString(Strings.LCase(Strings.Trim(this.game.Data.HistoricalUnitObj[index2].Name)), Strings.LCase(Strings.Trim(this.game.Data.UnitObj[unr].Name)), false) == 0 && this.game.Data.HistoricalUnitObj[index2].TempRegime == this.game.Data.UnitObj[unr].Regime)
              {
                index1 = index2;
                break;
              }
            }
            if (index1 > -1)
            {
              this.game.Data.HistoricalUnitObj[index1].Type = 2;
              this.game.Data.UnitObj[unr].Historical = index1;
            }
            else
            {
              this.game.Data.AddHistoricalUnit();
              let mut historicalUnitCounter2: i32 =  this.game.Data.HistoricalUnitCounter;
              this.game.Data.HistoricalUnitObj[historicalUnitCounter2].TempRegime = this.game.Data.UnitObj[unr].Regime;
              this.game.Data.HistoricalUnitObj[historicalUnitCounter2].Name = this.game.Data.UnitObj[unr].Name;
              this.game.Data.HistoricalUnitObj[historicalUnitCounter2].Type = 1;
              this.game.Data.HistoricalUnitObj[historicalUnitCounter2].Counter = 6;
              this.game.Data.HistoricalUnitObj[historicalUnitCounter2].CounterString = "";
              let mut length: i32 =  this.game.Data.UnitObj[unr].Name.Length;
              for (let mut Start: i32 =  1; Start <= length; Start += 1)
              {
                let mut Number: i32 =   Math.Round(Conversion.Val(Strings.Mid(this.game.Data.UnitObj[unr].Name, Start)));
                if (Number > 0)
                {
                  this.game.Data.HistoricalUnitObj[historicalUnitCounter2].CounterString = Strings.Trim(Conversion.Str( Number));
                  break;
                }
              }
              this.game.Data.UnitObj[unr].Historical = historicalUnitCounter2;
            }
          }
        }
      }
      this.SortHis();
    }

    pub fn SortHis()
    {
      let mut num1: i32 =  1;
      while (num1 == 1)
      {
        num1 = 0;
        let mut historicalUnitCounter1: i32 =  this.game.Data.HistoricalUnitCounter;
        for (let mut index1: i32 =  1; index1 <= historicalUnitCounter1; index1 += 1)
        {
          let mut num2: i32 =  this.game.Data.HistoricalUnitObj[index1 - 1].TempRegime * 100000 + this.game.Data.HistoricalUnitObj[index1 - 1].Type * 1000;
          if (this.game.Data.HistoricalUnitObj[index1 - 1].Model)
            num2 += 50000;
          let mut num3: i32 =  this.game.Data.HistoricalUnitObj[index1].TempRegime * 100000 + this.game.Data.HistoricalUnitObj[index1].Type * 1000;
          if (this.game.Data.HistoricalUnitObj[index1].Model)
            num3 += 50000;
          if (num2 < num3)
          {
            num1 = 1;
            HistoricalUnitClass historicalUnitClass = this.game.Data.HistoricalUnitObj[index1 - 1].Clone();
            this.game.Data.HistoricalUnitObj[index1 - 1] = this.game.Data.HistoricalUnitObj[index1];
            this.game.Data.HistoricalUnitObj[index1] = historicalUnitClass;
            let mut unitCounter: i32 =  this.game.Data.UnitCounter;
            for (let mut index2: i32 =  0; index2 <= unitCounter; index2 += 1)
            {
              if (this.game.Data.UnitObj[index2].Historical == index1)
                this.game.Data.UnitObj[index2].Historical = index1 - 1;
              else if (this.game.Data.UnitObj[index2].Historical == index1 - 1)
                this.game.Data.UnitObj[index2].Historical = index1;
            }
            let mut historicalUnitCounter2: i32 =  this.game.Data.HistoricalUnitCounter;
            for (let mut index3: i32 =  0; index3 <= historicalUnitCounter2; index3 += 1)
            {
              if (this.game.Data.HistoricalUnitObj[index3].ModelMaster == index1)
                this.game.Data.HistoricalUnitObj[index3].ModelMaster = index1 - 1;
              else if (this.game.Data.HistoricalUnitObj[index3].ModelMaster == index1 - 1)
                this.game.Data.HistoricalUnitObj[index3].ModelMaster = index1;
            }
          }
        }
      }
      let mut historicalUnitCounter: i32 =  this.game.Data.HistoricalUnitCounter;
      for (let mut index: i32 =  0; index <= historicalUnitCounter; index += 1)
        this.game.Data.HistoricalUnitObj[index].LoadSprites();
      this.detailnr = -1;
    }
  }
}
