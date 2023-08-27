// Decompiled with JetBrains decompiler
// Type: WindowsApplication1.LocWindowClass
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
  pub class LocWindowClass : WindowClass
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
     BRemoveAreaId: i32;
     BAddAreaId: i32;
     BAreaId: i32;
     bareatextid: i32;
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
     a10id: i32;
     a10bid: i32;
     a11id: i32;
     a11bid: i32;
     aa11id: i32;
     aa11bid: i32;
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
     a22id: i32;
     a22bid: i32;
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
     HisListId: i32;
     ListClass HisListObj;
     AreaListId: i32;
     ListClass AreaListObj;
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
     detailnr: i32;
     detailnr2: i32;
     detailnr3: i32;
     detailnr4: i32;
     detailnr5: i32;
     DescBox: i32;
     ss: String;

    pub LocWindowClass( tGame: GameClass)
      : base( tGame, tGame.ScreenWidth, tGame.ScreenHeight - 100, tDoBorders: 1, tHeaderString: "Hex")
    {
      this.locNr = !(this.game.SelectX > -1 & this.game.SelectY > -1) ? -1 : this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Location;
      this.detailnr = -1;
      this.detailnr2 = -1;
      this.detailnr3 = -1;
      this.detailnr4 = -1;
      this.detailnr5 = -1;
      this.MakeAreaList();
      this.showinfo();
    }

    pub fn MakeAreaList()
    {
      if (this.AreaListId > 0)
        this.RemoveSubPart(this.AreaListId);
      if (this.BAddAreaId > 0)
        this.RemoveSubPart(this.BAddAreaId);
      if (this.BRemoveAreaId > 0)
        this.RemoveSubPart(this.BRemoveAreaId);
      if (this.game.Data.AreaCounter > -1)
      {
        this.AreaListObj = ListClass::new();
        let mut num1: i32 = -1;
        let mut num2: i32 = -1;
        let mut areaCounter: i32 = this.game.Data.AreaCounter;
        for (let mut tdata: i32 = 0; tdata <= areaCounter; tdata += 1)
        {
          num2 += 1;
          str: String = "(" + this.game.Data.AreaObj[tdata].Slot.ToString() + "," + this.game.Data.AreaObj[tdata].Code.ToString() + ")";
          this.AreaListObj.add(Conversion.Str( this.game.Data.AreaObj[tdata].ID) + ") " + this.game.Data.AreaObj[tdata].Name + str, tdata);
          if (this.detailnr2 == tdata)
            num1 = num2;
        }
        if (num1 == -1)
          this.detailnr2 = -1;
        ListClass areaListObj = this.AreaListObj;
        let mut tlistselect: i32 = num1;
        let mut game: GameClass = this.game;
         local1: Bitmap =  this.OwnBitmap;
        font: Font =  null;
         local2: Font =  font;
        let mut tsubpart: SubPartClass =  new ListSubPartClass(areaListObj, 15, 300, tlistselect, game, tHeader: "Areas", tbackbitmap: ( local1), bbx: 10, bby: 400, overruleFont: ( local2));
        this.AreaListId = this.AddSubPart( tsubpart, 10, 400, 300, 288, 0);
      }
      if (this.BAddAreaId > 0)
        this.RemoveSubPart(this.BAddAreaId);
      this.ss = "Click to add another Area";
      let mut tsubpart1: SubPartClass =  ButtonPartClass::new(this.game.BUTTONPLUS, tDescript: this.ss);
      this.BAddAreaId = this.AddSubPart( tsubpart1, 330, 450, 32, 16, 1);
    }

    pub fn DoRefresh()
    {
      this.MakeAreaList();
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
      if (this.BRemoveAreaId > 0)
        this.RemoveSubPart(this.BRemoveAreaId);
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
      if (this.BAreaId > 0)
        this.RemoveSubPart(this.BAreaId);
      if (this.bareatextid > 0)
        this.RemoveSubPart(this.bareatextid);
      if (this.DescBox > 0)
        this.RemoveSubPart(this.DescBox);
      if (this.AutoListId > 0)
        this.RemoveSubPart(this.AutoListId);
      if (this.DeckListId > 0)
        this.RemoveSubPart(this.DeckListId);
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
      if (this.a22id > 0)
        this.RemoveSubPart(this.a22id);
      if (this.a22bid > 0)
        this.RemoveSubPart(this.a22bid);
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
      if (this.a32bid > 0)
        this.RemoveSubPart(this.a32bid);
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
      this.MakeAreaList();
      if (this.detailnr2 > -1)
      {
        this.ss = "Remove Area";
        let mut tsubpart: SubPartClass =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.BRemoveAreaId = this.AddSubPart( tsubpart, 330, 490, 32, 16, 1);
      }
      SubPartClass tsubpart1;
      if (this.locNr > -1)
      {
        this.ss = "Click to change the name of the location in this hex";
        let mut tsubpart2: SubPartClass =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BNameId = this.AddSubPart( tsubpart2, 10, 50, 32, 16, 1);
        tsubpart1 =  TextPartClass::new(this.game.Data.LocObj[this.locNr].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BNameTextId = this.AddSubPart( tsubpart1, 50, 49, 300, 20, 0);
        this.ss = "Click to remove the location from this hex. Note that the landscapetype will stay as it was!";
        tsubpart1 =  ButtonPartClass::new(this.game.BUTTONKILL, tDescript: this.ss);
        this.BRemoveId = this.AddSubPart( tsubpart1, 10, 70, 32, 16, 1);
        tsubpart1 =  TextPartClass::new("Remove Loc from Hex", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BRemoveTextid = this.AddSubPart( tsubpart1, 50, 69, 300, 20, 0);
        this.ss = "Click to change the people that this location belongs too";
        tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
        this.BPplId = this.AddSubPart( tsubpart1, 10, 90, 32, 16, 1);
        tsubpart1 =  TextPartClass::new(this.game.Data.PeopleObj[this.game.Data.LocObj[this.locNr].People].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.BPplTextId = this.AddSubPart( tsubpart1, 50, 89, 300, 20, 0);
      }
      this.ss = "Click to draw on the map a certain areacode slot with a certain value";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONDRAW, tDescript: this.ss);
      this.BAreaId = this.AddSubPart( tsubpart1, 10, 140, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Set Area Code Values", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.bareatextid = this.AddSubPart( tsubpart1, 50, 139, 300, 20, 0);
      this.ss = "Click to change the small label for this hex.";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a12id = this.AddSubPart( tsubpart1, 10, 170, 32, 16, 1);
      if (Operators.CompareString(this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SmallLabel, "", false) == 0)
      {
        tsubpart1 =  TextPartClass::new("SmallLabel: -none set-", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a12bid = this.AddSubPart( tsubpart1, 50, 169, 300, 20, 0);
      }
      else
      {
        tsubpart1 =  TextPartClass::new("SmallLabel:" + this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SmallLabel, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
        this.a12bid = this.AddSubPart( tsubpart1, 50, 169, 300, 20, 0);
      }
      this.ss = "Click to change the small label type for this hex. 00=normal, 10=top, 20=bottom, x0 = white, x1=red, x2=green, x3=blue, x4=yellow, x5=light blue. example: 12=  top+green, 4=center yellow";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a13id = this.AddSubPart( tsubpart1, 410, 170, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("SmallLabelType:" + this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SmallLabelType.ToString(), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a13bid = this.AddSubPart( tsubpart1, 450, 169, 300, 20, 0);
      this.ss = "Click to change the number of Victory Points on this hex.";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a1id = this.AddSubPart( tsubpart1, 10, 190, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Hex VP:" + Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].VP), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a1bid = this.AddSubPart( tsubpart1, 50, 189, 300, 20, 0);
      this.ss = "Click to change the name of the Hex (Location name overrules this name)";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a2id = this.AddSubPart( tsubpart1, 10, 210, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Name:" + this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].Name, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a2bid = this.AddSubPart( tsubpart1, 50, 209, 300, 20, 0);
      this.ss = "Click to set Text Label 1 (top of hex)";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a3id = this.AddSubPart( tsubpart1, 10, 230, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Label1:" + this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelText1, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a3bid = this.AddSubPart( tsubpart1, 50, 229, 300, 20, 0);
      this.ss = "Click to set Text Label 2 (bottom of hex)";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a4id = this.AddSubPart( tsubpart1, 10, 250, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Label2:" + this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelText2, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a4bid = this.AddSubPart( tsubpart1, 50, 249, 300, 20, 0);
      this.ss = "Click to set Type of Label for Label 1 (top of hex)";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a5id = this.AddSubPart( tsubpart1, 10, 270, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Type1:" + Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType1), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a5bid = this.AddSubPart( tsubpart1, 50, 269, 300, 20, 0);
      this.ss = "Click to set Type of Label for Label 2 (bottom of hex)";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONBLOCK, tDescript: this.ss);
      this.a6id = this.AddSubPart( tsubpart1, 10, 290, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Type2:" + Conversion.Str( this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType2), Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a6bid = this.AddSubPart( tsubpart1, 50, 289, 300, 20, 0);
      this.ss = "Click to clear";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.a7id = this.AddSubPart( tsubpart1, 10, 320, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Clear all label info for this hex", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a7bid = this.AddSubPart( tsubpart1, 50, 319, 300, 20, 0);
      this.ss = "Click to set multi hex label";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.a8id = this.AddSubPart( tsubpart1, 10, 350, 32, 16, 1);
      tsubpart1 =  TextPartClass::new("Set Multihex label", Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 300, 20, false, tDescript: this.ss);
      this.a8bid = this.AddSubPart( tsubpart1, 50, 349, 300, 20, 0);
      this.ss = "Click to set roads to go off map or to remove off map bits if no corresponding road.";
      tsubpart1 =  ButtonPartClass::new(this.game.BUTTONYELLOW, tDescript: this.ss);
      this.a9id = this.AddSubPart( tsubpart1, 10, 380, 32, 16, 1);
      txt: String = "Do Roads Offmap (";
      let mut index: i32 = 0;
      do
      {
        str: String = txt + this.game.Data.MapObj[0].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index].ToString();
        txt = index >= 5 ? str + ")" : str + ",";
        index += 1;
      }
      while (index <= 5);
      tsubpart1 =  TextPartClass::new(txt, Font::new("Times New Roman", 16f, FontStyle.Regular, GraphicsUnit.Pixel), 600, 20, false, tDescript: this.ss);
      this.a9bid = this.AddSubPart( tsubpart1, 50, 379, 300, 20, 0);
    }

    pub HandleMouseClick: WindowReturnClass(x: i32, y: i32, b: i32)
    {
      windowReturnClass: WindowReturnClass = WindowReturnClass::new();
      if (this.SubPartCounter > -1)
      {
        let mut subPartCounter: i32 = this.SubPartCounter;
        for (let mut index1: i32 = 0; index1 <= subPartCounter; index1 += 1)
        {
          if (x > this.SubPartX[index1] & x < this.SubPartX[index1] + this.SubPartW[index1] && y > this.SubPartY[index1] & y < this.SubPartY[index1] + this.SubPartH[index1])
          {
            let mut num1: i32 = this.SubPartID[index1];
            if (num1 == this.AreaListId)
            {
              let mut num2: i32 = this.SubPartList[index1].Click(x - this.SubPartX[index1], y - this.SubPartY[index1]);
              this.SubPartFlag[index1] = true;
              if (num2 > -1)
              {
                this.detailnr2 = num2;
                this.showinfo();
              }
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BRemoveAreaId)
            {
              this.game.Data.RemoveArea(this.detailnr2);
              --this.detailnr2;
              this.MakeAreaList();
              this.showinfo();
              windowReturnClass.SetFlag(true);
              return windowReturnClass;
            }
            if (num1 == this.BAddAreaId)
            {
              this.game.Data.AddArea();
              this.game.Data.AreaObj[this.game.Data.AreaCounter].Slot =  Math.Round(Conversion.Val(Interaction.InputBox("Give Area Slot # 0-9..", "Shadow Empire : Planetary Conquest")));
              this.game.Data.AreaObj[this.game.Data.AreaCounter].Code =  Math.Round(Conversion.Val(Interaction.InputBox("Give value/code..", "Shadow Empire : Planetary Conquest")));
              this.game.Data.AreaObj[this.game.Data.AreaCounter].Name = Interaction.InputBox("Give name for area", "Shadow Empire : Planetary Conquest");
              this.MakeAreaList();
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
            if (num1 == this.BAreaId)
            {
              this.game.EditObj.PencilType = 9;
              let mut num3: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Area Slot # 0-9..", "Shadow Empire : Planetary Conquest")));
              if (num3 > -1 & num3 < 10)
              {
                this.game.EditObj.PencilData1 = num3;
                let mut num4: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Area Code to Set..", "Shadow Empire : Planetary Conquest")));
                if (num4 > -2 & num4 < 99999)
                {
                  this.game.EditObj.PencilData2 = num4;
                  windowReturnClass.AddCommand(1, 13);
                  windowReturnClass.AddCommand(2, 13);
                  this.showinfo();
                  windowReturnClass.SetFlag(true);
                  return windowReturnClass;
                }
              }
            }
            else
            {
              if (num1 == this.a1id)
              {
                let mut num5: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give New VP for hex 0-99", "Shadow Empire : Planetary Conquest")));
                if (num5 < 0 | num5 > 99)
                {
                  let mut num6: i32 =  Interaction.MsgBox( "Oh.. please.. between 0 and 99!", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].VP = num5;
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
              if (num1 == this.a12id)
              {
                this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SmallLabel = Interaction.InputBox("Give new small Label text..", "Shadow Empire : Planetary Conquest");
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a13id)
              {
                str: String = Interaction.InputBox("Give new small Label type.. 00=normal, 10=top, 20=bottom, x0 = white, x1=red, x2=green, x3=blue, x4=yellow, x5=light blue... ", "Shadow Empire : Planetary Conquest");
                if (Operators.CompareString(str, "", false) != 0)
                  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].SmallLabelType =  Math.Round(Conversion.Val(str));
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
                let mut num7: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Label Type for top of hex (label1)", "Shadow Empire : Planetary Conquest")));
                if (num7 < 0 | num7 > 10)
                {
                  let mut num8: i32 =  Interaction.MsgBox( "Oh.. please.. between 0 and 10 (0=dont show, 1-5 = small font, 6-10 = big font)", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType1 = num7;
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a6id)
              {
                let mut num9: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Label Type for bottom of hex (label2)", "Shadow Empire : Planetary Conquest")));
                if (num9 < 0 | num9 > 10)
                {
                  let mut num10: i32 =  Interaction.MsgBox( "Oh.. please.. between 0 and 10 (0=dont show, 1-5 = small font, 6-10 = big font)", Title: ( "Shadow Empire : Planetary Conquest"));
                }
                else
                  this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].LabelType2 = num9;
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
                let mut usetype: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Label Type for top of hex (label1)", "Shadow Empire : Planetary Conquest")));
                if (usetype > 0 & usetype <= 10 && Strings.Len(str) > 0)
                {
                  this.game.HandyFunctionsObj.MakeSpecificAutoLabels(this.game.SelectX, this.game.SelectY, this.game.EditObj.MapSelected, usetype, str, true);
                  let mut num11: i32 =  Interaction.MsgBox( "Done");
                }
                this.showinfo();
                windowReturnClass.SetFlag(true);
                return windowReturnClass;
              }
              if (num1 == this.a9id)
              {
                let mut index2: i32 =  Math.Round(Conversion.Val(Interaction.InputBox("Give Hex Side (0=top,1=NE, 2=SE, 3=S, 4=SW, 5=NW)", "Shadow Empire : Planetary Conquest")));
                if (index2 >= 0 & index2 <= 5)
                {
                  object Left =  Conversion.Val(Interaction.InputBox("Give RoadType", "Shadow Empire : Planetary Conquest"));
                  if (Conversions.ToBoolean(Operators.AndObject(Operators.CompareObjectGreaterEqual(Left,  -1, false), Operators.CompareObjectLessEqual(Left,  this.game.Data.RoadTypeCounter, false))))
                  {
                    this.game.Data.MapObj[this.game.EditObj.MapSelected].HexObj[this.game.SelectX, this.game.SelectY].RoadType[index2] = Conversions.ToInteger(Left);
                    let mut num12: i32 =  Interaction.MsgBox( "Done");
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
  }
}
